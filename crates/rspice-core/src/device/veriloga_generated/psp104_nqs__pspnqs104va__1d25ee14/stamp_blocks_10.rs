#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_160(
        locals: &mut StampLocals,
    ) {
        let (assign81670_e121591, assign81670_e121591_d_n5, assign81670_e121591_d_n6, assign81670_e121591_d_n7, assign81670_e121591_d_n8, assign81670_e121591_d_n12, assign81670_e121591_d_n13, assign81670_e121591_d_n14, assign81670_e121591_d_n15, assign81670_e121591_d_n16, assign81670_e121591_d_n17, assign81670_e121591_d_n18, assign81670_e121591_d_n19, assign81670_e121591_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2220 == 0.0)) && (locals.var_guard2221 == 0.0)) {
        let assign81670_e121582: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign81670_e121583: f64 = (2.0 * assign81670_e121582);
        let assign81670_e121587: f64 = (1.0 - locals.var_nqs_d0);
        let assign81670_e121588: f64 = (locals.var_gp2 * assign81670_e121587);
        let assign81670_e121589: f64 = (assign81670_e121583 + assign81670_e121588);
        (assign81670_e121589, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign81670_e121587) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign81670_e121591;
        locals.var_nqs_p_dn5 = assign81670_e121591_d_n5;
        locals.var_nqs_p_dn6 = assign81670_e121591_d_n6;
        locals.var_nqs_p_dn7 = assign81670_e121591_d_n7;
        locals.var_nqs_p_dn8 = assign81670_e121591_d_n8;
        locals.var_nqs_p_dn12 = assign81670_e121591_d_n12;
        locals.var_nqs_p_dn13 = assign81670_e121591_d_n13;
        locals.var_nqs_p_dn14 = assign81670_e121591_d_n14;
        locals.var_nqs_p_dn15 = assign81670_e121591_d_n15;
        locals.var_nqs_p_dn16 = assign81670_e121591_d_n16;
        locals.var_nqs_p_dn17 = assign81670_e121591_d_n17;
        locals.var_nqs_p_dn18 = assign81670_e121591_d_n18;
        locals.var_nqs_p_dn19 = assign81670_e121591_d_n19;
        locals.var_nqs_p_dn20 = assign81670_e121591_d_n20;

        let (assign81680_e121629, assign81680_e121629_d_n5, assign81680_e121629_d_n6, assign81680_e121629_d_n7, assign81680_e121629_d_n8, assign81680_e121629_d_n12, assign81680_e121629_d_n13, assign81680_e121629_d_n14, assign81680_e121629_d_n15, assign81680_e121629_d_n16, assign81680_e121629_d_n17, assign81680_e121629_d_n18, assign81680_e121629_d_n19, assign81680_e121629_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2220 == 0.0)) && (locals.var_guard2221 == 0.0)) {
        let assign81680_e121615: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign81680_e121618: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign81680_e121619: f64 = (assign81680_e121615 * assign81680_e121618);
        let assign81680_e121623: f64 = (locals.var_nqs_x0 - 1.0);
        let assign81680_e121625: f64 = (assign81680_e121623 + locals.var_nqs_d0);
        let assign81680_e121626: f64 = (locals.var_gp2 * assign81680_e121625);
        let assign81680_e121627: f64 = (assign81680_e121619 - assign81680_e121626);
        (assign81680_e121627, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign81680_e121618) + (assign81680_e121615 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign81680_e121625) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign81680_e121629;
        locals.var_nqs_q_dn5 = assign81680_e121629_d_n5;
        locals.var_nqs_q_dn6 = assign81680_e121629_d_n6;
        locals.var_nqs_q_dn7 = assign81680_e121629_d_n7;
        locals.var_nqs_q_dn8 = assign81680_e121629_d_n8;
        locals.var_nqs_q_dn12 = assign81680_e121629_d_n12;
        locals.var_nqs_q_dn13 = assign81680_e121629_d_n13;
        locals.var_nqs_q_dn14 = assign81680_e121629_d_n14;
        locals.var_nqs_q_dn15 = assign81680_e121629_d_n15;
        locals.var_nqs_q_dn16 = assign81680_e121629_d_n16;
        locals.var_nqs_q_dn17 = assign81680_e121629_d_n17;
        locals.var_nqs_q_dn18 = assign81680_e121629_d_n18;
        locals.var_nqs_q_dn19 = assign81680_e121629_d_n19;
        locals.var_nqs_q_dn20 = assign81680_e121629_d_n20;

        let (assign81690_e121661, assign81690_e121661_d_n5, assign81690_e121661_d_n6, assign81690_e121661_d_n7, assign81690_e121661_d_n8, assign81690_e121661_d_n12, assign81690_e121661_d_n13, assign81690_e121661_d_n14, assign81690_e121661_d_n15, assign81690_e121661_d_n16, assign81690_e121661_d_n17, assign81690_e121661_d_n18, assign81690_e121661_d_n19, assign81690_e121661_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2220 == 0.0)) && (locals.var_guard2221 == 0.0)) {
        let assign81690_e121653: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign81690_e121656: f64 = (4.0 * locals.var_nqs_xi);
        let assign81690_e121658: f64 = (assign81690_e121656 * locals.var_nqs_q);
        let assign81690_e121659: f64 = (assign81690_e121653 - assign81690_e121658);
        (assign81690_e121659, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign81690_e121656 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign81690_e121661;
        locals.var_nqs_temp_dn5 = assign81690_e121661_d_n5;
        locals.var_nqs_temp_dn6 = assign81690_e121661_d_n6;
        locals.var_nqs_temp_dn7 = assign81690_e121661_d_n7;
        locals.var_nqs_temp_dn8 = assign81690_e121661_d_n8;
        locals.var_nqs_temp_dn12 = assign81690_e121661_d_n12;
        locals.var_nqs_temp_dn13 = assign81690_e121661_d_n13;
        locals.var_nqs_temp_dn14 = assign81690_e121661_d_n14;
        locals.var_nqs_temp_dn15 = assign81690_e121661_d_n15;
        locals.var_nqs_temp_dn16 = assign81690_e121661_d_n16;
        locals.var_nqs_temp_dn17 = assign81690_e121661_d_n17;
        locals.var_nqs_temp_dn18 = assign81690_e121661_d_n18;
        locals.var_nqs_temp_dn19 = assign81690_e121661_d_n19;
        locals.var_nqs_temp_dn20 = assign81690_e121661_d_n20;

        let (assign81700_e121692, assign81700_e121692_d_n5, assign81700_e121692_d_n6, assign81700_e121692_d_n7, assign81700_e121692_d_n8, assign81700_e121692_d_n12, assign81700_e121692_d_n13, assign81700_e121692_d_n14, assign81700_e121692_d_n15, assign81700_e121692_d_n16, assign81700_e121692_d_n17, assign81700_e121692_d_n18, assign81700_e121692_d_n19, assign81700_e121692_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2220 == 0.0)) && (locals.var_guard2221 == 0.0)) {
        let assign81700_e121685: f64 = (2.0 * locals.var_nqs_q);
        let assign81700_e121688: f64 = (locals.var_nqs_temp).sqrt();
        let assign81700_e121689: f64 = (locals.var_nqs_p + assign81700_e121688);
        let assign81700_e121690: f64 = (assign81700_e121685 / assign81700_e121689);
        (assign81700_e121690, ((((2.0 * locals.var_nqs_q_dn5) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn6) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn7) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn8) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn12) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn13) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn14) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn15) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn16) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn17) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn18) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn19) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)), ((((2.0 * locals.var_nqs_q_dn20) * assign81700_e121689) - (assign81700_e121685 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign81700_e121688))))) / (assign81700_e121689 * assign81700_e121689)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign81700_e121692;
        locals.var_nqs_u_dn5 = assign81700_e121692_d_n5;
        locals.var_nqs_u_dn6 = assign81700_e121692_d_n6;
        locals.var_nqs_u_dn7 = assign81700_e121692_d_n7;
        locals.var_nqs_u_dn8 = assign81700_e121692_d_n8;
        locals.var_nqs_u_dn12 = assign81700_e121692_d_n12;
        locals.var_nqs_u_dn13 = assign81700_e121692_d_n13;
        locals.var_nqs_u_dn14 = assign81700_e121692_d_n14;
        locals.var_nqs_u_dn15 = assign81700_e121692_d_n15;
        locals.var_nqs_u_dn16 = assign81700_e121692_d_n16;
        locals.var_nqs_u_dn17 = assign81700_e121692_d_n17;
        locals.var_nqs_u_dn18 = assign81700_e121692_d_n18;
        locals.var_nqs_u_dn19 = assign81700_e121692_d_n19;
        locals.var_nqs_u_dn20 = assign81700_e121692_d_n20;

        let (assign81710_e121718, assign81710_e121718_d_n5, assign81710_e121718_d_n6, assign81710_e121718_d_n7, assign81710_e121718_d_n8, assign81710_e121718_d_n12, assign81710_e121718_d_n13, assign81710_e121718_d_n14, assign81710_e121718_d_n15, assign81710_e121718_d_n16, assign81710_e121718_d_n17, assign81710_e121718_d_n18, assign81710_e121718_d_n19, assign81710_e121718_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2220 == 0.0)) && (locals.var_guard2221 == 0.0)) {
        let assign81710_e121716: f64 = (locals.var_nqs_x0 + locals.var_nqs_u);
        (assign81710_e121716, (locals.var_nqs_x0_dn5 + locals.var_nqs_u_dn5), (locals.var_nqs_x0_dn6 + locals.var_nqs_u_dn6), (locals.var_nqs_x0_dn7 + locals.var_nqs_u_dn7), (locals.var_nqs_x0_dn8 + locals.var_nqs_u_dn8), (locals.var_nqs_x0_dn12 + locals.var_nqs_u_dn12), (locals.var_nqs_x0_dn13 + locals.var_nqs_u_dn13), (locals.var_nqs_x0_dn14 + locals.var_nqs_u_dn14), (locals.var_nqs_x0_dn15 + locals.var_nqs_u_dn15), (locals.var_nqs_x0_dn16 + locals.var_nqs_u_dn16), (locals.var_nqs_x0_dn17 + locals.var_nqs_u_dn17), (locals.var_nqs_x0_dn18 + locals.var_nqs_u_dn18), (locals.var_nqs_x0_dn19 + locals.var_nqs_u_dn19), (locals.var_nqs_x0_dn20 + locals.var_nqs_u_dn20),)
    } else {
        (locals.var_temp7, locals.var_temp7_dn5, locals.var_temp7_dn6, locals.var_temp7_dn7, locals.var_temp7_dn8, locals.var_temp7_dn12, locals.var_temp7_dn13, locals.var_temp7_dn14, locals.var_temp7_dn15, locals.var_temp7_dn16, locals.var_temp7_dn17, locals.var_temp7_dn18, locals.var_temp7_dn19, locals.var_temp7_dn20,)
    }
};
        locals.var_temp7 = assign81710_e121718;
        locals.var_temp7_dn5 = assign81710_e121718_d_n5;
        locals.var_temp7_dn6 = assign81710_e121718_d_n6;
        locals.var_temp7_dn7 = assign81710_e121718_d_n7;
        locals.var_temp7_dn8 = assign81710_e121718_d_n8;
        locals.var_temp7_dn12 = assign81710_e121718_d_n12;
        locals.var_temp7_dn13 = assign81710_e121718_d_n13;
        locals.var_temp7_dn14 = assign81710_e121718_d_n14;
        locals.var_temp7_dn15 = assign81710_e121718_d_n15;
        locals.var_temp7_dn16 = assign81710_e121718_d_n16;
        locals.var_temp7_dn17 = assign81710_e121718_d_n17;
        locals.var_temp7_dn18 = assign81710_e121718_d_n18;
        locals.var_temp7_dn19 = assign81710_e121718_d_n19;
        locals.var_temp7_dn20 = assign81710_e121718_d_n20;

        let (assign81720_e121740, assign81720_e121740_d_n5, assign81720_e121740_d_n6, assign81720_e121740_d_n7, assign81720_e121740_d_n8, assign81720_e121740_d_n12, assign81720_e121740_d_n13, assign81720_e121740_d_n14, assign81720_e121740_d_n15, assign81720_e121740_d_n16, assign81720_e121740_d_n17, assign81720_e121740_d_n18, assign81720_e121740_d_n19, assign81720_e121740_d_n20,) = {
    if ((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        let assign81720_e121736: f64 = (locals.var_qp8 / locals.var_pd);
        let assign81720_e121738: f64 = (assign81720_e121736 + locals.var_xg_ac);
        (assign81720_e121738, ((-((locals.var_qp8 * locals.var_pd_dn5) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn5), ((-((locals.var_qp8 * locals.var_pd_dn6) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn6), ((-((locals.var_qp8 * locals.var_pd_dn7) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn7), ((-((locals.var_qp8 * locals.var_pd_dn8) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn8), ((-((locals.var_qp8 * locals.var_pd_dn12) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn12), ((-((locals.var_qp8 * locals.var_pd_dn13) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn13), ((-((locals.var_qp8 * locals.var_pd_dn14) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn14), ((-((locals.var_qp8 * locals.var_pd_dn15) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn15), ((-((locals.var_qp8 * locals.var_pd_dn16) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn16), ((-((locals.var_qp8 * locals.var_pd_dn17) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn17), ((-((locals.var_qp8 * locals.var_pd_dn18) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn18), ((((locals.var_qp8_dn19 * locals.var_pd) - (locals.var_qp8 * locals.var_pd_dn19)) / (locals.var_pd * locals.var_pd)) + locals.var_xg_ac_dn19), ((-((locals.var_qp8 * locals.var_pd_dn20) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn20),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign81720_e121740;
        locals.var_temp__blk1038_dn5 = assign81720_e121740_d_n5;
        locals.var_temp__blk1038_dn6 = assign81720_e121740_d_n6;
        locals.var_temp__blk1038_dn7 = assign81720_e121740_d_n7;
        locals.var_temp__blk1038_dn8 = assign81720_e121740_d_n8;
        locals.var_temp__blk1038_dn12 = assign81720_e121740_d_n12;
        locals.var_temp__blk1038_dn13 = assign81720_e121740_d_n13;
        locals.var_temp__blk1038_dn14 = assign81720_e121740_d_n14;
        locals.var_temp__blk1038_dn15 = assign81720_e121740_d_n15;
        locals.var_temp__blk1038_dn16 = assign81720_e121740_d_n16;
        locals.var_temp__blk1038_dn17 = assign81720_e121740_d_n17;
        locals.var_temp__blk1038_dn18 = assign81720_e121740_d_n18;
        locals.var_temp__blk1038_dn19 = assign81720_e121740_d_n19;
        locals.var_temp__blk1038_dn20 = assign81720_e121740_d_n20;

        let assign81730_e121742: f64 = (locals.var_temp__blk1038).abs();
        let assign81730_e121744: f64 = if assign81730_e121742 <= locals.var_marginp { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign81730_e121744;

        let (assign81740_e121766, assign81740_e121766_d_n5, assign81740_e121766_d_n6, assign81740_e121766_d_n7, assign81740_e121766_d_n8, assign81740_e121766_d_n12, assign81740_e121766_d_n13, assign81740_e121766_d_n14, assign81740_e121766_d_n15, assign81740_e121766_d_n16, assign81740_e121766_d_n17, assign81740_e121766_d_n18, assign81740_e121766_d_n19, assign81740_e121766_d_n20,) = {
    if (((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 != 0.0)) {
        let assign81740_e121764: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        (assign81740_e121764, (((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_temp8, locals.var_temp8_dn5, locals.var_temp8_dn6, locals.var_temp8_dn7, locals.var_temp8_dn8, locals.var_temp8_dn12, locals.var_temp8_dn13, locals.var_temp8_dn14, locals.var_temp8_dn15, locals.var_temp8_dn16, locals.var_temp8_dn17, locals.var_temp8_dn18, locals.var_temp8_dn19, locals.var_temp8_dn20,)
    }
};
        locals.var_temp8 = assign81740_e121766;
        locals.var_temp8_dn5 = assign81740_e121766_d_n5;
        locals.var_temp8_dn6 = assign81740_e121766_d_n6;
        locals.var_temp8_dn7 = assign81740_e121766_d_n7;
        locals.var_temp8_dn8 = assign81740_e121766_d_n8;
        locals.var_temp8_dn12 = assign81740_e121766_d_n12;
        locals.var_temp8_dn13 = assign81740_e121766_d_n13;
        locals.var_temp8_dn14 = assign81740_e121766_d_n14;
        locals.var_temp8_dn15 = assign81740_e121766_d_n15;
        locals.var_temp8_dn16 = assign81740_e121766_d_n16;
        locals.var_temp8_dn17 = assign81740_e121766_d_n17;
        locals.var_temp8_dn18 = assign81740_e121766_d_n18;
        locals.var_temp8_dn19 = assign81740_e121766_d_n19;
        locals.var_temp8_dn20 = assign81740_e121766_d_n20;

        let assign81750_e121769: f64 = (-locals.var_marginp);
        let assign81750_e121770: f64 = if locals.var_temp__blk1038 < assign81750_e121769 { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign81750_e121770;

        let (assign81760_e121794, assign81760_e121794_d_n5, assign81760_e121794_d_n6, assign81760_e121794_d_n7, assign81760_e121794_d_n8, assign81760_e121794_d_n12, assign81760_e121794_d_n13, assign81760_e121794_d_n14, assign81760_e121794_d_n15, assign81760_e121794_d_n16, assign81760_e121794_d_n17, assign81760_e121794_d_n18, assign81760_e121794_d_n19, assign81760_e121794_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81760_e121792: f64 = (-locals.var_temp__blk1038);
        (assign81760_e121792, (-locals.var_temp__blk1038_dn5), (-locals.var_temp__blk1038_dn6), (-locals.var_temp__blk1038_dn7), (-locals.var_temp__blk1038_dn8), (-locals.var_temp__blk1038_dn12), (-locals.var_temp__blk1038_dn13), (-locals.var_temp__blk1038_dn14), (-locals.var_temp__blk1038_dn15), (-locals.var_temp__blk1038_dn16), (-locals.var_temp__blk1038_dn17), (-locals.var_temp__blk1038_dn18), (-locals.var_temp__blk1038_dn19), (-locals.var_temp__blk1038_dn20),)
    } else {
        (locals.var_nqs_yg, locals.var_nqs_yg_dn5, locals.var_nqs_yg_dn6, locals.var_nqs_yg_dn7, locals.var_nqs_yg_dn8, locals.var_nqs_yg_dn12, locals.var_nqs_yg_dn13, locals.var_nqs_yg_dn14, locals.var_nqs_yg_dn15, locals.var_nqs_yg_dn16, locals.var_nqs_yg_dn17, locals.var_nqs_yg_dn18, locals.var_nqs_yg_dn19, locals.var_nqs_yg_dn20,)
    }
};
        locals.var_nqs_yg = assign81760_e121794;
        locals.var_nqs_yg_dn5 = assign81760_e121794_d_n5;
        locals.var_nqs_yg_dn6 = assign81760_e121794_d_n6;
        locals.var_nqs_yg_dn7 = assign81760_e121794_d_n7;
        locals.var_nqs_yg_dn8 = assign81760_e121794_d_n8;
        locals.var_nqs_yg_dn12 = assign81760_e121794_d_n12;
        locals.var_nqs_yg_dn13 = assign81760_e121794_d_n13;
        locals.var_nqs_yg_dn14 = assign81760_e121794_d_n14;
        locals.var_nqs_yg_dn15 = assign81760_e121794_d_n15;
        locals.var_nqs_yg_dn16 = assign81760_e121794_d_n16;
        locals.var_nqs_yg_dn17 = assign81760_e121794_d_n17;
        locals.var_nqs_yg_dn18 = assign81760_e121794_d_n18;
        locals.var_nqs_yg_dn19 = assign81760_e121794_d_n19;
        locals.var_nqs_yg_dn20 = assign81760_e121794_d_n20;

        let (assign81770_e121821, assign81770_e121821_d_n5, assign81770_e121821_d_n6, assign81770_e121821_d_n7, assign81770_e121821_d_n8, assign81770_e121821_d_n12, assign81770_e121821_d_n13, assign81770_e121821_d_n14, assign81770_e121821_d_n15, assign81770_e121821_d_n16, assign81770_e121821_d_n17, assign81770_e121821_d_n18, assign81770_e121821_d_n19, assign81770_e121821_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81770_e121817: f64 = (1.25 * locals.var_nqs_yg);
        let assign81770_e121819: f64 = (assign81770_e121817 / locals.var_a_factrp);
        (assign81770_e121819, ((((1.25 * locals.var_nqs_yg_dn5) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn6) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn7) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn8) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn12) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn13) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn14) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn15) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn16) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn17) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn18) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn19) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn20) * locals.var_a_factrp) - (assign81770_e121817 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_nqs_z, locals.var_nqs_z_dn5, locals.var_nqs_z_dn6, locals.var_nqs_z_dn7, locals.var_nqs_z_dn8, locals.var_nqs_z_dn12, locals.var_nqs_z_dn13, locals.var_nqs_z_dn14, locals.var_nqs_z_dn15, locals.var_nqs_z_dn16, locals.var_nqs_z_dn17, locals.var_nqs_z_dn18, locals.var_nqs_z_dn19, locals.var_nqs_z_dn20,)
    }
};
        locals.var_nqs_z = assign81770_e121821;
        locals.var_nqs_z_dn5 = assign81770_e121821_d_n5;
        locals.var_nqs_z_dn6 = assign81770_e121821_d_n6;
        locals.var_nqs_z_dn7 = assign81770_e121821_d_n7;
        locals.var_nqs_z_dn8 = assign81770_e121821_d_n8;
        locals.var_nqs_z_dn12 = assign81770_e121821_d_n12;
        locals.var_nqs_z_dn13 = assign81770_e121821_d_n13;
        locals.var_nqs_z_dn14 = assign81770_e121821_d_n14;
        locals.var_nqs_z_dn15 = assign81770_e121821_d_n15;
        locals.var_nqs_z_dn16 = assign81770_e121821_d_n16;
        locals.var_nqs_z_dn17 = assign81770_e121821_d_n17;
        locals.var_nqs_z_dn18 = assign81770_e121821_d_n18;
        locals.var_nqs_z_dn19 = assign81770_e121821_d_n19;
        locals.var_nqs_z_dn20 = assign81770_e121821_d_n20;

        let (assign81780_e121859, assign81780_e121859_d_n5, assign81780_e121859_d_n6, assign81780_e121859_d_n7, assign81780_e121859_d_n8, assign81780_e121859_d_n12, assign81780_e121859_d_n13, assign81780_e121859_d_n14, assign81780_e121859_d_n15, assign81780_e121859_d_n16, assign81780_e121859_d_n17, assign81780_e121859_d_n18, assign81780_e121859_d_n19, assign81780_e121859_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81780_e121844: f64 = (locals.var_nqs_z + 10.0);
        let assign81780_e121847: f64 = (locals.var_nqs_z - 6.0);
        let assign81780_e121850: f64 = (locals.var_nqs_z - 6.0);
        let assign81780_e121851: f64 = (assign81780_e121847 * assign81780_e121850);
        let assign81780_e121853: f64 = (assign81780_e121851 + 64.0);
        let assign81780_e121854: f64 = (assign81780_e121853).sqrt();
        let assign81780_e121855: f64 = (assign81780_e121844 - assign81780_e121854);
        let assign81780_e121857: f64 = (assign81780_e121855 * 0.5);
        (assign81780_e121857, ((locals.var_nqs_z_dn5 - (((locals.var_nqs_z_dn5 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn5)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn6 - (((locals.var_nqs_z_dn6 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn6)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn7 - (((locals.var_nqs_z_dn7 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn7)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn8 - (((locals.var_nqs_z_dn8 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn8)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn12 - (((locals.var_nqs_z_dn12 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn12)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn13 - (((locals.var_nqs_z_dn13 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn13)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn14 - (((locals.var_nqs_z_dn14 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn14)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn15 - (((locals.var_nqs_z_dn15 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn15)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn16 - (((locals.var_nqs_z_dn16 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn16)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn17 - (((locals.var_nqs_z_dn17 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn17)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn18 - (((locals.var_nqs_z_dn18 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn18)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn19 - (((locals.var_nqs_z_dn19 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn19)) / (2.0 * assign81780_e121854))) * 0.5), ((locals.var_nqs_z_dn20 - (((locals.var_nqs_z_dn20 * assign81780_e121850) + (assign81780_e121847 * locals.var_nqs_z_dn20)) / (2.0 * assign81780_e121854))) * 0.5),)
    } else {
        (locals.var_nqs_eta, locals.var_nqs_eta_dn5, locals.var_nqs_eta_dn6, locals.var_nqs_eta_dn7, locals.var_nqs_eta_dn8, locals.var_nqs_eta_dn12, locals.var_nqs_eta_dn13, locals.var_nqs_eta_dn14, locals.var_nqs_eta_dn15, locals.var_nqs_eta_dn16, locals.var_nqs_eta_dn17, locals.var_nqs_eta_dn18, locals.var_nqs_eta_dn19, locals.var_nqs_eta_dn20,)
    }
};
        locals.var_nqs_eta = assign81780_e121859;
        locals.var_nqs_eta_dn5 = assign81780_e121859_d_n5;
        locals.var_nqs_eta_dn6 = assign81780_e121859_d_n6;
        locals.var_nqs_eta_dn7 = assign81780_e121859_d_n7;
        locals.var_nqs_eta_dn8 = assign81780_e121859_d_n8;
        locals.var_nqs_eta_dn12 = assign81780_e121859_d_n12;
        locals.var_nqs_eta_dn13 = assign81780_e121859_d_n13;
        locals.var_nqs_eta_dn14 = assign81780_e121859_d_n14;
        locals.var_nqs_eta_dn15 = assign81780_e121859_d_n15;
        locals.var_nqs_eta_dn16 = assign81780_e121859_d_n16;
        locals.var_nqs_eta_dn17 = assign81780_e121859_d_n17;
        locals.var_nqs_eta_dn18 = assign81780_e121859_d_n18;
        locals.var_nqs_eta_dn19 = assign81780_e121859_d_n19;
        locals.var_nqs_eta_dn20 = assign81780_e121859_d_n20;

        let (assign81790_e121894, assign81790_e121894_d_n5, assign81790_e121894_d_n6, assign81790_e121894_d_n7, assign81790_e121894_d_n8, assign81790_e121894_d_n12, assign81790_e121894_d_n13, assign81790_e121894_d_n14, assign81790_e121894_d_n15, assign81790_e121894_d_n16, assign81790_e121894_d_n17, assign81790_e121894_d_n18, assign81790_e121894_d_n19, assign81790_e121894_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81790_e121882: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign81790_e121885: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign81790_e121886: f64 = (assign81790_e121882 * assign81790_e121885);
        let assign81790_e121890: f64 = (locals.var_nqs_eta + 1.0);
        let assign81790_e121891: f64 = (locals.var_gp2 * assign81790_e121890);
        let assign81790_e121892: f64 = (assign81790_e121886 + assign81790_e121891);
        (assign81790_e121892, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5))) + ((locals.var_gp2_dn5 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn5))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6))) + ((locals.var_gp2_dn6 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn6))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7))) + ((locals.var_gp2_dn7 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn7))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8))) + ((locals.var_gp2_dn8 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn8))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12))) + ((locals.var_gp2_dn12 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn12))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13))) + ((locals.var_gp2_dn13 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn13))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14))) + ((locals.var_gp2_dn14 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn14))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15))) + ((locals.var_gp2_dn15 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn15))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16))) + ((locals.var_gp2_dn16 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn16))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17))) + ((locals.var_gp2_dn17 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn17))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18))) + ((locals.var_gp2_dn18 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn18))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19))) + ((locals.var_gp2_dn19 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn19))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20) * assign81790_e121885) + (assign81790_e121882 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20))) + ((locals.var_gp2_dn20 * assign81790_e121890) + (locals.var_gp2 * locals.var_nqs_eta_dn20))),)
    } else {
        (locals.var_nqs_a, locals.var_nqs_a_dn5, locals.var_nqs_a_dn6, locals.var_nqs_a_dn7, locals.var_nqs_a_dn8, locals.var_nqs_a_dn12, locals.var_nqs_a_dn13, locals.var_nqs_a_dn14, locals.var_nqs_a_dn15, locals.var_nqs_a_dn16, locals.var_nqs_a_dn17, locals.var_nqs_a_dn18, locals.var_nqs_a_dn19, locals.var_nqs_a_dn20,)
    }
};
        locals.var_nqs_a = assign81790_e121894;
        locals.var_nqs_a_dn5 = assign81790_e121894_d_n5;
        locals.var_nqs_a_dn6 = assign81790_e121894_d_n6;
        locals.var_nqs_a_dn7 = assign81790_e121894_d_n7;
        locals.var_nqs_a_dn8 = assign81790_e121894_d_n8;
        locals.var_nqs_a_dn12 = assign81790_e121894_d_n12;
        locals.var_nqs_a_dn13 = assign81790_e121894_d_n13;
        locals.var_nqs_a_dn14 = assign81790_e121894_d_n14;
        locals.var_nqs_a_dn15 = assign81790_e121894_d_n15;
        locals.var_nqs_a_dn16 = assign81790_e121894_d_n16;
        locals.var_nqs_a_dn17 = assign81790_e121894_d_n17;
        locals.var_nqs_a_dn18 = assign81790_e121894_d_n18;
        locals.var_nqs_a_dn19 = assign81790_e121894_d_n19;
        locals.var_nqs_a_dn20 = assign81790_e121894_d_n20;

        let (assign81800_e121923, assign81800_e121923_d_n5, assign81800_e121923_d_n6, assign81800_e121923_d_n7, assign81800_e121923_d_n8, assign81800_e121923_d_n12, assign81800_e121923_d_n13, assign81800_e121923_d_n14, assign81800_e121923_d_n15, assign81800_e121923_d_n16, assign81800_e121923_d_n17, assign81800_e121923_d_n18, assign81800_e121923_d_n19, assign81800_e121923_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81800_e121918: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign81800_e121919: f64 = (2.0 * assign81800_e121918);
        let assign81800_e121921: f64 = (assign81800_e121919 - locals.var_gp2);
        (assign81800_e121921, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5)) - locals.var_gp2_dn5), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6)) - locals.var_gp2_dn6), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7)) - locals.var_gp2_dn7), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8)) - locals.var_gp2_dn8), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12)) - locals.var_gp2_dn12), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13)) - locals.var_gp2_dn13), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14)) - locals.var_gp2_dn14), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15)) - locals.var_gp2_dn15), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16)) - locals.var_gp2_dn16), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17)) - locals.var_gp2_dn17), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18)) - locals.var_gp2_dn18), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19)) - locals.var_gp2_dn19), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20)) - locals.var_gp2_dn20),)
    } else {
        (locals.var_nqs_c, locals.var_nqs_c_dn5, locals.var_nqs_c_dn6, locals.var_nqs_c_dn7, locals.var_nqs_c_dn8, locals.var_nqs_c_dn12, locals.var_nqs_c_dn13, locals.var_nqs_c_dn14, locals.var_nqs_c_dn15, locals.var_nqs_c_dn16, locals.var_nqs_c_dn17, locals.var_nqs_c_dn18, locals.var_nqs_c_dn19, locals.var_nqs_c_dn20,)
    }
};
        locals.var_nqs_c = assign81800_e121923;
        locals.var_nqs_c_dn5 = assign81800_e121923_d_n5;
        locals.var_nqs_c_dn6 = assign81800_e121923_d_n6;
        locals.var_nqs_c_dn7 = assign81800_e121923_d_n7;
        locals.var_nqs_c_dn8 = assign81800_e121923_d_n8;
        locals.var_nqs_c_dn12 = assign81800_e121923_d_n12;
        locals.var_nqs_c_dn13 = assign81800_e121923_d_n13;
        locals.var_nqs_c_dn14 = assign81800_e121923_d_n14;
        locals.var_nqs_c_dn15 = assign81800_e121923_d_n15;
        locals.var_nqs_c_dn16 = assign81800_e121923_d_n16;
        locals.var_nqs_c_dn17 = assign81800_e121923_d_n17;
        locals.var_nqs_c_dn18 = assign81800_e121923_d_n18;
        locals.var_nqs_c_dn19 = assign81800_e121923_d_n19;
        locals.var_nqs_c_dn20 = assign81800_e121923_d_n20;

        let (assign81810_e121951, assign81810_e121951_d_n5, assign81810_e121951_d_n6, assign81810_e121951_d_n7, assign81810_e121951_d_n8, assign81810_e121951_d_n12, assign81810_e121951_d_n13, assign81810_e121951_d_n14, assign81810_e121951_d_n15, assign81810_e121951_d_n16, assign81810_e121951_d_n17, assign81810_e121951_d_n18, assign81810_e121951_d_n19, assign81810_e121951_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81810_e121946: f64 = (locals.var_nqs_a / locals.var_gp2);
        let assign81810_e121947: f64 = (assign81810_e121946).ln();
        let assign81810_e121949: f64 = (assign81810_e121947 - locals.var_nqs_eta);
        (assign81810_e121949, (((((locals.var_nqs_a_dn5 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn5)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn5), (((((locals.var_nqs_a_dn6 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn6)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn6), (((((locals.var_nqs_a_dn7 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn7)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn7), (((((locals.var_nqs_a_dn8 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn8)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn8), (((((locals.var_nqs_a_dn12 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn12)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn12), (((((locals.var_nqs_a_dn13 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn13)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn13), (((((locals.var_nqs_a_dn14 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn14)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn14), (((((locals.var_nqs_a_dn15 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn15)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn15), (((((locals.var_nqs_a_dn16 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn16)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn16), (((((locals.var_nqs_a_dn17 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn17)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn17), (((((locals.var_nqs_a_dn18 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn18)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn18), (((((locals.var_nqs_a_dn19 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn19)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn19), (((((locals.var_nqs_a_dn20 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn20)) / (locals.var_gp2 * locals.var_gp2)) / assign81810_e121946) - locals.var_nqs_eta_dn20),)
    } else {
        (locals.var_nqs_tau, locals.var_nqs_tau_dn5, locals.var_nqs_tau_dn6, locals.var_nqs_tau_dn7, locals.var_nqs_tau_dn8, locals.var_nqs_tau_dn12, locals.var_nqs_tau_dn13, locals.var_nqs_tau_dn14, locals.var_nqs_tau_dn15, locals.var_nqs_tau_dn16, locals.var_nqs_tau_dn17, locals.var_nqs_tau_dn18, locals.var_nqs_tau_dn19, locals.var_nqs_tau_dn20,)
    }
};
        locals.var_nqs_tau = assign81810_e121951;
        locals.var_nqs_tau_dn5 = assign81810_e121951_d_n5;
        locals.var_nqs_tau_dn6 = assign81810_e121951_d_n6;
        locals.var_nqs_tau_dn7 = assign81810_e121951_d_n7;
        locals.var_nqs_tau_dn8 = assign81810_e121951_d_n8;
        locals.var_nqs_tau_dn12 = assign81810_e121951_d_n12;
        locals.var_nqs_tau_dn13 = assign81810_e121951_d_n13;
        locals.var_nqs_tau_dn14 = assign81810_e121951_d_n14;
        locals.var_nqs_tau_dn15 = assign81810_e121951_d_n15;
        locals.var_nqs_tau_dn16 = assign81810_e121951_d_n16;
        locals.var_nqs_tau_dn17 = assign81810_e121951_d_n17;
        locals.var_nqs_tau_dn18 = assign81810_e121951_d_n18;
        locals.var_nqs_tau_dn19 = assign81810_e121951_d_n19;
        locals.var_nqs_tau_dn20 = assign81810_e121951_d_n20;

        let (assign81820_e121976, assign81820_e121976_d_n5, assign81820_e121976_d_n6, assign81820_e121976_d_n7, assign81820_e121976_d_n8, assign81820_e121976_d_n12, assign81820_e121976_d_n13, assign81820_e121976_d_n14, assign81820_e121976_d_n15, assign81820_e121976_d_n16, assign81820_e121976_d_n17, assign81820_e121976_d_n18, assign81820_e121976_d_n19, assign81820_e121976_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81820_e121974: f64 = (locals.var_nqs_a + locals.var_nqs_c);
        (assign81820_e121974, (locals.var_nqs_a_dn5 + locals.var_nqs_c_dn5), (locals.var_nqs_a_dn6 + locals.var_nqs_c_dn6), (locals.var_nqs_a_dn7 + locals.var_nqs_c_dn7), (locals.var_nqs_a_dn8 + locals.var_nqs_c_dn8), (locals.var_nqs_a_dn12 + locals.var_nqs_c_dn12), (locals.var_nqs_a_dn13 + locals.var_nqs_c_dn13), (locals.var_nqs_a_dn14 + locals.var_nqs_c_dn14), (locals.var_nqs_a_dn15 + locals.var_nqs_c_dn15), (locals.var_nqs_a_dn16 + locals.var_nqs_c_dn16), (locals.var_nqs_a_dn17 + locals.var_nqs_c_dn17), (locals.var_nqs_a_dn18 + locals.var_nqs_c_dn18), (locals.var_nqs_a_dn19 + locals.var_nqs_c_dn19), (locals.var_nqs_a_dn20 + locals.var_nqs_c_dn20),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn12, locals.var_nu_dn13, locals.var_nu_dn14, locals.var_nu_dn15, locals.var_nu_dn16, locals.var_nu_dn17, locals.var_nu_dn18, locals.var_nu_dn19, locals.var_nu_dn20,)
    }
};
        locals.var_nu = assign81820_e121976;
        locals.var_nu_dn5 = assign81820_e121976_d_n5;
        locals.var_nu_dn6 = assign81820_e121976_d_n6;
        locals.var_nu_dn7 = assign81820_e121976_d_n7;
        locals.var_nu_dn8 = assign81820_e121976_d_n8;
        locals.var_nu_dn12 = assign81820_e121976_d_n12;
        locals.var_nu_dn13 = assign81820_e121976_d_n13;
        locals.var_nu_dn14 = assign81820_e121976_d_n14;
        locals.var_nu_dn15 = assign81820_e121976_d_n15;
        locals.var_nu_dn16 = assign81820_e121976_d_n16;
        locals.var_nu_dn17 = assign81820_e121976_d_n17;
        locals.var_nu_dn18 = assign81820_e121976_d_n18;
        locals.var_nu_dn19 = assign81820_e121976_d_n19;
        locals.var_nu_dn20 = assign81820_e121976_d_n20;

        let (assign81830_e122011, assign81830_e122011_d_n5, assign81830_e122011_d_n6, assign81830_e122011_d_n7, assign81830_e122011_d_n8, assign81830_e122011_d_n12, assign81830_e122011_d_n13, assign81830_e122011_d_n14, assign81830_e122011_d_n15, assign81830_e122011_d_n16, assign81830_e122011_d_n17, assign81830_e122011_d_n18, assign81830_e122011_d_n19, assign81830_e122011_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81830_e121999: f64 = (locals.var_nu * locals.var_nu);
        let assign81830_e122004: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign81830_e122005: f64 = (0.5 * assign81830_e122004);
        let assign81830_e122007: f64 = (assign81830_e122005 - locals.var_nqs_a);
        let assign81830_e122008: f64 = (locals.var_nqs_tau * assign81830_e122007);
        let assign81830_e122009: f64 = (assign81830_e121999 + assign81830_e122008);
        (assign81830_e122009, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_nqs_tau_dn5 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5))) - locals.var_nqs_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_nqs_tau_dn6 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6))) - locals.var_nqs_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_nqs_tau_dn7 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7))) - locals.var_nqs_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_nqs_tau_dn8 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8))) - locals.var_nqs_a_dn8)))), (((locals.var_nu_dn12 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn12)) + ((locals.var_nqs_tau_dn12 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12))) - locals.var_nqs_a_dn12)))), (((locals.var_nu_dn13 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn13)) + ((locals.var_nqs_tau_dn13 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13))) - locals.var_nqs_a_dn13)))), (((locals.var_nu_dn14 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn14)) + ((locals.var_nqs_tau_dn14 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14))) - locals.var_nqs_a_dn14)))), (((locals.var_nu_dn15 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn15)) + ((locals.var_nqs_tau_dn15 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15))) - locals.var_nqs_a_dn15)))), (((locals.var_nu_dn16 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn16)) + ((locals.var_nqs_tau_dn16 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16))) - locals.var_nqs_a_dn16)))), (((locals.var_nu_dn17 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn17)) + ((locals.var_nqs_tau_dn17 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17))) - locals.var_nqs_a_dn17)))), (((locals.var_nu_dn18 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn18)) + ((locals.var_nqs_tau_dn18 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18))) - locals.var_nqs_a_dn18)))), (((locals.var_nu_dn19 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn19)) + ((locals.var_nqs_tau_dn19 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19))) - locals.var_nqs_a_dn19)))), (((locals.var_nu_dn20 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn20)) + ((locals.var_nqs_tau_dn20 * assign81830_e122007) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20))) - locals.var_nqs_a_dn20)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn12, locals.var_mutau_dn13, locals.var_mutau_dn14, locals.var_mutau_dn15, locals.var_mutau_dn16, locals.var_mutau_dn17, locals.var_mutau_dn18, locals.var_mutau_dn19, locals.var_mutau_dn20,)
    }
};
        locals.var_mutau = assign81830_e122011;
        locals.var_mutau_dn5 = assign81830_e122011_d_n5;
        locals.var_mutau_dn6 = assign81830_e122011_d_n6;
        locals.var_mutau_dn7 = assign81830_e122011_d_n7;
        locals.var_mutau_dn8 = assign81830_e122011_d_n8;
        locals.var_mutau_dn12 = assign81830_e122011_d_n12;
        locals.var_mutau_dn13 = assign81830_e122011_d_n13;
        locals.var_mutau_dn14 = assign81830_e122011_d_n14;
        locals.var_mutau_dn15 = assign81830_e122011_d_n15;
        locals.var_mutau_dn16 = assign81830_e122011_d_n16;
        locals.var_mutau_dn17 = assign81830_e122011_d_n17;
        locals.var_mutau_dn18 = assign81830_e122011_d_n18;
        locals.var_mutau_dn19 = assign81830_e122011_d_n19;
        locals.var_mutau_dn20 = assign81830_e122011_d_n20;

        let (assign81840_e122060, assign81840_e122060_d_n5, assign81840_e122060_d_n6, assign81840_e122060_d_n7, assign81840_e122060_d_n8, assign81840_e122060_d_n12, assign81840_e122060_d_n13, assign81840_e122060_d_n14, assign81840_e122060_d_n15, assign81840_e122060_d_n16, assign81840_e122060_d_n17, assign81840_e122060_d_n18, assign81840_e122060_d_n19, assign81840_e122060_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81840_e122035: f64 = (locals.var_nqs_a * locals.var_nu);
        let assign81840_e122037: f64 = (assign81840_e122035 * locals.var_nqs_tau);
        let assign81840_e122041: f64 = (locals.var_nu / locals.var_mutau);
        let assign81840_e122043: f64 = (assign81840_e122041 * locals.var_nqs_tau);
        let assign81840_e122045: f64 = (assign81840_e122043 * locals.var_nqs_tau);
        let assign81840_e122047: f64 = (assign81840_e122045 * locals.var_nqs_c);
        let assign81840_e122050: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign81840_e122052: f64 = (assign81840_e122050 * 0.3333333333333333);
        let assign81840_e122054: f64 = (assign81840_e122052 - locals.var_nqs_a);
        let assign81840_e122055: f64 = (assign81840_e122047 * assign81840_e122054);
        let assign81840_e122056: f64 = (locals.var_mutau + assign81840_e122055);
        let assign81840_e122057: f64 = (assign81840_e122037 / assign81840_e122056);
        let assign81840_e122058: f64 = (locals.var_nqs_eta + assign81840_e122057);
        (assign81840_e122058, (locals.var_nqs_eta_dn5 + (((((((locals.var_nqs_a_dn5 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn5)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn5)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn5)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn5)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn5)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5)) * 0.3333333333333333) - locals.var_nqs_a_dn5)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn6 + (((((((locals.var_nqs_a_dn6 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn6)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn6)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn6)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn6)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn6)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6)) * 0.3333333333333333) - locals.var_nqs_a_dn6)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn7 + (((((((locals.var_nqs_a_dn7 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn7)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn7)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn7)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn7)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn7)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7)) * 0.3333333333333333) - locals.var_nqs_a_dn7)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn8 + (((((((locals.var_nqs_a_dn8 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn8)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn8)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn8)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn8)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn8)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8)) * 0.3333333333333333) - locals.var_nqs_a_dn8)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn12 + (((((((locals.var_nqs_a_dn12 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn12)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn12)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn12 + (((((((((((locals.var_nu_dn12 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn12)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn12)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn12)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn12)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12)) * 0.3333333333333333) - locals.var_nqs_a_dn12)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn13 + (((((((locals.var_nqs_a_dn13 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn13)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn13)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn13 + (((((((((((locals.var_nu_dn13 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn13)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn13)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn13)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn13)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13)) * 0.3333333333333333) - locals.var_nqs_a_dn13)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn14 + (((((((locals.var_nqs_a_dn14 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn14)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn14)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn14 + (((((((((((locals.var_nu_dn14 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn14)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn14)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn14)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn14)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14)) * 0.3333333333333333) - locals.var_nqs_a_dn14)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn15 + (((((((locals.var_nqs_a_dn15 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn15)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn15)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn15 + (((((((((((locals.var_nu_dn15 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn15)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn15)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn15)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn15)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15)) * 0.3333333333333333) - locals.var_nqs_a_dn15)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn16 + (((((((locals.var_nqs_a_dn16 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn16)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn16)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn16 + (((((((((((locals.var_nu_dn16 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn16)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn16)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn16)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn16)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16)) * 0.3333333333333333) - locals.var_nqs_a_dn16)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn17 + (((((((locals.var_nqs_a_dn17 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn17)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn17)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn17 + (((((((((((locals.var_nu_dn17 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn17)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn17)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn17)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn17)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17)) * 0.3333333333333333) - locals.var_nqs_a_dn17)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn18 + (((((((locals.var_nqs_a_dn18 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn18)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn18)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn18 + (((((((((((locals.var_nu_dn18 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn18)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn18)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn18)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn18)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18)) * 0.3333333333333333) - locals.var_nqs_a_dn18)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn19 + (((((((locals.var_nqs_a_dn19 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn19)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn19)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn19 + (((((((((((locals.var_nu_dn19 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn19)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn19)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn19)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn19)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19)) * 0.3333333333333333) - locals.var_nqs_a_dn19)))))) / (assign81840_e122056 * assign81840_e122056))), (locals.var_nqs_eta_dn20 + (((((((locals.var_nqs_a_dn20 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn20)) * locals.var_nqs_tau) + (assign81840_e122035 * locals.var_nqs_tau_dn20)) * assign81840_e122056) - (assign81840_e122037 * (locals.var_mutau_dn20 + (((((((((((locals.var_nu_dn20 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn20)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign81840_e122041 * locals.var_nqs_tau_dn20)) * locals.var_nqs_tau) + (assign81840_e122043 * locals.var_nqs_tau_dn20)) * locals.var_nqs_c) + (assign81840_e122045 * locals.var_nqs_c_dn20)) * assign81840_e122054) + (assign81840_e122047 * ((((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20)) * 0.3333333333333333) - locals.var_nqs_a_dn20)))))) / (assign81840_e122056 * assign81840_e122056))),)
    } else {
        (locals.var_nqs_y0, locals.var_nqs_y0_dn5, locals.var_nqs_y0_dn6, locals.var_nqs_y0_dn7, locals.var_nqs_y0_dn8, locals.var_nqs_y0_dn12, locals.var_nqs_y0_dn13, locals.var_nqs_y0_dn14, locals.var_nqs_y0_dn15, locals.var_nqs_y0_dn16, locals.var_nqs_y0_dn17, locals.var_nqs_y0_dn18, locals.var_nqs_y0_dn19, locals.var_nqs_y0_dn20,)
    }
};
        locals.var_nqs_y0 = assign81840_e122060;
        locals.var_nqs_y0_dn5 = assign81840_e122060_d_n5;
        locals.var_nqs_y0_dn6 = assign81840_e122060_d_n6;
        locals.var_nqs_y0_dn7 = assign81840_e122060_d_n7;
        locals.var_nqs_y0_dn8 = assign81840_e122060_d_n8;
        locals.var_nqs_y0_dn12 = assign81840_e122060_d_n12;
        locals.var_nqs_y0_dn13 = assign81840_e122060_d_n13;
        locals.var_nqs_y0_dn14 = assign81840_e122060_d_n14;
        locals.var_nqs_y0_dn15 = assign81840_e122060_d_n15;
        locals.var_nqs_y0_dn16 = assign81840_e122060_d_n16;
        locals.var_nqs_y0_dn17 = assign81840_e122060_d_n17;
        locals.var_nqs_y0_dn18 = assign81840_e122060_d_n18;
        locals.var_nqs_y0_dn19 = assign81840_e122060_d_n19;
        locals.var_nqs_y0_dn20 = assign81840_e122060_d_n20;

        let assign81850_e122062: f64 = (locals.var_nqs_y0).abs();
        let assign81850_e122064: f64 = if assign81850_e122062 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign81850_e122064;

        let (assign81860_e122090, assign81860_e122090_d_n5, assign81860_e122090_d_n6, assign81860_e122090_d_n7, assign81860_e122090_d_n8, assign81860_e122090_d_n12, assign81860_e122090_d_n13, assign81860_e122090_d_n14, assign81860_e122090_d_n15, assign81860_e122090_d_n16, assign81860_e122090_d_n17, assign81860_e122090_d_n18, assign81860_e122090_d_n19, assign81860_e122090_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 != 0.0)) {
        let assign81860_e122088: f64 = (locals.var_nqs_y0).exp();
        (assign81860_e122088, (assign81860_e122088 * locals.var_nqs_y0_dn5), (assign81860_e122088 * locals.var_nqs_y0_dn6), (assign81860_e122088 * locals.var_nqs_y0_dn7), (assign81860_e122088 * locals.var_nqs_y0_dn8), (assign81860_e122088 * locals.var_nqs_y0_dn12), (assign81860_e122088 * locals.var_nqs_y0_dn13), (assign81860_e122088 * locals.var_nqs_y0_dn14), (assign81860_e122088 * locals.var_nqs_y0_dn15), (assign81860_e122088 * locals.var_nqs_y0_dn16), (assign81860_e122088 * locals.var_nqs_y0_dn17), (assign81860_e122088 * locals.var_nqs_y0_dn18), (assign81860_e122088 * locals.var_nqs_y0_dn19), (assign81860_e122088 * locals.var_nqs_y0_dn20),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81860_e122090;
        locals.var_nqs_d0_dn5 = assign81860_e122090_d_n5;
        locals.var_nqs_d0_dn6 = assign81860_e122090_d_n6;
        locals.var_nqs_d0_dn7 = assign81860_e122090_d_n7;
        locals.var_nqs_d0_dn8 = assign81860_e122090_d_n8;
        locals.var_nqs_d0_dn12 = assign81860_e122090_d_n12;
        locals.var_nqs_d0_dn13 = assign81860_e122090_d_n13;
        locals.var_nqs_d0_dn14 = assign81860_e122090_d_n14;
        locals.var_nqs_d0_dn15 = assign81860_e122090_d_n15;
        locals.var_nqs_d0_dn16 = assign81860_e122090_d_n16;
        locals.var_nqs_d0_dn17 = assign81860_e122090_d_n17;
        locals.var_nqs_d0_dn18 = assign81860_e122090_d_n18;
        locals.var_nqs_d0_dn19 = assign81860_e122090_d_n19;
        locals.var_nqs_d0_dn20 = assign81860_e122090_d_n20;

        let assign81870_e122093: f64 = if locals.var_nqs_y0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign81870_e122093;

        let (assign81880_e122146, assign81880_e122146_d_n5, assign81880_e122146_d_n6, assign81880_e122146_d_n7, assign81880_e122146_d_n8, assign81880_e122146_d_n12, assign81880_e122146_d_n13, assign81880_e122146_d_n14, assign81880_e122146_d_n15, assign81880_e122146_d_n16, assign81880_e122146_d_n17, assign81880_e122146_d_n18, assign81880_e122146_d_n19, assign81880_e122146_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 != 0.0)) {
        let assign81880_e122122: f64 = (-230.25850929940458);
        let assign81880_e122124: f64 = (assign81880_e122122 - locals.var_nqs_y0);
        let assign81880_e122128: f64 = (-230.25850929940458);
        let assign81880_e122130: f64 = (assign81880_e122128 - locals.var_nqs_y0);
        let assign81880_e122133: f64 = (-230.25850929940458);
        let assign81880_e122135: f64 = (assign81880_e122133 - locals.var_nqs_y0);
        let assign81880_e122137: f64 = (assign81880_e122135 * 0.3333333333333333);
        let assign81880_e122138: f64 = (1.0 + assign81880_e122137);
        let assign81880_e122139: f64 = (assign81880_e122130 * assign81880_e122138);
        let assign81880_e122140: f64 = (0.5 * assign81880_e122139);
        let assign81880_e122141: f64 = (1.0 + assign81880_e122140);
        let assign81880_e122142: f64 = (assign81880_e122124 * assign81880_e122141);
        let assign81880_e122143: f64 = (1.0 + assign81880_e122142);
        let assign81880_e122144: f64 = (1e-100 / assign81880_e122143);
        (assign81880_e122144, (-((1e-100 * (((-locals.var_nqs_y0_dn5) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn5) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn5) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn6) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn6) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn6) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn7) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn7) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn7) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn8) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn8) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn8) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn12) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn12) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn12) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn13) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn13) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn13) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn14) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn14) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn14) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn15) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn15) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn15) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn16) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn16) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn16) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn17) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn17) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn17) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn18) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn18) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn18) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn19) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn19) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn19) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn20) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn20) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn20) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81880_e122146;
        locals.var_nqs_d0_dn5 = assign81880_e122146_d_n5;
        locals.var_nqs_d0_dn6 = assign81880_e122146_d_n6;
        locals.var_nqs_d0_dn7 = assign81880_e122146_d_n7;
        locals.var_nqs_d0_dn8 = assign81880_e122146_d_n8;
        locals.var_nqs_d0_dn12 = assign81880_e122146_d_n12;
        locals.var_nqs_d0_dn13 = assign81880_e122146_d_n13;
        locals.var_nqs_d0_dn14 = assign81880_e122146_d_n14;
        locals.var_nqs_d0_dn15 = assign81880_e122146_d_n15;
        locals.var_nqs_d0_dn16 = assign81880_e122146_d_n16;
        locals.var_nqs_d0_dn17 = assign81880_e122146_d_n17;
        locals.var_nqs_d0_dn18 = assign81880_e122146_d_n18;
        locals.var_nqs_d0_dn19 = assign81880_e122146_d_n19;
        locals.var_nqs_d0_dn20 = assign81880_e122146_d_n20;

    }

    pub(super) fn stamp_transient_block_161(
        locals: &mut StampLocals,
    ) {
        let (assign81890_e122197, assign81890_e122197_d_n5, assign81890_e122197_d_n6, assign81890_e122197_d_n7, assign81890_e122197_d_n8, assign81890_e122197_d_n12, assign81890_e122197_d_n13, assign81890_e122197_d_n14, assign81890_e122197_d_n15, assign81890_e122197_d_n16, assign81890_e122197_d_n17, assign81890_e122197_d_n18, assign81890_e122197_d_n19, assign81890_e122197_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 == 0.0)) {
        let assign81890_e122177: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122182: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122186: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122188: f64 = (assign81890_e122186 * 0.3333333333333333);
        let assign81890_e122189: f64 = (1.0 + assign81890_e122188);
        let assign81890_e122190: f64 = (assign81890_e122182 * assign81890_e122189);
        let assign81890_e122191: f64 = (0.5 * assign81890_e122190);
        let assign81890_e122192: f64 = (1.0 + assign81890_e122191);
        let assign81890_e122193: f64 = (assign81890_e122177 * assign81890_e122192);
        let assign81890_e122194: f64 = (1.0 + assign81890_e122193);
        let assign81890_e122195: f64 = (1e100 * assign81890_e122194);
        (assign81890_e122195, (1e100 * ((locals.var_nqs_y0_dn5 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn5 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn6 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn6 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn7 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn7 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn8 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn8 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn12 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn12 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn13 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn13 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn14 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn14 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn15 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn15 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn16 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn16 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn17 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn17 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn18 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn18 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn19 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn19 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn20 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn20 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81890_e122197;
        locals.var_nqs_d0_dn5 = assign81890_e122197_d_n5;
        locals.var_nqs_d0_dn6 = assign81890_e122197_d_n6;
        locals.var_nqs_d0_dn7 = assign81890_e122197_d_n7;
        locals.var_nqs_d0_dn8 = assign81890_e122197_d_n8;
        locals.var_nqs_d0_dn12 = assign81890_e122197_d_n12;
        locals.var_nqs_d0_dn13 = assign81890_e122197_d_n13;
        locals.var_nqs_d0_dn14 = assign81890_e122197_d_n14;
        locals.var_nqs_d0_dn15 = assign81890_e122197_d_n15;
        locals.var_nqs_d0_dn16 = assign81890_e122197_d_n16;
        locals.var_nqs_d0_dn17 = assign81890_e122197_d_n17;
        locals.var_nqs_d0_dn18 = assign81890_e122197_d_n18;
        locals.var_nqs_d0_dn19 = assign81890_e122197_d_n19;
        locals.var_nqs_d0_dn20 = assign81890_e122197_d_n20;

        let (assign81900_e122226, assign81900_e122226_d_n5, assign81900_e122226_d_n6, assign81900_e122226_d_n7, assign81900_e122226_d_n8, assign81900_e122226_d_n12, assign81900_e122226_d_n13, assign81900_e122226_d_n14, assign81900_e122226_d_n15, assign81900_e122226_d_n16, assign81900_e122226_d_n17, assign81900_e122226_d_n18, assign81900_e122226_d_n19, assign81900_e122226_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81900_e122221: f64 = (locals.var_gp2 * locals.var_nqs_d0);
        let assign81900_e122223: f64 = (assign81900_e122221 * 0.5);
        let assign81900_e122224: f64 = (1.0 - assign81900_e122223);
        (assign81900_e122224, (-(((locals.var_gp2_dn5 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn5)) * 0.5)), (-(((locals.var_gp2_dn6 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn6)) * 0.5)), (-(((locals.var_gp2_dn7 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn7)) * 0.5)), (-(((locals.var_gp2_dn8 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn8)) * 0.5)), (-(((locals.var_gp2_dn12 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn12)) * 0.5)), (-(((locals.var_gp2_dn13 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn13)) * 0.5)), (-(((locals.var_gp2_dn14 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn14)) * 0.5)), (-(((locals.var_gp2_dn15 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn15)) * 0.5)), (-(((locals.var_gp2_dn16 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn16)) * 0.5)), (-(((locals.var_gp2_dn17 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn17)) * 0.5)), (-(((locals.var_gp2_dn18 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn18)) * 0.5)), (-(((locals.var_gp2_dn19 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn19)) * 0.5)), (-(((locals.var_gp2_dn20 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn20)) * 0.5)),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign81900_e122226;
        locals.var_nqs_xi_dn5 = assign81900_e122226_d_n5;
        locals.var_nqs_xi_dn6 = assign81900_e122226_d_n6;
        locals.var_nqs_xi_dn7 = assign81900_e122226_d_n7;
        locals.var_nqs_xi_dn8 = assign81900_e122226_d_n8;
        locals.var_nqs_xi_dn12 = assign81900_e122226_d_n12;
        locals.var_nqs_xi_dn13 = assign81900_e122226_d_n13;
        locals.var_nqs_xi_dn14 = assign81900_e122226_d_n14;
        locals.var_nqs_xi_dn15 = assign81900_e122226_d_n15;
        locals.var_nqs_xi_dn16 = assign81900_e122226_d_n16;
        locals.var_nqs_xi_dn17 = assign81900_e122226_d_n17;
        locals.var_nqs_xi_dn18 = assign81900_e122226_d_n18;
        locals.var_nqs_xi_dn19 = assign81900_e122226_d_n19;
        locals.var_nqs_xi_dn20 = assign81900_e122226_d_n20;

        let (assign81910_e122259, assign81910_e122259_d_n5, assign81910_e122259_d_n6, assign81910_e122259_d_n7, assign81910_e122259_d_n8, assign81910_e122259_d_n12, assign81910_e122259_d_n13, assign81910_e122259_d_n14, assign81910_e122259_d_n15, assign81910_e122259_d_n16, assign81910_e122259_d_n17, assign81910_e122259_d_n18, assign81910_e122259_d_n19, assign81910_e122259_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81910_e122250: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81910_e122251: f64 = (2.0 * assign81910_e122250);
        let assign81910_e122255: f64 = (locals.var_nqs_d0 - 1.0);
        let assign81910_e122256: f64 = (locals.var_gp2 * assign81910_e122255);
        let assign81910_e122257: f64 = (assign81910_e122251 + assign81910_e122256);
        (assign81910_e122257, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5)) + ((locals.var_gp2_dn5 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn5))), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6)) + ((locals.var_gp2_dn6 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn6))), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7)) + ((locals.var_gp2_dn7 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn7))), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8)) + ((locals.var_gp2_dn8 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn8))), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12)) + ((locals.var_gp2_dn12 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn12))), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13)) + ((locals.var_gp2_dn13 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn13))), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14)) + ((locals.var_gp2_dn14 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn14))), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15)) + ((locals.var_gp2_dn15 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn15))), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16)) + ((locals.var_gp2_dn16 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn16))), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17)) + ((locals.var_gp2_dn17 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn17))), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18)) + ((locals.var_gp2_dn18 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn18))), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19)) + ((locals.var_gp2_dn19 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn19))), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20)) + ((locals.var_gp2_dn20 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign81910_e122259;
        locals.var_nqs_p_dn5 = assign81910_e122259_d_n5;
        locals.var_nqs_p_dn6 = assign81910_e122259_d_n6;
        locals.var_nqs_p_dn7 = assign81910_e122259_d_n7;
        locals.var_nqs_p_dn8 = assign81910_e122259_d_n8;
        locals.var_nqs_p_dn12 = assign81910_e122259_d_n12;
        locals.var_nqs_p_dn13 = assign81910_e122259_d_n13;
        locals.var_nqs_p_dn14 = assign81910_e122259_d_n14;
        locals.var_nqs_p_dn15 = assign81910_e122259_d_n15;
        locals.var_nqs_p_dn16 = assign81910_e122259_d_n16;
        locals.var_nqs_p_dn17 = assign81910_e122259_d_n17;
        locals.var_nqs_p_dn18 = assign81910_e122259_d_n18;
        locals.var_nqs_p_dn19 = assign81910_e122259_d_n19;
        locals.var_nqs_p_dn20 = assign81910_e122259_d_n20;

        let (assign81920_e122296, assign81920_e122296_d_n5, assign81920_e122296_d_n6, assign81920_e122296_d_n7, assign81920_e122296_d_n8, assign81920_e122296_d_n12, assign81920_e122296_d_n13, assign81920_e122296_d_n14, assign81920_e122296_d_n15, assign81920_e122296_d_n16, assign81920_e122296_d_n17, assign81920_e122296_d_n18, assign81920_e122296_d_n19, assign81920_e122296_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81920_e122282: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81920_e122285: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81920_e122286: f64 = (assign81920_e122282 * assign81920_e122285);
        let assign81920_e122290: f64 = (locals.var_nqs_y0 + 1.0);
        let assign81920_e122292: f64 = (assign81920_e122290 - locals.var_nqs_d0);
        let assign81920_e122293: f64 = (locals.var_gp2 * assign81920_e122292);
        let assign81920_e122294: f64 = (assign81920_e122286 + assign81920_e122293);
        (assign81920_e122294, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5))) + ((locals.var_gp2_dn5 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn5 - locals.var_nqs_d0_dn5)))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6))) + ((locals.var_gp2_dn6 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn6 - locals.var_nqs_d0_dn6)))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7))) + ((locals.var_gp2_dn7 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn7 - locals.var_nqs_d0_dn7)))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8))) + ((locals.var_gp2_dn8 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn8 - locals.var_nqs_d0_dn8)))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12))) + ((locals.var_gp2_dn12 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn12 - locals.var_nqs_d0_dn12)))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13))) + ((locals.var_gp2_dn13 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn13 - locals.var_nqs_d0_dn13)))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14))) + ((locals.var_gp2_dn14 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn14 - locals.var_nqs_d0_dn14)))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15))) + ((locals.var_gp2_dn15 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn15 - locals.var_nqs_d0_dn15)))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16))) + ((locals.var_gp2_dn16 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn16 - locals.var_nqs_d0_dn16)))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17))) + ((locals.var_gp2_dn17 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn17 - locals.var_nqs_d0_dn17)))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18))) + ((locals.var_gp2_dn18 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn18 - locals.var_nqs_d0_dn18)))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19))) + ((locals.var_gp2_dn19 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn19 - locals.var_nqs_d0_dn19)))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20))) + ((locals.var_gp2_dn20 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn20 - locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign81920_e122296;
        locals.var_nqs_q_dn5 = assign81920_e122296_d_n5;
        locals.var_nqs_q_dn6 = assign81920_e122296_d_n6;
        locals.var_nqs_q_dn7 = assign81920_e122296_d_n7;
        locals.var_nqs_q_dn8 = assign81920_e122296_d_n8;
        locals.var_nqs_q_dn12 = assign81920_e122296_d_n12;
        locals.var_nqs_q_dn13 = assign81920_e122296_d_n13;
        locals.var_nqs_q_dn14 = assign81920_e122296_d_n14;
        locals.var_nqs_q_dn15 = assign81920_e122296_d_n15;
        locals.var_nqs_q_dn16 = assign81920_e122296_d_n16;
        locals.var_nqs_q_dn17 = assign81920_e122296_d_n17;
        locals.var_nqs_q_dn18 = assign81920_e122296_d_n18;
        locals.var_nqs_q_dn19 = assign81920_e122296_d_n19;
        locals.var_nqs_q_dn20 = assign81920_e122296_d_n20;

        let (assign81930_e122327, assign81930_e122327_d_n5, assign81930_e122327_d_n6, assign81930_e122327_d_n7, assign81930_e122327_d_n8, assign81930_e122327_d_n12, assign81930_e122327_d_n13, assign81930_e122327_d_n14, assign81930_e122327_d_n15, assign81930_e122327_d_n16, assign81930_e122327_d_n17, assign81930_e122327_d_n18, assign81930_e122327_d_n19, assign81930_e122327_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81930_e122319: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign81930_e122322: f64 = (4.0 * locals.var_nqs_xi);
        let assign81930_e122324: f64 = (assign81930_e122322 * locals.var_nqs_q);
        let assign81930_e122325: f64 = (assign81930_e122319 - assign81930_e122324);
        (assign81930_e122325, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign81930_e122327;
        locals.var_nqs_temp_dn5 = assign81930_e122327_d_n5;
        locals.var_nqs_temp_dn6 = assign81930_e122327_d_n6;
        locals.var_nqs_temp_dn7 = assign81930_e122327_d_n7;
        locals.var_nqs_temp_dn8 = assign81930_e122327_d_n8;
        locals.var_nqs_temp_dn12 = assign81930_e122327_d_n12;
        locals.var_nqs_temp_dn13 = assign81930_e122327_d_n13;
        locals.var_nqs_temp_dn14 = assign81930_e122327_d_n14;
        locals.var_nqs_temp_dn15 = assign81930_e122327_d_n15;
        locals.var_nqs_temp_dn16 = assign81930_e122327_d_n16;
        locals.var_nqs_temp_dn17 = assign81930_e122327_d_n17;
        locals.var_nqs_temp_dn18 = assign81930_e122327_d_n18;
        locals.var_nqs_temp_dn19 = assign81930_e122327_d_n19;
        locals.var_nqs_temp_dn20 = assign81930_e122327_d_n20;

        let (assign81940_e122357, assign81940_e122357_d_n5, assign81940_e122357_d_n6, assign81940_e122357_d_n7, assign81940_e122357_d_n8, assign81940_e122357_d_n12, assign81940_e122357_d_n13, assign81940_e122357_d_n14, assign81940_e122357_d_n15, assign81940_e122357_d_n16, assign81940_e122357_d_n17, assign81940_e122357_d_n18, assign81940_e122357_d_n19, assign81940_e122357_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81940_e122350: f64 = (2.0 * locals.var_nqs_q);
        let assign81940_e122353: f64 = (locals.var_nqs_temp).sqrt();
        let assign81940_e122354: f64 = (locals.var_nqs_p + assign81940_e122353);
        let assign81940_e122355: f64 = (assign81940_e122350 / assign81940_e122354);
        (assign81940_e122355, ((((2.0 * locals.var_nqs_q_dn5) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn6) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn7) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn8) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn12) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn13) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn14) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn15) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn16) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn17) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn18) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn19) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn20) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign81940_e122357;
        locals.var_nqs_w_dn5 = assign81940_e122357_d_n5;
        locals.var_nqs_w_dn6 = assign81940_e122357_d_n6;
        locals.var_nqs_w_dn7 = assign81940_e122357_d_n7;
        locals.var_nqs_w_dn8 = assign81940_e122357_d_n8;
        locals.var_nqs_w_dn12 = assign81940_e122357_d_n12;
        locals.var_nqs_w_dn13 = assign81940_e122357_d_n13;
        locals.var_nqs_w_dn14 = assign81940_e122357_d_n14;
        locals.var_nqs_w_dn15 = assign81940_e122357_d_n15;
        locals.var_nqs_w_dn16 = assign81940_e122357_d_n16;
        locals.var_nqs_w_dn17 = assign81940_e122357_d_n17;
        locals.var_nqs_w_dn18 = assign81940_e122357_d_n18;
        locals.var_nqs_w_dn19 = assign81940_e122357_d_n19;
        locals.var_nqs_w_dn20 = assign81940_e122357_d_n20;

        let (assign81950_e122383, assign81950_e122383_d_n5, assign81950_e122383_d_n6, assign81950_e122383_d_n7, assign81950_e122383_d_n8, assign81950_e122383_d_n12, assign81950_e122383_d_n13, assign81950_e122383_d_n14, assign81950_e122383_d_n15, assign81950_e122383_d_n16, assign81950_e122383_d_n17, assign81950_e122383_d_n18, assign81950_e122383_d_n19, assign81950_e122383_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81950_e122380: f64 = (locals.var_nqs_y0 + locals.var_nqs_w);
        let assign81950_e122381: f64 = (-assign81950_e122380);
        (assign81950_e122381, (-(locals.var_nqs_y0_dn5 + locals.var_nqs_w_dn5)), (-(locals.var_nqs_y0_dn6 + locals.var_nqs_w_dn6)), (-(locals.var_nqs_y0_dn7 + locals.var_nqs_w_dn7)), (-(locals.var_nqs_y0_dn8 + locals.var_nqs_w_dn8)), (-(locals.var_nqs_y0_dn12 + locals.var_nqs_w_dn12)), (-(locals.var_nqs_y0_dn13 + locals.var_nqs_w_dn13)), (-(locals.var_nqs_y0_dn14 + locals.var_nqs_w_dn14)), (-(locals.var_nqs_y0_dn15 + locals.var_nqs_w_dn15)), (-(locals.var_nqs_y0_dn16 + locals.var_nqs_w_dn16)), (-(locals.var_nqs_y0_dn17 + locals.var_nqs_w_dn17)), (-(locals.var_nqs_y0_dn18 + locals.var_nqs_w_dn18)), (-(locals.var_nqs_y0_dn19 + locals.var_nqs_w_dn19)), (-(locals.var_nqs_y0_dn20 + locals.var_nqs_w_dn20)),)
    } else {
        (locals.var_temp8, locals.var_temp8_dn5, locals.var_temp8_dn6, locals.var_temp8_dn7, locals.var_temp8_dn8, locals.var_temp8_dn12, locals.var_temp8_dn13, locals.var_temp8_dn14, locals.var_temp8_dn15, locals.var_temp8_dn16, locals.var_temp8_dn17, locals.var_temp8_dn18, locals.var_temp8_dn19, locals.var_temp8_dn20,)
    }
};
        locals.var_temp8 = assign81950_e122383;
        locals.var_temp8_dn5 = assign81950_e122383_d_n5;
        locals.var_temp8_dn6 = assign81950_e122383_d_n6;
        locals.var_temp8_dn7 = assign81950_e122383_d_n7;
        locals.var_temp8_dn8 = assign81950_e122383_d_n8;
        locals.var_temp8_dn12 = assign81950_e122383_d_n12;
        locals.var_temp8_dn13 = assign81950_e122383_d_n13;
        locals.var_temp8_dn14 = assign81950_e122383_d_n14;
        locals.var_temp8_dn15 = assign81950_e122383_d_n15;
        locals.var_temp8_dn16 = assign81950_e122383_d_n16;
        locals.var_temp8_dn17 = assign81950_e122383_d_n17;
        locals.var_temp8_dn18 = assign81950_e122383_d_n18;
        locals.var_temp8_dn19 = assign81950_e122383_d_n19;
        locals.var_temp8_dn20 = assign81950_e122383_d_n20;

        let (assign81960_e122413, assign81960_e122413_d_n5, assign81960_e122413_d_n6, assign81960_e122413_d_n7, assign81960_e122413_d_n8, assign81960_e122413_d_n12, assign81960_e122413_d_n13, assign81960_e122413_d_n14, assign81960_e122413_d_n15, assign81960_e122413_d_n16, assign81960_e122413_d_n17, assign81960_e122413_d_n18, assign81960_e122413_d_n19, assign81960_e122413_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81960_e122409: f64 = (0.732464877560822 * locals.var_gp);
        let assign81960_e122410: f64 = (1.25 + assign81960_e122409);
        let assign81960_e122411: f64 = (1.0 / assign81960_e122410);
        (assign81960_e122411, (-((0.732464877560822 * locals.var_gp_dn5) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn6) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn7) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn8) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn12) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn13) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn14) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn15) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn16) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn17) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn18) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn19) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn20) / (assign81960_e122410 * assign81960_e122410))),)
    } else {
        (locals.var_nqs_xg1, locals.var_nqs_xg1_dn5, locals.var_nqs_xg1_dn6, locals.var_nqs_xg1_dn7, locals.var_nqs_xg1_dn8, locals.var_nqs_xg1_dn12, locals.var_nqs_xg1_dn13, locals.var_nqs_xg1_dn14, locals.var_nqs_xg1_dn15, locals.var_nqs_xg1_dn16, locals.var_nqs_xg1_dn17, locals.var_nqs_xg1_dn18, locals.var_nqs_xg1_dn19, locals.var_nqs_xg1_dn20,)
    }
};
        locals.var_nqs_xg1 = assign81960_e122413;
        locals.var_nqs_xg1_dn5 = assign81960_e122413_d_n5;
        locals.var_nqs_xg1_dn6 = assign81960_e122413_d_n6;
        locals.var_nqs_xg1_dn7 = assign81960_e122413_d_n7;
        locals.var_nqs_xg1_dn8 = assign81960_e122413_d_n8;
        locals.var_nqs_xg1_dn12 = assign81960_e122413_d_n12;
        locals.var_nqs_xg1_dn13 = assign81960_e122413_d_n13;
        locals.var_nqs_xg1_dn14 = assign81960_e122413_d_n14;
        locals.var_nqs_xg1_dn15 = assign81960_e122413_d_n15;
        locals.var_nqs_xg1_dn16 = assign81960_e122413_d_n16;
        locals.var_nqs_xg1_dn17 = assign81960_e122413_d_n17;
        locals.var_nqs_xg1_dn18 = assign81960_e122413_d_n18;
        locals.var_nqs_xg1_dn19 = assign81960_e122413_d_n19;
        locals.var_nqs_xg1_dn20 = assign81960_e122413_d_n20;

        let (assign81970_e122445, assign81970_e122445_d_n5, assign81970_e122445_d_n6, assign81970_e122445_d_n7, assign81970_e122445_d_n8, assign81970_e122445_d_n12, assign81970_e122445_d_n13, assign81970_e122445_d_n14, assign81970_e122445_d_n15, assign81970_e122445_d_n16, assign81970_e122445_d_n17, assign81970_e122445_d_n18, assign81970_e122445_d_n19, assign81970_e122445_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81970_e122437: f64 = (1.25 * locals.var_a_factrp);
        let assign81970_e122439: f64 = (assign81970_e122437 * locals.var_nqs_xg1);
        let assign81970_e122441: f64 = (assign81970_e122439 - 1.0);
        let assign81970_e122443: f64 = (assign81970_e122441 * locals.var_nqs_xg1);
        (assign81970_e122443, (((((1.25 * locals.var_a_factrp_dn5) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn5)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn5)), (((((1.25 * locals.var_a_factrp_dn6) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn6)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn6)), (((((1.25 * locals.var_a_factrp_dn7) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn7)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn7)), (((((1.25 * locals.var_a_factrp_dn8) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn8)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn8)), (((((1.25 * locals.var_a_factrp_dn12) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn12)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn12)), (((((1.25 * locals.var_a_factrp_dn13) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn13)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn13)), (((((1.25 * locals.var_a_factrp_dn14) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn14)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn14)), (((((1.25 * locals.var_a_factrp_dn15) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn15)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn15)), (((((1.25 * locals.var_a_factrp_dn16) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn16)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn16)), (((((1.25 * locals.var_a_factrp_dn17) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn17)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn17)), (((((1.25 * locals.var_a_factrp_dn18) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn18)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn18)), (((((1.25 * locals.var_a_factrp_dn19) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn19)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn19)), (((((1.25 * locals.var_a_factrp_dn20) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn20)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn20)),)
    } else {
        (locals.var_nqs_a_fac, locals.var_nqs_a_fac_dn5, locals.var_nqs_a_fac_dn6, locals.var_nqs_a_fac_dn7, locals.var_nqs_a_fac_dn8, locals.var_nqs_a_fac_dn12, locals.var_nqs_a_fac_dn13, locals.var_nqs_a_fac_dn14, locals.var_nqs_a_fac_dn15, locals.var_nqs_a_fac_dn16, locals.var_nqs_a_fac_dn17, locals.var_nqs_a_fac_dn18, locals.var_nqs_a_fac_dn19, locals.var_nqs_a_fac_dn20,)
    }
};
        locals.var_nqs_a_fac = assign81970_e122445;
        locals.var_nqs_a_fac_dn5 = assign81970_e122445_d_n5;
        locals.var_nqs_a_fac_dn6 = assign81970_e122445_d_n6;
        locals.var_nqs_a_fac_dn7 = assign81970_e122445_d_n7;
        locals.var_nqs_a_fac_dn8 = assign81970_e122445_d_n8;
        locals.var_nqs_a_fac_dn12 = assign81970_e122445_d_n12;
        locals.var_nqs_a_fac_dn13 = assign81970_e122445_d_n13;
        locals.var_nqs_a_fac_dn14 = assign81970_e122445_d_n14;
        locals.var_nqs_a_fac_dn15 = assign81970_e122445_d_n15;
        locals.var_nqs_a_fac_dn16 = assign81970_e122445_d_n16;
        locals.var_nqs_a_fac_dn17 = assign81970_e122445_d_n17;
        locals.var_nqs_a_fac_dn18 = assign81970_e122445_d_n18;
        locals.var_nqs_a_fac_dn19 = assign81970_e122445_d_n19;
        locals.var_nqs_a_fac_dn20 = assign81970_e122445_d_n20;

        let (assign81980_e122477, assign81980_e122477_d_n5, assign81980_e122477_d_n6, assign81980_e122477_d_n7, assign81980_e122477_d_n8, assign81980_e122477_d_n12, assign81980_e122477_d_n13, assign81980_e122477_d_n14, assign81980_e122477_d_n15, assign81980_e122477_d_n16, assign81980_e122477_d_n17, assign81980_e122477_d_n18, assign81980_e122477_d_n19, assign81980_e122477_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81980_e122469: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        let assign81980_e122473: f64 = (locals.var_nqs_a_fac * locals.var_temp__blk1038);
        let assign81980_e122474: f64 = (1.0 + assign81980_e122473);
        let assign81980_e122475: f64 = (assign81980_e122469 * assign81980_e122474);
        (assign81980_e122475, (((((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn5 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn5)))), (((((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn6 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn6)))), (((((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn7 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn7)))), (((((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn8 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn8)))), (((((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn12 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn12)))), (((((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn13 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn13)))), (((((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn14 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn14)))), (((((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn15 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn15)))), (((((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn16 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn16)))), (((((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn17 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn17)))), (((((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn18 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn18)))), (((((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn19 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn19)))), (((((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn20 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn20)))),)
    } else {
        (locals.var_nqs_xbar, locals.var_nqs_xbar_dn5, locals.var_nqs_xbar_dn6, locals.var_nqs_xbar_dn7, locals.var_nqs_xbar_dn8, locals.var_nqs_xbar_dn12, locals.var_nqs_xbar_dn13, locals.var_nqs_xbar_dn14, locals.var_nqs_xbar_dn15, locals.var_nqs_xbar_dn16, locals.var_nqs_xbar_dn17, locals.var_nqs_xbar_dn18, locals.var_nqs_xbar_dn19, locals.var_nqs_xbar_dn20,)
    }
};
        locals.var_nqs_xbar = assign81980_e122477;
        locals.var_nqs_xbar_dn5 = assign81980_e122477_d_n5;
        locals.var_nqs_xbar_dn6 = assign81980_e122477_d_n6;
        locals.var_nqs_xbar_dn7 = assign81980_e122477_d_n7;
        locals.var_nqs_xbar_dn8 = assign81980_e122477_d_n8;
        locals.var_nqs_xbar_dn12 = assign81980_e122477_d_n12;
        locals.var_nqs_xbar_dn13 = assign81980_e122477_d_n13;
        locals.var_nqs_xbar_dn14 = assign81980_e122477_d_n14;
        locals.var_nqs_xbar_dn15 = assign81980_e122477_d_n15;
        locals.var_nqs_xbar_dn16 = assign81980_e122477_d_n16;
        locals.var_nqs_xbar_dn17 = assign81980_e122477_d_n17;
        locals.var_nqs_xbar_dn18 = assign81980_e122477_d_n18;
        locals.var_nqs_xbar_dn19 = assign81980_e122477_d_n19;
        locals.var_nqs_xbar_dn20 = assign81980_e122477_d_n20;

        let assign81990_e122479: f64 = (-locals.var_nqs_xbar);
        let assign81990_e122480: f64 = (assign81990_e122479).abs();
        let assign81990_e122482: f64 = if assign81990_e122480 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign81990_e122482;

        let (assign82000_e122510, assign82000_e122510_d_n5, assign82000_e122510_d_n6, assign82000_e122510_d_n7, assign82000_e122510_d_n8, assign82000_e122510_d_n12, assign82000_e122510_d_n13, assign82000_e122510_d_n14, assign82000_e122510_d_n15, assign82000_e122510_d_n16, assign82000_e122510_d_n17, assign82000_e122510_d_n18, assign82000_e122510_d_n19, assign82000_e122510_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 != 0.0)) {
        let assign82000_e122507: f64 = (-locals.var_nqs_xbar);
        let assign82000_e122508: f64 = (assign82000_e122507).exp();
        (assign82000_e122508, (assign82000_e122508 * (-locals.var_nqs_xbar_dn5)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn6)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn7)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn8)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn12)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn13)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn14)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn15)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn16)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn17)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn18)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn19)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn20)),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82000_e122510;
        locals.var_nqs_temp_dn5 = assign82000_e122510_d_n5;
        locals.var_nqs_temp_dn6 = assign82000_e122510_d_n6;
        locals.var_nqs_temp_dn7 = assign82000_e122510_d_n7;
        locals.var_nqs_temp_dn8 = assign82000_e122510_d_n8;
        locals.var_nqs_temp_dn12 = assign82000_e122510_d_n12;
        locals.var_nqs_temp_dn13 = assign82000_e122510_d_n13;
        locals.var_nqs_temp_dn14 = assign82000_e122510_d_n14;
        locals.var_nqs_temp_dn15 = assign82000_e122510_d_n15;
        locals.var_nqs_temp_dn16 = assign82000_e122510_d_n16;
        locals.var_nqs_temp_dn17 = assign82000_e122510_d_n17;
        locals.var_nqs_temp_dn18 = assign82000_e122510_d_n18;
        locals.var_nqs_temp_dn19 = assign82000_e122510_d_n19;
        locals.var_nqs_temp_dn20 = assign82000_e122510_d_n20;

        let assign82010_e122512: f64 = (-locals.var_nqs_xbar);
        let assign82010_e122514: f64 = if assign82010_e122512 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign82010_e122514;

        let (assign82020_e122571, assign82020_e122571_d_n5, assign82020_e122571_d_n6, assign82020_e122571_d_n7, assign82020_e122571_d_n8, assign82020_e122571_d_n12, assign82020_e122571_d_n13, assign82020_e122571_d_n14, assign82020_e122571_d_n15, assign82020_e122571_d_n16, assign82020_e122571_d_n17, assign82020_e122571_d_n18, assign82020_e122571_d_n19, assign82020_e122571_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 == 0.0)) && (locals.var_guard2233 != 0.0)) {
        let assign82020_e122544: f64 = (-230.25850929940458);
        let assign82020_e122546: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122547: f64 = (assign82020_e122544 - assign82020_e122546);
        let assign82020_e122551: f64 = (-230.25850929940458);
        let assign82020_e122553: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122554: f64 = (assign82020_e122551 - assign82020_e122553);
        let assign82020_e122557: f64 = (-230.25850929940458);
        let assign82020_e122559: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122560: f64 = (assign82020_e122557 - assign82020_e122559);
        let assign82020_e122562: f64 = (assign82020_e122560 * 0.3333333333333333);
        let assign82020_e122563: f64 = (1.0 + assign82020_e122562);
        let assign82020_e122564: f64 = (assign82020_e122554 * assign82020_e122563);
        let assign82020_e122565: f64 = (0.5 * assign82020_e122564);
        let assign82020_e122566: f64 = (1.0 + assign82020_e122565);
        let assign82020_e122567: f64 = (assign82020_e122547 * assign82020_e122566);
        let assign82020_e122568: f64 = (1.0 + assign82020_e122567);
        let assign82020_e122569: f64 = (1e-100 / assign82020_e122568);
        (assign82020_e122569, (-((1e-100 * (((-(-locals.var_nqs_xbar_dn5)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn5)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn5)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn6)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn6)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn6)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn7)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn7)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn7)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn8)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn8)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn8)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn12)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn12)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn12)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn13)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn13)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn13)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn14)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn14)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn14)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn15)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn15)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn15)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn16)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn16)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn16)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn17)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn17)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn17)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn18)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn18)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn18)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn19)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn19)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn19)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn20)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn20)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn20)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82020_e122571;
        locals.var_nqs_temp_dn5 = assign82020_e122571_d_n5;
        locals.var_nqs_temp_dn6 = assign82020_e122571_d_n6;
        locals.var_nqs_temp_dn7 = assign82020_e122571_d_n7;
        locals.var_nqs_temp_dn8 = assign82020_e122571_d_n8;
        locals.var_nqs_temp_dn12 = assign82020_e122571_d_n12;
        locals.var_nqs_temp_dn13 = assign82020_e122571_d_n13;
        locals.var_nqs_temp_dn14 = assign82020_e122571_d_n14;
        locals.var_nqs_temp_dn15 = assign82020_e122571_d_n15;
        locals.var_nqs_temp_dn16 = assign82020_e122571_d_n16;
        locals.var_nqs_temp_dn17 = assign82020_e122571_d_n17;
        locals.var_nqs_temp_dn18 = assign82020_e122571_d_n18;
        locals.var_nqs_temp_dn19 = assign82020_e122571_d_n19;
        locals.var_nqs_temp_dn20 = assign82020_e122571_d_n20;

        let (assign82030_e122626, assign82030_e122626_d_n5, assign82030_e122626_d_n6, assign82030_e122626_d_n7, assign82030_e122626_d_n8, assign82030_e122626_d_n12, assign82030_e122626_d_n13, assign82030_e122626_d_n14, assign82030_e122626_d_n15, assign82030_e122626_d_n16, assign82030_e122626_d_n17, assign82030_e122626_d_n18, assign82030_e122626_d_n19, assign82030_e122626_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 == 0.0)) && (locals.var_guard2233 == 0.0)) {
        let assign82030_e122602: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122604: f64 = (assign82030_e122602 - 230.25850929940458);
        let assign82030_e122608: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122610: f64 = (assign82030_e122608 - 230.25850929940458);
        let assign82030_e122613: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122615: f64 = (assign82030_e122613 - 230.25850929940458);
        let assign82030_e122617: f64 = (assign82030_e122615 * 0.3333333333333333);
        let assign82030_e122618: f64 = (1.0 + assign82030_e122617);
        let assign82030_e122619: f64 = (assign82030_e122610 * assign82030_e122618);
        let assign82030_e122620: f64 = (0.5 * assign82030_e122619);
        let assign82030_e122621: f64 = (1.0 + assign82030_e122620);
        let assign82030_e122622: f64 = (assign82030_e122604 * assign82030_e122621);
        let assign82030_e122623: f64 = (1.0 + assign82030_e122622);
        let assign82030_e122624: f64 = (1e100 * assign82030_e122623);
        (assign82030_e122624, (1e100 * (((-locals.var_nqs_xbar_dn5) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn5) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn6) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn6) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn7) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn7) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn8) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn8) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn12) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn12) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn13) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn13) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn14) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn14) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn15) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn15) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn16) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn16) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn17) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn17) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn18) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn18) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn19) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn19) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn20) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn20) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82030_e122626;
        locals.var_nqs_temp_dn5 = assign82030_e122626_d_n5;
        locals.var_nqs_temp_dn6 = assign82030_e122626_d_n6;
        locals.var_nqs_temp_dn7 = assign82030_e122626_d_n7;
        locals.var_nqs_temp_dn8 = assign82030_e122626_d_n8;
        locals.var_nqs_temp_dn12 = assign82030_e122626_d_n12;
        locals.var_nqs_temp_dn13 = assign82030_e122626_d_n13;
        locals.var_nqs_temp_dn14 = assign82030_e122626_d_n14;
        locals.var_nqs_temp_dn15 = assign82030_e122626_d_n15;
        locals.var_nqs_temp_dn16 = assign82030_e122626_d_n16;
        locals.var_nqs_temp_dn17 = assign82030_e122626_d_n17;
        locals.var_nqs_temp_dn18 = assign82030_e122626_d_n18;
        locals.var_nqs_temp_dn19 = assign82030_e122626_d_n19;
        locals.var_nqs_temp_dn20 = assign82030_e122626_d_n20;

        let (assign82040_e122652, assign82040_e122652_d_n5, assign82040_e122652_d_n6, assign82040_e122652_d_n7, assign82040_e122652_d_n8, assign82040_e122652_d_n12, assign82040_e122652_d_n13, assign82040_e122652_d_n14, assign82040_e122652_d_n15, assign82040_e122652_d_n16, assign82040_e122652_d_n17, assign82040_e122652_d_n18, assign82040_e122652_d_n19, assign82040_e122652_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82040_e122650: f64 = (1.0 - locals.var_nqs_temp);
        (assign82040_e122650, (-locals.var_nqs_temp_dn5), (-locals.var_nqs_temp_dn6), (-locals.var_nqs_temp_dn7), (-locals.var_nqs_temp_dn8), (-locals.var_nqs_temp_dn12), (-locals.var_nqs_temp_dn13), (-locals.var_nqs_temp_dn14), (-locals.var_nqs_temp_dn15), (-locals.var_nqs_temp_dn16), (-locals.var_nqs_temp_dn17), (-locals.var_nqs_temp_dn18), (-locals.var_nqs_temp_dn19), (-locals.var_nqs_temp_dn20),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82040_e122652;
        locals.var_nqs_w_dn5 = assign82040_e122652_d_n5;
        locals.var_nqs_w_dn6 = assign82040_e122652_d_n6;
        locals.var_nqs_w_dn7 = assign82040_e122652_d_n7;
        locals.var_nqs_w_dn8 = assign82040_e122652_d_n8;
        locals.var_nqs_w_dn12 = assign82040_e122652_d_n12;
        locals.var_nqs_w_dn13 = assign82040_e122652_d_n13;
        locals.var_nqs_w_dn14 = assign82040_e122652_d_n14;
        locals.var_nqs_w_dn15 = assign82040_e122652_d_n15;
        locals.var_nqs_w_dn16 = assign82040_e122652_d_n16;
        locals.var_nqs_w_dn17 = assign82040_e122652_d_n17;
        locals.var_nqs_w_dn18 = assign82040_e122652_d_n18;
        locals.var_nqs_w_dn19 = assign82040_e122652_d_n19;
        locals.var_nqs_w_dn20 = assign82040_e122652_d_n20;

        let (assign82050_e122691, assign82050_e122691_d_n5, assign82050_e122691_d_n6, assign82050_e122691_d_n7, assign82050_e122691_d_n8, assign82050_e122691_d_n12, assign82050_e122691_d_n13, assign82050_e122691_d_n14, assign82050_e122691_d_n15, assign82050_e122691_d_n16, assign82050_e122691_d_n17, assign82050_e122691_d_n18, assign82050_e122691_d_n19, assign82050_e122691_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82050_e122677: f64 = (locals.var_gp2 * 0.5);
        let assign82050_e122678: f64 = (locals.var_temp__blk1038 + assign82050_e122677);
        let assign82050_e122683: f64 = (locals.var_gp2 * 0.25);
        let assign82050_e122684: f64 = (locals.var_temp__blk1038 + assign82050_e122683);
        let assign82050_e122686: f64 = (assign82050_e122684 - locals.var_nqs_w);
        let assign82050_e122687: f64 = (assign82050_e122686).sqrt();
        let assign82050_e122688: f64 = (locals.var_gp * assign82050_e122687);
        let assign82050_e122689: f64 = (assign82050_e122678 - assign82050_e122688);
        (assign82050_e122689, ((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.5)) - ((locals.var_gp_dn5 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.25)) - locals.var_nqs_w_dn5) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.5)) - ((locals.var_gp_dn6 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.25)) - locals.var_nqs_w_dn6) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.5)) - ((locals.var_gp_dn7 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.25)) - locals.var_nqs_w_dn7) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.5)) - ((locals.var_gp_dn8 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.25)) - locals.var_nqs_w_dn8) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.5)) - ((locals.var_gp_dn12 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.25)) - locals.var_nqs_w_dn12) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.5)) - ((locals.var_gp_dn13 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.25)) - locals.var_nqs_w_dn13) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.5)) - ((locals.var_gp_dn14 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.25)) - locals.var_nqs_w_dn14) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.5)) - ((locals.var_gp_dn15 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.25)) - locals.var_nqs_w_dn15) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.5)) - ((locals.var_gp_dn16 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.25)) - locals.var_nqs_w_dn16) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.5)) - ((locals.var_gp_dn17 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.25)) - locals.var_nqs_w_dn17) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.5)) - ((locals.var_gp_dn18 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.25)) - locals.var_nqs_w_dn18) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.5)) - ((locals.var_gp_dn19 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.25)) - locals.var_nqs_w_dn19) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.5)) - ((locals.var_gp_dn20 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.25)) - locals.var_nqs_w_dn20) / (2.0 * assign82050_e122687))))),)
    } else {
        (locals.var_nqs_x0, locals.var_nqs_x0_dn5, locals.var_nqs_x0_dn6, locals.var_nqs_x0_dn7, locals.var_nqs_x0_dn8, locals.var_nqs_x0_dn12, locals.var_nqs_x0_dn13, locals.var_nqs_x0_dn14, locals.var_nqs_x0_dn15, locals.var_nqs_x0_dn16, locals.var_nqs_x0_dn17, locals.var_nqs_x0_dn18, locals.var_nqs_x0_dn19, locals.var_nqs_x0_dn20,)
    }
};
        locals.var_nqs_x0 = assign82050_e122691;
        locals.var_nqs_x0_dn5 = assign82050_e122691_d_n5;
        locals.var_nqs_x0_dn6 = assign82050_e122691_d_n6;
        locals.var_nqs_x0_dn7 = assign82050_e122691_d_n7;
        locals.var_nqs_x0_dn8 = assign82050_e122691_d_n8;
        locals.var_nqs_x0_dn12 = assign82050_e122691_d_n12;
        locals.var_nqs_x0_dn13 = assign82050_e122691_d_n13;
        locals.var_nqs_x0_dn14 = assign82050_e122691_d_n14;
        locals.var_nqs_x0_dn15 = assign82050_e122691_d_n15;
        locals.var_nqs_x0_dn16 = assign82050_e122691_d_n16;
        locals.var_nqs_x0_dn17 = assign82050_e122691_d_n17;
        locals.var_nqs_x0_dn18 = assign82050_e122691_d_n18;
        locals.var_nqs_x0_dn19 = assign82050_e122691_d_n19;
        locals.var_nqs_x0_dn20 = assign82050_e122691_d_n20;

        let assign82060_e122693: f64 = (-locals.var_nqs_x0);
        let assign82060_e122694: f64 = (assign82060_e122693).abs();
        let assign82060_e122696: f64 = if assign82060_e122694 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign82060_e122696;

        let (assign82070_e122724, assign82070_e122724_d_n5, assign82070_e122724_d_n6, assign82070_e122724_d_n7, assign82070_e122724_d_n8, assign82070_e122724_d_n12, assign82070_e122724_d_n13, assign82070_e122724_d_n14, assign82070_e122724_d_n15, assign82070_e122724_d_n16, assign82070_e122724_d_n17, assign82070_e122724_d_n18, assign82070_e122724_d_n19, assign82070_e122724_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 != 0.0)) {
        let assign82070_e122721: f64 = (-locals.var_nqs_x0);
        let assign82070_e122722: f64 = (assign82070_e122721).exp();
        (assign82070_e122722, (assign82070_e122722 * (-locals.var_nqs_x0_dn5)), (assign82070_e122722 * (-locals.var_nqs_x0_dn6)), (assign82070_e122722 * (-locals.var_nqs_x0_dn7)), (assign82070_e122722 * (-locals.var_nqs_x0_dn8)), (assign82070_e122722 * (-locals.var_nqs_x0_dn12)), (assign82070_e122722 * (-locals.var_nqs_x0_dn13)), (assign82070_e122722 * (-locals.var_nqs_x0_dn14)), (assign82070_e122722 * (-locals.var_nqs_x0_dn15)), (assign82070_e122722 * (-locals.var_nqs_x0_dn16)), (assign82070_e122722 * (-locals.var_nqs_x0_dn17)), (assign82070_e122722 * (-locals.var_nqs_x0_dn18)), (assign82070_e122722 * (-locals.var_nqs_x0_dn19)), (assign82070_e122722 * (-locals.var_nqs_x0_dn20)),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82070_e122724;
        locals.var_nqs_d0_dn5 = assign82070_e122724_d_n5;
        locals.var_nqs_d0_dn6 = assign82070_e122724_d_n6;
        locals.var_nqs_d0_dn7 = assign82070_e122724_d_n7;
        locals.var_nqs_d0_dn8 = assign82070_e122724_d_n8;
        locals.var_nqs_d0_dn12 = assign82070_e122724_d_n12;
        locals.var_nqs_d0_dn13 = assign82070_e122724_d_n13;
        locals.var_nqs_d0_dn14 = assign82070_e122724_d_n14;
        locals.var_nqs_d0_dn15 = assign82070_e122724_d_n15;
        locals.var_nqs_d0_dn16 = assign82070_e122724_d_n16;
        locals.var_nqs_d0_dn17 = assign82070_e122724_d_n17;
        locals.var_nqs_d0_dn18 = assign82070_e122724_d_n18;
        locals.var_nqs_d0_dn19 = assign82070_e122724_d_n19;
        locals.var_nqs_d0_dn20 = assign82070_e122724_d_n20;

        let assign82080_e122726: f64 = (-locals.var_nqs_x0);
        let assign82080_e122728: f64 = if assign82080_e122726 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign82080_e122728;

        let (assign82090_e122785, assign82090_e122785_d_n5, assign82090_e122785_d_n6, assign82090_e122785_d_n7, assign82090_e122785_d_n8, assign82090_e122785_d_n12, assign82090_e122785_d_n13, assign82090_e122785_d_n14, assign82090_e122785_d_n15, assign82090_e122785_d_n16, assign82090_e122785_d_n17, assign82090_e122785_d_n18, assign82090_e122785_d_n19, assign82090_e122785_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 == 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign82090_e122758: f64 = (-230.25850929940458);
        let assign82090_e122760: f64 = (-locals.var_nqs_x0);
        let assign82090_e122761: f64 = (assign82090_e122758 - assign82090_e122760);
        let assign82090_e122765: f64 = (-230.25850929940458);
        let assign82090_e122767: f64 = (-locals.var_nqs_x0);
        let assign82090_e122768: f64 = (assign82090_e122765 - assign82090_e122767);
        let assign82090_e122771: f64 = (-230.25850929940458);
        let assign82090_e122773: f64 = (-locals.var_nqs_x0);
        let assign82090_e122774: f64 = (assign82090_e122771 - assign82090_e122773);
        let assign82090_e122776: f64 = (assign82090_e122774 * 0.3333333333333333);
        let assign82090_e122777: f64 = (1.0 + assign82090_e122776);
        let assign82090_e122778: f64 = (assign82090_e122768 * assign82090_e122777);
        let assign82090_e122779: f64 = (0.5 * assign82090_e122778);
        let assign82090_e122780: f64 = (1.0 + assign82090_e122779);
        let assign82090_e122781: f64 = (assign82090_e122761 * assign82090_e122780);
        let assign82090_e122782: f64 = (1.0 + assign82090_e122781);
        let assign82090_e122783: f64 = (1e-100 / assign82090_e122782);
        (assign82090_e122783, (-((1e-100 * (((-(-locals.var_nqs_x0_dn5)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn5)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn6)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn6)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn7)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn7)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn8)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn8)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn12)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn12)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn13)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn13)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn14)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn14)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn15)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn15)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn16)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn16)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn17)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn17)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn18)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn18)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn19)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn19)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn20)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn20)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82090_e122785;
        locals.var_nqs_d0_dn5 = assign82090_e122785_d_n5;
        locals.var_nqs_d0_dn6 = assign82090_e122785_d_n6;
        locals.var_nqs_d0_dn7 = assign82090_e122785_d_n7;
        locals.var_nqs_d0_dn8 = assign82090_e122785_d_n8;
        locals.var_nqs_d0_dn12 = assign82090_e122785_d_n12;
        locals.var_nqs_d0_dn13 = assign82090_e122785_d_n13;
        locals.var_nqs_d0_dn14 = assign82090_e122785_d_n14;
        locals.var_nqs_d0_dn15 = assign82090_e122785_d_n15;
        locals.var_nqs_d0_dn16 = assign82090_e122785_d_n16;
        locals.var_nqs_d0_dn17 = assign82090_e122785_d_n17;
        locals.var_nqs_d0_dn18 = assign82090_e122785_d_n18;
        locals.var_nqs_d0_dn19 = assign82090_e122785_d_n19;
        locals.var_nqs_d0_dn20 = assign82090_e122785_d_n20;

    }

    pub(super) fn stamp_transient_block_162(
        locals: &mut StampLocals,
    ) {
        let (assign82100_e122840, assign82100_e122840_d_n5, assign82100_e122840_d_n6, assign82100_e122840_d_n7, assign82100_e122840_d_n8, assign82100_e122840_d_n12, assign82100_e122840_d_n13, assign82100_e122840_d_n14, assign82100_e122840_d_n15, assign82100_e122840_d_n16, assign82100_e122840_d_n17, assign82100_e122840_d_n18, assign82100_e122840_d_n19, assign82100_e122840_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 == 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign82100_e122816: f64 = (-locals.var_nqs_x0);
        let assign82100_e122818: f64 = (assign82100_e122816 - 230.25850929940458);
        let assign82100_e122822: f64 = (-locals.var_nqs_x0);
        let assign82100_e122824: f64 = (assign82100_e122822 - 230.25850929940458);
        let assign82100_e122827: f64 = (-locals.var_nqs_x0);
        let assign82100_e122829: f64 = (assign82100_e122827 - 230.25850929940458);
        let assign82100_e122831: f64 = (assign82100_e122829 * 0.3333333333333333);
        let assign82100_e122832: f64 = (1.0 + assign82100_e122831);
        let assign82100_e122833: f64 = (assign82100_e122824 * assign82100_e122832);
        let assign82100_e122834: f64 = (0.5 * assign82100_e122833);
        let assign82100_e122835: f64 = (1.0 + assign82100_e122834);
        let assign82100_e122836: f64 = (assign82100_e122818 * assign82100_e122835);
        let assign82100_e122837: f64 = (1.0 + assign82100_e122836);
        let assign82100_e122838: f64 = (1e100 * assign82100_e122837);
        (assign82100_e122838, (1e100 * (((-locals.var_nqs_x0_dn5) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn5) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn6) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn6) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn7) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn7) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn8) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn8) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn12) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn12) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn13) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn13) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn14) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn14) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn15) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn15) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn16) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn16) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn17) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn17) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn18) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn18) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn19) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn19) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn20) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn20) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82100_e122840;
        locals.var_nqs_d0_dn5 = assign82100_e122840_d_n5;
        locals.var_nqs_d0_dn6 = assign82100_e122840_d_n6;
        locals.var_nqs_d0_dn7 = assign82100_e122840_d_n7;
        locals.var_nqs_d0_dn8 = assign82100_e122840_d_n8;
        locals.var_nqs_d0_dn12 = assign82100_e122840_d_n12;
        locals.var_nqs_d0_dn13 = assign82100_e122840_d_n13;
        locals.var_nqs_d0_dn14 = assign82100_e122840_d_n14;
        locals.var_nqs_d0_dn15 = assign82100_e122840_d_n15;
        locals.var_nqs_d0_dn16 = assign82100_e122840_d_n16;
        locals.var_nqs_d0_dn17 = assign82100_e122840_d_n17;
        locals.var_nqs_d0_dn18 = assign82100_e122840_d_n18;
        locals.var_nqs_d0_dn19 = assign82100_e122840_d_n19;
        locals.var_nqs_d0_dn20 = assign82100_e122840_d_n20;

        let (assign82110_e122870, assign82110_e122870_d_n5, assign82110_e122870_d_n6, assign82110_e122870_d_n7, assign82110_e122870_d_n8, assign82110_e122870_d_n12, assign82110_e122870_d_n13, assign82110_e122870_d_n14, assign82110_e122870_d_n15, assign82110_e122870_d_n16, assign82110_e122870_d_n17, assign82110_e122870_d_n18, assign82110_e122870_d_n19, assign82110_e122870_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82110_e122865: f64 = (locals.var_gp2 * 0.5);
        let assign82110_e122867: f64 = (assign82110_e122865 * locals.var_nqs_d0);
        let assign82110_e122868: f64 = (1.0 - assign82110_e122867);
        (assign82110_e122868, (-(((locals.var_gp2_dn5 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn5))), (-(((locals.var_gp2_dn6 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn6))), (-(((locals.var_gp2_dn7 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn7))), (-(((locals.var_gp2_dn8 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn8))), (-(((locals.var_gp2_dn12 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn12))), (-(((locals.var_gp2_dn13 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn13))), (-(((locals.var_gp2_dn14 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn14))), (-(((locals.var_gp2_dn15 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn15))), (-(((locals.var_gp2_dn16 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn16))), (-(((locals.var_gp2_dn17 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn17))), (-(((locals.var_gp2_dn18 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn18))), (-(((locals.var_gp2_dn19 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn19))), (-(((locals.var_gp2_dn20 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82110_e122870;
        locals.var_nqs_xi_dn5 = assign82110_e122870_d_n5;
        locals.var_nqs_xi_dn6 = assign82110_e122870_d_n6;
        locals.var_nqs_xi_dn7 = assign82110_e122870_d_n7;
        locals.var_nqs_xi_dn8 = assign82110_e122870_d_n8;
        locals.var_nqs_xi_dn12 = assign82110_e122870_d_n12;
        locals.var_nqs_xi_dn13 = assign82110_e122870_d_n13;
        locals.var_nqs_xi_dn14 = assign82110_e122870_d_n14;
        locals.var_nqs_xi_dn15 = assign82110_e122870_d_n15;
        locals.var_nqs_xi_dn16 = assign82110_e122870_d_n16;
        locals.var_nqs_xi_dn17 = assign82110_e122870_d_n17;
        locals.var_nqs_xi_dn18 = assign82110_e122870_d_n18;
        locals.var_nqs_xi_dn19 = assign82110_e122870_d_n19;
        locals.var_nqs_xi_dn20 = assign82110_e122870_d_n20;

        let (assign82120_e122904, assign82120_e122904_d_n5, assign82120_e122904_d_n6, assign82120_e122904_d_n7, assign82120_e122904_d_n8, assign82120_e122904_d_n12, assign82120_e122904_d_n13, assign82120_e122904_d_n14, assign82120_e122904_d_n15, assign82120_e122904_d_n16, assign82120_e122904_d_n17, assign82120_e122904_d_n18, assign82120_e122904_d_n19, assign82120_e122904_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82120_e122895: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82120_e122896: f64 = (2.0 * assign82120_e122895);
        let assign82120_e122900: f64 = (1.0 - locals.var_nqs_d0);
        let assign82120_e122901: f64 = (locals.var_gp2 * assign82120_e122900);
        let assign82120_e122902: f64 = (assign82120_e122896 + assign82120_e122901);
        (assign82120_e122902, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82120_e122904;
        locals.var_nqs_p_dn5 = assign82120_e122904_d_n5;
        locals.var_nqs_p_dn6 = assign82120_e122904_d_n6;
        locals.var_nqs_p_dn7 = assign82120_e122904_d_n7;
        locals.var_nqs_p_dn8 = assign82120_e122904_d_n8;
        locals.var_nqs_p_dn12 = assign82120_e122904_d_n12;
        locals.var_nqs_p_dn13 = assign82120_e122904_d_n13;
        locals.var_nqs_p_dn14 = assign82120_e122904_d_n14;
        locals.var_nqs_p_dn15 = assign82120_e122904_d_n15;
        locals.var_nqs_p_dn16 = assign82120_e122904_d_n16;
        locals.var_nqs_p_dn17 = assign82120_e122904_d_n17;
        locals.var_nqs_p_dn18 = assign82120_e122904_d_n18;
        locals.var_nqs_p_dn19 = assign82120_e122904_d_n19;
        locals.var_nqs_p_dn20 = assign82120_e122904_d_n20;

        let (assign82130_e122942, assign82130_e122942_d_n5, assign82130_e122942_d_n6, assign82130_e122942_d_n7, assign82130_e122942_d_n8, assign82130_e122942_d_n12, assign82130_e122942_d_n13, assign82130_e122942_d_n14, assign82130_e122942_d_n15, assign82130_e122942_d_n16, assign82130_e122942_d_n17, assign82130_e122942_d_n18, assign82130_e122942_d_n19, assign82130_e122942_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82130_e122928: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82130_e122931: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82130_e122932: f64 = (assign82130_e122928 * assign82130_e122931);
        let assign82130_e122936: f64 = (locals.var_nqs_x0 - 1.0);
        let assign82130_e122938: f64 = (assign82130_e122936 + locals.var_nqs_d0);
        let assign82130_e122939: f64 = (locals.var_gp2 * assign82130_e122938);
        let assign82130_e122940: f64 = (assign82130_e122932 - assign82130_e122939);
        (assign82130_e122940, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82130_e122942;
        locals.var_nqs_q_dn5 = assign82130_e122942_d_n5;
        locals.var_nqs_q_dn6 = assign82130_e122942_d_n6;
        locals.var_nqs_q_dn7 = assign82130_e122942_d_n7;
        locals.var_nqs_q_dn8 = assign82130_e122942_d_n8;
        locals.var_nqs_q_dn12 = assign82130_e122942_d_n12;
        locals.var_nqs_q_dn13 = assign82130_e122942_d_n13;
        locals.var_nqs_q_dn14 = assign82130_e122942_d_n14;
        locals.var_nqs_q_dn15 = assign82130_e122942_d_n15;
        locals.var_nqs_q_dn16 = assign82130_e122942_d_n16;
        locals.var_nqs_q_dn17 = assign82130_e122942_d_n17;
        locals.var_nqs_q_dn18 = assign82130_e122942_d_n18;
        locals.var_nqs_q_dn19 = assign82130_e122942_d_n19;
        locals.var_nqs_q_dn20 = assign82130_e122942_d_n20;

        let (assign82140_e122974, assign82140_e122974_d_n5, assign82140_e122974_d_n6, assign82140_e122974_d_n7, assign82140_e122974_d_n8, assign82140_e122974_d_n12, assign82140_e122974_d_n13, assign82140_e122974_d_n14, assign82140_e122974_d_n15, assign82140_e122974_d_n16, assign82140_e122974_d_n17, assign82140_e122974_d_n18, assign82140_e122974_d_n19, assign82140_e122974_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82140_e122966: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82140_e122969: f64 = (4.0 * locals.var_nqs_xi);
        let assign82140_e122971: f64 = (assign82140_e122969 * locals.var_nqs_q);
        let assign82140_e122972: f64 = (assign82140_e122966 - assign82140_e122971);
        (assign82140_e122972, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82140_e122974;
        locals.var_nqs_temp_dn5 = assign82140_e122974_d_n5;
        locals.var_nqs_temp_dn6 = assign82140_e122974_d_n6;
        locals.var_nqs_temp_dn7 = assign82140_e122974_d_n7;
        locals.var_nqs_temp_dn8 = assign82140_e122974_d_n8;
        locals.var_nqs_temp_dn12 = assign82140_e122974_d_n12;
        locals.var_nqs_temp_dn13 = assign82140_e122974_d_n13;
        locals.var_nqs_temp_dn14 = assign82140_e122974_d_n14;
        locals.var_nqs_temp_dn15 = assign82140_e122974_d_n15;
        locals.var_nqs_temp_dn16 = assign82140_e122974_d_n16;
        locals.var_nqs_temp_dn17 = assign82140_e122974_d_n17;
        locals.var_nqs_temp_dn18 = assign82140_e122974_d_n18;
        locals.var_nqs_temp_dn19 = assign82140_e122974_d_n19;
        locals.var_nqs_temp_dn20 = assign82140_e122974_d_n20;

        let (assign82150_e123005, assign82150_e123005_d_n5, assign82150_e123005_d_n6, assign82150_e123005_d_n7, assign82150_e123005_d_n8, assign82150_e123005_d_n12, assign82150_e123005_d_n13, assign82150_e123005_d_n14, assign82150_e123005_d_n15, assign82150_e123005_d_n16, assign82150_e123005_d_n17, assign82150_e123005_d_n18, assign82150_e123005_d_n19, assign82150_e123005_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82150_e122998: f64 = (2.0 * locals.var_nqs_q);
        let assign82150_e123001: f64 = (locals.var_nqs_temp).sqrt();
        let assign82150_e123002: f64 = (locals.var_nqs_p + assign82150_e123001);
        let assign82150_e123003: f64 = (assign82150_e122998 / assign82150_e123002);
        (assign82150_e123003, ((((2.0 * locals.var_nqs_q_dn5) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn6) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn7) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn8) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn12) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn13) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn14) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn15) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn16) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn17) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn18) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn19) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn20) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign82150_e123005;
        locals.var_nqs_u_dn5 = assign82150_e123005_d_n5;
        locals.var_nqs_u_dn6 = assign82150_e123005_d_n6;
        locals.var_nqs_u_dn7 = assign82150_e123005_d_n7;
        locals.var_nqs_u_dn8 = assign82150_e123005_d_n8;
        locals.var_nqs_u_dn12 = assign82150_e123005_d_n12;
        locals.var_nqs_u_dn13 = assign82150_e123005_d_n13;
        locals.var_nqs_u_dn14 = assign82150_e123005_d_n14;
        locals.var_nqs_u_dn15 = assign82150_e123005_d_n15;
        locals.var_nqs_u_dn16 = assign82150_e123005_d_n16;
        locals.var_nqs_u_dn17 = assign82150_e123005_d_n17;
        locals.var_nqs_u_dn18 = assign82150_e123005_d_n18;
        locals.var_nqs_u_dn19 = assign82150_e123005_d_n19;
        locals.var_nqs_u_dn20 = assign82150_e123005_d_n20;

        let (assign82160_e123031, assign82160_e123031_d_n5, assign82160_e123031_d_n6, assign82160_e123031_d_n7, assign82160_e123031_d_n8, assign82160_e123031_d_n12, assign82160_e123031_d_n13, assign82160_e123031_d_n14, assign82160_e123031_d_n15, assign82160_e123031_d_n16, assign82160_e123031_d_n17, assign82160_e123031_d_n18, assign82160_e123031_d_n19, assign82160_e123031_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82160_e123029: f64 = (locals.var_nqs_x0 + locals.var_nqs_u);
        (assign82160_e123029, (locals.var_nqs_x0_dn5 + locals.var_nqs_u_dn5), (locals.var_nqs_x0_dn6 + locals.var_nqs_u_dn6), (locals.var_nqs_x0_dn7 + locals.var_nqs_u_dn7), (locals.var_nqs_x0_dn8 + locals.var_nqs_u_dn8), (locals.var_nqs_x0_dn12 + locals.var_nqs_u_dn12), (locals.var_nqs_x0_dn13 + locals.var_nqs_u_dn13), (locals.var_nqs_x0_dn14 + locals.var_nqs_u_dn14), (locals.var_nqs_x0_dn15 + locals.var_nqs_u_dn15), (locals.var_nqs_x0_dn16 + locals.var_nqs_u_dn16), (locals.var_nqs_x0_dn17 + locals.var_nqs_u_dn17), (locals.var_nqs_x0_dn18 + locals.var_nqs_u_dn18), (locals.var_nqs_x0_dn19 + locals.var_nqs_u_dn19), (locals.var_nqs_x0_dn20 + locals.var_nqs_u_dn20),)
    } else {
        (locals.var_temp8, locals.var_temp8_dn5, locals.var_temp8_dn6, locals.var_temp8_dn7, locals.var_temp8_dn8, locals.var_temp8_dn12, locals.var_temp8_dn13, locals.var_temp8_dn14, locals.var_temp8_dn15, locals.var_temp8_dn16, locals.var_temp8_dn17, locals.var_temp8_dn18, locals.var_temp8_dn19, locals.var_temp8_dn20,)
    }
};
        locals.var_temp8 = assign82160_e123031;
        locals.var_temp8_dn5 = assign82160_e123031_d_n5;
        locals.var_temp8_dn6 = assign82160_e123031_d_n6;
        locals.var_temp8_dn7 = assign82160_e123031_d_n7;
        locals.var_temp8_dn8 = assign82160_e123031_d_n8;
        locals.var_temp8_dn12 = assign82160_e123031_d_n12;
        locals.var_temp8_dn13 = assign82160_e123031_d_n13;
        locals.var_temp8_dn14 = assign82160_e123031_d_n14;
        locals.var_temp8_dn15 = assign82160_e123031_d_n15;
        locals.var_temp8_dn16 = assign82160_e123031_d_n16;
        locals.var_temp8_dn17 = assign82160_e123031_d_n17;
        locals.var_temp8_dn18 = assign82160_e123031_d_n18;
        locals.var_temp8_dn19 = assign82160_e123031_d_n19;
        locals.var_temp8_dn20 = assign82160_e123031_d_n20;

        let (assign82170_e123053, assign82170_e123053_d_n5, assign82170_e123053_d_n6, assign82170_e123053_d_n7, assign82170_e123053_d_n8, assign82170_e123053_d_n12, assign82170_e123053_d_n13, assign82170_e123053_d_n14, assign82170_e123053_d_n15, assign82170_e123053_d_n16, assign82170_e123053_d_n17, assign82170_e123053_d_n18, assign82170_e123053_d_n19, assign82170_e123053_d_n20,) = {
    if ((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        let assign82170_e123049: f64 = (locals.var_qp9 / locals.var_pd);
        let assign82170_e123051: f64 = (assign82170_e123049 + locals.var_xg_ac);
        (assign82170_e123051, ((-((locals.var_qp9 * locals.var_pd_dn5) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn5), ((-((locals.var_qp9 * locals.var_pd_dn6) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn6), ((-((locals.var_qp9 * locals.var_pd_dn7) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn7), ((-((locals.var_qp9 * locals.var_pd_dn8) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn8), ((-((locals.var_qp9 * locals.var_pd_dn12) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn12), ((-((locals.var_qp9 * locals.var_pd_dn13) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn13), ((-((locals.var_qp9 * locals.var_pd_dn14) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn14), ((-((locals.var_qp9 * locals.var_pd_dn15) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn15), ((-((locals.var_qp9 * locals.var_pd_dn16) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn16), ((-((locals.var_qp9 * locals.var_pd_dn17) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn17), ((-((locals.var_qp9 * locals.var_pd_dn18) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn18), ((-((locals.var_qp9 * locals.var_pd_dn19) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn19), ((((locals.var_qp9_dn20 * locals.var_pd) - (locals.var_qp9 * locals.var_pd_dn20)) / (locals.var_pd * locals.var_pd)) + locals.var_xg_ac_dn20),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign82170_e123053;
        locals.var_temp__blk1038_dn5 = assign82170_e123053_d_n5;
        locals.var_temp__blk1038_dn6 = assign82170_e123053_d_n6;
        locals.var_temp__blk1038_dn7 = assign82170_e123053_d_n7;
        locals.var_temp__blk1038_dn8 = assign82170_e123053_d_n8;
        locals.var_temp__blk1038_dn12 = assign82170_e123053_d_n12;
        locals.var_temp__blk1038_dn13 = assign82170_e123053_d_n13;
        locals.var_temp__blk1038_dn14 = assign82170_e123053_d_n14;
        locals.var_temp__blk1038_dn15 = assign82170_e123053_d_n15;
        locals.var_temp__blk1038_dn16 = assign82170_e123053_d_n16;
        locals.var_temp__blk1038_dn17 = assign82170_e123053_d_n17;
        locals.var_temp__blk1038_dn18 = assign82170_e123053_d_n18;
        locals.var_temp__blk1038_dn19 = assign82170_e123053_d_n19;
        locals.var_temp__blk1038_dn20 = assign82170_e123053_d_n20;

        let assign82180_e123055: f64 = (locals.var_temp__blk1038).abs();
        let assign82180_e123057: f64 = if assign82180_e123055 <= locals.var_marginp { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign82180_e123057;

        let (assign82190_e123079, assign82190_e123079_d_n5, assign82190_e123079_d_n6, assign82190_e123079_d_n7, assign82190_e123079_d_n8, assign82190_e123079_d_n12, assign82190_e123079_d_n13, assign82190_e123079_d_n14, assign82190_e123079_d_n15, assign82190_e123079_d_n16, assign82190_e123079_d_n17, assign82190_e123079_d_n18, assign82190_e123079_d_n19, assign82190_e123079_d_n20,) = {
    if (((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 != 0.0)) {
        let assign82190_e123077: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        (assign82190_e123077, (((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82190_e123079;
        locals.var_temp9_dn5 = assign82190_e123079_d_n5;
        locals.var_temp9_dn6 = assign82190_e123079_d_n6;
        locals.var_temp9_dn7 = assign82190_e123079_d_n7;
        locals.var_temp9_dn8 = assign82190_e123079_d_n8;
        locals.var_temp9_dn12 = assign82190_e123079_d_n12;
        locals.var_temp9_dn13 = assign82190_e123079_d_n13;
        locals.var_temp9_dn14 = assign82190_e123079_d_n14;
        locals.var_temp9_dn15 = assign82190_e123079_d_n15;
        locals.var_temp9_dn16 = assign82190_e123079_d_n16;
        locals.var_temp9_dn17 = assign82190_e123079_d_n17;
        locals.var_temp9_dn18 = assign82190_e123079_d_n18;
        locals.var_temp9_dn19 = assign82190_e123079_d_n19;
        locals.var_temp9_dn20 = assign82190_e123079_d_n20;

        let assign82200_e123082: f64 = (-locals.var_marginp);
        let assign82200_e123083: f64 = if locals.var_temp__blk1038 < assign82200_e123082 { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign82200_e123083;

        let (assign82210_e123107, assign82210_e123107_d_n5, assign82210_e123107_d_n6, assign82210_e123107_d_n7, assign82210_e123107_d_n8, assign82210_e123107_d_n12, assign82210_e123107_d_n13, assign82210_e123107_d_n14, assign82210_e123107_d_n15, assign82210_e123107_d_n16, assign82210_e123107_d_n17, assign82210_e123107_d_n18, assign82210_e123107_d_n19, assign82210_e123107_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82210_e123105: f64 = (-locals.var_temp__blk1038);
        (assign82210_e123105, (-locals.var_temp__blk1038_dn5), (-locals.var_temp__blk1038_dn6), (-locals.var_temp__blk1038_dn7), (-locals.var_temp__blk1038_dn8), (-locals.var_temp__blk1038_dn12), (-locals.var_temp__blk1038_dn13), (-locals.var_temp__blk1038_dn14), (-locals.var_temp__blk1038_dn15), (-locals.var_temp__blk1038_dn16), (-locals.var_temp__blk1038_dn17), (-locals.var_temp__blk1038_dn18), (-locals.var_temp__blk1038_dn19), (-locals.var_temp__blk1038_dn20),)
    } else {
        (locals.var_nqs_yg, locals.var_nqs_yg_dn5, locals.var_nqs_yg_dn6, locals.var_nqs_yg_dn7, locals.var_nqs_yg_dn8, locals.var_nqs_yg_dn12, locals.var_nqs_yg_dn13, locals.var_nqs_yg_dn14, locals.var_nqs_yg_dn15, locals.var_nqs_yg_dn16, locals.var_nqs_yg_dn17, locals.var_nqs_yg_dn18, locals.var_nqs_yg_dn19, locals.var_nqs_yg_dn20,)
    }
};
        locals.var_nqs_yg = assign82210_e123107;
        locals.var_nqs_yg_dn5 = assign82210_e123107_d_n5;
        locals.var_nqs_yg_dn6 = assign82210_e123107_d_n6;
        locals.var_nqs_yg_dn7 = assign82210_e123107_d_n7;
        locals.var_nqs_yg_dn8 = assign82210_e123107_d_n8;
        locals.var_nqs_yg_dn12 = assign82210_e123107_d_n12;
        locals.var_nqs_yg_dn13 = assign82210_e123107_d_n13;
        locals.var_nqs_yg_dn14 = assign82210_e123107_d_n14;
        locals.var_nqs_yg_dn15 = assign82210_e123107_d_n15;
        locals.var_nqs_yg_dn16 = assign82210_e123107_d_n16;
        locals.var_nqs_yg_dn17 = assign82210_e123107_d_n17;
        locals.var_nqs_yg_dn18 = assign82210_e123107_d_n18;
        locals.var_nqs_yg_dn19 = assign82210_e123107_d_n19;
        locals.var_nqs_yg_dn20 = assign82210_e123107_d_n20;

        let (assign82220_e123134, assign82220_e123134_d_n5, assign82220_e123134_d_n6, assign82220_e123134_d_n7, assign82220_e123134_d_n8, assign82220_e123134_d_n12, assign82220_e123134_d_n13, assign82220_e123134_d_n14, assign82220_e123134_d_n15, assign82220_e123134_d_n16, assign82220_e123134_d_n17, assign82220_e123134_d_n18, assign82220_e123134_d_n19, assign82220_e123134_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82220_e123130: f64 = (1.25 * locals.var_nqs_yg);
        let assign82220_e123132: f64 = (assign82220_e123130 / locals.var_a_factrp);
        (assign82220_e123132, ((((1.25 * locals.var_nqs_yg_dn5) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn6) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn7) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn8) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn12) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn13) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn14) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn15) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn16) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn17) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn18) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn19) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn20) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_nqs_z, locals.var_nqs_z_dn5, locals.var_nqs_z_dn6, locals.var_nqs_z_dn7, locals.var_nqs_z_dn8, locals.var_nqs_z_dn12, locals.var_nqs_z_dn13, locals.var_nqs_z_dn14, locals.var_nqs_z_dn15, locals.var_nqs_z_dn16, locals.var_nqs_z_dn17, locals.var_nqs_z_dn18, locals.var_nqs_z_dn19, locals.var_nqs_z_dn20,)
    }
};
        locals.var_nqs_z = assign82220_e123134;
        locals.var_nqs_z_dn5 = assign82220_e123134_d_n5;
        locals.var_nqs_z_dn6 = assign82220_e123134_d_n6;
        locals.var_nqs_z_dn7 = assign82220_e123134_d_n7;
        locals.var_nqs_z_dn8 = assign82220_e123134_d_n8;
        locals.var_nqs_z_dn12 = assign82220_e123134_d_n12;
        locals.var_nqs_z_dn13 = assign82220_e123134_d_n13;
        locals.var_nqs_z_dn14 = assign82220_e123134_d_n14;
        locals.var_nqs_z_dn15 = assign82220_e123134_d_n15;
        locals.var_nqs_z_dn16 = assign82220_e123134_d_n16;
        locals.var_nqs_z_dn17 = assign82220_e123134_d_n17;
        locals.var_nqs_z_dn18 = assign82220_e123134_d_n18;
        locals.var_nqs_z_dn19 = assign82220_e123134_d_n19;
        locals.var_nqs_z_dn20 = assign82220_e123134_d_n20;

        let (assign82230_e123172, assign82230_e123172_d_n5, assign82230_e123172_d_n6, assign82230_e123172_d_n7, assign82230_e123172_d_n8, assign82230_e123172_d_n12, assign82230_e123172_d_n13, assign82230_e123172_d_n14, assign82230_e123172_d_n15, assign82230_e123172_d_n16, assign82230_e123172_d_n17, assign82230_e123172_d_n18, assign82230_e123172_d_n19, assign82230_e123172_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82230_e123157: f64 = (locals.var_nqs_z + 10.0);
        let assign82230_e123160: f64 = (locals.var_nqs_z - 6.0);
        let assign82230_e123163: f64 = (locals.var_nqs_z - 6.0);
        let assign82230_e123164: f64 = (assign82230_e123160 * assign82230_e123163);
        let assign82230_e123166: f64 = (assign82230_e123164 + 64.0);
        let assign82230_e123167: f64 = (assign82230_e123166).sqrt();
        let assign82230_e123168: f64 = (assign82230_e123157 - assign82230_e123167);
        let assign82230_e123170: f64 = (assign82230_e123168 * 0.5);
        (assign82230_e123170, ((locals.var_nqs_z_dn5 - (((locals.var_nqs_z_dn5 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn5)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn6 - (((locals.var_nqs_z_dn6 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn6)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn7 - (((locals.var_nqs_z_dn7 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn7)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn8 - (((locals.var_nqs_z_dn8 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn8)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn12 - (((locals.var_nqs_z_dn12 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn12)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn13 - (((locals.var_nqs_z_dn13 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn13)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn14 - (((locals.var_nqs_z_dn14 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn14)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn15 - (((locals.var_nqs_z_dn15 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn15)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn16 - (((locals.var_nqs_z_dn16 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn16)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn17 - (((locals.var_nqs_z_dn17 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn17)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn18 - (((locals.var_nqs_z_dn18 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn18)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn19 - (((locals.var_nqs_z_dn19 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn19)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn20 - (((locals.var_nqs_z_dn20 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn20)) / (2.0 * assign82230_e123167))) * 0.5),)
    } else {
        (locals.var_nqs_eta, locals.var_nqs_eta_dn5, locals.var_nqs_eta_dn6, locals.var_nqs_eta_dn7, locals.var_nqs_eta_dn8, locals.var_nqs_eta_dn12, locals.var_nqs_eta_dn13, locals.var_nqs_eta_dn14, locals.var_nqs_eta_dn15, locals.var_nqs_eta_dn16, locals.var_nqs_eta_dn17, locals.var_nqs_eta_dn18, locals.var_nqs_eta_dn19, locals.var_nqs_eta_dn20,)
    }
};
        locals.var_nqs_eta = assign82230_e123172;
        locals.var_nqs_eta_dn5 = assign82230_e123172_d_n5;
        locals.var_nqs_eta_dn6 = assign82230_e123172_d_n6;
        locals.var_nqs_eta_dn7 = assign82230_e123172_d_n7;
        locals.var_nqs_eta_dn8 = assign82230_e123172_d_n8;
        locals.var_nqs_eta_dn12 = assign82230_e123172_d_n12;
        locals.var_nqs_eta_dn13 = assign82230_e123172_d_n13;
        locals.var_nqs_eta_dn14 = assign82230_e123172_d_n14;
        locals.var_nqs_eta_dn15 = assign82230_e123172_d_n15;
        locals.var_nqs_eta_dn16 = assign82230_e123172_d_n16;
        locals.var_nqs_eta_dn17 = assign82230_e123172_d_n17;
        locals.var_nqs_eta_dn18 = assign82230_e123172_d_n18;
        locals.var_nqs_eta_dn19 = assign82230_e123172_d_n19;
        locals.var_nqs_eta_dn20 = assign82230_e123172_d_n20;

        let (assign82240_e123207, assign82240_e123207_d_n5, assign82240_e123207_d_n6, assign82240_e123207_d_n7, assign82240_e123207_d_n8, assign82240_e123207_d_n12, assign82240_e123207_d_n13, assign82240_e123207_d_n14, assign82240_e123207_d_n15, assign82240_e123207_d_n16, assign82240_e123207_d_n17, assign82240_e123207_d_n18, assign82240_e123207_d_n19, assign82240_e123207_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82240_e123195: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82240_e123198: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82240_e123199: f64 = (assign82240_e123195 * assign82240_e123198);
        let assign82240_e123203: f64 = (locals.var_nqs_eta + 1.0);
        let assign82240_e123204: f64 = (locals.var_gp2 * assign82240_e123203);
        let assign82240_e123205: f64 = (assign82240_e123199 + assign82240_e123204);
        (assign82240_e123205, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5))) + ((locals.var_gp2_dn5 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn5))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6))) + ((locals.var_gp2_dn6 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn6))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7))) + ((locals.var_gp2_dn7 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn7))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8))) + ((locals.var_gp2_dn8 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn8))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12))) + ((locals.var_gp2_dn12 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn12))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13))) + ((locals.var_gp2_dn13 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn13))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14))) + ((locals.var_gp2_dn14 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn14))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15))) + ((locals.var_gp2_dn15 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn15))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16))) + ((locals.var_gp2_dn16 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn16))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17))) + ((locals.var_gp2_dn17 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn17))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18))) + ((locals.var_gp2_dn18 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn18))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19))) + ((locals.var_gp2_dn19 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn19))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20))) + ((locals.var_gp2_dn20 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn20))),)
    } else {
        (locals.var_nqs_a, locals.var_nqs_a_dn5, locals.var_nqs_a_dn6, locals.var_nqs_a_dn7, locals.var_nqs_a_dn8, locals.var_nqs_a_dn12, locals.var_nqs_a_dn13, locals.var_nqs_a_dn14, locals.var_nqs_a_dn15, locals.var_nqs_a_dn16, locals.var_nqs_a_dn17, locals.var_nqs_a_dn18, locals.var_nqs_a_dn19, locals.var_nqs_a_dn20,)
    }
};
        locals.var_nqs_a = assign82240_e123207;
        locals.var_nqs_a_dn5 = assign82240_e123207_d_n5;
        locals.var_nqs_a_dn6 = assign82240_e123207_d_n6;
        locals.var_nqs_a_dn7 = assign82240_e123207_d_n7;
        locals.var_nqs_a_dn8 = assign82240_e123207_d_n8;
        locals.var_nqs_a_dn12 = assign82240_e123207_d_n12;
        locals.var_nqs_a_dn13 = assign82240_e123207_d_n13;
        locals.var_nqs_a_dn14 = assign82240_e123207_d_n14;
        locals.var_nqs_a_dn15 = assign82240_e123207_d_n15;
        locals.var_nqs_a_dn16 = assign82240_e123207_d_n16;
        locals.var_nqs_a_dn17 = assign82240_e123207_d_n17;
        locals.var_nqs_a_dn18 = assign82240_e123207_d_n18;
        locals.var_nqs_a_dn19 = assign82240_e123207_d_n19;
        locals.var_nqs_a_dn20 = assign82240_e123207_d_n20;

        let (assign82250_e123236, assign82250_e123236_d_n5, assign82250_e123236_d_n6, assign82250_e123236_d_n7, assign82250_e123236_d_n8, assign82250_e123236_d_n12, assign82250_e123236_d_n13, assign82250_e123236_d_n14, assign82250_e123236_d_n15, assign82250_e123236_d_n16, assign82250_e123236_d_n17, assign82250_e123236_d_n18, assign82250_e123236_d_n19, assign82250_e123236_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82250_e123231: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82250_e123232: f64 = (2.0 * assign82250_e123231);
        let assign82250_e123234: f64 = (assign82250_e123232 - locals.var_gp2);
        (assign82250_e123234, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5)) - locals.var_gp2_dn5), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6)) - locals.var_gp2_dn6), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7)) - locals.var_gp2_dn7), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8)) - locals.var_gp2_dn8), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12)) - locals.var_gp2_dn12), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13)) - locals.var_gp2_dn13), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14)) - locals.var_gp2_dn14), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15)) - locals.var_gp2_dn15), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16)) - locals.var_gp2_dn16), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17)) - locals.var_gp2_dn17), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18)) - locals.var_gp2_dn18), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19)) - locals.var_gp2_dn19), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20)) - locals.var_gp2_dn20),)
    } else {
        (locals.var_nqs_c, locals.var_nqs_c_dn5, locals.var_nqs_c_dn6, locals.var_nqs_c_dn7, locals.var_nqs_c_dn8, locals.var_nqs_c_dn12, locals.var_nqs_c_dn13, locals.var_nqs_c_dn14, locals.var_nqs_c_dn15, locals.var_nqs_c_dn16, locals.var_nqs_c_dn17, locals.var_nqs_c_dn18, locals.var_nqs_c_dn19, locals.var_nqs_c_dn20,)
    }
};
        locals.var_nqs_c = assign82250_e123236;
        locals.var_nqs_c_dn5 = assign82250_e123236_d_n5;
        locals.var_nqs_c_dn6 = assign82250_e123236_d_n6;
        locals.var_nqs_c_dn7 = assign82250_e123236_d_n7;
        locals.var_nqs_c_dn8 = assign82250_e123236_d_n8;
        locals.var_nqs_c_dn12 = assign82250_e123236_d_n12;
        locals.var_nqs_c_dn13 = assign82250_e123236_d_n13;
        locals.var_nqs_c_dn14 = assign82250_e123236_d_n14;
        locals.var_nqs_c_dn15 = assign82250_e123236_d_n15;
        locals.var_nqs_c_dn16 = assign82250_e123236_d_n16;
        locals.var_nqs_c_dn17 = assign82250_e123236_d_n17;
        locals.var_nqs_c_dn18 = assign82250_e123236_d_n18;
        locals.var_nqs_c_dn19 = assign82250_e123236_d_n19;
        locals.var_nqs_c_dn20 = assign82250_e123236_d_n20;

        let (assign82260_e123264, assign82260_e123264_d_n5, assign82260_e123264_d_n6, assign82260_e123264_d_n7, assign82260_e123264_d_n8, assign82260_e123264_d_n12, assign82260_e123264_d_n13, assign82260_e123264_d_n14, assign82260_e123264_d_n15, assign82260_e123264_d_n16, assign82260_e123264_d_n17, assign82260_e123264_d_n18, assign82260_e123264_d_n19, assign82260_e123264_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82260_e123259: f64 = (locals.var_nqs_a / locals.var_gp2);
        let assign82260_e123260: f64 = (assign82260_e123259).ln();
        let assign82260_e123262: f64 = (assign82260_e123260 - locals.var_nqs_eta);
        (assign82260_e123262, (((((locals.var_nqs_a_dn5 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn5)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn5), (((((locals.var_nqs_a_dn6 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn6)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn6), (((((locals.var_nqs_a_dn7 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn7)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn7), (((((locals.var_nqs_a_dn8 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn8)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn8), (((((locals.var_nqs_a_dn12 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn12)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn12), (((((locals.var_nqs_a_dn13 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn13)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn13), (((((locals.var_nqs_a_dn14 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn14)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn14), (((((locals.var_nqs_a_dn15 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn15)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn15), (((((locals.var_nqs_a_dn16 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn16)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn16), (((((locals.var_nqs_a_dn17 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn17)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn17), (((((locals.var_nqs_a_dn18 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn18)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn18), (((((locals.var_nqs_a_dn19 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn19)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn19), (((((locals.var_nqs_a_dn20 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn20)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn20),)
    } else {
        (locals.var_nqs_tau, locals.var_nqs_tau_dn5, locals.var_nqs_tau_dn6, locals.var_nqs_tau_dn7, locals.var_nqs_tau_dn8, locals.var_nqs_tau_dn12, locals.var_nqs_tau_dn13, locals.var_nqs_tau_dn14, locals.var_nqs_tau_dn15, locals.var_nqs_tau_dn16, locals.var_nqs_tau_dn17, locals.var_nqs_tau_dn18, locals.var_nqs_tau_dn19, locals.var_nqs_tau_dn20,)
    }
};
        locals.var_nqs_tau = assign82260_e123264;
        locals.var_nqs_tau_dn5 = assign82260_e123264_d_n5;
        locals.var_nqs_tau_dn6 = assign82260_e123264_d_n6;
        locals.var_nqs_tau_dn7 = assign82260_e123264_d_n7;
        locals.var_nqs_tau_dn8 = assign82260_e123264_d_n8;
        locals.var_nqs_tau_dn12 = assign82260_e123264_d_n12;
        locals.var_nqs_tau_dn13 = assign82260_e123264_d_n13;
        locals.var_nqs_tau_dn14 = assign82260_e123264_d_n14;
        locals.var_nqs_tau_dn15 = assign82260_e123264_d_n15;
        locals.var_nqs_tau_dn16 = assign82260_e123264_d_n16;
        locals.var_nqs_tau_dn17 = assign82260_e123264_d_n17;
        locals.var_nqs_tau_dn18 = assign82260_e123264_d_n18;
        locals.var_nqs_tau_dn19 = assign82260_e123264_d_n19;
        locals.var_nqs_tau_dn20 = assign82260_e123264_d_n20;

        let (assign82270_e123289, assign82270_e123289_d_n5, assign82270_e123289_d_n6, assign82270_e123289_d_n7, assign82270_e123289_d_n8, assign82270_e123289_d_n12, assign82270_e123289_d_n13, assign82270_e123289_d_n14, assign82270_e123289_d_n15, assign82270_e123289_d_n16, assign82270_e123289_d_n17, assign82270_e123289_d_n18, assign82270_e123289_d_n19, assign82270_e123289_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82270_e123287: f64 = (locals.var_nqs_a + locals.var_nqs_c);
        (assign82270_e123287, (locals.var_nqs_a_dn5 + locals.var_nqs_c_dn5), (locals.var_nqs_a_dn6 + locals.var_nqs_c_dn6), (locals.var_nqs_a_dn7 + locals.var_nqs_c_dn7), (locals.var_nqs_a_dn8 + locals.var_nqs_c_dn8), (locals.var_nqs_a_dn12 + locals.var_nqs_c_dn12), (locals.var_nqs_a_dn13 + locals.var_nqs_c_dn13), (locals.var_nqs_a_dn14 + locals.var_nqs_c_dn14), (locals.var_nqs_a_dn15 + locals.var_nqs_c_dn15), (locals.var_nqs_a_dn16 + locals.var_nqs_c_dn16), (locals.var_nqs_a_dn17 + locals.var_nqs_c_dn17), (locals.var_nqs_a_dn18 + locals.var_nqs_c_dn18), (locals.var_nqs_a_dn19 + locals.var_nqs_c_dn19), (locals.var_nqs_a_dn20 + locals.var_nqs_c_dn20),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn12, locals.var_nu_dn13, locals.var_nu_dn14, locals.var_nu_dn15, locals.var_nu_dn16, locals.var_nu_dn17, locals.var_nu_dn18, locals.var_nu_dn19, locals.var_nu_dn20,)
    }
};
        locals.var_nu = assign82270_e123289;
        locals.var_nu_dn5 = assign82270_e123289_d_n5;
        locals.var_nu_dn6 = assign82270_e123289_d_n6;
        locals.var_nu_dn7 = assign82270_e123289_d_n7;
        locals.var_nu_dn8 = assign82270_e123289_d_n8;
        locals.var_nu_dn12 = assign82270_e123289_d_n12;
        locals.var_nu_dn13 = assign82270_e123289_d_n13;
        locals.var_nu_dn14 = assign82270_e123289_d_n14;
        locals.var_nu_dn15 = assign82270_e123289_d_n15;
        locals.var_nu_dn16 = assign82270_e123289_d_n16;
        locals.var_nu_dn17 = assign82270_e123289_d_n17;
        locals.var_nu_dn18 = assign82270_e123289_d_n18;
        locals.var_nu_dn19 = assign82270_e123289_d_n19;
        locals.var_nu_dn20 = assign82270_e123289_d_n20;

        let (assign82280_e123324, assign82280_e123324_d_n5, assign82280_e123324_d_n6, assign82280_e123324_d_n7, assign82280_e123324_d_n8, assign82280_e123324_d_n12, assign82280_e123324_d_n13, assign82280_e123324_d_n14, assign82280_e123324_d_n15, assign82280_e123324_d_n16, assign82280_e123324_d_n17, assign82280_e123324_d_n18, assign82280_e123324_d_n19, assign82280_e123324_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82280_e123312: f64 = (locals.var_nu * locals.var_nu);
        let assign82280_e123317: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign82280_e123318: f64 = (0.5 * assign82280_e123317);
        let assign82280_e123320: f64 = (assign82280_e123318 - locals.var_nqs_a);
        let assign82280_e123321: f64 = (locals.var_nqs_tau * assign82280_e123320);
        let assign82280_e123322: f64 = (assign82280_e123312 + assign82280_e123321);
        (assign82280_e123322, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_nqs_tau_dn5 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5))) - locals.var_nqs_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_nqs_tau_dn6 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6))) - locals.var_nqs_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_nqs_tau_dn7 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7))) - locals.var_nqs_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_nqs_tau_dn8 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8))) - locals.var_nqs_a_dn8)))), (((locals.var_nu_dn12 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn12)) + ((locals.var_nqs_tau_dn12 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12))) - locals.var_nqs_a_dn12)))), (((locals.var_nu_dn13 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn13)) + ((locals.var_nqs_tau_dn13 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13))) - locals.var_nqs_a_dn13)))), (((locals.var_nu_dn14 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn14)) + ((locals.var_nqs_tau_dn14 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14))) - locals.var_nqs_a_dn14)))), (((locals.var_nu_dn15 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn15)) + ((locals.var_nqs_tau_dn15 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15))) - locals.var_nqs_a_dn15)))), (((locals.var_nu_dn16 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn16)) + ((locals.var_nqs_tau_dn16 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16))) - locals.var_nqs_a_dn16)))), (((locals.var_nu_dn17 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn17)) + ((locals.var_nqs_tau_dn17 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17))) - locals.var_nqs_a_dn17)))), (((locals.var_nu_dn18 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn18)) + ((locals.var_nqs_tau_dn18 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18))) - locals.var_nqs_a_dn18)))), (((locals.var_nu_dn19 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn19)) + ((locals.var_nqs_tau_dn19 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19))) - locals.var_nqs_a_dn19)))), (((locals.var_nu_dn20 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn20)) + ((locals.var_nqs_tau_dn20 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20))) - locals.var_nqs_a_dn20)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn12, locals.var_mutau_dn13, locals.var_mutau_dn14, locals.var_mutau_dn15, locals.var_mutau_dn16, locals.var_mutau_dn17, locals.var_mutau_dn18, locals.var_mutau_dn19, locals.var_mutau_dn20,)
    }
};
        locals.var_mutau = assign82280_e123324;
        locals.var_mutau_dn5 = assign82280_e123324_d_n5;
        locals.var_mutau_dn6 = assign82280_e123324_d_n6;
        locals.var_mutau_dn7 = assign82280_e123324_d_n7;
        locals.var_mutau_dn8 = assign82280_e123324_d_n8;
        locals.var_mutau_dn12 = assign82280_e123324_d_n12;
        locals.var_mutau_dn13 = assign82280_e123324_d_n13;
        locals.var_mutau_dn14 = assign82280_e123324_d_n14;
        locals.var_mutau_dn15 = assign82280_e123324_d_n15;
        locals.var_mutau_dn16 = assign82280_e123324_d_n16;
        locals.var_mutau_dn17 = assign82280_e123324_d_n17;
        locals.var_mutau_dn18 = assign82280_e123324_d_n18;
        locals.var_mutau_dn19 = assign82280_e123324_d_n19;
        locals.var_mutau_dn20 = assign82280_e123324_d_n20;

        let (assign82290_e123373, assign82290_e123373_d_n5, assign82290_e123373_d_n6, assign82290_e123373_d_n7, assign82290_e123373_d_n8, assign82290_e123373_d_n12, assign82290_e123373_d_n13, assign82290_e123373_d_n14, assign82290_e123373_d_n15, assign82290_e123373_d_n16, assign82290_e123373_d_n17, assign82290_e123373_d_n18, assign82290_e123373_d_n19, assign82290_e123373_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82290_e123348: f64 = (locals.var_nqs_a * locals.var_nu);
        let assign82290_e123350: f64 = (assign82290_e123348 * locals.var_nqs_tau);
        let assign82290_e123354: f64 = (locals.var_nu / locals.var_mutau);
        let assign82290_e123356: f64 = (assign82290_e123354 * locals.var_nqs_tau);
        let assign82290_e123358: f64 = (assign82290_e123356 * locals.var_nqs_tau);
        let assign82290_e123360: f64 = (assign82290_e123358 * locals.var_nqs_c);
        let assign82290_e123363: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign82290_e123365: f64 = (assign82290_e123363 * 0.3333333333333333);
        let assign82290_e123367: f64 = (assign82290_e123365 - locals.var_nqs_a);
        let assign82290_e123368: f64 = (assign82290_e123360 * assign82290_e123367);
        let assign82290_e123369: f64 = (locals.var_mutau + assign82290_e123368);
        let assign82290_e123370: f64 = (assign82290_e123350 / assign82290_e123369);
        let assign82290_e123371: f64 = (locals.var_nqs_eta + assign82290_e123370);
        (assign82290_e123371, (locals.var_nqs_eta_dn5 + (((((((locals.var_nqs_a_dn5 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn5)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn5)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn5)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn5)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn5)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5)) * 0.3333333333333333) - locals.var_nqs_a_dn5)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn6 + (((((((locals.var_nqs_a_dn6 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn6)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn6)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn6)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn6)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn6)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6)) * 0.3333333333333333) - locals.var_nqs_a_dn6)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn7 + (((((((locals.var_nqs_a_dn7 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn7)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn7)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn7)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn7)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn7)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7)) * 0.3333333333333333) - locals.var_nqs_a_dn7)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn8 + (((((((locals.var_nqs_a_dn8 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn8)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn8)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn8)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn8)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn8)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8)) * 0.3333333333333333) - locals.var_nqs_a_dn8)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn12 + (((((((locals.var_nqs_a_dn12 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn12)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn12)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn12 + (((((((((((locals.var_nu_dn12 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn12)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn12)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn12)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn12)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12)) * 0.3333333333333333) - locals.var_nqs_a_dn12)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn13 + (((((((locals.var_nqs_a_dn13 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn13)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn13)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn13 + (((((((((((locals.var_nu_dn13 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn13)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn13)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn13)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn13)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13)) * 0.3333333333333333) - locals.var_nqs_a_dn13)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn14 + (((((((locals.var_nqs_a_dn14 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn14)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn14)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn14 + (((((((((((locals.var_nu_dn14 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn14)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn14)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn14)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn14)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14)) * 0.3333333333333333) - locals.var_nqs_a_dn14)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn15 + (((((((locals.var_nqs_a_dn15 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn15)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn15)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn15 + (((((((((((locals.var_nu_dn15 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn15)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn15)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn15)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn15)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15)) * 0.3333333333333333) - locals.var_nqs_a_dn15)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn16 + (((((((locals.var_nqs_a_dn16 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn16)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn16)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn16 + (((((((((((locals.var_nu_dn16 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn16)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn16)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn16)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn16)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16)) * 0.3333333333333333) - locals.var_nqs_a_dn16)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn17 + (((((((locals.var_nqs_a_dn17 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn17)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn17)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn17 + (((((((((((locals.var_nu_dn17 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn17)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn17)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn17)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn17)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17)) * 0.3333333333333333) - locals.var_nqs_a_dn17)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn18 + (((((((locals.var_nqs_a_dn18 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn18)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn18)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn18 + (((((((((((locals.var_nu_dn18 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn18)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn18)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn18)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn18)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18)) * 0.3333333333333333) - locals.var_nqs_a_dn18)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn19 + (((((((locals.var_nqs_a_dn19 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn19)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn19)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn19 + (((((((((((locals.var_nu_dn19 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn19)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn19)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn19)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn19)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19)) * 0.3333333333333333) - locals.var_nqs_a_dn19)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn20 + (((((((locals.var_nqs_a_dn20 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn20)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn20)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn20 + (((((((((((locals.var_nu_dn20 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn20)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn20)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn20)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn20)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20)) * 0.3333333333333333) - locals.var_nqs_a_dn20)))))) / (assign82290_e123369 * assign82290_e123369))),)
    } else {
        (locals.var_nqs_y0, locals.var_nqs_y0_dn5, locals.var_nqs_y0_dn6, locals.var_nqs_y0_dn7, locals.var_nqs_y0_dn8, locals.var_nqs_y0_dn12, locals.var_nqs_y0_dn13, locals.var_nqs_y0_dn14, locals.var_nqs_y0_dn15, locals.var_nqs_y0_dn16, locals.var_nqs_y0_dn17, locals.var_nqs_y0_dn18, locals.var_nqs_y0_dn19, locals.var_nqs_y0_dn20,)
    }
};
        locals.var_nqs_y0 = assign82290_e123373;
        locals.var_nqs_y0_dn5 = assign82290_e123373_d_n5;
        locals.var_nqs_y0_dn6 = assign82290_e123373_d_n6;
        locals.var_nqs_y0_dn7 = assign82290_e123373_d_n7;
        locals.var_nqs_y0_dn8 = assign82290_e123373_d_n8;
        locals.var_nqs_y0_dn12 = assign82290_e123373_d_n12;
        locals.var_nqs_y0_dn13 = assign82290_e123373_d_n13;
        locals.var_nqs_y0_dn14 = assign82290_e123373_d_n14;
        locals.var_nqs_y0_dn15 = assign82290_e123373_d_n15;
        locals.var_nqs_y0_dn16 = assign82290_e123373_d_n16;
        locals.var_nqs_y0_dn17 = assign82290_e123373_d_n17;
        locals.var_nqs_y0_dn18 = assign82290_e123373_d_n18;
        locals.var_nqs_y0_dn19 = assign82290_e123373_d_n19;
        locals.var_nqs_y0_dn20 = assign82290_e123373_d_n20;

        let assign82300_e123375: f64 = (locals.var_nqs_y0).abs();
        let assign82300_e123377: f64 = if assign82300_e123375 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign82300_e123377;

    }

    pub(super) fn stamp_transient_block_163(
        locals: &mut StampLocals,
    ) {
        let (assign82310_e123403, assign82310_e123403_d_n5, assign82310_e123403_d_n6, assign82310_e123403_d_n7, assign82310_e123403_d_n8, assign82310_e123403_d_n12, assign82310_e123403_d_n13, assign82310_e123403_d_n14, assign82310_e123403_d_n15, assign82310_e123403_d_n16, assign82310_e123403_d_n17, assign82310_e123403_d_n18, assign82310_e123403_d_n19, assign82310_e123403_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign82310_e123401: f64 = (locals.var_nqs_y0).exp();
        (assign82310_e123401, (assign82310_e123401 * locals.var_nqs_y0_dn5), (assign82310_e123401 * locals.var_nqs_y0_dn6), (assign82310_e123401 * locals.var_nqs_y0_dn7), (assign82310_e123401 * locals.var_nqs_y0_dn8), (assign82310_e123401 * locals.var_nqs_y0_dn12), (assign82310_e123401 * locals.var_nqs_y0_dn13), (assign82310_e123401 * locals.var_nqs_y0_dn14), (assign82310_e123401 * locals.var_nqs_y0_dn15), (assign82310_e123401 * locals.var_nqs_y0_dn16), (assign82310_e123401 * locals.var_nqs_y0_dn17), (assign82310_e123401 * locals.var_nqs_y0_dn18), (assign82310_e123401 * locals.var_nqs_y0_dn19), (assign82310_e123401 * locals.var_nqs_y0_dn20),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82310_e123403;
        locals.var_nqs_d0_dn5 = assign82310_e123403_d_n5;
        locals.var_nqs_d0_dn6 = assign82310_e123403_d_n6;
        locals.var_nqs_d0_dn7 = assign82310_e123403_d_n7;
        locals.var_nqs_d0_dn8 = assign82310_e123403_d_n8;
        locals.var_nqs_d0_dn12 = assign82310_e123403_d_n12;
        locals.var_nqs_d0_dn13 = assign82310_e123403_d_n13;
        locals.var_nqs_d0_dn14 = assign82310_e123403_d_n14;
        locals.var_nqs_d0_dn15 = assign82310_e123403_d_n15;
        locals.var_nqs_d0_dn16 = assign82310_e123403_d_n16;
        locals.var_nqs_d0_dn17 = assign82310_e123403_d_n17;
        locals.var_nqs_d0_dn18 = assign82310_e123403_d_n18;
        locals.var_nqs_d0_dn19 = assign82310_e123403_d_n19;
        locals.var_nqs_d0_dn20 = assign82310_e123403_d_n20;

        let assign82320_e123406: f64 = if locals.var_nqs_y0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign82320_e123406;

        let (assign82330_e123459, assign82330_e123459_d_n5, assign82330_e123459_d_n6, assign82330_e123459_d_n7, assign82330_e123459_d_n8, assign82330_e123459_d_n12, assign82330_e123459_d_n13, assign82330_e123459_d_n14, assign82330_e123459_d_n15, assign82330_e123459_d_n16, assign82330_e123459_d_n17, assign82330_e123459_d_n18, assign82330_e123459_d_n19, assign82330_e123459_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign82330_e123435: f64 = (-230.25850929940458);
        let assign82330_e123437: f64 = (assign82330_e123435 - locals.var_nqs_y0);
        let assign82330_e123441: f64 = (-230.25850929940458);
        let assign82330_e123443: f64 = (assign82330_e123441 - locals.var_nqs_y0);
        let assign82330_e123446: f64 = (-230.25850929940458);
        let assign82330_e123448: f64 = (assign82330_e123446 - locals.var_nqs_y0);
        let assign82330_e123450: f64 = (assign82330_e123448 * 0.3333333333333333);
        let assign82330_e123451: f64 = (1.0 + assign82330_e123450);
        let assign82330_e123452: f64 = (assign82330_e123443 * assign82330_e123451);
        let assign82330_e123453: f64 = (0.5 * assign82330_e123452);
        let assign82330_e123454: f64 = (1.0 + assign82330_e123453);
        let assign82330_e123455: f64 = (assign82330_e123437 * assign82330_e123454);
        let assign82330_e123456: f64 = (1.0 + assign82330_e123455);
        let assign82330_e123457: f64 = (1e-100 / assign82330_e123456);
        (assign82330_e123457, (-((1e-100 * (((-locals.var_nqs_y0_dn5) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn5) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn5) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn6) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn6) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn6) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn7) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn7) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn7) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn8) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn8) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn8) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn12) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn12) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn12) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn13) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn13) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn13) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn14) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn14) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn14) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn15) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn15) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn15) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn16) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn16) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn16) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn17) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn17) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn17) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn18) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn18) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn18) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn19) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn19) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn19) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn20) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn20) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn20) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82330_e123459;
        locals.var_nqs_d0_dn5 = assign82330_e123459_d_n5;
        locals.var_nqs_d0_dn6 = assign82330_e123459_d_n6;
        locals.var_nqs_d0_dn7 = assign82330_e123459_d_n7;
        locals.var_nqs_d0_dn8 = assign82330_e123459_d_n8;
        locals.var_nqs_d0_dn12 = assign82330_e123459_d_n12;
        locals.var_nqs_d0_dn13 = assign82330_e123459_d_n13;
        locals.var_nqs_d0_dn14 = assign82330_e123459_d_n14;
        locals.var_nqs_d0_dn15 = assign82330_e123459_d_n15;
        locals.var_nqs_d0_dn16 = assign82330_e123459_d_n16;
        locals.var_nqs_d0_dn17 = assign82330_e123459_d_n17;
        locals.var_nqs_d0_dn18 = assign82330_e123459_d_n18;
        locals.var_nqs_d0_dn19 = assign82330_e123459_d_n19;
        locals.var_nqs_d0_dn20 = assign82330_e123459_d_n20;

        let (assign82340_e123510, assign82340_e123510_d_n5, assign82340_e123510_d_n6, assign82340_e123510_d_n7, assign82340_e123510_d_n8, assign82340_e123510_d_n12, assign82340_e123510_d_n13, assign82340_e123510_d_n14, assign82340_e123510_d_n15, assign82340_e123510_d_n16, assign82340_e123510_d_n17, assign82340_e123510_d_n18, assign82340_e123510_d_n19, assign82340_e123510_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign82340_e123490: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123495: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123499: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123501: f64 = (assign82340_e123499 * 0.3333333333333333);
        let assign82340_e123502: f64 = (1.0 + assign82340_e123501);
        let assign82340_e123503: f64 = (assign82340_e123495 * assign82340_e123502);
        let assign82340_e123504: f64 = (0.5 * assign82340_e123503);
        let assign82340_e123505: f64 = (1.0 + assign82340_e123504);
        let assign82340_e123506: f64 = (assign82340_e123490 * assign82340_e123505);
        let assign82340_e123507: f64 = (1.0 + assign82340_e123506);
        let assign82340_e123508: f64 = (1e100 * assign82340_e123507);
        (assign82340_e123508, (1e100 * ((locals.var_nqs_y0_dn5 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn5 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn6 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn6 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn7 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn7 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn8 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn8 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn12 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn12 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn13 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn13 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn14 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn14 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn15 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn15 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn16 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn16 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn17 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn17 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn18 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn18 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn19 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn19 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn20 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn20 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82340_e123510;
        locals.var_nqs_d0_dn5 = assign82340_e123510_d_n5;
        locals.var_nqs_d0_dn6 = assign82340_e123510_d_n6;
        locals.var_nqs_d0_dn7 = assign82340_e123510_d_n7;
        locals.var_nqs_d0_dn8 = assign82340_e123510_d_n8;
        locals.var_nqs_d0_dn12 = assign82340_e123510_d_n12;
        locals.var_nqs_d0_dn13 = assign82340_e123510_d_n13;
        locals.var_nqs_d0_dn14 = assign82340_e123510_d_n14;
        locals.var_nqs_d0_dn15 = assign82340_e123510_d_n15;
        locals.var_nqs_d0_dn16 = assign82340_e123510_d_n16;
        locals.var_nqs_d0_dn17 = assign82340_e123510_d_n17;
        locals.var_nqs_d0_dn18 = assign82340_e123510_d_n18;
        locals.var_nqs_d0_dn19 = assign82340_e123510_d_n19;
        locals.var_nqs_d0_dn20 = assign82340_e123510_d_n20;

        let (assign82350_e123539, assign82350_e123539_d_n5, assign82350_e123539_d_n6, assign82350_e123539_d_n7, assign82350_e123539_d_n8, assign82350_e123539_d_n12, assign82350_e123539_d_n13, assign82350_e123539_d_n14, assign82350_e123539_d_n15, assign82350_e123539_d_n16, assign82350_e123539_d_n17, assign82350_e123539_d_n18, assign82350_e123539_d_n19, assign82350_e123539_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82350_e123534: f64 = (locals.var_gp2 * locals.var_nqs_d0);
        let assign82350_e123536: f64 = (assign82350_e123534 * 0.5);
        let assign82350_e123537: f64 = (1.0 - assign82350_e123536);
        (assign82350_e123537, (-(((locals.var_gp2_dn5 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn5)) * 0.5)), (-(((locals.var_gp2_dn6 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn6)) * 0.5)), (-(((locals.var_gp2_dn7 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn7)) * 0.5)), (-(((locals.var_gp2_dn8 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn8)) * 0.5)), (-(((locals.var_gp2_dn12 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn12)) * 0.5)), (-(((locals.var_gp2_dn13 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn13)) * 0.5)), (-(((locals.var_gp2_dn14 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn14)) * 0.5)), (-(((locals.var_gp2_dn15 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn15)) * 0.5)), (-(((locals.var_gp2_dn16 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn16)) * 0.5)), (-(((locals.var_gp2_dn17 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn17)) * 0.5)), (-(((locals.var_gp2_dn18 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn18)) * 0.5)), (-(((locals.var_gp2_dn19 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn19)) * 0.5)), (-(((locals.var_gp2_dn20 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn20)) * 0.5)),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82350_e123539;
        locals.var_nqs_xi_dn5 = assign82350_e123539_d_n5;
        locals.var_nqs_xi_dn6 = assign82350_e123539_d_n6;
        locals.var_nqs_xi_dn7 = assign82350_e123539_d_n7;
        locals.var_nqs_xi_dn8 = assign82350_e123539_d_n8;
        locals.var_nqs_xi_dn12 = assign82350_e123539_d_n12;
        locals.var_nqs_xi_dn13 = assign82350_e123539_d_n13;
        locals.var_nqs_xi_dn14 = assign82350_e123539_d_n14;
        locals.var_nqs_xi_dn15 = assign82350_e123539_d_n15;
        locals.var_nqs_xi_dn16 = assign82350_e123539_d_n16;
        locals.var_nqs_xi_dn17 = assign82350_e123539_d_n17;
        locals.var_nqs_xi_dn18 = assign82350_e123539_d_n18;
        locals.var_nqs_xi_dn19 = assign82350_e123539_d_n19;
        locals.var_nqs_xi_dn20 = assign82350_e123539_d_n20;

        let (assign82360_e123572, assign82360_e123572_d_n5, assign82360_e123572_d_n6, assign82360_e123572_d_n7, assign82360_e123572_d_n8, assign82360_e123572_d_n12, assign82360_e123572_d_n13, assign82360_e123572_d_n14, assign82360_e123572_d_n15, assign82360_e123572_d_n16, assign82360_e123572_d_n17, assign82360_e123572_d_n18, assign82360_e123572_d_n19, assign82360_e123572_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82360_e123563: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82360_e123564: f64 = (2.0 * assign82360_e123563);
        let assign82360_e123568: f64 = (locals.var_nqs_d0 - 1.0);
        let assign82360_e123569: f64 = (locals.var_gp2 * assign82360_e123568);
        let assign82360_e123570: f64 = (assign82360_e123564 + assign82360_e123569);
        (assign82360_e123570, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5)) + ((locals.var_gp2_dn5 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn5))), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6)) + ((locals.var_gp2_dn6 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn6))), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7)) + ((locals.var_gp2_dn7 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn7))), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8)) + ((locals.var_gp2_dn8 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn8))), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12)) + ((locals.var_gp2_dn12 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn12))), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13)) + ((locals.var_gp2_dn13 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn13))), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14)) + ((locals.var_gp2_dn14 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn14))), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15)) + ((locals.var_gp2_dn15 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn15))), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16)) + ((locals.var_gp2_dn16 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn16))), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17)) + ((locals.var_gp2_dn17 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn17))), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18)) + ((locals.var_gp2_dn18 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn18))), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19)) + ((locals.var_gp2_dn19 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn19))), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20)) + ((locals.var_gp2_dn20 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82360_e123572;
        locals.var_nqs_p_dn5 = assign82360_e123572_d_n5;
        locals.var_nqs_p_dn6 = assign82360_e123572_d_n6;
        locals.var_nqs_p_dn7 = assign82360_e123572_d_n7;
        locals.var_nqs_p_dn8 = assign82360_e123572_d_n8;
        locals.var_nqs_p_dn12 = assign82360_e123572_d_n12;
        locals.var_nqs_p_dn13 = assign82360_e123572_d_n13;
        locals.var_nqs_p_dn14 = assign82360_e123572_d_n14;
        locals.var_nqs_p_dn15 = assign82360_e123572_d_n15;
        locals.var_nqs_p_dn16 = assign82360_e123572_d_n16;
        locals.var_nqs_p_dn17 = assign82360_e123572_d_n17;
        locals.var_nqs_p_dn18 = assign82360_e123572_d_n18;
        locals.var_nqs_p_dn19 = assign82360_e123572_d_n19;
        locals.var_nqs_p_dn20 = assign82360_e123572_d_n20;

        let (assign82370_e123609, assign82370_e123609_d_n5, assign82370_e123609_d_n6, assign82370_e123609_d_n7, assign82370_e123609_d_n8, assign82370_e123609_d_n12, assign82370_e123609_d_n13, assign82370_e123609_d_n14, assign82370_e123609_d_n15, assign82370_e123609_d_n16, assign82370_e123609_d_n17, assign82370_e123609_d_n18, assign82370_e123609_d_n19, assign82370_e123609_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82370_e123595: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82370_e123598: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82370_e123599: f64 = (assign82370_e123595 * assign82370_e123598);
        let assign82370_e123603: f64 = (locals.var_nqs_y0 + 1.0);
        let assign82370_e123605: f64 = (assign82370_e123603 - locals.var_nqs_d0);
        let assign82370_e123606: f64 = (locals.var_gp2 * assign82370_e123605);
        let assign82370_e123607: f64 = (assign82370_e123599 + assign82370_e123606);
        (assign82370_e123607, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5))) + ((locals.var_gp2_dn5 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn5 - locals.var_nqs_d0_dn5)))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6))) + ((locals.var_gp2_dn6 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn6 - locals.var_nqs_d0_dn6)))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7))) + ((locals.var_gp2_dn7 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn7 - locals.var_nqs_d0_dn7)))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8))) + ((locals.var_gp2_dn8 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn8 - locals.var_nqs_d0_dn8)))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12))) + ((locals.var_gp2_dn12 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn12 - locals.var_nqs_d0_dn12)))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13))) + ((locals.var_gp2_dn13 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn13 - locals.var_nqs_d0_dn13)))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14))) + ((locals.var_gp2_dn14 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn14 - locals.var_nqs_d0_dn14)))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15))) + ((locals.var_gp2_dn15 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn15 - locals.var_nqs_d0_dn15)))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16))) + ((locals.var_gp2_dn16 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn16 - locals.var_nqs_d0_dn16)))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17))) + ((locals.var_gp2_dn17 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn17 - locals.var_nqs_d0_dn17)))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18))) + ((locals.var_gp2_dn18 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn18 - locals.var_nqs_d0_dn18)))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19))) + ((locals.var_gp2_dn19 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn19 - locals.var_nqs_d0_dn19)))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20))) + ((locals.var_gp2_dn20 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn20 - locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82370_e123609;
        locals.var_nqs_q_dn5 = assign82370_e123609_d_n5;
        locals.var_nqs_q_dn6 = assign82370_e123609_d_n6;
        locals.var_nqs_q_dn7 = assign82370_e123609_d_n7;
        locals.var_nqs_q_dn8 = assign82370_e123609_d_n8;
        locals.var_nqs_q_dn12 = assign82370_e123609_d_n12;
        locals.var_nqs_q_dn13 = assign82370_e123609_d_n13;
        locals.var_nqs_q_dn14 = assign82370_e123609_d_n14;
        locals.var_nqs_q_dn15 = assign82370_e123609_d_n15;
        locals.var_nqs_q_dn16 = assign82370_e123609_d_n16;
        locals.var_nqs_q_dn17 = assign82370_e123609_d_n17;
        locals.var_nqs_q_dn18 = assign82370_e123609_d_n18;
        locals.var_nqs_q_dn19 = assign82370_e123609_d_n19;
        locals.var_nqs_q_dn20 = assign82370_e123609_d_n20;

        let (assign82380_e123640, assign82380_e123640_d_n5, assign82380_e123640_d_n6, assign82380_e123640_d_n7, assign82380_e123640_d_n8, assign82380_e123640_d_n12, assign82380_e123640_d_n13, assign82380_e123640_d_n14, assign82380_e123640_d_n15, assign82380_e123640_d_n16, assign82380_e123640_d_n17, assign82380_e123640_d_n18, assign82380_e123640_d_n19, assign82380_e123640_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82380_e123632: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82380_e123635: f64 = (4.0 * locals.var_nqs_xi);
        let assign82380_e123637: f64 = (assign82380_e123635 * locals.var_nqs_q);
        let assign82380_e123638: f64 = (assign82380_e123632 - assign82380_e123637);
        (assign82380_e123638, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82380_e123640;
        locals.var_nqs_temp_dn5 = assign82380_e123640_d_n5;
        locals.var_nqs_temp_dn6 = assign82380_e123640_d_n6;
        locals.var_nqs_temp_dn7 = assign82380_e123640_d_n7;
        locals.var_nqs_temp_dn8 = assign82380_e123640_d_n8;
        locals.var_nqs_temp_dn12 = assign82380_e123640_d_n12;
        locals.var_nqs_temp_dn13 = assign82380_e123640_d_n13;
        locals.var_nqs_temp_dn14 = assign82380_e123640_d_n14;
        locals.var_nqs_temp_dn15 = assign82380_e123640_d_n15;
        locals.var_nqs_temp_dn16 = assign82380_e123640_d_n16;
        locals.var_nqs_temp_dn17 = assign82380_e123640_d_n17;
        locals.var_nqs_temp_dn18 = assign82380_e123640_d_n18;
        locals.var_nqs_temp_dn19 = assign82380_e123640_d_n19;
        locals.var_nqs_temp_dn20 = assign82380_e123640_d_n20;

        let (assign82390_e123670, assign82390_e123670_d_n5, assign82390_e123670_d_n6, assign82390_e123670_d_n7, assign82390_e123670_d_n8, assign82390_e123670_d_n12, assign82390_e123670_d_n13, assign82390_e123670_d_n14, assign82390_e123670_d_n15, assign82390_e123670_d_n16, assign82390_e123670_d_n17, assign82390_e123670_d_n18, assign82390_e123670_d_n19, assign82390_e123670_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82390_e123663: f64 = (2.0 * locals.var_nqs_q);
        let assign82390_e123666: f64 = (locals.var_nqs_temp).sqrt();
        let assign82390_e123667: f64 = (locals.var_nqs_p + assign82390_e123666);
        let assign82390_e123668: f64 = (assign82390_e123663 / assign82390_e123667);
        (assign82390_e123668, ((((2.0 * locals.var_nqs_q_dn5) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn6) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn7) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn8) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn12) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn13) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn14) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn15) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn16) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn17) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn18) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn19) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn20) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82390_e123670;
        locals.var_nqs_w_dn5 = assign82390_e123670_d_n5;
        locals.var_nqs_w_dn6 = assign82390_e123670_d_n6;
        locals.var_nqs_w_dn7 = assign82390_e123670_d_n7;
        locals.var_nqs_w_dn8 = assign82390_e123670_d_n8;
        locals.var_nqs_w_dn12 = assign82390_e123670_d_n12;
        locals.var_nqs_w_dn13 = assign82390_e123670_d_n13;
        locals.var_nqs_w_dn14 = assign82390_e123670_d_n14;
        locals.var_nqs_w_dn15 = assign82390_e123670_d_n15;
        locals.var_nqs_w_dn16 = assign82390_e123670_d_n16;
        locals.var_nqs_w_dn17 = assign82390_e123670_d_n17;
        locals.var_nqs_w_dn18 = assign82390_e123670_d_n18;
        locals.var_nqs_w_dn19 = assign82390_e123670_d_n19;
        locals.var_nqs_w_dn20 = assign82390_e123670_d_n20;

        let (assign82400_e123696, assign82400_e123696_d_n5, assign82400_e123696_d_n6, assign82400_e123696_d_n7, assign82400_e123696_d_n8, assign82400_e123696_d_n12, assign82400_e123696_d_n13, assign82400_e123696_d_n14, assign82400_e123696_d_n15, assign82400_e123696_d_n16, assign82400_e123696_d_n17, assign82400_e123696_d_n18, assign82400_e123696_d_n19, assign82400_e123696_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82400_e123693: f64 = (locals.var_nqs_y0 + locals.var_nqs_w);
        let assign82400_e123694: f64 = (-assign82400_e123693);
        (assign82400_e123694, (-(locals.var_nqs_y0_dn5 + locals.var_nqs_w_dn5)), (-(locals.var_nqs_y0_dn6 + locals.var_nqs_w_dn6)), (-(locals.var_nqs_y0_dn7 + locals.var_nqs_w_dn7)), (-(locals.var_nqs_y0_dn8 + locals.var_nqs_w_dn8)), (-(locals.var_nqs_y0_dn12 + locals.var_nqs_w_dn12)), (-(locals.var_nqs_y0_dn13 + locals.var_nqs_w_dn13)), (-(locals.var_nqs_y0_dn14 + locals.var_nqs_w_dn14)), (-(locals.var_nqs_y0_dn15 + locals.var_nqs_w_dn15)), (-(locals.var_nqs_y0_dn16 + locals.var_nqs_w_dn16)), (-(locals.var_nqs_y0_dn17 + locals.var_nqs_w_dn17)), (-(locals.var_nqs_y0_dn18 + locals.var_nqs_w_dn18)), (-(locals.var_nqs_y0_dn19 + locals.var_nqs_w_dn19)), (-(locals.var_nqs_y0_dn20 + locals.var_nqs_w_dn20)),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82400_e123696;
        locals.var_temp9_dn5 = assign82400_e123696_d_n5;
        locals.var_temp9_dn6 = assign82400_e123696_d_n6;
        locals.var_temp9_dn7 = assign82400_e123696_d_n7;
        locals.var_temp9_dn8 = assign82400_e123696_d_n8;
        locals.var_temp9_dn12 = assign82400_e123696_d_n12;
        locals.var_temp9_dn13 = assign82400_e123696_d_n13;
        locals.var_temp9_dn14 = assign82400_e123696_d_n14;
        locals.var_temp9_dn15 = assign82400_e123696_d_n15;
        locals.var_temp9_dn16 = assign82400_e123696_d_n16;
        locals.var_temp9_dn17 = assign82400_e123696_d_n17;
        locals.var_temp9_dn18 = assign82400_e123696_d_n18;
        locals.var_temp9_dn19 = assign82400_e123696_d_n19;
        locals.var_temp9_dn20 = assign82400_e123696_d_n20;

        let (assign82410_e123726, assign82410_e123726_d_n5, assign82410_e123726_d_n6, assign82410_e123726_d_n7, assign82410_e123726_d_n8, assign82410_e123726_d_n12, assign82410_e123726_d_n13, assign82410_e123726_d_n14, assign82410_e123726_d_n15, assign82410_e123726_d_n16, assign82410_e123726_d_n17, assign82410_e123726_d_n18, assign82410_e123726_d_n19, assign82410_e123726_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82410_e123722: f64 = (0.732464877560822 * locals.var_gp);
        let assign82410_e123723: f64 = (1.25 + assign82410_e123722);
        let assign82410_e123724: f64 = (1.0 / assign82410_e123723);
        (assign82410_e123724, (-((0.732464877560822 * locals.var_gp_dn5) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn6) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn7) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn8) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn12) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn13) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn14) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn15) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn16) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn17) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn18) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn19) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn20) / (assign82410_e123723 * assign82410_e123723))),)
    } else {
        (locals.var_nqs_xg1, locals.var_nqs_xg1_dn5, locals.var_nqs_xg1_dn6, locals.var_nqs_xg1_dn7, locals.var_nqs_xg1_dn8, locals.var_nqs_xg1_dn12, locals.var_nqs_xg1_dn13, locals.var_nqs_xg1_dn14, locals.var_nqs_xg1_dn15, locals.var_nqs_xg1_dn16, locals.var_nqs_xg1_dn17, locals.var_nqs_xg1_dn18, locals.var_nqs_xg1_dn19, locals.var_nqs_xg1_dn20,)
    }
};
        locals.var_nqs_xg1 = assign82410_e123726;
        locals.var_nqs_xg1_dn5 = assign82410_e123726_d_n5;
        locals.var_nqs_xg1_dn6 = assign82410_e123726_d_n6;
        locals.var_nqs_xg1_dn7 = assign82410_e123726_d_n7;
        locals.var_nqs_xg1_dn8 = assign82410_e123726_d_n8;
        locals.var_nqs_xg1_dn12 = assign82410_e123726_d_n12;
        locals.var_nqs_xg1_dn13 = assign82410_e123726_d_n13;
        locals.var_nqs_xg1_dn14 = assign82410_e123726_d_n14;
        locals.var_nqs_xg1_dn15 = assign82410_e123726_d_n15;
        locals.var_nqs_xg1_dn16 = assign82410_e123726_d_n16;
        locals.var_nqs_xg1_dn17 = assign82410_e123726_d_n17;
        locals.var_nqs_xg1_dn18 = assign82410_e123726_d_n18;
        locals.var_nqs_xg1_dn19 = assign82410_e123726_d_n19;
        locals.var_nqs_xg1_dn20 = assign82410_e123726_d_n20;

        let (assign82420_e123758, assign82420_e123758_d_n5, assign82420_e123758_d_n6, assign82420_e123758_d_n7, assign82420_e123758_d_n8, assign82420_e123758_d_n12, assign82420_e123758_d_n13, assign82420_e123758_d_n14, assign82420_e123758_d_n15, assign82420_e123758_d_n16, assign82420_e123758_d_n17, assign82420_e123758_d_n18, assign82420_e123758_d_n19, assign82420_e123758_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82420_e123750: f64 = (1.25 * locals.var_a_factrp);
        let assign82420_e123752: f64 = (assign82420_e123750 * locals.var_nqs_xg1);
        let assign82420_e123754: f64 = (assign82420_e123752 - 1.0);
        let assign82420_e123756: f64 = (assign82420_e123754 * locals.var_nqs_xg1);
        (assign82420_e123756, (((((1.25 * locals.var_a_factrp_dn5) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn5)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn5)), (((((1.25 * locals.var_a_factrp_dn6) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn6)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn6)), (((((1.25 * locals.var_a_factrp_dn7) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn7)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn7)), (((((1.25 * locals.var_a_factrp_dn8) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn8)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn8)), (((((1.25 * locals.var_a_factrp_dn12) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn12)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn12)), (((((1.25 * locals.var_a_factrp_dn13) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn13)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn13)), (((((1.25 * locals.var_a_factrp_dn14) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn14)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn14)), (((((1.25 * locals.var_a_factrp_dn15) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn15)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn15)), (((((1.25 * locals.var_a_factrp_dn16) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn16)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn16)), (((((1.25 * locals.var_a_factrp_dn17) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn17)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn17)), (((((1.25 * locals.var_a_factrp_dn18) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn18)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn18)), (((((1.25 * locals.var_a_factrp_dn19) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn19)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn19)), (((((1.25 * locals.var_a_factrp_dn20) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn20)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn20)),)
    } else {
        (locals.var_nqs_a_fac, locals.var_nqs_a_fac_dn5, locals.var_nqs_a_fac_dn6, locals.var_nqs_a_fac_dn7, locals.var_nqs_a_fac_dn8, locals.var_nqs_a_fac_dn12, locals.var_nqs_a_fac_dn13, locals.var_nqs_a_fac_dn14, locals.var_nqs_a_fac_dn15, locals.var_nqs_a_fac_dn16, locals.var_nqs_a_fac_dn17, locals.var_nqs_a_fac_dn18, locals.var_nqs_a_fac_dn19, locals.var_nqs_a_fac_dn20,)
    }
};
        locals.var_nqs_a_fac = assign82420_e123758;
        locals.var_nqs_a_fac_dn5 = assign82420_e123758_d_n5;
        locals.var_nqs_a_fac_dn6 = assign82420_e123758_d_n6;
        locals.var_nqs_a_fac_dn7 = assign82420_e123758_d_n7;
        locals.var_nqs_a_fac_dn8 = assign82420_e123758_d_n8;
        locals.var_nqs_a_fac_dn12 = assign82420_e123758_d_n12;
        locals.var_nqs_a_fac_dn13 = assign82420_e123758_d_n13;
        locals.var_nqs_a_fac_dn14 = assign82420_e123758_d_n14;
        locals.var_nqs_a_fac_dn15 = assign82420_e123758_d_n15;
        locals.var_nqs_a_fac_dn16 = assign82420_e123758_d_n16;
        locals.var_nqs_a_fac_dn17 = assign82420_e123758_d_n17;
        locals.var_nqs_a_fac_dn18 = assign82420_e123758_d_n18;
        locals.var_nqs_a_fac_dn19 = assign82420_e123758_d_n19;
        locals.var_nqs_a_fac_dn20 = assign82420_e123758_d_n20;

        let (assign82430_e123790, assign82430_e123790_d_n5, assign82430_e123790_d_n6, assign82430_e123790_d_n7, assign82430_e123790_d_n8, assign82430_e123790_d_n12, assign82430_e123790_d_n13, assign82430_e123790_d_n14, assign82430_e123790_d_n15, assign82430_e123790_d_n16, assign82430_e123790_d_n17, assign82430_e123790_d_n18, assign82430_e123790_d_n19, assign82430_e123790_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82430_e123782: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        let assign82430_e123786: f64 = (locals.var_nqs_a_fac * locals.var_temp__blk1038);
        let assign82430_e123787: f64 = (1.0 + assign82430_e123786);
        let assign82430_e123788: f64 = (assign82430_e123782 * assign82430_e123787);
        (assign82430_e123788, (((((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn5 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn5)))), (((((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn6 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn6)))), (((((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn7 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn7)))), (((((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn8 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn8)))), (((((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn12 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn12)))), (((((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn13 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn13)))), (((((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn14 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn14)))), (((((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn15 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn15)))), (((((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn16 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn16)))), (((((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn17 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn17)))), (((((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn18 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn18)))), (((((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn19 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn19)))), (((((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn20 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn20)))),)
    } else {
        (locals.var_nqs_xbar, locals.var_nqs_xbar_dn5, locals.var_nqs_xbar_dn6, locals.var_nqs_xbar_dn7, locals.var_nqs_xbar_dn8, locals.var_nqs_xbar_dn12, locals.var_nqs_xbar_dn13, locals.var_nqs_xbar_dn14, locals.var_nqs_xbar_dn15, locals.var_nqs_xbar_dn16, locals.var_nqs_xbar_dn17, locals.var_nqs_xbar_dn18, locals.var_nqs_xbar_dn19, locals.var_nqs_xbar_dn20,)
    }
};
        locals.var_nqs_xbar = assign82430_e123790;
        locals.var_nqs_xbar_dn5 = assign82430_e123790_d_n5;
        locals.var_nqs_xbar_dn6 = assign82430_e123790_d_n6;
        locals.var_nqs_xbar_dn7 = assign82430_e123790_d_n7;
        locals.var_nqs_xbar_dn8 = assign82430_e123790_d_n8;
        locals.var_nqs_xbar_dn12 = assign82430_e123790_d_n12;
        locals.var_nqs_xbar_dn13 = assign82430_e123790_d_n13;
        locals.var_nqs_xbar_dn14 = assign82430_e123790_d_n14;
        locals.var_nqs_xbar_dn15 = assign82430_e123790_d_n15;
        locals.var_nqs_xbar_dn16 = assign82430_e123790_d_n16;
        locals.var_nqs_xbar_dn17 = assign82430_e123790_d_n17;
        locals.var_nqs_xbar_dn18 = assign82430_e123790_d_n18;
        locals.var_nqs_xbar_dn19 = assign82430_e123790_d_n19;
        locals.var_nqs_xbar_dn20 = assign82430_e123790_d_n20;

        let assign82440_e123792: f64 = (-locals.var_nqs_xbar);
        let assign82440_e123793: f64 = (assign82440_e123792).abs();
        let assign82440_e123795: f64 = if assign82440_e123793 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign82440_e123795;

        let (assign82450_e123823, assign82450_e123823_d_n5, assign82450_e123823_d_n6, assign82450_e123823_d_n7, assign82450_e123823_d_n8, assign82450_e123823_d_n12, assign82450_e123823_d_n13, assign82450_e123823_d_n14, assign82450_e123823_d_n15, assign82450_e123823_d_n16, assign82450_e123823_d_n17, assign82450_e123823_d_n18, assign82450_e123823_d_n19, assign82450_e123823_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign82450_e123820: f64 = (-locals.var_nqs_xbar);
        let assign82450_e123821: f64 = (assign82450_e123820).exp();
        (assign82450_e123821, (assign82450_e123821 * (-locals.var_nqs_xbar_dn5)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn6)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn7)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn8)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn12)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn13)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn14)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn15)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn16)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn17)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn18)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn19)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn20)),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82450_e123823;
        locals.var_nqs_temp_dn5 = assign82450_e123823_d_n5;
        locals.var_nqs_temp_dn6 = assign82450_e123823_d_n6;
        locals.var_nqs_temp_dn7 = assign82450_e123823_d_n7;
        locals.var_nqs_temp_dn8 = assign82450_e123823_d_n8;
        locals.var_nqs_temp_dn12 = assign82450_e123823_d_n12;
        locals.var_nqs_temp_dn13 = assign82450_e123823_d_n13;
        locals.var_nqs_temp_dn14 = assign82450_e123823_d_n14;
        locals.var_nqs_temp_dn15 = assign82450_e123823_d_n15;
        locals.var_nqs_temp_dn16 = assign82450_e123823_d_n16;
        locals.var_nqs_temp_dn17 = assign82450_e123823_d_n17;
        locals.var_nqs_temp_dn18 = assign82450_e123823_d_n18;
        locals.var_nqs_temp_dn19 = assign82450_e123823_d_n19;
        locals.var_nqs_temp_dn20 = assign82450_e123823_d_n20;

        let assign82460_e123825: f64 = (-locals.var_nqs_xbar);
        let assign82460_e123827: f64 = if assign82460_e123825 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign82460_e123827;

        let (assign82470_e123884, assign82470_e123884_d_n5, assign82470_e123884_d_n6, assign82470_e123884_d_n7, assign82470_e123884_d_n8, assign82470_e123884_d_n12, assign82470_e123884_d_n13, assign82470_e123884_d_n14, assign82470_e123884_d_n15, assign82470_e123884_d_n16, assign82470_e123884_d_n17, assign82470_e123884_d_n18, assign82470_e123884_d_n19, assign82470_e123884_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 == 0.0)) && (locals.var_guard2241 != 0.0)) {
        let assign82470_e123857: f64 = (-230.25850929940458);
        let assign82470_e123859: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123860: f64 = (assign82470_e123857 - assign82470_e123859);
        let assign82470_e123864: f64 = (-230.25850929940458);
        let assign82470_e123866: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123867: f64 = (assign82470_e123864 - assign82470_e123866);
        let assign82470_e123870: f64 = (-230.25850929940458);
        let assign82470_e123872: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123873: f64 = (assign82470_e123870 - assign82470_e123872);
        let assign82470_e123875: f64 = (assign82470_e123873 * 0.3333333333333333);
        let assign82470_e123876: f64 = (1.0 + assign82470_e123875);
        let assign82470_e123877: f64 = (assign82470_e123867 * assign82470_e123876);
        let assign82470_e123878: f64 = (0.5 * assign82470_e123877);
        let assign82470_e123879: f64 = (1.0 + assign82470_e123878);
        let assign82470_e123880: f64 = (assign82470_e123860 * assign82470_e123879);
        let assign82470_e123881: f64 = (1.0 + assign82470_e123880);
        let assign82470_e123882: f64 = (1e-100 / assign82470_e123881);
        (assign82470_e123882, (-((1e-100 * (((-(-locals.var_nqs_xbar_dn5)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn5)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn5)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn6)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn6)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn6)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn7)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn7)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn7)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn8)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn8)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn8)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn12)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn12)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn12)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn13)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn13)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn13)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn14)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn14)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn14)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn15)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn15)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn15)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn16)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn16)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn16)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn17)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn17)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn17)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn18)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn18)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn18)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn19)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn19)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn19)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn20)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn20)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn20)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82470_e123884;
        locals.var_nqs_temp_dn5 = assign82470_e123884_d_n5;
        locals.var_nqs_temp_dn6 = assign82470_e123884_d_n6;
        locals.var_nqs_temp_dn7 = assign82470_e123884_d_n7;
        locals.var_nqs_temp_dn8 = assign82470_e123884_d_n8;
        locals.var_nqs_temp_dn12 = assign82470_e123884_d_n12;
        locals.var_nqs_temp_dn13 = assign82470_e123884_d_n13;
        locals.var_nqs_temp_dn14 = assign82470_e123884_d_n14;
        locals.var_nqs_temp_dn15 = assign82470_e123884_d_n15;
        locals.var_nqs_temp_dn16 = assign82470_e123884_d_n16;
        locals.var_nqs_temp_dn17 = assign82470_e123884_d_n17;
        locals.var_nqs_temp_dn18 = assign82470_e123884_d_n18;
        locals.var_nqs_temp_dn19 = assign82470_e123884_d_n19;
        locals.var_nqs_temp_dn20 = assign82470_e123884_d_n20;

        let (assign82480_e123939, assign82480_e123939_d_n5, assign82480_e123939_d_n6, assign82480_e123939_d_n7, assign82480_e123939_d_n8, assign82480_e123939_d_n12, assign82480_e123939_d_n13, assign82480_e123939_d_n14, assign82480_e123939_d_n15, assign82480_e123939_d_n16, assign82480_e123939_d_n17, assign82480_e123939_d_n18, assign82480_e123939_d_n19, assign82480_e123939_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 == 0.0)) && (locals.var_guard2241 == 0.0)) {
        let assign82480_e123915: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123917: f64 = (assign82480_e123915 - 230.25850929940458);
        let assign82480_e123921: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123923: f64 = (assign82480_e123921 - 230.25850929940458);
        let assign82480_e123926: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123928: f64 = (assign82480_e123926 - 230.25850929940458);
        let assign82480_e123930: f64 = (assign82480_e123928 * 0.3333333333333333);
        let assign82480_e123931: f64 = (1.0 + assign82480_e123930);
        let assign82480_e123932: f64 = (assign82480_e123923 * assign82480_e123931);
        let assign82480_e123933: f64 = (0.5 * assign82480_e123932);
        let assign82480_e123934: f64 = (1.0 + assign82480_e123933);
        let assign82480_e123935: f64 = (assign82480_e123917 * assign82480_e123934);
        let assign82480_e123936: f64 = (1.0 + assign82480_e123935);
        let assign82480_e123937: f64 = (1e100 * assign82480_e123936);
        (assign82480_e123937, (1e100 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82480_e123939;
        locals.var_nqs_temp_dn5 = assign82480_e123939_d_n5;
        locals.var_nqs_temp_dn6 = assign82480_e123939_d_n6;
        locals.var_nqs_temp_dn7 = assign82480_e123939_d_n7;
        locals.var_nqs_temp_dn8 = assign82480_e123939_d_n8;
        locals.var_nqs_temp_dn12 = assign82480_e123939_d_n12;
        locals.var_nqs_temp_dn13 = assign82480_e123939_d_n13;
        locals.var_nqs_temp_dn14 = assign82480_e123939_d_n14;
        locals.var_nqs_temp_dn15 = assign82480_e123939_d_n15;
        locals.var_nqs_temp_dn16 = assign82480_e123939_d_n16;
        locals.var_nqs_temp_dn17 = assign82480_e123939_d_n17;
        locals.var_nqs_temp_dn18 = assign82480_e123939_d_n18;
        locals.var_nqs_temp_dn19 = assign82480_e123939_d_n19;
        locals.var_nqs_temp_dn20 = assign82480_e123939_d_n20;

        let (assign82490_e123965, assign82490_e123965_d_n5, assign82490_e123965_d_n6, assign82490_e123965_d_n7, assign82490_e123965_d_n8, assign82490_e123965_d_n12, assign82490_e123965_d_n13, assign82490_e123965_d_n14, assign82490_e123965_d_n15, assign82490_e123965_d_n16, assign82490_e123965_d_n17, assign82490_e123965_d_n18, assign82490_e123965_d_n19, assign82490_e123965_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82490_e123963: f64 = (1.0 - locals.var_nqs_temp);
        (assign82490_e123963, (-locals.var_nqs_temp_dn5), (-locals.var_nqs_temp_dn6), (-locals.var_nqs_temp_dn7), (-locals.var_nqs_temp_dn8), (-locals.var_nqs_temp_dn12), (-locals.var_nqs_temp_dn13), (-locals.var_nqs_temp_dn14), (-locals.var_nqs_temp_dn15), (-locals.var_nqs_temp_dn16), (-locals.var_nqs_temp_dn17), (-locals.var_nqs_temp_dn18), (-locals.var_nqs_temp_dn19), (-locals.var_nqs_temp_dn20),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82490_e123965;
        locals.var_nqs_w_dn5 = assign82490_e123965_d_n5;
        locals.var_nqs_w_dn6 = assign82490_e123965_d_n6;
        locals.var_nqs_w_dn7 = assign82490_e123965_d_n7;
        locals.var_nqs_w_dn8 = assign82490_e123965_d_n8;
        locals.var_nqs_w_dn12 = assign82490_e123965_d_n12;
        locals.var_nqs_w_dn13 = assign82490_e123965_d_n13;
        locals.var_nqs_w_dn14 = assign82490_e123965_d_n14;
        locals.var_nqs_w_dn15 = assign82490_e123965_d_n15;
        locals.var_nqs_w_dn16 = assign82490_e123965_d_n16;
        locals.var_nqs_w_dn17 = assign82490_e123965_d_n17;
        locals.var_nqs_w_dn18 = assign82490_e123965_d_n18;
        locals.var_nqs_w_dn19 = assign82490_e123965_d_n19;
        locals.var_nqs_w_dn20 = assign82490_e123965_d_n20;

        let (assign82500_e124004, assign82500_e124004_d_n5, assign82500_e124004_d_n6, assign82500_e124004_d_n7, assign82500_e124004_d_n8, assign82500_e124004_d_n12, assign82500_e124004_d_n13, assign82500_e124004_d_n14, assign82500_e124004_d_n15, assign82500_e124004_d_n16, assign82500_e124004_d_n17, assign82500_e124004_d_n18, assign82500_e124004_d_n19, assign82500_e124004_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82500_e123990: f64 = (locals.var_gp2 * 0.5);
        let assign82500_e123991: f64 = (locals.var_temp__blk1038 + assign82500_e123990);
        let assign82500_e123996: f64 = (locals.var_gp2 * 0.25);
        let assign82500_e123997: f64 = (locals.var_temp__blk1038 + assign82500_e123996);
        let assign82500_e123999: f64 = (assign82500_e123997 - locals.var_nqs_w);
        let assign82500_e124000: f64 = (assign82500_e123999).sqrt();
        let assign82500_e124001: f64 = (locals.var_gp * assign82500_e124000);
        let assign82500_e124002: f64 = (assign82500_e123991 - assign82500_e124001);
        (assign82500_e124002, ((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.5)) - ((locals.var_gp_dn5 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.25)) - locals.var_nqs_w_dn5) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.5)) - ((locals.var_gp_dn6 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.25)) - locals.var_nqs_w_dn6) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.5)) - ((locals.var_gp_dn7 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.25)) - locals.var_nqs_w_dn7) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.5)) - ((locals.var_gp_dn8 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.25)) - locals.var_nqs_w_dn8) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.5)) - ((locals.var_gp_dn12 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.25)) - locals.var_nqs_w_dn12) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.5)) - ((locals.var_gp_dn13 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.25)) - locals.var_nqs_w_dn13) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.5)) - ((locals.var_gp_dn14 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.25)) - locals.var_nqs_w_dn14) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.5)) - ((locals.var_gp_dn15 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.25)) - locals.var_nqs_w_dn15) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.5)) - ((locals.var_gp_dn16 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.25)) - locals.var_nqs_w_dn16) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.5)) - ((locals.var_gp_dn17 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.25)) - locals.var_nqs_w_dn17) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.5)) - ((locals.var_gp_dn18 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.25)) - locals.var_nqs_w_dn18) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.5)) - ((locals.var_gp_dn19 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.25)) - locals.var_nqs_w_dn19) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.5)) - ((locals.var_gp_dn20 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.25)) - locals.var_nqs_w_dn20) / (2.0 * assign82500_e124000))))),)
    } else {
        (locals.var_nqs_x0, locals.var_nqs_x0_dn5, locals.var_nqs_x0_dn6, locals.var_nqs_x0_dn7, locals.var_nqs_x0_dn8, locals.var_nqs_x0_dn12, locals.var_nqs_x0_dn13, locals.var_nqs_x0_dn14, locals.var_nqs_x0_dn15, locals.var_nqs_x0_dn16, locals.var_nqs_x0_dn17, locals.var_nqs_x0_dn18, locals.var_nqs_x0_dn19, locals.var_nqs_x0_dn20,)
    }
};
        locals.var_nqs_x0 = assign82500_e124004;
        locals.var_nqs_x0_dn5 = assign82500_e124004_d_n5;
        locals.var_nqs_x0_dn6 = assign82500_e124004_d_n6;
        locals.var_nqs_x0_dn7 = assign82500_e124004_d_n7;
        locals.var_nqs_x0_dn8 = assign82500_e124004_d_n8;
        locals.var_nqs_x0_dn12 = assign82500_e124004_d_n12;
        locals.var_nqs_x0_dn13 = assign82500_e124004_d_n13;
        locals.var_nqs_x0_dn14 = assign82500_e124004_d_n14;
        locals.var_nqs_x0_dn15 = assign82500_e124004_d_n15;
        locals.var_nqs_x0_dn16 = assign82500_e124004_d_n16;
        locals.var_nqs_x0_dn17 = assign82500_e124004_d_n17;
        locals.var_nqs_x0_dn18 = assign82500_e124004_d_n18;
        locals.var_nqs_x0_dn19 = assign82500_e124004_d_n19;
        locals.var_nqs_x0_dn20 = assign82500_e124004_d_n20;

        let assign82510_e124006: f64 = (-locals.var_nqs_x0);
        let assign82510_e124007: f64 = (assign82510_e124006).abs();
        let assign82510_e124009: f64 = if assign82510_e124007 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign82510_e124009;

    }

    pub(super) fn stamp_transient_block_164(
        locals: &mut StampLocals,
    ) {
        let (assign82520_e124037, assign82520_e124037_d_n5, assign82520_e124037_d_n6, assign82520_e124037_d_n7, assign82520_e124037_d_n8, assign82520_e124037_d_n12, assign82520_e124037_d_n13, assign82520_e124037_d_n14, assign82520_e124037_d_n15, assign82520_e124037_d_n16, assign82520_e124037_d_n17, assign82520_e124037_d_n18, assign82520_e124037_d_n19, assign82520_e124037_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign82520_e124034: f64 = (-locals.var_nqs_x0);
        let assign82520_e124035: f64 = (assign82520_e124034).exp();
        (assign82520_e124035, (assign82520_e124035 * (-locals.var_nqs_x0_dn5)), (assign82520_e124035 * (-locals.var_nqs_x0_dn6)), (assign82520_e124035 * (-locals.var_nqs_x0_dn7)), (assign82520_e124035 * (-locals.var_nqs_x0_dn8)), (assign82520_e124035 * (-locals.var_nqs_x0_dn12)), (assign82520_e124035 * (-locals.var_nqs_x0_dn13)), (assign82520_e124035 * (-locals.var_nqs_x0_dn14)), (assign82520_e124035 * (-locals.var_nqs_x0_dn15)), (assign82520_e124035 * (-locals.var_nqs_x0_dn16)), (assign82520_e124035 * (-locals.var_nqs_x0_dn17)), (assign82520_e124035 * (-locals.var_nqs_x0_dn18)), (assign82520_e124035 * (-locals.var_nqs_x0_dn19)), (assign82520_e124035 * (-locals.var_nqs_x0_dn20)),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82520_e124037;
        locals.var_nqs_d0_dn5 = assign82520_e124037_d_n5;
        locals.var_nqs_d0_dn6 = assign82520_e124037_d_n6;
        locals.var_nqs_d0_dn7 = assign82520_e124037_d_n7;
        locals.var_nqs_d0_dn8 = assign82520_e124037_d_n8;
        locals.var_nqs_d0_dn12 = assign82520_e124037_d_n12;
        locals.var_nqs_d0_dn13 = assign82520_e124037_d_n13;
        locals.var_nqs_d0_dn14 = assign82520_e124037_d_n14;
        locals.var_nqs_d0_dn15 = assign82520_e124037_d_n15;
        locals.var_nqs_d0_dn16 = assign82520_e124037_d_n16;
        locals.var_nqs_d0_dn17 = assign82520_e124037_d_n17;
        locals.var_nqs_d0_dn18 = assign82520_e124037_d_n18;
        locals.var_nqs_d0_dn19 = assign82520_e124037_d_n19;
        locals.var_nqs_d0_dn20 = assign82520_e124037_d_n20;

        let assign82530_e124039: f64 = (-locals.var_nqs_x0);
        let assign82530_e124041: f64 = if assign82530_e124039 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign82530_e124041;

        let (assign82540_e124098, assign82540_e124098_d_n5, assign82540_e124098_d_n6, assign82540_e124098_d_n7, assign82540_e124098_d_n8, assign82540_e124098_d_n12, assign82540_e124098_d_n13, assign82540_e124098_d_n14, assign82540_e124098_d_n15, assign82540_e124098_d_n16, assign82540_e124098_d_n17, assign82540_e124098_d_n18, assign82540_e124098_d_n19, assign82540_e124098_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 != 0.0)) {
        let assign82540_e124071: f64 = (-230.25850929940458);
        let assign82540_e124073: f64 = (-locals.var_nqs_x0);
        let assign82540_e124074: f64 = (assign82540_e124071 - assign82540_e124073);
        let assign82540_e124078: f64 = (-230.25850929940458);
        let assign82540_e124080: f64 = (-locals.var_nqs_x0);
        let assign82540_e124081: f64 = (assign82540_e124078 - assign82540_e124080);
        let assign82540_e124084: f64 = (-230.25850929940458);
        let assign82540_e124086: f64 = (-locals.var_nqs_x0);
        let assign82540_e124087: f64 = (assign82540_e124084 - assign82540_e124086);
        let assign82540_e124089: f64 = (assign82540_e124087 * 0.3333333333333333);
        let assign82540_e124090: f64 = (1.0 + assign82540_e124089);
        let assign82540_e124091: f64 = (assign82540_e124081 * assign82540_e124090);
        let assign82540_e124092: f64 = (0.5 * assign82540_e124091);
        let assign82540_e124093: f64 = (1.0 + assign82540_e124092);
        let assign82540_e124094: f64 = (assign82540_e124074 * assign82540_e124093);
        let assign82540_e124095: f64 = (1.0 + assign82540_e124094);
        let assign82540_e124096: f64 = (1e-100 / assign82540_e124095);
        (assign82540_e124096, (-((1e-100 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82540_e124098;
        locals.var_nqs_d0_dn5 = assign82540_e124098_d_n5;
        locals.var_nqs_d0_dn6 = assign82540_e124098_d_n6;
        locals.var_nqs_d0_dn7 = assign82540_e124098_d_n7;
        locals.var_nqs_d0_dn8 = assign82540_e124098_d_n8;
        locals.var_nqs_d0_dn12 = assign82540_e124098_d_n12;
        locals.var_nqs_d0_dn13 = assign82540_e124098_d_n13;
        locals.var_nqs_d0_dn14 = assign82540_e124098_d_n14;
        locals.var_nqs_d0_dn15 = assign82540_e124098_d_n15;
        locals.var_nqs_d0_dn16 = assign82540_e124098_d_n16;
        locals.var_nqs_d0_dn17 = assign82540_e124098_d_n17;
        locals.var_nqs_d0_dn18 = assign82540_e124098_d_n18;
        locals.var_nqs_d0_dn19 = assign82540_e124098_d_n19;
        locals.var_nqs_d0_dn20 = assign82540_e124098_d_n20;

        let (assign82550_e124153, assign82550_e124153_d_n5, assign82550_e124153_d_n6, assign82550_e124153_d_n7, assign82550_e124153_d_n8, assign82550_e124153_d_n12, assign82550_e124153_d_n13, assign82550_e124153_d_n14, assign82550_e124153_d_n15, assign82550_e124153_d_n16, assign82550_e124153_d_n17, assign82550_e124153_d_n18, assign82550_e124153_d_n19, assign82550_e124153_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 == 0.0)) {
        let assign82550_e124129: f64 = (-locals.var_nqs_x0);
        let assign82550_e124131: f64 = (assign82550_e124129 - 230.25850929940458);
        let assign82550_e124135: f64 = (-locals.var_nqs_x0);
        let assign82550_e124137: f64 = (assign82550_e124135 - 230.25850929940458);
        let assign82550_e124140: f64 = (-locals.var_nqs_x0);
        let assign82550_e124142: f64 = (assign82550_e124140 - 230.25850929940458);
        let assign82550_e124144: f64 = (assign82550_e124142 * 0.3333333333333333);
        let assign82550_e124145: f64 = (1.0 + assign82550_e124144);
        let assign82550_e124146: f64 = (assign82550_e124137 * assign82550_e124145);
        let assign82550_e124147: f64 = (0.5 * assign82550_e124146);
        let assign82550_e124148: f64 = (1.0 + assign82550_e124147);
        let assign82550_e124149: f64 = (assign82550_e124131 * assign82550_e124148);
        let assign82550_e124150: f64 = (1.0 + assign82550_e124149);
        let assign82550_e124151: f64 = (1e100 * assign82550_e124150);
        (assign82550_e124151, (1e100 * (((-locals.var_nqs_x0_dn5) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn5) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn6) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn6) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn7) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn7) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn8) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn8) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn12) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn12) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn13) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn13) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn14) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn14) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn15) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn15) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn16) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn16) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn17) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn17) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn18) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn18) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn19) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn19) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn20) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn20) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82550_e124153;
        locals.var_nqs_d0_dn5 = assign82550_e124153_d_n5;
        locals.var_nqs_d0_dn6 = assign82550_e124153_d_n6;
        locals.var_nqs_d0_dn7 = assign82550_e124153_d_n7;
        locals.var_nqs_d0_dn8 = assign82550_e124153_d_n8;
        locals.var_nqs_d0_dn12 = assign82550_e124153_d_n12;
        locals.var_nqs_d0_dn13 = assign82550_e124153_d_n13;
        locals.var_nqs_d0_dn14 = assign82550_e124153_d_n14;
        locals.var_nqs_d0_dn15 = assign82550_e124153_d_n15;
        locals.var_nqs_d0_dn16 = assign82550_e124153_d_n16;
        locals.var_nqs_d0_dn17 = assign82550_e124153_d_n17;
        locals.var_nqs_d0_dn18 = assign82550_e124153_d_n18;
        locals.var_nqs_d0_dn19 = assign82550_e124153_d_n19;
        locals.var_nqs_d0_dn20 = assign82550_e124153_d_n20;

        let (assign82560_e124183, assign82560_e124183_d_n5, assign82560_e124183_d_n6, assign82560_e124183_d_n7, assign82560_e124183_d_n8, assign82560_e124183_d_n12, assign82560_e124183_d_n13, assign82560_e124183_d_n14, assign82560_e124183_d_n15, assign82560_e124183_d_n16, assign82560_e124183_d_n17, assign82560_e124183_d_n18, assign82560_e124183_d_n19, assign82560_e124183_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82560_e124178: f64 = (locals.var_gp2 * 0.5);
        let assign82560_e124180: f64 = (assign82560_e124178 * locals.var_nqs_d0);
        let assign82560_e124181: f64 = (1.0 - assign82560_e124180);
        (assign82560_e124181, (-(((locals.var_gp2_dn5 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn5))), (-(((locals.var_gp2_dn6 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn6))), (-(((locals.var_gp2_dn7 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn7))), (-(((locals.var_gp2_dn8 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn8))), (-(((locals.var_gp2_dn12 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn12))), (-(((locals.var_gp2_dn13 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn13))), (-(((locals.var_gp2_dn14 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn14))), (-(((locals.var_gp2_dn15 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn15))), (-(((locals.var_gp2_dn16 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn16))), (-(((locals.var_gp2_dn17 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn17))), (-(((locals.var_gp2_dn18 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn18))), (-(((locals.var_gp2_dn19 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn19))), (-(((locals.var_gp2_dn20 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82560_e124183;
        locals.var_nqs_xi_dn5 = assign82560_e124183_d_n5;
        locals.var_nqs_xi_dn6 = assign82560_e124183_d_n6;
        locals.var_nqs_xi_dn7 = assign82560_e124183_d_n7;
        locals.var_nqs_xi_dn8 = assign82560_e124183_d_n8;
        locals.var_nqs_xi_dn12 = assign82560_e124183_d_n12;
        locals.var_nqs_xi_dn13 = assign82560_e124183_d_n13;
        locals.var_nqs_xi_dn14 = assign82560_e124183_d_n14;
        locals.var_nqs_xi_dn15 = assign82560_e124183_d_n15;
        locals.var_nqs_xi_dn16 = assign82560_e124183_d_n16;
        locals.var_nqs_xi_dn17 = assign82560_e124183_d_n17;
        locals.var_nqs_xi_dn18 = assign82560_e124183_d_n18;
        locals.var_nqs_xi_dn19 = assign82560_e124183_d_n19;
        locals.var_nqs_xi_dn20 = assign82560_e124183_d_n20;

        let (assign82570_e124217, assign82570_e124217_d_n5, assign82570_e124217_d_n6, assign82570_e124217_d_n7, assign82570_e124217_d_n8, assign82570_e124217_d_n12, assign82570_e124217_d_n13, assign82570_e124217_d_n14, assign82570_e124217_d_n15, assign82570_e124217_d_n16, assign82570_e124217_d_n17, assign82570_e124217_d_n18, assign82570_e124217_d_n19, assign82570_e124217_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82570_e124208: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82570_e124209: f64 = (2.0 * assign82570_e124208);
        let assign82570_e124213: f64 = (1.0 - locals.var_nqs_d0);
        let assign82570_e124214: f64 = (locals.var_gp2 * assign82570_e124213);
        let assign82570_e124215: f64 = (assign82570_e124209 + assign82570_e124214);
        (assign82570_e124215, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82570_e124217;
        locals.var_nqs_p_dn5 = assign82570_e124217_d_n5;
        locals.var_nqs_p_dn6 = assign82570_e124217_d_n6;
        locals.var_nqs_p_dn7 = assign82570_e124217_d_n7;
        locals.var_nqs_p_dn8 = assign82570_e124217_d_n8;
        locals.var_nqs_p_dn12 = assign82570_e124217_d_n12;
        locals.var_nqs_p_dn13 = assign82570_e124217_d_n13;
        locals.var_nqs_p_dn14 = assign82570_e124217_d_n14;
        locals.var_nqs_p_dn15 = assign82570_e124217_d_n15;
        locals.var_nqs_p_dn16 = assign82570_e124217_d_n16;
        locals.var_nqs_p_dn17 = assign82570_e124217_d_n17;
        locals.var_nqs_p_dn18 = assign82570_e124217_d_n18;
        locals.var_nqs_p_dn19 = assign82570_e124217_d_n19;
        locals.var_nqs_p_dn20 = assign82570_e124217_d_n20;

        let (assign82580_e124255, assign82580_e124255_d_n5, assign82580_e124255_d_n6, assign82580_e124255_d_n7, assign82580_e124255_d_n8, assign82580_e124255_d_n12, assign82580_e124255_d_n13, assign82580_e124255_d_n14, assign82580_e124255_d_n15, assign82580_e124255_d_n16, assign82580_e124255_d_n17, assign82580_e124255_d_n18, assign82580_e124255_d_n19, assign82580_e124255_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82580_e124241: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124244: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124245: f64 = (assign82580_e124241 * assign82580_e124244);
        let assign82580_e124249: f64 = (locals.var_nqs_x0 - 1.0);
        let assign82580_e124251: f64 = (assign82580_e124249 + locals.var_nqs_d0);
        let assign82580_e124252: f64 = (locals.var_gp2 * assign82580_e124251);
        let assign82580_e124253: f64 = (assign82580_e124245 - assign82580_e124252);
        (assign82580_e124253, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82580_e124255;
        locals.var_nqs_q_dn5 = assign82580_e124255_d_n5;
        locals.var_nqs_q_dn6 = assign82580_e124255_d_n6;
        locals.var_nqs_q_dn7 = assign82580_e124255_d_n7;
        locals.var_nqs_q_dn8 = assign82580_e124255_d_n8;
        locals.var_nqs_q_dn12 = assign82580_e124255_d_n12;
        locals.var_nqs_q_dn13 = assign82580_e124255_d_n13;
        locals.var_nqs_q_dn14 = assign82580_e124255_d_n14;
        locals.var_nqs_q_dn15 = assign82580_e124255_d_n15;
        locals.var_nqs_q_dn16 = assign82580_e124255_d_n16;
        locals.var_nqs_q_dn17 = assign82580_e124255_d_n17;
        locals.var_nqs_q_dn18 = assign82580_e124255_d_n18;
        locals.var_nqs_q_dn19 = assign82580_e124255_d_n19;
        locals.var_nqs_q_dn20 = assign82580_e124255_d_n20;

        let (assign82590_e124287, assign82590_e124287_d_n5, assign82590_e124287_d_n6, assign82590_e124287_d_n7, assign82590_e124287_d_n8, assign82590_e124287_d_n12, assign82590_e124287_d_n13, assign82590_e124287_d_n14, assign82590_e124287_d_n15, assign82590_e124287_d_n16, assign82590_e124287_d_n17, assign82590_e124287_d_n18, assign82590_e124287_d_n19, assign82590_e124287_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82590_e124279: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82590_e124282: f64 = (4.0 * locals.var_nqs_xi);
        let assign82590_e124284: f64 = (assign82590_e124282 * locals.var_nqs_q);
        let assign82590_e124285: f64 = (assign82590_e124279 - assign82590_e124284);
        (assign82590_e124285, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82590_e124287;
        locals.var_nqs_temp_dn5 = assign82590_e124287_d_n5;
        locals.var_nqs_temp_dn6 = assign82590_e124287_d_n6;
        locals.var_nqs_temp_dn7 = assign82590_e124287_d_n7;
        locals.var_nqs_temp_dn8 = assign82590_e124287_d_n8;
        locals.var_nqs_temp_dn12 = assign82590_e124287_d_n12;
        locals.var_nqs_temp_dn13 = assign82590_e124287_d_n13;
        locals.var_nqs_temp_dn14 = assign82590_e124287_d_n14;
        locals.var_nqs_temp_dn15 = assign82590_e124287_d_n15;
        locals.var_nqs_temp_dn16 = assign82590_e124287_d_n16;
        locals.var_nqs_temp_dn17 = assign82590_e124287_d_n17;
        locals.var_nqs_temp_dn18 = assign82590_e124287_d_n18;
        locals.var_nqs_temp_dn19 = assign82590_e124287_d_n19;
        locals.var_nqs_temp_dn20 = assign82590_e124287_d_n20;

        let (assign82600_e124318, assign82600_e124318_d_n5, assign82600_e124318_d_n6, assign82600_e124318_d_n7, assign82600_e124318_d_n8, assign82600_e124318_d_n12, assign82600_e124318_d_n13, assign82600_e124318_d_n14, assign82600_e124318_d_n15, assign82600_e124318_d_n16, assign82600_e124318_d_n17, assign82600_e124318_d_n18, assign82600_e124318_d_n19, assign82600_e124318_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82600_e124311: f64 = (2.0 * locals.var_nqs_q);
        let assign82600_e124314: f64 = (locals.var_nqs_temp).sqrt();
        let assign82600_e124315: f64 = (locals.var_nqs_p + assign82600_e124314);
        let assign82600_e124316: f64 = (assign82600_e124311 / assign82600_e124315);
        (assign82600_e124316, ((((2.0 * locals.var_nqs_q_dn5) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn6) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn7) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn8) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn12) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn13) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn14) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn15) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn16) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn17) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn18) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn19) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn20) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign82600_e124318;
        locals.var_nqs_u_dn5 = assign82600_e124318_d_n5;
        locals.var_nqs_u_dn6 = assign82600_e124318_d_n6;
        locals.var_nqs_u_dn7 = assign82600_e124318_d_n7;
        locals.var_nqs_u_dn8 = assign82600_e124318_d_n8;
        locals.var_nqs_u_dn12 = assign82600_e124318_d_n12;
        locals.var_nqs_u_dn13 = assign82600_e124318_d_n13;
        locals.var_nqs_u_dn14 = assign82600_e124318_d_n14;
        locals.var_nqs_u_dn15 = assign82600_e124318_d_n15;
        locals.var_nqs_u_dn16 = assign82600_e124318_d_n16;
        locals.var_nqs_u_dn17 = assign82600_e124318_d_n17;
        locals.var_nqs_u_dn18 = assign82600_e124318_d_n18;
        locals.var_nqs_u_dn19 = assign82600_e124318_d_n19;
        locals.var_nqs_u_dn20 = assign82600_e124318_d_n20;

        let (assign82610_e124344, assign82610_e124344_d_n5, assign82610_e124344_d_n6, assign82610_e124344_d_n7, assign82610_e124344_d_n8, assign82610_e124344_d_n12, assign82610_e124344_d_n13, assign82610_e124344_d_n14, assign82610_e124344_d_n15, assign82610_e124344_d_n16, assign82610_e124344_d_n17, assign82610_e124344_d_n18, assign82610_e124344_d_n19, assign82610_e124344_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82610_e124342: f64 = (locals.var_nqs_x0 + locals.var_nqs_u);
        (assign82610_e124342, (locals.var_nqs_x0_dn5 + locals.var_nqs_u_dn5), (locals.var_nqs_x0_dn6 + locals.var_nqs_u_dn6), (locals.var_nqs_x0_dn7 + locals.var_nqs_u_dn7), (locals.var_nqs_x0_dn8 + locals.var_nqs_u_dn8), (locals.var_nqs_x0_dn12 + locals.var_nqs_u_dn12), (locals.var_nqs_x0_dn13 + locals.var_nqs_u_dn13), (locals.var_nqs_x0_dn14 + locals.var_nqs_u_dn14), (locals.var_nqs_x0_dn15 + locals.var_nqs_u_dn15), (locals.var_nqs_x0_dn16 + locals.var_nqs_u_dn16), (locals.var_nqs_x0_dn17 + locals.var_nqs_u_dn17), (locals.var_nqs_x0_dn18 + locals.var_nqs_u_dn18), (locals.var_nqs_x0_dn19 + locals.var_nqs_u_dn19), (locals.var_nqs_x0_dn20 + locals.var_nqs_u_dn20),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82610_e124344;
        locals.var_temp9_dn5 = assign82610_e124344_d_n5;
        locals.var_temp9_dn6 = assign82610_e124344_d_n6;
        locals.var_temp9_dn7 = assign82610_e124344_d_n7;
        locals.var_temp9_dn8 = assign82610_e124344_d_n8;
        locals.var_temp9_dn12 = assign82610_e124344_d_n12;
        locals.var_temp9_dn13 = assign82610_e124344_d_n13;
        locals.var_temp9_dn14 = assign82610_e124344_d_n14;
        locals.var_temp9_dn15 = assign82610_e124344_d_n15;
        locals.var_temp9_dn16 = assign82610_e124344_d_n16;
        locals.var_temp9_dn17 = assign82610_e124344_d_n17;
        locals.var_temp9_dn18 = assign82610_e124344_d_n18;
        locals.var_temp9_dn19 = assign82610_e124344_d_n19;
        locals.var_temp9_dn20 = assign82610_e124344_d_n20;

        let (assign82620_e124390, assign82620_e124390_d_n5, assign82620_e124390_d_n6, assign82620_e124390_d_n7, assign82620_e124390_d_n8, assign82620_e124390_d_n12, assign82620_e124390_d_n13, assign82620_e124390_d_n14, assign82620_e124390_d_n15, assign82620_e124390_d_n16, assign82620_e124390_d_n17, assign82620_e124390_d_n18, assign82620_e124390_d_n19, assign82620_e124390_d_n20,) = {
    if ((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        let assign82620_e124365: f64 = (locals.var_temp1 + locals.var_temp3);
        let assign82620_e124367: f64 = (assign82620_e124365 + locals.var_temp5);
        let assign82620_e124369: f64 = (assign82620_e124367 + locals.var_temp7);
        let assign82620_e124371: f64 = (assign82620_e124369 + locals.var_temp9);
        let assign82620_e124372: f64 = (4.0 * assign82620_e124371);
        let assign82620_e124373: f64 = (locals.var_x_sp + assign82620_e124372);
        let assign82620_e124377: f64 = (locals.var_temp2 + locals.var_temp4);
        let assign82620_e124379: f64 = (assign82620_e124377 + locals.var_temp6);
        let assign82620_e124381: f64 = (assign82620_e124379 + locals.var_temp8);
        let assign82620_e124382: f64 = (2.0 * assign82620_e124381);
        let assign82620_e124383: f64 = (assign82620_e124373 + assign82620_e124382);
        let assign82620_e124385: f64 = (assign82620_e124383 + locals.var_x_dp);
        let assign82620_e124387: f64 = (assign82620_e124385 / 30.0);
        let assign82620_e124388: f64 = (locals.var_xg_ac - assign82620_e124387);
        (assign82620_e124388, (locals.var_xg_ac_dn5 - ((((locals.var_x_sp_dn5 + (4.0 * ((((locals.var_temp1_dn5 + locals.var_temp3_dn5) + locals.var_temp5_dn5) + locals.var_temp7_dn5) + locals.var_temp9_dn5))) + (2.0 * (((locals.var_temp2_dn5 + locals.var_temp4_dn5) + locals.var_temp6_dn5) + locals.var_temp8_dn5))) + locals.var_x_dp_dn5) / 30.0)), (locals.var_xg_ac_dn6 - ((((locals.var_x_sp_dn6 + (4.0 * ((((locals.var_temp1_dn6 + locals.var_temp3_dn6) + locals.var_temp5_dn6) + locals.var_temp7_dn6) + locals.var_temp9_dn6))) + (2.0 * (((locals.var_temp2_dn6 + locals.var_temp4_dn6) + locals.var_temp6_dn6) + locals.var_temp8_dn6))) + locals.var_x_dp_dn6) / 30.0)), (locals.var_xg_ac_dn7 - ((((locals.var_x_sp_dn7 + (4.0 * ((((locals.var_temp1_dn7 + locals.var_temp3_dn7) + locals.var_temp5_dn7) + locals.var_temp7_dn7) + locals.var_temp9_dn7))) + (2.0 * (((locals.var_temp2_dn7 + locals.var_temp4_dn7) + locals.var_temp6_dn7) + locals.var_temp8_dn7))) + locals.var_x_dp_dn7) / 30.0)), (locals.var_xg_ac_dn8 - ((((locals.var_x_sp_dn8 + (4.0 * ((((locals.var_temp1_dn8 + locals.var_temp3_dn8) + locals.var_temp5_dn8) + locals.var_temp7_dn8) + locals.var_temp9_dn8))) + (2.0 * (((locals.var_temp2_dn8 + locals.var_temp4_dn8) + locals.var_temp6_dn8) + locals.var_temp8_dn8))) + locals.var_x_dp_dn8) / 30.0)), (locals.var_xg_ac_dn12 - ((((locals.var_x_sp_dn12 + (4.0 * ((((locals.var_temp1_dn12 + locals.var_temp3_dn12) + locals.var_temp5_dn12) + locals.var_temp7_dn12) + locals.var_temp9_dn12))) + (2.0 * (((locals.var_temp2_dn12 + locals.var_temp4_dn12) + locals.var_temp6_dn12) + locals.var_temp8_dn12))) + locals.var_x_dp_dn12) / 30.0)), (locals.var_xg_ac_dn13 - ((((locals.var_x_sp_dn13 + (4.0 * ((((locals.var_temp1_dn13 + locals.var_temp3_dn13) + locals.var_temp5_dn13) + locals.var_temp7_dn13) + locals.var_temp9_dn13))) + (2.0 * (((locals.var_temp2_dn13 + locals.var_temp4_dn13) + locals.var_temp6_dn13) + locals.var_temp8_dn13))) + locals.var_x_dp_dn13) / 30.0)), (locals.var_xg_ac_dn14 - ((((locals.var_x_sp_dn14 + (4.0 * ((((locals.var_temp1_dn14 + locals.var_temp3_dn14) + locals.var_temp5_dn14) + locals.var_temp7_dn14) + locals.var_temp9_dn14))) + (2.0 * (((locals.var_temp2_dn14 + locals.var_temp4_dn14) + locals.var_temp6_dn14) + locals.var_temp8_dn14))) + locals.var_x_dp_dn14) / 30.0)), (locals.var_xg_ac_dn15 - ((((locals.var_x_sp_dn15 + (4.0 * ((((locals.var_temp1_dn15 + locals.var_temp3_dn15) + locals.var_temp5_dn15) + locals.var_temp7_dn15) + locals.var_temp9_dn15))) + (2.0 * (((locals.var_temp2_dn15 + locals.var_temp4_dn15) + locals.var_temp6_dn15) + locals.var_temp8_dn15))) + locals.var_x_dp_dn15) / 30.0)), (locals.var_xg_ac_dn16 - ((((locals.var_x_sp_dn16 + (4.0 * ((((locals.var_temp1_dn16 + locals.var_temp3_dn16) + locals.var_temp5_dn16) + locals.var_temp7_dn16) + locals.var_temp9_dn16))) + (2.0 * (((locals.var_temp2_dn16 + locals.var_temp4_dn16) + locals.var_temp6_dn16) + locals.var_temp8_dn16))) + locals.var_x_dp_dn16) / 30.0)), (locals.var_xg_ac_dn17 - ((((locals.var_x_sp_dn17 + (4.0 * ((((locals.var_temp1_dn17 + locals.var_temp3_dn17) + locals.var_temp5_dn17) + locals.var_temp7_dn17) + locals.var_temp9_dn17))) + (2.0 * (((locals.var_temp2_dn17 + locals.var_temp4_dn17) + locals.var_temp6_dn17) + locals.var_temp8_dn17))) + locals.var_x_dp_dn17) / 30.0)), (locals.var_xg_ac_dn18 - ((((locals.var_x_sp_dn18 + (4.0 * ((((locals.var_temp1_dn18 + locals.var_temp3_dn18) + locals.var_temp5_dn18) + locals.var_temp7_dn18) + locals.var_temp9_dn18))) + (2.0 * (((locals.var_temp2_dn18 + locals.var_temp4_dn18) + locals.var_temp6_dn18) + locals.var_temp8_dn18))) + locals.var_x_dp_dn18) / 30.0)), (locals.var_xg_ac_dn19 - ((((locals.var_x_sp_dn19 + (4.0 * ((((locals.var_temp1_dn19 + locals.var_temp3_dn19) + locals.var_temp5_dn19) + locals.var_temp7_dn19) + locals.var_temp9_dn19))) + (2.0 * (((locals.var_temp2_dn19 + locals.var_temp4_dn19) + locals.var_temp6_dn19) + locals.var_temp8_dn19))) + locals.var_x_dp_dn19) / 30.0)), (locals.var_xg_ac_dn20 - ((((locals.var_x_sp_dn20 + (4.0 * ((((locals.var_temp1_dn20 + locals.var_temp3_dn20) + locals.var_temp5_dn20) + locals.var_temp7_dn20) + locals.var_temp9_dn20))) + (2.0 * (((locals.var_temp2_dn20 + locals.var_temp4_dn20) + locals.var_temp6_dn20) + locals.var_temp8_dn20))) + locals.var_x_dp_dn20) / 30.0)),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn5, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn14, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, locals.var_qg_nqs_dn19, locals.var_qg_nqs_dn20,)
    }
};
        locals.var_qg_nqs = assign82620_e124390;
        locals.var_qg_nqs_dn5 = assign82620_e124390_d_n5;
        locals.var_qg_nqs_dn6 = assign82620_e124390_d_n6;
        locals.var_qg_nqs_dn7 = assign82620_e124390_d_n7;
        locals.var_qg_nqs_dn8 = assign82620_e124390_d_n8;
        locals.var_qg_nqs_dn12 = assign82620_e124390_d_n12;
        locals.var_qg_nqs_dn13 = assign82620_e124390_d_n13;
        locals.var_qg_nqs_dn14 = assign82620_e124390_d_n14;
        locals.var_qg_nqs_dn15 = assign82620_e124390_d_n15;
        locals.var_qg_nqs_dn16 = assign82620_e124390_d_n16;
        locals.var_qg_nqs_dn17 = assign82620_e124390_d_n17;
        locals.var_qg_nqs_dn18 = assign82620_e124390_d_n18;
        locals.var_qg_nqs_dn19 = assign82620_e124390_d_n19;
        locals.var_qg_nqs_dn20 = assign82620_e124390_d_n20;

        let (assign82630_e124396, assign82630_e124396_d_n5, assign82630_e124396_d_n6, assign82630_e124396_d_n7, assign82630_e124396_d_n8, assign82630_e124396_d_n12, assign82630_e124396_d_n13, assign82630_e124396_d_n14, assign82630_e124396_d_n15, assign82630_e124396_d_n16, assign82630_e124396_d_n17, assign82630_e124396_d_n18, assign82630_e124396_d_n19, assign82630_e124396_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82630_e124394: f64 = (locals.var_pd * locals.var_qg_nqs);
        (assign82630_e124394, ((locals.var_pd_dn5 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn5)), ((locals.var_pd_dn6 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn6)), ((locals.var_pd_dn7 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn7)), ((locals.var_pd_dn8 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn8)), ((locals.var_pd_dn12 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn12)), ((locals.var_pd_dn13 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn13)), ((locals.var_pd_dn14 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn14)), ((locals.var_pd_dn15 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn15)), ((locals.var_pd_dn16 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn16)), ((locals.var_pd_dn17 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn17)), ((locals.var_pd_dn18 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn18)), ((locals.var_pd_dn19 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn19)), ((locals.var_pd_dn20 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn20)),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn5, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn14, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, locals.var_qg_nqs_dn19, locals.var_qg_nqs_dn20,)
    }
};
        locals.var_qg_nqs = assign82630_e124396;
        locals.var_qg_nqs_dn5 = assign82630_e124396_d_n5;
        locals.var_qg_nqs_dn6 = assign82630_e124396_d_n6;
        locals.var_qg_nqs_dn7 = assign82630_e124396_d_n7;
        locals.var_qg_nqs_dn8 = assign82630_e124396_d_n8;
        locals.var_qg_nqs_dn12 = assign82630_e124396_d_n12;
        locals.var_qg_nqs_dn13 = assign82630_e124396_d_n13;
        locals.var_qg_nqs_dn14 = assign82630_e124396_d_n14;
        locals.var_qg_nqs_dn15 = assign82630_e124396_d_n15;
        locals.var_qg_nqs_dn16 = assign82630_e124396_d_n16;
        locals.var_qg_nqs_dn17 = assign82630_e124396_d_n17;
        locals.var_qg_nqs_dn18 = assign82630_e124396_d_n18;
        locals.var_qg_nqs_dn19 = assign82630_e124396_d_n19;
        locals.var_qg_nqs_dn20 = assign82630_e124396_d_n20;

        let assign82640_e124399: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign82640_e124399;

        let (assign82650_e124409, assign82650_e124409_d_n5, assign82650_e124409_d_n6, assign82650_e124409_d_n7, assign82650_e124409_d_n8, assign82650_e124409_d_n12, assign82650_e124409_d_n13, assign82650_e124409_d_n14, assign82650_e124409_d_n15, assign82650_e124409_d_n16, assign82650_e124409_d_n17, assign82650_e124409_d_n18, assign82650_e124409_d_n19, assign82650_e124409_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign82650_e124405: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82650_e124407: f64 = (assign82650_e124405 * locals.var_qs_nqs);
        (assign82650_e124407, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn20)),)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82650_e124409;
        locals.var_qs_dn5 = assign82650_e124409_d_n5;
        locals.var_qs_dn6 = assign82650_e124409_d_n6;
        locals.var_qs_dn7 = assign82650_e124409_d_n7;
        locals.var_qs_dn8 = assign82650_e124409_d_n8;
        locals.var_qs_dn12 = assign82650_e124409_d_n12;
        locals.var_qs_dn13 = assign82650_e124409_d_n13;
        locals.var_qs_dn14 = assign82650_e124409_d_n14;
        locals.var_qs_dn15 = assign82650_e124409_d_n15;
        locals.var_qs_dn16 = assign82650_e124409_d_n16;
        locals.var_qs_dn17 = assign82650_e124409_d_n17;
        locals.var_qs_dn18 = assign82650_e124409_d_n18;
        locals.var_qs_dn19 = assign82650_e124409_d_n19;
        locals.var_qs_dn20 = assign82650_e124409_d_n20;

        let (assign82660_e124419, assign82660_e124419_d_n5, assign82660_e124419_d_n6, assign82660_e124419_d_n7, assign82660_e124419_d_n8, assign82660_e124419_d_n12, assign82660_e124419_d_n13, assign82660_e124419_d_n14, assign82660_e124419_d_n15, assign82660_e124419_d_n16, assign82660_e124419_d_n17, assign82660_e124419_d_n18, assign82660_e124419_d_n19, assign82660_e124419_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign82660_e124415: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82660_e124417: f64 = (assign82660_e124415 * locals.var_qd_nqs);
        (assign82660_e124417, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn20)),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82660_e124419;
        locals.var_qd_dn5 = assign82660_e124419_d_n5;
        locals.var_qd_dn6 = assign82660_e124419_d_n6;
        locals.var_qd_dn7 = assign82660_e124419_d_n7;
        locals.var_qd_dn8 = assign82660_e124419_d_n8;
        locals.var_qd_dn12 = assign82660_e124419_d_n12;
        locals.var_qd_dn13 = assign82660_e124419_d_n13;
        locals.var_qd_dn14 = assign82660_e124419_d_n14;
        locals.var_qd_dn15 = assign82660_e124419_d_n15;
        locals.var_qd_dn16 = assign82660_e124419_d_n16;
        locals.var_qd_dn17 = assign82660_e124419_d_n17;
        locals.var_qd_dn18 = assign82660_e124419_d_n18;
        locals.var_qd_dn19 = assign82660_e124419_d_n19;
        locals.var_qd_dn20 = assign82660_e124419_d_n20;

        let (assign82670_e124430, assign82670_e124430_d_n5, assign82670_e124430_d_n6, assign82670_e124430_d_n7, assign82670_e124430_d_n8, assign82670_e124430_d_n12, assign82670_e124430_d_n13, assign82670_e124430_d_n14, assign82670_e124430_d_n15, assign82670_e124430_d_n16, assign82670_e124430_d_n17, assign82670_e124430_d_n18, assign82670_e124430_d_n19, assign82670_e124430_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign82670_e124426: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82670_e124428: f64 = (assign82670_e124426 * locals.var_qd_nqs);
        (assign82670_e124428, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn20)),)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82670_e124430;
        locals.var_qs_dn5 = assign82670_e124430_d_n5;
        locals.var_qs_dn6 = assign82670_e124430_d_n6;
        locals.var_qs_dn7 = assign82670_e124430_d_n7;
        locals.var_qs_dn8 = assign82670_e124430_d_n8;
        locals.var_qs_dn12 = assign82670_e124430_d_n12;
        locals.var_qs_dn13 = assign82670_e124430_d_n13;
        locals.var_qs_dn14 = assign82670_e124430_d_n14;
        locals.var_qs_dn15 = assign82670_e124430_d_n15;
        locals.var_qs_dn16 = assign82670_e124430_d_n16;
        locals.var_qs_dn17 = assign82670_e124430_d_n17;
        locals.var_qs_dn18 = assign82670_e124430_d_n18;
        locals.var_qs_dn19 = assign82670_e124430_d_n19;
        locals.var_qs_dn20 = assign82670_e124430_d_n20;

        let (assign82680_e124441, assign82680_e124441_d_n5, assign82680_e124441_d_n6, assign82680_e124441_d_n7, assign82680_e124441_d_n8, assign82680_e124441_d_n12, assign82680_e124441_d_n13, assign82680_e124441_d_n14, assign82680_e124441_d_n15, assign82680_e124441_d_n16, assign82680_e124441_d_n17, assign82680_e124441_d_n18, assign82680_e124441_d_n19, assign82680_e124441_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign82680_e124437: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82680_e124439: f64 = (assign82680_e124437 * locals.var_qs_nqs);
        (assign82680_e124439, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn20)),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82680_e124441;
        locals.var_qd_dn5 = assign82680_e124441_d_n5;
        locals.var_qd_dn6 = assign82680_e124441_d_n6;
        locals.var_qd_dn7 = assign82680_e124441_d_n7;
        locals.var_qd_dn8 = assign82680_e124441_d_n8;
        locals.var_qd_dn12 = assign82680_e124441_d_n12;
        locals.var_qd_dn13 = assign82680_e124441_d_n13;
        locals.var_qd_dn14 = assign82680_e124441_d_n14;
        locals.var_qd_dn15 = assign82680_e124441_d_n15;
        locals.var_qd_dn16 = assign82680_e124441_d_n16;
        locals.var_qd_dn17 = assign82680_e124441_d_n17;
        locals.var_qd_dn18 = assign82680_e124441_d_n18;
        locals.var_qd_dn19 = assign82680_e124441_d_n19;
        locals.var_qd_dn20 = assign82680_e124441_d_n20;

        let (assign82690_e124449, assign82690_e124449_d_n5, assign82690_e124449_d_n6, assign82690_e124449_d_n7, assign82690_e124449_d_n8, assign82690_e124449_d_n12, assign82690_e124449_d_n13, assign82690_e124449_d_n14, assign82690_e124449_d_n15, assign82690_e124449_d_n16, assign82690_e124449_d_n17, assign82690_e124449_d_n18, assign82690_e124449_d_n19, assign82690_e124449_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82690_e124445: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82690_e124447: f64 = (assign82690_e124445 * locals.var_qg_nqs);
        (assign82690_e124447, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn20)),)
    } else {
        (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn14, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, locals.var_qg_dn19, locals.var_qg_dn20,)
    }
};
        locals.var_qg = assign82690_e124449;
        locals.var_qg_dn5 = assign82690_e124449_d_n5;
        locals.var_qg_dn6 = assign82690_e124449_d_n6;
        locals.var_qg_dn7 = assign82690_e124449_d_n7;
        locals.var_qg_dn8 = assign82690_e124449_d_n8;
        locals.var_qg_dn12 = assign82690_e124449_d_n12;
        locals.var_qg_dn13 = assign82690_e124449_d_n13;
        locals.var_qg_dn14 = assign82690_e124449_d_n14;
        locals.var_qg_dn15 = assign82690_e124449_d_n15;
        locals.var_qg_dn16 = assign82690_e124449_d_n16;
        locals.var_qg_dn17 = assign82690_e124449_d_n17;
        locals.var_qg_dn18 = assign82690_e124449_d_n18;
        locals.var_qg_dn19 = assign82690_e124449_d_n19;
        locals.var_qg_dn20 = assign82690_e124449_d_n20;

        let (assign82700_e124458, assign82700_e124458_d_n5, assign82700_e124458_d_n6, assign82700_e124458_d_n7, assign82700_e124458_d_n8, assign82700_e124458_d_n12, assign82700_e124458_d_n13, assign82700_e124458_d_n14, assign82700_e124458_d_n15, assign82700_e124458_d_n16, assign82700_e124458_d_n17, assign82700_e124458_d_n18, assign82700_e124458_d_n19, assign82700_e124458_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82700_e124452: f64 = (-locals.var_qg);
        let assign82700_e124454: f64 = (assign82700_e124452 - locals.var_qs);
        let assign82700_e124456: f64 = (assign82700_e124454 - locals.var_qd);
        (assign82700_e124456, (((-locals.var_qg_dn5) - locals.var_qs_dn5) - locals.var_qd_dn5), (((-locals.var_qg_dn6) - locals.var_qs_dn6) - locals.var_qd_dn6), (((-locals.var_qg_dn7) - locals.var_qs_dn7) - locals.var_qd_dn7), (((-locals.var_qg_dn8) - locals.var_qs_dn8) - locals.var_qd_dn8), (((-locals.var_qg_dn12) - locals.var_qs_dn12) - locals.var_qd_dn12), (((-locals.var_qg_dn13) - locals.var_qs_dn13) - locals.var_qd_dn13), (((-locals.var_qg_dn14) - locals.var_qs_dn14) - locals.var_qd_dn14), (((-locals.var_qg_dn15) - locals.var_qs_dn15) - locals.var_qd_dn15), (((-locals.var_qg_dn16) - locals.var_qs_dn16) - locals.var_qd_dn16), (((-locals.var_qg_dn17) - locals.var_qs_dn17) - locals.var_qd_dn17), (((-locals.var_qg_dn18) - locals.var_qs_dn18) - locals.var_qd_dn18), (((-locals.var_qg_dn19) - locals.var_qs_dn19) - locals.var_qd_dn19), (((-locals.var_qg_dn20) - locals.var_qs_dn20) - locals.var_qd_dn20),)
    } else {
        (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn14, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, locals.var_qb_dn19, locals.var_qb_dn20,)
    }
};
        locals.var_qb = assign82700_e124458;
        locals.var_qb_dn5 = assign82700_e124458_d_n5;
        locals.var_qb_dn6 = assign82700_e124458_d_n6;
        locals.var_qb_dn7 = assign82700_e124458_d_n7;
        locals.var_qb_dn8 = assign82700_e124458_d_n8;
        locals.var_qb_dn12 = assign82700_e124458_d_n12;
        locals.var_qb_dn13 = assign82700_e124458_d_n13;
        locals.var_qb_dn14 = assign82700_e124458_d_n14;
        locals.var_qb_dn15 = assign82700_e124458_d_n15;
        locals.var_qb_dn16 = assign82700_e124458_d_n16;
        locals.var_qb_dn17 = assign82700_e124458_d_n17;
        locals.var_qb_dn18 = assign82700_e124458_d_n18;
        locals.var_qb_dn19 = assign82700_e124458_d_n19;
        locals.var_qb_dn20 = assign82700_e124458_d_n20;

        let assign82710_e124461: f64 = (locals.var_qg + locals.var_qb);
        let assign82710_e124463: f64 = (assign82710_e124461 + locals.var_qd);
        let assign82710_e124464: f64 = (-assign82710_e124463);
        locals.var_qs = assign82710_e124464;
        locals.var_qs_dn5 = (-((locals.var_qg_dn5 + locals.var_qb_dn5) + locals.var_qd_dn5));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn12 = (-((locals.var_qg_dn12 + locals.var_qb_dn12) + locals.var_qd_dn12));
        locals.var_qs_dn13 = (-((locals.var_qg_dn13 + locals.var_qb_dn13) + locals.var_qd_dn13));
        locals.var_qs_dn14 = (-((locals.var_qg_dn14 + locals.var_qb_dn14) + locals.var_qd_dn14));
        locals.var_qs_dn15 = (-((locals.var_qg_dn15 + locals.var_qb_dn15) + locals.var_qd_dn15));
        locals.var_qs_dn16 = (-((locals.var_qg_dn16 + locals.var_qb_dn16) + locals.var_qd_dn16));
        locals.var_qs_dn17 = (-((locals.var_qg_dn17 + locals.var_qb_dn17) + locals.var_qd_dn17));
        locals.var_qs_dn18 = (-((locals.var_qg_dn18 + locals.var_qb_dn18) + locals.var_qd_dn18));
        locals.var_qs_dn19 = (-((locals.var_qg_dn19 + locals.var_qb_dn19) + locals.var_qd_dn19));
        locals.var_qs_dn20 = (-((locals.var_qg_dn20 + locals.var_qb_dn20) + locals.var_qd_dn20));

        let assign82760_e124495: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign82760_e124495;

        let (assign82770_e124499, assign82770_e124499_d_n5, assign82770_e124499_d_n6, assign82770_e124499_d_n7, assign82770_e124499_d_n8, assign82770_e124499_d_n12, assign82770_e124499_d_n13, assign82770_e124499_d_n14, assign82770_e124499_d_n15, assign82770_e124499_d_n16, assign82770_e124499_d_n17, assign82770_e124499_d_n18, assign82770_e124499_d_n19, assign82770_e124499_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    } else {
        (locals.var_temp__blk2245, locals.var_temp__blk2245_dn5, locals.var_temp__blk2245_dn6, locals.var_temp__blk2245_dn7, locals.var_temp__blk2245_dn8, locals.var_temp__blk2245_dn12, locals.var_temp__blk2245_dn13, locals.var_temp__blk2245_dn14, locals.var_temp__blk2245_dn15, locals.var_temp__blk2245_dn16, locals.var_temp__blk2245_dn17, locals.var_temp__blk2245_dn18, locals.var_temp__blk2245_dn19, locals.var_temp__blk2245_dn20,)
    }
};
        locals.var_temp__blk2245 = assign82770_e124499;
        locals.var_temp__blk2245_dn5 = assign82770_e124499_d_n5;
        locals.var_temp__blk2245_dn6 = assign82770_e124499_d_n6;
        locals.var_temp__blk2245_dn7 = assign82770_e124499_d_n7;
        locals.var_temp__blk2245_dn8 = assign82770_e124499_d_n8;
        locals.var_temp__blk2245_dn12 = assign82770_e124499_d_n12;
        locals.var_temp__blk2245_dn13 = assign82770_e124499_d_n13;
        locals.var_temp__blk2245_dn14 = assign82770_e124499_d_n14;
        locals.var_temp__blk2245_dn15 = assign82770_e124499_d_n15;
        locals.var_temp__blk2245_dn16 = assign82770_e124499_d_n16;
        locals.var_temp__blk2245_dn17 = assign82770_e124499_d_n17;
        locals.var_temp__blk2245_dn18 = assign82770_e124499_d_n18;
        locals.var_temp__blk2245_dn19 = assign82770_e124499_d_n19;
        locals.var_temp__blk2245_dn20 = assign82770_e124499_d_n20;

    }

    pub(super) fn stamp_transient_block_165(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82780_e124503, assign82780_e124503_d_n5, assign82780_e124503_d_n6, assign82780_e124503_d_n7, assign82780_e124503_d_n8, assign82780_e124503_d_n12, assign82780_e124503_d_n13, assign82780_e124503_d_n14, assign82780_e124503_d_n15, assign82780_e124503_d_n16, assign82780_e124503_d_n17, assign82780_e124503_d_n18, assign82780_e124503_d_n19, assign82780_e124503_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82780_e124503;
        locals.var_qd_dn5 = assign82780_e124503_d_n5;
        locals.var_qd_dn6 = assign82780_e124503_d_n6;
        locals.var_qd_dn7 = assign82780_e124503_d_n7;
        locals.var_qd_dn8 = assign82780_e124503_d_n8;
        locals.var_qd_dn12 = assign82780_e124503_d_n12;
        locals.var_qd_dn13 = assign82780_e124503_d_n13;
        locals.var_qd_dn14 = assign82780_e124503_d_n14;
        locals.var_qd_dn15 = assign82780_e124503_d_n15;
        locals.var_qd_dn16 = assign82780_e124503_d_n16;
        locals.var_qd_dn17 = assign82780_e124503_d_n17;
        locals.var_qd_dn18 = assign82780_e124503_d_n18;
        locals.var_qd_dn19 = assign82780_e124503_d_n19;
        locals.var_qd_dn20 = assign82780_e124503_d_n20;

        let (assign82790_e124507, assign82790_e124507_d_n5, assign82790_e124507_d_n6, assign82790_e124507_d_n7, assign82790_e124507_d_n8, assign82790_e124507_d_n12, assign82790_e124507_d_n13, assign82790_e124507_d_n14, assign82790_e124507_d_n15, assign82790_e124507_d_n16, assign82790_e124507_d_n17, assign82790_e124507_d_n18, assign82790_e124507_d_n19, assign82790_e124507_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_temp__blk2245, locals.var_temp__blk2245_dn5, locals.var_temp__blk2245_dn6, locals.var_temp__blk2245_dn7, locals.var_temp__blk2245_dn8, locals.var_temp__blk2245_dn12, locals.var_temp__blk2245_dn13, locals.var_temp__blk2245_dn14, locals.var_temp__blk2245_dn15, locals.var_temp__blk2245_dn16, locals.var_temp__blk2245_dn17, locals.var_temp__blk2245_dn18, locals.var_temp__blk2245_dn19, locals.var_temp__blk2245_dn20,)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82790_e124507;
        locals.var_qs_dn5 = assign82790_e124507_d_n5;
        locals.var_qs_dn6 = assign82790_e124507_d_n6;
        locals.var_qs_dn7 = assign82790_e124507_d_n7;
        locals.var_qs_dn8 = assign82790_e124507_d_n8;
        locals.var_qs_dn12 = assign82790_e124507_d_n12;
        locals.var_qs_dn13 = assign82790_e124507_d_n13;
        locals.var_qs_dn14 = assign82790_e124507_d_n14;
        locals.var_qs_dn15 = assign82790_e124507_d_n15;
        locals.var_qs_dn16 = assign82790_e124507_d_n16;
        locals.var_qs_dn17 = assign82790_e124507_d_n17;
        locals.var_qs_dn18 = assign82790_e124507_d_n18;
        locals.var_qs_dn19 = assign82790_e124507_d_n19;
        locals.var_qs_dn20 = assign82790_e124507_d_n20;

        locals.var_sidexc = 0.0;
        locals.var_sidexc_dn5 = 0.0;
        locals.var_sidexc_dn6 = 0.0;
        locals.var_sidexc_dn7 = 0.0;
        locals.var_sidexc_dn8 = 0.0;
        locals.var_sidexc_dn12 = 0.0;
        locals.var_sidexc_dn13 = 0.0;
        locals.var_sidexc_dn14 = 0.0;
        locals.var_sidexc_dn15 = 0.0;
        locals.var_sidexc_dn16 = 0.0;
        locals.var_sidexc_dn17 = 0.0;
        locals.var_sidexc_dn18 = 0.0;
        locals.var_sidexc_dn19 = 0.0;
        locals.var_sidexc_dn20 = 0.0;

        locals.var_mid = 0.0;
        locals.var_mid_dn5 = 0.0;
        locals.var_mid_dn6 = 0.0;
        locals.var_mid_dn7 = 0.0;
        locals.var_mid_dn8 = 0.0;
        locals.var_mid_dn12 = 0.0;
        locals.var_mid_dn13 = 0.0;
        locals.var_mid_dn14 = 0.0;
        locals.var_mid_dn15 = 0.0;
        locals.var_mid_dn16 = 0.0;
        locals.var_mid_dn17 = 0.0;
        locals.var_mid_dn18 = 0.0;
        locals.var_mid_dn19 = 0.0;
        locals.var_mid_dn20 = 0.0;

        locals.var_mig = 1e-40;
        locals.var_mig_dn5 = 0.0;
        locals.var_mig_dn6 = 0.0;
        locals.var_mig_dn7 = 0.0;
        locals.var_mig_dn8 = 0.0;
        locals.var_mig_dn12 = 0.0;
        locals.var_mig_dn13 = 0.0;
        locals.var_mig_dn14 = 0.0;
        locals.var_mig_dn15 = 0.0;
        locals.var_mig_dn16 = 0.0;
        locals.var_mig_dn17 = 0.0;
        locals.var_mig_dn18 = 0.0;
        locals.var_mig_dn19 = 0.0;
        locals.var_mig_dn20 = 0.0;

        locals.var_migid = 0.0;
        locals.var_migid_dn5 = 0.0;
        locals.var_migid_dn6 = 0.0;
        locals.var_migid_dn7 = 0.0;
        locals.var_migid_dn8 = 0.0;
        locals.var_migid_dn12 = 0.0;
        locals.var_migid_dn13 = 0.0;
        locals.var_migid_dn14 = 0.0;
        locals.var_migid_dn15 = 0.0;
        locals.var_migid_dn16 = 0.0;
        locals.var_migid_dn17 = 0.0;
        locals.var_migid_dn18 = 0.0;
        locals.var_migid_dn19 = 0.0;
        locals.var_migid_dn20 = 0.0;

        locals.var_c_igid = 0.0;
        locals.var_c_igid_dn5 = 0.0;
        locals.var_c_igid_dn6 = 0.0;
        locals.var_c_igid_dn7 = 0.0;
        locals.var_c_igid_dn8 = 0.0;
        locals.var_c_igid_dn12 = 0.0;
        locals.var_c_igid_dn13 = 0.0;
        locals.var_c_igid_dn14 = 0.0;
        locals.var_c_igid_dn15 = 0.0;
        locals.var_c_igid_dn16 = 0.0;
        locals.var_c_igid_dn17 = 0.0;
        locals.var_c_igid_dn18 = 0.0;
        locals.var_c_igid_dn19 = 0.0;
        locals.var_c_igid_dn20 = 0.0;

        let assign82860_e124516: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign82860_e124516;
        locals.var_cgeff_dn5 = ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_dn12 = ((locals.var_cox_qm_dn12 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn12));
        locals.var_cgeff_dn13 = ((locals.var_cox_qm_dn13 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn13));
        locals.var_cgeff_dn14 = ((locals.var_cox_qm_dn14 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn14));
        locals.var_cgeff_dn15 = ((locals.var_cox_qm_dn15 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn15));
        locals.var_cgeff_dn16 = ((locals.var_cox_qm_dn16 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn16));
        locals.var_cgeff_dn17 = ((locals.var_cox_qm_dn17 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn17));
        locals.var_cgeff_dn18 = ((locals.var_cox_qm_dn18 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn18));
        locals.var_cgeff_dn19 = ((locals.var_cox_qm_dn19 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn19));
        locals.var_cgeff_dn20 = ((locals.var_cox_qm_dn20 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn20));

        locals.var_sqid = 0.0;
        locals.var_sqid_dn5 = 0.0;
        locals.var_sqid_dn6 = 0.0;
        locals.var_sqid_dn7 = 0.0;
        locals.var_sqid_dn8 = 0.0;
        locals.var_sqid_dn12 = 0.0;
        locals.var_sqid_dn13 = 0.0;
        locals.var_sqid_dn14 = 0.0;
        locals.var_sqid_dn15 = 0.0;
        locals.var_sqid_dn16 = 0.0;
        locals.var_sqid_dn17 = 0.0;
        locals.var_sqid_dn18 = 0.0;
        locals.var_sqid_dn19 = 0.0;
        locals.var_sqid_dn20 = 0.0;

        locals.var_sqig = 0.0;
        locals.var_sqig_dn5 = 0.0;
        locals.var_sqig_dn6 = 0.0;
        locals.var_sqig_dn7 = 0.0;
        locals.var_sqig_dn8 = 0.0;
        locals.var_sqig_dn12 = 0.0;
        locals.var_sqig_dn13 = 0.0;
        locals.var_sqig_dn14 = 0.0;
        locals.var_sqig_dn15 = 0.0;
        locals.var_sqig_dn16 = 0.0;
        locals.var_sqig_dn17 = 0.0;
        locals.var_sqig_dn18 = 0.0;
        locals.var_sqig_dn19 = 0.0;
        locals.var_sqig_dn20 = 0.0;

        let assign82920_e124528: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign82920_e124528;

        let assign83010_e124634: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign83010_e124634;

        let (assign83020_e124642, assign83020_e124642_d_n5, assign83020_e124642_d_n6, assign83020_e124642_d_n7, assign83020_e124642_d_n8, assign83020_e124642_d_n12, assign83020_e124642_d_n13, assign83020_e124642_d_n14, assign83020_e124642_d_n15, assign83020_e124642_d_n16, assign83020_e124642_d_n17, assign83020_e124642_d_n18, assign83020_e124642_d_n19, assign83020_e124642_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83020_e124640: f64 = (locals.var_qim1_dc / locals.var_alpha_dc);
        (assign83020_e124640, (((locals.var_qim1_dc_dn5 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn5)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn6 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn6)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn7 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn7)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn8 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn8)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn12 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn12)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn13 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn13)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn14 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn14)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn15 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn15)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn16 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn16)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn17 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn17)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn18 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn18)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn19 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn19)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn20 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn20)) / (locals.var_alpha_dc * locals.var_alpha_dc)),)
    } else {
        (locals.var_h0, locals.var_h0_dn5, locals.var_h0_dn6, locals.var_h0_dn7, locals.var_h0_dn8, locals.var_h0_dn12, locals.var_h0_dn13, locals.var_h0_dn14, locals.var_h0_dn15, locals.var_h0_dn16, locals.var_h0_dn17, locals.var_h0_dn18, locals.var_h0_dn19, locals.var_h0_dn20,)
    }
};
        locals.var_h0 = assign83020_e124642;
        locals.var_h0_dn5 = assign83020_e124642_d_n5;
        locals.var_h0_dn6 = assign83020_e124642_d_n6;
        locals.var_h0_dn7 = assign83020_e124642_d_n7;
        locals.var_h0_dn8 = assign83020_e124642_d_n8;
        locals.var_h0_dn12 = assign83020_e124642_d_n12;
        locals.var_h0_dn13 = assign83020_e124642_d_n13;
        locals.var_h0_dn14 = assign83020_e124642_d_n14;
        locals.var_h0_dn15 = assign83020_e124642_d_n15;
        locals.var_h0_dn16 = assign83020_e124642_d_n16;
        locals.var_h0_dn17 = assign83020_e124642_d_n17;
        locals.var_h0_dn18 = assign83020_e124642_d_n18;
        locals.var_h0_dn19 = assign83020_e124642_d_n19;
        locals.var_h0_dn20 = assign83020_e124642_d_n20;

        let (assign83030_e124650, assign83030_e124650_d_n5, assign83030_e124650_d_n6, assign83030_e124650_d_n7, assign83030_e124650_d_n8, assign83030_e124650_d_n12, assign83030_e124650_d_n13, assign83030_e124650_d_n14, assign83030_e124650_d_n15, assign83030_e124650_d_n16, assign83030_e124650_d_n17, assign83030_e124650_d_n18, assign83030_e124650_d_n19, assign83030_e124650_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83030_e124648: f64 = (locals.var_qim_dc / locals.var_qim1_dc);
        (assign83030_e124648, (((locals.var_qim_dc_dn5 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn6 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn7 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn8 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn12 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn12)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn13 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn13)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn14 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn14)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn15 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn15)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn16 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn16)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn17 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn17)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn18 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn18)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn19 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn19)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn20 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn20)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_t1, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, locals.var_t1_dn15, locals.var_t1_dn16, locals.var_t1_dn17, locals.var_t1_dn18, locals.var_t1_dn19, locals.var_t1_dn20,)
    }
};
        locals.var_t1 = assign83030_e124650;
        locals.var_t1_dn5 = assign83030_e124650_d_n5;
        locals.var_t1_dn6 = assign83030_e124650_d_n6;
        locals.var_t1_dn7 = assign83030_e124650_d_n7;
        locals.var_t1_dn8 = assign83030_e124650_d_n8;
        locals.var_t1_dn12 = assign83030_e124650_d_n12;
        locals.var_t1_dn13 = assign83030_e124650_d_n13;
        locals.var_t1_dn14 = assign83030_e124650_d_n14;
        locals.var_t1_dn15 = assign83030_e124650_d_n15;
        locals.var_t1_dn16 = assign83030_e124650_d_n16;
        locals.var_t1_dn17 = assign83030_e124650_d_n17;
        locals.var_t1_dn18 = assign83030_e124650_d_n18;
        locals.var_t1_dn19 = assign83030_e124650_d_n19;
        locals.var_t1_dn20 = assign83030_e124650_d_n20;

        let (assign83040_e124662, assign83040_e124662_d_n5, assign83040_e124662_d_n6, assign83040_e124662_d_n7, assign83040_e124662_d_n8, assign83040_e124662_d_n12, assign83040_e124662_d_n13, assign83040_e124662_d_n14, assign83040_e124662_d_n15, assign83040_e124662_d_n16, assign83040_e124662_d_n17, assign83040_e124662_d_n18, assign83040_e124662_d_n19, assign83040_e124662_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83040_e124656: f64 = (0.5 * 0.16666666666666666);
        let assign83040_e124659: f64 = (locals.var_dps_dc / locals.var_h0);
        let assign83040_e124660: f64 = (assign83040_e124656 * assign83040_e124659);
        (assign83040_e124660, (assign83040_e124656 * (((locals.var_dps_dc_dn5 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn5)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn6 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn6)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn7 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn7)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn8 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn8)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn12 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn12)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn13 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn13)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn14 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn14)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn15 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn15)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn16 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn16)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn17 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn17)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn18 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn18)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn19 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn19)) / (locals.var_h0 * locals.var_h0))), (assign83040_e124656 * (((locals.var_dps_dc_dn20 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn20)) / (locals.var_h0 * locals.var_h0))),)
    } else {
        (locals.var_sqt2, locals.var_sqt2_dn5, locals.var_sqt2_dn6, locals.var_sqt2_dn7, locals.var_sqt2_dn8, locals.var_sqt2_dn12, locals.var_sqt2_dn13, locals.var_sqt2_dn14, locals.var_sqt2_dn15, locals.var_sqt2_dn16, locals.var_sqt2_dn17, locals.var_sqt2_dn18, locals.var_sqt2_dn19, locals.var_sqt2_dn20,)
    }
};
        locals.var_sqt2 = assign83040_e124662;
        locals.var_sqt2_dn5 = assign83040_e124662_d_n5;
        locals.var_sqt2_dn6 = assign83040_e124662_d_n6;
        locals.var_sqt2_dn7 = assign83040_e124662_d_n7;
        locals.var_sqt2_dn8 = assign83040_e124662_d_n8;
        locals.var_sqt2_dn12 = assign83040_e124662_d_n12;
        locals.var_sqt2_dn13 = assign83040_e124662_d_n13;
        locals.var_sqt2_dn14 = assign83040_e124662_d_n14;
        locals.var_sqt2_dn15 = assign83040_e124662_d_n15;
        locals.var_sqt2_dn16 = assign83040_e124662_d_n16;
        locals.var_sqt2_dn17 = assign83040_e124662_d_n17;
        locals.var_sqt2_dn18 = assign83040_e124662_d_n18;
        locals.var_sqt2_dn19 = assign83040_e124662_d_n19;
        locals.var_sqt2_dn20 = assign83040_e124662_d_n20;

        let (assign83050_e124670, assign83050_e124670_d_n5, assign83050_e124670_d_n6, assign83050_e124670_d_n7, assign83050_e124670_d_n8, assign83050_e124670_d_n12, assign83050_e124670_d_n13, assign83050_e124670_d_n14, assign83050_e124670_d_n15, assign83050_e124670_d_n16, assign83050_e124670_d_n17, assign83050_e124670_d_n18, assign83050_e124670_d_n19, assign83050_e124670_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83050_e124668: f64 = (locals.var_sqt2 * locals.var_sqt2);
        (assign83050_e124668, ((locals.var_sqt2_dn5 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn5)), ((locals.var_sqt2_dn6 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn6)), ((locals.var_sqt2_dn7 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn7)), ((locals.var_sqt2_dn8 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn8)), ((locals.var_sqt2_dn12 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn12)), ((locals.var_sqt2_dn13 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn13)), ((locals.var_sqt2_dn14 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn14)), ((locals.var_sqt2_dn15 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn15)), ((locals.var_sqt2_dn16 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn16)), ((locals.var_sqt2_dn17 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn17)), ((locals.var_sqt2_dn18 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn18)), ((locals.var_sqt2_dn19 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn19)), ((locals.var_sqt2_dn20 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn20)),)
    } else {
        (locals.var_t2, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, locals.var_t2_dn15, locals.var_t2_dn16, locals.var_t2_dn17, locals.var_t2_dn18, locals.var_t2_dn19, locals.var_t2_dn20,)
    }
};
        locals.var_t2 = assign83050_e124670;
        locals.var_t2_dn5 = assign83050_e124670_d_n5;
        locals.var_t2_dn6 = assign83050_e124670_d_n6;
        locals.var_t2_dn7 = assign83050_e124670_d_n7;
        locals.var_t2_dn8 = assign83050_e124670_d_n8;
        locals.var_t2_dn12 = assign83050_e124670_d_n12;
        locals.var_t2_dn13 = assign83050_e124670_d_n13;
        locals.var_t2_dn14 = assign83050_e124670_d_n14;
        locals.var_t2_dn15 = assign83050_e124670_d_n15;
        locals.var_t2_dn16 = assign83050_e124670_d_n16;
        locals.var_t2_dn17 = assign83050_e124670_d_n17;
        locals.var_t2_dn18 = assign83050_e124670_d_n18;
        locals.var_t2_dn19 = assign83050_e124670_d_n19;
        locals.var_t2_dn20 = assign83050_e124670_d_n20;

        let (assign83060_e124680, assign83060_e124680_d_n5, assign83060_e124680_d_n6, assign83060_e124680_d_n7, assign83060_e124680_d_n8, assign83060_e124680_d_n12, assign83060_e124680_d_n13, assign83060_e124680_d_n14, assign83060_e124680_d_n15, assign83060_e124680_d_n16, assign83060_e124680_d_n17, assign83060_e124680_d_n18, assign83060_e124680_d_n19, assign83060_e124680_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83060_e124676: f64 = (locals.var_h0 / locals.var_h_dc);
        let assign83060_e124678: f64 = (assign83060_e124676 - 1.0);
        (assign83060_e124678, (((locals.var_h0_dn5 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn5)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn6 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn7 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn8 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn12 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn12)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn13 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn13)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn14 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn14)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn15 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn15)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn16 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn16)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn17 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn17)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn18 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn18)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn19 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn19)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn20 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn20)) / (locals.var_h_dc * locals.var_h_dc)),)
    } else {
        (locals.var_r, locals.var_r_dn5, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8, locals.var_r_dn12, locals.var_r_dn13, locals.var_r_dn14, locals.var_r_dn15, locals.var_r_dn16, locals.var_r_dn17, locals.var_r_dn18, locals.var_r_dn19, locals.var_r_dn20,)
    }
};
        locals.var_r = assign83060_e124680;
        locals.var_r_dn5 = assign83060_e124680_d_n5;
        locals.var_r_dn6 = assign83060_e124680_d_n6;
        locals.var_r_dn7 = assign83060_e124680_d_n7;
        locals.var_r_dn8 = assign83060_e124680_d_n8;
        locals.var_r_dn12 = assign83060_e124680_d_n12;
        locals.var_r_dn13 = assign83060_e124680_d_n13;
        locals.var_r_dn14 = assign83060_e124680_d_n14;
        locals.var_r_dn15 = assign83060_e124680_d_n15;
        locals.var_r_dn16 = assign83060_e124680_d_n16;
        locals.var_r_dn17 = assign83060_e124680_d_n17;
        locals.var_r_dn18 = assign83060_e124680_d_n18;
        locals.var_r_dn19 = assign83060_e124680_d_n19;
        locals.var_r_dn20 = assign83060_e124680_d_n20;

        let (assign83070_e124703, assign83070_e124703_d_n5, assign83070_e124703_d_n6, assign83070_e124703_d_n7, assign83070_e124703_d_n8, assign83070_e124703_d_n12, assign83070_e124703_d_n13, assign83070_e124703_d_n14, assign83070_e124703_d_n15, assign83070_e124703_d_n16, assign83070_e124703_d_n17, assign83070_e124703_d_n18, assign83070_e124703_d_n19, assign83070_e124703_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83070_e124688: f64 = (locals.var_r * locals.var_t2);
        let assign83070_e124689: f64 = (12.0 * assign83070_e124688);
        let assign83070_e124690: f64 = (1.0 - assign83070_e124689);
        let (assign83070_e124701, assign83070_e124701_d_n5, assign83070_e124701_d_n6, assign83070_e124701_d_n7, assign83070_e124701_d_n8, assign83070_e124701_d_n12, assign83070_e124701_d_n13, assign83070_e124701_d_n14, assign83070_e124701_d_n15, assign83070_e124701_d_n16, assign83070_e124701_d_n17, assign83070_e124701_d_n18, assign83070_e124701_d_n19, assign83070_e124701_d_n20,) = {
            if (assign83070_e124690 > 1e-20) {
                let assign83070_e124697: f64 = (locals.var_r * locals.var_t2);
                let assign83070_e124698: f64 = (12.0 * assign83070_e124697);
                let assign83070_e124699: f64 = (1.0 - assign83070_e124698);
                (assign83070_e124699, (-(12.0 * ((locals.var_r_dn5 * locals.var_t2) + (locals.var_r * locals.var_t2_dn5)))), (-(12.0 * ((locals.var_r_dn6 * locals.var_t2) + (locals.var_r * locals.var_t2_dn6)))), (-(12.0 * ((locals.var_r_dn7 * locals.var_t2) + (locals.var_r * locals.var_t2_dn7)))), (-(12.0 * ((locals.var_r_dn8 * locals.var_t2) + (locals.var_r * locals.var_t2_dn8)))), (-(12.0 * ((locals.var_r_dn12 * locals.var_t2) + (locals.var_r * locals.var_t2_dn12)))), (-(12.0 * ((locals.var_r_dn13 * locals.var_t2) + (locals.var_r * locals.var_t2_dn13)))), (-(12.0 * ((locals.var_r_dn14 * locals.var_t2) + (locals.var_r * locals.var_t2_dn14)))), (-(12.0 * ((locals.var_r_dn15 * locals.var_t2) + (locals.var_r * locals.var_t2_dn15)))), (-(12.0 * ((locals.var_r_dn16 * locals.var_t2) + (locals.var_r * locals.var_t2_dn16)))), (-(12.0 * ((locals.var_r_dn17 * locals.var_t2) + (locals.var_r * locals.var_t2_dn17)))), (-(12.0 * ((locals.var_r_dn18 * locals.var_t2) + (locals.var_r * locals.var_t2_dn18)))), (-(12.0 * ((locals.var_r_dn19 * locals.var_t2) + (locals.var_r * locals.var_t2_dn19)))), (-(12.0 * ((locals.var_r_dn20 * locals.var_t2) + (locals.var_r * locals.var_t2_dn20)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83070_e124701, assign83070_e124701_d_n5, assign83070_e124701_d_n6, assign83070_e124701_d_n7, assign83070_e124701_d_n8, assign83070_e124701_d_n12, assign83070_e124701_d_n13, assign83070_e124701_d_n14, assign83070_e124701_d_n15, assign83070_e124701_d_n16, assign83070_e124701_d_n17, assign83070_e124701_d_n18, assign83070_e124701_d_n19, assign83070_e124701_d_n20,)
    } else {
        (locals.var_lc, locals.var_lc_dn5, locals.var_lc_dn6, locals.var_lc_dn7, locals.var_lc_dn8, locals.var_lc_dn12, locals.var_lc_dn13, locals.var_lc_dn14, locals.var_lc_dn15, locals.var_lc_dn16, locals.var_lc_dn17, locals.var_lc_dn18, locals.var_lc_dn19, locals.var_lc_dn20,)
    }
};
        locals.var_lc = assign83070_e124703;
        locals.var_lc_dn5 = assign83070_e124703_d_n5;
        locals.var_lc_dn6 = assign83070_e124703_d_n6;
        locals.var_lc_dn7 = assign83070_e124703_d_n7;
        locals.var_lc_dn8 = assign83070_e124703_d_n8;
        locals.var_lc_dn12 = assign83070_e124703_d_n12;
        locals.var_lc_dn13 = assign83070_e124703_d_n13;
        locals.var_lc_dn14 = assign83070_e124703_d_n14;
        locals.var_lc_dn15 = assign83070_e124703_d_n15;
        locals.var_lc_dn16 = assign83070_e124703_d_n16;
        locals.var_lc_dn17 = assign83070_e124703_d_n17;
        locals.var_lc_dn18 = assign83070_e124703_d_n18;
        locals.var_lc_dn19 = assign83070_e124703_d_n19;
        locals.var_lc_dn20 = assign83070_e124703_d_n20;

        let (assign83080_e124713, assign83080_e124713_d_n5, assign83080_e124713_d_n6, assign83080_e124713_d_n7, assign83080_e124713_d_n8, assign83080_e124713_d_n12, assign83080_e124713_d_n13, assign83080_e124713_d_n14, assign83080_e124713_d_n15, assign83080_e124713_d_n16, assign83080_e124713_d_n17, assign83080_e124713_d_n18, assign83080_e124713_d_n19, assign83080_e124713_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83080_e124710: f64 = (locals.var_lc * locals.var_lc);
        let assign83080_e124711: f64 = (1.0 / assign83080_e124710);
        (assign83080_e124711, (-(((locals.var_lc_dn5 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn5)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn12 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn12)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn13 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn13)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn14 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn14)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn15 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn15)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn16 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn16)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn17 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn17)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn18 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn18)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn19 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn19)) / (assign83080_e124710 * assign83080_e124710))), (-(((locals.var_lc_dn20 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn20)) / (assign83080_e124710 * assign83080_e124710))),)
    } else {
        (locals.var_lcinv2, locals.var_lcinv2_dn5, locals.var_lcinv2_dn6, locals.var_lcinv2_dn7, locals.var_lcinv2_dn8, locals.var_lcinv2_dn12, locals.var_lcinv2_dn13, locals.var_lcinv2_dn14, locals.var_lcinv2_dn15, locals.var_lcinv2_dn16, locals.var_lcinv2_dn17, locals.var_lcinv2_dn18, locals.var_lcinv2_dn19, locals.var_lcinv2_dn20,)
    }
};
        locals.var_lcinv2 = assign83080_e124713;
        locals.var_lcinv2_dn5 = assign83080_e124713_d_n5;
        locals.var_lcinv2_dn6 = assign83080_e124713_d_n6;
        locals.var_lcinv2_dn7 = assign83080_e124713_d_n7;
        locals.var_lcinv2_dn8 = assign83080_e124713_d_n8;
        locals.var_lcinv2_dn12 = assign83080_e124713_d_n12;
        locals.var_lcinv2_dn13 = assign83080_e124713_d_n13;
        locals.var_lcinv2_dn14 = assign83080_e124713_d_n14;
        locals.var_lcinv2_dn15 = assign83080_e124713_d_n15;
        locals.var_lcinv2_dn16 = assign83080_e124713_d_n16;
        locals.var_lcinv2_dn17 = assign83080_e124713_d_n17;
        locals.var_lcinv2_dn18 = assign83080_e124713_d_n18;
        locals.var_lcinv2_dn19 = assign83080_e124713_d_n19;
        locals.var_lcinv2_dn20 = assign83080_e124713_d_n20;

        let (assign83090_e124723, assign83090_e124723_d_n5, assign83090_e124723_d_n6, assign83090_e124723_d_n7, assign83090_e124723_d_n8, assign83090_e124723_d_n12, assign83090_e124723_d_n13, assign83090_e124723_d_n14, assign83090_e124723_d_n15, assign83090_e124723_d_n16, assign83090_e124723_d_n17, assign83090_e124723_d_n18, assign83090_e124723_d_n19, assign83090_e124723_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83090_e124719: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign83090_e124721: f64 = (assign83090_e124719 * locals.var_gvsatinv_dc);
        (assign83090_e124721, (((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn5)), (((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn6)), (((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn7)), (((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn8)), (((locals.var_bet_i * locals.var_qim1_dc_dn12) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn12)), (((locals.var_bet_i * locals.var_qim1_dc_dn13) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn13)), (((locals.var_bet_i * locals.var_qim1_dc_dn14) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn14)), (((locals.var_bet_i * locals.var_qim1_dc_dn15) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn15)), (((locals.var_bet_i * locals.var_qim1_dc_dn16) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn16)), (((locals.var_bet_i * locals.var_qim1_dc_dn17) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn17)), (((locals.var_bet_i * locals.var_qim1_dc_dn18) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn18)), (((locals.var_bet_i * locals.var_qim1_dc_dn19) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn19)), (((locals.var_bet_i * locals.var_qim1_dc_dn20) * locals.var_gvsatinv_dc) + (assign83090_e124719 * locals.var_gvsatinv_dc_dn20)),)
    } else {
        (locals.var_g_ideal, locals.var_g_ideal_dn5, locals.var_g_ideal_dn6, locals.var_g_ideal_dn7, locals.var_g_ideal_dn8, locals.var_g_ideal_dn12, locals.var_g_ideal_dn13, locals.var_g_ideal_dn14, locals.var_g_ideal_dn15, locals.var_g_ideal_dn16, locals.var_g_ideal_dn17, locals.var_g_ideal_dn18, locals.var_g_ideal_dn19, locals.var_g_ideal_dn20,)
    }
};
        locals.var_g_ideal = assign83090_e124723;
        locals.var_g_ideal_dn5 = assign83090_e124723_d_n5;
        locals.var_g_ideal_dn6 = assign83090_e124723_d_n6;
        locals.var_g_ideal_dn7 = assign83090_e124723_d_n7;
        locals.var_g_ideal_dn8 = assign83090_e124723_d_n8;
        locals.var_g_ideal_dn12 = assign83090_e124723_d_n12;
        locals.var_g_ideal_dn13 = assign83090_e124723_d_n13;
        locals.var_g_ideal_dn14 = assign83090_e124723_d_n14;
        locals.var_g_ideal_dn15 = assign83090_e124723_d_n15;
        locals.var_g_ideal_dn16 = assign83090_e124723_d_n16;
        locals.var_g_ideal_dn17 = assign83090_e124723_d_n17;
        locals.var_g_ideal_dn18 = assign83090_e124723_d_n18;
        locals.var_g_ideal_dn19 = assign83090_e124723_d_n19;
        locals.var_g_ideal_dn20 = assign83090_e124723_d_n20;

        let (assign83100_e124743, assign83100_e124743_d_n5, assign83100_e124743_d_n6, assign83100_e124743_d_n7, assign83100_e124743_d_n8, assign83100_e124743_d_n12, assign83100_e124743_d_n13, assign83100_e124743_d_n14, assign83100_e124743_d_n15, assign83100_e124743_d_n16, assign83100_e124743_d_n17, assign83100_e124743_d_n18, assign83100_e124743_d_n19, assign83100_e124743_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83100_e124730: f64 = (12.0 * locals.var_t2);
        let assign83100_e124731: f64 = (locals.var_t1 + assign83100_e124730);
        let assign83100_e124735: f64 = (1.0 + locals.var_t1);
        let assign83100_e124737: f64 = (assign83100_e124735 * locals.var_t2);
        let assign83100_e124739: f64 = (assign83100_e124737 * locals.var_r);
        let assign83100_e124740: f64 = (24.0 * assign83100_e124739);
        let assign83100_e124741: f64 = (assign83100_e124731 - assign83100_e124740);
        (assign83100_e124741, ((locals.var_t1_dn5 + (12.0 * locals.var_t2_dn5)) - (24.0 * ((((locals.var_t1_dn5 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn5)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn5)))), ((locals.var_t1_dn6 + (12.0 * locals.var_t2_dn6)) - (24.0 * ((((locals.var_t1_dn6 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn6)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn6)))), ((locals.var_t1_dn7 + (12.0 * locals.var_t2_dn7)) - (24.0 * ((((locals.var_t1_dn7 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn7)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn7)))), ((locals.var_t1_dn8 + (12.0 * locals.var_t2_dn8)) - (24.0 * ((((locals.var_t1_dn8 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn8)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn8)))), ((locals.var_t1_dn12 + (12.0 * locals.var_t2_dn12)) - (24.0 * ((((locals.var_t1_dn12 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn12)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn12)))), ((locals.var_t1_dn13 + (12.0 * locals.var_t2_dn13)) - (24.0 * ((((locals.var_t1_dn13 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn13)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn13)))), ((locals.var_t1_dn14 + (12.0 * locals.var_t2_dn14)) - (24.0 * ((((locals.var_t1_dn14 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn14)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn14)))), ((locals.var_t1_dn15 + (12.0 * locals.var_t2_dn15)) - (24.0 * ((((locals.var_t1_dn15 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn15)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn15)))), ((locals.var_t1_dn16 + (12.0 * locals.var_t2_dn16)) - (24.0 * ((((locals.var_t1_dn16 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn16)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn16)))), ((locals.var_t1_dn17 + (12.0 * locals.var_t2_dn17)) - (24.0 * ((((locals.var_t1_dn17 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn17)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn17)))), ((locals.var_t1_dn18 + (12.0 * locals.var_t2_dn18)) - (24.0 * ((((locals.var_t1_dn18 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn18)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn18)))), ((locals.var_t1_dn19 + (12.0 * locals.var_t2_dn19)) - (24.0 * ((((locals.var_t1_dn19 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn19)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn19)))), ((locals.var_t1_dn20 + (12.0 * locals.var_t2_dn20)) - (24.0 * ((((locals.var_t1_dn20 * locals.var_t2) + (assign83100_e124735 * locals.var_t2_dn20)) * locals.var_r) + (assign83100_e124737 * locals.var_r_dn20)))),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn12, locals.var_mid_dn13, locals.var_mid_dn14, locals.var_mid_dn15, locals.var_mid_dn16, locals.var_mid_dn17, locals.var_mid_dn18, locals.var_mid_dn19, locals.var_mid_dn20,)
    }
};
        locals.var_mid = assign83100_e124743;
        locals.var_mid_dn5 = assign83100_e124743_d_n5;
        locals.var_mid_dn6 = assign83100_e124743_d_n6;
        locals.var_mid_dn7 = assign83100_e124743_d_n7;
        locals.var_mid_dn8 = assign83100_e124743_d_n8;
        locals.var_mid_dn12 = assign83100_e124743_d_n12;
        locals.var_mid_dn13 = assign83100_e124743_d_n13;
        locals.var_mid_dn14 = assign83100_e124743_d_n14;
        locals.var_mid_dn15 = assign83100_e124743_d_n15;
        locals.var_mid_dn16 = assign83100_e124743_d_n16;
        locals.var_mid_dn17 = assign83100_e124743_d_n17;
        locals.var_mid_dn18 = assign83100_e124743_d_n18;
        locals.var_mid_dn19 = assign83100_e124743_d_n19;
        locals.var_mid_dn20 = assign83100_e124743_d_n20;

        let (assign83110_e124754, assign83110_e124754_d_n5, assign83110_e124754_d_n6, assign83110_e124754_d_n7, assign83110_e124754_d_n8, assign83110_e124754_d_n12, assign83110_e124754_d_n13, assign83110_e124754_d_n14, assign83110_e124754_d_n15, assign83110_e124754_d_n16, assign83110_e124754_d_n17, assign83110_e124754_d_n18, assign83110_e124754_d_n19, assign83110_e124754_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let (assign83110_e124752, assign83110_e124752_d_n5, assign83110_e124752_d_n6, assign83110_e124752_d_n7, assign83110_e124752_d_n8, assign83110_e124752_d_n12, assign83110_e124752_d_n13, assign83110_e124752_d_n14, assign83110_e124752_d_n15, assign83110_e124752_d_n16, assign83110_e124752_d_n17, assign83110_e124752_d_n18, assign83110_e124752_d_n19, assign83110_e124752_d_n20,) = {
            if (locals.var_mid > 1e-40) {
                (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn12, locals.var_mid_dn13, locals.var_mid_dn14, locals.var_mid_dn15, locals.var_mid_dn16, locals.var_mid_dn17, locals.var_mid_dn18, locals.var_mid_dn19, locals.var_mid_dn20,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83110_e124752, assign83110_e124752_d_n5, assign83110_e124752_d_n6, assign83110_e124752_d_n7, assign83110_e124752_d_n8, assign83110_e124752_d_n12, assign83110_e124752_d_n13, assign83110_e124752_d_n14, assign83110_e124752_d_n15, assign83110_e124752_d_n16, assign83110_e124752_d_n17, assign83110_e124752_d_n18, assign83110_e124752_d_n19, assign83110_e124752_d_n20,)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn12, locals.var_mid_dn13, locals.var_mid_dn14, locals.var_mid_dn15, locals.var_mid_dn16, locals.var_mid_dn17, locals.var_mid_dn18, locals.var_mid_dn19, locals.var_mid_dn20,)
    }
};
        locals.var_mid = assign83110_e124754;
        locals.var_mid_dn5 = assign83110_e124754_d_n5;
        locals.var_mid_dn6 = assign83110_e124754_d_n6;
        locals.var_mid_dn7 = assign83110_e124754_d_n7;
        locals.var_mid_dn8 = assign83110_e124754_d_n8;
        locals.var_mid_dn12 = assign83110_e124754_d_n12;
        locals.var_mid_dn13 = assign83110_e124754_d_n13;
        locals.var_mid_dn14 = assign83110_e124754_d_n14;
        locals.var_mid_dn15 = assign83110_e124754_d_n15;
        locals.var_mid_dn16 = assign83110_e124754_d_n16;
        locals.var_mid_dn17 = assign83110_e124754_d_n17;
        locals.var_mid_dn18 = assign83110_e124754_d_n18;
        locals.var_mid_dn19 = assign83110_e124754_d_n19;
        locals.var_mid_dn20 = assign83110_e124754_d_n20;

        let (assign83120_e124764, assign83120_e124764_d_n5, assign83120_e124764_d_n6, assign83120_e124764_d_n7, assign83120_e124764_d_n8, assign83120_e124764_d_n12, assign83120_e124764_d_n13, assign83120_e124764_d_n14, assign83120_e124764_d_n15, assign83120_e124764_d_n16, assign83120_e124764_d_n17, assign83120_e124764_d_n18, assign83120_e124764_d_n19, assign83120_e124764_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83120_e124760: f64 = (locals.var_g_ideal * locals.var_lcinv2);
        let assign83120_e124762: f64 = (assign83120_e124760 * locals.var_mid);
        (assign83120_e124762, ((((locals.var_g_ideal_dn5 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn5)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn5)), ((((locals.var_g_ideal_dn6 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn6)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn6)), ((((locals.var_g_ideal_dn7 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn7)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn7)), ((((locals.var_g_ideal_dn8 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn8)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn8)), ((((locals.var_g_ideal_dn12 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn12)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn12)), ((((locals.var_g_ideal_dn13 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn13)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn13)), ((((locals.var_g_ideal_dn14 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn14)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn14)), ((((locals.var_g_ideal_dn15 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn15)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn15)), ((((locals.var_g_ideal_dn16 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn16)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn16)), ((((locals.var_g_ideal_dn17 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn17)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn17)), ((((locals.var_g_ideal_dn18 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn18)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn18)), ((((locals.var_g_ideal_dn19 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn19)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn19)), ((((locals.var_g_ideal_dn20 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn20)) * locals.var_mid) + (assign83120_e124760 * locals.var_mid_dn20)),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn12, locals.var_mid_dn13, locals.var_mid_dn14, locals.var_mid_dn15, locals.var_mid_dn16, locals.var_mid_dn17, locals.var_mid_dn18, locals.var_mid_dn19, locals.var_mid_dn20,)
    }
};
        locals.var_mid = assign83120_e124764;
        locals.var_mid_dn5 = assign83120_e124764_d_n5;
        locals.var_mid_dn6 = assign83120_e124764_d_n6;
        locals.var_mid_dn7 = assign83120_e124764_d_n7;
        locals.var_mid_dn8 = assign83120_e124764_d_n8;
        locals.var_mid_dn12 = assign83120_e124764_d_n12;
        locals.var_mid_dn13 = assign83120_e124764_d_n13;
        locals.var_mid_dn14 = assign83120_e124764_d_n14;
        locals.var_mid_dn15 = assign83120_e124764_d_n15;
        locals.var_mid_dn16 = assign83120_e124764_d_n16;
        locals.var_mid_dn17 = assign83120_e124764_d_n17;
        locals.var_mid_dn18 = assign83120_e124764_d_n18;
        locals.var_mid_dn19 = assign83120_e124764_d_n19;
        locals.var_mid_dn20 = assign83120_e124764_d_n20;

        let assign83130_e124767: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign83130_e124767;

        let (assign83140_e124777, assign83140_e124777_d_n5, assign83140_e124777_d_n6, assign83140_e124777_d_n7, assign83140_e124777_d_n8, assign83140_e124777_d_n12, assign83140_e124777_d_n13, assign83140_e124777_d_n14, assign83140_e124777_d_n15, assign83140_e124777_d_n16, assign83140_e124777_d_n17, assign83140_e124777_d_n18, assign83140_e124777_d_n19, assign83140_e124777_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83140_e124775: f64 = (locals.var_thesateff_dc / locals.var_gmob_dc);
        (assign83140_e124775, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn12 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn12)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn13 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn13)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn14 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn14)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn15 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn15)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn16 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn16)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn17 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn17)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn18 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn18)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn19 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn19)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn20 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn20)) / (locals.var_gmob_dc * locals.var_gmob_dc)),)
    } else {
        (locals.var_thesat1_exc, locals.var_thesat1_exc_dn5, locals.var_thesat1_exc_dn6, locals.var_thesat1_exc_dn7, locals.var_thesat1_exc_dn8, locals.var_thesat1_exc_dn12, locals.var_thesat1_exc_dn13, locals.var_thesat1_exc_dn14, locals.var_thesat1_exc_dn15, locals.var_thesat1_exc_dn16, locals.var_thesat1_exc_dn17, locals.var_thesat1_exc_dn18, locals.var_thesat1_exc_dn19, locals.var_thesat1_exc_dn20,)
    }
};
        locals.var_thesat1_exc = assign83140_e124777;
        locals.var_thesat1_exc_dn5 = assign83140_e124777_d_n5;
        locals.var_thesat1_exc_dn6 = assign83140_e124777_d_n6;
        locals.var_thesat1_exc_dn7 = assign83140_e124777_d_n7;
        locals.var_thesat1_exc_dn8 = assign83140_e124777_d_n8;
        locals.var_thesat1_exc_dn12 = assign83140_e124777_d_n12;
        locals.var_thesat1_exc_dn13 = assign83140_e124777_d_n13;
        locals.var_thesat1_exc_dn14 = assign83140_e124777_d_n14;
        locals.var_thesat1_exc_dn15 = assign83140_e124777_d_n15;
        locals.var_thesat1_exc_dn16 = assign83140_e124777_d_n16;
        locals.var_thesat1_exc_dn17 = assign83140_e124777_d_n17;
        locals.var_thesat1_exc_dn18 = assign83140_e124777_d_n18;
        locals.var_thesat1_exc_dn19 = assign83140_e124777_d_n19;
        locals.var_thesat1_exc_dn20 = assign83140_e124777_d_n20;

        let (assign83150_e124791, assign83150_e124791_d_n5, assign83150_e124791_d_n6, assign83150_e124791_d_n7, assign83150_e124791_d_n8, assign83150_e124791_d_n12, assign83150_e124791_d_n13, assign83150_e124791_d_n14, assign83150_e124791_d_n15, assign83150_e124791_d_n16, assign83150_e124791_d_n17, assign83150_e124791_d_n18, assign83150_e124791_d_n19, assign83150_e124791_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83150_e124785: f64 = (locals.var_thesat1_exc * locals.var_thesat1_exc);
        let assign83150_e124787: f64 = (assign83150_e124785 * locals.var_dps_dc);
        let assign83150_e124789: f64 = (assign83150_e124787 * locals.var_dps_dc);
        (assign83150_e124789, ((((((locals.var_thesat1_exc_dn5 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn5)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_exc_dn6 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn6)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_exc_dn7 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn7)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_exc_dn8 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn8)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn8)), ((((((locals.var_thesat1_exc_dn12 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn12)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn12)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn12)), ((((((locals.var_thesat1_exc_dn13 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn13)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn13)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn13)), ((((((locals.var_thesat1_exc_dn14 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn14)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn14)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn14)), ((((((locals.var_thesat1_exc_dn15 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn15)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn15)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn15)), ((((((locals.var_thesat1_exc_dn16 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn16)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn16)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn16)), ((((((locals.var_thesat1_exc_dn17 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn17)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn17)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn17)), ((((((locals.var_thesat1_exc_dn18 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn18)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn18)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn18)), ((((((locals.var_thesat1_exc_dn19 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn19)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn19)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn19)), ((((((locals.var_thesat1_exc_dn20 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn20)) * locals.var_dps_dc) + (assign83150_e124785 * locals.var_dps_dc_dn20)) * locals.var_dps_dc) + (assign83150_e124787 * locals.var_dps_dc_dn20)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, locals.var_zsat_exc_dn12, locals.var_zsat_exc_dn13, locals.var_zsat_exc_dn14, locals.var_zsat_exc_dn15, locals.var_zsat_exc_dn16, locals.var_zsat_exc_dn17, locals.var_zsat_exc_dn18, locals.var_zsat_exc_dn19, locals.var_zsat_exc_dn20,)
    }
};
        locals.var_zsat_exc = assign83150_e124791;
        locals.var_zsat_exc_dn5 = assign83150_e124791_d_n5;
        locals.var_zsat_exc_dn6 = assign83150_e124791_d_n6;
        locals.var_zsat_exc_dn7 = assign83150_e124791_d_n7;
        locals.var_zsat_exc_dn8 = assign83150_e124791_d_n8;
        locals.var_zsat_exc_dn12 = assign83150_e124791_d_n12;
        locals.var_zsat_exc_dn13 = assign83150_e124791_d_n13;
        locals.var_zsat_exc_dn14 = assign83150_e124791_d_n14;
        locals.var_zsat_exc_dn15 = assign83150_e124791_d_n15;
        locals.var_zsat_exc_dn16 = assign83150_e124791_d_n16;
        locals.var_zsat_exc_dn17 = assign83150_e124791_d_n17;
        locals.var_zsat_exc_dn18 = assign83150_e124791_d_n18;
        locals.var_zsat_exc_dn19 = assign83150_e124791_d_n19;
        locals.var_zsat_exc_dn20 = assign83150_e124791_d_n20;

        let assign83160_e124794: f64 = (-1.0);
        let assign83160_e124795: f64 = if locals.var_chnl_type == assign83160_e124794 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign83160_e124795;

    }

    pub(super) fn stamp_transient_block_166(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83170_e124811, assign83170_e124811_d_n5, assign83170_e124811_d_n6, assign83170_e124811_d_n7, assign83170_e124811_d_n8, assign83170_e124811_d_n12, assign83170_e124811_d_n13, assign83170_e124811_d_n14, assign83170_e124811_d_n15, assign83170_e124811_d_n16, assign83170_e124811_d_n17, assign83170_e124811_d_n18, assign83170_e124811_d_n19, assign83170_e124811_d_n20,) = {
    if ((((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 != 0.0)) {
        let assign83170_e124807: f64 = (locals.var_thesat1_exc * locals.var_dps_dc);
        let assign83170_e124808: f64 = (1.0 + assign83170_e124807);
        let assign83170_e124809: f64 = (locals.var_zsat_exc / assign83170_e124808);
        (assign83170_e124809, (((locals.var_zsat_exc_dn5 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn5)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn6 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn6)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn7 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn7)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn8 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn8)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn12 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn12 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn12)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn13 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn13 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn13)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn14 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn14 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn14)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn15 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn15 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn15)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn16 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn16 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn16)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn17 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn17 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn17)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn18 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn18 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn18)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn19 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn19 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn19)))) / (assign83170_e124808 * assign83170_e124808)), (((locals.var_zsat_exc_dn20 * assign83170_e124808) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn20 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn20)))) / (assign83170_e124808 * assign83170_e124808)),)
    } else {
        (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, locals.var_zsat_exc_dn12, locals.var_zsat_exc_dn13, locals.var_zsat_exc_dn14, locals.var_zsat_exc_dn15, locals.var_zsat_exc_dn16, locals.var_zsat_exc_dn17, locals.var_zsat_exc_dn18, locals.var_zsat_exc_dn19, locals.var_zsat_exc_dn20,)
    }
};
        locals.var_zsat_exc = assign83170_e124811;
        locals.var_zsat_exc_dn5 = assign83170_e124811_d_n5;
        locals.var_zsat_exc_dn6 = assign83170_e124811_d_n6;
        locals.var_zsat_exc_dn7 = assign83170_e124811_d_n7;
        locals.var_zsat_exc_dn8 = assign83170_e124811_d_n8;
        locals.var_zsat_exc_dn12 = assign83170_e124811_d_n12;
        locals.var_zsat_exc_dn13 = assign83170_e124811_d_n13;
        locals.var_zsat_exc_dn14 = assign83170_e124811_d_n14;
        locals.var_zsat_exc_dn15 = assign83170_e124811_d_n15;
        locals.var_zsat_exc_dn16 = assign83170_e124811_d_n16;
        locals.var_zsat_exc_dn17 = assign83170_e124811_d_n17;
        locals.var_zsat_exc_dn18 = assign83170_e124811_d_n18;
        locals.var_zsat_exc_dn19 = assign83170_e124811_d_n19;
        locals.var_zsat_exc_dn20 = assign83170_e124811_d_n20;

        let (assign83180_e124830, assign83180_e124830_d_n5, assign83180_e124830_d_n6, assign83180_e124830_d_n7, assign83180_e124830_d_n8, assign83180_e124830_d_n12, assign83180_e124830_d_n13, assign83180_e124830_d_n14, assign83180_e124830_d_n15, assign83180_e124830_d_n16, assign83180_e124830_d_n17, assign83180_e124830_d_n18, assign83180_e124830_d_n19, assign83180_e124830_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83180_e124823: f64 = (2.0 * locals.var_zsat_exc);
        let assign83180_e124824: f64 = (1.0 + assign83180_e124823);
        let assign83180_e124825: f64 = (assign83180_e124824).sqrt();
        let assign83180_e124826: f64 = (1.0 + assign83180_e124825);
        let assign83180_e124827: f64 = (locals.var_gmob_dc * assign83180_e124826);
        let assign83180_e124828: f64 = (0.5 * assign83180_e124827);
        (assign83180_e124828, (0.5 * ((locals.var_gmob_dc_dn5 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn5) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn6 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn6) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn7 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn7) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn8 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn8) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn12 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn12) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn13 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn13) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn14 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn14) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn15 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn15) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn16 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn16) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn17 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn17) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn18 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn18) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn19 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn19) / (2.0 * assign83180_e124825))))), (0.5 * ((locals.var_gmob_dc_dn20 * assign83180_e124826) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn20) / (2.0 * assign83180_e124825))))),)
    } else {
        (locals.var_gvsat_exc, locals.var_gvsat_exc_dn5, locals.var_gvsat_exc_dn6, locals.var_gvsat_exc_dn7, locals.var_gvsat_exc_dn8, locals.var_gvsat_exc_dn12, locals.var_gvsat_exc_dn13, locals.var_gvsat_exc_dn14, locals.var_gvsat_exc_dn15, locals.var_gvsat_exc_dn16, locals.var_gvsat_exc_dn17, locals.var_gvsat_exc_dn18, locals.var_gvsat_exc_dn19, locals.var_gvsat_exc_dn20,)
    }
};
        locals.var_gvsat_exc = assign83180_e124830;
        locals.var_gvsat_exc_dn5 = assign83180_e124830_d_n5;
        locals.var_gvsat_exc_dn6 = assign83180_e124830_d_n6;
        locals.var_gvsat_exc_dn7 = assign83180_e124830_d_n7;
        locals.var_gvsat_exc_dn8 = assign83180_e124830_d_n8;
        locals.var_gvsat_exc_dn12 = assign83180_e124830_d_n12;
        locals.var_gvsat_exc_dn13 = assign83180_e124830_d_n13;
        locals.var_gvsat_exc_dn14 = assign83180_e124830_d_n14;
        locals.var_gvsat_exc_dn15 = assign83180_e124830_d_n15;
        locals.var_gvsat_exc_dn16 = assign83180_e124830_d_n16;
        locals.var_gvsat_exc_dn17 = assign83180_e124830_d_n17;
        locals.var_gvsat_exc_dn18 = assign83180_e124830_d_n18;
        locals.var_gvsat_exc_dn19 = assign83180_e124830_d_n19;
        locals.var_gvsat_exc_dn20 = assign83180_e124830_d_n20;

        let (assign83190_e124842, assign83190_e124842_d_n5, assign83190_e124842_d_n6, assign83190_e124842_d_n7, assign83190_e124842_d_n8, assign83190_e124842_d_n12, assign83190_e124842_d_n13, assign83190_e124842_d_n14, assign83190_e124842_d_n15, assign83190_e124842_d_n16, assign83190_e124842_d_n17, assign83190_e124842_d_n18, assign83190_e124842_d_n19, assign83190_e124842_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83190_e124839: f64 = (locals.var_gvsat_exc * locals.var_lc);
        let assign83190_e124840: f64 = (locals.var_gmob_dc / assign83190_e124839);
        (assign83190_e124840, (((locals.var_gmob_dc_dn5 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn5 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn5)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn6 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn6 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn6)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn7 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn7 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn7)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn8 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn8 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn8)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn12 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn12 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn12)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn13 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn13 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn13)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn14 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn14 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn14)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn15 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn15 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn15)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn16 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn16 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn16)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn17 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn17 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn17)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn18 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn18 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn18)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn19 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn19 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn19)))) / (assign83190_e124839 * assign83190_e124839)), (((locals.var_gmob_dc_dn20 * assign83190_e124839) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn20 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn20)))) / (assign83190_e124839 * assign83190_e124839)),)
    } else {
        (locals.var_gfac, locals.var_gfac_dn5, locals.var_gfac_dn6, locals.var_gfac_dn7, locals.var_gfac_dn8, locals.var_gfac_dn12, locals.var_gfac_dn13, locals.var_gfac_dn14, locals.var_gfac_dn15, locals.var_gfac_dn16, locals.var_gfac_dn17, locals.var_gfac_dn18, locals.var_gfac_dn19, locals.var_gfac_dn20,)
    }
};
        locals.var_gfac = assign83190_e124842;
        locals.var_gfac_dn5 = assign83190_e124842_d_n5;
        locals.var_gfac_dn6 = assign83190_e124842_d_n6;
        locals.var_gfac_dn7 = assign83190_e124842_d_n7;
        locals.var_gfac_dn8 = assign83190_e124842_d_n8;
        locals.var_gfac_dn12 = assign83190_e124842_d_n12;
        locals.var_gfac_dn13 = assign83190_e124842_d_n13;
        locals.var_gfac_dn14 = assign83190_e124842_d_n14;
        locals.var_gfac_dn15 = assign83190_e124842_d_n15;
        locals.var_gfac_dn16 = assign83190_e124842_d_n16;
        locals.var_gfac_dn17 = assign83190_e124842_d_n17;
        locals.var_gfac_dn18 = assign83190_e124842_d_n18;
        locals.var_gfac_dn19 = assign83190_e124842_d_n19;
        locals.var_gfac_dn20 = assign83190_e124842_d_n20;

        let (assign83200_e124858, assign83200_e124858_d_n5, assign83200_e124858_d_n6, assign83200_e124858_d_n7, assign83200_e124858_d_n8, assign83200_e124858_d_n12, assign83200_e124858_d_n13, assign83200_e124858_d_n14, assign83200_e124858_d_n15, assign83200_e124858_d_n16, assign83200_e124858_d_n17, assign83200_e124858_d_n18, assign83200_e124858_d_n19, assign83200_e124858_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83200_e124850: f64 = (locals.var_fac_exc * locals.var_i_ds);
        let assign83200_e124852: f64 = (assign83200_e124850 * locals.var_vdse_dc);
        let assign83200_e124854: f64 = (assign83200_e124852 * locals.var_gfac);
        let assign83200_e124856: f64 = (assign83200_e124854 * locals.var_gfac);
        (assign83200_e124856, (((((((locals.var_fac_exc * locals.var_i_ds_dn5) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn5)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn5)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn5)), (((((((locals.var_fac_exc * locals.var_i_ds_dn6) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn6)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn6)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn6)), (((((((locals.var_fac_exc * locals.var_i_ds_dn7) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn7)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn7)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn7)), (((((((locals.var_fac_exc * locals.var_i_ds_dn8) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn8)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn8)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn8)), (((((((locals.var_fac_exc * locals.var_i_ds_dn12) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn12)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn12)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn12)), (((((((locals.var_fac_exc * locals.var_i_ds_dn13) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn13)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn13)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn13)), (((((((locals.var_fac_exc * locals.var_i_ds_dn14) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn14)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn14)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn14)), (((((((locals.var_fac_exc * locals.var_i_ds_dn15) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn15)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn15)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn15)), (((((((locals.var_fac_exc * locals.var_i_ds_dn16) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn16)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn16)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn16)), (((((((locals.var_fac_exc * locals.var_i_ds_dn17) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn17)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn17)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn17)), (((((((locals.var_fac_exc * locals.var_i_ds_dn18) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn18)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn18)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn18)), (((((((locals.var_fac_exc * locals.var_i_ds_dn19) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn19)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn19)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn19)), (((((((locals.var_fac_exc * locals.var_i_ds_dn20) * locals.var_vdse_dc) + (assign83200_e124850 * locals.var_vdse_dc_dn20)) * locals.var_gfac) + (assign83200_e124852 * locals.var_gfac_dn20)) * locals.var_gfac) + (assign83200_e124854 * locals.var_gfac_dn20)),)
    } else {
        (locals.var_sidexc, locals.var_sidexc_dn5, locals.var_sidexc_dn6, locals.var_sidexc_dn7, locals.var_sidexc_dn8, locals.var_sidexc_dn12, locals.var_sidexc_dn13, locals.var_sidexc_dn14, locals.var_sidexc_dn15, locals.var_sidexc_dn16, locals.var_sidexc_dn17, locals.var_sidexc_dn18, locals.var_sidexc_dn19, locals.var_sidexc_dn20,)
    }
};
        locals.var_sidexc = assign83200_e124858;
        locals.var_sidexc_dn5 = assign83200_e124858_d_n5;
        locals.var_sidexc_dn6 = assign83200_e124858_d_n6;
        locals.var_sidexc_dn7 = assign83200_e124858_d_n7;
        locals.var_sidexc_dn8 = assign83200_e124858_d_n8;
        locals.var_sidexc_dn12 = assign83200_e124858_d_n12;
        locals.var_sidexc_dn13 = assign83200_e124858_d_n13;
        locals.var_sidexc_dn14 = assign83200_e124858_d_n14;
        locals.var_sidexc_dn15 = assign83200_e124858_d_n15;
        locals.var_sidexc_dn16 = assign83200_e124858_d_n16;
        locals.var_sidexc_dn17 = assign83200_e124858_d_n17;
        locals.var_sidexc_dn18 = assign83200_e124858_d_n18;
        locals.var_sidexc_dn19 = assign83200_e124858_d_n19;
        locals.var_sidexc_dn20 = assign83200_e124858_d_n20;

        let (assign83210_e124870, assign83210_e124870_d_n5, assign83210_e124870_d_n6, assign83210_e124870_d_n7, assign83210_e124870_d_n8, assign83210_e124870_d_n12, assign83210_e124870_d_n13, assign83210_e124870_d_n14, assign83210_e124870_d_n15, assign83210_e124870_d_n16, assign83210_e124870_d_n17, assign83210_e124870_d_n18, assign83210_e124870_d_n19, assign83210_e124870_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign83210_e124867: f64 = (locals.var_sidexc / locals.var_nt0);
        let assign83210_e124868: f64 = (locals.var_mid + assign83210_e124867);
        (assign83210_e124868, (locals.var_mid_dn5 + (locals.var_sidexc_dn5 / locals.var_nt0)), (locals.var_mid_dn6 + (locals.var_sidexc_dn6 / locals.var_nt0)), (locals.var_mid_dn7 + (locals.var_sidexc_dn7 / locals.var_nt0)), (locals.var_mid_dn8 + (locals.var_sidexc_dn8 / locals.var_nt0)), (locals.var_mid_dn12 + (locals.var_sidexc_dn12 / locals.var_nt0)), (locals.var_mid_dn13 + (locals.var_sidexc_dn13 / locals.var_nt0)), (locals.var_mid_dn14 + (locals.var_sidexc_dn14 / locals.var_nt0)), (locals.var_mid_dn15 + (locals.var_sidexc_dn15 / locals.var_nt0)), (locals.var_mid_dn16 + (locals.var_sidexc_dn16 / locals.var_nt0)), (locals.var_mid_dn17 + (locals.var_sidexc_dn17 / locals.var_nt0)), (locals.var_mid_dn18 + (locals.var_sidexc_dn18 / locals.var_nt0)), (locals.var_mid_dn19 + (locals.var_sidexc_dn19 / locals.var_nt0)), (locals.var_mid_dn20 + (locals.var_sidexc_dn20 / locals.var_nt0)),)
    } else {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, locals.var_mid_dn12, locals.var_mid_dn13, locals.var_mid_dn14, locals.var_mid_dn15, locals.var_mid_dn16, locals.var_mid_dn17, locals.var_mid_dn18, locals.var_mid_dn19, locals.var_mid_dn20,)
    }
};
        locals.var_mid = assign83210_e124870;
        locals.var_mid_dn5 = assign83210_e124870_d_n5;
        locals.var_mid_dn6 = assign83210_e124870_d_n6;
        locals.var_mid_dn7 = assign83210_e124870_d_n7;
        locals.var_mid_dn8 = assign83210_e124870_d_n8;
        locals.var_mid_dn12 = assign83210_e124870_d_n12;
        locals.var_mid_dn13 = assign83210_e124870_d_n13;
        locals.var_mid_dn14 = assign83210_e124870_d_n14;
        locals.var_mid_dn15 = assign83210_e124870_d_n15;
        locals.var_mid_dn16 = assign83210_e124870_d_n16;
        locals.var_mid_dn17 = assign83210_e124870_d_n17;
        locals.var_mid_dn18 = assign83210_e124870_d_n18;
        locals.var_mid_dn19 = assign83210_e124870_d_n19;
        locals.var_mid_dn20 = assign83210_e124870_d_n20;

        let (assign83220_e124879, assign83220_e124879_d_n5, assign83220_e124879_d_n6, assign83220_e124879_d_n7, assign83220_e124879_d_n8, assign83220_e124879_d_n12, assign83220_e124879_d_n13, assign83220_e124879_d_n14, assign83220_e124879_d_n15, assign83220_e124879_d_n16, assign83220_e124879_d_n17, assign83220_e124879_d_n18, assign83220_e124879_d_n19, assign83220_e124879_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2281 != 0.0)) {
        let assign83220_e124876: f64 = (locals.var_nt * locals.var_mid);
        let assign83220_e124877: f64 = (assign83220_e124876).sqrt();
        (assign83220_e124877, ((locals.var_nt * locals.var_mid_dn5) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn6) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn7) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn8) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn12) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn13) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn14) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn15) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn16) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn17) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn18) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn19) / (2.0 * assign83220_e124877)), ((locals.var_nt * locals.var_mid_dn20) / (2.0 * assign83220_e124877)),)
    } else {
        (locals.var_sqid, locals.var_sqid_dn5, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8, locals.var_sqid_dn12, locals.var_sqid_dn13, locals.var_sqid_dn14, locals.var_sqid_dn15, locals.var_sqid_dn16, locals.var_sqid_dn17, locals.var_sqid_dn18, locals.var_sqid_dn19, locals.var_sqid_dn20,)
    }
};
        locals.var_sqid = assign83220_e124879;
        locals.var_sqid_dn5 = assign83220_e124879_d_n5;
        locals.var_sqid_dn6 = assign83220_e124879_d_n6;
        locals.var_sqid_dn7 = assign83220_e124879_d_n7;
        locals.var_sqid_dn8 = assign83220_e124879_d_n8;
        locals.var_sqid_dn12 = assign83220_e124879_d_n12;
        locals.var_sqid_dn13 = assign83220_e124879_d_n13;
        locals.var_sqid_dn14 = assign83220_e124879_d_n14;
        locals.var_sqid_dn15 = assign83220_e124879_d_n15;
        locals.var_sqid_dn16 = assign83220_e124879_d_n16;
        locals.var_sqid_dn17 = assign83220_e124879_d_n17;
        locals.var_sqid_dn18 = assign83220_e124879_d_n18;
        locals.var_sqid_dn19 = assign83220_e124879_d_n19;
        locals.var_sqid_dn20 = assign83220_e124879_d_n20;

        let assign83230_e124894: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign83230_e124894;

        let (assign83240_e124926, assign83240_e124926_d_n5, assign83240_e124926_d_n6, assign83240_e124926_d_n7, assign83240_e124926_d_n8, assign83240_e124926_d_n12, assign83240_e124926_d_n13, assign83240_e124926_d_n14, assign83240_e124926_d_n15, assign83240_e124926_d_n16, assign83240_e124926_d_n17, assign83240_e124926_d_n18, assign83240_e124926_d_n19, assign83240_e124926_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83240_e124900: f64 = (locals.var_t1 / 12.0);
        let assign83240_e124904: f64 = (locals.var_t1 + 0.2);
        let assign83240_e124907: f64 = (12.0 * locals.var_t2);
        let assign83240_e124908: f64 = (assign83240_e124904 - assign83240_e124907);
        let assign83240_e124909: f64 = (locals.var_t2 * assign83240_e124908);
        let assign83240_e124910: f64 = (assign83240_e124900 - assign83240_e124909);
        let assign83240_e124915: f64 = (locals.var_t1 + 1.0);
        let assign83240_e124918: f64 = (12.0 * locals.var_t2);
        let assign83240_e124919: f64 = (assign83240_e124915 - assign83240_e124918);
        let assign83240_e124920: f64 = (locals.var_t2 * assign83240_e124919);
        let assign83240_e124922: f64 = (assign83240_e124920 * locals.var_r);
        let assign83240_e124923: f64 = (1.6 * assign83240_e124922);
        let assign83240_e124924: f64 = (assign83240_e124910 - assign83240_e124923);
        (assign83240_e124924, (((locals.var_t1_dn5 / 12.0) - ((locals.var_t2_dn5 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5))))) - (1.6 * ((((locals.var_t2_dn5 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn5)))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6))))) - (1.6 * ((((locals.var_t2_dn6 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn6)))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7))))) - (1.6 * ((((locals.var_t2_dn7 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn7)))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8))))) - (1.6 * ((((locals.var_t2_dn8 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn8)))), (((locals.var_t1_dn12 / 12.0) - ((locals.var_t2_dn12 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn12 - (12.0 * locals.var_t2_dn12))))) - (1.6 * ((((locals.var_t2_dn12 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn12 - (12.0 * locals.var_t2_dn12)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn12)))), (((locals.var_t1_dn13 / 12.0) - ((locals.var_t2_dn13 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn13 - (12.0 * locals.var_t2_dn13))))) - (1.6 * ((((locals.var_t2_dn13 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn13 - (12.0 * locals.var_t2_dn13)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn13)))), (((locals.var_t1_dn14 / 12.0) - ((locals.var_t2_dn14 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn14 - (12.0 * locals.var_t2_dn14))))) - (1.6 * ((((locals.var_t2_dn14 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn14 - (12.0 * locals.var_t2_dn14)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn14)))), (((locals.var_t1_dn15 / 12.0) - ((locals.var_t2_dn15 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn15 - (12.0 * locals.var_t2_dn15))))) - (1.6 * ((((locals.var_t2_dn15 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn15 - (12.0 * locals.var_t2_dn15)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn15)))), (((locals.var_t1_dn16 / 12.0) - ((locals.var_t2_dn16 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn16 - (12.0 * locals.var_t2_dn16))))) - (1.6 * ((((locals.var_t2_dn16 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn16 - (12.0 * locals.var_t2_dn16)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn16)))), (((locals.var_t1_dn17 / 12.0) - ((locals.var_t2_dn17 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn17 - (12.0 * locals.var_t2_dn17))))) - (1.6 * ((((locals.var_t2_dn17 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn17 - (12.0 * locals.var_t2_dn17)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn17)))), (((locals.var_t1_dn18 / 12.0) - ((locals.var_t2_dn18 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn18 - (12.0 * locals.var_t2_dn18))))) - (1.6 * ((((locals.var_t2_dn18 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn18 - (12.0 * locals.var_t2_dn18)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn18)))), (((locals.var_t1_dn19 / 12.0) - ((locals.var_t2_dn19 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn19 - (12.0 * locals.var_t2_dn19))))) - (1.6 * ((((locals.var_t2_dn19 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn19 - (12.0 * locals.var_t2_dn19)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn19)))), (((locals.var_t1_dn20 / 12.0) - ((locals.var_t2_dn20 * assign83240_e124908) + (locals.var_t2 * (locals.var_t1_dn20 - (12.0 * locals.var_t2_dn20))))) - (1.6 * ((((locals.var_t2_dn20 * assign83240_e124919) + (locals.var_t2 * (locals.var_t1_dn20 - (12.0 * locals.var_t2_dn20)))) * locals.var_r) + (assign83240_e124920 * locals.var_r_dn20)))),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn12, locals.var_mig_dn13, locals.var_mig_dn14, locals.var_mig_dn15, locals.var_mig_dn16, locals.var_mig_dn17, locals.var_mig_dn18, locals.var_mig_dn19, locals.var_mig_dn20,)
    }
};
        locals.var_mig = assign83240_e124926;
        locals.var_mig_dn5 = assign83240_e124926_d_n5;
        locals.var_mig_dn6 = assign83240_e124926_d_n6;
        locals.var_mig_dn7 = assign83240_e124926_d_n7;
        locals.var_mig_dn8 = assign83240_e124926_d_n8;
        locals.var_mig_dn12 = assign83240_e124926_d_n12;
        locals.var_mig_dn13 = assign83240_e124926_d_n13;
        locals.var_mig_dn14 = assign83240_e124926_d_n14;
        locals.var_mig_dn15 = assign83240_e124926_d_n15;
        locals.var_mig_dn16 = assign83240_e124926_d_n16;
        locals.var_mig_dn17 = assign83240_e124926_d_n17;
        locals.var_mig_dn18 = assign83240_e124926_d_n18;
        locals.var_mig_dn19 = assign83240_e124926_d_n19;
        locals.var_mig_dn20 = assign83240_e124926_d_n20;

        let (assign83250_e124937, assign83250_e124937_d_n5, assign83250_e124937_d_n6, assign83250_e124937_d_n7, assign83250_e124937_d_n8, assign83250_e124937_d_n12, assign83250_e124937_d_n13, assign83250_e124937_d_n14, assign83250_e124937_d_n15, assign83250_e124937_d_n16, assign83250_e124937_d_n17, assign83250_e124937_d_n18, assign83250_e124937_d_n19, assign83250_e124937_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let (assign83250_e124935, assign83250_e124935_d_n5, assign83250_e124935_d_n6, assign83250_e124935_d_n7, assign83250_e124935_d_n8, assign83250_e124935_d_n12, assign83250_e124935_d_n13, assign83250_e124935_d_n14, assign83250_e124935_d_n15, assign83250_e124935_d_n16, assign83250_e124935_d_n17, assign83250_e124935_d_n18, assign83250_e124935_d_n19, assign83250_e124935_d_n20,) = {
            if (locals.var_mig > 1e-40) {
                (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn12, locals.var_mig_dn13, locals.var_mig_dn14, locals.var_mig_dn15, locals.var_mig_dn16, locals.var_mig_dn17, locals.var_mig_dn18, locals.var_mig_dn19, locals.var_mig_dn20,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83250_e124935, assign83250_e124935_d_n5, assign83250_e124935_d_n6, assign83250_e124935_d_n7, assign83250_e124935_d_n8, assign83250_e124935_d_n12, assign83250_e124935_d_n13, assign83250_e124935_d_n14, assign83250_e124935_d_n15, assign83250_e124935_d_n16, assign83250_e124935_d_n17, assign83250_e124935_d_n18, assign83250_e124935_d_n19, assign83250_e124935_d_n20,)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn12, locals.var_mig_dn13, locals.var_mig_dn14, locals.var_mig_dn15, locals.var_mig_dn16, locals.var_mig_dn17, locals.var_mig_dn18, locals.var_mig_dn19, locals.var_mig_dn20,)
    }
};
        locals.var_mig = assign83250_e124937;
        locals.var_mig_dn5 = assign83250_e124937_d_n5;
        locals.var_mig_dn6 = assign83250_e124937_d_n6;
        locals.var_mig_dn7 = assign83250_e124937_d_n7;
        locals.var_mig_dn8 = assign83250_e124937_d_n8;
        locals.var_mig_dn12 = assign83250_e124937_d_n12;
        locals.var_mig_dn13 = assign83250_e124937_d_n13;
        locals.var_mig_dn14 = assign83250_e124937_d_n14;
        locals.var_mig_dn15 = assign83250_e124937_d_n15;
        locals.var_mig_dn16 = assign83250_e124937_d_n16;
        locals.var_mig_dn17 = assign83250_e124937_d_n17;
        locals.var_mig_dn18 = assign83250_e124937_d_n18;
        locals.var_mig_dn19 = assign83250_e124937_d_n19;
        locals.var_mig_dn20 = assign83250_e124937_d_n20;

        let (assign83260_e124947, assign83260_e124947_d_n5, assign83260_e124947_d_n6, assign83260_e124947_d_n7, assign83260_e124947_d_n8, assign83260_e124947_d_n12, assign83260_e124947_d_n13, assign83260_e124947_d_n14, assign83260_e124947_d_n15, assign83260_e124947_d_n16, assign83260_e124947_d_n17, assign83260_e124947_d_n18, assign83260_e124947_d_n19, assign83260_e124947_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83260_e124943: f64 = (locals.var_lcinv2 / locals.var_g_ideal);
        let assign83260_e124945: f64 = (assign83260_e124943 * locals.var_mig);
        (assign83260_e124945, (((((locals.var_lcinv2_dn5 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn5)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn5)), (((((locals.var_lcinv2_dn6 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn6)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn6)), (((((locals.var_lcinv2_dn7 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn7)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn7)), (((((locals.var_lcinv2_dn8 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn8)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn8)), (((((locals.var_lcinv2_dn12 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn12)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn12)), (((((locals.var_lcinv2_dn13 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn13)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn13)), (((((locals.var_lcinv2_dn14 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn14)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn14)), (((((locals.var_lcinv2_dn15 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn15)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn15)), (((((locals.var_lcinv2_dn16 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn16)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn16)), (((((locals.var_lcinv2_dn17 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn17)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn17)), (((((locals.var_lcinv2_dn18 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn18)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn18)), (((((locals.var_lcinv2_dn19 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn19)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn19)), (((((locals.var_lcinv2_dn20 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn20)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign83260_e124943 * locals.var_mig_dn20)),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn12, locals.var_mig_dn13, locals.var_mig_dn14, locals.var_mig_dn15, locals.var_mig_dn16, locals.var_mig_dn17, locals.var_mig_dn18, locals.var_mig_dn19, locals.var_mig_dn20,)
    }
};
        locals.var_mig = assign83260_e124947;
        locals.var_mig_dn5 = assign83260_e124947_d_n5;
        locals.var_mig_dn6 = assign83260_e124947_d_n6;
        locals.var_mig_dn7 = assign83260_e124947_d_n7;
        locals.var_mig_dn8 = assign83260_e124947_d_n8;
        locals.var_mig_dn12 = assign83260_e124947_d_n12;
        locals.var_mig_dn13 = assign83260_e124947_d_n13;
        locals.var_mig_dn14 = assign83260_e124947_d_n14;
        locals.var_mig_dn15 = assign83260_e124947_d_n15;
        locals.var_mig_dn16 = assign83260_e124947_d_n16;
        locals.var_mig_dn17 = assign83260_e124947_d_n17;
        locals.var_mig_dn18 = assign83260_e124947_d_n18;
        locals.var_mig_dn19 = assign83260_e124947_d_n19;
        locals.var_mig_dn20 = assign83260_e124947_d_n20;

        let (assign83270_e124975, assign83270_e124975_d_n5, assign83270_e124975_d_n6, assign83270_e124975_d_n7, assign83270_e124975_d_n8, assign83270_e124975_d_n12, assign83270_e124975_d_n13, assign83270_e124975_d_n14, assign83270_e124975_d_n15, assign83270_e124975_d_n16, assign83270_e124975_d_n17, assign83270_e124975_d_n18, assign83270_e124975_d_n19, assign83270_e124975_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83270_e124953: f64 = (locals.var_lcinv2 * locals.var_sqt2);
        let assign83270_e124957: f64 = (12.0 * locals.var_t2);
        let assign83270_e124958: f64 = (1.0 - assign83270_e124957);
        let assign83270_e124962: f64 = (19.2 * locals.var_t2);
        let assign83270_e124963: f64 = (locals.var_t1 + assign83270_e124962);
        let assign83270_e124967: f64 = (locals.var_t1 * locals.var_t2);
        let assign83270_e124968: f64 = (12.0 * assign83270_e124967);
        let assign83270_e124969: f64 = (assign83270_e124963 - assign83270_e124968);
        let assign83270_e124971: f64 = (assign83270_e124969 * locals.var_r);
        let assign83270_e124972: f64 = (assign83270_e124958 - assign83270_e124971);
        let assign83270_e124973: f64 = (assign83270_e124953 * assign83270_e124972);
        (assign83270_e124973, ((((locals.var_lcinv2_dn5 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn5)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn5)) - ((((locals.var_t1_dn5 + (19.2 * locals.var_t2_dn5)) - (12.0 * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn5))))), ((((locals.var_lcinv2_dn6 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn6)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn6)) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - (12.0 * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn7)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn7)) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - (12.0 * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn8)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn8)) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - (12.0 * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn8))))), ((((locals.var_lcinv2_dn12 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn12)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn12)) - ((((locals.var_t1_dn12 + (19.2 * locals.var_t2_dn12)) - (12.0 * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn12))))), ((((locals.var_lcinv2_dn13 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn13)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn13)) - ((((locals.var_t1_dn13 + (19.2 * locals.var_t2_dn13)) - (12.0 * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn13))))), ((((locals.var_lcinv2_dn14 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn14)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn14)) - ((((locals.var_t1_dn14 + (19.2 * locals.var_t2_dn14)) - (12.0 * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn14))))), ((((locals.var_lcinv2_dn15 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn15)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn15)) - ((((locals.var_t1_dn15 + (19.2 * locals.var_t2_dn15)) - (12.0 * ((locals.var_t1_dn15 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn15)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn15))))), ((((locals.var_lcinv2_dn16 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn16)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn16)) - ((((locals.var_t1_dn16 + (19.2 * locals.var_t2_dn16)) - (12.0 * ((locals.var_t1_dn16 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn16)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn16))))), ((((locals.var_lcinv2_dn17 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn17)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn17)) - ((((locals.var_t1_dn17 + (19.2 * locals.var_t2_dn17)) - (12.0 * ((locals.var_t1_dn17 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn17)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn17))))), ((((locals.var_lcinv2_dn18 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn18)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn18)) - ((((locals.var_t1_dn18 + (19.2 * locals.var_t2_dn18)) - (12.0 * ((locals.var_t1_dn18 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn18)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn18))))), ((((locals.var_lcinv2_dn19 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn19)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn19)) - ((((locals.var_t1_dn19 + (19.2 * locals.var_t2_dn19)) - (12.0 * ((locals.var_t1_dn19 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn19)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn19))))), ((((locals.var_lcinv2_dn20 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn20)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * locals.var_t2_dn20)) - ((((locals.var_t1_dn20 + (19.2 * locals.var_t2_dn20)) - (12.0 * ((locals.var_t1_dn20 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn20)))) * locals.var_r) + (assign83270_e124969 * locals.var_r_dn20))))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, locals.var_migid0_dn12, locals.var_migid0_dn13, locals.var_migid0_dn14, locals.var_migid0_dn15, locals.var_migid0_dn16, locals.var_migid0_dn17, locals.var_migid0_dn18, locals.var_migid0_dn19, locals.var_migid0_dn20,)
    }
};
        locals.var_migid0 = assign83270_e124975;
        locals.var_migid0_dn5 = assign83270_e124975_d_n5;
        locals.var_migid0_dn6 = assign83270_e124975_d_n6;
        locals.var_migid0_dn7 = assign83270_e124975_d_n7;
        locals.var_migid0_dn8 = assign83270_e124975_d_n8;
        locals.var_migid0_dn12 = assign83270_e124975_d_n12;
        locals.var_migid0_dn13 = assign83270_e124975_d_n13;
        locals.var_migid0_dn14 = assign83270_e124975_d_n14;
        locals.var_migid0_dn15 = assign83270_e124975_d_n15;
        locals.var_migid0_dn16 = assign83270_e124975_d_n16;
        locals.var_migid0_dn17 = assign83270_e124975_d_n17;
        locals.var_migid0_dn18 = assign83270_e124975_d_n18;
        locals.var_migid0_dn19 = assign83270_e124975_d_n19;
        locals.var_migid0_dn20 = assign83270_e124975_d_n20;

        let (assign83280_e124991, assign83280_e124991_d_n5, assign83280_e124991_d_n6, assign83280_e124991_d_n7, assign83280_e124991_d_n8, assign83280_e124991_d_n12, assign83280_e124991_d_n13, assign83280_e124991_d_n14, assign83280_e124991_d_n15, assign83280_e124991_d_n16, assign83280_e124991_d_n17, assign83280_e124991_d_n18, assign83280_e124991_d_n19, assign83280_e124991_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83280_e124981: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign83280_e124983: f64 = (assign83280_e124981 * locals.var_cox_qm);
        let assign83280_e124985: f64 = (assign83280_e124983 * locals.var_eta_p_ac);
        let assign83280_e124988: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign83280_e124989: f64 = (assign83280_e124985 / assign83280_e124988);
        (assign83280_e124989, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn5)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn6)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn7)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn8)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn12 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn12)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn12)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn12)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn12 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn12)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn13 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn13)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn13)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn13)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn13 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn13)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn14 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn14)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn14)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn14)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn14 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn14)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn15 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn15)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn15)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn15)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn15 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn15)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn16 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn16)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn16)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn16)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn16 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn16)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn17 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn17)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn17)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn17)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn17 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn17)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn18 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn18)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn18)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn18)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn18 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn18)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn19 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn19)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn19)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn19)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn19 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn19)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn20 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn20)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn20)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn20)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn20 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn20)))) / (assign83280_e124988 * assign83280_e124988)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn12, locals.var_cgeff_dn13, locals.var_cgeff_dn14, locals.var_cgeff_dn15, locals.var_cgeff_dn16, locals.var_cgeff_dn17, locals.var_cgeff_dn18, locals.var_cgeff_dn19, locals.var_cgeff_dn20,)
    }
};
        locals.var_cgeff = assign83280_e124991;
        locals.var_cgeff_dn5 = assign83280_e124991_d_n5;
        locals.var_cgeff_dn6 = assign83280_e124991_d_n6;
        locals.var_cgeff_dn7 = assign83280_e124991_d_n7;
        locals.var_cgeff_dn8 = assign83280_e124991_d_n8;
        locals.var_cgeff_dn12 = assign83280_e124991_d_n12;
        locals.var_cgeff_dn13 = assign83280_e124991_d_n13;
        locals.var_cgeff_dn14 = assign83280_e124991_d_n14;
        locals.var_cgeff_dn15 = assign83280_e124991_d_n15;
        locals.var_cgeff_dn16 = assign83280_e124991_d_n16;
        locals.var_cgeff_dn17 = assign83280_e124991_d_n17;
        locals.var_cgeff_dn18 = assign83280_e124991_d_n18;
        locals.var_cgeff_dn19 = assign83280_e124991_d_n19;
        locals.var_cgeff_dn20 = assign83280_e124991_d_n20;

        let assign83290_e124994: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign83290_e124994;

        let (assign83300_e125018, assign83300_e125018_d_n5, assign83300_e125018_d_n6, assign83300_e125018_d_n7, assign83300_e125018_d_n8, assign83300_e125018_d_n12, assign83300_e125018_d_n13, assign83300_e125018_d_n14, assign83300_e125018_d_n15, assign83300_e125018_d_n16, assign83300_e125018_d_n17, assign83300_e125018_d_n18, assign83300_e125018_d_n19, assign83300_e125018_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign83300_e125005: f64 = (12.0 * locals.var_t2);
        let assign83300_e125006: f64 = (1.0 + assign83300_e125005);
        let assign83300_e125007: f64 = (locals.var_sidexc * assign83300_e125006);
        let assign83300_e125010: f64 = (12.0 * locals.var_g_ideal);
        let assign83300_e125012: f64 = (assign83300_e125010 * locals.var_g_ideal);
        let assign83300_e125014: f64 = (assign83300_e125012 * locals.var_nt0);
        let assign83300_e125015: f64 = (assign83300_e125007 / assign83300_e125014);
        let assign83300_e125016: f64 = (locals.var_mig + assign83300_e125015);
        (assign83300_e125016, (locals.var_mig_dn5 + (((((locals.var_sidexc_dn5 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn5))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn5) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn5)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn6 + (((((locals.var_sidexc_dn6 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn6))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn6) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn6)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn7 + (((((locals.var_sidexc_dn7 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn7))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn7) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn7)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn8 + (((((locals.var_sidexc_dn8 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn8))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn8) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn8)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn12 + (((((locals.var_sidexc_dn12 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn12))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn12) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn12)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn13 + (((((locals.var_sidexc_dn13 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn13))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn13) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn13)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn14 + (((((locals.var_sidexc_dn14 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn14))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn14) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn14)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn15 + (((((locals.var_sidexc_dn15 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn15))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn15) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn15)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn16 + (((((locals.var_sidexc_dn16 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn16))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn16) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn16)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn17 + (((((locals.var_sidexc_dn17 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn17))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn17) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn17)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn18 + (((((locals.var_sidexc_dn18 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn18))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn18) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn18)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn19 + (((((locals.var_sidexc_dn19 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn19))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn19) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn19)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (locals.var_mig_dn20 + (((((locals.var_sidexc_dn20 * assign83300_e125006) + (locals.var_sidexc * (12.0 * locals.var_t2_dn20))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * locals.var_g_ideal_dn20) * locals.var_g_ideal) + (assign83300_e125010 * locals.var_g_ideal_dn20)) * locals.var_nt0))) / (assign83300_e125014 * assign83300_e125014))),)
    } else {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, locals.var_mig_dn12, locals.var_mig_dn13, locals.var_mig_dn14, locals.var_mig_dn15, locals.var_mig_dn16, locals.var_mig_dn17, locals.var_mig_dn18, locals.var_mig_dn19, locals.var_mig_dn20,)
    }
};
        locals.var_mig = assign83300_e125018;
        locals.var_mig_dn5 = assign83300_e125018_d_n5;
        locals.var_mig_dn6 = assign83300_e125018_d_n6;
        locals.var_mig_dn7 = assign83300_e125018_d_n7;
        locals.var_mig_dn8 = assign83300_e125018_d_n8;
        locals.var_mig_dn12 = assign83300_e125018_d_n12;
        locals.var_mig_dn13 = assign83300_e125018_d_n13;
        locals.var_mig_dn14 = assign83300_e125018_d_n14;
        locals.var_mig_dn15 = assign83300_e125018_d_n15;
        locals.var_mig_dn16 = assign83300_e125018_d_n16;
        locals.var_mig_dn17 = assign83300_e125018_d_n17;
        locals.var_mig_dn18 = assign83300_e125018_d_n18;
        locals.var_mig_dn19 = assign83300_e125018_d_n19;
        locals.var_mig_dn20 = assign83300_e125018_d_n20;

        let (assign83310_e125038, assign83310_e125038_d_n5, assign83310_e125038_d_n6, assign83310_e125038_d_n7, assign83310_e125038_d_n8, assign83310_e125038_d_n12, assign83310_e125038_d_n13, assign83310_e125038_d_n14, assign83310_e125038_d_n15, assign83310_e125038_d_n16, assign83310_e125038_d_n17, assign83310_e125038_d_n18, assign83310_e125038_d_n19, assign83310_e125038_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign83310_e125027: f64 = (locals.var_sidexc * locals.var_sqt2);
        let assign83310_e125030: f64 = (1.0 + locals.var_r);
        let assign83310_e125031: f64 = (assign83310_e125027 * assign83310_e125030);
        let assign83310_e125034: f64 = (locals.var_g_ideal * locals.var_nt0);
        let assign83310_e125035: f64 = (assign83310_e125031 / assign83310_e125034);
        let assign83310_e125036: f64 = (locals.var_migid0 - assign83310_e125035);
        (assign83310_e125036, (locals.var_migid0_dn5 - (((((((locals.var_sidexc_dn5 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn5)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn5)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn5 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn6 - (((((((locals.var_sidexc_dn6 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn6)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn6)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn6 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn7 - (((((((locals.var_sidexc_dn7 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn7)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn7)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn7 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn8 - (((((((locals.var_sidexc_dn8 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn8)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn8)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn8 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn12 - (((((((locals.var_sidexc_dn12 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn12)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn12)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn12 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn13 - (((((((locals.var_sidexc_dn13 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn13)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn13)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn13 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn14 - (((((((locals.var_sidexc_dn14 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn14)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn14)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn14 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn15 - (((((((locals.var_sidexc_dn15 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn15)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn15)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn15 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn16 - (((((((locals.var_sidexc_dn16 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn16)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn16)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn16 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn17 - (((((((locals.var_sidexc_dn17 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn17)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn17)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn17 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn18 - (((((((locals.var_sidexc_dn18 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn18)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn18)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn18 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn19 - (((((((locals.var_sidexc_dn19 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn19)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn19)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn19 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (locals.var_migid0_dn20 - (((((((locals.var_sidexc_dn20 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn20)) * assign83310_e125030) + (assign83310_e125027 * locals.var_r_dn20)) * assign83310_e125034) - (assign83310_e125031 * (locals.var_g_ideal_dn20 * locals.var_nt0))) / (assign83310_e125034 * assign83310_e125034))),)
    } else {
        (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, locals.var_migid0_dn12, locals.var_migid0_dn13, locals.var_migid0_dn14, locals.var_migid0_dn15, locals.var_migid0_dn16, locals.var_migid0_dn17, locals.var_migid0_dn18, locals.var_migid0_dn19, locals.var_migid0_dn20,)
    }
};
        locals.var_migid0 = assign83310_e125038;
        locals.var_migid0_dn5 = assign83310_e125038_d_n5;
        locals.var_migid0_dn6 = assign83310_e125038_d_n6;
        locals.var_migid0_dn7 = assign83310_e125038_d_n7;
        locals.var_migid0_dn8 = assign83310_e125038_d_n8;
        locals.var_migid0_dn12 = assign83310_e125038_d_n12;
        locals.var_migid0_dn13 = assign83310_e125038_d_n13;
        locals.var_migid0_dn14 = assign83310_e125038_d_n14;
        locals.var_migid0_dn15 = assign83310_e125038_d_n15;
        locals.var_migid0_dn16 = assign83310_e125038_d_n16;
        locals.var_migid0_dn17 = assign83310_e125038_d_n17;
        locals.var_migid0_dn18 = assign83310_e125038_d_n18;
        locals.var_migid0_dn19 = assign83310_e125038_d_n19;
        locals.var_migid0_dn20 = assign83310_e125038_d_n20;

        let (assign83320_e125047, assign83320_e125047_d_n5, assign83320_e125047_d_n6, assign83320_e125047_d_n7, assign83320_e125047_d_n8, assign83320_e125047_d_n12, assign83320_e125047_d_n13, assign83320_e125047_d_n14, assign83320_e125047_d_n15, assign83320_e125047_d_n16, assign83320_e125047_d_n17, assign83320_e125047_d_n18, assign83320_e125047_d_n19, assign83320_e125047_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83320_e125044: f64 = (locals.var_nt / locals.var_mig);
        let assign83320_e125045: f64 = (assign83320_e125044).sqrt();
        (assign83320_e125045, ((-((locals.var_nt * locals.var_mig_dn5) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn12) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn13) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn14) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn15) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn16) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn17) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn18) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn19) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)), ((-((locals.var_nt * locals.var_mig_dn20) / (locals.var_mig * locals.var_mig))) / (2.0 * assign83320_e125045)),)
    } else {
        (locals.var_sqig, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, locals.var_sqig_dn12, locals.var_sqig_dn13, locals.var_sqig_dn14, locals.var_sqig_dn15, locals.var_sqig_dn16, locals.var_sqig_dn17, locals.var_sqig_dn18, locals.var_sqig_dn19, locals.var_sqig_dn20,)
    }
};
        locals.var_sqig = assign83320_e125047;
        locals.var_sqig_dn5 = assign83320_e125047_d_n5;
        locals.var_sqig_dn6 = assign83320_e125047_d_n6;
        locals.var_sqig_dn7 = assign83320_e125047_d_n7;
        locals.var_sqig_dn8 = assign83320_e125047_d_n8;
        locals.var_sqig_dn12 = assign83320_e125047_d_n12;
        locals.var_sqig_dn13 = assign83320_e125047_d_n13;
        locals.var_sqig_dn14 = assign83320_e125047_d_n14;
        locals.var_sqig_dn15 = assign83320_e125047_d_n15;
        locals.var_sqig_dn16 = assign83320_e125047_d_n16;
        locals.var_sqig_dn17 = assign83320_e125047_d_n17;
        locals.var_sqig_dn18 = assign83320_e125047_d_n18;
        locals.var_sqig_dn19 = assign83320_e125047_d_n19;
        locals.var_sqig_dn20 = assign83320_e125047_d_n20;

        let assign83330_e125050: f64 = if locals.var_sqid <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign83330_e125050;

        let (assign83340_e125058, assign83340_e125058_d_n5, assign83340_e125058_d_n6, assign83340_e125058_d_n7, assign83340_e125058_d_n8, assign83340_e125058_d_n12, assign83340_e125058_d_n13, assign83340_e125058_d_n14, assign83340_e125058_d_n15, assign83340_e125058_d_n16, assign83340_e125058_d_n17, assign83340_e125058_d_n18, assign83340_e125058_d_n19, assign83340_e125058_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2286 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn12, locals.var_c_igid_dn13, locals.var_c_igid_dn14, locals.var_c_igid_dn15, locals.var_c_igid_dn16, locals.var_c_igid_dn17, locals.var_c_igid_dn18, locals.var_c_igid_dn19, locals.var_c_igid_dn20,)
    }
};
        locals.var_c_igid = assign83340_e125058;
        locals.var_c_igid_dn5 = assign83340_e125058_d_n5;
        locals.var_c_igid_dn6 = assign83340_e125058_d_n6;
        locals.var_c_igid_dn7 = assign83340_e125058_d_n7;
        locals.var_c_igid_dn8 = assign83340_e125058_d_n8;
        locals.var_c_igid_dn12 = assign83340_e125058_d_n12;
        locals.var_c_igid_dn13 = assign83340_e125058_d_n13;
        locals.var_c_igid_dn14 = assign83340_e125058_d_n14;
        locals.var_c_igid_dn15 = assign83340_e125058_d_n15;
        locals.var_c_igid_dn16 = assign83340_e125058_d_n16;
        locals.var_c_igid_dn17 = assign83340_e125058_d_n17;
        locals.var_c_igid_dn18 = assign83340_e125058_d_n18;
        locals.var_c_igid_dn19 = assign83340_e125058_d_n19;
        locals.var_c_igid_dn20 = assign83340_e125058_d_n20;

        let (assign83350_e125071, assign83350_e125071_d_n5, assign83350_e125071_d_n6, assign83350_e125071_d_n7, assign83350_e125071_d_n8, assign83350_e125071_d_n12, assign83350_e125071_d_n13, assign83350_e125071_d_n14, assign83350_e125071_d_n15, assign83350_e125071_d_n16, assign83350_e125071_d_n17, assign83350_e125071_d_n18, assign83350_e125071_d_n19, assign83350_e125071_d_n20,) = {
    if (((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        let assign83350_e125067: f64 = (locals.var_migid0 * locals.var_sqig);
        let assign83350_e125069: f64 = (assign83350_e125067 / locals.var_sqid);
        (assign83350_e125069, (((((locals.var_migid0_dn5 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn5)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn5)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn6 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn6)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn6)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn7 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn7)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn7)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn8 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn8)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn8)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn12 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn12)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn12)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn13 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn13)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn13)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn14 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn14)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn14)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn15 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn15)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn15)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn16 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn16)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn16)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn17 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn17)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn17)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn18 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn18)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn18)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn19 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn19)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn19)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn20 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn20)) * locals.var_sqid) - (assign83350_e125067 * locals.var_sqid_dn20)) / (locals.var_sqid * locals.var_sqid)),)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn12, locals.var_c_igid_dn13, locals.var_c_igid_dn14, locals.var_c_igid_dn15, locals.var_c_igid_dn16, locals.var_c_igid_dn17, locals.var_c_igid_dn18, locals.var_c_igid_dn19, locals.var_c_igid_dn20,)
    }
};
        locals.var_c_igid = assign83350_e125071;
        locals.var_c_igid_dn5 = assign83350_e125071_d_n5;
        locals.var_c_igid_dn6 = assign83350_e125071_d_n6;
        locals.var_c_igid_dn7 = assign83350_e125071_d_n7;
        locals.var_c_igid_dn8 = assign83350_e125071_d_n8;
        locals.var_c_igid_dn12 = assign83350_e125071_d_n12;
        locals.var_c_igid_dn13 = assign83350_e125071_d_n13;
        locals.var_c_igid_dn14 = assign83350_e125071_d_n14;
        locals.var_c_igid_dn15 = assign83350_e125071_d_n15;
        locals.var_c_igid_dn16 = assign83350_e125071_d_n16;
        locals.var_c_igid_dn17 = assign83350_e125071_d_n17;
        locals.var_c_igid_dn18 = assign83350_e125071_d_n18;
        locals.var_c_igid_dn19 = assign83350_e125071_d_n19;
        locals.var_c_igid_dn20 = assign83350_e125071_d_n20;

        let (assign83360_e125087, assign83360_e125087_d_n5, assign83360_e125087_d_n6, assign83360_e125087_d_n7, assign83360_e125087_d_n8, assign83360_e125087_d_n12, assign83360_e125087_d_n13, assign83360_e125087_d_n14, assign83360_e125087_d_n15, assign83360_e125087_d_n16, assign83360_e125087_d_n17, assign83360_e125087_d_n18, assign83360_e125087_d_n19, assign83360_e125087_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let (assign83360_e125085, assign83360_e125085_d_n5, assign83360_e125085_d_n6, assign83360_e125085_d_n7, assign83360_e125085_d_n8, assign83360_e125085_d_n12, assign83360_e125085_d_n13, assign83360_e125085_d_n14, assign83360_e125085_d_n15, assign83360_e125085_d_n16, assign83360_e125085_d_n17, assign83360_e125085_d_n18, assign83360_e125085_d_n19, assign83360_e125085_d_n20,) = {
            if (locals.var_c_igid > 0.0) {
                let (assign83360_e125083, assign83360_e125083_d_n5, assign83360_e125083_d_n6, assign83360_e125083_d_n7, assign83360_e125083_d_n8, assign83360_e125083_d_n12, assign83360_e125083_d_n13, assign83360_e125083_d_n14, assign83360_e125083_d_n15, assign83360_e125083_d_n16, assign83360_e125083_d_n17, assign83360_e125083_d_n18, assign83360_e125083_d_n19, assign83360_e125083_d_n20,) = {
                    if (locals.var_c_igid < 1.0) {
                        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn12, locals.var_c_igid_dn13, locals.var_c_igid_dn14, locals.var_c_igid_dn15, locals.var_c_igid_dn16, locals.var_c_igid_dn17, locals.var_c_igid_dn18, locals.var_c_igid_dn19, locals.var_c_igid_dn20,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign83360_e125083, assign83360_e125083_d_n5, assign83360_e125083_d_n6, assign83360_e125083_d_n7, assign83360_e125083_d_n8, assign83360_e125083_d_n12, assign83360_e125083_d_n13, assign83360_e125083_d_n14, assign83360_e125083_d_n15, assign83360_e125083_d_n16, assign83360_e125083_d_n17, assign83360_e125083_d_n18, assign83360_e125083_d_n19, assign83360_e125083_d_n20,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83360_e125085, assign83360_e125085_d_n5, assign83360_e125085_d_n6, assign83360_e125085_d_n7, assign83360_e125085_d_n8, assign83360_e125085_d_n12, assign83360_e125085_d_n13, assign83360_e125085_d_n14, assign83360_e125085_d_n15, assign83360_e125085_d_n16, assign83360_e125085_d_n17, assign83360_e125085_d_n18, assign83360_e125085_d_n19, assign83360_e125085_d_n20,)
    } else {
        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, locals.var_c_igid_dn12, locals.var_c_igid_dn13, locals.var_c_igid_dn14, locals.var_c_igid_dn15, locals.var_c_igid_dn16, locals.var_c_igid_dn17, locals.var_c_igid_dn18, locals.var_c_igid_dn19, locals.var_c_igid_dn20,)
    }
};
        locals.var_c_igid = assign83360_e125087;
        locals.var_c_igid_dn5 = assign83360_e125087_d_n5;
        locals.var_c_igid_dn6 = assign83360_e125087_d_n6;
        locals.var_c_igid_dn7 = assign83360_e125087_d_n7;
        locals.var_c_igid_dn8 = assign83360_e125087_d_n8;
        locals.var_c_igid_dn12 = assign83360_e125087_d_n12;
        locals.var_c_igid_dn13 = assign83360_e125087_d_n13;
        locals.var_c_igid_dn14 = assign83360_e125087_d_n14;
        locals.var_c_igid_dn15 = assign83360_e125087_d_n15;
        locals.var_c_igid_dn16 = assign83360_e125087_d_n16;
        locals.var_c_igid_dn17 = assign83360_e125087_d_n17;
        locals.var_c_igid_dn18 = assign83360_e125087_d_n18;
        locals.var_c_igid_dn19 = assign83360_e125087_d_n19;
        locals.var_c_igid_dn20 = assign83360_e125087_d_n20;

        let (assign83370_e125097, assign83370_e125097_d_n5, assign83370_e125097_d_n6, assign83370_e125097_d_n7, assign83370_e125097_d_n8, assign83370_e125097_d_n12, assign83370_e125097_d_n13, assign83370_e125097_d_n14, assign83370_e125097_d_n15, assign83370_e125097_d_n16, assign83370_e125097_d_n17, assign83370_e125097_d_n18, assign83370_e125097_d_n19, assign83370_e125097_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83370_e125093: f64 = (locals.var_c_igid * locals.var_sqid);
        let assign83370_e125095: f64 = (assign83370_e125093 / locals.var_sqig);
        (assign83370_e125095, (((((locals.var_c_igid_dn5 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn5)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn5)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn6 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn6)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn6)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn7 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn7)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn7)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn8 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn8)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn8)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn12 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn12)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn12)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn13 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn13)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn13)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn14 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn14)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn14)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn15 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn15)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn15)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn16 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn16)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn16)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn17 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn17)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn17)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn18 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn18)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn18)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn19 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn19)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn19)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn20 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn20)) * locals.var_sqig) - (assign83370_e125093 * locals.var_sqig_dn20)) / (locals.var_sqig * locals.var_sqig)),)
    } else {
        (locals.var_migid, locals.var_migid_dn5, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, locals.var_migid_dn12, locals.var_migid_dn13, locals.var_migid_dn14, locals.var_migid_dn15, locals.var_migid_dn16, locals.var_migid_dn17, locals.var_migid_dn18, locals.var_migid_dn19, locals.var_migid_dn20,)
    }
};
        locals.var_migid = assign83370_e125097;
        locals.var_migid_dn5 = assign83370_e125097_d_n5;
        locals.var_migid_dn6 = assign83370_e125097_d_n6;
        locals.var_migid_dn7 = assign83370_e125097_d_n7;
        locals.var_migid_dn8 = assign83370_e125097_d_n8;
        locals.var_migid_dn12 = assign83370_e125097_d_n12;
        locals.var_migid_dn13 = assign83370_e125097_d_n13;
        locals.var_migid_dn14 = assign83370_e125097_d_n14;
        locals.var_migid_dn15 = assign83370_e125097_d_n15;
        locals.var_migid_dn16 = assign83370_e125097_d_n16;
        locals.var_migid_dn17 = assign83370_e125097_d_n17;
        locals.var_migid_dn18 = assign83370_e125097_d_n18;
        locals.var_migid_dn19 = assign83370_e125097_d_n19;
        locals.var_migid_dn20 = assign83370_e125097_d_n20;

        let assign83540_e125205: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign83540_e125205;

    }

    pub(super) fn stamp_transient_block_167(
        locals: &mut StampLocals,
    ) {
        let (assign83550_e125213, assign83550_e125213_d_n5, assign83550_e125213_d_n6, assign83550_e125213_d_n7, assign83550_e125213_d_n8, assign83550_e125213_d_n12, assign83550_e125213_d_n13, assign83550_e125213_d_n14, assign83550_e125213_d_n15, assign83550_e125213_d_n16, assign83550_e125213_d_n17, assign83550_e125213_d_n18, assign83550_e125213_d_n19, assign83550_e125213_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83550_e125209: f64 = (4.0 * locals.var_dsqredge);
        let assign83550_e125211: f64 = (assign83550_e125209 / locals.var_gfedge2);
        (assign83550_e125211, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn12) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn13) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn14) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn15) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn16) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn17) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn18) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn19) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn20) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83550_e125213;
        locals.var_temp1_dn5 = assign83550_e125213_d_n5;
        locals.var_temp1_dn6 = assign83550_e125213_d_n6;
        locals.var_temp1_dn7 = assign83550_e125213_d_n7;
        locals.var_temp1_dn8 = assign83550_e125213_d_n8;
        locals.var_temp1_dn12 = assign83550_e125213_d_n12;
        locals.var_temp1_dn13 = assign83550_e125213_d_n13;
        locals.var_temp1_dn14 = assign83550_e125213_d_n14;
        locals.var_temp1_dn15 = assign83550_e125213_d_n15;
        locals.var_temp1_dn16 = assign83550_e125213_d_n16;
        locals.var_temp1_dn17 = assign83550_e125213_d_n17;
        locals.var_temp1_dn18 = assign83550_e125213_d_n18;
        locals.var_temp1_dn19 = assign83550_e125213_d_n19;
        locals.var_temp1_dn20 = assign83550_e125213_d_n20;

        let (assign83570_e125233, assign83570_e125233_d_n5, assign83570_e125233_d_n6, assign83570_e125233_d_n7, assign83570_e125233_d_n8, assign83570_e125233_d_n12, assign83570_e125233_d_n13, assign83570_e125233_d_n14, assign83570_e125233_d_n15, assign83570_e125233_d_n16, assign83570_e125233_d_n17, assign83570_e125233_d_n18, assign83570_e125233_d_n19, assign83570_e125233_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83570_e125231: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign83570_e125231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83570_e125233;
        locals.var_temp1_dn5 = assign83570_e125233_d_n5;
        locals.var_temp1_dn6 = assign83570_e125233_d_n6;
        locals.var_temp1_dn7 = assign83570_e125233_d_n7;
        locals.var_temp1_dn8 = assign83570_e125233_d_n8;
        locals.var_temp1_dn12 = assign83570_e125233_d_n12;
        locals.var_temp1_dn13 = assign83570_e125233_d_n13;
        locals.var_temp1_dn14 = assign83570_e125233_d_n14;
        locals.var_temp1_dn15 = assign83570_e125233_d_n15;
        locals.var_temp1_dn16 = assign83570_e125233_d_n16;
        locals.var_temp1_dn17 = assign83570_e125233_d_n17;
        locals.var_temp1_dn18 = assign83570_e125233_d_n18;
        locals.var_temp1_dn19 = assign83570_e125233_d_n19;
        locals.var_temp1_dn20 = assign83570_e125233_d_n20;

        let (assign83700_e125373, assign83700_e125373_d_n5, assign83700_e125373_d_n6, assign83700_e125373_d_n7, assign83700_e125373_d_n8, assign83700_e125373_d_n12, assign83700_e125373_d_n13, assign83700_e125373_d_n14, assign83700_e125373_d_n15, assign83700_e125373_d_n16, assign83700_e125373_d_n17, assign83700_e125373_d_n18, assign83700_e125373_d_n19, assign83700_e125373_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83700_e125371: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign83700_e125371, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn12 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn12)), ((locals.var_alpha_dc_dn13 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn13)), ((locals.var_alpha_dc_dn14 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn14)), ((locals.var_alpha_dc_dn15 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn15)), ((locals.var_alpha_dc_dn16 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn16)), ((locals.var_alpha_dc_dn17 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn17)), ((locals.var_alpha_dc_dn18 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn18)), ((locals.var_alpha_dc_dn19 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn19)), ((locals.var_alpha_dc_dn20 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn20)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83700_e125373;
        locals.var_temp1_dn5 = assign83700_e125373_d_n5;
        locals.var_temp1_dn6 = assign83700_e125373_d_n6;
        locals.var_temp1_dn7 = assign83700_e125373_d_n7;
        locals.var_temp1_dn8 = assign83700_e125373_d_n8;
        locals.var_temp1_dn12 = assign83700_e125373_d_n12;
        locals.var_temp1_dn13 = assign83700_e125373_d_n13;
        locals.var_temp1_dn14 = assign83700_e125373_d_n14;
        locals.var_temp1_dn15 = assign83700_e125373_d_n15;
        locals.var_temp1_dn16 = assign83700_e125373_d_n16;
        locals.var_temp1_dn17 = assign83700_e125373_d_n17;
        locals.var_temp1_dn18 = assign83700_e125373_d_n18;
        locals.var_temp1_dn19 = assign83700_e125373_d_n19;
        locals.var_temp1_dn20 = assign83700_e125373_d_n20;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1569;
        locals.var_guard1_rv = 0.0;

        let (assign10_e1574,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1572: f64 = 1.0;
        (assign10_e1572,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1574;
        locals.var_chnl_type_rv = 0.0;

        let (assign20_e1580,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1578: f64 = (-1.0);
        (assign20_e1578,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1580;
        locals.var_chnl_type_rv = 0.0;

        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1583;
        locals.var_epssi_rv = 0.0;

        let assign40_e1586: f64 = if p.p51 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign40_e1586;
        locals.var_guard2_rv = 0.0;

        let (assign50_e1590,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign50_e1590;
        locals.var_swnqs_i_rv = 0.0;

        let assign60_e1593: f64 = if p.p51 < 1.5 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign60_e1593;
        locals.var_guard3_rv = 0.0;

        let (assign70_e1600,) = {
    if ((locals.var_guard2 == 0.0) && (locals.var_guard3 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign70_e1600;
        locals.var_swnqs_i_rv = 0.0;

        let assign80_e1603: f64 = if p.p51 < 2.5 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign80_e1603;
        locals.var_guard4_rv = 0.0;

        let (assign90_e1613,) = {
    if (((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign90_e1613;
        locals.var_swnqs_i_rv = 0.0;

        let assign100_e1616: f64 = if p.p51 < 4.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign100_e1616;
        locals.var_guard5_rv = 0.0;

        let (assign110_e1629,) = {
    if ((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign110_e1629;
        locals.var_swnqs_i_rv = 0.0;

        let assign120_e1632: f64 = if p.p51 < 7.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign120_e1632;
        locals.var_guard6_rv = 0.0;

        let (assign130_e1648,) = {
    if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 != 0.0)) {
        (5.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign130_e1648;
        locals.var_swnqs_i_rv = 0.0;

        let (assign140_e1665,) = {
    if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 == 0.0)) {
        (9.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign140_e1665;
        locals.var_swnqs_i_rv = 0.0;

        locals.var_vnorm = 10.0;
        locals.var_vnorm_rv = 0.0;

        let assign170_e1670: f64 = (1.0 / locals.var_vnorm);
        locals.var_vnorm_inv = assign170_e1670;
        locals.var_vnorm_inv_rv = 0.0;

        let assign180_e1673: f64 = (273.15 + p.p38);
        locals.var_tkr = assign180_e1673;
        locals.var_tkr_rv = 0.0;

        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        locals.var_tka = assign2190_e2708;
        locals.var_tka_rv = 0.0;

        let assign2200_e2711: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2200_e2711;
        locals.var_rta_rv = 0.0;

        let assign2210_e2714: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2210_e2714;
        locals.var_delta_rv = 0.0;

        let assign2220_e2717: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        locals.var_phita = assign2220_e2719;
        locals.var_phita_rv = 0.0;

        let assign2230_e2722: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2230_e2722;
        locals.var_inv_phita_rv = 0.0;

        locals.var_tkd = locals.var_tka;
        locals.var_tkd_rv = 0.0;

        let assign2250_e2726: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign2250_e2726;
        locals.var_tkd_sq_rv = 0.0;

        let assign2260_e2729: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign2260_e2729;
        locals.var_delt_rv = 0.0;

        let assign2270_e2732: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign2270_e2732;
        locals.var_rtn_rv = 0.0;

        let assign2280_e2734: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign2280_e2734;
        locals.var_ln_rtn_rv = 0.0;

        let assign2290_e2737: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign2290_e2739: f64 = (assign2290_e2737 / 1.6021918e-19);
        locals.var_phit = assign2290_e2739;
        locals.var_phit_rv = 0.0;

        let assign2300_e2742: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign2300_e2742;
        locals.var_inv_phit_rv = 0.0;

        let assign2310_e2746: f64 = (9.025e-5 * locals.var_tkd);
        let assign2310_e2747: f64 = (1.179 - assign2310_e2746);
        let assign2310_e2750: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign2310_e2751: f64 = (assign2310_e2747 - assign2310_e2750);
        locals.var_eg = assign2310_e2751;
        locals.var_eg_rv = 0.0;

        let assign2320_e2755: f64 = (0.00045 * locals.var_tkd);
        let assign2320_e2756: f64 = (1.045 + assign2320_e2755);
        let assign2320_e2760: f64 = (0.0014 * locals.var_tkd);
        let assign2320_e2761: f64 = (0.523 + assign2320_e2760);
        let assign2320_e2764: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign2320_e2765: f64 = (assign2320_e2761 - assign2320_e2764);
        let assign2320_e2766: f64 = (assign2320_e2756 * assign2320_e2765);
        let assign2320_e2768: f64 = (assign2320_e2766 * locals.var_tkd_sq);
        let assign2320_e2770: f64 = (assign2320_e2768 / 90000.0);
        locals.var_phibfac = assign2320_e2770;
        locals.var_phibfac_rv = 0.0;

        let (assign2330_e2776,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac,)
    } else {
        (0.001,)
    }
};
        locals.var_phibfac = assign2330_e2776;
        locals.var_phibfac_rv = 0.0;

        locals.var_nf_i = 1.0;
        locals.var_nf_i_rv = 0.0;

        locals.var_invnf = 1.0;
        locals.var_invnf_rv = 0.0;

        locals.var_le = 0.0;
        locals.var_le_rv = 0.0;

        locals.var_we = 0.0;
        locals.var_we_rv = 0.0;

        locals.var_l_i = p.p0;
        locals.var_l_i_rv = 0.0;

        locals.var_w_i = p.p1;
        locals.var_w_i_rv = 0.0;

        locals.var_sa_i = p.p2;
        locals.var_sa_i_rv = 0.0;

        locals.var_sb_i = p.p3;
        locals.var_sb_i_rv = 0.0;

        locals.var_sd_i = p.p4;
        locals.var_sd_i_rv = 0.0;

        locals.var_sc_i = p.p8;
        locals.var_sc_i_rv = 0.0;

        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3640_e3629;
        locals.var_guard34_rv = 0.0;

        let (assign3650_e3638,) = {
    if (locals.var_guard34 != 0.0) {
        let (assign3650_e3636,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3650_e3636,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3650_e3638;
        locals.var_nf_i_rv = 0.0;

        let (assign3660_e3645,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3660_e3642: f64 = (locals.var_nf_i + 0.5);
        let assign3660_e3643: f64 = (assign3660_e3642).floor();
        (assign3660_e3643,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3660_e3645;
        locals.var_nf_i_rv = 0.0;

        let (assign3670_e3651,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3670_e3649: f64 = (1.0 / locals.var_nf_i);
        (assign3670_e3649,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3670_e3651;
        locals.var_invnf_rv = 0.0;

        let assign3680_e3654: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3680_e3661;
        locals.var_w_i_rv = 0.0;

        locals.var_sca_i = p.p5;
        locals.var_sca_i_rv = 0.0;

        locals.var_scb_i = p.p6;
        locals.var_scb_i_rv = 0.0;

        locals.var_scc_i = p.p7;
        locals.var_scc_i_rv = 0.0;

        let assign3730_e3673: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3730_e3673;
        locals.var_il_rv = 0.0;

        let assign3740_e3676: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3740_e3676;
        locals.var_iw_rv = 0.0;

        let assign3750_e3681: f64 = (p.p189 * locals.var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * locals.var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        locals.var_dellps = assign3750_e3689;
        locals.var_dellps_rv = 0.0;

        let assign3760_e3694: f64 = (p.p193 * locals.var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * locals.var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        locals.var_delwod = assign3760_e3702;
        locals.var_delwod_rv = 0.0;

        let assign3770_e3705: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3770_e3720;
        locals.var_le_rv = 0.0;

        let assign3780_e3723: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3780_e3738;
        locals.var_we_rv = 0.0;

        let assign3790_e3741: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3790_e3741;
        locals.var_ile_rv = 0.0;

        let assign3800_e3744: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3800_e3744;
        locals.var_ile2_rv = 0.0;

        let assign3810_e3747: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3810_e3747;
        locals.var_iwe_rv = 0.0;

        let assign3820_e3750: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3820_e3750;
        locals.var_iiwe_rv = 0.0;

        let assign3830_e3753: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3830_e3753;
        locals.var_iae_rv = 0.0;

        let assign3840_e3756: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3840_e3756;
        locals.var_iiae_rv = 0.0;

        let assign3850_e3759: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3762: f64 = (2.0 * p.p191);
        let assign3850_e3763: f64 = (assign3850_e3759 - assign3850_e3762);
        let assign3850_e3765: f64 = (assign3850_e3763 + p.p196);
        let (assign3850_e3778,) = {
    if (assign3850_e3765 > 1e-9) {
        let assign3850_e3770: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3773: f64 = (2.0 * p.p191);
        let assign3850_e3774: f64 = (assign3850_e3770 - assign3850_e3773);
        let assign3850_e3776: f64 = (assign3850_e3774 + p.p196);
        (assign3850_e3776,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3850_e3778;
        locals.var_lecv_rv = 0.0;

        let assign3860_e3781: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3860_e3800;
        locals.var_wecv_rv = 0.0;

        let assign3870_e3803: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3870_e3803;
        locals.var_iiwecv_rv = 0.0;

        let assign3880_e3806: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3808: f64 = (assign3880_e3806 + p.p196);
        let (assign3880_e3817,) = {
    if (assign3880_e3808 > 1e-9) {
        let assign3880_e3813: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3815: f64 = (assign3880_e3813 + p.p196);
        (assign3880_e3815,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3880_e3817;
        locals.var_lcv_rv = 0.0;

        let assign3900_e3834: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3900_e3834;
        locals.var_iilcv_rv = 0.0;

        locals.var_vfb_p = p.p57;
        locals.var_vfb_p_rv = 0.0;

        locals.var_stvfb_p = p.p58;
        locals.var_stvfb_p_rv = 0.0;

        locals.var_st2vfb_p = p.p59;
        locals.var_st2vfb_p_rv = 0.0;

        locals.var_tox_p = p.p60;
        locals.var_tox_p_rv = 0.0;

        locals.var_epsrox_p = p.p61;
        locals.var_epsrox_p_rv = 0.0;

        locals.var_neff_p = p.p62;
        locals.var_neff_p_rv = 0.0;

        locals.var_gfacnud_p = p.p63;
        locals.var_gfacnud_p_rv = 0.0;

        locals.var_vsbnud_p = p.p64;
        locals.var_vsbnud_p_rv = 0.0;

        locals.var_dvsbnud_p = p.p65;
        locals.var_dvsbnud_p_rv = 0.0;

        locals.var_dphib_p = p.p66;
        locals.var_dphib_p_rv = 0.0;

        locals.var_np_p = p.p67;
        locals.var_np_p_rv = 0.0;

        locals.var_toxov_p = p.p68;
        locals.var_toxov_p_rv = 0.0;

        locals.var_toxovd_p = p.p69;
        locals.var_toxovd_p_rv = 0.0;

        locals.var_nov_p = p.p70;
        locals.var_nov_p_rv = 0.0;

        locals.var_novd_p = p.p71;
        locals.var_novd_p_rv = 0.0;

        locals.var_ct_p = p.p72;
        locals.var_ct_p_rv = 0.0;

        locals.var_ctg_p = p.p74;
        locals.var_ctg_p_rv = 0.0;

        locals.var_ctb_p = p.p73;
        locals.var_ctb_p_rv = 0.0;

        locals.var_stct_p = p.p75;
        locals.var_stct_p_rv = 0.0;

        locals.var_psce_p = p.p79;
        locals.var_psce_p_rv = 0.0;

        locals.var_psced_p = p.p81;
        locals.var_psced_p_rv = 0.0;

        locals.var_psceb_p = p.p80;
        locals.var_psceb_p_rv = 0.0;

        locals.var_cf_p = p.p76;
        locals.var_cf_p_rv = 0.0;

        locals.var_cfd_p = p.p78;
        locals.var_cfd_p_rv = 0.0;

        locals.var_cfb_p = p.p77;
        locals.var_cfb_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_betn_p = p.p82;
        locals.var_betn_p_rv = 0.0;

        locals.var_stbet_p = p.p83;
        locals.var_stbet_p_rv = 0.0;

        locals.var_mue_p = p.p84;
        locals.var_mue_p_rv = 0.0;

        locals.var_stmue_p = p.p85;
        locals.var_stmue_p_rv = 0.0;

        locals.var_themu_p = p.p86;
        locals.var_themu_p_rv = 0.0;

        locals.var_stthemu_p = p.p87;
        locals.var_stthemu_p_rv = 0.0;

        locals.var_cs_p = p.p88;
        locals.var_cs_p_rv = 0.0;

        locals.var_stcs_p = p.p89;
        locals.var_stcs_p_rv = 0.0;

        locals.var_thecs_p = p.p90;
        locals.var_thecs_p_rv = 0.0;

        locals.var_stthecs_p = p.p91;
        locals.var_stthecs_p_rv = 0.0;

        locals.var_xcor_p = p.p92;
        locals.var_xcor_p_rv = 0.0;

        locals.var_stxcor_p = p.p93;
        locals.var_stxcor_p_rv = 0.0;

        locals.var_feta_p = p.p94;
        locals.var_feta_p_rv = 0.0;

        locals.var_rs_p = p.p95;
        locals.var_rs_p_rv = 0.0;

        locals.var_strs_p = p.p96;
        locals.var_strs_p_rv = 0.0;

        locals.var_rsb_p = p.p97;
        locals.var_rsb_p_rv = 0.0;

        locals.var_rsg_p = p.p98;
        locals.var_rsg_p_rv = 0.0;

        locals.var_thesat_p = p.p99;
        locals.var_thesat_p_rv = 0.0;

        locals.var_stthesat_p = p.p100;
        locals.var_stthesat_p_rv = 0.0;

        locals.var_thesatb_p = p.p101;
        locals.var_thesatb_p_rv = 0.0;

        locals.var_thesatg_p = p.p102;
        locals.var_thesatg_p_rv = 0.0;

        locals.var_thesatt_p = p.p103;
        locals.var_thesatt_p_rv = 0.0;

        locals.var_ax_p = p.p104;
        locals.var_ax_p_rv = 0.0;

        locals.var_alp_p = p.p105;
        locals.var_alp_p_rv = 0.0;

        locals.var_alp1_p = p.p106;
        locals.var_alp1_p_rv = 0.0;

        locals.var_alp2_p = p.p107;
        locals.var_alp2_p_rv = 0.0;

        locals.var_vp_p = p.p108;
        locals.var_vp_p_rv = 0.0;

        locals.var_a1_p = p.p109;
        locals.var_a1_p_rv = 0.0;

        locals.var_a2_p = p.p110;
        locals.var_a2_p_rv = 0.0;

        locals.var_sta2_p = p.p111;
        locals.var_sta2_p_rv = 0.0;

        locals.var_a3_p = p.p112;
        locals.var_a3_p_rv = 0.0;

        locals.var_a4_p = p.p113;
        locals.var_a4_p_rv = 0.0;

        locals.var_imaxii_p = p.p114;
        locals.var_imaxii_p_rv = 0.0;

        locals.var_gco_p = p.p115;
        locals.var_gco_p_rv = 0.0;

        locals.var_iginv_p = p.p116;
        locals.var_iginv_p_rv = 0.0;

        locals.var_igov_p = p.p117;
        locals.var_igov_p_rv = 0.0;

        locals.var_igovd_p = p.p118;
        locals.var_igovd_p_rv = 0.0;

        locals.var_stig_p = p.p119;
        locals.var_stig_p_rv = 0.0;

        locals.var_gc2_p = p.p120;
        locals.var_gc2_p_rv = 0.0;

        locals.var_gc3_p = p.p121;
        locals.var_gc3_p_rv = 0.0;

        locals.var_gc2ov_p = p.p120;
        locals.var_gc2ov_p_rv = 0.0;

        let assign4620_e3949: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4620_e3951: f64 = if assign4620_e3949 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4620_e3951;
        locals.var_guard35_rv = 0.0;

        let (assign4630_e3955,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4630_e3955;
        locals.var_gc2ov_p_rv = 0.0;

        locals.var_gc3ov_p = p.p121;
        locals.var_gc3ov_p_rv = 0.0;

        let assign4650_e3958: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4650_e3960: f64 = if assign4650_e3958 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign4650_e3960;
        locals.var_guard36_rv = 0.0;

        let (assign4660_e3964,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4660_e3964;
        locals.var_gc3ov_p_rv = 0.0;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign4680_e3967: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4680_e3969: f64 = if assign4680_e3967 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4680_e3969;
        locals.var_guard37_rv = 0.0;

        let (assign4690_e3973,) = {
    if (locals.var_guard37 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4690_e3973;
        locals.var_gc2ovd_p_rv = 0.0;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign4710_e3976: f64 = if param_given[125] { 1.0 } else { 0.0 };
        let assign4710_e3978: f64 = if assign4710_e3976 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign4710_e3978;
        locals.var_guard38_rv = 0.0;

        let (assign4720_e3982,) = {
    if (locals.var_guard38 != 0.0) {
        (p.p125,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4720_e3982;
        locals.var_gc3ovd_p_rv = 0.0;

        locals.var_chib_p = p.p126;
        locals.var_chib_p_rv = 0.0;

        locals.var_agidl_p = p.p127;
        locals.var_agidl_p_rv = 0.0;

        locals.var_agidld_p = p.p128;
        locals.var_agidld_p_rv = 0.0;

        locals.var_bgidl_p = p.p129;
        locals.var_bgidl_p_rv = 0.0;

        locals.var_bgidld_p = p.p130;
        locals.var_bgidld_p_rv = 0.0;

        locals.var_stbgidl_p = p.p131;
        locals.var_stbgidl_p_rv = 0.0;

        locals.var_stbgidld_p = p.p132;
        locals.var_stbgidld_p_rv = 0.0;

        locals.var_cgidl_p = p.p133;
        locals.var_cgidl_p_rv = 0.0;

        locals.var_cgidld_p = p.p134;
        locals.var_cgidld_p_rv = 0.0;

        locals.var_cox_p = p.p135;
        locals.var_cox_p_rv = 0.0;

        locals.var_delvtac_p = p.p136;
        locals.var_delvtac_p_rv = 0.0;

        locals.var_facneffac_p = p.p137;
        locals.var_facneffac_p_rv = 0.0;

        locals.var_thesatac_p = p.p99;
        locals.var_thesatac_p_rv = 0.0;

        let assign4860_e3997: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4860_e3999: f64 = if assign4860_e3997 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign4860_e3999;
        locals.var_guard39_rv = 0.0;

        let (assign4870_e4003,) = {
    if (locals.var_guard39 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4870_e4003;
        locals.var_thesatac_p_rv = 0.0;

        locals.var_axac_p = p.p104;
        locals.var_axac_p_rv = 0.0;

        let assign4890_e4006: f64 = if param_given[139] { 1.0 } else { 0.0 };
        let assign4890_e4008: f64 = if assign4890_e4006 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4890_e4008;
        locals.var_guard40_rv = 0.0;

        let (assign4900_e4012,) = {
    if (locals.var_guard40 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4900_e4012;
        locals.var_axac_p_rv = 0.0;

        locals.var_alpac_p = p.p140;
        locals.var_alpac_p_rv = 0.0;

        locals.var_alp1ac_p = p.p141;
        locals.var_alp1ac_p_rv = 0.0;

        locals.var_cgov_p = p.p142;
        locals.var_cgov_p_rv = 0.0;

        locals.var_cgovd_p = p.p143;
        locals.var_cgovd_p_rv = 0.0;

        locals.var_fcgovacc_p = p.p144;
        locals.var_fcgovacc_p_rv = 0.0;

        locals.var_fcgovaccd_p = p.p145;
        locals.var_fcgovaccd_p_rv = 0.0;

        locals.var_cgovaccg_p = p.p146;
        locals.var_cgovaccg_p_rv = 0.0;

        locals.var_cgbov_p = p.p147;
        locals.var_cgbov_p_rv = 0.0;

        locals.var_cinr_p = p.p148;
        locals.var_cinr_p_rv = 0.0;

        locals.var_cinrd_p = p.p149;
        locals.var_cinrd_p_rv = 0.0;

        locals.var_dvfbinr_p = p.p150;
        locals.var_dvfbinr_p_rv = 0.0;

        locals.var_fcinrdep_p = p.p151;
        locals.var_fcinrdep_p_rv = 0.0;

        locals.var_fcinracc_p = p.p152;
        locals.var_fcinracc_p_rv = 0.0;

        locals.var_axinr_p = p.p153;
        locals.var_axinr_p_rv = 0.0;

        locals.var_fnt_p = p.p156;
        locals.var_fnt_p_rv = 0.0;

        locals.var_vfbedge_p = p.p162;
        locals.var_vfbedge_p_rv = 0.0;

        locals.var_stvfbedge_p = p.p163;
        locals.var_stvfbedge_p_rv = 0.0;

        locals.var_dphibedge_p = p.p164;
        locals.var_dphibedge_p_rv = 0.0;

        locals.var_neffedge_p = p.p165;
        locals.var_neffedge_p_rv = 0.0;

        locals.var_ctedge_p = p.p166;
        locals.var_ctedge_p_rv = 0.0;

        locals.var_betnedge_p = p.p167;
        locals.var_betnedge_p_rv = 0.0;

        locals.var_stbetedge_p = p.p168;
        locals.var_stbetedge_p_rv = 0.0;

        locals.var_psceedge_p = p.p169;
        locals.var_psceedge_p_rv = 0.0;

        locals.var_pscebedge_p = p.p170;
        locals.var_pscebedge_p_rv = 0.0;

        locals.var_pscededge_p = p.p171;
        locals.var_pscededge_p_rv = 0.0;

        locals.var_cfedge_p = p.p172;
        locals.var_cfedge_p_rv = 0.0;

        locals.var_cfdedge_p = p.p174;
        locals.var_cfdedge_p_rv = 0.0;

        locals.var_cfbedge_p = p.p173;
        locals.var_cfbedge_p_rv = 0.0;

        locals.var_munqs_p = p.p187;
        locals.var_munqs_p_rv = 0.0;

        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign5390_e4063;
        locals.var_guard41_rv = 0.0;

        let (assign5400_e4081,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5400_e4069: f64 = (locals.var_ile).powf(p.p200);
        let assign5400_e4070: f64 = (p.p199 * assign5400_e4069);
        let assign5400_e4071: f64 = (p.p198 + assign5400_e4070);
        let assign5400_e4074: f64 = (p.p201 * locals.var_iwe);
        let assign5400_e4075: f64 = (assign5400_e4071 + assign5400_e4074);
        let assign5400_e4078: f64 = (p.p202 * locals.var_iae);
        let assign5400_e4079: f64 = (assign5400_e4075 + assign5400_e4078);
        (assign5400_e4079,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5400_e4081;
        locals.var_vfb_p_rv = 0.0;

        let (assign5410_e4097,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5410_e4086: f64 = (p.p204 * locals.var_ile);
        let assign5410_e4087: f64 = (p.p203 + assign5410_e4086);
        let assign5410_e4090: f64 = (p.p205 * locals.var_iwe);
        let assign5410_e4091: f64 = (assign5410_e4087 + assign5410_e4090);
        let assign5410_e4094: f64 = (p.p206 * locals.var_iae);
        let assign5410_e4095: f64 = (assign5410_e4091 + assign5410_e4094);
        (assign5410_e4095,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5410_e4097;
        locals.var_stvfb_p_rv = 0.0;

        let (assign5420_e4101,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5420_e4101;
        locals.var_st2vfb_p_rv = 0.0;

        let (assign5430_e4105,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5430_e4105;
        locals.var_tox_p_rv = 0.0;

        let (assign5440_e4109,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5440_e4109;
        locals.var_epsrox_p_rv = 0.0;

        let (assign5450_e4142,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5450_e4115: f64 = (p.p211 * locals.var_iwe);
        let assign5450_e4119: f64 = (locals.var_we / p.p212);
        let assign5450_e4120: f64 = (1.0 + assign5450_e4119);
        let assign5450_e4121: f64 = (assign5450_e4120).ln();
        let assign5450_e4122: f64 = (assign5450_e4115 * assign5450_e4121);
        let assign5450_e4123: f64 = (1.0 + assign5450_e4122);
        let (assign5450_e4139,) = {
            if (assign5450_e4123 > 0.001) {
                let assign5450_e4129: f64 = (p.p211 * locals.var_iwe);
                let assign5450_e4133: f64 = (locals.var_we / p.p212);
                let assign5450_e4134: f64 = (1.0 + assign5450_e4133);
                let assign5450_e4135: f64 = (assign5450_e4134).ln();
                let assign5450_e4136: f64 = (assign5450_e4129 * assign5450_e4135);
                let assign5450_e4137: f64 = (1.0 + assign5450_e4136);
                (assign5450_e4137,)
            } else {
                (0.001,)
            }
        };
        let assign5450_e4140: f64 = (p.p210 * assign5450_e4139);
        (assign5450_e4140,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5450_e4142;
        locals.var_nsub0e_rv = 0.0;

        let (assign5460_e4175,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5460_e4148: f64 = (p.p214 * locals.var_iwe);
        let assign5460_e4152: f64 = (locals.var_we / p.p215);
        let assign5460_e4153: f64 = (1.0 + assign5460_e4152);
        let assign5460_e4154: f64 = (assign5460_e4153).ln();
        let assign5460_e4155: f64 = (assign5460_e4148 * assign5460_e4154);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4172,) = {
            if (assign5460_e4156 > 0.001) {
                let assign5460_e4162: f64 = (p.p214 * locals.var_iwe);
                let assign5460_e4166: f64 = (locals.var_we / p.p215);
                let assign5460_e4167: f64 = (1.0 + assign5460_e4166);
                let assign5460_e4168: f64 = (assign5460_e4167).ln();
                let assign5460_e4169: f64 = (assign5460_e4162 * assign5460_e4168);
                let assign5460_e4170: f64 = (1.0 + assign5460_e4169);
                (assign5460_e4170,)
            } else {
                (0.001,)
            }
        };
        let assign5460_e4173: f64 = (p.p213 * assign5460_e4172);
        (assign5460_e4173,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5460_e4175;
        locals.var_npcke_rv = 0.0;

        let (assign5470_e4208,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5470_e4181: f64 = (p.p217 * locals.var_iwe);
        let assign5470_e4185: f64 = (locals.var_we / p.p215);
        let assign5470_e4186: f64 = (1.0 + assign5470_e4185);
        let assign5470_e4187: f64 = (assign5470_e4186).ln();
        let assign5470_e4188: f64 = (assign5470_e4181 * assign5470_e4187);
        let assign5470_e4189: f64 = (1.0 + assign5470_e4188);
        let (assign5470_e4205,) = {
            if (assign5470_e4189 > 0.001) {
                let assign5470_e4195: f64 = (p.p217 * locals.var_iwe);
                let assign5470_e4199: f64 = (locals.var_we / p.p215);
                let assign5470_e4200: f64 = (1.0 + assign5470_e4199);
                let assign5470_e4201: f64 = (assign5470_e4200).ln();
                let assign5470_e4202: f64 = (assign5470_e4195 * assign5470_e4201);
                let assign5470_e4203: f64 = (1.0 + assign5470_e4202);
                (assign5470_e4203,)
            } else {
                (0.001,)
            }
        };
        let assign5470_e4206: f64 = (p.p216 * assign5470_e4205);
        (assign5470_e4206,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5470_e4208;
        locals.var_lpcke_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5480_e4212: f64 = (2.0 * locals.var_lpcke);
        let assign5480_e4213: f64 = if locals.var_le > assign5480_e4212 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign5480_e4213;
        locals.var_guard42_rv = 0.0;

        let (assign5490_e4219,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5490_e4219;
        locals.var_aa_rv = 0.0;

        let (assign5500_e4233,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5500_e4226: f64 = (0.5 * locals.var_npcke);
        let assign5500_e4227: f64 = (locals.var_nsub0e + assign5500_e4226);
        let assign5500_e4228: f64 = (assign5500_e4227).sqrt();
        let assign5500_e4230: f64 = (locals.var_nsub0e).sqrt();
        let assign5500_e4231: f64 = (assign5500_e4228 - assign5500_e4230);
        (assign5500_e4231,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5500_e4233;
        locals.var_bb_rv = 0.0;

        let (assign5510_e4258,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5510_e4238: f64 = (locals.var_nsub0e).sqrt();
        let assign5510_e4243: f64 = (2.0 * locals.var_lpcke);
        let assign5510_e4245: f64 = (assign5510_e4243 / locals.var_le);
        let assign5510_e4248: f64 = (locals.var_bb / locals.var_aa);
        let assign5510_e4249: f64 = (assign5510_e4248).exp();
        let assign5510_e4251: f64 = (assign5510_e4249 - 1.0);
        let assign5510_e4252: f64 = (assign5510_e4245 * assign5510_e4251);
        let assign5510_e4253: f64 = (1.0 + assign5510_e4252);
        let assign5510_e4254: f64 = (assign5510_e4253).ln();
        let assign5510_e4255: f64 = (locals.var_aa * assign5510_e4254);
        let assign5510_e4256: f64 = (assign5510_e4238 + assign5510_e4255);
        (assign5510_e4256,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5510_e4258;
        locals.var_nsub_rv = 0.0;

        let (assign5520_e4266,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5520_e4264: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5520_e4264,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5520_e4266;
        locals.var_nsub_rv = 0.0;

        let assign5530_e4269: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard43 = assign5530_e4269;
        locals.var_guard43_rv = 0.0;

        let (assign5540_e4284,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
        let assign5540_e4279: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5540_e4281: f64 = (assign5540_e4279 / locals.var_le);
        let assign5540_e4282: f64 = (locals.var_nsub0e + assign5540_e4281);
        (assign5540_e4282,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5540_e4284;
        locals.var_nsub_rv = 0.0;

        let (assign5550_e4302,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 == 0.0)) {
        let assign5550_e4297: f64 = (locals.var_le / locals.var_lpcke);
        let assign5550_e4298: f64 = (2.0 - assign5550_e4297);
        let assign5550_e4299: f64 = (locals.var_npcke * assign5550_e4298);
        let assign5550_e4300: f64 = (locals.var_nsub0e + assign5550_e4299);
        (assign5550_e4300,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5550_e4302;
        locals.var_nsub_rv = 0.0;

        let (assign5560_e4316,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5560_e4308: f64 = (p.p218 * locals.var_ile);
        let assign5560_e4309: f64 = (1.0 - assign5560_e4308);
        let assign5560_e4312: f64 = (p.p219 * locals.var_ile2);
        let assign5560_e4313: f64 = (assign5560_e4309 - assign5560_e4312);
        let assign5560_e4314: f64 = (locals.var_nsub * assign5560_e4313);
        (assign5560_e4314,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5560_e4316;
        locals.var_neff_p_rv = 0.0;

        let (assign5570_e4334,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5570_e4322: f64 = (locals.var_ile).powf(p.p222);
        let assign5570_e4323: f64 = (p.p221 * assign5570_e4322);
        let assign5570_e4324: f64 = (p.p220 + assign5570_e4323);
        let assign5570_e4327: f64 = (p.p223 * locals.var_iwe);
        let assign5570_e4328: f64 = (assign5570_e4324 + assign5570_e4327);
        let assign5570_e4331: f64 = (p.p224 * locals.var_iae);
        let assign5570_e4332: f64 = (assign5570_e4328 + assign5570_e4331);
        (assign5570_e4332,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5570_e4334;
        locals.var_gfacnud_p_rv = 0.0;

        let (assign5580_e4338,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p225,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5580_e4338;
        locals.var_vsbnud_p_rv = 0.0;

        let (assign5590_e4342,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5590_e4342;
        locals.var_dvsbnud_p_rv = 0.0;

        let (assign5600_e4360,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5600_e4348: f64 = (locals.var_ile).powf(p.p229);
        let assign5600_e4349: f64 = (p.p228 * assign5600_e4348);
        let assign5600_e4350: f64 = (p.p227 + assign5600_e4349);
        let assign5600_e4353: f64 = (p.p230 * locals.var_iwe);
        let assign5600_e4354: f64 = (assign5600_e4350 + assign5600_e4353);
        let assign5600_e4357: f64 = (p.p231 * locals.var_iae);
        let assign5600_e4358: f64 = (assign5600_e4354 + assign5600_e4357);
        (assign5600_e4358,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5600_e4360;
        locals.var_dphib_p_rv = 0.0;

        let (assign5610_e4379,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5610_e4367: f64 = (p.p233 * locals.var_ile);
        let assign5610_e4368: f64 = (1.0 + assign5610_e4367);
        let (assign5610_e4376,) = {
            if (1e-6 > assign5610_e4368) {
                (1e-6,)
            } else {
                let assign5610_e4374: f64 = (p.p233 * locals.var_ile);
                let assign5610_e4375: f64 = (1.0 + assign5610_e4374);
                (assign5610_e4375,)
            }
        };
        let assign5610_e4377: f64 = (p.p232 * assign5610_e4376);
        (assign5610_e4377,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5610_e4379;
        locals.var_np_p_rv = 0.0;

        let (assign5620_e4383,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p234,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5620_e4383;
        locals.var_toxov_p_rv = 0.0;

        let (assign5630_e4387,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p235,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5630_e4387;
        locals.var_toxovd_p_rv = 0.0;

        let (assign5640_e4391,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p238,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5640_e4391;
        locals.var_nov_p_rv = 0.0;

        let (assign5650_e4395,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5650_e4395;
        locals.var_novd_p_rv = 0.0;

        let (assign5660_e4417,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5660_e4401: f64 = (locals.var_ile).powf(p.p242);
        let assign5660_e4402: f64 = (p.p241 * assign5660_e4401);
        let assign5660_e4403: f64 = (p.p240 + assign5660_e4402);
        let assign5660_e4407: f64 = (p.p243 * locals.var_iwe);
        let assign5660_e4408: f64 = (1.0 + assign5660_e4407);
        let assign5660_e4409: f64 = (assign5660_e4403 * assign5660_e4408);
        let assign5660_e4413: f64 = (p.p244 * locals.var_iae);
        let assign5660_e4414: f64 = (1.0 + assign5660_e4413);
        let assign5660_e4415: f64 = (assign5660_e4409 * assign5660_e4414);
        (assign5660_e4415,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5660_e4417;
        locals.var_ct_p_rv = 0.0;

        let (assign5670_e4421,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5670_e4421;
        locals.var_ctg_p_rv = 0.0;

        let (assign5680_e4425,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5680_e4425;
        locals.var_ctb_p_rv = 0.0;

        let (assign5690_e4429,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5690_e4429;
        locals.var_stct_p_rv = 0.0;

        let (assign5700_e4443,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5700_e4434: f64 = (locals.var_ile).powf(p.p249);
        let assign5700_e4435: f64 = (p.p248 * assign5700_e4434);
        let assign5700_e4439: f64 = (p.p250 * locals.var_iwe);
        let assign5700_e4440: f64 = (1.0 + assign5700_e4439);
        let assign5700_e4441: f64 = (assign5700_e4435 * assign5700_e4440);
        (assign5700_e4441,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5700_e4443;
        locals.var_cf_p_rv = 0.0;

        let (assign5710_e4447,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5710_e4447;
        locals.var_cfd_p_rv = 0.0;

        let (assign5720_e4451,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p251,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5720_e4451;
        locals.var_cfb_p_rv = 0.0;

        let (assign5730_e4465,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5730_e4456: f64 = (locals.var_ile).powf(p.p254);
        let assign5730_e4457: f64 = (p.p253 * assign5730_e4456);
        let assign5730_e4461: f64 = (p.p255 * locals.var_iwe);
        let assign5730_e4462: f64 = (1.0 + assign5730_e4461);
        let assign5730_e4463: f64 = (assign5730_e4457 * assign5730_e4462);
        (assign5730_e4463,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5730_e4465;
        locals.var_psce_p_rv = 0.0;

        let (assign5740_e4469,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p257,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5740_e4469;
        locals.var_psced_p_rv = 0.0;

        let (assign5750_e4473,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p256,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5750_e4473;
        locals.var_psceb_p_rv = 0.0;

        let (assign5760_e4483,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5760_e4479: f64 = (p.p260 * locals.var_iwe);
        let assign5760_e4480: f64 = (1.0 + assign5760_e4479);
        let assign5760_e4481: f64 = (p.p259 * assign5760_e4480);
        (assign5760_e4481,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5760_e4483;
        locals.var_fbet1e_rv = 0.0;

        let (assign5770_e4502,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5770_e4489: f64 = (p.p262 * locals.var_iwe);
        let assign5770_e4490: f64 = (1.0 + assign5770_e4489);
        let (assign5770_e4499,) = {
            if (assign5770_e4490 > 0.001) {
                let assign5770_e4496: f64 = (p.p262 * locals.var_iwe);
                let assign5770_e4497: f64 = (1.0 + assign5770_e4496);
                (assign5770_e4497,)
            } else {
                (0.001,)
            }
        };
        let assign5770_e4500: f64 = (p.p261 * assign5770_e4499);
        (assign5770_e4500,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5770_e4502;
        locals.var_lp1e_rv = 0.0;

        let (assign5780_e4534,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5780_e4507: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5780_e4509: f64 = (assign5780_e4507 / locals.var_le);
        let assign5780_e4512: f64 = (-locals.var_le);
        let assign5780_e4514: f64 = (assign5780_e4512 / locals.var_lp1e);
        let assign5780_e4515: f64 = (assign5780_e4514).exp();
        let assign5780_e4516: f64 = (1.0 - assign5780_e4515);
        let assign5780_e4517: f64 = (assign5780_e4509 * assign5780_e4516);
        let assign5780_e4518: f64 = (1.0 + assign5780_e4517);
        let assign5780_e4521: f64 = (p.p263 * p.p264);
        let assign5780_e4523: f64 = (assign5780_e4521 / locals.var_le);
        let assign5780_e4526: f64 = (-locals.var_le);
        let assign5780_e4528: f64 = (assign5780_e4526 / p.p264);
        let assign5780_e4529: f64 = (assign5780_e4528).exp();
        let assign5780_e4530: f64 = (1.0 - assign5780_e4529);
        let assign5780_e4531: f64 = (assign5780_e4523 * assign5780_e4530);
        let assign5780_e4532: f64 = (assign5780_e4518 + assign5780_e4531);
        (assign5780_e4532,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5780_e4534;
        locals.var_gpe_rv = 0.0;

        let (assign5790_e4543,) = {
    if (locals.var_guard41 != 0.0) {
        let (assign5790_e4541,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5790_e4541,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5790_e4543;
        locals.var_gpe_rv = 0.0;

        let (assign5800_e4562,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5800_e4548: f64 = (p.p265 * locals.var_iwe);
        let assign5800_e4549: f64 = (1.0 + assign5800_e4548);
        let assign5800_e4552: f64 = (p.p266 * locals.var_iwe);
        let assign5800_e4556: f64 = (locals.var_we / p.p267);
        let assign5800_e4557: f64 = (1.0 + assign5800_e4556);
        let assign5800_e4558: f64 = (assign5800_e4557).ln();
        let assign5800_e4559: f64 = (assign5800_e4552 * assign5800_e4558);
        let assign5800_e4560: f64 = (assign5800_e4549 + assign5800_e4559);
        (assign5800_e4560,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5800_e4562;
        locals.var_gwe_rv = 0.0;

        let (assign5810_e4574,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5810_e4566: f64 = (p.p258 * locals.var_we);
        let assign5810_e4569: f64 = (locals.var_gpe * locals.var_le);
        let assign5810_e4570: f64 = (assign5810_e4566 / assign5810_e4569);
        let assign5810_e4572: f64 = (assign5810_e4570 * locals.var_gwe);
        (assign5810_e4572,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5810_e4574;
        locals.var_betn_p_rv = 0.0;

        let (assign5820_e4590,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5820_e4579: f64 = (p.p269 * locals.var_ile);
        let assign5820_e4580: f64 = (p.p268 + assign5820_e4579);
        let assign5820_e4583: f64 = (p.p270 * locals.var_iwe);
        let assign5820_e4584: f64 = (assign5820_e4580 + assign5820_e4583);
        let assign5820_e4587: f64 = (p.p271 * locals.var_iae);
        let assign5820_e4588: f64 = (assign5820_e4584 + assign5820_e4587);
        (assign5820_e4588,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5820_e4590;
        locals.var_stbet_p_rv = 0.0;

        let (assign5830_e4600,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5830_e4596: f64 = (p.p273 * locals.var_iwe);
        let assign5830_e4597: f64 = (1.0 + assign5830_e4596);
        let assign5830_e4598: f64 = (p.p272 * assign5830_e4597);
        (assign5830_e4598,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5830_e4600;
        locals.var_mue_p_rv = 0.0;

        let (assign5840_e4604,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p274,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5840_e4604;
        locals.var_stmue_p_rv = 0.0;

        let (assign5850_e4608,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p275,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5850_e4608;
        locals.var_themu_p_rv = 0.0;

        let (assign5860_e4612,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5860_e4612;
        locals.var_stthemu_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5870_e4634,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5870_e4618: f64 = (locals.var_ile).powf(p.p279);
        let assign5870_e4619: f64 = (p.p278 * assign5870_e4618);
        let assign5870_e4620: f64 = (p.p277 + assign5870_e4619);
        let assign5870_e4624: f64 = (p.p280 * locals.var_iwe);
        let assign5870_e4625: f64 = (1.0 + assign5870_e4624);
        let assign5870_e4626: f64 = (assign5870_e4620 * assign5870_e4625);
        let assign5870_e4630: f64 = (p.p281 * locals.var_iae);
        let assign5870_e4631: f64 = (1.0 + assign5870_e4630);
        let assign5870_e4632: f64 = (assign5870_e4626 * assign5870_e4631);
        (assign5870_e4632,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5870_e4634;
        locals.var_cs_p_rv = 0.0;

        let (assign5880_e4638,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p282,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5880_e4638;
        locals.var_stcs_p_rv = 0.0;

        let (assign5890_e4642,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5890_e4642;
        locals.var_thecs_p_rv = 0.0;

        let (assign5900_e4646,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p284,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5900_e4646;
        locals.var_stthecs_p_rv = 0.0;

        let (assign5910_e4668,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5910_e4652: f64 = (p.p286 * locals.var_ile);
        let assign5910_e4653: f64 = (1.0 + assign5910_e4652);
        let assign5910_e4654: f64 = (p.p285 * assign5910_e4653);
        let assign5910_e4658: f64 = (p.p287 * locals.var_iwe);
        let assign5910_e4659: f64 = (1.0 + assign5910_e4658);
        let assign5910_e4660: f64 = (assign5910_e4654 * assign5910_e4659);
        let assign5910_e4664: f64 = (p.p288 * locals.var_iae);
        let assign5910_e4665: f64 = (1.0 + assign5910_e4664);
        let assign5910_e4666: f64 = (assign5910_e4660 * assign5910_e4665);
        (assign5910_e4666,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5910_e4668;
        locals.var_xcor_p_rv = 0.0;

        let (assign5920_e4672,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p289,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5920_e4672;
        locals.var_stxcor_p_rv = 0.0;

        let (assign5930_e4676,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p290,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5930_e4676;
        locals.var_feta_p_rv = 0.0;

        let (assign5940_e4688,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5940_e4680: f64 = (p.p291 * locals.var_iwe);
        let assign5940_e4684: f64 = (p.p292 * locals.var_iwe);
        let assign5940_e4685: f64 = (1.0 + assign5940_e4684);
        let assign5940_e4686: f64 = (assign5940_e4680 * assign5940_e4685);
        (assign5940_e4686,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5940_e4688;
        locals.var_rs_p_rv = 0.0;

        let (assign5950_e4692,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p293,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5950_e4692;
        locals.var_strs_p_rv = 0.0;

        let (assign5960_e4696,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5960_e4696;
        locals.var_rsb_p_rv = 0.0;

        let (assign5970_e4700,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5970_e4700;
        locals.var_rsg_p_rv = 0.0;

        let (assign5980_e4726,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5980_e4705: f64 = (p.p297 * locals.var_gwe);
        let assign5980_e4707: f64 = (assign5980_e4705 / locals.var_gpe);
        let assign5980_e4710: f64 = (locals.var_ile).powf(p.p298);
        let assign5980_e4711: f64 = (assign5980_e4707 * assign5980_e4710);
        let assign5980_e4712: f64 = (p.p296 + assign5980_e4711);
        let assign5980_e4716: f64 = (p.p299 * locals.var_iwe);
        let assign5980_e4717: f64 = (1.0 + assign5980_e4716);
        let assign5980_e4718: f64 = (assign5980_e4712 * assign5980_e4717);
        let assign5980_e4722: f64 = (p.p300 * locals.var_iae);
        let assign5980_e4723: f64 = (1.0 + assign5980_e4722);
        let assign5980_e4724: f64 = (assign5980_e4718 * assign5980_e4723);
        (assign5980_e4724,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5980_e4726;
        locals.var_thesat_p_rv = 0.0;

        let (assign5990_e4742,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5990_e4731: f64 = (p.p302 * locals.var_ile);
        let assign5990_e4732: f64 = (p.p301 + assign5990_e4731);
        let assign5990_e4735: f64 = (p.p303 * locals.var_iwe);
        let assign5990_e4736: f64 = (assign5990_e4732 + assign5990_e4735);
        let assign5990_e4739: f64 = (p.p304 * locals.var_iae);
        let assign5990_e4740: f64 = (assign5990_e4736 + assign5990_e4739);
        (assign5990_e4740,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5990_e4742;
        locals.var_stthesat_p_rv = 0.0;

        let (assign6000_e4746,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p305,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign6000_e4746;
        locals.var_thesatb_p_rv = 0.0;

        let (assign6010_e4750,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign6010_e4750;
        locals.var_thesatg_p_rv = 0.0;

        let (assign6020_e4754,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign6020_e4754;
        locals.var_thesatt_p_rv = 0.0;

        let (assign6030_e4764,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6030_e4760: f64 = (p.p309 * locals.var_ile);
        let assign6030_e4761: f64 = (1.0 + assign6030_e4760);
        let assign6030_e4762: f64 = (p.p308 / assign6030_e4761);
        (assign6030_e4762,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign6030_e4764;
        locals.var_ax_p_rv = 0.0;

        let (assign6040_e4778,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6040_e4769: f64 = (locals.var_ile).powf(p.p311);
        let assign6040_e4770: f64 = (p.p310 * assign6040_e4769);
        let assign6040_e4774: f64 = (p.p312 * locals.var_iwe);
        let assign6040_e4775: f64 = (1.0 + assign6040_e4774);
        let assign6040_e4776: f64 = (assign6040_e4770 * assign6040_e4775);
        (assign6040_e4776,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign6040_e4778;
        locals.var_alp_p_rv = 0.0;

        let (assign6050_e4784,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6050_e4782: f64 = (locals.var_ile).powf(p.p314);
        (assign6050_e4782,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6050_e4784;
        locals.var_tmpx_rv = 0.0;

        let (assign6060_e4804,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6060_e4788: f64 = (p.p313 * locals.var_tmpx);
        let assign6060_e4792: f64 = (p.p316 * locals.var_iwe);
        let assign6060_e4793: f64 = (1.0 + assign6060_e4792);
        let assign6060_e4794: f64 = (assign6060_e4788 * assign6060_e4793);
        let assign6060_e4798: f64 = (p.p315 * locals.var_ile);
        let assign6060_e4800: f64 = (assign6060_e4798 * locals.var_tmpx);
        let assign6060_e4801: f64 = (1.0 + assign6060_e4800);
        let assign6060_e4802: f64 = (assign6060_e4794 / assign6060_e4801);
        (assign6060_e4802,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign6060_e4804;
        locals.var_alp1_p_rv = 0.0;

        let (assign6070_e4810,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6070_e4808: f64 = (locals.var_ile).powf(p.p318);
        (assign6070_e4808,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6070_e4810;
        locals.var_tmpx_rv = 0.0;

        let (assign6080_e4830,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6080_e4814: f64 = (p.p317 * locals.var_tmpx);
        let assign6080_e4818: f64 = (p.p320 * locals.var_iwe);
        let assign6080_e4819: f64 = (1.0 + assign6080_e4818);
        let assign6080_e4820: f64 = (assign6080_e4814 * assign6080_e4819);
        let assign6080_e4824: f64 = (p.p319 * locals.var_ile);
        let assign6080_e4826: f64 = (assign6080_e4824 * locals.var_tmpx);
        let assign6080_e4827: f64 = (1.0 + assign6080_e4826);
        let assign6080_e4828: f64 = (assign6080_e4820 / assign6080_e4827);
        (assign6080_e4828,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign6080_e4830;
        locals.var_alp2_p_rv = 0.0;

        let (assign6090_e4834,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p321,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign6090_e4834;
        locals.var_vp_p_rv = 0.0;

        let (assign6100_e4850,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6100_e4840: f64 = (p.p323 * locals.var_ile);
        let assign6100_e4841: f64 = (1.0 + assign6100_e4840);
        let assign6100_e4842: f64 = (p.p322 * assign6100_e4841);
        let assign6100_e4846: f64 = (p.p324 * locals.var_iwe);
        let assign6100_e4847: f64 = (1.0 + assign6100_e4846);
        let assign6100_e4848: f64 = (assign6100_e4842 * assign6100_e4847);
        (assign6100_e4848,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign6100_e4850;
        locals.var_a1_p_rv = 0.0;

        let (assign6110_e4854,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p325,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign6110_e4854;
        locals.var_a2_p_rv = 0.0;

        let (assign6120_e4858,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p326,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign6120_e4858;
        locals.var_sta2_p_rv = 0.0;

        let (assign6130_e4874,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6130_e4864: f64 = (p.p328 * locals.var_ile);
        let assign6130_e4865: f64 = (1.0 + assign6130_e4864);
        let assign6130_e4866: f64 = (p.p327 * assign6130_e4865);
        let assign6130_e4870: f64 = (p.p329 * locals.var_iwe);
        let assign6130_e4871: f64 = (1.0 + assign6130_e4870);
        let assign6130_e4872: f64 = (assign6130_e4866 * assign6130_e4871);
        (assign6130_e4872,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign6130_e4874;
        locals.var_a3_p_rv = 0.0;

        let (assign6140_e4890,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6140_e4880: f64 = (p.p331 * locals.var_ile);
        let assign6140_e4881: f64 = (1.0 + assign6140_e4880);
        let assign6140_e4882: f64 = (p.p330 * assign6140_e4881);
        let assign6140_e4886: f64 = (p.p332 * locals.var_iwe);
        let assign6140_e4887: f64 = (1.0 + assign6140_e4886);
        let assign6140_e4888: f64 = (assign6140_e4882 * assign6140_e4887);
        (assign6140_e4888,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign6140_e4890;
        locals.var_a4_p_rv = 0.0;

        let (assign6150_e4894,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p333,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign6150_e4894;
        locals.var_imaxii_p_rv = 0.0;

        let (assign6160_e4898,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p334,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign6160_e4898;
        locals.var_gco_p_rv = 0.0;

        let (assign6170_e4904,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6170_e4902: f64 = (p.p335 / locals.var_iae);
        (assign6170_e4902,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign6170_e4904;
        locals.var_iginv_p_rv = 0.0;

        let (assign6180_e4914,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6180_e4908: f64 = (p.p336 * p.p236);
        let assign6180_e4911: f64 = (1e-6 * locals.var_iwe);
        let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
        (assign6180_e4912,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign6180_e4914;
        locals.var_igov_p_rv = 0.0;

        let (assign6190_e4924,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6190_e4918: f64 = (p.p337 * p.p237);
        let assign6190_e4921: f64 = (1e-6 * locals.var_iwe);
        let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
        (assign6190_e4922,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign6190_e4924;
        locals.var_igovd_p_rv = 0.0;

        let (assign6200_e4928,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign6200_e4928;
        locals.var_stig_p_rv = 0.0;

        let (assign6210_e4932,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign6210_e4932;
        locals.var_gc2_p_rv = 0.0;

        let (assign6220_e4936,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign6220_e4936;
        locals.var_gc3_p_rv = 0.0;

        let (assign6230_e4940,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6230_e4940;
        locals.var_gc2ov_p_rv = 0.0;

        let assign6240_e4942: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6240_e4944: f64 = if assign6240_e4942 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6240_e4944;
        locals.var_guard44_rv = 0.0;

        let (assign6250_e4950,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p341,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6250_e4950;
        locals.var_gc2ov_p_rv = 0.0;

        let (assign6260_e4954,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6260_e4954;
        locals.var_gc3ov_p_rv = 0.0;

        let assign6270_e4956: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6270_e4958: f64 = if assign6270_e4956 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6270_e4958;
        locals.var_guard45_rv = 0.0;

        let (assign6280_e4964,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6280_e4964;
        locals.var_gc3ov_p_rv = 0.0;

        let (assign6290_e4968,) = {
    if (locals.var_guard41 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6290_e4968;
        locals.var_gc2ovd_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign6300_e4970: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6300_e4972: f64 = if assign6300_e4970 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6300_e4972;
        locals.var_guard46_rv = 0.0;

        let (assign6310_e4978,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p343,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6310_e4978;
        locals.var_gc2ovd_p_rv = 0.0;

        let (assign6320_e4982,) = {
    if (locals.var_guard41 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6320_e4982;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign6330_e4984: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6330_e4986: f64 = if assign6330_e4984 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6330_e4986;
        locals.var_guard47_rv = 0.0;

        let (assign6340_e4992,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p344,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6340_e4992;
        locals.var_gc3ovd_p_rv = 0.0;

        let (assign6350_e4996,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p345,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6350_e4996;
        locals.var_chib_p_rv = 0.0;

        let (assign6360_e5006,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6360_e5000: f64 = (p.p346 * p.p236);
        let assign6360_e5003: f64 = (1e-6 * locals.var_iwe);
        let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
        (assign6360_e5004,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6360_e5006;
        locals.var_agidl_p_rv = 0.0;

        let (assign6370_e5016,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6370_e5010: f64 = (p.p347 * p.p237);
        let assign6370_e5013: f64 = (1e-6 * locals.var_iwe);
        let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
        (assign6370_e5014,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6370_e5016;
        locals.var_agidld_p_rv = 0.0;

        let (assign6380_e5020,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p348,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6380_e5020;
        locals.var_bgidl_p_rv = 0.0;

        let (assign6390_e5024,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6390_e5024;
        locals.var_bgidld_p_rv = 0.0;

        let (assign6400_e5028,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6400_e5028;
        locals.var_stbgidl_p_rv = 0.0;

        let (assign6410_e5032,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6410_e5032;
        locals.var_stbgidld_p_rv = 0.0;

        let (assign6420_e5036,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p352,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6420_e5036;
        locals.var_cgidl_p_rv = 0.0;

        let (assign6430_e5040,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p353,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6430_e5040;
        locals.var_cgidld_p_rv = 0.0;

        let (assign6440_e5052,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6440_e5044: f64 = (8.8541878176e-12 * p.p209);
        let assign6440_e5046: f64 = (assign6440_e5044 * locals.var_wecv);
        let assign6440_e5048: f64 = (assign6440_e5046 * locals.var_lecv);
        let assign6440_e5050: f64 = (assign6440_e5048 / p.p208);
        (assign6440_e5050,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6440_e5052;
        locals.var_cox_p_rv = 0.0;

        let (assign6450_e5064,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
        let assign6450_e5058: f64 = (assign6450_e5056 * locals.var_wecv);
        let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
        let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
        (assign6450_e5062,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6450_e5064;
        locals.var_cgov_p_rv = 0.0;

        let (assign6460_e5076,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
        let assign6460_e5070: f64 = (assign6460_e5068 * locals.var_wecv);
        let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
        let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
        (assign6460_e5074,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6460_e5076;
        locals.var_cgovd_p_rv = 0.0;

        let (assign6470_e5094,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6470_e5082: f64 = (locals.var_ile).powf(p.p356);
        let assign6470_e5083: f64 = (p.p355 * assign6470_e5082);
        let assign6470_e5084: f64 = (p.p354 + assign6470_e5083);
        let assign6470_e5087: f64 = (p.p357 * locals.var_iwe);
        let assign6470_e5088: f64 = (assign6470_e5084 + assign6470_e5087);
        let assign6470_e5091: f64 = (p.p358 * locals.var_iae);
        let assign6470_e5092: f64 = (assign6470_e5088 + assign6470_e5091);
        (assign6470_e5092,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6470_e5094;
        locals.var_delvtac_p_rv = 0.0;

        let (assign6480_e5110,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6480_e5099: f64 = (p.p360 * locals.var_ile);
        let assign6480_e5100: f64 = (p.p359 + assign6480_e5099);
        let assign6480_e5103: f64 = (p.p361 * locals.var_iwe);
        let assign6480_e5104: f64 = (assign6480_e5100 + assign6480_e5103);
        let assign6480_e5107: f64 = (p.p362 * locals.var_iae);
        let assign6480_e5108: f64 = (assign6480_e5104 + assign6480_e5107);
        (assign6480_e5108,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6480_e5110;
        locals.var_facneffac_p_rv = 0.0;

        let (assign6490_e5114,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6490_e5114;
        locals.var_thesataco_i_rv = 0.0;

        let assign6500_e5116: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6500_e5118: f64 = if assign6500_e5116 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6500_e5118;
        locals.var_guard48_rv = 0.0;

        let (assign6510_e5124,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p363,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6510_e5124;
        locals.var_thesataco_i_rv = 0.0;

        let (assign6520_e5128,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6520_e5128;
        locals.var_thesatacl_i_rv = 0.0;

        let assign6530_e5130: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6530_e5132: f64 = if assign6530_e5130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6530_e5132;
        locals.var_guard49_rv = 0.0;

        let (assign6540_e5138,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6540_e5138;
        locals.var_thesatacl_i_rv = 0.0;

        let (assign6550_e5142,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6550_e5142;
        locals.var_thesataclexp_i_rv = 0.0;

        let assign6560_e5144: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6560_e5146: f64 = if assign6560_e5144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign6560_e5146;
        locals.var_guard50_rv = 0.0;

        let (assign6570_e5152,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard50 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6570_e5152;
        locals.var_thesataclexp_i_rv = 0.0;

        let (assign6580_e5156,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6580_e5156;
        locals.var_thesatacw_i_rv = 0.0;

        let assign6590_e5158: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6590_e5160: f64 = if assign6590_e5158 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign6590_e5160;
        locals.var_guard51_rv = 0.0;

        let (assign6600_e5166,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard51 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6600_e5166;
        locals.var_thesatacw_i_rv = 0.0;

        let (assign6610_e5170,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p300,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6610_e5170;
        locals.var_thesataclw_i_rv = 0.0;

        let assign6620_e5172: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6620_e5174: f64 = if assign6620_e5172 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign6620_e5174;
        locals.var_guard52_rv = 0.0;

        let (assign6630_e5180,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard52 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6630_e5180;
        locals.var_thesataclw_i_rv = 0.0;

        let (assign6640_e5206,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6640_e5185: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6640_e5187: f64 = (assign6640_e5185 / locals.var_gpe);
        let assign6640_e5190: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6640_e5191: f64 = (assign6640_e5187 * assign6640_e5190);
        let assign6640_e5192: f64 = (locals.var_thesataco_i + assign6640_e5191);
        let assign6640_e5196: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6640_e5197: f64 = (1.0 + assign6640_e5196);
        let assign6640_e5198: f64 = (assign6640_e5192 * assign6640_e5197);
        let assign6640_e5202: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6640_e5203: f64 = (1.0 + assign6640_e5202);
        let assign6640_e5204: f64 = (assign6640_e5198 * assign6640_e5203);
        (assign6640_e5204,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6640_e5206;
        locals.var_thesatac_p_rv = 0.0;

        let (assign6650_e5210,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p308,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6650_e5210;
        locals.var_axaco_i_rv = 0.0;

        let assign6660_e5212: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6660_e5214: f64 = if assign6660_e5212 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign6660_e5214;
        locals.var_guard53_rv = 0.0;

        let (assign6670_e5220,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard53 != 0.0)) {
        (p.p368,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6670_e5220;
        locals.var_axaco_i_rv = 0.0;

        let (assign6680_e5224,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p309,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6680_e5224;
        locals.var_axacl_i_rv = 0.0;

        let assign6690_e5226: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6690_e5228: f64 = if assign6690_e5226 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign6690_e5228;
        locals.var_guard54_rv = 0.0;

        let (assign6700_e5234,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard54 != 0.0)) {
        (p.p369,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6700_e5234;
        locals.var_axacl_i_rv = 0.0;

        let (assign6710_e5244,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6710_e5240: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6710_e5241: f64 = (1.0 + assign6710_e5240);
        let assign6710_e5242: f64 = (locals.var_axaco_i / assign6710_e5241);
        (assign6710_e5242,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6710_e5244;
        locals.var_axac_p_rv = 0.0;

        let (assign6720_e5258,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6720_e5249: f64 = (locals.var_ile).powf(p.p371);
        let assign6720_e5250: f64 = (p.p370 * assign6720_e5249);
        let assign6720_e5254: f64 = (p.p372 * locals.var_iwe);
        let assign6720_e5255: f64 = (1.0 + assign6720_e5254);
        let assign6720_e5256: f64 = (assign6720_e5250 * assign6720_e5255);
        (assign6720_e5256,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6720_e5258;
        locals.var_alpac_p_rv = 0.0;

        let (assign6730_e5264,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6730_e5262: f64 = (locals.var_ile).powf(p.p374);
        (assign6730_e5262,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6730_e5264;
        locals.var_tmpx_rv = 0.0;

        let (assign6740_e5284,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6740_e5268: f64 = (p.p373 * locals.var_tmpx);
        let assign6740_e5272: f64 = (p.p376 * locals.var_iwe);
        let assign6740_e5273: f64 = (1.0 + assign6740_e5272);
        let assign6740_e5274: f64 = (assign6740_e5268 * assign6740_e5273);
        let assign6740_e5278: f64 = (p.p375 * locals.var_ile);
        let assign6740_e5280: f64 = (assign6740_e5278 * locals.var_tmpx);
        let assign6740_e5281: f64 = (1.0 + assign6740_e5280);
        let assign6740_e5282: f64 = (assign6740_e5274 / assign6740_e5281);
        (assign6740_e5282,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6740_e5284;
        locals.var_alp1ac_p_rv = 0.0;

        let (assign6750_e5288,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p377,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6750_e5288;
        locals.var_fcgovacc_p_rv = 0.0;

        let (assign6760_e5292,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p378,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6760_e5292;
        locals.var_fcgovaccd_p_rv = 0.0;

        let (assign6770_e5296,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p379,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6770_e5296;
        locals.var_cgovaccg_p_rv = 0.0;

        let (assign6780_e5302,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6780_e5300: f64 = (p.p380 * locals.var_iilcv);
        (assign6780_e5300,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6780_e5302;
        locals.var_cgbov_p_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6790_e5308,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6790_e5306: f64 = (p.p381 * locals.var_iiwecv);
        (assign6790_e5306,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6790_e5308;
        locals.var_cinr_p_rv = 0.0;

        let (assign6800_e5314,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6800_e5312: f64 = (p.p382 * locals.var_iiwecv);
        (assign6800_e5312,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6800_e5314;
        locals.var_cinrd_p_rv = 0.0;

        let (assign6810_e5318,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p383,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6810_e5318;
        locals.var_dvfbinr_p_rv = 0.0;

        let (assign6820_e5322,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6820_e5322;
        locals.var_fcinrdep_p_rv = 0.0;

        let (assign6830_e5326,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p385,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6830_e5326;
        locals.var_fcinracc_p_rv = 0.0;

        let (assign6840_e5330,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p386,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6840_e5330;
        locals.var_axinr_p_rv = 0.0;

        let (assign6870_e5352,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6870_e5347: f64 = (2.0 * p.p395);
        let assign6870_e5349: f64 = (assign6870_e5347 / locals.var_le);
        let assign6870_e5350: f64 = (1.0 - assign6870_e5349);
        (assign6870_e5350,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6870_e5352;
        locals.var_temp0_rv = 0.0;

        let (assign6900_e5373,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p389,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6900_e5373;
        locals.var_fnt_p_rv = 0.0;

        let (assign6960_e5423,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6960_e5417: f64 = (2.0 * p.p397);
        let assign6960_e5420: f64 = (p.p398 * locals.var_we);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6960_e5423;
        locals.var_we_edge_rv = 0.0;

        let (assign6990_e5439,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p399,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6990_e5439;
        locals.var_vfbedge_p_rv = 0.0;

        let (assign7000_e5455,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7000_e5444: f64 = (p.p401 * locals.var_ile);
        let assign7000_e5445: f64 = (p.p400 + assign7000_e5444);
        let assign7000_e5448: f64 = (p.p402 * locals.var_iwe);
        let assign7000_e5449: f64 = (assign7000_e5445 + assign7000_e5448);
        let assign7000_e5452: f64 = (p.p403 * locals.var_iae);
        let assign7000_e5453: f64 = (assign7000_e5449 + assign7000_e5452);
        (assign7000_e5453,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign7000_e5455;
        locals.var_stvfbedge_p_rv = 0.0;

        let (assign7010_e5473,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7010_e5461: f64 = (locals.var_ile).powf(p.p406);
        let assign7010_e5462: f64 = (p.p405 * assign7010_e5461);
        let assign7010_e5463: f64 = (p.p404 + assign7010_e5462);
        let assign7010_e5466: f64 = (p.p407 * locals.var_iwe);
        let assign7010_e5467: f64 = (assign7010_e5463 + assign7010_e5466);
        let assign7010_e5470: f64 = (p.p408 * locals.var_iae);
        let assign7010_e5471: f64 = (assign7010_e5467 + assign7010_e5470);
        (assign7010_e5471,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign7010_e5473;
        locals.var_dphibedge_p_rv = 0.0;

        let (assign7020_e5497,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7020_e5480: f64 = (locals.var_ile).powf(p.p411);
        let assign7020_e5481: f64 = (p.p410 * assign7020_e5480);
        let assign7020_e5482: f64 = (1.0 + assign7020_e5481);
        let assign7020_e5483: f64 = (p.p409 * assign7020_e5482);
        let assign7020_e5487: f64 = (p.p412 * locals.var_iwe);
        let assign7020_e5488: f64 = (1.0 + assign7020_e5487);
        let assign7020_e5489: f64 = (assign7020_e5483 * assign7020_e5488);
        let assign7020_e5493: f64 = (p.p413 * locals.var_iae);
        let assign7020_e5494: f64 = (1.0 + assign7020_e5493);
        let assign7020_e5495: f64 = (assign7020_e5489 * assign7020_e5494);
        (assign7020_e5495,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign7020_e5497;
        locals.var_neffedge_p_rv = 0.0;

        let (assign7030_e5507,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7030_e5503: f64 = (locals.var_ile).powf(p.p416);
        let assign7030_e5504: f64 = (p.p415 * assign7030_e5503);
        let assign7030_e5505: f64 = (p.p414 + assign7030_e5504);
        (assign7030_e5505,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign7030_e5507;
        locals.var_ctedge_p_rv = 0.0;

        let (assign7040_e5525,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7040_e5512: f64 = (p.p417 * p.p418);
        let assign7040_e5514: f64 = (assign7040_e5512 / locals.var_le);
        let assign7040_e5517: f64 = (-locals.var_le);
        let assign7040_e5519: f64 = (assign7040_e5517 / p.p418);
        let assign7040_e5520: f64 = (assign7040_e5519).exp();
        let assign7040_e5521: f64 = (1.0 - assign7040_e5520);
        let assign7040_e5522: f64 = (assign7040_e5514 * assign7040_e5521);
        let assign7040_e5523: f64 = (1.0 + assign7040_e5522);
        (assign7040_e5523,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign7040_e5525;
        locals.var_gpe_edge_rv = 0.0;

        let (assign7050_e5534,) = {
    if (locals.var_guard41 != 0.0) {
        let (assign7050_e5532,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign7050_e5532,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign7050_e5534;
        locals.var_gpe_edge_rv = 0.0;

        let (assign7060_e5550,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7060_e5538: f64 = (p.p258 * locals.var_we_edge);
        let assign7060_e5541: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign7060_e5542: f64 = (assign7060_e5538 / assign7060_e5541);
        let assign7060_e5546: f64 = (p.p419 * locals.var_iwe);
        let assign7060_e5547: f64 = (1.0 + assign7060_e5546);
        let assign7060_e5548: f64 = (assign7060_e5542 * assign7060_e5547);
        (assign7060_e5548,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign7060_e5550;
        locals.var_betnedge_p_rv = 0.0;

        let (assign7070_e5566,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7070_e5555: f64 = (p.p421 * locals.var_ile);
        let assign7070_e5556: f64 = (p.p420 + assign7070_e5555);
        let assign7070_e5559: f64 = (p.p422 * locals.var_iwe);
        let assign7070_e5560: f64 = (assign7070_e5556 + assign7070_e5559);
        let assign7070_e5563: f64 = (p.p423 * locals.var_iae);
        let assign7070_e5564: f64 = (assign7070_e5560 + assign7070_e5563);
        (assign7070_e5564,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign7070_e5566;
        locals.var_stbetedge_p_rv = 0.0;

        let (assign7080_e5580,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7080_e5571: f64 = (locals.var_ile).powf(p.p425);
        let assign7080_e5572: f64 = (p.p424 * assign7080_e5571);
        let assign7080_e5576: f64 = (p.p426 * locals.var_iwe);
        let assign7080_e5577: f64 = (1.0 + assign7080_e5576);
        let assign7080_e5578: f64 = (assign7080_e5572 * assign7080_e5577);
        (assign7080_e5578,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign7080_e5580;
        locals.var_psceedge_p_rv = 0.0;

        let (assign7090_e5584,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p427,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign7090_e5584;
        locals.var_pscebedge_p_rv = 0.0;

        let (assign7100_e5588,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p428,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign7100_e5588;
        locals.var_pscededge_p_rv = 0.0;

        let (assign7110_e5602,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7110_e5593: f64 = (locals.var_ile).powf(p.p430);
        let assign7110_e5594: f64 = (p.p429 * assign7110_e5593);
        let assign7110_e5598: f64 = (p.p431 * locals.var_iwe);
        let assign7110_e5599: f64 = (1.0 + assign7110_e5598);
        let assign7110_e5600: f64 = (assign7110_e5594 * assign7110_e5599);
        (assign7110_e5600,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign7110_e5602;
        locals.var_cfedge_p_rv = 0.0;

        let (assign7120_e5606,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p433,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign7120_e5606;
        locals.var_cfdedge_p_rv = 0.0;

        let (assign7130_e5610,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p432,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign7130_e5610;
        locals.var_cfbedge_p_rv = 0.0;

        let (assign7190_e5652,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7190_e5641: f64 = (p.p815 * locals.var_ile);
        let assign7190_e5642: f64 = (p.p814 + assign7190_e5641);
        let assign7190_e5645: f64 = (p.p816 * locals.var_iwe);
        let assign7190_e5646: f64 = (assign7190_e5642 + assign7190_e5645);
        let assign7190_e5649: f64 = (p.p817 * locals.var_iae);
        let assign7190_e5650: f64 = (assign7190_e5646 + assign7190_e5649);
        (assign7190_e5650,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign7190_e5652;
        locals.var_kvthowe_rv = 0.0;

        let (assign7200_e5668,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7200_e5657: f64 = (p.p819 * locals.var_ile);
        let assign7200_e5658: f64 = (p.p818 + assign7200_e5657);
        let assign7200_e5661: f64 = (p.p820 * locals.var_iwe);
        let assign7200_e5662: f64 = (assign7200_e5658 + assign7200_e5661);
        let assign7200_e5665: f64 = (p.p821 * locals.var_iae);
        let assign7200_e5666: f64 = (assign7200_e5662 + assign7200_e5665);
        (assign7200_e5666,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign7200_e5668;
        locals.var_kuowe_rv = 0.0;

        let (assign7320_e5767,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p450,)
    } else {
        (locals.var_munqs_p,)
    }
};
        locals.var_munqs_p = assign7320_e5767;
        locals.var_munqs_p_rv = 0.0;

        let assign7330_e5786: f64 = if (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7330_e5786;
        locals.var_guard56_rv = 0.0;

        let (assign7340_e5804,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7340_e5793: f64 = (p.p452 * locals.var_ile);
        let assign7340_e5794: f64 = (p.p451 + assign7340_e5793);
        let assign7340_e5797: f64 = (p.p453 * locals.var_iwe);
        let assign7340_e5798: f64 = (assign7340_e5794 + assign7340_e5797);
        let assign7340_e5801: f64 = (p.p454 * locals.var_iae);
        let assign7340_e5802: f64 = (assign7340_e5798 + assign7340_e5801);
        (assign7340_e5802,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7340_e5804;
        locals.var_vfb_p_rv = 0.0;

        let assign7350_e5823: f64 = if (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7350_e5823;
        locals.var_guard57_rv = 0.0;

        let (assign7360_e5841,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7360_e5830: f64 = (p.p456 * locals.var_ile);
        let assign7360_e5831: f64 = (p.p455 + assign7360_e5830);
        let assign7360_e5834: f64 = (p.p457 * locals.var_iwe);
        let assign7360_e5835: f64 = (assign7360_e5831 + assign7360_e5834);
        let assign7360_e5838: f64 = (p.p458 * locals.var_iae);
        let assign7360_e5839: f64 = (assign7360_e5835 + assign7360_e5838);
        (assign7360_e5839,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7360_e5841;
        locals.var_stvfb_p_rv = 0.0;

        let assign7370_e5860: f64 = if (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7370_e5860;
        locals.var_guard58_rv = 0.0;

        let (assign7380_e5878,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7380_e5867: f64 = (p.p460 * locals.var_ile);
        let assign7380_e5868: f64 = (p.p459 + assign7380_e5867);
        let assign7380_e5871: f64 = (p.p461 * locals.var_iwe);
        let assign7380_e5872: f64 = (assign7380_e5868 + assign7380_e5871);
        let assign7380_e5875: f64 = (p.p462 * locals.var_iae);
        let assign7380_e5876: f64 = (assign7380_e5872 + assign7380_e5875);
        (assign7380_e5876,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7380_e5878;
        locals.var_neff_p_rv = 0.0;

        let assign7390_e5897: f64 = if (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7390_e5897;
        locals.var_guard59_rv = 0.0;

        let (assign7400_e5915,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7400_e5904: f64 = (p.p464 * locals.var_ile);
        let assign7400_e5905: f64 = (p.p463 + assign7400_e5904);
        let assign7400_e5908: f64 = (p.p465 * locals.var_iwe);
        let assign7400_e5909: f64 = (assign7400_e5905 + assign7400_e5908);
        let assign7400_e5912: f64 = (p.p466 * locals.var_iae);
        let assign7400_e5913: f64 = (assign7400_e5909 + assign7400_e5912);
        (assign7400_e5913,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7400_e5915;
        locals.var_gfacnud_p_rv = 0.0;

        let assign7410_e5934: f64 = if (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7410_e5934;
        locals.var_guard60_rv = 0.0;

        let (assign7420_e5952,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7420_e5941: f64 = (p.p468 * locals.var_ile);
        let assign7420_e5942: f64 = (p.p467 + assign7420_e5941);
        let assign7420_e5945: f64 = (p.p469 * locals.var_iwe);
        let assign7420_e5946: f64 = (assign7420_e5942 + assign7420_e5945);
        let assign7420_e5949: f64 = (p.p470 * locals.var_iae);
        let assign7420_e5950: f64 = (assign7420_e5946 + assign7420_e5949);
        (assign7420_e5950,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7420_e5952;
        locals.var_vsbnud_p_rv = 0.0;

        let assign7430_e5971: f64 = if (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7430_e5971;
        locals.var_guard61_rv = 0.0;

        let (assign7440_e5989,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7440_e5978: f64 = (p.p472 * locals.var_ile);
        let assign7440_e5979: f64 = (p.p471 + assign7440_e5978);
        let assign7440_e5982: f64 = (p.p473 * locals.var_iwe);
        let assign7440_e5983: f64 = (assign7440_e5979 + assign7440_e5982);
        let assign7440_e5986: f64 = (p.p474 * locals.var_iae);
        let assign7440_e5987: f64 = (assign7440_e5983 + assign7440_e5986);
        (assign7440_e5987,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7440_e5989;
        locals.var_dphib_p_rv = 0.0;

        let assign7450_e6008: f64 = if (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7450_e6008;
        locals.var_guard62_rv = 0.0;

        let (assign7460_e6026,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7460_e6015: f64 = (p.p476 * locals.var_ile);
        let assign7460_e6016: f64 = (p.p475 + assign7460_e6015);
        let assign7460_e6019: f64 = (p.p477 * locals.var_iwe);
        let assign7460_e6020: f64 = (assign7460_e6016 + assign7460_e6019);
        let assign7460_e6023: f64 = (p.p478 * locals.var_iae);
        let assign7460_e6024: f64 = (assign7460_e6020 + assign7460_e6023);
        (assign7460_e6024,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7460_e6026;
        locals.var_np_p_rv = 0.0;

        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7470_e6045;
        locals.var_guard63_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7480_e6063,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7480_e6052: f64 = (p.p480 * locals.var_ile);
        let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
        let assign7480_e6056: f64 = (p.p481 * locals.var_iwe);
        let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
        let assign7480_e6060: f64 = (p.p482 * locals.var_iae);
        let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
        (assign7480_e6061,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7480_e6063;
        locals.var_nov_p_rv = 0.0;

        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7490_e6082;
        locals.var_guard64_rv = 0.0;

        let (assign7500_e6100,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7500_e6089: f64 = (p.p484 * locals.var_ile);
        let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
        let assign7500_e6093: f64 = (p.p485 * locals.var_iwe);
        let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
        let assign7500_e6097: f64 = (p.p486 * locals.var_iae);
        let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
        (assign7500_e6098,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7500_e6100;
        locals.var_novd_p_rv = 0.0;

        let assign7510_e6119: f64 = if (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7510_e6119;
        locals.var_guard65_rv = 0.0;

        let (assign7520_e6137,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7520_e6126: f64 = (p.p488 * locals.var_ile);
        let assign7520_e6127: f64 = (p.p487 + assign7520_e6126);
        let assign7520_e6130: f64 = (p.p489 * locals.var_iwe);
        let assign7520_e6131: f64 = (assign7520_e6127 + assign7520_e6130);
        let assign7520_e6134: f64 = (p.p490 * locals.var_iae);
        let assign7520_e6135: f64 = (assign7520_e6131 + assign7520_e6134);
        (assign7520_e6135,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7520_e6137;
        locals.var_ct_p_rv = 0.0;

        let assign7530_e6156: f64 = if (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7530_e6156;
        locals.var_guard66_rv = 0.0;

        let (assign7540_e6174,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7540_e6163: f64 = (p.p496 * locals.var_ile);
        let assign7540_e6164: f64 = (p.p495 + assign7540_e6163);
        let assign7540_e6167: f64 = (p.p497 * locals.var_iwe);
        let assign7540_e6168: f64 = (assign7540_e6164 + assign7540_e6167);
        let assign7540_e6171: f64 = (p.p498 * locals.var_iae);
        let assign7540_e6172: f64 = (assign7540_e6168 + assign7540_e6171);
        (assign7540_e6172,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7540_e6174;
        locals.var_ctg_p_rv = 0.0;

        let assign7550_e6193: f64 = if (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7550_e6193;
        locals.var_guard67_rv = 0.0;

        let (assign7560_e6211,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7560_e6200: f64 = (p.p492 * locals.var_ile);
        let assign7560_e6201: f64 = (p.p491 + assign7560_e6200);
        let assign7560_e6204: f64 = (p.p493 * locals.var_iwe);
        let assign7560_e6205: f64 = (assign7560_e6201 + assign7560_e6204);
        let assign7560_e6208: f64 = (p.p494 * locals.var_iae);
        let assign7560_e6209: f64 = (assign7560_e6205 + assign7560_e6208);
        (assign7560_e6209,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7560_e6211;
        locals.var_ctb_p_rv = 0.0;

        let assign7570_e6230: f64 = if (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7570_e6230;
        locals.var_guard68_rv = 0.0;

        let (assign7580_e6248,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7580_e6237: f64 = (p.p500 * locals.var_ile);
        let assign7580_e6238: f64 = (p.p499 + assign7580_e6237);
        let assign7580_e6241: f64 = (p.p501 * locals.var_iwe);
        let assign7580_e6242: f64 = (assign7580_e6238 + assign7580_e6241);
        let assign7580_e6245: f64 = (p.p502 * locals.var_iae);
        let assign7580_e6246: f64 = (assign7580_e6242 + assign7580_e6245);
        (assign7580_e6246,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7580_e6248;
        locals.var_stct_p_rv = 0.0;

        let assign7590_e6267: f64 = if (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7590_e6267;
        locals.var_guard69_rv = 0.0;

        let (assign7600_e6287,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7600_e6275: f64 = (p.p504 * locals.var_ile);
        let assign7600_e6276: f64 = (p.p503 + assign7600_e6275);
        let assign7600_e6279: f64 = (p.p505 * locals.var_iwe);
        let assign7600_e6280: f64 = (assign7600_e6276 + assign7600_e6279);
        let assign7600_e6283: f64 = (p.p506 * locals.var_iae);
        let assign7600_e6284: f64 = (assign7600_e6280 + assign7600_e6283);
        let assign7600_e6285: f64 = (locals.var_ile2 * assign7600_e6284);
        (assign7600_e6285,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7600_e6287;
        locals.var_cf_p_rv = 0.0;

        let assign7610_e6306: f64 = if (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7610_e6306;
        locals.var_guard70_rv = 0.0;

        let (assign7620_e6324,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7620_e6313: f64 = (p.p512 * locals.var_ile);
        let assign7620_e6314: f64 = (p.p511 + assign7620_e6313);
        let assign7620_e6317: f64 = (p.p513 * locals.var_iwe);
        let assign7620_e6318: f64 = (assign7620_e6314 + assign7620_e6317);
        let assign7620_e6321: f64 = (p.p514 * locals.var_iae);
        let assign7620_e6322: f64 = (assign7620_e6318 + assign7620_e6321);
        (assign7620_e6322,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7620_e6324;
        locals.var_cfd_p_rv = 0.0;

        let assign7630_e6343: f64 = if (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7630_e6343;
        locals.var_guard71_rv = 0.0;

        let (assign7640_e6361,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7640_e6350: f64 = (p.p508 * locals.var_ile);
        let assign7640_e6351: f64 = (p.p507 + assign7640_e6350);
        let assign7640_e6354: f64 = (p.p509 * locals.var_iwe);
        let assign7640_e6355: f64 = (assign7640_e6351 + assign7640_e6354);
        let assign7640_e6358: f64 = (p.p510 * locals.var_iae);
        let assign7640_e6359: f64 = (assign7640_e6355 + assign7640_e6358);
        (assign7640_e6359,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7640_e6361;
        locals.var_cfb_p_rv = 0.0;

        let assign7650_e6380: f64 = if (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7650_e6380;
        locals.var_guard72_rv = 0.0;

        let (assign7660_e6400,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7660_e6388: f64 = (p.p516 * locals.var_ile);
        let assign7660_e6389: f64 = (p.p515 + assign7660_e6388);
        let assign7660_e6392: f64 = (p.p517 * locals.var_iwe);
        let assign7660_e6393: f64 = (assign7660_e6389 + assign7660_e6392);
        let assign7660_e6396: f64 = (p.p518 * locals.var_iae);
        let assign7660_e6397: f64 = (assign7660_e6393 + assign7660_e6396);
        let assign7660_e6398: f64 = (locals.var_ile2 * assign7660_e6397);
        (assign7660_e6398,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7660_e6400;
        locals.var_psce_p_rv = 0.0;

        let assign7670_e6419: f64 = if (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7670_e6419;
        locals.var_guard73_rv = 0.0;

        let (assign7680_e6437,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7680_e6426: f64 = (p.p524 * locals.var_ile);
        let assign7680_e6427: f64 = (p.p523 + assign7680_e6426);
        let assign7680_e6430: f64 = (p.p525 * locals.var_iwe);
        let assign7680_e6431: f64 = (assign7680_e6427 + assign7680_e6430);
        let assign7680_e6434: f64 = (p.p526 * locals.var_iae);
        let assign7680_e6435: f64 = (assign7680_e6431 + assign7680_e6434);
        (assign7680_e6435,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7680_e6437;
        locals.var_psced_p_rv = 0.0;

        let assign7690_e6456: f64 = if (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7690_e6456;
        locals.var_guard74_rv = 0.0;

        let (assign7700_e6474,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7700_e6463: f64 = (p.p520 * locals.var_ile);
        let assign7700_e6464: f64 = (p.p519 + assign7700_e6463);
        let assign7700_e6467: f64 = (p.p521 * locals.var_iwe);
        let assign7700_e6468: f64 = (assign7700_e6464 + assign7700_e6467);
        let assign7700_e6471: f64 = (p.p522 * locals.var_iae);
        let assign7700_e6472: f64 = (assign7700_e6468 + assign7700_e6471);
        (assign7700_e6472,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7700_e6474;
        locals.var_psceb_p_rv = 0.0;

        let assign7710_e6493: f64 = if (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7710_e6493;
        locals.var_guard75_rv = 0.0;

        let (assign7720_e6515,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7720_e6499: f64 = (locals.var_we / locals.var_le);
        let assign7720_e6503: f64 = (p.p528 * locals.var_ile);
        let assign7720_e6504: f64 = (p.p527 + assign7720_e6503);
        let assign7720_e6507: f64 = (p.p529 * locals.var_iwe);
        let assign7720_e6508: f64 = (assign7720_e6504 + assign7720_e6507);
        let assign7720_e6511: f64 = (p.p530 * locals.var_iae);
        let assign7720_e6512: f64 = (assign7720_e6508 + assign7720_e6511);
        let assign7720_e6513: f64 = (assign7720_e6499 * assign7720_e6512);
        (assign7720_e6513,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7720_e6515;
        locals.var_betn_p_rv = 0.0;

        let assign7730_e6534: f64 = if (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7730_e6534;
        locals.var_guard76_rv = 0.0;

        let (assign7740_e6552,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7740_e6541: f64 = (p.p532 * locals.var_ile);
        let assign7740_e6542: f64 = (p.p531 + assign7740_e6541);
        let assign7740_e6545: f64 = (p.p533 * locals.var_iwe);
        let assign7740_e6546: f64 = (assign7740_e6542 + assign7740_e6545);
        let assign7740_e6549: f64 = (p.p534 * locals.var_iae);
        let assign7740_e6550: f64 = (assign7740_e6546 + assign7740_e6549);
        (assign7740_e6550,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7740_e6552;
        locals.var_stbet_p_rv = 0.0;

        let assign7750_e6571: f64 = if (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7750_e6571;
        locals.var_guard77_rv = 0.0;

        let (assign7760_e6589,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7760_e6578: f64 = (p.p536 * locals.var_ile);
        let assign7760_e6579: f64 = (p.p535 + assign7760_e6578);
        let assign7760_e6582: f64 = (p.p537 * locals.var_iwe);
        let assign7760_e6583: f64 = (assign7760_e6579 + assign7760_e6582);
        let assign7760_e6586: f64 = (p.p538 * locals.var_iae);
        let assign7760_e6587: f64 = (assign7760_e6583 + assign7760_e6586);
        (assign7760_e6587,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7760_e6589;
        locals.var_mue_p_rv = 0.0;

        let assign7770_e6608: f64 = if (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7770_e6608;
        locals.var_guard78_rv = 0.0;

        let (assign7780_e6626,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7780_e6615: f64 = (p.p540 * locals.var_ile);
        let assign7780_e6616: f64 = (p.p539 + assign7780_e6615);
        let assign7780_e6619: f64 = (p.p541 * locals.var_iwe);
        let assign7780_e6620: f64 = (assign7780_e6616 + assign7780_e6619);
        let assign7780_e6623: f64 = (p.p542 * locals.var_iae);
        let assign7780_e6624: f64 = (assign7780_e6620 + assign7780_e6623);
        (assign7780_e6624,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7780_e6626;
        locals.var_themu_p_rv = 0.0;

        let assign7790_e6645: f64 = if (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7790_e6645;
        locals.var_guard79_rv = 0.0;

        let (assign7800_e6663,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7800_e6652: f64 = (p.p544 * locals.var_ile);
        let assign7800_e6653: f64 = (p.p543 + assign7800_e6652);
        let assign7800_e6656: f64 = (p.p545 * locals.var_iwe);
        let assign7800_e6657: f64 = (assign7800_e6653 + assign7800_e6656);
        let assign7800_e6660: f64 = (p.p546 * locals.var_iae);
        let assign7800_e6661: f64 = (assign7800_e6657 + assign7800_e6660);
        (assign7800_e6661,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7800_e6663;
        locals.var_cs_p_rv = 0.0;

        let assign7810_e6682: f64 = if (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7810_e6682;
        locals.var_guard80_rv = 0.0;

        let (assign7820_e6700,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7820_e6689: f64 = (p.p548 * locals.var_ile);
        let assign7820_e6690: f64 = (p.p547 + assign7820_e6689);
        let assign7820_e6693: f64 = (p.p549 * locals.var_iwe);
        let assign7820_e6694: f64 = (assign7820_e6690 + assign7820_e6693);
        let assign7820_e6697: f64 = (p.p550 * locals.var_iae);
        let assign7820_e6698: f64 = (assign7820_e6694 + assign7820_e6697);
        (assign7820_e6698,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7820_e6700;
        locals.var_thecs_p_rv = 0.0;

        let assign7830_e6719: f64 = if (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7830_e6719;
        locals.var_guard81_rv = 0.0;

        let (assign7840_e6737,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7840_e6726: f64 = (p.p552 * locals.var_ile);
        let assign7840_e6727: f64 = (p.p551 + assign7840_e6726);
        let assign7840_e6730: f64 = (p.p553 * locals.var_iwe);
        let assign7840_e6731: f64 = (assign7840_e6727 + assign7840_e6730);
        let assign7840_e6734: f64 = (p.p554 * locals.var_iae);
        let assign7840_e6735: f64 = (assign7840_e6731 + assign7840_e6734);
        (assign7840_e6735,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7840_e6737;
        locals.var_xcor_p_rv = 0.0;

        let assign7850_e6756: f64 = if (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7850_e6756;
        locals.var_guard82_rv = 0.0;

        let (assign7860_e6776,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7860_e6764: f64 = (p.p556 * locals.var_ile);
        let assign7860_e6765: f64 = (p.p555 + assign7860_e6764);
        let assign7860_e6768: f64 = (p.p557 * locals.var_iwe);
        let assign7860_e6769: f64 = (assign7860_e6765 + assign7860_e6768);
        let assign7860_e6772: f64 = (p.p558 * locals.var_iae);
        let assign7860_e6773: f64 = (assign7860_e6769 + assign7860_e6772);
        let assign7860_e6774: f64 = (locals.var_iwe * assign7860_e6773);
        (assign7860_e6774,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7860_e6776;
        locals.var_rs_p_rv = 0.0;

        let assign7870_e6795: f64 = if (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7870_e6795;
        locals.var_guard83_rv = 0.0;

        let (assign7880_e6813,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7880_e6802: f64 = (p.p560 * locals.var_ile);
        let assign7880_e6803: f64 = (p.p559 + assign7880_e6802);
        let assign7880_e6806: f64 = (p.p561 * locals.var_iwe);
        let assign7880_e6807: f64 = (assign7880_e6803 + assign7880_e6806);
        let assign7880_e6810: f64 = (p.p562 * locals.var_iae);
        let assign7880_e6811: f64 = (assign7880_e6807 + assign7880_e6810);
        (assign7880_e6811,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7880_e6813;
        locals.var_strs_p_rv = 0.0;

        let assign7890_e6832: f64 = if (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7890_e6832;
        locals.var_guard84_rv = 0.0;

        let (assign7900_e6850,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7900_e6839: f64 = (p.p564 * locals.var_ile);
        let assign7900_e6840: f64 = (p.p563 + assign7900_e6839);
        let assign7900_e6843: f64 = (p.p565 * locals.var_iwe);
        let assign7900_e6844: f64 = (assign7900_e6840 + assign7900_e6843);
        let assign7900_e6847: f64 = (p.p566 * locals.var_iae);
        let assign7900_e6848: f64 = (assign7900_e6844 + assign7900_e6847);
        (assign7900_e6848,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7900_e6850;
        locals.var_rsb_p_rv = 0.0;

        let assign7910_e6869: f64 = if (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7910_e6869;
        locals.var_guard85_rv = 0.0;

        let (assign7920_e6887,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7920_e6876: f64 = (p.p568 * locals.var_ile);
        let assign7920_e6877: f64 = (p.p567 + assign7920_e6876);
        let assign7920_e6880: f64 = (p.p569 * locals.var_iwe);
        let assign7920_e6881: f64 = (assign7920_e6877 + assign7920_e6880);
        let assign7920_e6884: f64 = (p.p570 * locals.var_iae);
        let assign7920_e6885: f64 = (assign7920_e6881 + assign7920_e6884);
        (assign7920_e6885,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7920_e6887;
        locals.var_rsg_p_rv = 0.0;

        let assign7930_e6906: f64 = if (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7930_e6906;
        locals.var_guard86_rv = 0.0;

        let (assign7940_e6926,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7940_e6914: f64 = (p.p572 * locals.var_ile);
        let assign7940_e6915: f64 = (p.p571 + assign7940_e6914);
        let assign7940_e6918: f64 = (p.p573 * locals.var_iwe);
        let assign7940_e6919: f64 = (assign7940_e6915 + assign7940_e6918);
        let assign7940_e6922: f64 = (p.p574 * locals.var_iae);
        let assign7940_e6923: f64 = (assign7940_e6919 + assign7940_e6922);
        let assign7940_e6924: f64 = (locals.var_ile * assign7940_e6923);
        (assign7940_e6924,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7940_e6926;
        locals.var_thesat_p_rv = 0.0;

        let assign7950_e6945: f64 = if (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7950_e6945;
        locals.var_guard87_rv = 0.0;

        let (assign7960_e6963,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7960_e6952: f64 = (p.p576 * locals.var_ile);
        let assign7960_e6953: f64 = (p.p575 + assign7960_e6952);
        let assign7960_e6956: f64 = (p.p577 * locals.var_iwe);
        let assign7960_e6957: f64 = (assign7960_e6953 + assign7960_e6956);
        let assign7960_e6960: f64 = (p.p578 * locals.var_iae);
        let assign7960_e6961: f64 = (assign7960_e6957 + assign7960_e6960);
        (assign7960_e6961,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7960_e6963;
        locals.var_stthesat_p_rv = 0.0;

        let assign7970_e6982: f64 = if (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7970_e6982;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7980_e7000,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7980_e6989: f64 = (p.p580 * locals.var_ile);
        let assign7980_e6990: f64 = (p.p579 + assign7980_e6989);
        let assign7980_e6993: f64 = (p.p581 * locals.var_iwe);
        let assign7980_e6994: f64 = (assign7980_e6990 + assign7980_e6993);
        let assign7980_e6997: f64 = (p.p582 * locals.var_iae);
        let assign7980_e6998: f64 = (assign7980_e6994 + assign7980_e6997);
        (assign7980_e6998,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7980_e7000;
        locals.var_thesatb_p_rv = 0.0;

        let assign7990_e7019: f64 = if (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7990_e7019;
        locals.var_guard89_rv = 0.0;

        let (assign8000_e7037,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign8000_e7026: f64 = (p.p584 * locals.var_ile);
        let assign8000_e7027: f64 = (p.p583 + assign8000_e7026);
        let assign8000_e7030: f64 = (p.p585 * locals.var_iwe);
        let assign8000_e7031: f64 = (assign8000_e7027 + assign8000_e7030);
        let assign8000_e7034: f64 = (p.p586 * locals.var_iae);
        let assign8000_e7035: f64 = (assign8000_e7031 + assign8000_e7034);
        (assign8000_e7035,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign8000_e7037;
        locals.var_thesatg_p_rv = 0.0;

        let assign8010_e7056: f64 = if (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign8010_e7056;
        locals.var_guard90_rv = 0.0;

        let (assign8020_e7074,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign8020_e7063: f64 = (p.p588 * locals.var_ile);
        let assign8020_e7064: f64 = (p.p587 + assign8020_e7063);
        let assign8020_e7067: f64 = (p.p589 * locals.var_iwe);
        let assign8020_e7068: f64 = (assign8020_e7064 + assign8020_e7067);
        let assign8020_e7071: f64 = (p.p590 * locals.var_iae);
        let assign8020_e7072: f64 = (assign8020_e7068 + assign8020_e7071);
        (assign8020_e7072,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign8020_e7074;
        locals.var_ax_p_rv = 0.0;

        let assign8030_e7093: f64 = if (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign8030_e7093;
        locals.var_guard91_rv = 0.0;

        let (assign8040_e7113,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign8040_e7101: f64 = (p.p592 * locals.var_ile);
        let assign8040_e7102: f64 = (p.p591 + assign8040_e7101);
        let assign8040_e7105: f64 = (p.p593 * locals.var_iwe);
        let assign8040_e7106: f64 = (assign8040_e7102 + assign8040_e7105);
        let assign8040_e7109: f64 = (p.p594 * locals.var_iae);
        let assign8040_e7110: f64 = (assign8040_e7106 + assign8040_e7109);
        let assign8040_e7111: f64 = (locals.var_ile * assign8040_e7110);
        (assign8040_e7111,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign8040_e7113;
        locals.var_alp_p_rv = 0.0;

        let assign8050_e7132: f64 = if (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign8050_e7132;
        locals.var_guard92_rv = 0.0;

        let (assign8060_e7150,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign8060_e7139: f64 = (p.p596 * locals.var_ile);
        let assign8060_e7140: f64 = (p.p595 + assign8060_e7139);
        let assign8060_e7143: f64 = (p.p597 * locals.var_iwe);
        let assign8060_e7144: f64 = (assign8060_e7140 + assign8060_e7143);
        let assign8060_e7147: f64 = (p.p598 * locals.var_iae);
        let assign8060_e7148: f64 = (assign8060_e7144 + assign8060_e7147);
        (assign8060_e7148,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign8060_e7150;
        locals.var_alp1_p_rv = 0.0;

        let assign8070_e7169: f64 = if (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8070_e7169;
        locals.var_guard93_rv = 0.0;

        let (assign8080_e7187,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign8080_e7176: f64 = (p.p600 * locals.var_ile);
        let assign8080_e7177: f64 = (p.p599 + assign8080_e7176);
        let assign8080_e7180: f64 = (p.p601 * locals.var_iwe);
        let assign8080_e7181: f64 = (assign8080_e7177 + assign8080_e7180);
        let assign8080_e7184: f64 = (p.p602 * locals.var_iae);
        let assign8080_e7185: f64 = (assign8080_e7181 + assign8080_e7184);
        (assign8080_e7185,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign8080_e7187;
        locals.var_alp2_p_rv = 0.0;

        let assign8090_e7206: f64 = if (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8090_e7206;
        locals.var_guard94_rv = 0.0;

        let (assign8100_e7224,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8100_e7213: f64 = (p.p604 * locals.var_ile);
        let assign8100_e7214: f64 = (p.p603 + assign8100_e7213);
        let assign8100_e7217: f64 = (p.p605 * locals.var_iwe);
        let assign8100_e7218: f64 = (assign8100_e7214 + assign8100_e7217);
        let assign8100_e7221: f64 = (p.p606 * locals.var_iae);
        let assign8100_e7222: f64 = (assign8100_e7218 + assign8100_e7221);
        (assign8100_e7222,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign8100_e7224;
        locals.var_a1_p_rv = 0.0;

        let assign8110_e7243: f64 = if (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8110_e7243;
        locals.var_guard95_rv = 0.0;

        let (assign8120_e7261,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8120_e7250: f64 = (p.p608 * locals.var_ile);
        let assign8120_e7251: f64 = (p.p607 + assign8120_e7250);
        let assign8120_e7254: f64 = (p.p609 * locals.var_iwe);
        let assign8120_e7255: f64 = (assign8120_e7251 + assign8120_e7254);
        let assign8120_e7258: f64 = (p.p610 * locals.var_iae);
        let assign8120_e7259: f64 = (assign8120_e7255 + assign8120_e7258);
        (assign8120_e7259,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign8120_e7261;
        locals.var_sta2_p_rv = 0.0;

        let assign8130_e7280: f64 = if (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8130_e7280;
        locals.var_guard96_rv = 0.0;

        let (assign8140_e7298,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8140_e7287: f64 = (p.p612 * locals.var_ile);
        let assign8140_e7288: f64 = (p.p611 + assign8140_e7287);
        let assign8140_e7291: f64 = (p.p613 * locals.var_iwe);
        let assign8140_e7292: f64 = (assign8140_e7288 + assign8140_e7291);
        let assign8140_e7295: f64 = (p.p614 * locals.var_iae);
        let assign8140_e7296: f64 = (assign8140_e7292 + assign8140_e7295);
        (assign8140_e7296,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign8140_e7298;
        locals.var_a3_p_rv = 0.0;

        let assign8150_e7317: f64 = if (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8150_e7317;
        locals.var_guard97_rv = 0.0;

        let (assign8160_e7335,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8160_e7324: f64 = (p.p616 * locals.var_ile);
        let assign8160_e7325: f64 = (p.p615 + assign8160_e7324);
        let assign8160_e7328: f64 = (p.p617 * locals.var_iwe);
        let assign8160_e7329: f64 = (assign8160_e7325 + assign8160_e7328);
        let assign8160_e7332: f64 = (p.p618 * locals.var_iae);
        let assign8160_e7333: f64 = (assign8160_e7329 + assign8160_e7332);
        (assign8160_e7333,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign8160_e7335;
        locals.var_a4_p_rv = 0.0;

        let assign8170_e7354: f64 = if (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8170_e7354;
        locals.var_guard98_rv = 0.0;

        let (assign8180_e7374,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8180_e7362: f64 = (p.p620 * locals.var_ile);
        let assign8180_e7363: f64 = (p.p619 + assign8180_e7362);
        let assign8180_e7366: f64 = (p.p621 * locals.var_iwe);
        let assign8180_e7367: f64 = (assign8180_e7363 + assign8180_e7366);
        let assign8180_e7370: f64 = (p.p622 * locals.var_iae);
        let assign8180_e7371: f64 = (assign8180_e7367 + assign8180_e7370);
        let assign8180_e7372: f64 = (locals.var_iiae * assign8180_e7371);
        (assign8180_e7372,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign8180_e7374;
        locals.var_iginv_p_rv = 0.0;

        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8190_e7393;
        locals.var_guard99_rv = 0.0;

        let (assign8200_e7413,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8200_e7401: f64 = (p.p624 * locals.var_ile);
        let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
        let assign8200_e7405: f64 = (p.p625 * locals.var_iwe);
        let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
        let assign8200_e7409: f64 = (p.p626 * locals.var_iae);
        let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
        let assign8200_e7411: f64 = (locals.var_iiwe * assign8200_e7410);
        (assign8200_e7411,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8200_e7413;
        locals.var_igov_p_rv = 0.0;

        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8210_e7432;
        locals.var_guard100_rv = 0.0;

        let (assign8220_e7452,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8220_e7440: f64 = (p.p628 * locals.var_ile);
        let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
        let assign8220_e7444: f64 = (p.p629 * locals.var_iwe);
        let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
        let assign8220_e7448: f64 = (p.p630 * locals.var_iae);
        let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
        let assign8220_e7450: f64 = (locals.var_iiwe * assign8220_e7449);
        (assign8220_e7450,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8220_e7452;
        locals.var_igovd_p_rv = 0.0;

        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8230_e7471;
        locals.var_guard101_rv = 0.0;

        let (assign8240_e7489,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8240_e7478: f64 = (p.p632 * locals.var_ile);
        let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
        let assign8240_e7482: f64 = (p.p633 * locals.var_iwe);
        let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
        let assign8240_e7486: f64 = (p.p634 * locals.var_iae);
        let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
        (assign8240_e7487,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8240_e7489;
        locals.var_stig_p_rv = 0.0;

        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8250_e7508;
        locals.var_guard102_rv = 0.0;

        let (assign8260_e7528,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8260_e7516: f64 = (p.p636 * locals.var_ile);
        let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
        let assign8260_e7520: f64 = (p.p637 * locals.var_iwe);
        let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
        let assign8260_e7524: f64 = (p.p638 * locals.var_iae);
        let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
        let assign8260_e7526: f64 = (locals.var_iiwe * assign8260_e7525);
        (assign8260_e7526,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8260_e7528;
        locals.var_agidl_p_rv = 0.0;

        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8270_e7547;
        locals.var_guard103_rv = 0.0;

        let (assign8280_e7567,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8280_e7555: f64 = (p.p640 * locals.var_ile);
        let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
        let assign8280_e7559: f64 = (p.p641 * locals.var_iwe);
        let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
        let assign8280_e7563: f64 = (p.p642 * locals.var_iae);
        let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
        let assign8280_e7565: f64 = (locals.var_iiwe * assign8280_e7564);
        (assign8280_e7565,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8280_e7567;
        locals.var_agidld_p_rv = 0.0;

        let assign8290_e7586: f64 = if (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8290_e7586;
        locals.var_guard104_rv = 0.0;

        let (assign8300_e7604,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8300_e7593: f64 = (p.p644 * locals.var_ile);
        let assign8300_e7594: f64 = (p.p643 + assign8300_e7593);
        let assign8300_e7597: f64 = (p.p645 * locals.var_iwe);
        let assign8300_e7598: f64 = (assign8300_e7594 + assign8300_e7597);
        let assign8300_e7601: f64 = (p.p646 * locals.var_iae);
        let assign8300_e7602: f64 = (assign8300_e7598 + assign8300_e7601);
        (assign8300_e7602,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8300_e7604;
        locals.var_stbgidl_p_rv = 0.0;

        let assign8310_e7623: f64 = if (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8310_e7623;
        locals.var_guard105_rv = 0.0;

        let (assign8320_e7641,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard105 != 0.0)) {
        let assign8320_e7630: f64 = (p.p648 * locals.var_ile);
        let assign8320_e7631: f64 = (p.p647 + assign8320_e7630);
        let assign8320_e7634: f64 = (p.p649 * locals.var_iwe);
        let assign8320_e7635: f64 = (assign8320_e7631 + assign8320_e7634);
        let assign8320_e7638: f64 = (p.p650 * locals.var_iae);
        let assign8320_e7639: f64 = (assign8320_e7635 + assign8320_e7638);
        (assign8320_e7639,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8320_e7641;
        locals.var_stbgidld_p_rv = 0.0;

        let assign8330_e7660: f64 = if (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]) { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8330_e7660;
        locals.var_guard106_rv = 0.0;

        let (assign8340_e7684,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard106 != 0.0)) {
        let assign8340_e7666: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8340_e7668: f64 = (assign8340_e7666 / 1e-6);
        let assign8340_e7672: f64 = (p.p652 * locals.var_ile);
        let assign8340_e7673: f64 = (p.p651 + assign8340_e7672);
        let assign8340_e7676: f64 = (p.p653 * locals.var_iwe);
        let assign8340_e7677: f64 = (assign8340_e7673 + assign8340_e7676);
        let assign8340_e7680: f64 = (p.p654 * locals.var_iae);
        let assign8340_e7681: f64 = (assign8340_e7677 + assign8340_e7680);
        let assign8340_e7682: f64 = (assign8340_e7668 * assign8340_e7681);
        (assign8340_e7682,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8340_e7684;
        locals.var_cox_p_rv = 0.0;

        let assign8350_e7703: f64 = if (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8350_e7703;
        locals.var_guard107_rv = 0.0;

        let (assign8360_e7721,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard107 != 0.0)) {
        let assign8360_e7710: f64 = (p.p656 * locals.var_ile);
        let assign8360_e7711: f64 = (p.p655 + assign8360_e7710);
        let assign8360_e7714: f64 = (p.p657 * locals.var_iwe);
        let assign8360_e7715: f64 = (assign8360_e7711 + assign8360_e7714);
        let assign8360_e7718: f64 = (p.p658 * locals.var_iae);
        let assign8360_e7719: f64 = (assign8360_e7715 + assign8360_e7718);
        (assign8360_e7719,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8360_e7721;
        locals.var_delvtac_p_rv = 0.0;

        let assign8370_e7740: f64 = if (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]) { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8370_e7740;
        locals.var_guard108_rv = 0.0;

        let (assign8380_e7758,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard108 != 0.0)) {
        let assign8380_e7747: f64 = (p.p660 * locals.var_ile);
        let assign8380_e7748: f64 = (p.p659 + assign8380_e7747);
        let assign8380_e7751: f64 = (p.p661 * locals.var_iwe);
        let assign8380_e7752: f64 = (assign8380_e7748 + assign8380_e7751);
        let assign8380_e7755: f64 = (p.p662 * locals.var_iae);
        let assign8380_e7756: f64 = (assign8380_e7752 + assign8380_e7755);
        (assign8380_e7756,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8380_e7758;
        locals.var_facneffac_p_rv = 0.0;

        let assign8390_e7797: f64 = if (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8390_e7797;
        locals.var_guard109_rv = 0.0;

        let (assign8400_e7803,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p571,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8400_e7803;
        locals.var_poparam_i_rv = 0.0;

        let assign8410_e7805: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8410_e7807: f64 = if assign8410_e7805 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8410_e7807;
        locals.var_guard110_rv = 0.0;

        let (assign8420_e7815,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p663,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8420_e7815;
        locals.var_poparam_i_rv = 0.0;

        let (assign8430_e7821,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p572,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8430_e7821;
        locals.var_plparam_i_rv = 0.0;

        let assign8440_e7823: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8440_e7825: f64 = if assign8440_e7823 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8440_e7825;
        locals.var_guard111_rv = 0.0;

        let (assign8450_e7833,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p664,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8450_e7833;
        locals.var_plparam_i_rv = 0.0;

        let (assign8460_e7839,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p573,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8460_e7839;
        locals.var_pwparam_i_rv = 0.0;

        let assign8470_e7841: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8470_e7843: f64 = if assign8470_e7841 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8470_e7843;
        locals.var_guard112_rv = 0.0;

        let (assign8480_e7851,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p665,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8480_e7851;
        locals.var_pwparam_i_rv = 0.0;

    }
}
