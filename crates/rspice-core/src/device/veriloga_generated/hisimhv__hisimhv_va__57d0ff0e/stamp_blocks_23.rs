#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_368(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let assign100010_e152150: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100010_e152150;

        let (assign100020_e152158, assign100020_e152158_d_n0, assign100020_e152158_d_n2, assign100020_e152158_d_n4, assign100020_e152158_d_n5, assign100020_e152158_d_n6, assign100020_e152158_d_n7, assign100020_e152158_d_n8, assign100020_e152158_d_n9, assign100020_e152158_d_n10, assign100020_e152158_d_n11, assign100020_e152158_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2309 != 0.0)) {
        let assign100020_e152156: f64 = (locals.var_exp_k * p.p541);
        (assign100020_e152156, (locals.var_exp_k_dn0 * p.p541), (locals.var_exp_k_dn2 * p.p541), (locals.var_exp_k_dn4 * p.p541), (locals.var_exp_k_dn5 * p.p541), (locals.var_exp_k_dn6 * p.p541), (locals.var_exp_k_dn7 * p.p541), (locals.var_exp_k_dn8 * p.p541), (locals.var_exp_k_dn9 * p.p541), (locals.var_exp_k_dn10 * p.p541), (locals.var_exp_k_dn11 * p.p541), (locals.var_exp_k_dn14 * p.p541),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100020_e152158;
        locals.var_exp_k2_dn0 = assign100020_e152158_d_n0;
        locals.var_exp_k2_dn2 = assign100020_e152158_d_n2;
        locals.var_exp_k2_dn4 = assign100020_e152158_d_n4;
        locals.var_exp_k2_dn5 = assign100020_e152158_d_n5;
        locals.var_exp_k2_dn6 = assign100020_e152158_d_n6;
        locals.var_exp_k2_dn7 = assign100020_e152158_d_n7;
        locals.var_exp_k2_dn8 = assign100020_e152158_d_n8;
        locals.var_exp_k2_dn9 = assign100020_e152158_d_n9;
        locals.var_exp_k2_dn10 = assign100020_e152158_d_n10;
        locals.var_exp_k2_dn11 = assign100020_e152158_d_n11;
        locals.var_exp_k2_dn14 = assign100020_e152158_d_n14;

        let (assign100030_e152187, assign100030_e152187_d_n0, assign100030_e152187_d_n2, assign100030_e152187_d_n4, assign100030_e152187_d_n5, assign100030_e152187_d_n6, assign100030_e152187_d_n7, assign100030_e152187_d_n8, assign100030_e152187_d_n9, assign100030_e152187_d_n10, assign100030_e152187_d_n11, assign100030_e152187_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2309 == 0.0)) {
        let assign100030_e152165: f64 = (locals.var_exp_k * p.p541);
        let assign100030_e152167: f64 = (-p.p542);
        let assign100030_e152170: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100030_e152171: f64 = (assign100030_e152167 * assign100030_e152170);
        let assign100030_e152174: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign100030_e152175: f64 = (assign100030_e152171 * assign100030_e152174);
        let assign100030_e152179: f64 = (1.0 / locals.var_tratio);
        let assign100030_e152180: f64 = (assign100030_e152179).ln();
        let assign100030_e152181: f64 = (p.p548 * assign100030_e152180);
        let assign100030_e152182: f64 = (assign100030_e152181).exp();
        let assign100030_e152183: f64 = (assign100030_e152175 * assign100030_e152182);
        let assign100030_e152184: f64 = (assign100030_e152183).exp();
        let assign100030_e152185: f64 = (assign100030_e152165 * assign100030_e152184);
        (assign100030_e152185, (((locals.var_exp_k_dn0 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0)) * assign100030_e152174) + (assign100030_e152171 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn2 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn2)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn2))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn4 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn4)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn4))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn5 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn5)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn5))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn6 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn6)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn6))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn7 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn7)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn7))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn8 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn8)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn8))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn9 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn9)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn9))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn10 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10)) * assign100030_e152174) + (assign100030_e152171 * (locals.var_vbd_jct_dn10 - locals.var_v_hk_dn10))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn11 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn11)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn11))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn11 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))), (((locals.var_exp_k_dn14 * p.p541) * assign100030_e152184) + (assign100030_e152165 * (assign100030_e152184 * (((((assign100030_e152167 * (-locals.var_v_hk_dn14)) * assign100030_e152174) + (assign100030_e152171 * (-locals.var_v_hk_dn14))) * assign100030_e152182) + (assign100030_e152175 * (assign100030_e152182 * (p.p548 * ((-(locals.var_tratio_dn14 / (locals.var_tratio * locals.var_tratio))) / assign100030_e152179)))))))),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100030_e152187;
        locals.var_exp_k2_dn0 = assign100030_e152187_d_n0;
        locals.var_exp_k2_dn2 = assign100030_e152187_d_n2;
        locals.var_exp_k2_dn4 = assign100030_e152187_d_n4;
        locals.var_exp_k2_dn5 = assign100030_e152187_d_n5;
        locals.var_exp_k2_dn6 = assign100030_e152187_d_n6;
        locals.var_exp_k2_dn7 = assign100030_e152187_d_n7;
        locals.var_exp_k2_dn8 = assign100030_e152187_d_n8;
        locals.var_exp_k2_dn9 = assign100030_e152187_d_n9;
        locals.var_exp_k2_dn10 = assign100030_e152187_d_n10;
        locals.var_exp_k2_dn11 = assign100030_e152187_d_n11;
        locals.var_exp_k2_dn14 = assign100030_e152187_d_n14;

        let (assign100040_e152196, assign100040_e152196_d_n0, assign100040_e152196_d_n2, assign100040_e152196_d_n4, assign100040_e152196_d_n5, assign100040_e152196_d_n6, assign100040_e152196_d_n7, assign100040_e152196_d_n8, assign100040_e152196_d_n9, assign100040_e152196_d_n10, assign100040_e152196_d_n11, assign100040_e152196_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let (assign100040_e152194, assign100040_e152194_d_n0, assign100040_e152194_d_n2, assign100040_e152194_d_n4, assign100040_e152194_d_n5, assign100040_e152194_d_n6, assign100040_e152194_d_n7, assign100040_e152194_d_n8, assign100040_e152194_d_n9, assign100040_e152194_d_n10, assign100040_e152194_d_n11, assign100040_e152194_d_n14,) = {
            if (locals.var_exp_k2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
            }
        };
        (assign100040_e152194, assign100040_e152194_d_n0, assign100040_e152194_d_n2, assign100040_e152194_d_n4, assign100040_e152194_d_n5, assign100040_e152194_d_n6, assign100040_e152194_d_n7, assign100040_e152194_d_n8, assign100040_e152194_d_n9, assign100040_e152194_d_n10, assign100040_e152194_d_n11, assign100040_e152194_d_n14,)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn11, locals.var_exp_k2_dn14,)
    }
};
        locals.var_exp_k2 = assign100040_e152196;
        locals.var_exp_k2_dn0 = assign100040_e152196_d_n0;
        locals.var_exp_k2_dn2 = assign100040_e152196_d_n2;
        locals.var_exp_k2_dn4 = assign100040_e152196_d_n4;
        locals.var_exp_k2_dn5 = assign100040_e152196_d_n5;
        locals.var_exp_k2_dn6 = assign100040_e152196_d_n6;
        locals.var_exp_k2_dn7 = assign100040_e152196_d_n7;
        locals.var_exp_k2_dn8 = assign100040_e152196_d_n8;
        locals.var_exp_k2_dn9 = assign100040_e152196_d_n9;
        locals.var_exp_k2_dn10 = assign100040_e152196_d_n10;
        locals.var_exp_k2_dn11 = assign100040_e152196_d_n11;
        locals.var_exp_k2_dn14 = assign100040_e152196_d_n14;

        let (assign100050_e152202, assign100050_e152202_d_n0, assign100050_e152202_d_n2, assign100050_e152202_d_n4, assign100050_e152202_d_n5, assign100050_e152202_d_n6, assign100050_e152202_d_n7, assign100050_e152202_d_n8, assign100050_e152202_d_n9, assign100050_e152202_d_n10, assign100050_e152202_d_n11, assign100050_e152202_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100050_e152200: f64 = (locals.var_pn0 * locals.var_exp_k2);
        (assign100050_e152200, ((locals.var_pn0_dn0 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn10)), ((locals.var_pn0_dn11 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn11)), ((locals.var_pn0_dn14 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn14)),)
    } else {
        (locals.var_p_nk, locals.var_p_nk_dn0, locals.var_p_nk_dn2, locals.var_p_nk_dn4, locals.var_p_nk_dn5, locals.var_p_nk_dn6, locals.var_p_nk_dn7, locals.var_p_nk_dn8, locals.var_p_nk_dn9, locals.var_p_nk_dn10, locals.var_p_nk_dn11, locals.var_p_nk_dn14,)
    }
};
        locals.var_p_nk = assign100050_e152202;
        locals.var_p_nk_dn0 = assign100050_e152202_d_n0;
        locals.var_p_nk_dn2 = assign100050_e152202_d_n2;
        locals.var_p_nk_dn4 = assign100050_e152202_d_n4;
        locals.var_p_nk_dn5 = assign100050_e152202_d_n5;
        locals.var_p_nk_dn6 = assign100050_e152202_d_n6;
        locals.var_p_nk_dn7 = assign100050_e152202_d_n7;
        locals.var_p_nk_dn8 = assign100050_e152202_d_n8;
        locals.var_p_nk_dn9 = assign100050_e152202_d_n9;
        locals.var_p_nk_dn10 = assign100050_e152202_d_n10;
        locals.var_p_nk_dn11 = assign100050_e152202_d_n11;
        locals.var_p_nk_dn14 = assign100050_e152202_d_n14;

        let (assign100060_e152212, assign100060_e152212_d_n0, assign100060_e152212_d_n2, assign100060_e152212_d_n4, assign100060_e152212_d_n5, assign100060_e152212_d_n6, assign100060_e152212_d_n7, assign100060_e152212_d_n8, assign100060_e152212_d_n9, assign100060_e152212_d_n10, assign100060_e152212_d_n11, assign100060_e152212_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100060_e152206: f64 = (1.6021918e-19 * p.p13);
        let assign100060_e152209: f64 = (locals.var_p_nk - locals.var_pn0);
        let assign100060_e152210: f64 = (assign100060_e152206 * assign100060_e152209);
        (assign100060_e152210, (assign100060_e152206 * (locals.var_p_nk_dn0 - locals.var_pn0_dn0)), (assign100060_e152206 * (locals.var_p_nk_dn2 - locals.var_pn0_dn2)), (assign100060_e152206 * (locals.var_p_nk_dn4 - locals.var_pn0_dn4)), (assign100060_e152206 * (locals.var_p_nk_dn5 - locals.var_pn0_dn5)), (assign100060_e152206 * (locals.var_p_nk_dn6 - locals.var_pn0_dn6)), (assign100060_e152206 * (locals.var_p_nk_dn7 - locals.var_pn0_dn7)), (assign100060_e152206 * (locals.var_p_nk_dn8 - locals.var_pn0_dn8)), (assign100060_e152206 * (locals.var_p_nk_dn9 - locals.var_pn0_dn9)), (assign100060_e152206 * (locals.var_p_nk_dn10 - locals.var_pn0_dn10)), (assign100060_e152206 * (locals.var_p_nk_dn11 - locals.var_pn0_dn11)), (assign100060_e152206 * (locals.var_p_nk_dn14 - locals.var_pn0_dn14)),)
    } else {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    }
};
        locals.var_q_pexk = assign100060_e152212;
        locals.var_q_pexk_dn0 = assign100060_e152212_d_n0;
        locals.var_q_pexk_dn2 = assign100060_e152212_d_n2;
        locals.var_q_pexk_dn4 = assign100060_e152212_d_n4;
        locals.var_q_pexk_dn5 = assign100060_e152212_d_n5;
        locals.var_q_pexk_dn6 = assign100060_e152212_d_n6;
        locals.var_q_pexk_dn7 = assign100060_e152212_d_n7;
        locals.var_q_pexk_dn8 = assign100060_e152212_d_n8;
        locals.var_q_pexk_dn9 = assign100060_e152212_d_n9;
        locals.var_q_pexk_dn10 = assign100060_e152212_d_n10;
        locals.var_q_pexk_dn11 = assign100060_e152212_d_n11;
        locals.var_q_pexk_dn14 = assign100060_e152212_d_n14;

        let assign100070_e152215: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100070_e152215;

        let (assign100080_e152223, assign100080_e152223_d_n0, assign100080_e152223_d_n2, assign100080_e152223_d_n4, assign100080_e152223_d_n5, assign100080_e152223_d_n6, assign100080_e152223_d_n7, assign100080_e152223_d_n8, assign100080_e152223_d_n9, assign100080_e152223_d_n10, assign100080_e152223_d_n11, assign100080_e152223_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100080_e152221: f64 = (locals.var_q_pexk * p.p543);
        (assign100080_e152221, (locals.var_q_pexk_dn0 * p.p543), (locals.var_q_pexk_dn2 * p.p543), (locals.var_q_pexk_dn4 * p.p543), (locals.var_q_pexk_dn5 * p.p543), (locals.var_q_pexk_dn6 * p.p543), (locals.var_q_pexk_dn7 * p.p543), (locals.var_q_pexk_dn8 * p.p543), (locals.var_q_pexk_dn9 * p.p543), (locals.var_q_pexk_dn10 * p.p543), (locals.var_q_pexk_dn11 * p.p543), (locals.var_q_pexk_dn14 * p.p543),)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100080_e152223;
        locals.var_q_qs_k_dn0 = assign100080_e152223_d_n0;
        locals.var_q_qs_k_dn2 = assign100080_e152223_d_n2;
        locals.var_q_qs_k_dn4 = assign100080_e152223_d_n4;
        locals.var_q_qs_k_dn5 = assign100080_e152223_d_n5;
        locals.var_q_qs_k_dn6 = assign100080_e152223_d_n6;
        locals.var_q_qs_k_dn7 = assign100080_e152223_d_n7;
        locals.var_q_qs_k_dn8 = assign100080_e152223_d_n8;
        locals.var_q_qs_k_dn9 = assign100080_e152223_d_n9;
        locals.var_q_qs_k_dn10 = assign100080_e152223_d_n10;
        locals.var_q_qs_k_dn11 = assign100080_e152223_d_n11;
        locals.var_q_qs_k_dn14 = assign100080_e152223_d_n14;

        let (assign100090_e152231, assign100090_e152231_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100090_e152229: f64 = (p.p543 * (nv17 - 0.0));
        (assign100090_e152229, p.p543,)
    } else {
        (locals.var_q_nqs_k, locals.var_q_nqs_k_dn17,)
    }
};
        locals.var_q_nqs_k = assign100090_e152231;
        locals.var_q_nqs_k_dn17 = assign100090_e152231_d_n17;

        let (assign100100_e152241, assign100100_e152241_d_n0, assign100100_e152241_d_n2, assign100100_e152241_d_n4, assign100100_e152241_d_n5, assign100100_e152241_d_n6, assign100100_e152241_d_n7, assign100100_e152241_d_n8, assign100100_e152241_d_n9, assign100100_e152241_d_n10, assign100100_e152241_d_n11, assign100100_e152241_d_n14, assign100100_e152241_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100100_e152237: f64 = (locals.var_q_nqs_k - locals.var_q_qs_k);
        let assign100100_e152239: f64 = (assign100100_e152237 / p.p543);
        (assign100100_e152239, ((-locals.var_q_qs_k_dn0) / p.p543), ((-locals.var_q_qs_k_dn2) / p.p543), ((-locals.var_q_qs_k_dn4) / p.p543), ((-locals.var_q_qs_k_dn5) / p.p543), ((-locals.var_q_qs_k_dn6) / p.p543), ((-locals.var_q_qs_k_dn7) / p.p543), ((-locals.var_q_qs_k_dn8) / p.p543), ((-locals.var_q_qs_k_dn9) / p.p543), ((-locals.var_q_qs_k_dn10) / p.p543), ((-locals.var_q_qs_k_dn11) / p.p543), ((-locals.var_q_qs_k_dn14) / p.p543), (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_inqs0_k, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, locals.var_inqs0_k_dn17,)
    }
};
        locals.var_inqs0_k = assign100100_e152241;
        locals.var_inqs0_k_dn0 = assign100100_e152241_d_n0;
        locals.var_inqs0_k_dn2 = assign100100_e152241_d_n2;
        locals.var_inqs0_k_dn4 = assign100100_e152241_d_n4;
        locals.var_inqs0_k_dn5 = assign100100_e152241_d_n5;
        locals.var_inqs0_k_dn6 = assign100100_e152241_d_n6;
        locals.var_inqs0_k_dn7 = assign100100_e152241_d_n7;
        locals.var_inqs0_k_dn8 = assign100100_e152241_d_n8;
        locals.var_inqs0_k_dn9 = assign100100_e152241_d_n9;
        locals.var_inqs0_k_dn10 = assign100100_e152241_d_n10;
        locals.var_inqs0_k_dn11 = assign100100_e152241_d_n11;
        locals.var_inqs0_k_dn14 = assign100100_e152241_d_n14;
        locals.var_inqs0_k_dn17 = assign100100_e152241_d_n17;

        let (assign100110_e152249, assign100110_e152249_d_n0, assign100110_e152249_d_n2, assign100110_e152249_d_n4, assign100110_e152249_d_n5, assign100110_e152249_d_n6, assign100110_e152249_d_n7, assign100110_e152249_d_n8, assign100110_e152249_d_n9, assign100110_e152249_d_n10, assign100110_e152249_d_n11, assign100110_e152249_d_n14, assign100110_e152249_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 != 0.0)) {
        let assign100110_e152247: f64 = (locals.var_q_nqs_k / p.p543);
        (assign100110_e152247, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_k_dn17 / p.p543),)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100110_e152249;
        locals.var_q_pexk_nqs_dn0 = assign100110_e152249_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100110_e152249_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100110_e152249_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100110_e152249_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100110_e152249_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100110_e152249_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100110_e152249_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100110_e152249_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100110_e152249_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100110_e152249_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100110_e152249_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100110_e152249_d_n17;

        let (assign100120_e152256, assign100120_e152256_d_n0, assign100120_e152256_d_n2, assign100120_e152256_d_n4, assign100120_e152256_d_n5, assign100120_e152256_d_n6, assign100120_e152256_d_n7, assign100120_e152256_d_n8, assign100120_e152256_d_n9, assign100120_e152256_d_n10, assign100120_e152256_d_n11, assign100120_e152256_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn11, locals.var_q_pexk_dn14,)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14,)
    }
};
        locals.var_q_qs_k = assign100120_e152256;
        locals.var_q_qs_k_dn0 = assign100120_e152256_d_n0;
        locals.var_q_qs_k_dn2 = assign100120_e152256_d_n2;
        locals.var_q_qs_k_dn4 = assign100120_e152256_d_n4;
        locals.var_q_qs_k_dn5 = assign100120_e152256_d_n5;
        locals.var_q_qs_k_dn6 = assign100120_e152256_d_n6;
        locals.var_q_qs_k_dn7 = assign100120_e152256_d_n7;
        locals.var_q_qs_k_dn8 = assign100120_e152256_d_n8;
        locals.var_q_qs_k_dn9 = assign100120_e152256_d_n9;
        locals.var_q_qs_k_dn10 = assign100120_e152256_d_n10;
        locals.var_q_qs_k_dn11 = assign100120_e152256_d_n11;
        locals.var_q_qs_k_dn14 = assign100120_e152256_d_n14;

        let (assign100130_e152263, assign100130_e152263_d_n0, assign100130_e152263_d_n2, assign100130_e152263_d_n4, assign100130_e152263_d_n5, assign100130_e152263_d_n6, assign100130_e152263_d_n7, assign100130_e152263_d_n8, assign100130_e152263_d_n9, assign100130_e152263_d_n10, assign100130_e152263_d_n11, assign100130_e152263_d_n14, assign100130_e152263_d_n17,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2310 == 0.0)) {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn11, locals.var_q_qs_k_dn14, 0.0,)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn11, locals.var_q_pexk_nqs_dn14, locals.var_q_pexk_nqs_dn17,)
    }
};
        locals.var_q_pexk_nqs = assign100130_e152263;
        locals.var_q_pexk_nqs_dn0 = assign100130_e152263_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100130_e152263_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100130_e152263_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100130_e152263_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100130_e152263_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100130_e152263_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100130_e152263_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100130_e152263_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100130_e152263_d_n10;
        locals.var_q_pexk_nqs_dn11 = assign100130_e152263_d_n11;
        locals.var_q_pexk_nqs_dn14 = assign100130_e152263_d_n14;
        locals.var_q_pexk_nqs_dn17 = assign100130_e152263_d_n17;

        let (assign100140_e152269, assign100140_e152269_d_n0, assign100140_e152269_d_n2, assign100140_e152269_d_n4, assign100140_e152269_d_n5, assign100140_e152269_d_n6, assign100140_e152269_d_n7, assign100140_e152269_d_n8, assign100140_e152269_d_n9, assign100140_e152269_d_n10, assign100140_e152269_d_n11, assign100140_e152269_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100140_e152267: f64 = (p.p506 - locals.var_vbd_jct);
        (assign100140_e152267, (-locals.var_vbd_jct_dn0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100140_e152269;
        locals.var_vjunc_a_dn0 = assign100140_e152269_d_n0;
        locals.var_vjunc_a_dn2 = assign100140_e152269_d_n2;
        locals.var_vjunc_a_dn4 = assign100140_e152269_d_n4;
        locals.var_vjunc_a_dn5 = assign100140_e152269_d_n5;
        locals.var_vjunc_a_dn6 = assign100140_e152269_d_n6;
        locals.var_vjunc_a_dn7 = assign100140_e152269_d_n7;
        locals.var_vjunc_a_dn8 = assign100140_e152269_d_n8;
        locals.var_vjunc_a_dn9 = assign100140_e152269_d_n9;
        locals.var_vjunc_a_dn10 = assign100140_e152269_d_n10;
        locals.var_vjunc_a_dn11 = assign100140_e152269_d_n11;
        locals.var_vjunc_a_dn14 = assign100140_e152269_d_n14;

        let (assign100150_e152282, assign100150_e152282_d_n0, assign100150_e152282_d_n2, assign100150_e152282_d_n4, assign100150_e152282_d_n5, assign100150_e152282_d_n6, assign100150_e152282_d_n7, assign100150_e152282_d_n8, assign100150_e152282_d_n9, assign100150_e152282_d_n10, assign100150_e152282_d_n11, assign100150_e152282_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100150_e152273: f64 = (locals.var_vjunc_a * locals.var_vjunc_a);
        let assign100150_e152276: f64 = (4.0 * locals.var_juncdlt);
        let assign100150_e152278: f64 = (assign100150_e152276 * locals.var_juncdlt);
        let assign100150_e152279: f64 = (assign100150_e152273 + assign100150_e152278);
        let assign100150_e152280: f64 = (assign100150_e152279).sqrt();
        (assign100150_e152280, (((locals.var_vjunc_a_dn0 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn0)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn2 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn2)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn4 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn4)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn5 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn5)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn6 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn6)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn7 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn7)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn8 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn8)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn9 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn9)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn10 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn10)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn11 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn11)) / (2.0 * assign100150_e152280)), (((locals.var_vjunc_a_dn14 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn14)) / (2.0 * assign100150_e152280)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100150_e152282;
        locals.var_tmf2_dn0 = assign100150_e152282_d_n0;
        locals.var_tmf2_dn2 = assign100150_e152282_d_n2;
        locals.var_tmf2_dn4 = assign100150_e152282_d_n4;
        locals.var_tmf2_dn5 = assign100150_e152282_d_n5;
        locals.var_tmf2_dn6 = assign100150_e152282_d_n6;
        locals.var_tmf2_dn7 = assign100150_e152282_d_n7;
        locals.var_tmf2_dn8 = assign100150_e152282_d_n8;
        locals.var_tmf2_dn9 = assign100150_e152282_d_n9;
        locals.var_tmf2_dn10 = assign100150_e152282_d_n10;
        locals.var_tmf2_dn11 = assign100150_e152282_d_n11;
        locals.var_tmf2_dn14 = assign100150_e152282_d_n14;

        let (assign100160_e152292, assign100160_e152292_d_n0, assign100160_e152292_d_n2, assign100160_e152292_d_n4, assign100160_e152292_d_n5, assign100160_e152292_d_n6, assign100160_e152292_d_n7, assign100160_e152292_d_n8, assign100160_e152292_d_n9, assign100160_e152292_d_n10, assign100160_e152292_d_n11, assign100160_e152292_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100160_e152288: f64 = (locals.var_vjunc_a / locals.var_tmf2);
        let assign100160_e152289: f64 = (1.0 + assign100160_e152288);
        let assign100160_e152290: f64 = (0.5 * assign100160_e152289);
        (assign100160_e152290, (0.5 * (((locals.var_vjunc_a_dn0 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn2 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn4 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn5 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn6 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn7 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn8 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn9 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn10 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn11 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn14 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100160_e152292;
        locals.var_t0_dn0 = assign100160_e152292_d_n0;
        locals.var_t0_dn2 = assign100160_e152292_d_n2;
        locals.var_t0_dn4 = assign100160_e152292_d_n4;
        locals.var_t0_dn5 = assign100160_e152292_d_n5;
        locals.var_t0_dn6 = assign100160_e152292_d_n6;
        locals.var_t0_dn7 = assign100160_e152292_d_n7;
        locals.var_t0_dn8 = assign100160_e152292_d_n8;
        locals.var_t0_dn9 = assign100160_e152292_d_n9;
        locals.var_t0_dn10 = assign100160_e152292_d_n10;
        locals.var_t0_dn11 = assign100160_e152292_d_n11;
        locals.var_t0_dn14 = assign100160_e152292_d_n14;

        let (assign100170_e152300, assign100170_e152300_d_n0, assign100170_e152300_d_n2, assign100170_e152300_d_n4, assign100170_e152300_d_n5, assign100170_e152300_d_n6, assign100170_e152300_d_n7, assign100170_e152300_d_n8, assign100170_e152300_d_n9, assign100170_e152300_d_n10, assign100170_e152300_d_n11, assign100170_e152300_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100170_e152297: f64 = (locals.var_vjunc_a + locals.var_tmf2);
        let assign100170_e152298: f64 = (0.5 * assign100170_e152297);
        (assign100170_e152298, (0.5 * (locals.var_vjunc_a_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vjunc_a_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vjunc_a_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vjunc_a_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vjunc_a_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vjunc_a_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vjunc_a_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vjunc_a_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vjunc_a_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vjunc_a_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vjunc_a_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100170_e152300;
        locals.var_vjunc_a_dn0 = assign100170_e152300_d_n0;
        locals.var_vjunc_a_dn2 = assign100170_e152300_d_n2;
        locals.var_vjunc_a_dn4 = assign100170_e152300_d_n4;
        locals.var_vjunc_a_dn5 = assign100170_e152300_d_n5;
        locals.var_vjunc_a_dn6 = assign100170_e152300_d_n6;
        locals.var_vjunc_a_dn7 = assign100170_e152300_d_n7;
        locals.var_vjunc_a_dn8 = assign100170_e152300_d_n8;
        locals.var_vjunc_a_dn9 = assign100170_e152300_d_n9;
        locals.var_vjunc_a_dn10 = assign100170_e152300_d_n10;
        locals.var_vjunc_a_dn11 = assign100170_e152300_d_n11;
        locals.var_vjunc_a_dn14 = assign100170_e152300_d_n14;

        let assign100180_e152303: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100180_e152303;

        let (assign100190_e152309, assign100190_e152309_d_n0, assign100190_e152309_d_n2, assign100190_e152309_d_n4, assign100190_e152309_d_n5, assign100190_e152309_d_n6, assign100190_e152309_d_n7, assign100190_e152309_d_n8, assign100190_e152309_d_n9, assign100190_e152309_d_n10, assign100190_e152309_d_n11, assign100190_e152309_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn11, locals.var_vjunc_a_dn14,)
    }
};
        locals.var_vjunc_a = assign100190_e152309;
        locals.var_vjunc_a_dn0 = assign100190_e152309_d_n0;
        locals.var_vjunc_a_dn2 = assign100190_e152309_d_n2;
        locals.var_vjunc_a_dn4 = assign100190_e152309_d_n4;
        locals.var_vjunc_a_dn5 = assign100190_e152309_d_n5;
        locals.var_vjunc_a_dn6 = assign100190_e152309_d_n6;
        locals.var_vjunc_a_dn7 = assign100190_e152309_d_n7;
        locals.var_vjunc_a_dn8 = assign100190_e152309_d_n8;
        locals.var_vjunc_a_dn9 = assign100190_e152309_d_n9;
        locals.var_vjunc_a_dn10 = assign100190_e152309_d_n10;
        locals.var_vjunc_a_dn11 = assign100190_e152309_d_n11;
        locals.var_vjunc_a_dn14 = assign100190_e152309_d_n14;

        let (assign100200_e152315, assign100200_e152315_d_n0, assign100200_e152315_d_n2, assign100200_e152315_d_n4, assign100200_e152315_d_n5, assign100200_e152315_d_n6, assign100200_e152315_d_n7, assign100200_e152315_d_n8, assign100200_e152315_d_n9, assign100200_e152315_d_n10, assign100200_e152315_d_n11, assign100200_e152315_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100200_e152315;
        locals.var_t0_dn0 = assign100200_e152315_d_n0;
        locals.var_t0_dn2 = assign100200_e152315_d_n2;
        locals.var_t0_dn4 = assign100200_e152315_d_n4;
        locals.var_t0_dn5 = assign100200_e152315_d_n5;
        locals.var_t0_dn6 = assign100200_e152315_d_n6;
        locals.var_t0_dn7 = assign100200_e152315_d_n7;
        locals.var_t0_dn8 = assign100200_e152315_d_n8;
        locals.var_t0_dn9 = assign100200_e152315_d_n9;
        locals.var_t0_dn10 = assign100200_e152315_d_n10;
        locals.var_t0_dn11 = assign100200_e152315_d_n11;
        locals.var_t0_dn14 = assign100200_e152315_d_n14;

        let (assign100210_e152328, assign100210_e152328_d_n0, assign100210_e152328_d_n2, assign100210_e152328_d_n4, assign100210_e152328_d_n5, assign100210_e152328_d_n6, assign100210_e152328_d_n7, assign100210_e152328_d_n8, assign100210_e152328_d_n9, assign100210_e152328_d_n10, assign100210_e152328_d_n11, assign100210_e152328_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100210_e152319: f64 = (2.0 * 1.034943e-10);
        let assign100210_e152321: f64 = (assign100210_e152319 * locals.var_vjunc_a);
        let assign100210_e152324: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign100210_e152325: f64 = (assign100210_e152321 / assign100210_e152324);
        let assign100210_e152326: f64 = (assign100210_e152325).sqrt();
        (assign100210_e152326, (((assign100210_e152319 * locals.var_vjunc_a_dn0) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn2) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn4) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn5) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn6) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn7) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn8) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn9) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn10) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn11) / assign100210_e152324) / (2.0 * assign100210_e152326)), (((assign100210_e152319 * locals.var_vjunc_a_dn14) / assign100210_e152324) / (2.0 * assign100210_e152326)),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100210_e152328;
        locals.var_w_depa_dn0 = assign100210_e152328_d_n0;
        locals.var_w_depa_dn2 = assign100210_e152328_d_n2;
        locals.var_w_depa_dn4 = assign100210_e152328_d_n4;
        locals.var_w_depa_dn5 = assign100210_e152328_d_n5;
        locals.var_w_depa_dn6 = assign100210_e152328_d_n6;
        locals.var_w_depa_dn7 = assign100210_e152328_d_n7;
        locals.var_w_depa_dn8 = assign100210_e152328_d_n8;
        locals.var_w_depa_dn9 = assign100210_e152328_d_n9;
        locals.var_w_depa_dn10 = assign100210_e152328_d_n10;
        locals.var_w_depa_dn11 = assign100210_e152328_d_n11;
        locals.var_w_depa_dn14 = assign100210_e152328_d_n14;

        let (assign100220_e152336, assign100220_e152336_d_n0, assign100220_e152336_d_n2, assign100220_e152336_d_n4, assign100220_e152336_d_n5, assign100220_e152336_d_n6, assign100220_e152336_d_n7, assign100220_e152336_d_n8, assign100220_e152336_d_n9, assign100220_e152336_d_n10, assign100220_e152336_d_n11, assign100220_e152336_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100220_e152332: f64 = (p.p545 - locals.var_w_depa);
        let assign100220_e152334: f64 = (assign100220_e152332 - 1e-7);
        (assign100220_e152334, (-locals.var_w_depa_dn0), (-locals.var_w_depa_dn2), (-locals.var_w_depa_dn4), (-locals.var_w_depa_dn5), (-locals.var_w_depa_dn6), (-locals.var_w_depa_dn7), (-locals.var_w_depa_dn8), (-locals.var_w_depa_dn9), (-locals.var_w_depa_dn10), (-locals.var_w_depa_dn11), (-locals.var_w_depa_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100220_e152336;
        locals.var_tmf1_dn0 = assign100220_e152336_d_n0;
        locals.var_tmf1_dn2 = assign100220_e152336_d_n2;
        locals.var_tmf1_dn4 = assign100220_e152336_d_n4;
        locals.var_tmf1_dn5 = assign100220_e152336_d_n5;
        locals.var_tmf1_dn6 = assign100220_e152336_d_n6;
        locals.var_tmf1_dn7 = assign100220_e152336_d_n7;
        locals.var_tmf1_dn8 = assign100220_e152336_d_n8;
        locals.var_tmf1_dn9 = assign100220_e152336_d_n9;
        locals.var_tmf1_dn10 = assign100220_e152336_d_n10;
        locals.var_tmf1_dn11 = assign100220_e152336_d_n11;
        locals.var_tmf1_dn14 = assign100220_e152336_d_n14;

        let (assign100230_e152344, assign100230_e152344_d_n0, assign100230_e152344_d_n2, assign100230_e152344_d_n4, assign100230_e152344_d_n5, assign100230_e152344_d_n6, assign100230_e152344_d_n7, assign100230_e152344_d_n8, assign100230_e152344_d_n9, assign100230_e152344_d_n10, assign100230_e152344_d_n11, assign100230_e152344_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100230_e152340: f64 = (4.0 * p.p545);
        let assign100230_e152342: f64 = (assign100230_e152340 * 1e-7);
        (assign100230_e152342, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100230_e152344;
        locals.var_tmf2_dn0 = assign100230_e152344_d_n0;
        locals.var_tmf2_dn2 = assign100230_e152344_d_n2;
        locals.var_tmf2_dn4 = assign100230_e152344_d_n4;
        locals.var_tmf2_dn5 = assign100230_e152344_d_n5;
        locals.var_tmf2_dn6 = assign100230_e152344_d_n6;
        locals.var_tmf2_dn7 = assign100230_e152344_d_n7;
        locals.var_tmf2_dn8 = assign100230_e152344_d_n8;
        locals.var_tmf2_dn9 = assign100230_e152344_d_n9;
        locals.var_tmf2_dn10 = assign100230_e152344_d_n10;
        locals.var_tmf2_dn11 = assign100230_e152344_d_n11;
        locals.var_tmf2_dn14 = assign100230_e152344_d_n14;

        let (assign100240_e152354, assign100240_e152354_d_n0, assign100240_e152354_d_n2, assign100240_e152354_d_n4, assign100240_e152354_d_n5, assign100240_e152354_d_n6, assign100240_e152354_d_n7, assign100240_e152354_d_n8, assign100240_e152354_d_n9, assign100240_e152354_d_n10, assign100240_e152354_d_n11, assign100240_e152354_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let (assign100240_e152352, assign100240_e152352_d_n0, assign100240_e152352_d_n2, assign100240_e152352_d_n4, assign100240_e152352_d_n5, assign100240_e152352_d_n6, assign100240_e152352_d_n7, assign100240_e152352_d_n8, assign100240_e152352_d_n9, assign100240_e152352_d_n10, assign100240_e152352_d_n11, assign100240_e152352_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign100240_e152351: f64 = (-locals.var_tmf2);
                (assign100240_e152351, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign100240_e152352, assign100240_e152352_d_n0, assign100240_e152352_d_n2, assign100240_e152352_d_n4, assign100240_e152352_d_n5, assign100240_e152352_d_n6, assign100240_e152352_d_n7, assign100240_e152352_d_n8, assign100240_e152352_d_n9, assign100240_e152352_d_n10, assign100240_e152352_d_n11, assign100240_e152352_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100240_e152354;
        locals.var_tmf2_dn0 = assign100240_e152354_d_n0;
        locals.var_tmf2_dn2 = assign100240_e152354_d_n2;
        locals.var_tmf2_dn4 = assign100240_e152354_d_n4;
        locals.var_tmf2_dn5 = assign100240_e152354_d_n5;
        locals.var_tmf2_dn6 = assign100240_e152354_d_n6;
        locals.var_tmf2_dn7 = assign100240_e152354_d_n7;
        locals.var_tmf2_dn8 = assign100240_e152354_d_n8;
        locals.var_tmf2_dn9 = assign100240_e152354_d_n9;
        locals.var_tmf2_dn10 = assign100240_e152354_d_n10;
        locals.var_tmf2_dn11 = assign100240_e152354_d_n11;
        locals.var_tmf2_dn14 = assign100240_e152354_d_n14;

        let (assign100250_e152363, assign100250_e152363_d_n0, assign100250_e152363_d_n2, assign100250_e152363_d_n4, assign100250_e152363_d_n5, assign100250_e152363_d_n6, assign100250_e152363_d_n7, assign100250_e152363_d_n8, assign100250_e152363_d_n9, assign100250_e152363_d_n10, assign100250_e152363_d_n11, assign100250_e152363_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100250_e152358: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign100250_e152360: f64 = (assign100250_e152358 + locals.var_tmf2);
        let assign100250_e152361: f64 = (assign100250_e152360).sqrt();
        (assign100250_e152361, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign100250_e152361)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign100250_e152361)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100250_e152363;
        locals.var_tmf2_dn0 = assign100250_e152363_d_n0;
        locals.var_tmf2_dn2 = assign100250_e152363_d_n2;
        locals.var_tmf2_dn4 = assign100250_e152363_d_n4;
        locals.var_tmf2_dn5 = assign100250_e152363_d_n5;
        locals.var_tmf2_dn6 = assign100250_e152363_d_n6;
        locals.var_tmf2_dn7 = assign100250_e152363_d_n7;
        locals.var_tmf2_dn8 = assign100250_e152363_d_n8;
        locals.var_tmf2_dn9 = assign100250_e152363_d_n9;
        locals.var_tmf2_dn10 = assign100250_e152363_d_n10;
        locals.var_tmf2_dn11 = assign100250_e152363_d_n11;
        locals.var_tmf2_dn14 = assign100250_e152363_d_n14;

    }

    pub(super) fn stamp_transient_block_369(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv18 = ctx.node_voltage(nodes[18]);
        let (assign100260_e152373, assign100260_e152373_d_n0, assign100260_e152373_d_n2, assign100260_e152373_d_n4, assign100260_e152373_d_n5, assign100260_e152373_d_n6, assign100260_e152373_d_n7, assign100260_e152373_d_n8, assign100260_e152373_d_n9, assign100260_e152373_d_n10, assign100260_e152373_d_n11, assign100260_e152373_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100260_e152369: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign100260_e152370: f64 = (1.0 + assign100260_e152369);
        let assign100260_e152371: f64 = (0.5 * assign100260_e152370);
        (assign100260_e152371, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100260_e152373;
        locals.var_t0_dn0 = assign100260_e152373_d_n0;
        locals.var_t0_dn2 = assign100260_e152373_d_n2;
        locals.var_t0_dn4 = assign100260_e152373_d_n4;
        locals.var_t0_dn5 = assign100260_e152373_d_n5;
        locals.var_t0_dn6 = assign100260_e152373_d_n6;
        locals.var_t0_dn7 = assign100260_e152373_d_n7;
        locals.var_t0_dn8 = assign100260_e152373_d_n8;
        locals.var_t0_dn9 = assign100260_e152373_d_n9;
        locals.var_t0_dn10 = assign100260_e152373_d_n10;
        locals.var_t0_dn11 = assign100260_e152373_d_n11;
        locals.var_t0_dn14 = assign100260_e152373_d_n14;

        let (assign100270_e152383, assign100270_e152383_d_n0, assign100270_e152383_d_n2, assign100270_e152383_d_n4, assign100270_e152383_d_n5, assign100270_e152383_d_n6, assign100270_e152383_d_n7, assign100270_e152383_d_n8, assign100270_e152383_d_n9, assign100270_e152383_d_n10, assign100270_e152383_d_n11, assign100270_e152383_d_n14,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100270_e152379: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign100270_e152380: f64 = (0.5 * assign100270_e152379);
        let assign100270_e152381: f64 = (p.p545 - assign100270_e152380);
        (assign100270_e152381, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    }
};
        locals.var_w_depa = assign100270_e152383;
        locals.var_w_depa_dn0 = assign100270_e152383_d_n0;
        locals.var_w_depa_dn2 = assign100270_e152383_d_n2;
        locals.var_w_depa_dn4 = assign100270_e152383_d_n4;
        locals.var_w_depa_dn5 = assign100270_e152383_d_n5;
        locals.var_w_depa_dn6 = assign100270_e152383_d_n6;
        locals.var_w_depa_dn7 = assign100270_e152383_d_n7;
        locals.var_w_depa_dn8 = assign100270_e152383_d_n8;
        locals.var_w_depa_dn9 = assign100270_e152383_d_n9;
        locals.var_w_depa_dn10 = assign100270_e152383_d_n10;
        locals.var_w_depa_dn11 = assign100270_e152383_d_n11;
        locals.var_w_depa_dn14 = assign100270_e152383_d_n14;

        let assign100280_e152386: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100280_e152386;

        let (assign100290_e152394, assign100290_e152394_d_n0, assign100290_e152394_d_n2, assign100290_e152394_d_n4, assign100290_e152394_d_n5, assign100290_e152394_d_n6, assign100290_e152394_d_n7, assign100290_e152394_d_n8, assign100290_e152394_d_n9, assign100290_e152394_d_n10, assign100290_e152394_d_n11, assign100290_e152394_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100290_e152392: f64 = (locals.var_w_depa * p.p546);
        (assign100290_e152392, (locals.var_w_depa_dn0 * p.p546), (locals.var_w_depa_dn2 * p.p546), (locals.var_w_depa_dn4 * p.p546), (locals.var_w_depa_dn5 * p.p546), (locals.var_w_depa_dn6 * p.p546), (locals.var_w_depa_dn7 * p.p546), (locals.var_w_depa_dn8 * p.p546), (locals.var_w_depa_dn9 * p.p546), (locals.var_w_depa_dn10 * p.p546), (locals.var_w_depa_dn11 * p.p546), (locals.var_w_depa_dn14 * p.p546),)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100290_e152394;
        locals.var_w_qs_a_dn0 = assign100290_e152394_d_n0;
        locals.var_w_qs_a_dn2 = assign100290_e152394_d_n2;
        locals.var_w_qs_a_dn4 = assign100290_e152394_d_n4;
        locals.var_w_qs_a_dn5 = assign100290_e152394_d_n5;
        locals.var_w_qs_a_dn6 = assign100290_e152394_d_n6;
        locals.var_w_qs_a_dn7 = assign100290_e152394_d_n7;
        locals.var_w_qs_a_dn8 = assign100290_e152394_d_n8;
        locals.var_w_qs_a_dn9 = assign100290_e152394_d_n9;
        locals.var_w_qs_a_dn10 = assign100290_e152394_d_n10;
        locals.var_w_qs_a_dn11 = assign100290_e152394_d_n11;
        locals.var_w_qs_a_dn14 = assign100290_e152394_d_n14;

        let (assign100300_e152402, assign100300_e152402_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100300_e152400: f64 = (p.p546 * (nv18 - 0.0));
        (assign100300_e152400, p.p546,)
    } else {
        (locals.var_w_nqs_a, locals.var_w_nqs_a_dn18,)
    }
};
        locals.var_w_nqs_a = assign100300_e152402;
        locals.var_w_nqs_a_dn18 = assign100300_e152402_d_n18;

        let (assign100310_e152412, assign100310_e152412_d_n0, assign100310_e152412_d_n2, assign100310_e152412_d_n4, assign100310_e152412_d_n5, assign100310_e152412_d_n6, assign100310_e152412_d_n7, assign100310_e152412_d_n8, assign100310_e152412_d_n9, assign100310_e152412_d_n10, assign100310_e152412_d_n11, assign100310_e152412_d_n14, assign100310_e152412_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100310_e152408: f64 = (locals.var_w_nqs_a - locals.var_w_qs_a);
        let assign100310_e152410: f64 = (assign100310_e152408 / p.p546);
        (assign100310_e152410, ((-locals.var_w_qs_a_dn0) / p.p546), ((-locals.var_w_qs_a_dn2) / p.p546), ((-locals.var_w_qs_a_dn4) / p.p546), ((-locals.var_w_qs_a_dn5) / p.p546), ((-locals.var_w_qs_a_dn6) / p.p546), ((-locals.var_w_qs_a_dn7) / p.p546), ((-locals.var_w_qs_a_dn8) / p.p546), ((-locals.var_w_qs_a_dn9) / p.p546), ((-locals.var_w_qs_a_dn10) / p.p546), ((-locals.var_w_qs_a_dn11) / p.p546), ((-locals.var_w_qs_a_dn14) / p.p546), (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_iwnqs0_a, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, locals.var_iwnqs0_a_dn18,)
    }
};
        locals.var_iwnqs0_a = assign100310_e152412;
        locals.var_iwnqs0_a_dn0 = assign100310_e152412_d_n0;
        locals.var_iwnqs0_a_dn2 = assign100310_e152412_d_n2;
        locals.var_iwnqs0_a_dn4 = assign100310_e152412_d_n4;
        locals.var_iwnqs0_a_dn5 = assign100310_e152412_d_n5;
        locals.var_iwnqs0_a_dn6 = assign100310_e152412_d_n6;
        locals.var_iwnqs0_a_dn7 = assign100310_e152412_d_n7;
        locals.var_iwnqs0_a_dn8 = assign100310_e152412_d_n8;
        locals.var_iwnqs0_a_dn9 = assign100310_e152412_d_n9;
        locals.var_iwnqs0_a_dn10 = assign100310_e152412_d_n10;
        locals.var_iwnqs0_a_dn11 = assign100310_e152412_d_n11;
        locals.var_iwnqs0_a_dn14 = assign100310_e152412_d_n14;
        locals.var_iwnqs0_a_dn18 = assign100310_e152412_d_n18;

        let (assign100320_e152420, assign100320_e152420_d_n0, assign100320_e152420_d_n2, assign100320_e152420_d_n4, assign100320_e152420_d_n5, assign100320_e152420_d_n6, assign100320_e152420_d_n7, assign100320_e152420_d_n8, assign100320_e152420_d_n9, assign100320_e152420_d_n10, assign100320_e152420_d_n11, assign100320_e152420_d_n14, assign100320_e152420_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100320_e152418: f64 = (locals.var_w_nqs_a / p.p546);
        (assign100320_e152418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_w_nqs_a_dn18 / p.p546),)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100320_e152420;
        locals.var_w_depa_nqs_dn0 = assign100320_e152420_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100320_e152420_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100320_e152420_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100320_e152420_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100320_e152420_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100320_e152420_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100320_e152420_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100320_e152420_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100320_e152420_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100320_e152420_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100320_e152420_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100320_e152420_d_n18;

        let (assign100330_e152427, assign100330_e152427_d_n0, assign100330_e152427_d_n2, assign100330_e152427_d_n4, assign100330_e152427_d_n5, assign100330_e152427_d_n6, assign100330_e152427_d_n7, assign100330_e152427_d_n8, assign100330_e152427_d_n9, assign100330_e152427_d_n10, assign100330_e152427_d_n11, assign100330_e152427_d_n14,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 == 0.0)) {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn11, locals.var_w_depa_dn14,)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14,)
    }
};
        locals.var_w_qs_a = assign100330_e152427;
        locals.var_w_qs_a_dn0 = assign100330_e152427_d_n0;
        locals.var_w_qs_a_dn2 = assign100330_e152427_d_n2;
        locals.var_w_qs_a_dn4 = assign100330_e152427_d_n4;
        locals.var_w_qs_a_dn5 = assign100330_e152427_d_n5;
        locals.var_w_qs_a_dn6 = assign100330_e152427_d_n6;
        locals.var_w_qs_a_dn7 = assign100330_e152427_d_n7;
        locals.var_w_qs_a_dn8 = assign100330_e152427_d_n8;
        locals.var_w_qs_a_dn9 = assign100330_e152427_d_n9;
        locals.var_w_qs_a_dn10 = assign100330_e152427_d_n10;
        locals.var_w_qs_a_dn11 = assign100330_e152427_d_n11;
        locals.var_w_qs_a_dn14 = assign100330_e152427_d_n14;

        let (assign100340_e152434, assign100340_e152434_d_n0, assign100340_e152434_d_n2, assign100340_e152434_d_n4, assign100340_e152434_d_n5, assign100340_e152434_d_n6, assign100340_e152434_d_n7, assign100340_e152434_d_n8, assign100340_e152434_d_n9, assign100340_e152434_d_n10, assign100340_e152434_d_n11, assign100340_e152434_d_n14, assign100340_e152434_d_n18,) = {
    if ((locals.var_guard2305 != 0.0) && (locals.var_guard2312 == 0.0)) {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn11, locals.var_w_qs_a_dn14, 0.0,)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn11, locals.var_w_depa_nqs_dn14, locals.var_w_depa_nqs_dn18,)
    }
};
        locals.var_w_depa_nqs = assign100340_e152434;
        locals.var_w_depa_nqs_dn0 = assign100340_e152434_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100340_e152434_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100340_e152434_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100340_e152434_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100340_e152434_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100340_e152434_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100340_e152434_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100340_e152434_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100340_e152434_d_n10;
        locals.var_w_depa_nqs_dn11 = assign100340_e152434_d_n11;
        locals.var_w_depa_nqs_dn14 = assign100340_e152434_d_n14;
        locals.var_w_depa_nqs_dn18 = assign100340_e152434_d_n18;

        let (assign100350_e152445,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100350_e152438: f64 = (locals.var_ndi_i * p.p13);
        let assign100350_e152440: f64 = (assign100350_e152438 * 1.6021918e-19);
        let assign100350_e152441: f64 = (-assign100350_e152440);
        let assign100350_e152443: f64 = (assign100350_e152441 * p.p545);
        (assign100350_e152443,)
    } else {
        (locals.var_q_n0,)
    }
};
        locals.var_q_n0 = assign100350_e152445;

        let (assign100360_e152463, assign100360_e152463_d_n0, assign100360_e152463_d_n2, assign100360_e152463_d_n4, assign100360_e152463_d_n5, assign100360_e152463_d_n6, assign100360_e152463_d_n7, assign100360_e152463_d_n8, assign100360_e152463_d_n9, assign100360_e152463_d_n10, assign100360_e152463_d_n11, assign100360_e152463_d_n14, assign100360_e152463_d_n16, assign100360_e152463_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100360_e152449: f64 = (locals.var_la * locals.var_q_pexa_nqs);
        let assign100360_e152451: f64 = (-p.p545);
        let assign100360_e152453: f64 = (assign100360_e152451 / locals.var_la);
        let assign100360_e152454: f64 = (assign100360_e152453).exp();
        let assign100360_e152456: f64 = (-locals.var_w_depa_nqs);
        let assign100360_e152458: f64 = (assign100360_e152456 / locals.var_la);
        let assign100360_e152459: f64 = (assign100360_e152458).exp();
        let assign100360_e152460: f64 = (assign100360_e152454 - assign100360_e152459);
        let assign100360_e152461: f64 = (assign100360_e152449 * assign100360_e152460);
        (assign100360_e152461, ((((locals.var_la_dn0 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn0)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn0) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn0) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn0)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn2 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn2)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn2) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn2) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn2)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn4 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn4)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn4) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn4) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn4)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn5 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn5)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn5) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn5) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn5)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn6 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn6)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn6) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn6) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn6)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn7 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn7)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn7) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn7) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn7)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn8 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn8)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn8) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn8) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn8)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn9 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn9)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn9) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn9) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn9)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn10 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn10)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn10) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn10) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn10)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn11 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn11)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn11) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn11) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn11)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn14 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn14)) * assign100360_e152460) + (assign100360_e152449 * ((assign100360_e152454 * (-((assign100360_e152451 * locals.var_la_dn14) / (locals.var_la * locals.var_la)))) - (assign100360_e152459 * ((((-locals.var_w_depa_nqs_dn14) * locals.var_la) - (assign100360_e152456 * locals.var_la_dn14)) / (locals.var_la * locals.var_la)))))), ((locals.var_la * locals.var_q_pexa_nqs_dn16) * assign100360_e152460), (assign100360_e152449 * (-(assign100360_e152459 * ((-locals.var_w_depa_nqs_dn18) / locals.var_la)))),)
    } else {
        (locals.var_q_nexa_nqs, locals.var_q_nexa_nqs_dn0, locals.var_q_nexa_nqs_dn2, locals.var_q_nexa_nqs_dn4, locals.var_q_nexa_nqs_dn5, locals.var_q_nexa_nqs_dn6, locals.var_q_nexa_nqs_dn7, locals.var_q_nexa_nqs_dn8, locals.var_q_nexa_nqs_dn9, locals.var_q_nexa_nqs_dn10, locals.var_q_nexa_nqs_dn11, locals.var_q_nexa_nqs_dn14, locals.var_q_nexa_nqs_dn16, locals.var_q_nexa_nqs_dn18,)
    }
};
        locals.var_q_nexa_nqs = assign100360_e152463;
        locals.var_q_nexa_nqs_dn0 = assign100360_e152463_d_n0;
        locals.var_q_nexa_nqs_dn2 = assign100360_e152463_d_n2;
        locals.var_q_nexa_nqs_dn4 = assign100360_e152463_d_n4;
        locals.var_q_nexa_nqs_dn5 = assign100360_e152463_d_n5;
        locals.var_q_nexa_nqs_dn6 = assign100360_e152463_d_n6;
        locals.var_q_nexa_nqs_dn7 = assign100360_e152463_d_n7;
        locals.var_q_nexa_nqs_dn8 = assign100360_e152463_d_n8;
        locals.var_q_nexa_nqs_dn9 = assign100360_e152463_d_n9;
        locals.var_q_nexa_nqs_dn10 = assign100360_e152463_d_n10;
        locals.var_q_nexa_nqs_dn11 = assign100360_e152463_d_n11;
        locals.var_q_nexa_nqs_dn14 = assign100360_e152463_d_n14;
        locals.var_q_nexa_nqs_dn16 = assign100360_e152463_d_n16;
        locals.var_q_nexa_nqs_dn18 = assign100360_e152463_d_n18;

        let (assign100370_e152479, assign100370_e152479_d_n0, assign100370_e152479_d_n2, assign100370_e152479_d_n4, assign100370_e152479_d_n5, assign100370_e152479_d_n6, assign100370_e152479_d_n7, assign100370_e152479_d_n8, assign100370_e152479_d_n9, assign100370_e152479_d_n10, assign100370_e152479_d_n11, assign100370_e152479_d_n14, assign100370_e152479_d_n17, assign100370_e152479_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100370_e152467: f64 = (locals.var_la * locals.var_q_pexk_nqs);
        let assign100370_e152470: f64 = (p.p545 - locals.var_w_depa_nqs);
        let assign100370_e152471: f64 = (-assign100370_e152470);
        let assign100370_e152473: f64 = (assign100370_e152471 / locals.var_la);
        let assign100370_e152474: f64 = (assign100370_e152473).exp();
        let assign100370_e152476: f64 = (assign100370_e152474 - 1.0);
        let assign100370_e152477: f64 = (assign100370_e152467 * assign100370_e152476);
        (assign100370_e152477, ((((locals.var_la_dn0 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn0)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn0)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn0)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn2 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn2)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn2)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn2)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn4 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn4)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn4)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn4)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn5 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn5)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn5)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn5)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn6 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn6)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn6)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn6)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn7 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn7)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn7)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn7)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn8 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn8)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn8)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn8)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn9 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn9)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn9)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn9)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn10 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn10)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn10)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn10)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn11 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn11)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn11)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn11)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn14 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn14)) * assign100370_e152476) + (assign100370_e152467 * (assign100370_e152474 * ((((-(-locals.var_w_depa_nqs_dn14)) * locals.var_la) - (assign100370_e152471 * locals.var_la_dn14)) / (locals.var_la * locals.var_la))))), ((locals.var_la * locals.var_q_pexk_nqs_dn17) * assign100370_e152476), (assign100370_e152467 * (assign100370_e152474 * ((-(-locals.var_w_depa_nqs_dn18)) / locals.var_la))),)
    } else {
        (locals.var_q_nexk_nqs, locals.var_q_nexk_nqs_dn0, locals.var_q_nexk_nqs_dn2, locals.var_q_nexk_nqs_dn4, locals.var_q_nexk_nqs_dn5, locals.var_q_nexk_nqs_dn6, locals.var_q_nexk_nqs_dn7, locals.var_q_nexk_nqs_dn8, locals.var_q_nexk_nqs_dn9, locals.var_q_nexk_nqs_dn10, locals.var_q_nexk_nqs_dn11, locals.var_q_nexk_nqs_dn14, locals.var_q_nexk_nqs_dn17, locals.var_q_nexk_nqs_dn18,)
    }
};
        locals.var_q_nexk_nqs = assign100370_e152479;
        locals.var_q_nexk_nqs_dn0 = assign100370_e152479_d_n0;
        locals.var_q_nexk_nqs_dn2 = assign100370_e152479_d_n2;
        locals.var_q_nexk_nqs_dn4 = assign100370_e152479_d_n4;
        locals.var_q_nexk_nqs_dn5 = assign100370_e152479_d_n5;
        locals.var_q_nexk_nqs_dn6 = assign100370_e152479_d_n6;
        locals.var_q_nexk_nqs_dn7 = assign100370_e152479_d_n7;
        locals.var_q_nexk_nqs_dn8 = assign100370_e152479_d_n8;
        locals.var_q_nexk_nqs_dn9 = assign100370_e152479_d_n9;
        locals.var_q_nexk_nqs_dn10 = assign100370_e152479_d_n10;
        locals.var_q_nexk_nqs_dn11 = assign100370_e152479_d_n11;
        locals.var_q_nexk_nqs_dn14 = assign100370_e152479_d_n14;
        locals.var_q_nexk_nqs_dn17 = assign100370_e152479_d_n17;
        locals.var_q_nexk_nqs_dn18 = assign100370_e152479_d_n18;

        let (assign100380_e152488, assign100380_e152488_d_n0, assign100380_e152488_d_n2, assign100380_e152488_d_n4, assign100380_e152488_d_n5, assign100380_e152488_d_n6, assign100380_e152488_d_n7, assign100380_e152488_d_n8, assign100380_e152488_d_n9, assign100380_e152488_d_n10, assign100380_e152488_d_n11, assign100380_e152488_d_n14, assign100380_e152488_d_n16, assign100380_e152488_d_n17, assign100380_e152488_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100380_e152483: f64 = (locals.var_q_n0 + locals.var_q_nexa_nqs);
        let assign100380_e152485: f64 = (assign100380_e152483 + locals.var_q_nexk_nqs);
        let assign100380_e152486: f64 = (-assign100380_e152485);
        (assign100380_e152486, (-(locals.var_q_nexa_nqs_dn0 + locals.var_q_nexk_nqs_dn0)), (-(locals.var_q_nexa_nqs_dn2 + locals.var_q_nexk_nqs_dn2)), (-(locals.var_q_nexa_nqs_dn4 + locals.var_q_nexk_nqs_dn4)), (-(locals.var_q_nexa_nqs_dn5 + locals.var_q_nexk_nqs_dn5)), (-(locals.var_q_nexa_nqs_dn6 + locals.var_q_nexk_nqs_dn6)), (-(locals.var_q_nexa_nqs_dn7 + locals.var_q_nexk_nqs_dn7)), (-(locals.var_q_nexa_nqs_dn8 + locals.var_q_nexk_nqs_dn8)), (-(locals.var_q_nexa_nqs_dn9 + locals.var_q_nexk_nqs_dn9)), (-(locals.var_q_nexa_nqs_dn10 + locals.var_q_nexk_nqs_dn10)), (-(locals.var_q_nexa_nqs_dn11 + locals.var_q_nexk_nqs_dn11)), (-(locals.var_q_nexa_nqs_dn14 + locals.var_q_nexk_nqs_dn14)), (-locals.var_q_nexa_nqs_dn16), (-locals.var_q_nexk_nqs_dn17), (-(locals.var_q_nexa_nqs_dn18 + locals.var_q_nexk_nqs_dn18)),)
    } else {
        (locals.var_qrr, locals.var_qrr_dn0, locals.var_qrr_dn2, locals.var_qrr_dn4, locals.var_qrr_dn5, locals.var_qrr_dn6, locals.var_qrr_dn7, locals.var_qrr_dn8, locals.var_qrr_dn9, locals.var_qrr_dn10, locals.var_qrr_dn11, locals.var_qrr_dn14, locals.var_qrr_dn16, locals.var_qrr_dn17, locals.var_qrr_dn18,)
    }
};
        locals.var_qrr = assign100380_e152488;
        locals.var_qrr_dn0 = assign100380_e152488_d_n0;
        locals.var_qrr_dn2 = assign100380_e152488_d_n2;
        locals.var_qrr_dn4 = assign100380_e152488_d_n4;
        locals.var_qrr_dn5 = assign100380_e152488_d_n5;
        locals.var_qrr_dn6 = assign100380_e152488_d_n6;
        locals.var_qrr_dn7 = assign100380_e152488_d_n7;
        locals.var_qrr_dn8 = assign100380_e152488_d_n8;
        locals.var_qrr_dn9 = assign100380_e152488_d_n9;
        locals.var_qrr_dn10 = assign100380_e152488_d_n10;
        locals.var_qrr_dn11 = assign100380_e152488_d_n11;
        locals.var_qrr_dn14 = assign100380_e152488_d_n14;
        locals.var_qrr_dn16 = assign100380_e152488_d_n16;
        locals.var_qrr_dn17 = assign100380_e152488_d_n17;
        locals.var_qrr_dn18 = assign100380_e152488_d_n18;

        let (assign100390_e152496, assign100390_e152496_d_n0, assign100390_e152496_d_n2, assign100390_e152496_d_n4, assign100390_e152496_d_n5, assign100390_e152496_d_n6, assign100390_e152496_d_n7, assign100390_e152496_d_n8, assign100390_e152496_d_n9, assign100390_e152496_d_n10, assign100390_e152496_d_n11, assign100390_e152496_d_n14, assign100390_e152496_d_n16, assign100390_e152496_d_n17, assign100390_e152496_d_n18,) = {
    if (locals.var_guard2305 != 0.0) {
        let assign100390_e152493: f64 = (locals.var_mfactor * locals.var_qrr);
        let assign100390_e152494: f64 = (locals.var_qbd + assign100390_e152493);
        (assign100390_e152494, (locals.var_qbd_dn0 + (locals.var_mfactor * locals.var_qrr_dn0)), (locals.var_qbd_dn2 + (locals.var_mfactor * locals.var_qrr_dn2)), (locals.var_qbd_dn4 + (locals.var_mfactor * locals.var_qrr_dn4)), (locals.var_qbd_dn5 + (locals.var_mfactor * locals.var_qrr_dn5)), (locals.var_qbd_dn6 + (locals.var_mfactor * locals.var_qrr_dn6)), (locals.var_qbd_dn7 + (locals.var_mfactor * locals.var_qrr_dn7)), (locals.var_qbd_dn8 + (locals.var_mfactor * locals.var_qrr_dn8)), (locals.var_qbd_dn9 + (locals.var_mfactor * locals.var_qrr_dn9)), (locals.var_qbd_dn10 + (locals.var_mfactor * locals.var_qrr_dn10)), (locals.var_qbd_dn11 + (locals.var_mfactor * locals.var_qrr_dn11)), (locals.var_qbd_dn14 + (locals.var_mfactor * locals.var_qrr_dn14)), (locals.var_qbd_dn16 + (locals.var_mfactor * locals.var_qrr_dn16)), (locals.var_qbd_dn17 + (locals.var_mfactor * locals.var_qrr_dn17)), (locals.var_qbd_dn18 + (locals.var_mfactor * locals.var_qrr_dn18)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn14, locals.var_qbd_dn16, locals.var_qbd_dn17, locals.var_qbd_dn18,)
    }
};
        locals.var_qbd = assign100390_e152496;
        locals.var_qbd_dn0 = assign100390_e152496_d_n0;
        locals.var_qbd_dn2 = assign100390_e152496_d_n2;
        locals.var_qbd_dn4 = assign100390_e152496_d_n4;
        locals.var_qbd_dn5 = assign100390_e152496_d_n5;
        locals.var_qbd_dn6 = assign100390_e152496_d_n6;
        locals.var_qbd_dn7 = assign100390_e152496_d_n7;
        locals.var_qbd_dn8 = assign100390_e152496_d_n8;
        locals.var_qbd_dn9 = assign100390_e152496_d_n9;
        locals.var_qbd_dn10 = assign100390_e152496_d_n10;
        locals.var_qbd_dn11 = assign100390_e152496_d_n11;
        locals.var_qbd_dn14 = assign100390_e152496_d_n14;
        locals.var_qbd_dn16 = assign100390_e152496_d_n16;
        locals.var_qbd_dn17 = assign100390_e152496_d_n17;
        locals.var_qbd_dn18 = assign100390_e152496_d_n18;

        let assign100400_e152503: f64 = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2313 = assign100400_e152503;

        let assign100410_e152510: f64 = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2314 = assign100410_e152510;

        let assign100420_e152513: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100420_e152513;

        let assign100430_e152520: f64 = if ((locals.var_uc_sub1snp > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100430_e152520;

        let (assign100440_e152528, assign100440_e152528_d_n0, assign100440_e152528_d_n2, assign100440_e152528_d_n4, assign100440_e152528_d_n5, assign100440_e152528_d_n6, assign100440_e152528_d_n7, assign100440_e152528_d_n8, assign100440_e152528_d_n9, assign100440_e152528_d_n10, assign100440_e152528_d_n11, assign100440_e152528_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100440_e152526: f64 = (locals.var_vg2const_1 * locals.var_vgp);
        (assign100440_e152526, ((locals.var_vg2const_1_dn0 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn0)), ((locals.var_vg2const_1_dn2 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn2)), ((locals.var_vg2const_1_dn4 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn4)), ((locals.var_vg2const_1_dn5 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn5)), ((locals.var_vg2const_1_dn6 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn6)), ((locals.var_vg2const_1_dn7 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn7)), ((locals.var_vg2const_1_dn8 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn8)), ((locals.var_vg2const_1_dn9 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn9)), ((locals.var_vg2const_1_dn10 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn10)), ((locals.var_vg2const_1_dn11 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn11)), ((locals.var_vg2const_1_dn14 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign100440_e152528;
        locals.var_t1_dn0 = assign100440_e152528_d_n0;
        locals.var_t1_dn2 = assign100440_e152528_d_n2;
        locals.var_t1_dn4 = assign100440_e152528_d_n4;
        locals.var_t1_dn5 = assign100440_e152528_d_n5;
        locals.var_t1_dn6 = assign100440_e152528_d_n6;
        locals.var_t1_dn7 = assign100440_e152528_d_n7;
        locals.var_t1_dn8 = assign100440_e152528_d_n8;
        locals.var_t1_dn9 = assign100440_e152528_d_n9;
        locals.var_t1_dn10 = assign100440_e152528_d_n10;
        locals.var_t1_dn11 = assign100440_e152528_d_n11;
        locals.var_t1_dn14 = assign100440_e152528_d_n14;

        let (assign100450_e152538, assign100450_e152538_d_n0, assign100450_e152538_d_n2, assign100450_e152538_d_n4, assign100450_e152538_d_n5, assign100450_e152538_d_n6, assign100450_e152538_d_n7, assign100450_e152538_d_n8, assign100450_e152538_d_n9, assign100450_e152538_d_n10, assign100450_e152538_d_n11, assign100450_e152538_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100450_e152535: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100450_e152536: f64 = (locals.var_qnsub_esi / assign100450_e152535);
        (assign100450_e152536, (locals.var_qnsub_esi_dn0 / assign100450_e152535), (locals.var_qnsub_esi_dn2 / assign100450_e152535), (locals.var_qnsub_esi_dn4 / assign100450_e152535), (locals.var_qnsub_esi_dn5 / assign100450_e152535), (locals.var_qnsub_esi_dn6 / assign100450_e152535), (locals.var_qnsub_esi_dn7 / assign100450_e152535), (locals.var_qnsub_esi_dn8 / assign100450_e152535), (locals.var_qnsub_esi_dn9 / assign100450_e152535), (locals.var_qnsub_esi_dn10 / assign100450_e152535), (locals.var_qnsub_esi_dn11 / assign100450_e152535), (locals.var_qnsub_esi_dn14 / assign100450_e152535),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign100450_e152538;
        locals.var_t3_dn0 = assign100450_e152538_d_n0;
        locals.var_t3_dn2 = assign100450_e152538_d_n2;
        locals.var_t3_dn4 = assign100450_e152538_d_n4;
        locals.var_t3_dn5 = assign100450_e152538_d_n5;
        locals.var_t3_dn6 = assign100450_e152538_d_n6;
        locals.var_t3_dn7 = assign100450_e152538_d_n7;
        locals.var_t3_dn8 = assign100450_e152538_d_n8;
        locals.var_t3_dn9 = assign100450_e152538_d_n9;
        locals.var_t3_dn10 = assign100450_e152538_d_n10;
        locals.var_t3_dn11 = assign100450_e152538_d_n11;
        locals.var_t3_dn14 = assign100450_e152538_d_n14;

        let (assign100460_e152550, assign100460_e152550_d_n0, assign100460_e152550_d_n2, assign100460_e152550_d_n4, assign100460_e152550_d_n5, assign100460_e152550_d_n6, assign100460_e152550_d_n7, assign100460_e152550_d_n8, assign100460_e152550_d_n9, assign100460_e152550_d_n10, assign100460_e152550_d_n11, assign100460_e152550_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100460_e152544: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100460_e152547: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100460_e152548: f64 = (assign100460_e152544 * assign100460_e152547);
        (assign100460_e152548, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547), ((-((2.0 * locals.var_qnsub_esi_dn14) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100460_e152547),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign100460_e152550;
        locals.var_t4_dn0 = assign100460_e152550_d_n0;
        locals.var_t4_dn2 = assign100460_e152550_d_n2;
        locals.var_t4_dn4 = assign100460_e152550_d_n4;
        locals.var_t4_dn5 = assign100460_e152550_d_n5;
        locals.var_t4_dn6 = assign100460_e152550_d_n6;
        locals.var_t4_dn7 = assign100460_e152550_d_n7;
        locals.var_t4_dn8 = assign100460_e152550_d_n8;
        locals.var_t4_dn9 = assign100460_e152550_d_n9;
        locals.var_t4_dn10 = assign100460_e152550_d_n10;
        locals.var_t4_dn11 = assign100460_e152550_d_n11;
        locals.var_t4_dn14 = assign100460_e152550_d_n14;

        let (assign100470_e152562, assign100470_e152562_d_n0, assign100470_e152562_d_n2, assign100470_e152562_d_n4, assign100470_e152562_d_n5, assign100470_e152562_d_n6, assign100470_e152562_d_n7, assign100470_e152562_d_n8, assign100470_e152562_d_n9, assign100470_e152562_d_n10, assign100470_e152562_d_n11, assign100470_e152562_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100470_e152556: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100470_e152559: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk442);
        let assign100470_e152560: f64 = (assign100470_e152556 - assign100470_e152559);
        (assign100470_e152560, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn10)), ((locals.var_t1_dn11 - locals.var_beta_inv_dn11) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn11)), ((locals.var_t1_dn14 - locals.var_beta_inv_dn14) - (locals.var_xvbs_1 * locals.var_vbsz__blk442_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign100470_e152562;
        locals.var_t5_dn0 = assign100470_e152562_d_n0;
        locals.var_t5_dn2 = assign100470_e152562_d_n2;
        locals.var_t5_dn4 = assign100470_e152562_d_n4;
        locals.var_t5_dn5 = assign100470_e152562_d_n5;
        locals.var_t5_dn6 = assign100470_e152562_d_n6;
        locals.var_t5_dn7 = assign100470_e152562_d_n7;
        locals.var_t5_dn8 = assign100470_e152562_d_n8;
        locals.var_t5_dn9 = assign100470_e152562_d_n9;
        locals.var_t5_dn10 = assign100470_e152562_d_n10;
        locals.var_t5_dn11 = assign100470_e152562_d_n11;
        locals.var_t5_dn14 = assign100470_e152562_d_n14;

        let (assign100480_e152572, assign100480_e152572_d_n0, assign100480_e152572_d_n2, assign100480_e152572_d_n4, assign100480_e152572_d_n5, assign100480_e152572_d_n6, assign100480_e152572_d_n7, assign100480_e152572_d_n8, assign100480_e152572_d_n9, assign100480_e152572_d_n10, assign100480_e152572_d_n11, assign100480_e152572_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100480_e152569: f64 = (locals.var_t4 * locals.var_t5);
        let assign100480_e152570: f64 = (1.0 + assign100480_e152569);
        (assign100480_e152570, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100480_e152572;
        locals.var_t6_dn0 = assign100480_e152572_d_n0;
        locals.var_t6_dn2 = assign100480_e152572_d_n2;
        locals.var_t6_dn4 = assign100480_e152572_d_n4;
        locals.var_t6_dn5 = assign100480_e152572_d_n5;
        locals.var_t6_dn6 = assign100480_e152572_d_n6;
        locals.var_t6_dn7 = assign100480_e152572_d_n7;
        locals.var_t6_dn8 = assign100480_e152572_d_n8;
        locals.var_t6_dn9 = assign100480_e152572_d_n9;
        locals.var_t6_dn10 = assign100480_e152572_d_n10;
        locals.var_t6_dn11 = assign100480_e152572_d_n11;
        locals.var_t6_dn14 = assign100480_e152572_d_n14;

        let (assign100490_e152582, assign100490_e152582_d_n0, assign100490_e152582_d_n2, assign100490_e152582_d_n4, assign100490_e152582_d_n5, assign100490_e152582_d_n6, assign100490_e152582_d_n7, assign100490_e152582_d_n8, assign100490_e152582_d_n9, assign100490_e152582_d_n10, assign100490_e152582_d_n11, assign100490_e152582_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100490_e152579: f64 = (1.0 + locals.var_t4);
        let assign100490_e152580: f64 = (2.0 * assign100490_e152579);
        (assign100490_e152580, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn11), (2.0 * locals.var_t4_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign100490_e152582;
        locals.var_t7_dn0 = assign100490_e152582_d_n0;
        locals.var_t7_dn2 = assign100490_e152582_d_n2;
        locals.var_t7_dn4 = assign100490_e152582_d_n4;
        locals.var_t7_dn5 = assign100490_e152582_d_n5;
        locals.var_t7_dn6 = assign100490_e152582_d_n6;
        locals.var_t7_dn7 = assign100490_e152582_d_n7;
        locals.var_t7_dn8 = assign100490_e152582_d_n8;
        locals.var_t7_dn9 = assign100490_e152582_d_n9;
        locals.var_t7_dn10 = assign100490_e152582_d_n10;
        locals.var_t7_dn11 = assign100490_e152582_d_n11;
        locals.var_t7_dn14 = assign100490_e152582_d_n14;

        let assign100500_e152586: f64 = locals.var_t7;
        let assign100500_e152591: f64 = if ((locals.var_t6 < assign100500_e152586) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100500_e152591;

        let (assign100510_e152603, assign100510_e152603_d_n0, assign100510_e152603_d_n2, assign100510_e152603_d_n4, assign100510_e152603_d_n5, assign100510_e152603_d_n6, assign100510_e152603_d_n7, assign100510_e152603_d_n8, assign100510_e152603_d_n9, assign100510_e152603_d_n10, assign100510_e152603_d_n11, assign100510_e152603_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100510_e152599: f64 = locals.var_t7;
        let assign100510_e152601: f64 = (assign100510_e152599 - locals.var_t6);
        (assign100510_e152601, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn11 - locals.var_t6_dn11), (locals.var_t7_dn14 - locals.var_t6_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign100510_e152603;
        locals.var_tmf1_dn0 = assign100510_e152603_d_n0;
        locals.var_tmf1_dn2 = assign100510_e152603_d_n2;
        locals.var_tmf1_dn4 = assign100510_e152603_d_n4;
        locals.var_tmf1_dn5 = assign100510_e152603_d_n5;
        locals.var_tmf1_dn6 = assign100510_e152603_d_n6;
        locals.var_tmf1_dn7 = assign100510_e152603_d_n7;
        locals.var_tmf1_dn8 = assign100510_e152603_d_n8;
        locals.var_tmf1_dn9 = assign100510_e152603_d_n9;
        locals.var_tmf1_dn10 = assign100510_e152603_d_n10;
        locals.var_tmf1_dn11 = assign100510_e152603_d_n11;
        locals.var_tmf1_dn14 = assign100510_e152603_d_n14;

        let (assign100520_e152613, assign100520_e152613_d_n0, assign100520_e152613_d_n2, assign100520_e152613_d_n4, assign100520_e152613_d_n5, assign100520_e152613_d_n6, assign100520_e152613_d_n7, assign100520_e152613_d_n8, assign100520_e152613_d_n9, assign100520_e152613_d_n10, assign100520_e152613_d_n11, assign100520_e152613_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100520_e152611: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100520_e152611, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign100520_e152613;
        locals.var_x2_dn0 = assign100520_e152613_d_n0;
        locals.var_x2_dn2 = assign100520_e152613_d_n2;
        locals.var_x2_dn4 = assign100520_e152613_d_n4;
        locals.var_x2_dn5 = assign100520_e152613_d_n5;
        locals.var_x2_dn6 = assign100520_e152613_d_n6;
        locals.var_x2_dn7 = assign100520_e152613_d_n7;
        locals.var_x2_dn8 = assign100520_e152613_d_n8;
        locals.var_x2_dn9 = assign100520_e152613_d_n9;
        locals.var_x2_dn10 = assign100520_e152613_d_n10;
        locals.var_x2_dn11 = assign100520_e152613_d_n11;
        locals.var_x2_dn14 = assign100520_e152613_d_n14;

        let (assign100530_e152623, assign100530_e152623_d_n0, assign100530_e152623_d_n2, assign100530_e152623_d_n4, assign100530_e152623_d_n5, assign100530_e152623_d_n6, assign100530_e152623_d_n7, assign100530_e152623_d_n8, assign100530_e152623_d_n9, assign100530_e152623_d_n10, assign100530_e152623_d_n11, assign100530_e152623_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100530_e152621: f64 = (locals.var_t7 * locals.var_t7);
        (assign100530_e152621, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)), ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign100530_e152623;
        locals.var_xmax2_dn0 = assign100530_e152623_d_n0;
        locals.var_xmax2_dn2 = assign100530_e152623_d_n2;
        locals.var_xmax2_dn4 = assign100530_e152623_d_n4;
        locals.var_xmax2_dn5 = assign100530_e152623_d_n5;
        locals.var_xmax2_dn6 = assign100530_e152623_d_n6;
        locals.var_xmax2_dn7 = assign100530_e152623_d_n7;
        locals.var_xmax2_dn8 = assign100530_e152623_d_n8;
        locals.var_xmax2_dn9 = assign100530_e152623_d_n9;
        locals.var_xmax2_dn10 = assign100530_e152623_d_n10;
        locals.var_xmax2_dn11 = assign100530_e152623_d_n11;
        locals.var_xmax2_dn14 = assign100530_e152623_d_n14;

    }

    pub(super) fn stamp_transient_block_370(
        locals: &mut StampLocals,
    ) {
        let (assign100540_e152631, assign100540_e152631_d_n0, assign100540_e152631_d_n2, assign100540_e152631_d_n4, assign100540_e152631_d_n5, assign100540_e152631_d_n6, assign100540_e152631_d_n7, assign100540_e152631_d_n8, assign100540_e152631_d_n9, assign100540_e152631_d_n10, assign100540_e152631_d_n11, assign100540_e152631_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100540_e152631;
        locals.var_xp_dn0 = assign100540_e152631_d_n0;
        locals.var_xp_dn2 = assign100540_e152631_d_n2;
        locals.var_xp_dn4 = assign100540_e152631_d_n4;
        locals.var_xp_dn5 = assign100540_e152631_d_n5;
        locals.var_xp_dn6 = assign100540_e152631_d_n6;
        locals.var_xp_dn7 = assign100540_e152631_d_n7;
        locals.var_xp_dn8 = assign100540_e152631_d_n8;
        locals.var_xp_dn9 = assign100540_e152631_d_n9;
        locals.var_xp_dn10 = assign100540_e152631_d_n10;
        locals.var_xp_dn11 = assign100540_e152631_d_n11;
        locals.var_xp_dn14 = assign100540_e152631_d_n14;

        let (assign100550_e152639, assign100550_e152639_d_n0, assign100550_e152639_d_n2, assign100550_e152639_d_n4, assign100550_e152639_d_n5, assign100550_e152639_d_n6, assign100550_e152639_d_n7, assign100550_e152639_d_n8, assign100550_e152639_d_n9, assign100550_e152639_d_n10, assign100550_e152639_d_n11, assign100550_e152639_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100550_e152639;
        locals.var_xmp_dn0 = assign100550_e152639_d_n0;
        locals.var_xmp_dn2 = assign100550_e152639_d_n2;
        locals.var_xmp_dn4 = assign100550_e152639_d_n4;
        locals.var_xmp_dn5 = assign100550_e152639_d_n5;
        locals.var_xmp_dn6 = assign100550_e152639_d_n6;
        locals.var_xmp_dn7 = assign100550_e152639_d_n7;
        locals.var_xmp_dn8 = assign100550_e152639_d_n8;
        locals.var_xmp_dn9 = assign100550_e152639_d_n9;
        locals.var_xmp_dn10 = assign100550_e152639_d_n10;
        locals.var_xmp_dn11 = assign100550_e152639_d_n11;
        locals.var_xmp_dn14 = assign100550_e152639_d_n14;

        let (assign100560_e152647,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100560_e152647;

        let (assign100570_e152655,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100570_e152655;

        let (assign100580_e152663, assign100580_e152663_d_n0, assign100580_e152663_d_n2, assign100580_e152663_d_n4, assign100580_e152663_d_n5, assign100580_e152663_d_n6, assign100580_e152663_d_n7, assign100580_e152663_d_n8, assign100580_e152663_d_n9, assign100580_e152663_d_n10, assign100580_e152663_d_n11, assign100580_e152663_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100580_e152663;
        locals.var_arg_dn0 = assign100580_e152663_d_n0;
        locals.var_arg_dn2 = assign100580_e152663_d_n2;
        locals.var_arg_dn4 = assign100580_e152663_d_n4;
        locals.var_arg_dn5 = assign100580_e152663_d_n5;
        locals.var_arg_dn6 = assign100580_e152663_d_n6;
        locals.var_arg_dn7 = assign100580_e152663_d_n7;
        locals.var_arg_dn8 = assign100580_e152663_d_n8;
        locals.var_arg_dn9 = assign100580_e152663_d_n9;
        locals.var_arg_dn10 = assign100580_e152663_d_n10;
        locals.var_arg_dn11 = assign100580_e152663_d_n11;
        locals.var_arg_dn14 = assign100580_e152663_d_n14;

        let (assign100590_e152671, assign100590_e152671_d_n0, assign100590_e152671_d_n2, assign100590_e152671_d_n4, assign100590_e152671_d_n5, assign100590_e152671_d_n6, assign100590_e152671_d_n7, assign100590_e152671_d_n8, assign100590_e152671_d_n9, assign100590_e152671_d_n10, assign100590_e152671_d_n11, assign100590_e152671_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100590_e152671;
        locals.var_dnm_dn0 = assign100590_e152671_d_n0;
        locals.var_dnm_dn2 = assign100590_e152671_d_n2;
        locals.var_dnm_dn4 = assign100590_e152671_d_n4;
        locals.var_dnm_dn5 = assign100590_e152671_d_n5;
        locals.var_dnm_dn6 = assign100590_e152671_d_n6;
        locals.var_dnm_dn7 = assign100590_e152671_d_n7;
        locals.var_dnm_dn8 = assign100590_e152671_d_n8;
        locals.var_dnm_dn9 = assign100590_e152671_d_n9;
        locals.var_dnm_dn10 = assign100590_e152671_d_n10;
        locals.var_dnm_dn11 = assign100590_e152671_d_n11;
        locals.var_dnm_dn14 = assign100590_e152671_d_n14;

        let (assign100600_e152681, assign100600_e152681_d_n0, assign100600_e152681_d_n2, assign100600_e152681_d_n4, assign100600_e152681_d_n5, assign100600_e152681_d_n6, assign100600_e152681_d_n7, assign100600_e152681_d_n8, assign100600_e152681_d_n9, assign100600_e152681_d_n10, assign100600_e152681_d_n11, assign100600_e152681_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100600_e152679: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152679, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100600_e152681;
        locals.var_xp_dn0 = assign100600_e152681_d_n0;
        locals.var_xp_dn2 = assign100600_e152681_d_n2;
        locals.var_xp_dn4 = assign100600_e152681_d_n4;
        locals.var_xp_dn5 = assign100600_e152681_d_n5;
        locals.var_xp_dn6 = assign100600_e152681_d_n6;
        locals.var_xp_dn7 = assign100600_e152681_d_n7;
        locals.var_xp_dn8 = assign100600_e152681_d_n8;
        locals.var_xp_dn9 = assign100600_e152681_d_n9;
        locals.var_xp_dn10 = assign100600_e152681_d_n10;
        locals.var_xp_dn11 = assign100600_e152681_d_n11;
        locals.var_xp_dn14 = assign100600_e152681_d_n14;

        let (assign100610_e152691, assign100610_e152691_d_n0, assign100610_e152691_d_n2, assign100610_e152691_d_n4, assign100610_e152691_d_n5, assign100610_e152691_d_n6, assign100610_e152691_d_n7, assign100610_e152691_d_n8, assign100610_e152691_d_n9, assign100610_e152691_d_n10, assign100610_e152691_d_n11, assign100610_e152691_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100610_e152689: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152689, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100610_e152691;
        locals.var_xmp_dn0 = assign100610_e152691_d_n0;
        locals.var_xmp_dn2 = assign100610_e152691_d_n2;
        locals.var_xmp_dn4 = assign100610_e152691_d_n4;
        locals.var_xmp_dn5 = assign100610_e152691_d_n5;
        locals.var_xmp_dn6 = assign100610_e152691_d_n6;
        locals.var_xmp_dn7 = assign100610_e152691_d_n7;
        locals.var_xmp_dn8 = assign100610_e152691_d_n8;
        locals.var_xmp_dn9 = assign100610_e152691_d_n9;
        locals.var_xmp_dn10 = assign100610_e152691_d_n10;
        locals.var_xmp_dn11 = assign100610_e152691_d_n11;
        locals.var_xmp_dn14 = assign100610_e152691_d_n14;

        let (assign100620_e152701, assign100620_e152701_d_n0, assign100620_e152701_d_n2, assign100620_e152701_d_n4, assign100620_e152701_d_n5, assign100620_e152701_d_n6, assign100620_e152701_d_n7, assign100620_e152701_d_n8, assign100620_e152701_d_n9, assign100620_e152701_d_n10, assign100620_e152701_d_n11, assign100620_e152701_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100620_e152699: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100620_e152701;
        locals.var_xp_dn0 = assign100620_e152701_d_n0;
        locals.var_xp_dn2 = assign100620_e152701_d_n2;
        locals.var_xp_dn4 = assign100620_e152701_d_n4;
        locals.var_xp_dn5 = assign100620_e152701_d_n5;
        locals.var_xp_dn6 = assign100620_e152701_d_n6;
        locals.var_xp_dn7 = assign100620_e152701_d_n7;
        locals.var_xp_dn8 = assign100620_e152701_d_n8;
        locals.var_xp_dn9 = assign100620_e152701_d_n9;
        locals.var_xp_dn10 = assign100620_e152701_d_n10;
        locals.var_xp_dn11 = assign100620_e152701_d_n11;
        locals.var_xp_dn14 = assign100620_e152701_d_n14;

        let (assign100630_e152711, assign100630_e152711_d_n0, assign100630_e152711_d_n2, assign100630_e152711_d_n4, assign100630_e152711_d_n5, assign100630_e152711_d_n6, assign100630_e152711_d_n7, assign100630_e152711_d_n8, assign100630_e152711_d_n9, assign100630_e152711_d_n10, assign100630_e152711_d_n11, assign100630_e152711_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100630_e152709: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152709, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100630_e152711;
        locals.var_xmp_dn0 = assign100630_e152711_d_n0;
        locals.var_xmp_dn2 = assign100630_e152711_d_n2;
        locals.var_xmp_dn4 = assign100630_e152711_d_n4;
        locals.var_xmp_dn5 = assign100630_e152711_d_n5;
        locals.var_xmp_dn6 = assign100630_e152711_d_n6;
        locals.var_xmp_dn7 = assign100630_e152711_d_n7;
        locals.var_xmp_dn8 = assign100630_e152711_d_n8;
        locals.var_xmp_dn9 = assign100630_e152711_d_n9;
        locals.var_xmp_dn10 = assign100630_e152711_d_n10;
        locals.var_xmp_dn11 = assign100630_e152711_d_n11;
        locals.var_xmp_dn14 = assign100630_e152711_d_n14;

        let (assign100640_e152721, assign100640_e152721_d_n0, assign100640_e152721_d_n2, assign100640_e152721_d_n4, assign100640_e152721_d_n5, assign100640_e152721_d_n6, assign100640_e152721_d_n7, assign100640_e152721_d_n8, assign100640_e152721_d_n9, assign100640_e152721_d_n10, assign100640_e152721_d_n11, assign100640_e152721_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100640_e152719: f64 = (locals.var_xp * locals.var_x2);
        (assign100640_e152719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100640_e152721;
        locals.var_xp_dn0 = assign100640_e152721_d_n0;
        locals.var_xp_dn2 = assign100640_e152721_d_n2;
        locals.var_xp_dn4 = assign100640_e152721_d_n4;
        locals.var_xp_dn5 = assign100640_e152721_d_n5;
        locals.var_xp_dn6 = assign100640_e152721_d_n6;
        locals.var_xp_dn7 = assign100640_e152721_d_n7;
        locals.var_xp_dn8 = assign100640_e152721_d_n8;
        locals.var_xp_dn9 = assign100640_e152721_d_n9;
        locals.var_xp_dn10 = assign100640_e152721_d_n10;
        locals.var_xp_dn11 = assign100640_e152721_d_n11;
        locals.var_xp_dn14 = assign100640_e152721_d_n14;

        let (assign100650_e152731, assign100650_e152731_d_n0, assign100650_e152731_d_n2, assign100650_e152731_d_n4, assign100650_e152731_d_n5, assign100650_e152731_d_n6, assign100650_e152731_d_n7, assign100650_e152731_d_n8, assign100650_e152731_d_n9, assign100650_e152731_d_n10, assign100650_e152731_d_n11, assign100650_e152731_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100650_e152729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100650_e152729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100650_e152731;
        locals.var_xmp_dn0 = assign100650_e152731_d_n0;
        locals.var_xmp_dn2 = assign100650_e152731_d_n2;
        locals.var_xmp_dn4 = assign100650_e152731_d_n4;
        locals.var_xmp_dn5 = assign100650_e152731_d_n5;
        locals.var_xmp_dn6 = assign100650_e152731_d_n6;
        locals.var_xmp_dn7 = assign100650_e152731_d_n7;
        locals.var_xmp_dn8 = assign100650_e152731_d_n8;
        locals.var_xmp_dn9 = assign100650_e152731_d_n9;
        locals.var_xmp_dn10 = assign100650_e152731_d_n10;
        locals.var_xmp_dn11 = assign100650_e152731_d_n11;
        locals.var_xmp_dn14 = assign100650_e152731_d_n14;

        let (assign100660_e152741, assign100660_e152741_d_n0, assign100660_e152741_d_n2, assign100660_e152741_d_n4, assign100660_e152741_d_n5, assign100660_e152741_d_n6, assign100660_e152741_d_n7, assign100660_e152741_d_n8, assign100660_e152741_d_n9, assign100660_e152741_d_n10, assign100660_e152741_d_n11, assign100660_e152741_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100660_e152739: f64 = (locals.var_xp * locals.var_x2);
        (assign100660_e152739, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign100660_e152741;
        locals.var_xp_dn0 = assign100660_e152741_d_n0;
        locals.var_xp_dn2 = assign100660_e152741_d_n2;
        locals.var_xp_dn4 = assign100660_e152741_d_n4;
        locals.var_xp_dn5 = assign100660_e152741_d_n5;
        locals.var_xp_dn6 = assign100660_e152741_d_n6;
        locals.var_xp_dn7 = assign100660_e152741_d_n7;
        locals.var_xp_dn8 = assign100660_e152741_d_n8;
        locals.var_xp_dn9 = assign100660_e152741_d_n9;
        locals.var_xp_dn10 = assign100660_e152741_d_n10;
        locals.var_xp_dn11 = assign100660_e152741_d_n11;
        locals.var_xp_dn14 = assign100660_e152741_d_n14;

        let (assign100670_e152751, assign100670_e152751_d_n0, assign100670_e152751_d_n2, assign100670_e152751_d_n4, assign100670_e152751_d_n5, assign100670_e152751_d_n6, assign100670_e152751_d_n7, assign100670_e152751_d_n8, assign100670_e152751_d_n9, assign100670_e152751_d_n10, assign100670_e152751_d_n11, assign100670_e152751_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100670_e152749: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100670_e152749, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign100670_e152751;
        locals.var_xmp_dn0 = assign100670_e152751_d_n0;
        locals.var_xmp_dn2 = assign100670_e152751_d_n2;
        locals.var_xmp_dn4 = assign100670_e152751_d_n4;
        locals.var_xmp_dn5 = assign100670_e152751_d_n5;
        locals.var_xmp_dn6 = assign100670_e152751_d_n6;
        locals.var_xmp_dn7 = assign100670_e152751_d_n7;
        locals.var_xmp_dn8 = assign100670_e152751_d_n8;
        locals.var_xmp_dn9 = assign100670_e152751_d_n9;
        locals.var_xmp_dn10 = assign100670_e152751_d_n10;
        locals.var_xmp_dn11 = assign100670_e152751_d_n11;
        locals.var_xmp_dn14 = assign100670_e152751_d_n14;

        let (assign100680_e152761, assign100680_e152761_d_n0, assign100680_e152761_d_n2, assign100680_e152761_d_n4, assign100680_e152761_d_n5, assign100680_e152761_d_n6, assign100680_e152761_d_n7, assign100680_e152761_d_n8, assign100680_e152761_d_n9, assign100680_e152761_d_n10, assign100680_e152761_d_n11, assign100680_e152761_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100680_e152759: f64 = (locals.var_xp + locals.var_xmp);
        (assign100680_e152759, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign100680_e152761;
        locals.var_arg_dn0 = assign100680_e152761_d_n0;
        locals.var_arg_dn2 = assign100680_e152761_d_n2;
        locals.var_arg_dn4 = assign100680_e152761_d_n4;
        locals.var_arg_dn5 = assign100680_e152761_d_n5;
        locals.var_arg_dn6 = assign100680_e152761_d_n6;
        locals.var_arg_dn7 = assign100680_e152761_d_n7;
        locals.var_arg_dn8 = assign100680_e152761_d_n8;
        locals.var_arg_dn9 = assign100680_e152761_d_n9;
        locals.var_arg_dn10 = assign100680_e152761_d_n10;
        locals.var_arg_dn11 = assign100680_e152761_d_n11;
        locals.var_arg_dn14 = assign100680_e152761_d_n14;

        let (assign100690_e152769, assign100690_e152769_d_n0, assign100690_e152769_d_n2, assign100690_e152769_d_n4, assign100690_e152769_d_n5, assign100690_e152769_d_n6, assign100690_e152769_d_n7, assign100690_e152769_d_n8, assign100690_e152769_d_n9, assign100690_e152769_d_n10, assign100690_e152769_d_n11, assign100690_e152769_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100690_e152769;
        locals.var_dnm_dn0 = assign100690_e152769_d_n0;
        locals.var_dnm_dn2 = assign100690_e152769_d_n2;
        locals.var_dnm_dn4 = assign100690_e152769_d_n4;
        locals.var_dnm_dn5 = assign100690_e152769_d_n5;
        locals.var_dnm_dn6 = assign100690_e152769_d_n6;
        locals.var_dnm_dn7 = assign100690_e152769_d_n7;
        locals.var_dnm_dn8 = assign100690_e152769_d_n8;
        locals.var_dnm_dn9 = assign100690_e152769_d_n9;
        locals.var_dnm_dn10 = assign100690_e152769_d_n10;
        locals.var_dnm_dn11 = assign100690_e152769_d_n11;
        locals.var_dnm_dn14 = assign100690_e152769_d_n14;

        let assign100700_e152784: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100700_e152784;

        let assign100710_e152787: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2319 = assign100710_e152787;

        let (assign100720_e152799,) = {
    if (((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152799;

        let assign100730_e152802: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2320 = assign100730_e152802;

        let (assign100740_e152817,) = {
    if ((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152817;

        let assign100750_e152820: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2321 = assign100750_e152820;

        let (assign100760_e152838,) = {
    if (((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 == 0.0)) && (locals.var_guard2321 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100760_e152838;

        let assign100770_e152841: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2322 = assign100770_e152841;

        let (assign100780_e152862,) = {
    if ((((((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_guard2319 == 0.0)) && (locals.var_guard2320 == 0.0)) && (locals.var_guard2321 == 0.0)) && (locals.var_guard2322 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100780_e152862;

        let (assign100790_e152872,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100790_e152872;

        let mut assign100800_loop_guard: usize = 0;
        while {
            let assign100800_cond_e152883: f64 = if (((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100800_cond_e152883 != 0.0
        } {
            assign100800_loop_guard += 1;
            assert!(assign100800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100800_body0_e152894, assign100800_body0_e152894_d_n0, assign100800_body0_e152894_d_n2, assign100800_body0_e152894_d_n4, assign100800_body0_e152894_d_n5, assign100800_body0_e152894_d_n6, assign100800_body0_e152894_d_n7, assign100800_body0_e152894_d_n8, assign100800_body0_e152894_d_n9, assign100800_body0_e152894_d_n10, assign100800_body0_e152894_d_n11, assign100800_body0_e152894_d_n14,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        let assign100800_body0_e152892: f64 = (locals.var_dnm).sqrt();
        (assign100800_body0_e152892, (locals.var_dnm_dn0 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn2 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn4 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn5 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn6 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn7 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn8 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn9 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn10 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn11 / (2.0 * assign100800_body0_e152892)), (locals.var_dnm_dn14 / (2.0 * assign100800_body0_e152892)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign100800_body0_e152894;
            locals.var_dnm_dn0 = assign100800_body0_e152894_d_n0;
            locals.var_dnm_dn2 = assign100800_body0_e152894_d_n2;
            locals.var_dnm_dn4 = assign100800_body0_e152894_d_n4;
            locals.var_dnm_dn5 = assign100800_body0_e152894_d_n5;
            locals.var_dnm_dn6 = assign100800_body0_e152894_d_n6;
            locals.var_dnm_dn7 = assign100800_body0_e152894_d_n7;
            locals.var_dnm_dn8 = assign100800_body0_e152894_d_n8;
            locals.var_dnm_dn9 = assign100800_body0_e152894_d_n9;
            locals.var_dnm_dn10 = assign100800_body0_e152894_d_n10;
            locals.var_dnm_dn11 = assign100800_body0_e152894_d_n11;
            locals.var_dnm_dn14 = assign100800_body0_e152894_d_n14;
            let (assign100800_body1_e152906,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 != 0.0)) {
        let assign100800_body1_e152904: f64 = (locals.var_m0 + 1.0);
        (assign100800_body1_e152904,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100800_body1_e152906;
        }

        let (assign100810_e152928, assign100810_e152928_d_n0, assign100810_e152928_d_n2, assign100810_e152928_d_n4, assign100810_e152928_d_n5, assign100810_e152928_d_n6, assign100810_e152928_d_n7, assign100810_e152928_d_n8, assign100810_e152928_d_n9, assign100810_e152928_d_n10, assign100810_e152928_d_n11, assign100810_e152928_d_n14,) = {
    if ((((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) && (locals.var_guard2318 == 0.0)) {
        let (assign100810_e152926, assign100810_e152926_d_n0, assign100810_e152926_d_n2, assign100810_e152926_d_n4, assign100810_e152926_d_n5, assign100810_e152926_d_n6, assign100810_e152926_d_n7, assign100810_e152926_d_n8, assign100810_e152926_d_n9, assign100810_e152926_d_n10, assign100810_e152926_d_n11, assign100810_e152926_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100810_e152923: f64 = (2.0 * 4.0);
                let assign100810_e152924: f64 = (1.0 / assign100810_e152923);
                let assign100810_e152925: f64 = (locals.var_dnm).powf(assign100810_e152924);
                (assign100810_e152925, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn11)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100810_e152924) as f64).is_finite() && ((assign100810_e152924) as f64).fract() == 0.0 { if assign100810_e152924 == 0.0 { 0.0 } else { (assign100810_e152924 * ((locals.var_dnm).powf(assign100810_e152924 - 1.0) * locals.var_dnm_dn14)) } } else { (assign100810_e152925 * (assign100810_e152924 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign100810_e152926, assign100810_e152926_d_n0, assign100810_e152926_d_n2, assign100810_e152926_d_n4, assign100810_e152926_d_n5, assign100810_e152926_d_n6, assign100810_e152926_d_n7, assign100810_e152926_d_n8, assign100810_e152926_d_n9, assign100810_e152926_d_n10, assign100810_e152926_d_n11, assign100810_e152926_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100810_e152928;
        locals.var_dnm_dn0 = assign100810_e152928_d_n0;
        locals.var_dnm_dn2 = assign100810_e152928_d_n2;
        locals.var_dnm_dn4 = assign100810_e152928_d_n4;
        locals.var_dnm_dn5 = assign100810_e152928_d_n5;
        locals.var_dnm_dn6 = assign100810_e152928_d_n6;
        locals.var_dnm_dn7 = assign100810_e152928_d_n7;
        locals.var_dnm_dn8 = assign100810_e152928_d_n8;
        locals.var_dnm_dn9 = assign100810_e152928_d_n9;
        locals.var_dnm_dn10 = assign100810_e152928_d_n10;
        locals.var_dnm_dn11 = assign100810_e152928_d_n11;
        locals.var_dnm_dn14 = assign100810_e152928_d_n14;

        let (assign100820_e152938, assign100820_e152938_d_n0, assign100820_e152938_d_n2, assign100820_e152938_d_n4, assign100820_e152938_d_n5, assign100820_e152938_d_n6, assign100820_e152938_d_n7, assign100820_e152938_d_n8, assign100820_e152938_d_n9, assign100820_e152938_d_n10, assign100820_e152938_d_n11, assign100820_e152938_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100820_e152936: f64 = (1.0 / locals.var_dnm);
        (assign100820_e152936, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign100820_e152938;
        locals.var_dnm_dn0 = assign100820_e152938_d_n0;
        locals.var_dnm_dn2 = assign100820_e152938_d_n2;
        locals.var_dnm_dn4 = assign100820_e152938_d_n4;
        locals.var_dnm_dn5 = assign100820_e152938_d_n5;
        locals.var_dnm_dn6 = assign100820_e152938_d_n6;
        locals.var_dnm_dn7 = assign100820_e152938_d_n7;
        locals.var_dnm_dn8 = assign100820_e152938_d_n8;
        locals.var_dnm_dn9 = assign100820_e152938_d_n9;
        locals.var_dnm_dn10 = assign100820_e152938_d_n10;
        locals.var_dnm_dn11 = assign100820_e152938_d_n11;
        locals.var_dnm_dn14 = assign100820_e152938_d_n14;

        let (assign100830_e152950, assign100830_e152950_d_n0, assign100830_e152950_d_n2, assign100830_e152950_d_n4, assign100830_e152950_d_n5, assign100830_e152950_d_n6, assign100830_e152950_d_n7, assign100830_e152950_d_n8, assign100830_e152950_d_n9, assign100830_e152950_d_n10, assign100830_e152950_d_n11, assign100830_e152950_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100830_e152946: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100830_e152948: f64 = (assign100830_e152946 * locals.var_dnm);
        (assign100830_e152948, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn11)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn14)) * locals.var_dnm) + (assign100830_e152946 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign100830_e152950;
        locals.var_tmf0_dn0 = assign100830_e152950_d_n0;
        locals.var_tmf0_dn2 = assign100830_e152950_d_n2;
        locals.var_tmf0_dn4 = assign100830_e152950_d_n4;
        locals.var_tmf0_dn5 = assign100830_e152950_d_n5;
        locals.var_tmf0_dn6 = assign100830_e152950_d_n6;
        locals.var_tmf0_dn7 = assign100830_e152950_d_n7;
        locals.var_tmf0_dn8 = assign100830_e152950_d_n8;
        locals.var_tmf0_dn9 = assign100830_e152950_d_n9;
        locals.var_tmf0_dn10 = assign100830_e152950_d_n10;
        locals.var_tmf0_dn11 = assign100830_e152950_d_n11;
        locals.var_tmf0_dn14 = assign100830_e152950_d_n14;

        let (assign100840_e152964, assign100840_e152964_d_n0, assign100840_e152964_d_n2, assign100840_e152964_d_n4, assign100840_e152964_d_n5, assign100840_e152964_d_n6, assign100840_e152964_d_n7, assign100840_e152964_d_n8, assign100840_e152964_d_n9, assign100840_e152964_d_n10, assign100840_e152964_d_n11, assign100840_e152964_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100840_e152958: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100840_e152960: f64 = (assign100840_e152958 * locals.var_dnm);
        let assign100840_e152962: f64 = (assign100840_e152960 / locals.var_arg);
        (assign100840_e152962, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn11 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn11)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn14 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign100840_e152958 * locals.var_dnm_dn14)) * locals.var_arg) - (assign100840_e152960 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100840_e152964;
        locals.var_t0_dn0 = assign100840_e152964_d_n0;
        locals.var_t0_dn2 = assign100840_e152964_d_n2;
        locals.var_t0_dn4 = assign100840_e152964_d_n4;
        locals.var_t0_dn5 = assign100840_e152964_d_n5;
        locals.var_t0_dn6 = assign100840_e152964_d_n6;
        locals.var_t0_dn7 = assign100840_e152964_d_n7;
        locals.var_t0_dn8 = assign100840_e152964_d_n8;
        locals.var_t0_dn9 = assign100840_e152964_d_n9;
        locals.var_t0_dn10 = assign100840_e152964_d_n10;
        locals.var_t0_dn11 = assign100840_e152964_d_n11;
        locals.var_t0_dn14 = assign100840_e152964_d_n14;

    }

    pub(super) fn stamp_transient_block_371(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100850_e152976, assign100850_e152976_d_n0, assign100850_e152976_d_n2, assign100850_e152976_d_n4, assign100850_e152976_d_n5, assign100850_e152976_d_n6, assign100850_e152976_d_n7, assign100850_e152976_d_n8, assign100850_e152976_d_n9, assign100850_e152976_d_n10, assign100850_e152976_d_n11, assign100850_e152976_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        let assign100850_e152972: f64 = locals.var_t7;
        let assign100850_e152974: f64 = (assign100850_e152972 - locals.var_tmf0);
        (assign100850_e152974, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn11 - locals.var_tmf0_dn11), (locals.var_t7_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100850_e152976;
        locals.var_t6_dn0 = assign100850_e152976_d_n0;
        locals.var_t6_dn2 = assign100850_e152976_d_n2;
        locals.var_t6_dn4 = assign100850_e152976_d_n4;
        locals.var_t6_dn5 = assign100850_e152976_d_n5;
        locals.var_t6_dn6 = assign100850_e152976_d_n6;
        locals.var_t6_dn7 = assign100850_e152976_d_n7;
        locals.var_t6_dn8 = assign100850_e152976_d_n8;
        locals.var_t6_dn9 = assign100850_e152976_d_n9;
        locals.var_t6_dn10 = assign100850_e152976_d_n10;
        locals.var_t6_dn11 = assign100850_e152976_d_n11;
        locals.var_t6_dn14 = assign100850_e152976_d_n14;

        let (assign100860_e152984, assign100860_e152984_d_n0, assign100860_e152984_d_n2, assign100860_e152984_d_n4, assign100860_e152984_d_n5, assign100860_e152984_d_n6, assign100860_e152984_d_n7, assign100860_e152984_d_n8, assign100860_e152984_d_n9, assign100860_e152984_d_n10, assign100860_e152984_d_n11, assign100860_e152984_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100860_e152984;
        locals.var_t0_dn0 = assign100860_e152984_d_n0;
        locals.var_t0_dn2 = assign100860_e152984_d_n2;
        locals.var_t0_dn4 = assign100860_e152984_d_n4;
        locals.var_t0_dn5 = assign100860_e152984_d_n5;
        locals.var_t0_dn6 = assign100860_e152984_d_n6;
        locals.var_t0_dn7 = assign100860_e152984_d_n7;
        locals.var_t0_dn8 = assign100860_e152984_d_n8;
        locals.var_t0_dn9 = assign100860_e152984_d_n9;
        locals.var_t0_dn10 = assign100860_e152984_d_n10;
        locals.var_t0_dn11 = assign100860_e152984_d_n11;
        locals.var_t0_dn14 = assign100860_e152984_d_n14;

        let (assign100870_e152993, assign100870_e152993_d_n0, assign100870_e152993_d_n2, assign100870_e152993_d_n4, assign100870_e152993_d_n5, assign100870_e152993_d_n6, assign100870_e152993_d_n7, assign100870_e152993_d_n8, assign100870_e152993_d_n9, assign100870_e152993_d_n10, assign100870_e152993_d_n11, assign100870_e152993_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100870_e152993;
        locals.var_t6_dn0 = assign100870_e152993_d_n0;
        locals.var_t6_dn2 = assign100870_e152993_d_n2;
        locals.var_t6_dn4 = assign100870_e152993_d_n4;
        locals.var_t6_dn5 = assign100870_e152993_d_n5;
        locals.var_t6_dn6 = assign100870_e152993_d_n6;
        locals.var_t6_dn7 = assign100870_e152993_d_n7;
        locals.var_t6_dn8 = assign100870_e152993_d_n8;
        locals.var_t6_dn9 = assign100870_e152993_d_n9;
        locals.var_t6_dn10 = assign100870_e152993_d_n10;
        locals.var_t6_dn11 = assign100870_e152993_d_n11;
        locals.var_t6_dn14 = assign100870_e152993_d_n14;

        let (assign100880_e153002, assign100880_e153002_d_n0, assign100880_e153002_d_n2, assign100880_e153002_d_n4, assign100880_e153002_d_n5, assign100880_e153002_d_n6, assign100880_e153002_d_n7, assign100880_e153002_d_n8, assign100880_e153002_d_n9, assign100880_e153002_d_n10, assign100880_e153002_d_n11, assign100880_e153002_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2317 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign100880_e153002;
        locals.var_t0_dn0 = assign100880_e153002_d_n0;
        locals.var_t0_dn2 = assign100880_e153002_d_n2;
        locals.var_t0_dn4 = assign100880_e153002_d_n4;
        locals.var_t0_dn5 = assign100880_e153002_d_n5;
        locals.var_t0_dn6 = assign100880_e153002_d_n6;
        locals.var_t0_dn7 = assign100880_e153002_d_n7;
        locals.var_t0_dn8 = assign100880_e153002_d_n8;
        locals.var_t0_dn9 = assign100880_e153002_d_n9;
        locals.var_t0_dn10 = assign100880_e153002_d_n10;
        locals.var_t0_dn11 = assign100880_e153002_d_n11;
        locals.var_t0_dn14 = assign100880_e153002_d_n14;

        let (assign100890_e153009, assign100890_e153009_d_n0, assign100890_e153009_d_n2, assign100890_e153009_d_n4, assign100890_e153009_d_n5, assign100890_e153009_d_n6, assign100890_e153009_d_n7, assign100890_e153009_d_n8, assign100890_e153009_d_n9, assign100890_e153009_d_n10, assign100890_e153009_d_n11, assign100890_e153009_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100890_e153007: f64 = (locals.var_t6).sqrt();
        (assign100890_e153007, (locals.var_t6_dn0 / (2.0 * assign100890_e153007)), (locals.var_t6_dn2 / (2.0 * assign100890_e153007)), (locals.var_t6_dn4 / (2.0 * assign100890_e153007)), (locals.var_t6_dn5 / (2.0 * assign100890_e153007)), (locals.var_t6_dn6 / (2.0 * assign100890_e153007)), (locals.var_t6_dn7 / (2.0 * assign100890_e153007)), (locals.var_t6_dn8 / (2.0 * assign100890_e153007)), (locals.var_t6_dn9 / (2.0 * assign100890_e153007)), (locals.var_t6_dn10 / (2.0 * assign100890_e153007)), (locals.var_t6_dn11 / (2.0 * assign100890_e153007)), (locals.var_t6_dn14 / (2.0 * assign100890_e153007)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign100890_e153009;
        locals.var_t6_dn0 = assign100890_e153009_d_n0;
        locals.var_t6_dn2 = assign100890_e153009_d_n2;
        locals.var_t6_dn4 = assign100890_e153009_d_n4;
        locals.var_t6_dn5 = assign100890_e153009_d_n5;
        locals.var_t6_dn6 = assign100890_e153009_d_n6;
        locals.var_t6_dn7 = assign100890_e153009_d_n7;
        locals.var_t6_dn8 = assign100890_e153009_d_n8;
        locals.var_t6_dn9 = assign100890_e153009_d_n9;
        locals.var_t6_dn10 = assign100890_e153009_d_n10;
        locals.var_t6_dn11 = assign100890_e153009_d_n11;
        locals.var_t6_dn14 = assign100890_e153009_d_n14;

        let (assign100900_e153021, assign100900_e153021_d_n0, assign100900_e153021_d_n2, assign100900_e153021_d_n4, assign100900_e153021_d_n5, assign100900_e153021_d_n6, assign100900_e153021_d_n7, assign100900_e153021_d_n8, assign100900_e153021_d_n9, assign100900_e153021_d_n10, assign100900_e153021_d_n11, assign100900_e153021_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100900_e153017: f64 = (1.0 - locals.var_t6);
        let assign100900_e153018: f64 = (locals.var_t3 * assign100900_e153017);
        let assign100900_e153019: f64 = (locals.var_t1 + assign100900_e153018);
        (assign100900_e153019, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn11 + ((locals.var_t3_dn11 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn11)))), (locals.var_t1_dn14 + ((locals.var_t3_dn14 * assign100900_e153017) + (locals.var_t3 * (-locals.var_t6_dn14)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn14,)
    }
};
        locals.var_psislsat = assign100900_e153021;
        locals.var_psislsat_dn0 = assign100900_e153021_d_n0;
        locals.var_psislsat_dn2 = assign100900_e153021_d_n2;
        locals.var_psislsat_dn4 = assign100900_e153021_d_n4;
        locals.var_psislsat_dn5 = assign100900_e153021_d_n5;
        locals.var_psislsat_dn6 = assign100900_e153021_d_n6;
        locals.var_psislsat_dn7 = assign100900_e153021_d_n7;
        locals.var_psislsat_dn8 = assign100900_e153021_d_n8;
        locals.var_psislsat_dn9 = assign100900_e153021_d_n9;
        locals.var_psislsat_dn10 = assign100900_e153021_d_n10;
        locals.var_psislsat_dn11 = assign100900_e153021_d_n11;
        locals.var_psislsat_dn14 = assign100900_e153021_d_n14;

        let (assign100910_e153031, assign100910_e153031_d_n0, assign100910_e153031_d_n2, assign100910_e153031_d_n4, assign100910_e153031_d_n5, assign100910_e153031_d_n6, assign100910_e153031_d_n7, assign100910_e153031_d_n8, assign100910_e153031_d_n9, assign100910_e153031_d_n10, assign100910_e153031_d_n11, assign100910_e153031_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100910_e153028: f64 = (locals.var_xgate_1 + locals.var_lgate);
        let assign100910_e153029: f64 = (locals.var_lgate / assign100910_e153028);
        (assign100910_e153029, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign100910_e153031;
        locals.var_t2_dn0 = assign100910_e153031_d_n0;
        locals.var_t2_dn2 = assign100910_e153031_d_n2;
        locals.var_t2_dn4 = assign100910_e153031_d_n4;
        locals.var_t2_dn5 = assign100910_e153031_d_n5;
        locals.var_t2_dn6 = assign100910_e153031_d_n6;
        locals.var_t2_dn7 = assign100910_e153031_d_n7;
        locals.var_t2_dn8 = assign100910_e153031_d_n8;
        locals.var_t2_dn9 = assign100910_e153031_d_n9;
        locals.var_t2_dn10 = assign100910_e153031_d_n10;
        locals.var_t2_dn11 = assign100910_e153031_d_n11;
        locals.var_t2_dn14 = assign100910_e153031_d_n14;

        let (assign100920_e153045, assign100920_e153045_d_n0, assign100920_e153045_d_n2, assign100920_e153045_d_n4, assign100920_e153045_d_n5, assign100920_e153045_d_n6, assign100920_e153045_d_n7, assign100920_e153045_d_n8, assign100920_e153045_d_n9, assign100920_e153045_d_n10, assign100920_e153045_d_n11, assign100920_e153045_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100920_e153037: f64 = (locals.var_uc_svdssnp * locals.var_vdsz__blk443);
        let assign100920_e153039: f64 = (assign100920_e153037 + locals.var_ps0z);
        let assign100920_e153042: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign100920_e153043: f64 = (assign100920_e153039 - assign100920_e153042);
        (assign100920_e153043, (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2_dn11 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn11))), (((locals.var_uc_svdssnp * locals.var_vdsz__blk443_dn14) + locals.var_ps0z_dn14) - ((locals.var_t2_dn14 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn14))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100920_e153045;
        locals.var_psisubsat_dn0 = assign100920_e153045_d_n0;
        locals.var_psisubsat_dn2 = assign100920_e153045_d_n2;
        locals.var_psisubsat_dn4 = assign100920_e153045_d_n4;
        locals.var_psisubsat_dn5 = assign100920_e153045_d_n5;
        locals.var_psisubsat_dn6 = assign100920_e153045_d_n6;
        locals.var_psisubsat_dn7 = assign100920_e153045_d_n7;
        locals.var_psisubsat_dn8 = assign100920_e153045_d_n8;
        locals.var_psisubsat_dn9 = assign100920_e153045_d_n9;
        locals.var_psisubsat_dn10 = assign100920_e153045_d_n10;
        locals.var_psisubsat_dn11 = assign100920_e153045_d_n11;
        locals.var_psisubsat_dn14 = assign100920_e153045_d_n14;

        let (assign100930_e153060, assign100930_e153060_d_n0, assign100930_e153060_d_n2, assign100930_e153060_d_n4, assign100930_e153060_d_n5, assign100930_e153060_d_n6, assign100930_e153060_d_n7, assign100930_e153060_d_n8, assign100930_e153060_d_n9, assign100930_e153060_d_n10, assign100930_e153060_d_n11, assign100930_e153060_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100930_e153051: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign100930_e153054: f64 = (4.0 * 0.001);
        let assign100930_e153056: f64 = (assign100930_e153054 * 0.001);
        let assign100930_e153057: f64 = (assign100930_e153051 + assign100930_e153056);
        let assign100930_e153058: f64 = (assign100930_e153057).sqrt();
        (assign100930_e153058, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign100930_e153058)), (((locals.var_psisubsat_dn14 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn14)) / (2.0 * assign100930_e153058)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign100930_e153060;
        locals.var_tmf2_dn0 = assign100930_e153060_d_n0;
        locals.var_tmf2_dn2 = assign100930_e153060_d_n2;
        locals.var_tmf2_dn4 = assign100930_e153060_d_n4;
        locals.var_tmf2_dn5 = assign100930_e153060_d_n5;
        locals.var_tmf2_dn6 = assign100930_e153060_d_n6;
        locals.var_tmf2_dn7 = assign100930_e153060_d_n7;
        locals.var_tmf2_dn8 = assign100930_e153060_d_n8;
        locals.var_tmf2_dn9 = assign100930_e153060_d_n9;
        locals.var_tmf2_dn10 = assign100930_e153060_d_n10;
        locals.var_tmf2_dn11 = assign100930_e153060_d_n11;
        locals.var_tmf2_dn14 = assign100930_e153060_d_n14;

        let (assign100940_e153072, assign100940_e153072_d_n0, assign100940_e153072_d_n2, assign100940_e153072_d_n4, assign100940_e153072_d_n5, assign100940_e153072_d_n6, assign100940_e153072_d_n7, assign100940_e153072_d_n8, assign100940_e153072_d_n9, assign100940_e153072_d_n10, assign100940_e153072_d_n11, assign100940_e153072_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100940_e153068: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign100940_e153069: f64 = (1.0 + assign100940_e153068);
        let assign100940_e153070: f64 = (0.5 * assign100940_e153069);
        (assign100940_e153070, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn14 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100940_e153072;
        locals.var_t9_dn0 = assign100940_e153072_d_n0;
        locals.var_t9_dn2 = assign100940_e153072_d_n2;
        locals.var_t9_dn4 = assign100940_e153072_d_n4;
        locals.var_t9_dn5 = assign100940_e153072_d_n5;
        locals.var_t9_dn6 = assign100940_e153072_d_n6;
        locals.var_t9_dn7 = assign100940_e153072_d_n7;
        locals.var_t9_dn8 = assign100940_e153072_d_n8;
        locals.var_t9_dn9 = assign100940_e153072_d_n9;
        locals.var_t9_dn10 = assign100940_e153072_d_n10;
        locals.var_t9_dn11 = assign100940_e153072_d_n11;
        locals.var_t9_dn14 = assign100940_e153072_d_n14;

        let (assign100950_e153082, assign100950_e153082_d_n0, assign100950_e153082_d_n2, assign100950_e153082_d_n4, assign100950_e153082_d_n5, assign100950_e153082_d_n6, assign100950_e153082_d_n7, assign100950_e153082_d_n8, assign100950_e153082_d_n9, assign100950_e153082_d_n10, assign100950_e153082_d_n11, assign100950_e153082_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100950_e153079: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign100950_e153080: f64 = (0.5 * assign100950_e153079);
        (assign100950_e153080, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100950_e153082;
        locals.var_psisubsat_dn0 = assign100950_e153082_d_n0;
        locals.var_psisubsat_dn2 = assign100950_e153082_d_n2;
        locals.var_psisubsat_dn4 = assign100950_e153082_d_n4;
        locals.var_psisubsat_dn5 = assign100950_e153082_d_n5;
        locals.var_psisubsat_dn6 = assign100950_e153082_d_n6;
        locals.var_psisubsat_dn7 = assign100950_e153082_d_n7;
        locals.var_psisubsat_dn8 = assign100950_e153082_d_n8;
        locals.var_psisubsat_dn9 = assign100950_e153082_d_n9;
        locals.var_psisubsat_dn10 = assign100950_e153082_d_n10;
        locals.var_psisubsat_dn11 = assign100950_e153082_d_n11;
        locals.var_psisubsat_dn14 = assign100950_e153082_d_n14;

        let assign100960_e153085: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2323 = assign100960_e153085;

        let (assign100970_e153093, assign100970_e153093_d_n0, assign100970_e153093_d_n2, assign100970_e153093_d_n4, assign100970_e153093_d_n5, assign100970_e153093_d_n6, assign100970_e153093_d_n7, assign100970_e153093_d_n8, assign100970_e153093_d_n9, assign100970_e153093_d_n10, assign100970_e153093_d_n11, assign100970_e153093_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100970_e153093;
        locals.var_psisubsat_dn0 = assign100970_e153093_d_n0;
        locals.var_psisubsat_dn2 = assign100970_e153093_d_n2;
        locals.var_psisubsat_dn4 = assign100970_e153093_d_n4;
        locals.var_psisubsat_dn5 = assign100970_e153093_d_n5;
        locals.var_psisubsat_dn6 = assign100970_e153093_d_n6;
        locals.var_psisubsat_dn7 = assign100970_e153093_d_n7;
        locals.var_psisubsat_dn8 = assign100970_e153093_d_n8;
        locals.var_psisubsat_dn9 = assign100970_e153093_d_n9;
        locals.var_psisubsat_dn10 = assign100970_e153093_d_n10;
        locals.var_psisubsat_dn11 = assign100970_e153093_d_n11;
        locals.var_psisubsat_dn14 = assign100970_e153093_d_n14;

        let (assign100980_e153101, assign100980_e153101_d_n0, assign100980_e153101_d_n2, assign100980_e153101_d_n4, assign100980_e153101_d_n5, assign100980_e153101_d_n6, assign100980_e153101_d_n7, assign100980_e153101_d_n8, assign100980_e153101_d_n9, assign100980_e153101_d_n10, assign100980_e153101_d_n11, assign100980_e153101_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign100980_e153101;
        locals.var_t9_dn0 = assign100980_e153101_d_n0;
        locals.var_t9_dn2 = assign100980_e153101_d_n2;
        locals.var_t9_dn4 = assign100980_e153101_d_n4;
        locals.var_t9_dn5 = assign100980_e153101_d_n5;
        locals.var_t9_dn6 = assign100980_e153101_d_n6;
        locals.var_t9_dn7 = assign100980_e153101_d_n7;
        locals.var_t9_dn8 = assign100980_e153101_d_n8;
        locals.var_t9_dn9 = assign100980_e153101_d_n9;
        locals.var_t9_dn10 = assign100980_e153101_d_n10;
        locals.var_t9_dn11 = assign100980_e153101_d_n11;
        locals.var_t9_dn14 = assign100980_e153101_d_n14;

        let (assign100990_e153109, assign100990_e153109_d_n0, assign100990_e153109_d_n2, assign100990_e153109_d_n4, assign100990_e153109_d_n5, assign100990_e153109_d_n6, assign100990_e153109_d_n7, assign100990_e153109_d_n8, assign100990_e153109_d_n9, assign100990_e153109_d_n10, assign100990_e153109_d_n11, assign100990_e153109_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign100990_e153107: f64 = (locals.var_psisubsat + 1e-25);
        (assign100990_e153107, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn14,)
    }
};
        locals.var_psisubsat = assign100990_e153109;
        locals.var_psisubsat_dn0 = assign100990_e153109_d_n0;
        locals.var_psisubsat_dn2 = assign100990_e153109_d_n2;
        locals.var_psisubsat_dn4 = assign100990_e153109_d_n4;
        locals.var_psisubsat_dn5 = assign100990_e153109_d_n5;
        locals.var_psisubsat_dn6 = assign100990_e153109_d_n6;
        locals.var_psisubsat_dn7 = assign100990_e153109_d_n7;
        locals.var_psisubsat_dn8 = assign100990_e153109_d_n8;
        locals.var_psisubsat_dn9 = assign100990_e153109_d_n9;
        locals.var_psisubsat_dn10 = assign100990_e153109_d_n10;
        locals.var_psisubsat_dn11 = assign100990_e153109_d_n11;
        locals.var_psisubsat_dn14 = assign100990_e153109_d_n14;

        let (assign101000_e153121, assign101000_e153121_d_n0, assign101000_e153121_d_n2, assign101000_e153121_d_n4, assign101000_e153121_d_n5, assign101000_e153121_d_n6, assign101000_e153121_d_n7, assign101000_e153121_d_n8, assign101000_e153121_d_n9, assign101000_e153121_d_n10, assign101000_e153121_d_n11, assign101000_e153121_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101000_e153117: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign101000_e153118: f64 = (locals.var_uc_subtmp * assign101000_e153117);
        let assign101000_e153119: f64 = (1.0 + assign101000_e153118);
        (assign101000_e153119, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn11), (locals.var_uc_subtmp * locals.var_ttemp_dn14),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign101000_e153121;
        locals.var_xsubtmp_dn0 = assign101000_e153121_d_n0;
        locals.var_xsubtmp_dn2 = assign101000_e153121_d_n2;
        locals.var_xsubtmp_dn4 = assign101000_e153121_d_n4;
        locals.var_xsubtmp_dn5 = assign101000_e153121_d_n5;
        locals.var_xsubtmp_dn6 = assign101000_e153121_d_n6;
        locals.var_xsubtmp_dn7 = assign101000_e153121_d_n7;
        locals.var_xsubtmp_dn8 = assign101000_e153121_d_n8;
        locals.var_xsubtmp_dn9 = assign101000_e153121_d_n9;
        locals.var_xsubtmp_dn10 = assign101000_e153121_d_n10;
        locals.var_xsubtmp_dn11 = assign101000_e153121_d_n11;
        locals.var_xsubtmp_dn14 = assign101000_e153121_d_n14;

        let (assign101010_e153132, assign101010_e153132_d_n0, assign101010_e153132_d_n2, assign101010_e153132_d_n4, assign101010_e153132_d_n5, assign101010_e153132_d_n6, assign101010_e153132_d_n7, assign101010_e153132_d_n8, assign101010_e153132_d_n9, assign101010_e153132_d_n10, assign101010_e153132_d_n11, assign101010_e153132_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let (assign101010_e153130, assign101010_e153130_d_n0, assign101010_e153130_d_n2, assign101010_e153130_d_n4, assign101010_e153130_d_n5, assign101010_e153130_d_n6, assign101010_e153130_d_n7, assign101010_e153130_d_n8, assign101010_e153130_d_n9, assign101010_e153130_d_n10, assign101010_e153130_d_n11, assign101010_e153130_d_n14,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
            }
        };
        (assign101010_e153130, assign101010_e153130_d_n0, assign101010_e153130_d_n2, assign101010_e153130_d_n4, assign101010_e153130_d_n5, assign101010_e153130_d_n6, assign101010_e153130_d_n7, assign101010_e153130_d_n8, assign101010_e153130_d_n9, assign101010_e153130_d_n10, assign101010_e153130_d_n11, assign101010_e153130_d_n14,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn11, locals.var_xsubtmp_dn14,)
    }
};
        locals.var_xsubtmp = assign101010_e153132;
        locals.var_xsubtmp_dn0 = assign101010_e153132_d_n0;
        locals.var_xsubtmp_dn2 = assign101010_e153132_d_n2;
        locals.var_xsubtmp_dn4 = assign101010_e153132_d_n4;
        locals.var_xsubtmp_dn5 = assign101010_e153132_d_n5;
        locals.var_xsubtmp_dn6 = assign101010_e153132_d_n6;
        locals.var_xsubtmp_dn7 = assign101010_e153132_d_n7;
        locals.var_xsubtmp_dn8 = assign101010_e153132_d_n8;
        locals.var_xsubtmp_dn9 = assign101010_e153132_d_n9;
        locals.var_xsubtmp_dn10 = assign101010_e153132_d_n10;
        locals.var_xsubtmp_dn11 = assign101010_e153132_d_n11;
        locals.var_xsubtmp_dn14 = assign101010_e153132_d_n14;

        let (assign101020_e153140, assign101020_e153140_d_n0, assign101020_e153140_d_n2, assign101020_e153140_d_n4, assign101020_e153140_d_n5, assign101020_e153140_d_n6, assign101020_e153140_d_n7, assign101020_e153140_d_n8, assign101020_e153140_d_n9, assign101020_e153140_d_n10, assign101020_e153140_d_n11, assign101020_e153140_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101020_e153138: f64 = (locals.var_xsub1_1 / locals.var_xsubtmp);
        (assign101020_e153138, (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn11) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1_1 * locals.var_xsubtmp_dn14) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101020_e153140;
        locals.var_t5_dn0 = assign101020_e153140_d_n0;
        locals.var_t5_dn2 = assign101020_e153140_d_n2;
        locals.var_t5_dn4 = assign101020_e153140_d_n4;
        locals.var_t5_dn5 = assign101020_e153140_d_n5;
        locals.var_t5_dn6 = assign101020_e153140_d_n6;
        locals.var_t5_dn7 = assign101020_e153140_d_n7;
        locals.var_t5_dn8 = assign101020_e153140_d_n8;
        locals.var_t5_dn9 = assign101020_e153140_d_n9;
        locals.var_t5_dn10 = assign101020_e153140_d_n10;
        locals.var_t5_dn11 = assign101020_e153140_d_n11;
        locals.var_t5_dn14 = assign101020_e153140_d_n14;

        let (assign101030_e153148, assign101030_e153148_d_n0, assign101030_e153148_d_n2, assign101030_e153148_d_n4, assign101030_e153148_d_n5, assign101030_e153148_d_n6, assign101030_e153148_d_n7, assign101030_e153148_d_n8, assign101030_e153148_d_n9, assign101030_e153148_d_n10, assign101030_e153148_d_n11, assign101030_e153148_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101030_e153146: f64 = (locals.var_xsub2_1 * locals.var_xsubtmp);
        (assign101030_e153146, (locals.var_xsub2_1 * locals.var_xsubtmp_dn0), (locals.var_xsub2_1 * locals.var_xsubtmp_dn2), (locals.var_xsub2_1 * locals.var_xsubtmp_dn4), (locals.var_xsub2_1 * locals.var_xsubtmp_dn5), (locals.var_xsub2_1 * locals.var_xsubtmp_dn6), (locals.var_xsub2_1 * locals.var_xsubtmp_dn7), (locals.var_xsub2_1 * locals.var_xsubtmp_dn8), (locals.var_xsub2_1 * locals.var_xsubtmp_dn9), (locals.var_xsub2_1 * locals.var_xsubtmp_dn10), (locals.var_xsub2_1 * locals.var_xsubtmp_dn11), (locals.var_xsub2_1 * locals.var_xsubtmp_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign101030_e153148;
        locals.var_t6_dn0 = assign101030_e153148_d_n0;
        locals.var_t6_dn2 = assign101030_e153148_d_n2;
        locals.var_t6_dn4 = assign101030_e153148_d_n4;
        locals.var_t6_dn5 = assign101030_e153148_d_n5;
        locals.var_t6_dn6 = assign101030_e153148_d_n6;
        locals.var_t6_dn7 = assign101030_e153148_d_n7;
        locals.var_t6_dn8 = assign101030_e153148_d_n8;
        locals.var_t6_dn9 = assign101030_e153148_d_n9;
        locals.var_t6_dn10 = assign101030_e153148_d_n10;
        locals.var_t6_dn11 = assign101030_e153148_d_n11;
        locals.var_t6_dn14 = assign101030_e153148_d_n14;

        let (assign101040_e153158, assign101040_e153158_d_n0, assign101040_e153158_d_n2, assign101040_e153158_d_n4, assign101040_e153158_d_n5, assign101040_e153158_d_n6, assign101040_e153158_d_n7, assign101040_e153158_d_n8, assign101040_e153158_d_n9, assign101040_e153158_d_n10, assign101040_e153158_d_n11, assign101040_e153158_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101040_e153153: f64 = (-locals.var_t6);
        let assign101040_e153155: f64 = (assign101040_e153153 / locals.var_psisubsat);
        let assign101040_e153156: f64 = (assign101040_e153155).exp();
        (assign101040_e153156, (assign101040_e153156 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn11) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn11)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign101040_e153156 * ((((-locals.var_t6_dn14) * locals.var_psisubsat) - (assign101040_e153153 * locals.var_psisubsat_dn14)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101040_e153158;
        locals.var_t2_dn0 = assign101040_e153158_d_n0;
        locals.var_t2_dn2 = assign101040_e153158_d_n2;
        locals.var_t2_dn4 = assign101040_e153158_d_n4;
        locals.var_t2_dn5 = assign101040_e153158_d_n5;
        locals.var_t2_dn6 = assign101040_e153158_d_n6;
        locals.var_t2_dn7 = assign101040_e153158_d_n7;
        locals.var_t2_dn8 = assign101040_e153158_d_n8;
        locals.var_t2_dn9 = assign101040_e153158_d_n9;
        locals.var_t2_dn10 = assign101040_e153158_d_n10;
        locals.var_t2_dn11 = assign101040_e153158_d_n11;
        locals.var_t2_dn14 = assign101040_e153158_d_n14;

        let (assign101050_e153168, assign101050_e153168_d_n0, assign101050_e153168_d_n2, assign101050_e153168_d_n4, assign101050_e153168_d_n5, assign101050_e153168_d_n6, assign101050_e153168_d_n7, assign101050_e153168_d_n8, assign101050_e153168_d_n9, assign101050_e153168_d_n10, assign101050_e153168_d_n11, assign101050_e153168_d_n14,) = {
    if ((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) {
        let assign101050_e153164: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign101050_e153166: f64 = (assign101050_e153164 * locals.var_t2);
        (assign101050_e153166, ((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn0)), ((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn2)), ((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn4)), ((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn5)), ((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn6)), ((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn7)), ((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn8)), ((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn9)), ((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn10)), ((((locals.var_t5_dn11 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn11)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn11)), ((((locals.var_t5_dn14 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn14)) * locals.var_t2) + (assign101050_e153164 * locals.var_t2_dn14)),)
    } else {
        (locals.var_iifac, locals.var_iifac_dn0, locals.var_iifac_dn2, locals.var_iifac_dn4, locals.var_iifac_dn5, locals.var_iifac_dn6, locals.var_iifac_dn7, locals.var_iifac_dn8, locals.var_iifac_dn9, locals.var_iifac_dn10, locals.var_iifac_dn11, locals.var_iifac_dn14,)
    }
};
        locals.var_iifac = assign101050_e153168;
        locals.var_iifac_dn0 = assign101050_e153168_d_n0;
        locals.var_iifac_dn2 = assign101050_e153168_d_n2;
        locals.var_iifac_dn4 = assign101050_e153168_d_n4;
        locals.var_iifac_dn5 = assign101050_e153168_d_n5;
        locals.var_iifac_dn6 = assign101050_e153168_d_n6;
        locals.var_iifac_dn7 = assign101050_e153168_d_n7;
        locals.var_iifac_dn8 = assign101050_e153168_d_n8;
        locals.var_iifac_dn9 = assign101050_e153168_d_n9;
        locals.var_iifac_dn10 = assign101050_e153168_d_n10;
        locals.var_iifac_dn11 = assign101050_e153168_d_n11;
        locals.var_iifac_dn14 = assign101050_e153168_d_n14;

        let assign101060_e153171: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2324 = assign101060_e153171;

        let (assign101070_e153183, assign101070_e153183_d_n0, assign101070_e153183_d_n2, assign101070_e153183_d_n4, assign101070_e153183_d_n5, assign101070_e153183_d_n6, assign101070_e153183_d_n7, assign101070_e153183_d_n8, assign101070_e153183_d_n9, assign101070_e153183_d_n10, assign101070_e153183_d_n11, assign101070_e153183_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2324 != 0.0)) {
        let assign101070_e153179: f64 = (1.0 + locals.var_iifac);
        let assign101070_e153181: f64 = (assign101070_e153179 * locals.var_ibsi);
        (assign101070_e153181, ((locals.var_iifac_dn0 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn0)), ((locals.var_iifac_dn2 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn2)), ((locals.var_iifac_dn4 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn4)), ((locals.var_iifac_dn5 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn5)), ((locals.var_iifac_dn6 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn6)), ((locals.var_iifac_dn7 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn7)), ((locals.var_iifac_dn8 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn8)), ((locals.var_iifac_dn9 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn9)), ((locals.var_iifac_dn10 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn10)), ((locals.var_iifac_dn11 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn11)), ((locals.var_iifac_dn14 * locals.var_ibsi) + (assign101070_e153179 * locals.var_ibsi_dn14)),)
    } else {
        (locals.var_wibjt, locals.var_wibjt_dn0, locals.var_wibjt_dn2, locals.var_wibjt_dn4, locals.var_wibjt_dn5, locals.var_wibjt_dn6, locals.var_wibjt_dn7, locals.var_wibjt_dn8, locals.var_wibjt_dn9, locals.var_wibjt_dn10, locals.var_wibjt_dn11, locals.var_wibjt_dn14,)
    }
};
        locals.var_wibjt = assign101070_e153183;
        locals.var_wibjt_dn0 = assign101070_e153183_d_n0;
        locals.var_wibjt_dn2 = assign101070_e153183_d_n2;
        locals.var_wibjt_dn4 = assign101070_e153183_d_n4;
        locals.var_wibjt_dn5 = assign101070_e153183_d_n5;
        locals.var_wibjt_dn6 = assign101070_e153183_d_n6;
        locals.var_wibjt_dn7 = assign101070_e153183_d_n7;
        locals.var_wibjt_dn8 = assign101070_e153183_d_n8;
        locals.var_wibjt_dn9 = assign101070_e153183_d_n9;
        locals.var_wibjt_dn10 = assign101070_e153183_d_n10;
        locals.var_wibjt_dn11 = assign101070_e153183_d_n11;
        locals.var_wibjt_dn14 = assign101070_e153183_d_n14;

        let (assign101080_e153196, assign101080_e153196_d_n0, assign101080_e153196_d_n2, assign101080_e153196_d_n4, assign101080_e153196_d_n5, assign101080_e153196_d_n6, assign101080_e153196_d_n7, assign101080_e153196_d_n8, assign101080_e153196_d_n9, assign101080_e153196_d_n10, assign101080_e153196_d_n11, assign101080_e153196_d_n14,) = {
    if (((locals.var_guard2315 != 0.0) && (locals.var_guard2316 != 0.0)) && (locals.var_guard2324 == 0.0)) {
        let assign101080_e153192: f64 = (1.0 + locals.var_iifac);
        let assign101080_e153194: f64 = (assign101080_e153192 * locals.var_ibs);
        (assign101080_e153194, ((locals.var_iifac_dn0 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn0)), ((locals.var_iifac_dn2 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn2)), ((locals.var_iifac_dn4 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn4)), ((locals.var_iifac_dn5 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn5)), ((locals.var_iifac_dn6 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn6)), ((locals.var_iifac_dn7 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn7)), ((locals.var_iifac_dn8 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn8)), ((locals.var_iifac_dn9 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn9)), ((locals.var_iifac_dn10 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn10)), ((locals.var_iifac_dn11 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn11)), ((locals.var_iifac_dn14 * locals.var_ibs) + (assign101080_e153192 * locals.var_ibs_dn14)),)
    } else {
        (locals.var_wibjt, locals.var_wibjt_dn0, locals.var_wibjt_dn2, locals.var_wibjt_dn4, locals.var_wibjt_dn5, locals.var_wibjt_dn6, locals.var_wibjt_dn7, locals.var_wibjt_dn8, locals.var_wibjt_dn9, locals.var_wibjt_dn10, locals.var_wibjt_dn11, locals.var_wibjt_dn14,)
    }
};
        locals.var_wibjt = assign101080_e153196;
        locals.var_wibjt_dn0 = assign101080_e153196_d_n0;
        locals.var_wibjt_dn2 = assign101080_e153196_d_n2;
        locals.var_wibjt_dn4 = assign101080_e153196_d_n4;
        locals.var_wibjt_dn5 = assign101080_e153196_d_n5;
        locals.var_wibjt_dn6 = assign101080_e153196_d_n6;
        locals.var_wibjt_dn7 = assign101080_e153196_d_n7;
        locals.var_wibjt_dn8 = assign101080_e153196_d_n8;
        locals.var_wibjt_dn9 = assign101080_e153196_d_n9;
        locals.var_wibjt_dn10 = assign101080_e153196_d_n10;
        locals.var_wibjt_dn11 = assign101080_e153196_d_n11;
        locals.var_wibjt_dn14 = assign101080_e153196_d_n14;

        let assign101090_e153199: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2325 = assign101090_e153199;

    }

    pub(super) fn stamp_transient_block_372(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101100_e153205, assign101100_e153205_d_n0, assign101100_e153205_d_n2, assign101100_e153205_d_n4, assign101100_e153205_d_n5, assign101100_e153205_d_n6, assign101100_e153205_d_n7, assign101100_e153205_d_n8, assign101100_e153205_d_n9, assign101100_e153205_d_n10, assign101100_e153205_d_n11, assign101100_e153205_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101100_e153205;
        locals.var_t12_dn0 = assign101100_e153205_d_n0;
        locals.var_t12_dn2 = assign101100_e153205_d_n2;
        locals.var_t12_dn4 = assign101100_e153205_d_n4;
        locals.var_t12_dn5 = assign101100_e153205_d_n5;
        locals.var_t12_dn6 = assign101100_e153205_d_n6;
        locals.var_t12_dn7 = assign101100_e153205_d_n7;
        locals.var_t12_dn8 = assign101100_e153205_d_n8;
        locals.var_t12_dn9 = assign101100_e153205_d_n9;
        locals.var_t12_dn10 = assign101100_e153205_d_n10;
        locals.var_t12_dn11 = assign101100_e153205_d_n11;
        locals.var_t12_dn14 = assign101100_e153205_d_n14;

        let (assign101110_e153211, assign101110_e153211_d_n0, assign101110_e153211_d_n2, assign101110_e153211_d_n4, assign101110_e153211_d_n5, assign101110_e153211_d_n6, assign101110_e153211_d_n7, assign101110_e153211_d_n8, assign101110_e153211_d_n9, assign101110_e153211_d_n10, assign101110_e153211_d_n11, assign101110_e153211_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (p.p271, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101110_e153211;
        locals.var_t10_dn0 = assign101110_e153211_d_n0;
        locals.var_t10_dn2 = assign101110_e153211_d_n2;
        locals.var_t10_dn4 = assign101110_e153211_d_n4;
        locals.var_t10_dn5 = assign101110_e153211_d_n5;
        locals.var_t10_dn6 = assign101110_e153211_d_n6;
        locals.var_t10_dn7 = assign101110_e153211_d_n7;
        locals.var_t10_dn8 = assign101110_e153211_d_n8;
        locals.var_t10_dn9 = assign101110_e153211_d_n9;
        locals.var_t10_dn10 = assign101110_e153211_d_n10;
        locals.var_t10_dn11 = assign101110_e153211_d_n11;
        locals.var_t10_dn14 = assign101110_e153211_d_n14;

        let (assign101120_e153217, assign101120_e153217_d_n0, assign101120_e153217_d_n2, assign101120_e153217_d_n4, assign101120_e153217_d_n5, assign101120_e153217_d_n6, assign101120_e153217_d_n7, assign101120_e153217_d_n8, assign101120_e153217_d_n9, assign101120_e153217_d_n10, assign101120_e153217_d_n11, assign101120_e153217_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101120_e153217;
        locals.var_t3_dn0 = assign101120_e153217_d_n0;
        locals.var_t3_dn2 = assign101120_e153217_d_n2;
        locals.var_t3_dn4 = assign101120_e153217_d_n4;
        locals.var_t3_dn5 = assign101120_e153217_d_n5;
        locals.var_t3_dn6 = assign101120_e153217_d_n6;
        locals.var_t3_dn7 = assign101120_e153217_d_n7;
        locals.var_t3_dn8 = assign101120_e153217_d_n8;
        locals.var_t3_dn9 = assign101120_e153217_d_n9;
        locals.var_t3_dn10 = assign101120_e153217_d_n10;
        locals.var_t3_dn11 = assign101120_e153217_d_n11;
        locals.var_t3_dn14 = assign101120_e153217_d_n14;

        let (assign101130_e153229, assign101130_e153229_d_n0, assign101130_e153229_d_n2, assign101130_e153229_d_n4, assign101130_e153229_d_n5, assign101130_e153229_d_n6, assign101130_e153229_d_n7, assign101130_e153229_d_n8, assign101130_e153229_d_n9, assign101130_e153229_d_n10, assign101130_e153229_d_n11, assign101130_e153229_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        let assign101130_e153223: f64 = (locals.var_t12 * locals.var_t10);
        let assign101130_e153225: f64 = (assign101130_e153223 * locals.var_t3);
        let assign101130_e153227: f64 = (assign101130_e153225 * locals.var_t3);
        (assign101130_e153227, ((((((locals.var_t12_dn0 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn0)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn0)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn0)), ((((((locals.var_t12_dn2 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn2)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn2)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn2)), ((((((locals.var_t12_dn4 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn4)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn4)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn4)), ((((((locals.var_t12_dn5 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn5)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn5)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn5)), ((((((locals.var_t12_dn6 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn6)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn6)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn6)), ((((((locals.var_t12_dn7 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn7)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn7)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn7)), ((((((locals.var_t12_dn8 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn8)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn8)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn8)), ((((((locals.var_t12_dn9 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn9)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn9)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn9)), ((((((locals.var_t12_dn10 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn10)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn10)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn10)), ((((((locals.var_t12_dn11 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn11)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn11)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn11)), ((((((locals.var_t12_dn14 * locals.var_t10) + (locals.var_t12 * locals.var_t10_dn14)) * locals.var_t3) + (assign101130_e153223 * locals.var_t3_dn14)) * locals.var_t3) + (assign101130_e153225 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101130_e153229;
        locals.var_t1_dn0 = assign101130_e153229_d_n0;
        locals.var_t1_dn2 = assign101130_e153229_d_n2;
        locals.var_t1_dn4 = assign101130_e153229_d_n4;
        locals.var_t1_dn5 = assign101130_e153229_d_n5;
        locals.var_t1_dn6 = assign101130_e153229_d_n6;
        locals.var_t1_dn7 = assign101130_e153229_d_n7;
        locals.var_t1_dn8 = assign101130_e153229_d_n8;
        locals.var_t1_dn9 = assign101130_e153229_d_n9;
        locals.var_t1_dn10 = assign101130_e153229_d_n10;
        locals.var_t1_dn11 = assign101130_e153229_d_n11;
        locals.var_t1_dn14 = assign101130_e153229_d_n14;

        let (assign101140_e153247, assign101140_e153247_d_n0, assign101140_e153247_d_n2, assign101140_e153247_d_n4, assign101140_e153247_d_n5, assign101140_e153247_d_n6, assign101140_e153247_d_n7, assign101140_e153247_d_n8, assign101140_e153247_d_n9, assign101140_e153247_d_n10, assign101140_e153247_d_n11, assign101140_e153247_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        let assign101140_e153235: f64 = (locals.var_mu * locals.var_vgvt);
        let assign101140_e153237: f64 = (assign101140_e153235 * locals.var_t12);
        let assign101140_e153240: f64 = (locals.var_t10 * locals.var_t3);
        let assign101140_e153242: f64 = (assign101140_e153240 * locals.var_t3);
        let assign101140_e153243: f64 = (assign101140_e153237 + assign101140_e153242);
        let assign101140_e153245: f64 = (assign101140_e153243 + 1e-25);
        (assign101140_e153245, (((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn0)) + ((((locals.var_t10_dn0 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn0)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn0))), (((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn2)) + ((((locals.var_t10_dn2 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn2)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn2))), (((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn4)) + ((((locals.var_t10_dn4 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn4)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn4))), (((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn5)) + ((((locals.var_t10_dn5 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn5)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn5))), (((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn6)) + ((((locals.var_t10_dn6 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn6)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn6))), (((((locals.var_mu_dn7 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn7)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn7)) + ((((locals.var_t10_dn7 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn7)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn7))), (((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn8)) + ((((locals.var_t10_dn8 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn8)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn8))), (((((locals.var_mu_dn9 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn9)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn9)) + ((((locals.var_t10_dn9 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn9)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn9))), (((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn10)) + ((((locals.var_t10_dn10 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn10)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn10))), (((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn11)) + ((((locals.var_t10_dn11 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn11)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn11))), (((((locals.var_mu_dn14 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn14)) * locals.var_t12) + (assign101140_e153235 * locals.var_t12_dn14)) + ((((locals.var_t10_dn14 * locals.var_t3) + (locals.var_t10 * locals.var_t3_dn14)) * locals.var_t3) + (assign101140_e153240 * locals.var_t3_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101140_e153247;
        locals.var_t2_dn0 = assign101140_e153247_d_n0;
        locals.var_t2_dn2 = assign101140_e153247_d_n2;
        locals.var_t2_dn4 = assign101140_e153247_d_n4;
        locals.var_t2_dn5 = assign101140_e153247_d_n5;
        locals.var_t2_dn6 = assign101140_e153247_d_n6;
        locals.var_t2_dn7 = assign101140_e153247_d_n7;
        locals.var_t2_dn8 = assign101140_e153247_d_n8;
        locals.var_t2_dn9 = assign101140_e153247_d_n9;
        locals.var_t2_dn10 = assign101140_e153247_d_n10;
        locals.var_t2_dn11 = assign101140_e153247_d_n11;
        locals.var_t2_dn14 = assign101140_e153247_d_n14;

        let (assign101150_e153255, assign101150_e153255_d_n0, assign101150_e153255_d_n2, assign101150_e153255_d_n4, assign101150_e153255_d_n5, assign101150_e153255_d_n6, assign101150_e153255_d_n7, assign101150_e153255_d_n8, assign101150_e153255_d_n9, assign101150_e153255_d_n10, assign101150_e153255_d_n11, assign101150_e153255_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 != 0.0)) {
        let assign101150_e153253: f64 = (locals.var_t1 / locals.var_t2);
        (assign101150_e153253, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn7, locals.var_tau_dn8, locals.var_tau_dn9, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn14,)
    }
};
        locals.var_tau = assign101150_e153255;
        locals.var_tau_dn0 = assign101150_e153255_d_n0;
        locals.var_tau_dn2 = assign101150_e153255_d_n2;
        locals.var_tau_dn4 = assign101150_e153255_d_n4;
        locals.var_tau_dn5 = assign101150_e153255_d_n5;
        locals.var_tau_dn6 = assign101150_e153255_d_n6;
        locals.var_tau_dn7 = assign101150_e153255_d_n7;
        locals.var_tau_dn8 = assign101150_e153255_d_n8;
        locals.var_tau_dn9 = assign101150_e153255_d_n9;
        locals.var_tau_dn10 = assign101150_e153255_d_n10;
        locals.var_tau_dn11 = assign101150_e153255_d_n11;
        locals.var_tau_dn14 = assign101150_e153255_d_n14;

        let (assign101160_e153262, assign101160_e153262_d_n0, assign101160_e153262_d_n2, assign101160_e153262_d_n4, assign101160_e153262_d_n5, assign101160_e153262_d_n6, assign101160_e153262_d_n7, assign101160_e153262_d_n8, assign101160_e153262_d_n9, assign101160_e153262_d_n10, assign101160_e153262_d_n11, assign101160_e153262_d_n14,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard2325 == 0.0)) {
        (p.p270, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn7, locals.var_tau_dn8, locals.var_tau_dn9, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn14,)
    }
};
        locals.var_tau = assign101160_e153262;
        locals.var_tau_dn0 = assign101160_e153262_d_n0;
        locals.var_tau_dn2 = assign101160_e153262_d_n2;
        locals.var_tau_dn4 = assign101160_e153262_d_n4;
        locals.var_tau_dn5 = assign101160_e153262_d_n5;
        locals.var_tau_dn6 = assign101160_e153262_d_n6;
        locals.var_tau_dn7 = assign101160_e153262_d_n7;
        locals.var_tau_dn8 = assign101160_e153262_d_n8;
        locals.var_tau_dn9 = assign101160_e153262_d_n9;
        locals.var_tau_dn10 = assign101160_e153262_d_n10;
        locals.var_tau_dn11 = assign101160_e153262_d_n11;
        locals.var_tau_dn14 = assign101160_e153262_d_n14;

        let (assign101170_e153266, assign101170_e153266_d_n0, assign101170_e153266_d_n2, assign101170_e153266_d_n4, assign101170_e153266_d_n5, assign101170_e153266_d_n6, assign101170_e153266_d_n7, assign101170_e153266_d_n8, assign101170_e153266_d_n9, assign101170_e153266_d_n10, assign101170_e153266_d_n11, assign101170_e153266_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        (locals.var_mks_dly3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101170_e153266;
        locals.var_t2_dn0 = assign101170_e153266_d_n0;
        locals.var_t2_dn2 = assign101170_e153266_d_n2;
        locals.var_t2_dn4 = assign101170_e153266_d_n4;
        locals.var_t2_dn5 = assign101170_e153266_d_n5;
        locals.var_t2_dn6 = assign101170_e153266_d_n6;
        locals.var_t2_dn7 = assign101170_e153266_d_n7;
        locals.var_t2_dn8 = assign101170_e153266_d_n8;
        locals.var_t2_dn9 = assign101170_e153266_d_n9;
        locals.var_t2_dn10 = assign101170_e153266_d_n10;
        locals.var_t2_dn11 = assign101170_e153266_d_n11;
        locals.var_t2_dn14 = assign101170_e153266_d_n14;

        let (assign101180_e153272, assign101180_e153272_d_n0, assign101180_e153272_d_n2, assign101180_e153272_d_n4, assign101180_e153272_d_n5, assign101180_e153272_d_n6, assign101180_e153272_d_n7, assign101180_e153272_d_n8, assign101180_e153272_d_n9, assign101180_e153272_d_n10, assign101180_e153272_d_n11, assign101180_e153272_d_n14,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign101180_e153270: f64 = (locals.var_t2 * locals.var_cox);
        (assign101180_e153270, ((locals.var_t2_dn0 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn0)), ((locals.var_t2_dn2 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn2)), ((locals.var_t2_dn4 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn4)), ((locals.var_t2_dn5 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn5)), ((locals.var_t2_dn6 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn6)), ((locals.var_t2_dn7 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn7)), ((locals.var_t2_dn8 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn8)), ((locals.var_t2_dn9 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn9)), ((locals.var_t2_dn10 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn10)), ((locals.var_t2_dn11 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn11)), ((locals.var_t2_dn14 * locals.var_cox) + (locals.var_t2 * locals.var_cox_dn14)),)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn7, locals.var_taub_dn8, locals.var_taub_dn9, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn14,)
    }
};
        locals.var_taub = assign101180_e153272;
        locals.var_taub_dn0 = assign101180_e153272_d_n0;
        locals.var_taub_dn2 = assign101180_e153272_d_n2;
        locals.var_taub_dn4 = assign101180_e153272_d_n4;
        locals.var_taub_dn5 = assign101180_e153272_d_n5;
        locals.var_taub_dn6 = assign101180_e153272_d_n6;
        locals.var_taub_dn7 = assign101180_e153272_d_n7;
        locals.var_taub_dn8 = assign101180_e153272_d_n8;
        locals.var_taub_dn9 = assign101180_e153272_d_n9;
        locals.var_taub_dn10 = assign101180_e153272_d_n10;
        locals.var_taub_dn11 = assign101180_e153272_d_n11;
        locals.var_taub_dn14 = assign101180_e153272_d_n14;

        let assign101190_e153278: f64 = if ((p.p26 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2326 = assign101190_e153278;

        let (assign101200_e153282,) = {
    if (locals.var_guard2326 != 0.0) {
        (locals.var_uc_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign101200_e153282;

        let (assign101220_e153290,) = {
    if (locals.var_guard2326 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign101220_e153290;

        let (assign101230_e153296, assign101230_e153296_d_n0, assign101230_e153296_d_n2, assign101230_e153296_d_n4, assign101230_e153296_d_n5, assign101230_e153296_d_n6, assign101230_e153296_d_n7, assign101230_e153296_d_n8, assign101230_e153296_d_n9, assign101230_e153296_d_n10, assign101230_e153296_d_n11, assign101230_e153296_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101230_e153294: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign101230_e153294, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn7 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn9 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn14 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign101230_e153296;
        locals.var_t1_dn0 = assign101230_e153296_d_n0;
        locals.var_t1_dn2 = assign101230_e153296_d_n2;
        locals.var_t1_dn4 = assign101230_e153296_d_n4;
        locals.var_t1_dn5 = assign101230_e153296_d_n5;
        locals.var_t1_dn6 = assign101230_e153296_d_n6;
        locals.var_t1_dn7 = assign101230_e153296_d_n7;
        locals.var_t1_dn8 = assign101230_e153296_d_n8;
        locals.var_t1_dn9 = assign101230_e153296_d_n9;
        locals.var_t1_dn10 = assign101230_e153296_d_n10;
        locals.var_t1_dn11 = assign101230_e153296_d_n11;
        locals.var_t1_dn14 = assign101230_e153296_d_n14;

        let (assign101240_e153313, assign101240_e153313_d_n0, assign101240_e153313_d_n2, assign101240_e153313_d_n4, assign101240_e153313_d_n5, assign101240_e153313_d_n6, assign101240_e153313_d_n7, assign101240_e153313_d_n8, assign101240_e153313_d_n9, assign101240_e153313_d_n10, assign101240_e153313_d_n11, assign101240_e153313_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101240_e153300: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101240_e153303: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101240_e153304: f64 = (assign101240_e153300 * assign101240_e153303);
        let assign101240_e153307: f64 = (4.0 * 0.001);
        let assign101240_e153309: f64 = (assign101240_e153307 * 0.001);
        let assign101240_e153310: f64 = (assign101240_e153304 + assign101240_e153309);
        let assign101240_e153311: f64 = (assign101240_e153310).sqrt();
        (assign101240_e153311, ((((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11))) / (2.0 * assign101240_e153311)), ((((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) * assign101240_e153303) + (assign101240_e153300 * (locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14))) / (2.0 * assign101240_e153311)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101240_e153313;
        locals.var_tmf2_dn0 = assign101240_e153313_d_n0;
        locals.var_tmf2_dn2 = assign101240_e153313_d_n2;
        locals.var_tmf2_dn4 = assign101240_e153313_d_n4;
        locals.var_tmf2_dn5 = assign101240_e153313_d_n5;
        locals.var_tmf2_dn6 = assign101240_e153313_d_n6;
        locals.var_tmf2_dn7 = assign101240_e153313_d_n7;
        locals.var_tmf2_dn8 = assign101240_e153313_d_n8;
        locals.var_tmf2_dn9 = assign101240_e153313_d_n9;
        locals.var_tmf2_dn10 = assign101240_e153313_d_n10;
        locals.var_tmf2_dn11 = assign101240_e153313_d_n11;
        locals.var_tmf2_dn14 = assign101240_e153313_d_n14;

        let (assign101250_e153325, assign101250_e153325_d_n0, assign101250_e153325_d_n2, assign101250_e153325_d_n4, assign101250_e153325_d_n5, assign101250_e153325_d_n6, assign101250_e153325_d_n7, assign101250_e153325_d_n8, assign101250_e153325_d_n9, assign101250_e153325_d_n10, assign101250_e153325_d_n11, assign101250_e153325_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101250_e153319: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101250_e153321: f64 = (assign101250_e153319 / locals.var_tmf2);
        let assign101250_e153322: f64 = (1.0 + assign101250_e153321);
        let assign101250_e153323: f64 = (0.5 * assign101250_e153322);
        (assign101250_e153323, (0.5 * ((((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * ((((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) * locals.var_tmf2) - (assign101250_e153319 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101250_e153325;
        locals.var_t0_dn0 = assign101250_e153325_d_n0;
        locals.var_t0_dn2 = assign101250_e153325_d_n2;
        locals.var_t0_dn4 = assign101250_e153325_d_n4;
        locals.var_t0_dn5 = assign101250_e153325_d_n5;
        locals.var_t0_dn6 = assign101250_e153325_d_n6;
        locals.var_t0_dn7 = assign101250_e153325_d_n7;
        locals.var_t0_dn8 = assign101250_e153325_d_n8;
        locals.var_t0_dn9 = assign101250_e153325_d_n9;
        locals.var_t0_dn10 = assign101250_e153325_d_n10;
        locals.var_t0_dn11 = assign101250_e153325_d_n11;
        locals.var_t0_dn14 = assign101250_e153325_d_n14;

        let (assign101260_e153335, assign101260_e153335_d_n0, assign101260_e153335_d_n2, assign101260_e153335_d_n4, assign101260_e153335_d_n5, assign101260_e153335_d_n6, assign101260_e153335_d_n7, assign101260_e153335_d_n8, assign101260_e153335_d_n9, assign101260_e153335_d_n10, assign101260_e153335_d_n11, assign101260_e153335_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101260_e153330: f64 = (locals.var_ps0 - locals.var_vbscl__blk439);
        let assign101260_e153332: f64 = (assign101260_e153330 + locals.var_tmf2);
        let assign101260_e153333: f64 = (0.5 * assign101260_e153332);
        (assign101260_e153333, (0.5 * ((locals.var_ps0_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_tmf2_dn0)), (0.5 * ((locals.var_ps0_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_tmf2_dn2)), (0.5 * ((locals.var_ps0_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_tmf2_dn4)), (0.5 * ((locals.var_ps0_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_tmf2_dn5)), (0.5 * ((locals.var_ps0_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_tmf2_dn6)), (0.5 * ((locals.var_ps0_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_tmf2_dn7)), (0.5 * ((locals.var_ps0_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_tmf2_dn8)), (0.5 * ((locals.var_ps0_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_tmf2_dn9)), (0.5 * ((locals.var_ps0_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_tmf2_dn10)), (0.5 * ((locals.var_ps0_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_tmf2_dn11)), (0.5 * ((locals.var_ps0_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101260_e153335;
        locals.var_t5_dn0 = assign101260_e153335_d_n0;
        locals.var_t5_dn2 = assign101260_e153335_d_n2;
        locals.var_t5_dn4 = assign101260_e153335_d_n4;
        locals.var_t5_dn5 = assign101260_e153335_d_n5;
        locals.var_t5_dn6 = assign101260_e153335_d_n6;
        locals.var_t5_dn7 = assign101260_e153335_d_n7;
        locals.var_t5_dn8 = assign101260_e153335_d_n8;
        locals.var_t5_dn9 = assign101260_e153335_d_n9;
        locals.var_t5_dn10 = assign101260_e153335_d_n10;
        locals.var_t5_dn11 = assign101260_e153335_d_n11;
        locals.var_t5_dn14 = assign101260_e153335_d_n14;

        let assign101270_e153338: f64 = if locals.var_t5 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2327 = assign101270_e153338;

        let (assign101280_e153344, assign101280_e153344_d_n0, assign101280_e153344_d_n2, assign101280_e153344_d_n4, assign101280_e153344_d_n5, assign101280_e153344_d_n6, assign101280_e153344_d_n7, assign101280_e153344_d_n8, assign101280_e153344_d_n9, assign101280_e153344_d_n10, assign101280_e153344_d_n11, assign101280_e153344_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2327 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101280_e153344;
        locals.var_t5_dn0 = assign101280_e153344_d_n0;
        locals.var_t5_dn2 = assign101280_e153344_d_n2;
        locals.var_t5_dn4 = assign101280_e153344_d_n4;
        locals.var_t5_dn5 = assign101280_e153344_d_n5;
        locals.var_t5_dn6 = assign101280_e153344_d_n6;
        locals.var_t5_dn7 = assign101280_e153344_d_n7;
        locals.var_t5_dn8 = assign101280_e153344_d_n8;
        locals.var_t5_dn9 = assign101280_e153344_d_n9;
        locals.var_t5_dn10 = assign101280_e153344_d_n10;
        locals.var_t5_dn11 = assign101280_e153344_d_n11;
        locals.var_t5_dn14 = assign101280_e153344_d_n14;

        let (assign101290_e153350, assign101290_e153350_d_n0, assign101290_e153350_d_n2, assign101290_e153350_d_n4, assign101290_e153350_d_n5, assign101290_e153350_d_n6, assign101290_e153350_d_n7, assign101290_e153350_d_n8, assign101290_e153350_d_n9, assign101290_e153350_d_n10, assign101290_e153350_d_n11, assign101290_e153350_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2327 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101290_e153350;
        locals.var_t0_dn0 = assign101290_e153350_d_n0;
        locals.var_t0_dn2 = assign101290_e153350_d_n2;
        locals.var_t0_dn4 = assign101290_e153350_d_n4;
        locals.var_t0_dn5 = assign101290_e153350_d_n5;
        locals.var_t0_dn6 = assign101290_e153350_d_n6;
        locals.var_t0_dn7 = assign101290_e153350_d_n7;
        locals.var_t0_dn8 = assign101290_e153350_d_n8;
        locals.var_t0_dn9 = assign101290_e153350_d_n9;
        locals.var_t0_dn10 = assign101290_e153350_d_n10;
        locals.var_t0_dn11 = assign101290_e153350_d_n11;
        locals.var_t0_dn14 = assign101290_e153350_d_n14;

        let (assign101300_e153364, assign101300_e153364_d_n0, assign101300_e153364_d_n2, assign101300_e153364_d_n4, assign101300_e153364_d_n5, assign101300_e153364_d_n6, assign101300_e153364_d_n7, assign101300_e153364_d_n8, assign101300_e153364_d_n9, assign101300_e153364_d_n10, assign101300_e153364_d_n11, assign101300_e153364_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101300_e153355: f64 = (locals.var_qn0 / locals.var_t5);
        let assign101300_e153356: f64 = (locals.var_cox + assign101300_e153355);
        let assign101300_e153358: f64 = (assign101300_e153356 + locals.var_cite);
        let assign101300_e153360: f64 = (assign101300_e153358 * locals.var_beta_inv);
        let assign101300_e153362: f64 = (assign101300_e153360 / 1.6021918e-19);
        (assign101300_e153362, ((((locals.var_cox_dn0 + (((locals.var_qn0_dn0 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn0)) / 1.6021918e-19), ((((locals.var_cox_dn2 + (((locals.var_qn0_dn2 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn2)) / 1.6021918e-19), ((((locals.var_cox_dn4 + (((locals.var_qn0_dn4 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn4)) / 1.6021918e-19), ((((locals.var_cox_dn5 + (((locals.var_qn0_dn5 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn5)) / 1.6021918e-19), ((((locals.var_cox_dn6 + (((locals.var_qn0_dn6 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn6)) / 1.6021918e-19), ((((locals.var_cox_dn7 + (((locals.var_qn0_dn7 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn7)) / 1.6021918e-19), ((((locals.var_cox_dn8 + (((locals.var_qn0_dn8 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn8)) / 1.6021918e-19), ((((locals.var_cox_dn9 + (((locals.var_qn0_dn9 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn9)) / 1.6021918e-19), ((((locals.var_cox_dn10 + (((locals.var_qn0_dn10 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn10)) / 1.6021918e-19), ((((locals.var_cox_dn11 + (((locals.var_qn0_dn11 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn11)) / 1.6021918e-19), ((((locals.var_cox_dn14 + (((locals.var_qn0_dn14 * locals.var_t5) - (locals.var_qn0 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))) * locals.var_beta_inv) + (assign101300_e153358 * locals.var_beta_inv_dn14)) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101300_e153364;
        locals.var_t2_dn0 = assign101300_e153364_d_n0;
        locals.var_t2_dn2 = assign101300_e153364_d_n2;
        locals.var_t2_dn4 = assign101300_e153364_d_n4;
        locals.var_t2_dn5 = assign101300_e153364_d_n5;
        locals.var_t2_dn6 = assign101300_e153364_d_n6;
        locals.var_t2_dn7 = assign101300_e153364_d_n7;
        locals.var_t2_dn8 = assign101300_e153364_d_n8;
        locals.var_t2_dn9 = assign101300_e153364_d_n9;
        locals.var_t2_dn10 = assign101300_e153364_d_n10;
        locals.var_t2_dn11 = assign101300_e153364_d_n11;
        locals.var_t2_dn14 = assign101300_e153364_d_n14;

        let (assign101310_e153379, assign101310_e153379_d_n0, assign101310_e153379_d_n2, assign101310_e153379_d_n4, assign101310_e153379_d_n5, assign101310_e153379_d_n6, assign101310_e153379_d_n7, assign101310_e153379_d_n8, assign101310_e153379_d_n9, assign101310_e153379_d_n10, assign101310_e153379_d_n11, assign101310_e153379_d_n14,) = {
    if (locals.var_guard2326 != 0.0) {
        let assign101310_e153367: f64 = (-2.0);
        let assign101310_e153369: f64 = (assign101310_e153367 * locals.var_qi_noi);
        let assign101310_e153371: f64 = (assign101310_e153369 / 1.6021918e-19);
        let assign101310_e153373: f64 = (assign101310_e153371 / locals.var_lch);
        let assign101310_e153375: f64 = (assign101310_e153373 / locals.var_weffcv_nf);
        let assign101310_e153377: f64 = (assign101310_e153375 - locals.var_t1);
        (assign101310_e153377, (((((((assign101310_e153367 * locals.var_qi_noi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn0), (((((((assign101310_e153367 * locals.var_qi_noi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn2), (((((((assign101310_e153367 * locals.var_qi_noi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn4), (((((((assign101310_e153367 * locals.var_qi_noi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn5), (((((((assign101310_e153367 * locals.var_qi_noi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn6), (((((((assign101310_e153367 * locals.var_qi_noi_dn7) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn7), (((((((assign101310_e153367 * locals.var_qi_noi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn8), (((((((assign101310_e153367 * locals.var_qi_noi_dn9) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn9), (((((((assign101310_e153367 * locals.var_qi_noi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn10), (((((((assign101310_e153367 * locals.var_qi_noi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn11), (((((((assign101310_e153367 * locals.var_qi_noi_dn14) / 1.6021918e-19) * locals.var_lch) - (assign101310_e153371 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)) / locals.var_weffcv_nf) - locals.var_t1_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101310_e153379;
        locals.var_t3_dn0 = assign101310_e153379_d_n0;
        locals.var_t3_dn2 = assign101310_e153379_d_n2;
        locals.var_t3_dn4 = assign101310_e153379_d_n4;
        locals.var_t3_dn5 = assign101310_e153379_d_n5;
        locals.var_t3_dn6 = assign101310_e153379_d_n6;
        locals.var_t3_dn7 = assign101310_e153379_d_n7;
        locals.var_t3_dn8 = assign101310_e153379_d_n8;
        locals.var_t3_dn9 = assign101310_e153379_d_n9;
        locals.var_t3_dn10 = assign101310_e153379_d_n10;
        locals.var_t3_dn11 = assign101310_e153379_d_n11;
        locals.var_t3_dn14 = assign101310_e153379_d_n14;

        let assign101320_e153382: f64 = (locals.var_t3 - locals.var_t1);
        let assign101320_e153383: f64 = (assign101320_e153382).abs();
        let assign101320_e153386: f64 = (10.0 * 2.220446049250313e-16);
        let assign101320_e153387: f64 = if assign101320_e153383 > assign101320_e153386 { 1.0 } else { 0.0 };
        locals.var_guard2328 = assign101320_e153387;

        let (assign101330_e153434, assign101330_e153434_d_n0, assign101330_e153434_d_n2, assign101330_e153434_d_n4, assign101330_e153434_d_n5, assign101330_e153434_d_n6, assign101330_e153434_d_n7, assign101330_e153434_d_n8, assign101330_e153434_d_n9, assign101330_e153434_d_n10, assign101330_e153434_d_n11, assign101330_e153434_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2328 != 0.0)) {
        let assign101330_e153394: f64 = (locals.var_t1 + locals.var_t2);
        let assign101330_e153395: f64 = (1.0 / assign101330_e153394);
        let assign101330_e153398: f64 = (locals.var_t3 + locals.var_t2);
        let assign101330_e153399: f64 = (assign101330_e153395 / assign101330_e153398);
        let assign101330_e153402: f64 = (2.0 * locals.var_nfalpe);
        let assign101330_e153404: f64 = (assign101330_e153402 * locals.var_ey);
        let assign101330_e153406: f64 = (assign101330_e153404 * locals.var_mu);
        let assign101330_e153409: f64 = (locals.var_t3 - locals.var_t1);
        let assign101330_e153410: f64 = (assign101330_e153406 / assign101330_e153409);
        let assign101330_e153413: f64 = (locals.var_t3 + locals.var_t2);
        let assign101330_e153416: f64 = (locals.var_t1 + locals.var_t2);
        let assign101330_e153417: f64 = (assign101330_e153413 / assign101330_e153416);
        let assign101330_e153418: f64 = (assign101330_e153417).ln();
        let assign101330_e153419: f64 = (assign101330_e153410 * assign101330_e153418);
        let assign101330_e153420: f64 = (assign101330_e153399 + assign101330_e153419);
        let assign101330_e153423: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101330_e153425: f64 = (assign101330_e153423 * locals.var_mu);
        let assign101330_e153427: f64 = (assign101330_e153425 * locals.var_nfalpe);
        let assign101330_e153429: f64 = (assign101330_e153427 * locals.var_ey);
        let assign101330_e153431: f64 = (assign101330_e153429 * locals.var_mu);
        let assign101330_e153432: f64 = (assign101330_e153420 + assign101330_e153431);
        (assign101330_e153432, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn0) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn0)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn0)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn2) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn2)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn2)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn4) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn4)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn4)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn5) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn5)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn5)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn6) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn6)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn6)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn7) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn7)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn7 - locals.var_t1_dn7))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn7 + locals.var_t2_dn7) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn7)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn8) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn8)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn8)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn9) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn9)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn9 - locals.var_t1_dn9))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn9 + locals.var_t2_dn9) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn9)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn10) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn10)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn10)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn11) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn11)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn11)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101330_e153394 * assign101330_e153394))) * assign101330_e153398) - (assign101330_e153395 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101330_e153398 * assign101330_e153398)) + ((((((((assign101330_e153402 * locals.var_ey_dn14) * locals.var_mu) + (assign101330_e153404 * locals.var_mu_dn14)) * assign101330_e153409) - (assign101330_e153406 * (locals.var_t3_dn14 - locals.var_t1_dn14))) / (assign101330_e153409 * assign101330_e153409)) * assign101330_e153418) + (assign101330_e153410 * (((((locals.var_t3_dn14 + locals.var_t2_dn14) * assign101330_e153416) - (assign101330_e153413 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101330_e153416 * assign101330_e153416)) / assign101330_e153417)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101330_e153423 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101330_e153427 * locals.var_ey_dn14)) * locals.var_mu) + (assign101330_e153429 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101330_e153434;
        locals.var_t4_dn0 = assign101330_e153434_d_n0;
        locals.var_t4_dn2 = assign101330_e153434_d_n2;
        locals.var_t4_dn4 = assign101330_e153434_d_n4;
        locals.var_t4_dn5 = assign101330_e153434_d_n5;
        locals.var_t4_dn6 = assign101330_e153434_d_n6;
        locals.var_t4_dn7 = assign101330_e153434_d_n7;
        locals.var_t4_dn8 = assign101330_e153434_d_n8;
        locals.var_t4_dn9 = assign101330_e153434_d_n9;
        locals.var_t4_dn10 = assign101330_e153434_d_n10;
        locals.var_t4_dn11 = assign101330_e153434_d_n11;
        locals.var_t4_dn14 = assign101330_e153434_d_n14;

        let (assign101340_e153473, assign101340_e153473_d_n0, assign101340_e153473_d_n2, assign101340_e153473_d_n4, assign101340_e153473_d_n5, assign101340_e153473_d_n6, assign101340_e153473_d_n7, assign101340_e153473_d_n8, assign101340_e153473_d_n9, assign101340_e153473_d_n10, assign101340_e153473_d_n11, assign101340_e153473_d_n14,) = {
    if ((locals.var_guard2326 != 0.0) && (locals.var_guard2328 == 0.0)) {
        let assign101340_e153442: f64 = (locals.var_t1 + locals.var_t2);
        let assign101340_e153443: f64 = (1.0 / assign101340_e153442);
        let assign101340_e153446: f64 = (locals.var_t3 + locals.var_t2);
        let assign101340_e153447: f64 = (assign101340_e153443 / assign101340_e153446);
        let assign101340_e153450: f64 = (2.0 * locals.var_nfalpe);
        let assign101340_e153452: f64 = (assign101340_e153450 * locals.var_ey);
        let assign101340_e153454: f64 = (assign101340_e153452 * locals.var_mu);
        let assign101340_e153457: f64 = (locals.var_t1 + locals.var_t2);
        let assign101340_e153458: f64 = (assign101340_e153454 / assign101340_e153457);
        let assign101340_e153459: f64 = (assign101340_e153447 + assign101340_e153458);
        let assign101340_e153462: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign101340_e153464: f64 = (assign101340_e153462 * locals.var_mu);
        let assign101340_e153466: f64 = (assign101340_e153464 * locals.var_nfalpe);
        let assign101340_e153468: f64 = (assign101340_e153466 * locals.var_ey);
        let assign101340_e153470: f64 = (assign101340_e153468 * locals.var_mu);
        let assign101340_e153471: f64 = (assign101340_e153459 + assign101340_e153470);
        (assign101340_e153471, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn0) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn0)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn0)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn2) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn2)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn2)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn4) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn4)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn4)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn5) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn5)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn5)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn6) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn6)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn6)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn7 + locals.var_t2_dn7) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn7 + locals.var_t2_dn7))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn7) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn7)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn7 + locals.var_t2_dn7))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn7) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn7)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn7)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn7))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn8) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn8)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn8)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn9 + locals.var_t2_dn9) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn9 + locals.var_t2_dn9))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn9) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn9)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn9 + locals.var_t2_dn9))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn9) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn9)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn9)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn9))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn10) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn10)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn10)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn11) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn11)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn11)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn14 + locals.var_t2_dn14) / (assign101340_e153442 * assign101340_e153442))) * assign101340_e153446) - (assign101340_e153443 * (locals.var_t3_dn14 + locals.var_t2_dn14))) / (assign101340_e153446 * assign101340_e153446)) + ((((((assign101340_e153450 * locals.var_ey_dn14) * locals.var_mu) + (assign101340_e153452 * locals.var_mu_dn14)) * assign101340_e153457) - (assign101340_e153454 * (locals.var_t1_dn14 + locals.var_t2_dn14))) / (assign101340_e153457 * assign101340_e153457))) + ((((((((locals.var_nfalpe * locals.var_ey_dn14) * locals.var_mu) + (assign101340_e153462 * locals.var_mu_dn14)) * locals.var_nfalpe) * locals.var_ey) + (assign101340_e153466 * locals.var_ey_dn14)) * locals.var_mu) + (assign101340_e153468 * locals.var_mu_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101340_e153473;
        locals.var_t4_dn0 = assign101340_e153473_d_n0;
        locals.var_t4_dn2 = assign101340_e153473_d_n2;
        locals.var_t4_dn4 = assign101340_e153473_d_n4;
        locals.var_t4_dn5 = assign101340_e153473_d_n5;
        locals.var_t4_dn6 = assign101340_e153473_d_n6;
        locals.var_t4_dn7 = assign101340_e153473_d_n7;
        locals.var_t4_dn8 = assign101340_e153473_d_n8;
        locals.var_t4_dn9 = assign101340_e153473_d_n9;
        locals.var_t4_dn10 = assign101340_e153473_d_n10;
        locals.var_t4_dn11 = assign101340_e153473_d_n11;
        locals.var_t4_dn14 = assign101340_e153473_d_n14;

        let assign101370_e153504: f64 = if (((p.p30 != 0.0) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2329 = assign101370_e153504;

        let (assign101380_e153516, assign101380_e153516_d_n0, assign101380_e153516_d_n2, assign101380_e153516_d_n4, assign101380_e153516_d_n5, assign101380_e153516_d_n6, assign101380_e153516_d_n7, assign101380_e153516_d_n8, assign101380_e153516_d_n9, assign101380_e153516_d_n10, assign101380_e153516_d_n11, assign101380_e153516_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101380_e153508: f64 = (locals.var_psdl - locals.var_ps0);
        let assign101380_e153511: f64 = (10.0 * 2.220446049250313e-16);
        let assign101380_e153512: f64 = (assign101380_e153508 + assign101380_e153511);
        let assign101380_e153514: f64 = (assign101380_e153512 / locals.var_lch);
        (assign101380_e153514, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn7 - locals.var_ps0_dn7) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn9 - locals.var_ps0_dn9) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn14 - locals.var_ps0_dn14) * locals.var_lch) - (assign101380_e153512 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101380_e153516;
        locals.var_eyd_dn0 = assign101380_e153516_d_n0;
        locals.var_eyd_dn2 = assign101380_e153516_d_n2;
        locals.var_eyd_dn4 = assign101380_e153516_d_n4;
        locals.var_eyd_dn5 = assign101380_e153516_d_n5;
        locals.var_eyd_dn6 = assign101380_e153516_d_n6;
        locals.var_eyd_dn7 = assign101380_e153516_d_n7;
        locals.var_eyd_dn8 = assign101380_e153516_d_n8;
        locals.var_eyd_dn9 = assign101380_e153516_d_n9;
        locals.var_eyd_dn10 = assign101380_e153516_d_n10;
        locals.var_eyd_dn11 = assign101380_e153516_d_n11;
        locals.var_eyd_dn14 = assign101380_e153516_d_n14;

    }

    pub(super) fn stamp_transient_block_373(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101390_e153525, assign101390_e153525_d_n0, assign101390_e153525_d_n2, assign101390_e153525_d_n4, assign101390_e153525_d_n5, assign101390_e153525_d_n6, assign101390_e153525_d_n7, assign101390_e153525_d_n8, assign101390_e153525_d_n9, assign101390_e153525_d_n10, assign101390_e153525_d_n11, assign101390_e153525_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let (assign101390_e153523, assign101390_e153523_d_n0, assign101390_e153523_d_n2, assign101390_e153523_d_n4, assign101390_e153523_d_n5, assign101390_e153523_d_n6, assign101390_e153523_d_n7, assign101390_e153523_d_n8, assign101390_e153523_d_n9, assign101390_e153523_d_n10, assign101390_e153523_d_n11, assign101390_e153523_d_n14,) = {
            if (locals.var_eyd >= 0.0) {
                (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign101390_e153523, assign101390_e153523_d_n0, assign101390_e153523_d_n2, assign101390_e153523_d_n4, assign101390_e153523_d_n5, assign101390_e153523_d_n6, assign101390_e153523_d_n7, assign101390_e153523_d_n8, assign101390_e153523_d_n9, assign101390_e153523_d_n10, assign101390_e153523_d_n11, assign101390_e153523_d_n14,)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn7, locals.var_eyd_dn8, locals.var_eyd_dn9, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn14,)
    }
};
        locals.var_eyd = assign101390_e153525;
        locals.var_eyd_dn0 = assign101390_e153525_d_n0;
        locals.var_eyd_dn2 = assign101390_e153525_d_n2;
        locals.var_eyd_dn4 = assign101390_e153525_d_n4;
        locals.var_eyd_dn5 = assign101390_e153525_d_n5;
        locals.var_eyd_dn6 = assign101390_e153525_d_n6;
        locals.var_eyd_dn7 = assign101390_e153525_d_n7;
        locals.var_eyd_dn8 = assign101390_e153525_d_n8;
        locals.var_eyd_dn9 = assign101390_e153525_d_n9;
        locals.var_eyd_dn10 = assign101390_e153525_d_n10;
        locals.var_eyd_dn11 = assign101390_e153525_d_n11;
        locals.var_eyd_dn14 = assign101390_e153525_d_n14;

        let (assign101400_e153533, assign101400_e153533_d_n0, assign101400_e153533_d_n2, assign101400_e153533_d_n4, assign101400_e153533_d_n5, assign101400_e153533_d_n6, assign101400_e153533_d_n7, assign101400_e153533_d_n8, assign101400_e153533_d_n9, assign101400_e153533_d_n10, assign101400_e153533_d_n11, assign101400_e153533_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101400_e153529: f64 = (locals.var_muun * locals.var_eyd);
        let assign101400_e153531: f64 = (assign101400_e153529 / 10000000.0);
        (assign101400_e153531, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / 10000000.0), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / 10000000.0), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / 10000000.0), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / 10000000.0), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / 10000000.0), (((locals.var_muun_dn7 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn7)) / 10000000.0), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / 10000000.0), (((locals.var_muun_dn9 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn9)) / 10000000.0), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / 10000000.0), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / 10000000.0), (((locals.var_muun_dn14 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn14)) / 10000000.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    }
};
        locals.var_t12 = assign101400_e153533;
        locals.var_t12_dn0 = assign101400_e153533_d_n0;
        locals.var_t12_dn2 = assign101400_e153533_d_n2;
        locals.var_t12_dn4 = assign101400_e153533_d_n4;
        locals.var_t12_dn5 = assign101400_e153533_d_n5;
        locals.var_t12_dn6 = assign101400_e153533_d_n6;
        locals.var_t12_dn7 = assign101400_e153533_d_n7;
        locals.var_t12_dn8 = assign101400_e153533_d_n8;
        locals.var_t12_dn9 = assign101400_e153533_d_n9;
        locals.var_t12_dn10 = assign101400_e153533_d_n10;
        locals.var_t12_dn11 = assign101400_e153533_d_n11;
        locals.var_t12_dn14 = assign101400_e153533_d_n14;

        let assign101410_e153537: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153538: f64 = (1.0 - assign101410_e153537);
        let assign101410_e153545: f64 = (10.0 * 2.220446049250313e-16);
        let assign101410_e153546: f64 = (1.0 + assign101410_e153545);
        let assign101410_e153548: f64 = if ((assign101410_e153538 <= p.p178) && (p.p178 <= assign101410_e153546)) { 1.0 } else { 0.0 };
        locals.var_guard2330 = assign101410_e153548;

        let (assign101420_e153554, assign101420_e153554_d_n0, assign101420_e153554_d_n2, assign101420_e153554_d_n4, assign101420_e153554_d_n5, assign101420_e153554_d_n6, assign101420_e153554_d_n7, assign101420_e153554_d_n8, assign101420_e153554_d_n9, assign101420_e153554_d_n10, assign101420_e153554_d_n11, assign101420_e153554_d_n14,) = {
    if ((locals.var_guard2329 != 0.0) && (locals.var_guard2330 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101420_e153554;
        locals.var_t7_dn0 = assign101420_e153554_d_n0;
        locals.var_t7_dn2 = assign101420_e153554_d_n2;
        locals.var_t7_dn4 = assign101420_e153554_d_n4;
        locals.var_t7_dn5 = assign101420_e153554_d_n5;
        locals.var_t7_dn6 = assign101420_e153554_d_n6;
        locals.var_t7_dn7 = assign101420_e153554_d_n7;
        locals.var_t7_dn8 = assign101420_e153554_d_n8;
        locals.var_t7_dn9 = assign101420_e153554_d_n9;
        locals.var_t7_dn10 = assign101420_e153554_d_n10;
        locals.var_t7_dn11 = assign101420_e153554_d_n11;
        locals.var_t7_dn14 = assign101420_e153554_d_n14;

        let assign101430_e153558: f64 = (10.0 * 2.220446049250313e-16);
        let assign101430_e153559: f64 = (2.0 - assign101430_e153558);
        let assign101430_e153566: f64 = (10.0 * 2.220446049250313e-16);
        let assign101430_e153567: f64 = (2.0 + assign101430_e153566);
        let assign101430_e153569: f64 = if ((assign101430_e153559 <= p.p178) && (p.p178 <= assign101430_e153567)) { 1.0 } else { 0.0 };
        locals.var_guard2331 = assign101430_e153569;

        let (assign101440_e153578, assign101440_e153578_d_n0, assign101440_e153578_d_n2, assign101440_e153578_d_n4, assign101440_e153578_d_n5, assign101440_e153578_d_n6, assign101440_e153578_d_n7, assign101440_e153578_d_n8, assign101440_e153578_d_n9, assign101440_e153578_d_n10, assign101440_e153578_d_n11, assign101440_e153578_d_n14,) = {
    if (((locals.var_guard2329 != 0.0) && (locals.var_guard2330 == 0.0)) && (locals.var_guard2331 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101440_e153578;
        locals.var_t7_dn0 = assign101440_e153578_d_n0;
        locals.var_t7_dn2 = assign101440_e153578_d_n2;
        locals.var_t7_dn4 = assign101440_e153578_d_n4;
        locals.var_t7_dn5 = assign101440_e153578_d_n5;
        locals.var_t7_dn6 = assign101440_e153578_d_n6;
        locals.var_t7_dn7 = assign101440_e153578_d_n7;
        locals.var_t7_dn8 = assign101440_e153578_d_n8;
        locals.var_t7_dn9 = assign101440_e153578_d_n9;
        locals.var_t7_dn10 = assign101440_e153578_d_n10;
        locals.var_t7_dn11 = assign101440_e153578_d_n11;
        locals.var_t7_dn14 = assign101440_e153578_d_n14;

        let (assign101450_e153597, assign101450_e153597_d_n0, assign101450_e153597_d_n2, assign101450_e153597_d_n4, assign101450_e153597_d_n5, assign101450_e153597_d_n6, assign101450_e153597_d_n7, assign101450_e153597_d_n8, assign101450_e153597_d_n9, assign101450_e153597_d_n10, assign101450_e153597_d_n11, assign101450_e153597_d_n14,) = {
    if (((locals.var_guard2329 != 0.0) && (locals.var_guard2330 == 0.0)) && (locals.var_guard2331 == 0.0)) {
        let (assign101450_e153595, assign101450_e153595_d_n0, assign101450_e153595_d_n2, assign101450_e153595_d_n4, assign101450_e153595_d_n5, assign101450_e153595_d_n6, assign101450_e153595_d_n7, assign101450_e153595_d_n8, assign101450_e153595_d_n9, assign101450_e153595_d_n10, assign101450_e153595_d_n11, assign101450_e153595_d_n14,) = {
            if (locals.var_eyd == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101450_e153593: f64 = (p.p178 - 1.0);
                let assign101450_e153594: f64 = (locals.var_eyd).powf(assign101450_e153593);
                (assign101450_e153594, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn0)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn0 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn2)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn2 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn4)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn4 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn5)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn5 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn6)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn6 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn7)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn7 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn8)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn8 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn9)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn9 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn10)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn10 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn11)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn11 / locals.var_eyd))) }, if 0.0 == 0.0 && ((assign101450_e153593) as f64).is_finite() && ((assign101450_e153593) as f64).fract() == 0.0 { if assign101450_e153593 == 0.0 { 0.0 } else { (assign101450_e153593 * ((locals.var_eyd).powf(assign101450_e153593 - 1.0) * locals.var_eyd_dn14)) } } else { (assign101450_e153594 * (assign101450_e153593 * (locals.var_eyd_dn14 / locals.var_eyd))) },)
            }
        };
        (assign101450_e153595, assign101450_e153595_d_n0, assign101450_e153595_d_n2, assign101450_e153595_d_n4, assign101450_e153595_d_n5, assign101450_e153595_d_n6, assign101450_e153595_d_n7, assign101450_e153595_d_n8, assign101450_e153595_d_n9, assign101450_e153595_d_n10, assign101450_e153595_d_n11, assign101450_e153595_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101450_e153597;
        locals.var_t7_dn0 = assign101450_e153597_d_n0;
        locals.var_t7_dn2 = assign101450_e153597_d_n2;
        locals.var_t7_dn4 = assign101450_e153597_d_n4;
        locals.var_t7_dn5 = assign101450_e153597_d_n5;
        locals.var_t7_dn6 = assign101450_e153597_d_n6;
        locals.var_t7_dn7 = assign101450_e153597_d_n7;
        locals.var_t7_dn8 = assign101450_e153597_d_n8;
        locals.var_t7_dn9 = assign101450_e153597_d_n9;
        locals.var_t7_dn10 = assign101450_e153597_d_n10;
        locals.var_t7_dn11 = assign101450_e153597_d_n11;
        locals.var_t7_dn14 = assign101450_e153597_d_n14;

        let (assign101460_e153603, assign101460_e153603_d_n0, assign101460_e153603_d_n2, assign101460_e153603_d_n4, assign101460_e153603_d_n5, assign101460_e153603_d_n6, assign101460_e153603_d_n7, assign101460_e153603_d_n8, assign101460_e153603_d_n9, assign101460_e153603_d_n10, assign101460_e153603_d_n11, assign101460_e153603_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101460_e153601: f64 = (locals.var_t12 * locals.var_t7);
        (assign101460_e153601, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn7 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn7)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn9 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn9)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn14 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign101460_e153603;
        locals.var_t8_dn0 = assign101460_e153603_d_n0;
        locals.var_t8_dn2 = assign101460_e153603_d_n2;
        locals.var_t8_dn4 = assign101460_e153603_d_n4;
        locals.var_t8_dn5 = assign101460_e153603_d_n5;
        locals.var_t8_dn6 = assign101460_e153603_d_n6;
        locals.var_t8_dn7 = assign101460_e153603_d_n7;
        locals.var_t8_dn8 = assign101460_e153603_d_n8;
        locals.var_t8_dn9 = assign101460_e153603_d_n9;
        locals.var_t8_dn10 = assign101460_e153603_d_n10;
        locals.var_t8_dn11 = assign101460_e153603_d_n11;
        locals.var_t8_dn14 = assign101460_e153603_d_n14;

        let (assign101470_e153609, assign101470_e153609_d_n0, assign101470_e153609_d_n2, assign101470_e153609_d_n4, assign101470_e153609_d_n5, assign101470_e153609_d_n6, assign101470_e153609_d_n7, assign101470_e153609_d_n8, assign101470_e153609_d_n9, assign101470_e153609_d_n10, assign101470_e153609_d_n11, assign101470_e153609_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101470_e153607: f64 = (1.0 + locals.var_t8);
        (assign101470_e153607, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign101470_e153609;
        locals.var_t9_dn0 = assign101470_e153609_d_n0;
        locals.var_t9_dn2 = assign101470_e153609_d_n2;
        locals.var_t9_dn4 = assign101470_e153609_d_n4;
        locals.var_t9_dn5 = assign101470_e153609_d_n5;
        locals.var_t9_dn6 = assign101470_e153609_d_n6;
        locals.var_t9_dn7 = assign101470_e153609_d_n7;
        locals.var_t9_dn8 = assign101470_e153609_d_n8;
        locals.var_t9_dn9 = assign101470_e153609_d_n9;
        locals.var_t9_dn10 = assign101470_e153609_d_n10;
        locals.var_t9_dn11 = assign101470_e153609_d_n11;
        locals.var_t9_dn14 = assign101470_e153609_d_n14;

        let (assign101480_e153625, assign101480_e153625_d_n0, assign101480_e153625_d_n2, assign101480_e153625_d_n4, assign101480_e153625_d_n5, assign101480_e153625_d_n6, assign101480_e153625_d_n7, assign101480_e153625_d_n8, assign101480_e153625_d_n9, assign101480_e153625_d_n10, assign101480_e153625_d_n11, assign101480_e153625_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let (assign101480_e153623, assign101480_e153623_d_n0, assign101480_e153623_d_n2, assign101480_e153623_d_n4, assign101480_e153623_d_n5, assign101480_e153623_d_n6, assign101480_e153623_d_n7, assign101480_e153623_d_n8, assign101480_e153623_d_n9, assign101480_e153623_d_n10, assign101480_e153623_d_n11, assign101480_e153623_d_n14,) = {
            if (locals.var_t9 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign101480_e153617: f64 = (-1.0);
                let assign101480_e153619: f64 = (assign101480_e153617 / p.p178);
                let assign101480_e153621: f64 = (assign101480_e153619 - 1.0);
                let assign101480_e153622: f64 = (locals.var_t9).powf(assign101480_e153621);
                (assign101480_e153622, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn0)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn2)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn4)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn5)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn6)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn7)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn7 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn8)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn9)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn9 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn10)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn11)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign101480_e153621) as f64).is_finite() && ((assign101480_e153621) as f64).fract() == 0.0 { if assign101480_e153621 == 0.0 { 0.0 } else { (assign101480_e153621 * ((locals.var_t9).powf(assign101480_e153621 - 1.0) * locals.var_t9_dn14)) } } else { (assign101480_e153622 * (assign101480_e153621 * (locals.var_t9_dn14 / locals.var_t9))) },)
            }
        };
        (assign101480_e153623, assign101480_e153623_d_n0, assign101480_e153623_d_n2, assign101480_e153623_d_n4, assign101480_e153623_d_n5, assign101480_e153623_d_n6, assign101480_e153623_d_n7, assign101480_e153623_d_n8, assign101480_e153623_d_n9, assign101480_e153623_d_n10, assign101480_e153623_d_n11, assign101480_e153623_d_n14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101480_e153625;
        locals.var_t10_dn0 = assign101480_e153625_d_n0;
        locals.var_t10_dn2 = assign101480_e153625_d_n2;
        locals.var_t10_dn4 = assign101480_e153625_d_n4;
        locals.var_t10_dn5 = assign101480_e153625_d_n5;
        locals.var_t10_dn6 = assign101480_e153625_d_n6;
        locals.var_t10_dn7 = assign101480_e153625_d_n7;
        locals.var_t10_dn8 = assign101480_e153625_d_n8;
        locals.var_t10_dn9 = assign101480_e153625_d_n9;
        locals.var_t10_dn10 = assign101480_e153625_d_n10;
        locals.var_t10_dn11 = assign101480_e153625_d_n11;
        locals.var_t10_dn14 = assign101480_e153625_d_n14;

        let (assign101490_e153631, assign101490_e153631_d_n0, assign101490_e153631_d_n2, assign101490_e153631_d_n4, assign101490_e153631_d_n5, assign101490_e153631_d_n6, assign101490_e153631_d_n7, assign101490_e153631_d_n8, assign101490_e153631_d_n9, assign101490_e153631_d_n10, assign101490_e153631_d_n11, assign101490_e153631_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101490_e153629: f64 = (locals.var_t9 * locals.var_t10);
        (assign101490_e153629, ((locals.var_t9_dn0 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn0)), ((locals.var_t9_dn2 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn2)), ((locals.var_t9_dn4 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn4)), ((locals.var_t9_dn5 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn5)), ((locals.var_t9_dn6 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn6)), ((locals.var_t9_dn7 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn7)), ((locals.var_t9_dn8 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn8)), ((locals.var_t9_dn9 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn9)), ((locals.var_t9_dn10 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn10)), ((locals.var_t9_dn11 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn11)), ((locals.var_t9_dn14 * locals.var_t10) + (locals.var_t9 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign101490_e153631;
        locals.var_t11_dn0 = assign101490_e153631_d_n0;
        locals.var_t11_dn2 = assign101490_e153631_d_n2;
        locals.var_t11_dn4 = assign101490_e153631_d_n4;
        locals.var_t11_dn5 = assign101490_e153631_d_n5;
        locals.var_t11_dn6 = assign101490_e153631_d_n6;
        locals.var_t11_dn7 = assign101490_e153631_d_n7;
        locals.var_t11_dn8 = assign101490_e153631_d_n8;
        locals.var_t11_dn9 = assign101490_e153631_d_n9;
        locals.var_t11_dn10 = assign101490_e153631_d_n10;
        locals.var_t11_dn11 = assign101490_e153631_d_n11;
        locals.var_t11_dn14 = assign101490_e153631_d_n14;

        let (assign101500_e153637, assign101500_e153637_d_n0, assign101500_e153637_d_n2, assign101500_e153637_d_n4, assign101500_e153637_d_n5, assign101500_e153637_d_n6, assign101500_e153637_d_n7, assign101500_e153637_d_n8, assign101500_e153637_d_n9, assign101500_e153637_d_n10, assign101500_e153637_d_n11, assign101500_e153637_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101500_e153635: f64 = (locals.var_muun * locals.var_t11);
        (assign101500_e153635, ((locals.var_muun_dn0 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn0)), ((locals.var_muun_dn2 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn2)), ((locals.var_muun_dn4 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn4)), ((locals.var_muun_dn5 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn5)), ((locals.var_muun_dn6 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn6)), ((locals.var_muun_dn7 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn7)), ((locals.var_muun_dn8 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn8)), ((locals.var_muun_dn9 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn9)), ((locals.var_muun_dn10 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn10)), ((locals.var_muun_dn11 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn11)), ((locals.var_muun_dn14 * locals.var_t11) + (locals.var_muun * locals.var_t11_dn14)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn7, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn9, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn14,)
    }
};
        locals.var_mud_hoso = assign101500_e153637;
        locals.var_mud_hoso_dn0 = assign101500_e153637_d_n0;
        locals.var_mud_hoso_dn2 = assign101500_e153637_d_n2;
        locals.var_mud_hoso_dn4 = assign101500_e153637_d_n4;
        locals.var_mud_hoso_dn5 = assign101500_e153637_d_n5;
        locals.var_mud_hoso_dn6 = assign101500_e153637_d_n6;
        locals.var_mud_hoso_dn7 = assign101500_e153637_d_n7;
        locals.var_mud_hoso_dn8 = assign101500_e153637_d_n8;
        locals.var_mud_hoso_dn9 = assign101500_e153637_d_n9;
        locals.var_mud_hoso_dn10 = assign101500_e153637_d_n10;
        locals.var_mud_hoso_dn11 = assign101500_e153637_d_n11;
        locals.var_mud_hoso_dn14 = assign101500_e153637_d_n14;

        let (assign101510_e153645, assign101510_e153645_d_n0, assign101510_e153645_d_n2, assign101510_e153645_d_n4, assign101510_e153645_d_n5, assign101510_e153645_d_n6, assign101510_e153645_d_n7, assign101510_e153645_d_n8, assign101510_e153645_d_n9, assign101510_e153645_d_n10, assign101510_e153645_d_n11, assign101510_e153645_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101510_e153641: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign101510_e153643: f64 = (assign101510_e153641 / 2.0);
        (assign101510_e153643, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn7 + locals.var_mud_hoso_dn7) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn9 + locals.var_mud_hoso_dn9) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn14 + locals.var_mud_hoso_dn14) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn7, locals.var_mu_ave_dn8, locals.var_mu_ave_dn9, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn14,)
    }
};
        locals.var_mu_ave = assign101510_e153645;
        locals.var_mu_ave_dn0 = assign101510_e153645_d_n0;
        locals.var_mu_ave_dn2 = assign101510_e153645_d_n2;
        locals.var_mu_ave_dn4 = assign101510_e153645_d_n4;
        locals.var_mu_ave_dn5 = assign101510_e153645_d_n5;
        locals.var_mu_ave_dn6 = assign101510_e153645_d_n6;
        locals.var_mu_ave_dn7 = assign101510_e153645_d_n7;
        locals.var_mu_ave_dn8 = assign101510_e153645_d_n8;
        locals.var_mu_ave_dn9 = assign101510_e153645_d_n9;
        locals.var_mu_ave_dn10 = assign101510_e153645_d_n10;
        locals.var_mu_ave_dn11 = assign101510_e153645_d_n11;
        locals.var_mu_ave_dn14 = assign101510_e153645_d_n14;

        let (assign101520_e153651, assign101520_e153651_d_n0, assign101520_e153651_d_n2, assign101520_e153651_d_n4, assign101520_e153651_d_n5, assign101520_e153651_d_n6, assign101520_e153651_d_n7, assign101520_e153651_d_n8, assign101520_e153651_d_n9, assign101520_e153651_d_n10, assign101520_e153651_d_n11, assign101520_e153651_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101520_e153649: f64 = (locals.var_alpha * locals.var_alpha);
        (assign101520_e153649, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101520_e153651;
        locals.var_t0_dn0 = assign101520_e153651_d_n0;
        locals.var_t0_dn2 = assign101520_e153651_d_n2;
        locals.var_t0_dn4 = assign101520_e153651_d_n4;
        locals.var_t0_dn5 = assign101520_e153651_d_n5;
        locals.var_t0_dn6 = assign101520_e153651_d_n6;
        locals.var_t0_dn7 = assign101520_e153651_d_n7;
        locals.var_t0_dn8 = assign101520_e153651_d_n8;
        locals.var_t0_dn9 = assign101520_e153651_d_n9;
        locals.var_t0_dn10 = assign101520_e153651_d_n10;
        locals.var_t0_dn11 = assign101520_e153651_d_n11;
        locals.var_t0_dn14 = assign101520_e153651_d_n14;

        let (assign101530_e153713, assign101530_e153713_d_n0, assign101530_e153713_d_n2, assign101530_e153713_d_n4, assign101530_e153713_d_n5, assign101530_e153713_d_n6, assign101530_e153713_d_n7, assign101530_e153713_d_n8, assign101530_e153713_d_n9, assign101530_e153713_d_n10, assign101530_e153713_d_n11, assign101530_e153713_d_n14,) = {
    if (locals.var_guard2329 != 0.0) {
        let assign101530_e153655: f64 = (locals.var_weff_nf * locals.var_cox);
        let assign101530_e153657: f64 = (assign101530_e153655 * locals.var_vgvt);
        let assign101530_e153659: f64 = (assign101530_e153657 * locals.var_mu);
        let assign101530_e153663: f64 = (3.0 * locals.var_alpha);
        let assign101530_e153664: f64 = (1.0 + assign101530_e153663);
        let assign101530_e153667: f64 = (6.0 * locals.var_t0);
        let assign101530_e153668: f64 = (assign101530_e153664 + assign101530_e153667);
        let assign101530_e153670: f64 = (assign101530_e153668 * locals.var_mud_hoso);
        let assign101530_e153672: f64 = (assign101530_e153670 * locals.var_mud_hoso);
        let assign101530_e153676: f64 = (4.0 * locals.var_alpha);
        let assign101530_e153677: f64 = (3.0 + assign101530_e153676);
        let assign101530_e153680: f64 = (3.0 * locals.var_t0);
        let assign101530_e153681: f64 = (assign101530_e153677 + assign101530_e153680);
        let assign101530_e153683: f64 = (assign101530_e153681 * locals.var_mud_hoso);
        let assign101530_e153685: f64 = (assign101530_e153683 * locals.var_mu);
        let assign101530_e153686: f64 = (assign101530_e153672 + assign101530_e153685);
        let assign101530_e153690: f64 = (3.0 * locals.var_alpha);
        let assign101530_e153691: f64 = (6.0 + assign101530_e153690);
        let assign101530_e153693: f64 = (assign101530_e153691 + locals.var_t0);
        let assign101530_e153695: f64 = (assign101530_e153693 * locals.var_mu);
        let assign101530_e153697: f64 = (assign101530_e153695 * locals.var_mu);
        let assign101530_e153698: f64 = (assign101530_e153686 + assign101530_e153697);
        let assign101530_e153699: f64 = (assign101530_e153659 * assign101530_e153698);
        let assign101530_e153702: f64 = (15.0 * locals.var_lch);
        let assign101530_e153705: f64 = (1.0 + locals.var_alpha);
        let assign101530_e153706: f64 = (assign101530_e153702 * assign101530_e153705);
        let assign101530_e153708: f64 = (assign101530_e153706 * locals.var_mu_ave);
        let assign101530_e153710: f64 = (assign101530_e153708 * locals.var_mu_ave);
        let assign101530_e153711: f64 = (assign101530_e153699 / assign101530_e153710);
        (assign101530_e153711, ((((((((((locals.var_weff_nf * locals.var_cox_dn0) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn0)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn0)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn0))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn0) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn0)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn2) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn2)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn2)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn2))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn2) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn2)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn4) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn4)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn4)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn4))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn4) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn4)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn5) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn5)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn5)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn5))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn5) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn5)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn6) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn6)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn6)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn6))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn6) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn6)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn7) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn7)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn7)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn7) + (6.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn7)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn7)) + ((((((4.0 * locals.var_alpha_dn7) + (3.0 * locals.var_t0_dn7)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn7)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn7))) + ((((((3.0 * locals.var_alpha_dn7) + locals.var_t0_dn7) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn7)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn7))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn7) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn7)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn7)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn7)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn8) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn8)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn8)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn8))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn8) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn8)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn9) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn9)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn9)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn9) + (6.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn9)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn9)) + ((((((4.0 * locals.var_alpha_dn9) + (3.0 * locals.var_t0_dn9)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn9)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn9))) + ((((((3.0 * locals.var_alpha_dn9) + locals.var_t0_dn9) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn9)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn9))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn9) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn9)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn9)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn9)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn10) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn10)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn10)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn10))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn10) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn10)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn11) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn11)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn11)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn11))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn11) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn11)))) / (assign101530_e153710 * assign101530_e153710)), ((((((((((locals.var_weff_nf * locals.var_cox_dn14) * locals.var_vgvt) + (assign101530_e153655 * locals.var_vgvt_dn14)) * locals.var_mu) + (assign101530_e153657 * locals.var_mu_dn14)) * assign101530_e153698) + (assign101530_e153659 * ((((((((3.0 * locals.var_alpha_dn14) + (6.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101530_e153668 * locals.var_mud_hoso_dn14)) * locals.var_mud_hoso) + (assign101530_e153670 * locals.var_mud_hoso_dn14)) + ((((((4.0 * locals.var_alpha_dn14) + (3.0 * locals.var_t0_dn14)) * locals.var_mud_hoso) + (assign101530_e153681 * locals.var_mud_hoso_dn14)) * locals.var_mu) + (assign101530_e153683 * locals.var_mu_dn14))) + ((((((3.0 * locals.var_alpha_dn14) + locals.var_t0_dn14) * locals.var_mu) + (assign101530_e153693 * locals.var_mu_dn14)) * locals.var_mu) + (assign101530_e153695 * locals.var_mu_dn14))))) * assign101530_e153710) - (assign101530_e153699 * (((((((15.0 * locals.var_lch_dn14) * assign101530_e153705) + (assign101530_e153702 * locals.var_alpha_dn14)) * locals.var_mu_ave) + (assign101530_e153706 * locals.var_mu_ave_dn14)) * locals.var_mu_ave) + (assign101530_e153708 * locals.var_mu_ave_dn14)))) / (assign101530_e153710 * assign101530_e153710)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101530_e153713;
        locals.var_nthrml_dn0 = assign101530_e153713_d_n0;
        locals.var_nthrml_dn2 = assign101530_e153713_d_n2;
        locals.var_nthrml_dn4 = assign101530_e153713_d_n4;
        locals.var_nthrml_dn5 = assign101530_e153713_d_n5;
        locals.var_nthrml_dn6 = assign101530_e153713_d_n6;
        locals.var_nthrml_dn7 = assign101530_e153713_d_n7;
        locals.var_nthrml_dn8 = assign101530_e153713_d_n8;
        locals.var_nthrml_dn9 = assign101530_e153713_d_n9;
        locals.var_nthrml_dn10 = assign101530_e153713_d_n10;
        locals.var_nthrml_dn11 = assign101530_e153713_d_n11;
        locals.var_nthrml_dn14 = assign101530_e153713_d_n14;

        let (assign101540_e153718, assign101540_e153718_d_n0, assign101540_e153718_d_n2, assign101540_e153718_d_n4, assign101540_e153718_d_n5, assign101540_e153718_d_n6, assign101540_e153718_d_n7, assign101540_e153718_d_n8, assign101540_e153718_d_n9, assign101540_e153718_d_n10, assign101540_e153718_d_n11, assign101540_e153718_d_n14,) = {
    if (locals.var_guard2329 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn7, locals.var_nthrml_dn8, locals.var_nthrml_dn9, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn14,)
    }
};
        locals.var_nthrml = assign101540_e153718;
        locals.var_nthrml_dn0 = assign101540_e153718_d_n0;
        locals.var_nthrml_dn2 = assign101540_e153718_d_n2;
        locals.var_nthrml_dn4 = assign101540_e153718_d_n4;
        locals.var_nthrml_dn5 = assign101540_e153718_d_n5;
        locals.var_nthrml_dn6 = assign101540_e153718_d_n6;
        locals.var_nthrml_dn7 = assign101540_e153718_d_n7;
        locals.var_nthrml_dn8 = assign101540_e153718_d_n8;
        locals.var_nthrml_dn9 = assign101540_e153718_d_n9;
        locals.var_nthrml_dn10 = assign101540_e153718_d_n10;
        locals.var_nthrml_dn11 = assign101540_e153718_d_n11;
        locals.var_nthrml_dn14 = assign101540_e153718_d_n14;

        let assign101550_e153736: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2332 = assign101550_e153736;

        let (assign101560_e153741, assign101560_e153741_d_n0, assign101560_e153741_d_n2, assign101560_e153741_d_n4, assign101560_e153741_d_n5, assign101560_e153741_d_n6, assign101560_e153741_d_n7, assign101560_e153741_d_n8, assign101560_e153741_d_n9, assign101560_e153741_d_n10, assign101560_e153741_d_n11, assign101560_e153741_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101560_e153739: f64 = (locals.var_kusail).sqrt();
        (assign101560_e153739, (locals.var_kusail_dn0 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn2 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn4 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn5 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn6 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn7 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn8 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn9 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn10 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn11 / (2.0 * assign101560_e153739)), (locals.var_kusail_dn14 / (2.0 * assign101560_e153739)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn7, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn9, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn14,)
    }
};
        locals.var_sqrtkusail = assign101560_e153741;
        locals.var_sqrtkusail_dn0 = assign101560_e153741_d_n0;
        locals.var_sqrtkusail_dn2 = assign101560_e153741_d_n2;
        locals.var_sqrtkusail_dn4 = assign101560_e153741_d_n4;
        locals.var_sqrtkusail_dn5 = assign101560_e153741_d_n5;
        locals.var_sqrtkusail_dn6 = assign101560_e153741_d_n6;
        locals.var_sqrtkusail_dn7 = assign101560_e153741_d_n7;
        locals.var_sqrtkusail_dn8 = assign101560_e153741_d_n8;
        locals.var_sqrtkusail_dn9 = assign101560_e153741_d_n9;
        locals.var_sqrtkusail_dn10 = assign101560_e153741_d_n10;
        locals.var_sqrtkusail_dn11 = assign101560_e153741_d_n11;
        locals.var_sqrtkusail_dn14 = assign101560_e153741_d_n14;

        let (assign101570_e153747, assign101570_e153747_d_n0, assign101570_e153747_d_n2, assign101570_e153747_d_n4, assign101570_e153747_d_n5, assign101570_e153747_d_n6, assign101570_e153747_d_n7, assign101570_e153747_d_n8, assign101570_e153747_d_n9, assign101570_e153747_d_n10, assign101570_e153747_d_n11, assign101570_e153747_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101570_e153745: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign101570_e153745, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign101570_e153747;
        locals.var_t2_dn0 = assign101570_e153747_d_n0;
        locals.var_t2_dn2 = assign101570_e153747_d_n2;
        locals.var_t2_dn4 = assign101570_e153747_d_n4;
        locals.var_t2_dn5 = assign101570_e153747_d_n5;
        locals.var_t2_dn6 = assign101570_e153747_d_n6;
        locals.var_t2_dn7 = assign101570_e153747_d_n7;
        locals.var_t2_dn8 = assign101570_e153747_d_n8;
        locals.var_t2_dn9 = assign101570_e153747_d_n9;
        locals.var_t2_dn10 = assign101570_e153747_d_n10;
        locals.var_t2_dn11 = assign101570_e153747_d_n11;
        locals.var_t2_dn14 = assign101570_e153747_d_n14;

        let (assign101580_e153753, assign101580_e153753_d_n0, assign101580_e153753_d_n2, assign101580_e153753_d_n4, assign101580_e153753_d_n5, assign101580_e153753_d_n6, assign101580_e153753_d_n7, assign101580_e153753_d_n8, assign101580_e153753_d_n9, assign101580_e153753_d_n10, assign101580_e153753_d_n11, assign101580_e153753_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101580_e153751: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign101580_e153751, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign101580_e153753;
        locals.var_t3_dn0 = assign101580_e153753_d_n0;
        locals.var_t3_dn2 = assign101580_e153753_d_n2;
        locals.var_t3_dn4 = assign101580_e153753_d_n4;
        locals.var_t3_dn5 = assign101580_e153753_d_n5;
        locals.var_t3_dn6 = assign101580_e153753_d_n6;
        locals.var_t3_dn7 = assign101580_e153753_d_n7;
        locals.var_t3_dn8 = assign101580_e153753_d_n8;
        locals.var_t3_dn9 = assign101580_e153753_d_n9;
        locals.var_t3_dn10 = assign101580_e153753_d_n10;
        locals.var_t3_dn11 = assign101580_e153753_d_n11;
        locals.var_t3_dn14 = assign101580_e153753_d_n14;

        let (assign101590_e153759, assign101590_e153759_d_n0, assign101590_e153759_d_n2, assign101590_e153759_d_n4, assign101590_e153759_d_n5, assign101590_e153759_d_n6, assign101590_e153759_d_n7, assign101590_e153759_d_n8, assign101590_e153759_d_n9, assign101590_e153759_d_n10, assign101590_e153759_d_n11, assign101590_e153759_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101590_e153757: f64 = (locals.var_kusail * locals.var_kusail);
        (assign101590_e153757, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign101590_e153759;
        locals.var_t4_dn0 = assign101590_e153759_d_n0;
        locals.var_t4_dn2 = assign101590_e153759_d_n2;
        locals.var_t4_dn4 = assign101590_e153759_d_n4;
        locals.var_t4_dn5 = assign101590_e153759_d_n5;
        locals.var_t4_dn6 = assign101590_e153759_d_n6;
        locals.var_t4_dn7 = assign101590_e153759_d_n7;
        locals.var_t4_dn8 = assign101590_e153759_d_n8;
        locals.var_t4_dn9 = assign101590_e153759_d_n9;
        locals.var_t4_dn10 = assign101590_e153759_d_n10;
        locals.var_t4_dn11 = assign101590_e153759_d_n11;
        locals.var_t4_dn14 = assign101590_e153759_d_n14;

        let (assign101600_e153767, assign101600_e153767_d_n0, assign101600_e153767_d_n2, assign101600_e153767_d_n4, assign101600_e153767_d_n5, assign101600_e153767_d_n6, assign101600_e153767_d_n7, assign101600_e153767_d_n8, assign101600_e153767_d_n9, assign101600_e153767_d_n10, assign101600_e153767_d_n11, assign101600_e153767_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101600_e153763: f64 = (42.0 * locals.var_kusai00);
        let assign101600_e153765: f64 = (assign101600_e153763 * locals.var_kusail);
        (assign101600_e153765, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn7) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn7)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn9) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn9)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn14) * locals.var_kusail) + (assign101600_e153763 * locals.var_kusail_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101600_e153767;
        locals.var_t5_dn0 = assign101600_e153767_d_n0;
        locals.var_t5_dn2 = assign101600_e153767_d_n2;
        locals.var_t5_dn4 = assign101600_e153767_d_n4;
        locals.var_t5_dn5 = assign101600_e153767_d_n5;
        locals.var_t5_dn6 = assign101600_e153767_d_n6;
        locals.var_t5_dn7 = assign101600_e153767_d_n7;
        locals.var_t5_dn8 = assign101600_e153767_d_n8;
        locals.var_t5_dn9 = assign101600_e153767_d_n9;
        locals.var_t5_dn10 = assign101600_e153767_d_n10;
        locals.var_t5_dn11 = assign101600_e153767_d_n11;
        locals.var_t5_dn14 = assign101600_e153767_d_n14;

        let (assign101610_e153777, assign101610_e153777_d_n0, assign101610_e153777_d_n2, assign101610_e153777_d_n4, assign101610_e153777_d_n5, assign101610_e153777_d_n6, assign101610_e153777_d_n7, assign101610_e153777_d_n8, assign101610_e153777_d_n9, assign101610_e153777_d_n10, assign101610_e153777_d_n11, assign101610_e153777_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101610_e153773: f64 = (locals.var_t3 + locals.var_t4);
        let assign101610_e153774: f64 = (4.0 * assign101610_e153773);
        let assign101610_e153775: f64 = (locals.var_t5 + assign101610_e153774);
        (assign101610_e153775, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn7 + (4.0 * (locals.var_t3_dn7 + locals.var_t4_dn7))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn9 + (4.0 * (locals.var_t3_dn9 + locals.var_t4_dn9))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn14 + (4.0 * (locals.var_t3_dn14 + locals.var_t4_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101610_e153777;
        locals.var_t5_dn0 = assign101610_e153777_d_n0;
        locals.var_t5_dn2 = assign101610_e153777_d_n2;
        locals.var_t5_dn4 = assign101610_e153777_d_n4;
        locals.var_t5_dn5 = assign101610_e153777_d_n5;
        locals.var_t5_dn6 = assign101610_e153777_d_n6;
        locals.var_t5_dn7 = assign101610_e153777_d_n7;
        locals.var_t5_dn8 = assign101610_e153777_d_n8;
        locals.var_t5_dn9 = assign101610_e153777_d_n9;
        locals.var_t5_dn10 = assign101610_e153777_d_n10;
        locals.var_t5_dn11 = assign101610_e153777_d_n11;
        locals.var_t5_dn14 = assign101610_e153777_d_n14;

    }

    pub(super) fn stamp_transient_block_374(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign101620_e153791, assign101620_e153791_d_n0, assign101620_e153791_d_n2, assign101620_e153791_d_n4, assign101620_e153791_d_n5, assign101620_e153791_d_n6, assign101620_e153791_d_n7, assign101620_e153791_d_n8, assign101620_e153791_d_n9, assign101620_e153791_d_n10, assign101620_e153791_d_n11, assign101620_e153791_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101620_e153782: f64 = (20.0 * locals.var_sqrtkusail);
        let assign101620_e153784: f64 = (assign101620_e153782 * locals.var_vgvt);
        let assign101620_e153787: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign101620_e153788: f64 = (assign101620_e153784 * assign101620_e153787);
        let assign101620_e153789: f64 = (locals.var_t5 + assign101620_e153788);
        (assign101620_e153789, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn0)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn2)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn4)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn5)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn6)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn7 + (((((20.0 * locals.var_sqrtkusail_dn7) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn7)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn7 + locals.var_kusail_dn7)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn8)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn9 + (((((20.0 * locals.var_sqrtkusail_dn9) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn9)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn9 + locals.var_kusail_dn9)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn10)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn11)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn14 + (((((20.0 * locals.var_sqrtkusail_dn14) * locals.var_vgvt) + (assign101620_e153782 * locals.var_vgvt_dn14)) * assign101620_e153787) + (assign101620_e153784 * (locals.var_kusai00_dn14 + locals.var_kusail_dn14)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign101620_e153791;
        locals.var_t5_dn0 = assign101620_e153791_d_n0;
        locals.var_t5_dn2 = assign101620_e153791_d_n2;
        locals.var_t5_dn4 = assign101620_e153791_d_n4;
        locals.var_t5_dn5 = assign101620_e153791_d_n5;
        locals.var_t5_dn6 = assign101620_e153791_d_n6;
        locals.var_t5_dn7 = assign101620_e153791_d_n7;
        locals.var_t5_dn8 = assign101620_e153791_d_n8;
        locals.var_t5_dn9 = assign101620_e153791_d_n9;
        locals.var_t5_dn10 = assign101620_e153791_d_n10;
        locals.var_t5_dn11 = assign101620_e153791_d_n11;
        locals.var_t5_dn14 = assign101620_e153791_d_n14;

        let (assign101630_e153797, assign101630_e153797_d_n0, assign101630_e153797_d_n2, assign101630_e153797_d_n4, assign101630_e153797_d_n5, assign101630_e153797_d_n6, assign101630_e153797_d_n7, assign101630_e153797_d_n8, assign101630_e153797_d_n9, assign101630_e153797_d_n10, assign101630_e153797_d_n11, assign101630_e153797_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101630_e153795: f64 = (locals.var_t2 * locals.var_t2);
        (assign101630_e153795, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101630_e153797;
        locals.var_t10_dn0 = assign101630_e153797_d_n0;
        locals.var_t10_dn2 = assign101630_e153797_d_n2;
        locals.var_t10_dn4 = assign101630_e153797_d_n4;
        locals.var_t10_dn5 = assign101630_e153797_d_n5;
        locals.var_t10_dn6 = assign101630_e153797_d_n6;
        locals.var_t10_dn7 = assign101630_e153797_d_n7;
        locals.var_t10_dn8 = assign101630_e153797_d_n8;
        locals.var_t10_dn9 = assign101630_e153797_d_n9;
        locals.var_t10_dn10 = assign101630_e153797_d_n10;
        locals.var_t10_dn11 = assign101630_e153797_d_n11;
        locals.var_t10_dn14 = assign101630_e153797_d_n14;

        let (assign101640_e153803, assign101640_e153803_d_n0, assign101640_e153803_d_n2, assign101640_e153803_d_n4, assign101640_e153803_d_n5, assign101640_e153803_d_n6, assign101640_e153803_d_n7, assign101640_e153803_d_n8, assign101640_e153803_d_n9, assign101640_e153803_d_n10, assign101640_e153803_d_n11, assign101640_e153803_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101640_e153801: f64 = (locals.var_t10 * locals.var_t10);
        (assign101640_e153801, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)), ((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign101640_e153803;
        locals.var_t10_dn0 = assign101640_e153803_d_n0;
        locals.var_t10_dn2 = assign101640_e153803_d_n2;
        locals.var_t10_dn4 = assign101640_e153803_d_n4;
        locals.var_t10_dn5 = assign101640_e153803_d_n5;
        locals.var_t10_dn6 = assign101640_e153803_d_n6;
        locals.var_t10_dn7 = assign101640_e153803_d_n7;
        locals.var_t10_dn8 = assign101640_e153803_d_n8;
        locals.var_t10_dn9 = assign101640_e153803_d_n9;
        locals.var_t10_dn10 = assign101640_e153803_d_n10;
        locals.var_t10_dn11 = assign101640_e153803_d_n11;
        locals.var_t10_dn14 = assign101640_e153803_d_n14;

        let (assign101650_e153811, assign101650_e153811_d_n0, assign101650_e153811_d_n2, assign101650_e153811_d_n4, assign101650_e153811_d_n5, assign101650_e153811_d_n6, assign101650_e153811_d_n7, assign101650_e153811_d_n8, assign101650_e153811_d_n9, assign101650_e153811_d_n10, assign101650_e153811_d_n11, assign101650_e153811_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101650_e153808: f64 = (locals.var_t10 * locals.var_t2);
        let assign101650_e153809: f64 = (locals.var_t5 / assign101650_e153808);
        (assign101650_e153809, (((locals.var_t5_dn0 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn0 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn0)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn2 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn2 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn2)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn4 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn4 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn4)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn5 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn5 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn5)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn6 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn6 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn6)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn7 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn7 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn7)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn8 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn8 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn8)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn9 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn9 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn9)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn10 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn10 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn10)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn11 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn11 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn11)))) / (assign101650_e153808 * assign101650_e153808)), (((locals.var_t5_dn14 * assign101650_e153808) - (locals.var_t5 * ((locals.var_t10_dn14 * locals.var_t2) + (locals.var_t10 * locals.var_t2_dn14)))) / (assign101650_e153808 * assign101650_e153808)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn7, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn9, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn14,)
    }
};
        locals.var_kusai_ig = assign101650_e153811;
        locals.var_kusai_ig_dn0 = assign101650_e153811_d_n0;
        locals.var_kusai_ig_dn2 = assign101650_e153811_d_n2;
        locals.var_kusai_ig_dn4 = assign101650_e153811_d_n4;
        locals.var_kusai_ig_dn5 = assign101650_e153811_d_n5;
        locals.var_kusai_ig_dn6 = assign101650_e153811_d_n6;
        locals.var_kusai_ig_dn7 = assign101650_e153811_d_n7;
        locals.var_kusai_ig_dn8 = assign101650_e153811_d_n8;
        locals.var_kusai_ig_dn9 = assign101650_e153811_d_n9;
        locals.var_kusai_ig_dn10 = assign101650_e153811_d_n10;
        locals.var_kusai_ig_dn11 = assign101650_e153811_d_n11;
        locals.var_kusai_ig_dn14 = assign101650_e153811_d_n14;

        let (assign101660_e153821, assign101660_e153821_d_n0, assign101660_e153821_d_n2, assign101660_e153821_d_n4, assign101660_e153821_d_n5, assign101660_e153821_d_n6, assign101660_e153821_d_n7, assign101660_e153821_d_n8, assign101660_e153821_d_n9, assign101660_e153821_d_n10, assign101660_e153821_d_n11, assign101660_e153821_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101660_e153815: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign101660_e153817: f64 = (assign101660_e153815 * locals.var_mu);
        let assign101660_e153819: f64 = (assign101660_e153817 * locals.var_cox);
        (assign101660_e153819, (((((-((locals.var_weff_nf * locals.var_lch_dn0) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn0)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn0)), (((((-((locals.var_weff_nf * locals.var_lch_dn2) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn2)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn2)), (((((-((locals.var_weff_nf * locals.var_lch_dn4) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn4)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn4)), (((((-((locals.var_weff_nf * locals.var_lch_dn5) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn5)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn5)), (((((-((locals.var_weff_nf * locals.var_lch_dn6) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn6)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn6)), (((((-((locals.var_weff_nf * locals.var_lch_dn7) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn7)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn7)), (((((-((locals.var_weff_nf * locals.var_lch_dn8) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn8)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn8)), (((((-((locals.var_weff_nf * locals.var_lch_dn9) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn9)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn9)), (((((-((locals.var_weff_nf * locals.var_lch_dn10) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn10)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn10)), (((((-((locals.var_weff_nf * locals.var_lch_dn11) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn11)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn11)), (((((-((locals.var_weff_nf * locals.var_lch_dn14) / (locals.var_lch * locals.var_lch))) * locals.var_mu) + (assign101660_e153815 * locals.var_mu_dn14)) * locals.var_cox) + (assign101660_e153817 * locals.var_cox_dn14)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn7, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn9, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn14,)
    }
};
        locals.var_gds0_ign = assign101660_e153821;
        locals.var_gds0_ign_dn0 = assign101660_e153821_d_n0;
        locals.var_gds0_ign_dn2 = assign101660_e153821_d_n2;
        locals.var_gds0_ign_dn4 = assign101660_e153821_d_n4;
        locals.var_gds0_ign_dn5 = assign101660_e153821_d_n5;
        locals.var_gds0_ign_dn6 = assign101660_e153821_d_n6;
        locals.var_gds0_ign_dn7 = assign101660_e153821_d_n7;
        locals.var_gds0_ign_dn8 = assign101660_e153821_d_n8;
        locals.var_gds0_ign_dn9 = assign101660_e153821_d_n9;
        locals.var_gds0_ign_dn10 = assign101660_e153821_d_n10;
        locals.var_gds0_ign_dn11 = assign101660_e153821_d_n11;
        locals.var_gds0_ign_dn14 = assign101660_e153821_d_n14;

        let (assign101670_e153827, assign101670_e153827_d_n0, assign101670_e153827_d_n2, assign101670_e153827_d_n4, assign101670_e153827_d_n5, assign101670_e153827_d_n6, assign101670_e153827_d_n7, assign101670_e153827_d_n8, assign101670_e153827_d_n9, assign101670_e153827_d_n10, assign101670_e153827_d_n11, assign101670_e153827_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101670_e153825: f64 = (locals.var_gds0_ign * locals.var_vgvt);
        (assign101670_e153825, ((locals.var_gds0_ign_dn0 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn0)), ((locals.var_gds0_ign_dn2 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn2)), ((locals.var_gds0_ign_dn4 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn4)), ((locals.var_gds0_ign_dn5 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn5)), ((locals.var_gds0_ign_dn6 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn6)), ((locals.var_gds0_ign_dn7 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn7)), ((locals.var_gds0_ign_dn8 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn8)), ((locals.var_gds0_ign_dn9 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn9)), ((locals.var_gds0_ign_dn10 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn10)), ((locals.var_gds0_ign_dn11 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn11)), ((locals.var_gds0_ign_dn14 * locals.var_vgvt) + (locals.var_gds0_ign * locals.var_vgvt_dn14)),)
    } else {
        (locals.var_gds0_h2, locals.var_gds0_h2_dn0, locals.var_gds0_h2_dn2, locals.var_gds0_h2_dn4, locals.var_gds0_h2_dn5, locals.var_gds0_h2_dn6, locals.var_gds0_h2_dn7, locals.var_gds0_h2_dn8, locals.var_gds0_h2_dn9, locals.var_gds0_h2_dn10, locals.var_gds0_h2_dn11, locals.var_gds0_h2_dn14,)
    }
};
        locals.var_gds0_h2 = assign101670_e153827;
        locals.var_gds0_h2_dn0 = assign101670_e153827_d_n0;
        locals.var_gds0_h2_dn2 = assign101670_e153827_d_n2;
        locals.var_gds0_h2_dn4 = assign101670_e153827_d_n4;
        locals.var_gds0_h2_dn5 = assign101670_e153827_d_n5;
        locals.var_gds0_h2_dn6 = assign101670_e153827_d_n6;
        locals.var_gds0_h2_dn7 = assign101670_e153827_d_n7;
        locals.var_gds0_h2_dn8 = assign101670_e153827_d_n8;
        locals.var_gds0_h2_dn9 = assign101670_e153827_d_n9;
        locals.var_gds0_h2_dn10 = assign101670_e153827_d_n10;
        locals.var_gds0_h2_dn11 = assign101670_e153827_d_n11;
        locals.var_gds0_h2_dn14 = assign101670_e153827_d_n14;

        let (assign101680_e153833, assign101680_e153833_d_n0, assign101680_e153833_d_n2, assign101680_e153833_d_n4, assign101680_e153833_d_n5, assign101680_e153833_d_n6, assign101680_e153833_d_n7, assign101680_e153833_d_n8, assign101680_e153833_d_n9, assign101680_e153833_d_n10, assign101680_e153833_d_n11, assign101680_e153833_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101680_e153831: f64 = (locals.var_nthrml / locals.var_gds0_h2);
        (assign101680_e153831, (((locals.var_nthrml_dn0 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn0)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn2 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn2)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn4 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn4)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn5 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn5)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn6 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn6)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn7 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn7)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn8 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn8)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn9 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn9)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn10 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn10)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn11 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn11)) / (locals.var_gds0_h2 * locals.var_gds0_h2)), (((locals.var_nthrml_dn14 * locals.var_gds0_h2) - (locals.var_nthrml * locals.var_gds0_h2_dn14)) / (locals.var_gds0_h2 * locals.var_gds0_h2)),)
    } else {
        (locals.var_gamma, locals.var_gamma_dn0, locals.var_gamma_dn2, locals.var_gamma_dn4, locals.var_gamma_dn5, locals.var_gamma_dn6, locals.var_gamma_dn7, locals.var_gamma_dn8, locals.var_gamma_dn9, locals.var_gamma_dn10, locals.var_gamma_dn11, locals.var_gamma_dn14,)
    }
};
        locals.var_gamma = assign101680_e153833;
        locals.var_gamma_dn0 = assign101680_e153833_d_n0;
        locals.var_gamma_dn2 = assign101680_e153833_d_n2;
        locals.var_gamma_dn4 = assign101680_e153833_d_n4;
        locals.var_gamma_dn5 = assign101680_e153833_d_n5;
        locals.var_gamma_dn6 = assign101680_e153833_d_n6;
        locals.var_gamma_dn7 = assign101680_e153833_d_n7;
        locals.var_gamma_dn8 = assign101680_e153833_d_n8;
        locals.var_gamma_dn9 = assign101680_e153833_d_n9;
        locals.var_gamma_dn10 = assign101680_e153833_d_n10;
        locals.var_gamma_dn11 = assign101680_e153833_d_n11;
        locals.var_gamma_dn14 = assign101680_e153833_d_n14;

        let (assign101690_e153845, assign101690_e153845_d_n0, assign101690_e153845_d_n2, assign101690_e153845_d_n4, assign101690_e153845_d_n5, assign101690_e153845_d_n6, assign101690_e153845_d_n7, assign101690_e153845_d_n8, assign101690_e153845_d_n9, assign101690_e153845_d_n10, assign101690_e153845_d_n11, assign101690_e153845_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101690_e153838: f64 = (4.0 * locals.var_vgvt);
        let assign101690_e153840: f64 = (assign101690_e153838 * locals.var_sqrtkusail);
        let assign101690_e153841: f64 = (locals.var_kusai00 + assign101690_e153840);
        let assign101690_e153843: f64 = (assign101690_e153841 + locals.var_kusail);
        (assign101690_e153843, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn7 + (((4.0 * locals.var_vgvt_dn7) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn9 + (((4.0 * locals.var_vgvt_dn9) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn14 + (((4.0 * locals.var_vgvt_dn14) * locals.var_sqrtkusail) + (assign101690_e153838 * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign101690_e153845;
        locals.var_t7_dn0 = assign101690_e153845_d_n0;
        locals.var_t7_dn2 = assign101690_e153845_d_n2;
        locals.var_t7_dn4 = assign101690_e153845_d_n4;
        locals.var_t7_dn5 = assign101690_e153845_d_n5;
        locals.var_t7_dn6 = assign101690_e153845_d_n6;
        locals.var_t7_dn7 = assign101690_e153845_d_n7;
        locals.var_t7_dn8 = assign101690_e153845_d_n8;
        locals.var_t7_dn9 = assign101690_e153845_d_n9;
        locals.var_t7_dn10 = assign101690_e153845_d_n10;
        locals.var_t7_dn11 = assign101690_e153845_d_n11;
        locals.var_t7_dn14 = assign101690_e153845_d_n14;

        let (assign101700_e153866, assign101700_e153866_d_n0, assign101700_e153866_d_n2, assign101700_e153866_d_n4, assign101700_e153866_d_n5, assign101700_e153866_d_n6, assign101700_e153866_d_n7, assign101700_e153866_d_n8, assign101700_e153866_d_n9, assign101700_e153866_d_n10, assign101700_e153866_d_n11, assign101700_e153866_d_n14,) = {
    if (locals.var_guard2332 != 0.0) {
        let assign101700_e153849: f64 = (3.872983346207417 * locals.var_kusai00l);
        let assign101700_e153851: f64 = (assign101700_e153849 * locals.var_t7);
        let assign101700_e153854: f64 = (6.0 * locals.var_t2);
        let assign101700_e153857: f64 = (locals.var_gamma * locals.var_t2);
        let assign101700_e153859: f64 = (assign101700_e153857 * locals.var_vgvt);
        let assign101700_e153861: f64 = (assign101700_e153859 * locals.var_t5);
        let assign101700_e153862: f64 = (assign101700_e153861).sqrt();
        let assign101700_e153863: f64 = (assign101700_e153854 * assign101700_e153862);
        let assign101700_e153864: f64 = (assign101700_e153851 / assign101700_e153863);
        (assign101700_e153864, ((((((3.872983346207417 * locals.var_kusai00l_dn0) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn0)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn0) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn0 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn0)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn0)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn0)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn2) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn2)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn2) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn2 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn2)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn2)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn2)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn4) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn4)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn4) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn4 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn4)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn4)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn4)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn5) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn5)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn5) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn5 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn5)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn5)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn5)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn6) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn6)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn6) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn6 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn6)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn6)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn6)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn7) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn7)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn7) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn7 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn7)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn7)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn7)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn8) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn8)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn8) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn8 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn8)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn8)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn8)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn9) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn9)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn9) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn9 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn9)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn9)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn9)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn10) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn10)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn10) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn10 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn10)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn10)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn10)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn11) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn11)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn11) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn11 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn11)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn11)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn11)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)), ((((((3.872983346207417 * locals.var_kusai00l_dn14) * locals.var_t7) + (assign101700_e153849 * locals.var_t7_dn14)) * assign101700_e153863) - (assign101700_e153851 * (((6.0 * locals.var_t2_dn14) * assign101700_e153862) + (assign101700_e153854 * (((((((locals.var_gamma_dn14 * locals.var_t2) + (locals.var_gamma * locals.var_t2_dn14)) * locals.var_vgvt) + (assign101700_e153857 * locals.var_vgvt_dn14)) * locals.var_t5) + (assign101700_e153859 * locals.var_t5_dn14)) / (2.0 * assign101700_e153862)))))) / (assign101700_e153863 * assign101700_e153863)),)
    } else {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn14,)
    }
};
        locals.var_crl_f = assign101700_e153866;
        locals.var_crl_f_dn0 = assign101700_e153866_d_n0;
        locals.var_crl_f_dn2 = assign101700_e153866_d_n2;
        locals.var_crl_f_dn4 = assign101700_e153866_d_n4;
        locals.var_crl_f_dn5 = assign101700_e153866_d_n5;
        locals.var_crl_f_dn6 = assign101700_e153866_d_n6;
        locals.var_crl_f_dn7 = assign101700_e153866_d_n7;
        locals.var_crl_f_dn8 = assign101700_e153866_d_n8;
        locals.var_crl_f_dn9 = assign101700_e153866_d_n9;
        locals.var_crl_f_dn10 = assign101700_e153866_d_n10;
        locals.var_crl_f_dn11 = assign101700_e153866_d_n11;
        locals.var_crl_f_dn14 = assign101700_e153866_d_n14;

        let assign101710_e153869: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign101710_e153869;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn7 = (locals.var_mfactor * locals.var_ids_dn7);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn9 = (locals.var_mfactor * locals.var_ids_dn9);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn14 = (locals.var_mfactor * locals.var_ids_dn14);

        let assign101750_e153881: f64 = (locals.var_mfactor * locals.var_idsibpc);
        locals.var_idsibpce = assign101750_e153881;
        locals.var_idsibpce_dn0 = (locals.var_mfactor * locals.var_idsibpc_dn0);
        locals.var_idsibpce_dn2 = (locals.var_mfactor * locals.var_idsibpc_dn2);
        locals.var_idsibpce_dn4 = (locals.var_mfactor * locals.var_idsibpc_dn4);
        locals.var_idsibpce_dn5 = (locals.var_mfactor * locals.var_idsibpc_dn5);
        locals.var_idsibpce_dn6 = (locals.var_mfactor * locals.var_idsibpc_dn6);
        locals.var_idsibpce_dn7 = (locals.var_mfactor * locals.var_idsibpc_dn7);
        locals.var_idsibpce_dn8 = (locals.var_mfactor * locals.var_idsibpc_dn8);
        locals.var_idsibpce_dn9 = (locals.var_mfactor * locals.var_idsibpc_dn9);
        locals.var_idsibpce_dn10 = (locals.var_mfactor * locals.var_idsibpc_dn10);
        locals.var_idsibpce_dn11 = (locals.var_mfactor * locals.var_idsibpc_dn11);
        locals.var_idsibpce_dn14 = (locals.var_mfactor * locals.var_idsibpc_dn14);

        locals.var_ibjte = locals.var_wibjt;
        locals.var_ibjte_dn0 = locals.var_wibjt_dn0;
        locals.var_ibjte_dn2 = locals.var_wibjt_dn2;
        locals.var_ibjte_dn4 = locals.var_wibjt_dn4;
        locals.var_ibjte_dn5 = locals.var_wibjt_dn5;
        locals.var_ibjte_dn6 = locals.var_wibjt_dn6;
        locals.var_ibjte_dn7 = locals.var_wibjt_dn7;
        locals.var_ibjte_dn8 = locals.var_wibjt_dn8;
        locals.var_ibjte_dn9 = locals.var_wibjt_dn9;
        locals.var_ibjte_dn10 = locals.var_wibjt_dn10;
        locals.var_ibjte_dn11 = locals.var_wibjt_dn11;
        locals.var_ibjte_dn14 = locals.var_wibjt_dn14;

        locals.var_qgexte = 0.0;
        locals.var_qgexte_dn0 = 0.0;
        locals.var_qgexte_dn2 = 0.0;
        locals.var_qgexte_dn4 = 0.0;
        locals.var_qgexte_dn5 = 0.0;
        locals.var_qgexte_dn6 = 0.0;
        locals.var_qgexte_dn7 = 0.0;
        locals.var_qgexte_dn8 = 0.0;
        locals.var_qgexte_dn9 = 0.0;
        locals.var_qgexte_dn10 = 0.0;
        locals.var_qgexte_dn11 = 0.0;
        locals.var_qgexte_dn14 = 0.0;

        locals.var_qdexte = 0.0;
        locals.var_qdexte_dn0 = 0.0;
        locals.var_qdexte_dn2 = 0.0;
        locals.var_qdexte_dn4 = 0.0;
        locals.var_qdexte_dn5 = 0.0;
        locals.var_qdexte_dn6 = 0.0;
        locals.var_qdexte_dn7 = 0.0;
        locals.var_qdexte_dn8 = 0.0;
        locals.var_qdexte_dn9 = 0.0;
        locals.var_qdexte_dn10 = 0.0;
        locals.var_qdexte_dn11 = 0.0;
        locals.var_qdexte_dn14 = 0.0;

        locals.var_qsexte = 0.0;
        locals.var_qsexte_dn0 = 0.0;
        locals.var_qsexte_dn2 = 0.0;
        locals.var_qsexte_dn4 = 0.0;
        locals.var_qsexte_dn5 = 0.0;
        locals.var_qsexte_dn6 = 0.0;
        locals.var_qsexte_dn7 = 0.0;
        locals.var_qsexte_dn8 = 0.0;
        locals.var_qsexte_dn9 = 0.0;
        locals.var_qsexte_dn10 = 0.0;
        locals.var_qsexte_dn11 = 0.0;
        locals.var_qsexte_dn14 = 0.0;

        locals.var_qgov = 0.0;
        locals.var_qgov_dn0 = 0.0;
        locals.var_qgov_dn2 = 0.0;
        locals.var_qgov_dn4 = 0.0;
        locals.var_qgov_dn5 = 0.0;
        locals.var_qgov_dn6 = 0.0;
        locals.var_qgov_dn7 = 0.0;
        locals.var_qgov_dn8 = 0.0;
        locals.var_qgov_dn9 = 0.0;
        locals.var_qgov_dn10 = 0.0;
        locals.var_qgov_dn11 = 0.0;
        locals.var_qgov_dn14 = 0.0;

        locals.var_qdov = 0.0;
        locals.var_qdov_dn0 = 0.0;
        locals.var_qdov_dn2 = 0.0;
        locals.var_qdov_dn4 = 0.0;
        locals.var_qdov_dn5 = 0.0;
        locals.var_qdov_dn6 = 0.0;
        locals.var_qdov_dn7 = 0.0;
        locals.var_qdov_dn8 = 0.0;
        locals.var_qdov_dn9 = 0.0;
        locals.var_qdov_dn10 = 0.0;
        locals.var_qdov_dn11 = 0.0;
        locals.var_qdov_dn14 = 0.0;

        locals.var_qsov = 0.0;
        locals.var_qsov_dn0 = 0.0;
        locals.var_qsov_dn2 = 0.0;
        locals.var_qsov_dn4 = 0.0;
        locals.var_qsov_dn5 = 0.0;
        locals.var_qsov_dn6 = 0.0;
        locals.var_qsov_dn7 = 0.0;
        locals.var_qsov_dn8 = 0.0;
        locals.var_qsov_dn9 = 0.0;
        locals.var_qsov_dn10 = 0.0;
        locals.var_qsov_dn11 = 0.0;
        locals.var_qsov_dn14 = 0.0;

        locals.var_qdp = 0.0;
        locals.var_qdp_dn0 = 0.0;
        locals.var_qdp_dn2 = 0.0;
        locals.var_qdp_dn7 = 0.0;

        locals.var_qsp = 0.0;
        locals.var_qsp_dn2 = 0.0;
        locals.var_qsp_dn7 = 0.0;

        let assign101850_e153895: f64 = if ((locals.var_flg_nqs != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard2333 = assign101850_e153895;

        let (assign101860_e153899, assign101860_e153899_d_n0, assign101860_e153899_d_n2, assign101860_e153899_d_n4, assign101860_e153899_d_n5, assign101860_e153899_d_n6, assign101860_e153899_d_n7, assign101860_e153899_d_n8, assign101860_e153899_d_n9, assign101860_e153899_d_n10, assign101860_e153899_d_n11, assign101860_e153899_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101860_e153899;
        locals.var_qge_dn0 = assign101860_e153899_d_n0;
        locals.var_qge_dn2 = assign101860_e153899_d_n2;
        locals.var_qge_dn4 = assign101860_e153899_d_n4;
        locals.var_qge_dn5 = assign101860_e153899_d_n5;
        locals.var_qge_dn6 = assign101860_e153899_d_n6;
        locals.var_qge_dn7 = assign101860_e153899_d_n7;
        locals.var_qge_dn8 = assign101860_e153899_d_n8;
        locals.var_qge_dn9 = assign101860_e153899_d_n9;
        locals.var_qge_dn10 = assign101860_e153899_d_n10;
        locals.var_qge_dn11 = assign101860_e153899_d_n11;
        locals.var_qge_dn14 = assign101860_e153899_d_n14;

        let (assign101870_e153903, assign101870_e153903_d_n0, assign101870_e153903_d_n2, assign101870_e153903_d_n4, assign101870_e153903_d_n5, assign101870_e153903_d_n6, assign101870_e153903_d_n7, assign101870_e153903_d_n8, assign101870_e153903_d_n9, assign101870_e153903_d_n10, assign101870_e153903_d_n11, assign101870_e153903_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101870_e153903;
        locals.var_qde_dn0 = assign101870_e153903_d_n0;
        locals.var_qde_dn2 = assign101870_e153903_d_n2;
        locals.var_qde_dn4 = assign101870_e153903_d_n4;
        locals.var_qde_dn5 = assign101870_e153903_d_n5;
        locals.var_qde_dn6 = assign101870_e153903_d_n6;
        locals.var_qde_dn7 = assign101870_e153903_d_n7;
        locals.var_qde_dn8 = assign101870_e153903_d_n8;
        locals.var_qde_dn9 = assign101870_e153903_d_n9;
        locals.var_qde_dn10 = assign101870_e153903_d_n10;
        locals.var_qde_dn11 = assign101870_e153903_d_n11;
        locals.var_qde_dn14 = assign101870_e153903_d_n14;

        let (assign101880_e153907, assign101880_e153907_d_n0, assign101880_e153907_d_n2, assign101880_e153907_d_n4, assign101880_e153907_d_n5, assign101880_e153907_d_n6, assign101880_e153907_d_n7, assign101880_e153907_d_n8, assign101880_e153907_d_n9, assign101880_e153907_d_n10, assign101880_e153907_d_n11, assign101880_e153907_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101880_e153907;
        locals.var_qse_dn0 = assign101880_e153907_d_n0;
        locals.var_qse_dn2 = assign101880_e153907_d_n2;
        locals.var_qse_dn4 = assign101880_e153907_d_n4;
        locals.var_qse_dn5 = assign101880_e153907_d_n5;
        locals.var_qse_dn6 = assign101880_e153907_d_n6;
        locals.var_qse_dn7 = assign101880_e153907_d_n7;
        locals.var_qse_dn8 = assign101880_e153907_d_n8;
        locals.var_qse_dn9 = assign101880_e153907_d_n9;
        locals.var_qse_dn10 = assign101880_e153907_d_n10;
        locals.var_qse_dn11 = assign101880_e153907_d_n11;
        locals.var_qse_dn14 = assign101880_e153907_d_n14;

        let (assign101890_e153911, assign101890_e153911_d_n0, assign101890_e153911_d_n2, assign101890_e153911_d_n4, assign101890_e153911_d_n5, assign101890_e153911_d_n6, assign101890_e153911_d_n7, assign101890_e153911_d_n8, assign101890_e153911_d_n9, assign101890_e153911_d_n10, assign101890_e153911_d_n11, assign101890_e153911_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn14,)
    }
};
        locals.var_xd = assign101890_e153911;
        locals.var_xd_dn0 = assign101890_e153911_d_n0;
        locals.var_xd_dn2 = assign101890_e153911_d_n2;
        locals.var_xd_dn4 = assign101890_e153911_d_n4;
        locals.var_xd_dn5 = assign101890_e153911_d_n5;
        locals.var_xd_dn6 = assign101890_e153911_d_n6;
        locals.var_xd_dn7 = assign101890_e153911_d_n7;
        locals.var_xd_dn8 = assign101890_e153911_d_n8;
        locals.var_xd_dn9 = assign101890_e153911_d_n9;
        locals.var_xd_dn10 = assign101890_e153911_d_n10;
        locals.var_xd_dn11 = assign101890_e153911_d_n11;
        locals.var_xd_dn14 = assign101890_e153911_d_n14;

        let (assign101900_e153917, assign101900_e153917_d_n0, assign101900_e153917_d_n2, assign101900_e153917_d_n4, assign101900_e153917_d_n5, assign101900_e153917_d_n6, assign101900_e153917_d_n7, assign101900_e153917_d_n8, assign101900_e153917_d_n9, assign101900_e153917_d_n10, assign101900_e153917_d_n11, assign101900_e153917_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign101900_e153915: f64 = (locals.var_mfactor * locals.var_qb);
        (assign101900_e153915, (locals.var_mfactor * locals.var_qb_dn0), (locals.var_mfactor * locals.var_qb_dn2), (locals.var_mfactor * locals.var_qb_dn4), (locals.var_mfactor * locals.var_qb_dn5), (locals.var_mfactor * locals.var_qb_dn6), (locals.var_mfactor * locals.var_qb_dn7), (locals.var_mfactor * locals.var_qb_dn8), (locals.var_mfactor * locals.var_qb_dn9), (locals.var_mfactor * locals.var_qb_dn10), (locals.var_mfactor * locals.var_qb_dn11), (locals.var_mfactor * locals.var_qb_dn14),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn0, locals.var_qbulk_dn2, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn14,)
    }
};
        locals.var_qbulk = assign101900_e153917;
        locals.var_qbulk_dn0 = assign101900_e153917_d_n0;
        locals.var_qbulk_dn2 = assign101900_e153917_d_n2;
        locals.var_qbulk_dn4 = assign101900_e153917_d_n4;
        locals.var_qbulk_dn5 = assign101900_e153917_d_n5;
        locals.var_qbulk_dn6 = assign101900_e153917_d_n6;
        locals.var_qbulk_dn7 = assign101900_e153917_d_n7;
        locals.var_qbulk_dn8 = assign101900_e153917_d_n8;
        locals.var_qbulk_dn9 = assign101900_e153917_d_n9;
        locals.var_qbulk_dn10 = assign101900_e153917_d_n10;
        locals.var_qbulk_dn11 = assign101900_e153917_d_n11;
        locals.var_qbulk_dn14 = assign101900_e153917_d_n14;

        let (assign101910_e153923, assign101910_e153923_d_n0, assign101910_e153923_d_n2, assign101910_e153923_d_n4, assign101910_e153923_d_n5, assign101910_e153923_d_n6, assign101910_e153923_d_n7, assign101910_e153923_d_n8, assign101910_e153923_d_n9, assign101910_e153923_d_n10, assign101910_e153923_d_n11, assign101910_e153923_d_n14,) = {
    if (locals.var_guard2333 != 0.0) {
        let assign101910_e153921: f64 = (locals.var_mfactor * locals.var_qi);
        (assign101910_e153921, (locals.var_mfactor * locals.var_qi_dn0), (locals.var_mfactor * locals.var_qi_dn2), (locals.var_mfactor * locals.var_qi_dn4), (locals.var_mfactor * locals.var_qi_dn5), (locals.var_mfactor * locals.var_qi_dn6), (locals.var_mfactor * locals.var_qi_dn7), (locals.var_mfactor * locals.var_qi_dn8), (locals.var_mfactor * locals.var_qi_dn9), (locals.var_mfactor * locals.var_qi_dn10), (locals.var_mfactor * locals.var_qi_dn11), (locals.var_mfactor * locals.var_qi_dn14),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn14,)
    }
};
        locals.var_qi = assign101910_e153923;
        locals.var_qi_dn0 = assign101910_e153923_d_n0;
        locals.var_qi_dn2 = assign101910_e153923_d_n2;
        locals.var_qi_dn4 = assign101910_e153923_d_n4;
        locals.var_qi_dn5 = assign101910_e153923_d_n5;
        locals.var_qi_dn6 = assign101910_e153923_d_n6;
        locals.var_qi_dn7 = assign101910_e153923_d_n7;
        locals.var_qi_dn8 = assign101910_e153923_d_n8;
        locals.var_qi_dn9 = assign101910_e153923_d_n9;
        locals.var_qi_dn10 = assign101910_e153923_d_n10;
        locals.var_qi_dn11 = assign101910_e153923_d_n11;
        locals.var_qi_dn14 = assign101910_e153923_d_n14;

        let (assign101920_e153933, assign101920_e153933_d_n0, assign101920_e153933_d_n2, assign101920_e153933_d_n4, assign101920_e153933_d_n5, assign101920_e153933_d_n6, assign101920_e153933_d_n7, assign101920_e153933_d_n8, assign101920_e153933_d_n9, assign101920_e153933_d_n10, assign101920_e153933_d_n11, assign101920_e153933_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101920_e153929: f64 = (locals.var_qb + locals.var_qi);
        let assign101920_e153930: f64 = (-assign101920_e153929);
        let assign101920_e153931: f64 = (locals.var_mfactor * assign101920_e153930);
        (assign101920_e153931, (locals.var_mfactor * (-(locals.var_qb_dn0 + locals.var_qi_dn0))), (locals.var_mfactor * (-(locals.var_qb_dn2 + locals.var_qi_dn2))), (locals.var_mfactor * (-(locals.var_qb_dn4 + locals.var_qi_dn4))), (locals.var_mfactor * (-(locals.var_qb_dn5 + locals.var_qi_dn5))), (locals.var_mfactor * (-(locals.var_qb_dn6 + locals.var_qi_dn6))), (locals.var_mfactor * (-(locals.var_qb_dn7 + locals.var_qi_dn7))), (locals.var_mfactor * (-(locals.var_qb_dn8 + locals.var_qi_dn8))), (locals.var_mfactor * (-(locals.var_qb_dn9 + locals.var_qi_dn9))), (locals.var_mfactor * (-(locals.var_qb_dn10 + locals.var_qi_dn10))), (locals.var_mfactor * (-(locals.var_qb_dn11 + locals.var_qi_dn11))), (locals.var_mfactor * (-(locals.var_qb_dn14 + locals.var_qi_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign101920_e153933;
        locals.var_qge_dn0 = assign101920_e153933_d_n0;
        locals.var_qge_dn2 = assign101920_e153933_d_n2;
        locals.var_qge_dn4 = assign101920_e153933_d_n4;
        locals.var_qge_dn5 = assign101920_e153933_d_n5;
        locals.var_qge_dn6 = assign101920_e153933_d_n6;
        locals.var_qge_dn7 = assign101920_e153933_d_n7;
        locals.var_qge_dn8 = assign101920_e153933_d_n8;
        locals.var_qge_dn9 = assign101920_e153933_d_n9;
        locals.var_qge_dn10 = assign101920_e153933_d_n10;
        locals.var_qge_dn11 = assign101920_e153933_d_n11;
        locals.var_qge_dn14 = assign101920_e153933_d_n14;

        let (assign101930_e153940, assign101930_e153940_d_n0, assign101930_e153940_d_n2, assign101930_e153940_d_n4, assign101930_e153940_d_n5, assign101930_e153940_d_n6, assign101930_e153940_d_n7, assign101930_e153940_d_n8, assign101930_e153940_d_n9, assign101930_e153940_d_n10, assign101930_e153940_d_n11, assign101930_e153940_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101930_e153938: f64 = (locals.var_mfactor * locals.var_qd);
        (assign101930_e153938, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn4), (locals.var_mfactor * locals.var_qd_dn5), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn8), (locals.var_mfactor * locals.var_qd_dn9), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn14),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign101930_e153940;
        locals.var_qde_dn0 = assign101930_e153940_d_n0;
        locals.var_qde_dn2 = assign101930_e153940_d_n2;
        locals.var_qde_dn4 = assign101930_e153940_d_n4;
        locals.var_qde_dn5 = assign101930_e153940_d_n5;
        locals.var_qde_dn6 = assign101930_e153940_d_n6;
        locals.var_qde_dn7 = assign101930_e153940_d_n7;
        locals.var_qde_dn8 = assign101930_e153940_d_n8;
        locals.var_qde_dn9 = assign101930_e153940_d_n9;
        locals.var_qde_dn10 = assign101930_e153940_d_n10;
        locals.var_qde_dn11 = assign101930_e153940_d_n11;
        locals.var_qde_dn14 = assign101930_e153940_d_n14;

    }

    pub(super) fn stamp_transient_block_375(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv14 = ctx.node_voltage(nodes[14]);
        let (assign101940_e153949, assign101940_e153949_d_n0, assign101940_e153949_d_n2, assign101940_e153949_d_n4, assign101940_e153949_d_n5, assign101940_e153949_d_n6, assign101940_e153949_d_n7, assign101940_e153949_d_n8, assign101940_e153949_d_n9, assign101940_e153949_d_n10, assign101940_e153949_d_n11, assign101940_e153949_d_n14,) = {
    if (locals.var_guard2333 == 0.0) {
        let assign101940_e153946: f64 = (locals.var_qi - locals.var_qd);
        let assign101940_e153947: f64 = (locals.var_mfactor * assign101940_e153946);
        (assign101940_e153947, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn4 - locals.var_qd_dn4)), (locals.var_mfactor * (locals.var_qi_dn5 - locals.var_qd_dn5)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn8 - locals.var_qd_dn8)), (locals.var_mfactor * (locals.var_qi_dn9 - locals.var_qd_dn9)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn14 - locals.var_qd_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign101940_e153949;
        locals.var_qse_dn0 = assign101940_e153949_d_n0;
        locals.var_qse_dn2 = assign101940_e153949_d_n2;
        locals.var_qse_dn4 = assign101940_e153949_d_n4;
        locals.var_qse_dn5 = assign101940_e153949_d_n5;
        locals.var_qse_dn6 = assign101940_e153949_d_n6;
        locals.var_qse_dn7 = assign101940_e153949_d_n7;
        locals.var_qse_dn8 = assign101940_e153949_d_n8;
        locals.var_qse_dn9 = assign101940_e153949_d_n9;
        locals.var_qse_dn10 = assign101940_e153949_d_n10;
        locals.var_qse_dn11 = assign101940_e153949_d_n11;
        locals.var_qse_dn14 = assign101940_e153949_d_n14;

        let (assign101950_e153955, assign101950_e153955_d_n0, assign101950_e153955_d_n2, assign101950_e153955_d_n4, assign101950_e153955_d_n5, assign101950_e153955_d_n6, assign101950_e153955_d_n7, assign101950_e153955_d_n8, assign101950_e153955_d_n9, assign101950_e153955_d_n10, assign101950_e153955_d_n11, assign101950_e153955_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101950_e153953: f64 = (locals.var_mks_dlyov * locals.var_psl);
        (assign101950_e153953, ((locals.var_mks_dlyov_dn0 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn0)), ((locals.var_mks_dlyov_dn2 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn2)), ((locals.var_mks_dlyov_dn4 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn4)), ((locals.var_mks_dlyov_dn5 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn5)), ((locals.var_mks_dlyov_dn6 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn6)), ((locals.var_mks_dlyov_dn7 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn7)), ((locals.var_mks_dlyov_dn8 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn8)), ((locals.var_mks_dlyov_dn9 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn9)), ((locals.var_mks_dlyov_dn10 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn10)), ((locals.var_mks_dlyov_dn11 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn11)), ((locals.var_mks_dlyov_dn14 * locals.var_psl) + (locals.var_mks_dlyov * locals.var_psl_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101950_e153955;
        locals.var_mks_dlyov_dn0 = assign101950_e153955_d_n0;
        locals.var_mks_dlyov_dn2 = assign101950_e153955_d_n2;
        locals.var_mks_dlyov_dn4 = assign101950_e153955_d_n4;
        locals.var_mks_dlyov_dn5 = assign101950_e153955_d_n5;
        locals.var_mks_dlyov_dn6 = assign101950_e153955_d_n6;
        locals.var_mks_dlyov_dn7 = assign101950_e153955_d_n7;
        locals.var_mks_dlyov_dn8 = assign101950_e153955_d_n8;
        locals.var_mks_dlyov_dn9 = assign101950_e153955_d_n9;
        locals.var_mks_dlyov_dn10 = assign101950_e153955_d_n10;
        locals.var_mks_dlyov_dn11 = assign101950_e153955_d_n11;
        locals.var_mks_dlyov_dn14 = assign101950_e153955_d_n14;

        let (assign101960_e153968, assign101960_e153968_d_n0, assign101960_e153968_d_n2, assign101960_e153968_d_n4, assign101960_e153968_d_n5, assign101960_e153968_d_n6, assign101960_e153968_d_n7, assign101960_e153968_d_n8, assign101960_e153968_d_n9, assign101960_e153968_d_n10, assign101960_e153968_d_n11, assign101960_e153968_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101960_e153959: f64 = (locals.var_mks_dlyov * locals.var_mks_dlyov);
        let assign101960_e153962: f64 = (4.0 * 1e-12);
        let assign101960_e153964: f64 = (assign101960_e153962 * 1e-12);
        let assign101960_e153965: f64 = (assign101960_e153959 + assign101960_e153964);
        let assign101960_e153966: f64 = (assign101960_e153965).sqrt();
        (assign101960_e153966, (((locals.var_mks_dlyov_dn0 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn0)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn2 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn2)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn4 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn4)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn5 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn5)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn6 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn6)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn7 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn7)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn8 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn8)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn9 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn9)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn10 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn10)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn11 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn11)) / (2.0 * assign101960_e153966)), (((locals.var_mks_dlyov_dn14 * locals.var_mks_dlyov) + (locals.var_mks_dlyov * locals.var_mks_dlyov_dn14)) / (2.0 * assign101960_e153966)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign101960_e153968;
        locals.var_tmf2_dn0 = assign101960_e153968_d_n0;
        locals.var_tmf2_dn2 = assign101960_e153968_d_n2;
        locals.var_tmf2_dn4 = assign101960_e153968_d_n4;
        locals.var_tmf2_dn5 = assign101960_e153968_d_n5;
        locals.var_tmf2_dn6 = assign101960_e153968_d_n6;
        locals.var_tmf2_dn7 = assign101960_e153968_d_n7;
        locals.var_tmf2_dn8 = assign101960_e153968_d_n8;
        locals.var_tmf2_dn9 = assign101960_e153968_d_n9;
        locals.var_tmf2_dn10 = assign101960_e153968_d_n10;
        locals.var_tmf2_dn11 = assign101960_e153968_d_n11;
        locals.var_tmf2_dn14 = assign101960_e153968_d_n14;

        let (assign101970_e153978, assign101970_e153978_d_n0, assign101970_e153978_d_n2, assign101970_e153978_d_n4, assign101970_e153978_d_n5, assign101970_e153978_d_n6, assign101970_e153978_d_n7, assign101970_e153978_d_n8, assign101970_e153978_d_n9, assign101970_e153978_d_n10, assign101970_e153978_d_n11, assign101970_e153978_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101970_e153974: f64 = (locals.var_mks_dlyov / locals.var_tmf2);
        let assign101970_e153975: f64 = (1.0 + assign101970_e153974);
        let assign101970_e153976: f64 = (0.5 * assign101970_e153975);
        (assign101970_e153976, (0.5 * (((locals.var_mks_dlyov_dn0 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn2 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn4 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn5 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn6 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn7 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn8 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn9 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn10 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn11 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_mks_dlyov_dn14 * locals.var_tmf2) - (locals.var_mks_dlyov * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign101970_e153978;
        locals.var_t0_dn0 = assign101970_e153978_d_n0;
        locals.var_t0_dn2 = assign101970_e153978_d_n2;
        locals.var_t0_dn4 = assign101970_e153978_d_n4;
        locals.var_t0_dn5 = assign101970_e153978_d_n5;
        locals.var_t0_dn6 = assign101970_e153978_d_n6;
        locals.var_t0_dn7 = assign101970_e153978_d_n7;
        locals.var_t0_dn8 = assign101970_e153978_d_n8;
        locals.var_t0_dn9 = assign101970_e153978_d_n9;
        locals.var_t0_dn10 = assign101970_e153978_d_n10;
        locals.var_t0_dn11 = assign101970_e153978_d_n11;
        locals.var_t0_dn14 = assign101970_e153978_d_n14;

        let (assign101980_e153986, assign101980_e153986_d_n0, assign101980_e153986_d_n2, assign101980_e153986_d_n4, assign101980_e153986_d_n5, assign101980_e153986_d_n6, assign101980_e153986_d_n7, assign101980_e153986_d_n8, assign101980_e153986_d_n9, assign101980_e153986_d_n10, assign101980_e153986_d_n11, assign101980_e153986_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign101980_e153983: f64 = (locals.var_mks_dlyov + locals.var_tmf2);
        let assign101980_e153984: f64 = (0.5 * assign101980_e153983);
        (assign101980_e153984, (0.5 * (locals.var_mks_dlyov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_mks_dlyov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_mks_dlyov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_mks_dlyov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_mks_dlyov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_mks_dlyov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_mks_dlyov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_mks_dlyov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_mks_dlyov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_mks_dlyov_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_mks_dlyov_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign101980_e153986;
        locals.var_mks_dlyov_dn0 = assign101980_e153986_d_n0;
        locals.var_mks_dlyov_dn2 = assign101980_e153986_d_n2;
        locals.var_mks_dlyov_dn4 = assign101980_e153986_d_n4;
        locals.var_mks_dlyov_dn5 = assign101980_e153986_d_n5;
        locals.var_mks_dlyov_dn6 = assign101980_e153986_d_n6;
        locals.var_mks_dlyov_dn7 = assign101980_e153986_d_n7;
        locals.var_mks_dlyov_dn8 = assign101980_e153986_d_n8;
        locals.var_mks_dlyov_dn9 = assign101980_e153986_d_n9;
        locals.var_mks_dlyov_dn10 = assign101980_e153986_d_n10;
        locals.var_mks_dlyov_dn11 = assign101980_e153986_d_n11;
        locals.var_mks_dlyov_dn14 = assign101980_e153986_d_n14;

        let assign101990_e153989: f64 = if locals.var_mks_dlyov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2334 = assign101990_e153989;

        let (assign102000_e153995, assign102000_e153995_d_n0, assign102000_e153995_d_n2, assign102000_e153995_d_n4, assign102000_e153995_d_n5, assign102000_e153995_d_n6, assign102000_e153995_d_n7, assign102000_e153995_d_n8, assign102000_e153995_d_n9, assign102000_e153995_d_n10, assign102000_e153995_d_n11, assign102000_e153995_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2334 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mks_dlyov, locals.var_mks_dlyov_dn0, locals.var_mks_dlyov_dn2, locals.var_mks_dlyov_dn4, locals.var_mks_dlyov_dn5, locals.var_mks_dlyov_dn6, locals.var_mks_dlyov_dn7, locals.var_mks_dlyov_dn8, locals.var_mks_dlyov_dn9, locals.var_mks_dlyov_dn10, locals.var_mks_dlyov_dn11, locals.var_mks_dlyov_dn14,)
    }
};
        locals.var_mks_dlyov = assign102000_e153995;
        locals.var_mks_dlyov_dn0 = assign102000_e153995_d_n0;
        locals.var_mks_dlyov_dn2 = assign102000_e153995_d_n2;
        locals.var_mks_dlyov_dn4 = assign102000_e153995_d_n4;
        locals.var_mks_dlyov_dn5 = assign102000_e153995_d_n5;
        locals.var_mks_dlyov_dn6 = assign102000_e153995_d_n6;
        locals.var_mks_dlyov_dn7 = assign102000_e153995_d_n7;
        locals.var_mks_dlyov_dn8 = assign102000_e153995_d_n8;
        locals.var_mks_dlyov_dn9 = assign102000_e153995_d_n9;
        locals.var_mks_dlyov_dn10 = assign102000_e153995_d_n10;
        locals.var_mks_dlyov_dn11 = assign102000_e153995_d_n11;
        locals.var_mks_dlyov_dn14 = assign102000_e153995_d_n14;

        let (assign102010_e154001, assign102010_e154001_d_n0, assign102010_e154001_d_n2, assign102010_e154001_d_n4, assign102010_e154001_d_n5, assign102010_e154001_d_n6, assign102010_e154001_d_n7, assign102010_e154001_d_n8, assign102010_e154001_d_n9, assign102010_e154001_d_n10, assign102010_e154001_d_n11, assign102010_e154001_d_n14,) = {
    if ((p.p29 != 0.0) && (locals.var_guard2334 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102010_e154001;
        locals.var_t0_dn0 = assign102010_e154001_d_n0;
        locals.var_t0_dn2 = assign102010_e154001_d_n2;
        locals.var_t0_dn4 = assign102010_e154001_d_n4;
        locals.var_t0_dn5 = assign102010_e154001_d_n5;
        locals.var_t0_dn6 = assign102010_e154001_d_n6;
        locals.var_t0_dn7 = assign102010_e154001_d_n7;
        locals.var_t0_dn8 = assign102010_e154001_d_n8;
        locals.var_t0_dn9 = assign102010_e154001_d_n9;
        locals.var_t0_dn10 = assign102010_e154001_d_n10;
        locals.var_t0_dn11 = assign102010_e154001_d_n11;
        locals.var_t0_dn14 = assign102010_e154001_d_n14;

        let (assign102020_e154007, assign102020_e154007_d_n0, assign102020_e154007_d_n2, assign102020_e154007_d_n4, assign102020_e154007_d_n5, assign102020_e154007_d_n6, assign102020_e154007_d_n7, assign102020_e154007_d_n8, assign102020_e154007_d_n9, assign102020_e154007_d_n10, assign102020_e154007_d_n11, assign102020_e154007_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102020_e154005: f64 = (locals.var_mks_dlyov * locals.var_cox0);
        (assign102020_e154005, (locals.var_mks_dlyov_dn0 * locals.var_cox0), (locals.var_mks_dlyov_dn2 * locals.var_cox0), (locals.var_mks_dlyov_dn4 * locals.var_cox0), (locals.var_mks_dlyov_dn5 * locals.var_cox0), (locals.var_mks_dlyov_dn6 * locals.var_cox0), (locals.var_mks_dlyov_dn7 * locals.var_cox0), (locals.var_mks_dlyov_dn8 * locals.var_cox0), (locals.var_mks_dlyov_dn9 * locals.var_cox0), (locals.var_mks_dlyov_dn10 * locals.var_cox0), (locals.var_mks_dlyov_dn11 * locals.var_cox0), (locals.var_mks_dlyov_dn14 * locals.var_cox0),)
    } else {
        (locals.var_tauov, locals.var_tauov_dn0, locals.var_tauov_dn2, locals.var_tauov_dn4, locals.var_tauov_dn5, locals.var_tauov_dn6, locals.var_tauov_dn7, locals.var_tauov_dn8, locals.var_tauov_dn9, locals.var_tauov_dn10, locals.var_tauov_dn11, locals.var_tauov_dn14,)
    }
};
        locals.var_tauov = assign102020_e154007;
        locals.var_tauov_dn0 = assign102020_e154007_d_n0;
        locals.var_tauov_dn2 = assign102020_e154007_d_n2;
        locals.var_tauov_dn4 = assign102020_e154007_d_n4;
        locals.var_tauov_dn5 = assign102020_e154007_d_n5;
        locals.var_tauov_dn6 = assign102020_e154007_d_n6;
        locals.var_tauov_dn7 = assign102020_e154007_d_n7;
        locals.var_tauov_dn8 = assign102020_e154007_d_n8;
        locals.var_tauov_dn9 = assign102020_e154007_d_n9;
        locals.var_tauov_dn10 = assign102020_e154007_d_n10;
        locals.var_tauov_dn11 = assign102020_e154007_d_n11;
        locals.var_tauov_dn14 = assign102020_e154007_d_n14;

        let (assign102030_e154011, assign102030_e154011_d_n0, assign102030_e154011_d_n2, assign102030_e154011_d_n4, assign102030_e154011_d_n5, assign102030_e154011_d_n6, assign102030_e154011_d_n7, assign102030_e154011_d_n8, assign102030_e154011_d_n9, assign102030_e154011_d_n10, assign102030_e154011_d_n11, assign102030_e154011_d_n14,) = {
    if (p.p29 != 0.0) {
        ((nv14 - 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102030_e154011;
        locals.var_qbd_nqs_dn0 = assign102030_e154011_d_n0;
        locals.var_qbd_nqs_dn2 = assign102030_e154011_d_n2;
        locals.var_qbd_nqs_dn4 = assign102030_e154011_d_n4;
        locals.var_qbd_nqs_dn5 = assign102030_e154011_d_n5;
        locals.var_qbd_nqs_dn6 = assign102030_e154011_d_n6;
        locals.var_qbd_nqs_dn7 = assign102030_e154011_d_n7;
        locals.var_qbd_nqs_dn8 = assign102030_e154011_d_n8;
        locals.var_qbd_nqs_dn9 = assign102030_e154011_d_n9;
        locals.var_qbd_nqs_dn10 = assign102030_e154011_d_n10;
        locals.var_qbd_nqs_dn11 = assign102030_e154011_d_n11;
        locals.var_qbd_nqs_dn14 = assign102030_e154011_d_n14;

        let (assign102040_e154019, assign102040_e154019_d_n0, assign102040_e154019_d_n2, assign102040_e154019_d_n4, assign102040_e154019_d_n5, assign102040_e154019_d_n6, assign102040_e154019_d_n7, assign102040_e154019_d_n8, assign102040_e154019_d_n9, assign102040_e154019_d_n10, assign102040_e154019_d_n11, assign102040_e154019_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102040_e154015: f64 = (locals.var_qbd_nqs - locals.var_qbd_qs);
        let assign102040_e154017: f64 = (assign102040_e154015 / locals.var_tauov);
        (assign102040_e154017, ((((locals.var_qbd_nqs_dn0 - locals.var_qbd_qs_dn0) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn0)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn2 - locals.var_qbd_qs_dn2) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn2)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn4 - locals.var_qbd_qs_dn4) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn4)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn5 - locals.var_qbd_qs_dn5) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn5)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn6 - locals.var_qbd_qs_dn6) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn6)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn7 - locals.var_qbd_qs_dn7) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn7)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn8 - locals.var_qbd_qs_dn8) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn8)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn9 - locals.var_qbd_qs_dn9) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn9)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn10 - locals.var_qbd_qs_dn10) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn10)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn11 - locals.var_qbd_qs_dn11) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn11)) / (locals.var_tauov * locals.var_tauov)), ((((locals.var_qbd_nqs_dn14 - locals.var_qbd_qs_dn14) * locals.var_tauov) - (assign102040_e154015 * locals.var_tauov_dn14)) / (locals.var_tauov * locals.var_tauov)),)
    } else {
        (locals.var_ibd_nqs, locals.var_ibd_nqs_dn0, locals.var_ibd_nqs_dn2, locals.var_ibd_nqs_dn4, locals.var_ibd_nqs_dn5, locals.var_ibd_nqs_dn6, locals.var_ibd_nqs_dn7, locals.var_ibd_nqs_dn8, locals.var_ibd_nqs_dn9, locals.var_ibd_nqs_dn10, locals.var_ibd_nqs_dn11, locals.var_ibd_nqs_dn14,)
    }
};
        locals.var_ibd_nqs = assign102040_e154019;
        locals.var_ibd_nqs_dn0 = assign102040_e154019_d_n0;
        locals.var_ibd_nqs_dn2 = assign102040_e154019_d_n2;
        locals.var_ibd_nqs_dn4 = assign102040_e154019_d_n4;
        locals.var_ibd_nqs_dn5 = assign102040_e154019_d_n5;
        locals.var_ibd_nqs_dn6 = assign102040_e154019_d_n6;
        locals.var_ibd_nqs_dn7 = assign102040_e154019_d_n7;
        locals.var_ibd_nqs_dn8 = assign102040_e154019_d_n8;
        locals.var_ibd_nqs_dn9 = assign102040_e154019_d_n9;
        locals.var_ibd_nqs_dn10 = assign102040_e154019_d_n10;
        locals.var_ibd_nqs_dn11 = assign102040_e154019_d_n11;
        locals.var_ibd_nqs_dn14 = assign102040_e154019_d_n14;

        let (assign102050_e154027, assign102050_e154027_d_n0, assign102050_e154027_d_n2, assign102050_e154027_d_n4, assign102050_e154027_d_n5, assign102050_e154027_d_n6, assign102050_e154027_d_n7, assign102050_e154027_d_n8, assign102050_e154027_d_n9, assign102050_e154027_d_n10, assign102050_e154027_d_n11, assign102050_e154027_d_n14,) = {
    if (p.p29 != 0.0) {
        let assign102050_e154024: f64 = (locals.var_qbd_qs - locals.var_qbd_nqs);
        let assign102050_e154025: f64 = (locals.var_qovd - assign102050_e154024);
        (assign102050_e154025, (locals.var_qovd_dn0 - (locals.var_qbd_qs_dn0 - locals.var_qbd_nqs_dn0)), (locals.var_qovd_dn2 - (locals.var_qbd_qs_dn2 - locals.var_qbd_nqs_dn2)), (locals.var_qovd_dn4 - (locals.var_qbd_qs_dn4 - locals.var_qbd_nqs_dn4)), (locals.var_qovd_dn5 - (locals.var_qbd_qs_dn5 - locals.var_qbd_nqs_dn5)), (locals.var_qovd_dn6 - (locals.var_qbd_qs_dn6 - locals.var_qbd_nqs_dn6)), (locals.var_qovd_dn7 - (locals.var_qbd_qs_dn7 - locals.var_qbd_nqs_dn7)), (locals.var_qovd_dn8 - (locals.var_qbd_qs_dn8 - locals.var_qbd_nqs_dn8)), (locals.var_qovd_dn9 - (locals.var_qbd_qs_dn9 - locals.var_qbd_nqs_dn9)), (locals.var_qovd_dn10 - (locals.var_qbd_qs_dn10 - locals.var_qbd_nqs_dn10)), (locals.var_qovd_dn11 - (locals.var_qbd_qs_dn11 - locals.var_qbd_nqs_dn11)), (locals.var_qovd_dn14 - (locals.var_qbd_qs_dn14 - locals.var_qbd_nqs_dn14)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign102050_e154027;
        locals.var_qovd_dn0 = assign102050_e154027_d_n0;
        locals.var_qovd_dn2 = assign102050_e154027_d_n2;
        locals.var_qovd_dn4 = assign102050_e154027_d_n4;
        locals.var_qovd_dn5 = assign102050_e154027_d_n5;
        locals.var_qovd_dn6 = assign102050_e154027_d_n6;
        locals.var_qovd_dn7 = assign102050_e154027_d_n7;
        locals.var_qovd_dn8 = assign102050_e154027_d_n8;
        locals.var_qovd_dn9 = assign102050_e154027_d_n9;
        locals.var_qovd_dn10 = assign102050_e154027_d_n10;
        locals.var_qovd_dn11 = assign102050_e154027_d_n11;
        locals.var_qovd_dn14 = assign102050_e154027_d_n14;

        let (assign102060_e154031, assign102060_e154031_d_n0, assign102060_e154031_d_n2, assign102060_e154031_d_n4, assign102060_e154031_d_n5, assign102060_e154031_d_n6, assign102060_e154031_d_n7, assign102060_e154031_d_n8, assign102060_e154031_d_n9, assign102060_e154031_d_n10, assign102060_e154031_d_n11, assign102060_e154031_d_n14,) = {
    if (p.p29 != 0.0) {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn14,)
    }
};
        locals.var_qbdld = assign102060_e154031;
        locals.var_qbdld_dn0 = assign102060_e154031_d_n0;
        locals.var_qbdld_dn2 = assign102060_e154031_d_n2;
        locals.var_qbdld_dn4 = assign102060_e154031_d_n4;
        locals.var_qbdld_dn5 = assign102060_e154031_d_n5;
        locals.var_qbdld_dn6 = assign102060_e154031_d_n6;
        locals.var_qbdld_dn7 = assign102060_e154031_d_n7;
        locals.var_qbdld_dn8 = assign102060_e154031_d_n8;
        locals.var_qbdld_dn9 = assign102060_e154031_d_n9;
        locals.var_qbdld_dn10 = assign102060_e154031_d_n10;
        locals.var_qbdld_dn11 = assign102060_e154031_d_n11;
        locals.var_qbdld_dn14 = assign102060_e154031_d_n14;

        let (assign102070_e154036, assign102070_e154036_d_n0, assign102070_e154036_d_n2, assign102070_e154036_d_n4, assign102070_e154036_d_n5, assign102070_e154036_d_n6, assign102070_e154036_d_n7, assign102070_e154036_d_n8, assign102070_e154036_d_n9, assign102070_e154036_d_n10, assign102070_e154036_d_n11, assign102070_e154036_d_n14,) = {
    if (p.p29 == 0.0) {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn11, locals.var_qbd_qs_dn14,)
    } else {
        (locals.var_qbd_nqs, locals.var_qbd_nqs_dn0, locals.var_qbd_nqs_dn2, locals.var_qbd_nqs_dn4, locals.var_qbd_nqs_dn5, locals.var_qbd_nqs_dn6, locals.var_qbd_nqs_dn7, locals.var_qbd_nqs_dn8, locals.var_qbd_nqs_dn9, locals.var_qbd_nqs_dn10, locals.var_qbd_nqs_dn11, locals.var_qbd_nqs_dn14,)
    }
};
        locals.var_qbd_nqs = assign102070_e154036;
        locals.var_qbd_nqs_dn0 = assign102070_e154036_d_n0;
        locals.var_qbd_nqs_dn2 = assign102070_e154036_d_n2;
        locals.var_qbd_nqs_dn4 = assign102070_e154036_d_n4;
        locals.var_qbd_nqs_dn5 = assign102070_e154036_d_n5;
        locals.var_qbd_nqs_dn6 = assign102070_e154036_d_n6;
        locals.var_qbd_nqs_dn7 = assign102070_e154036_d_n7;
        locals.var_qbd_nqs_dn8 = assign102070_e154036_d_n8;
        locals.var_qbd_nqs_dn9 = assign102070_e154036_d_n9;
        locals.var_qbd_nqs_dn10 = assign102070_e154036_d_n10;
        locals.var_qbd_nqs_dn11 = assign102070_e154036_d_n11;
        locals.var_qbd_nqs_dn14 = assign102070_e154036_d_n14;

        let assign102080_e154039: f64 = if p.p22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2335 = assign102080_e154039;

        let (assign102090_e154053, assign102090_e154053_d_n0, assign102090_e154053_d_n2, assign102090_e154053_d_n4, assign102090_e154053_d_n5, assign102090_e154053_d_n6, assign102090_e154053_d_n7, assign102090_e154053_d_n8, assign102090_e154053_d_n9, assign102090_e154053_d_n10, assign102090_e154053_d_n11, assign102090_e154053_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102090_e154044: f64 = (locals.var_qgbo - locals.var_qovd);
        let assign102090_e154046: f64 = (assign102090_e154044 - locals.var_qovs);
        let assign102090_e154048: f64 = (assign102090_e154046 + locals.var_qgos);
        let assign102090_e154050: f64 = (assign102090_e154048 + locals.var_qgod);
        let assign102090_e154051: f64 = (locals.var_mfactor * assign102090_e154050);
        (assign102090_e154051, (locals.var_mfactor * ((((-locals.var_qovd_dn0) - locals.var_qovs_dn0) + locals.var_qgos_dn0) + locals.var_qgod_dn0)), (locals.var_mfactor * ((((-locals.var_qovd_dn2) - locals.var_qovs_dn2) + locals.var_qgos_dn2) + locals.var_qgod_dn2)), (locals.var_mfactor * ((((-locals.var_qovd_dn4) - locals.var_qovs_dn4) + locals.var_qgos_dn4) + locals.var_qgod_dn4)), (locals.var_mfactor * ((((-locals.var_qovd_dn5) - locals.var_qovs_dn5) + locals.var_qgos_dn5) + locals.var_qgod_dn5)), (locals.var_mfactor * ((((-locals.var_qovd_dn6) - locals.var_qovs_dn6) + locals.var_qgos_dn6) + locals.var_qgod_dn6)), (locals.var_mfactor * ((((locals.var_qgbo_dn7 - locals.var_qovd_dn7) - locals.var_qovs_dn7) + locals.var_qgos_dn7) + locals.var_qgod_dn7)), (locals.var_mfactor * ((((locals.var_qgbo_dn8 - locals.var_qovd_dn8) - locals.var_qovs_dn8) + locals.var_qgos_dn8) + locals.var_qgod_dn8)), (locals.var_mfactor * ((((locals.var_qgbo_dn9 - locals.var_qovd_dn9) - locals.var_qovs_dn9) + locals.var_qgos_dn9) + locals.var_qgod_dn9)), (locals.var_mfactor * ((((-locals.var_qovd_dn10) - locals.var_qovs_dn10) + locals.var_qgos_dn10) + locals.var_qgod_dn10)), (locals.var_mfactor * ((((-locals.var_qovd_dn11) - locals.var_qovs_dn11) + locals.var_qgos_dn11) + locals.var_qgod_dn11)), (locals.var_mfactor * ((((-locals.var_qovd_dn14) - locals.var_qovs_dn14) + locals.var_qgos_dn14) + locals.var_qgod_dn14)),)
    } else {
        (locals.var_qgov, locals.var_qgov_dn0, locals.var_qgov_dn2, locals.var_qgov_dn4, locals.var_qgov_dn5, locals.var_qgov_dn6, locals.var_qgov_dn7, locals.var_qgov_dn8, locals.var_qgov_dn9, locals.var_qgov_dn10, locals.var_qgov_dn11, locals.var_qgov_dn14,)
    }
};
        locals.var_qgov = assign102090_e154053;
        locals.var_qgov_dn0 = assign102090_e154053_d_n0;
        locals.var_qgov_dn2 = assign102090_e154053_d_n2;
        locals.var_qgov_dn4 = assign102090_e154053_d_n4;
        locals.var_qgov_dn5 = assign102090_e154053_d_n5;
        locals.var_qgov_dn6 = assign102090_e154053_d_n6;
        locals.var_qgov_dn7 = assign102090_e154053_d_n7;
        locals.var_qgov_dn8 = assign102090_e154053_d_n8;
        locals.var_qgov_dn9 = assign102090_e154053_d_n9;
        locals.var_qgov_dn10 = assign102090_e154053_d_n10;
        locals.var_qgov_dn11 = assign102090_e154053_d_n11;
        locals.var_qgov_dn14 = assign102090_e154053_d_n14;

        let (assign102100_e154062, assign102100_e154062_d_n0, assign102100_e154062_d_n2, assign102100_e154062_d_n4, assign102100_e154062_d_n5, assign102100_e154062_d_n6, assign102100_e154062_d_n7, assign102100_e154062_d_n8, assign102100_e154062_d_n9, assign102100_e154062_d_n10, assign102100_e154062_d_n11, assign102100_e154062_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102100_e154057: f64 = locals.var_qbdld;
        let assign102100_e154059: f64 = (assign102100_e154057 - locals.var_qgod);
        let assign102100_e154060: f64 = (locals.var_mfactor * assign102100_e154059);
        (assign102100_e154060, (locals.var_mfactor * (locals.var_qbdld_dn0 - locals.var_qgod_dn0)), (locals.var_mfactor * (locals.var_qbdld_dn2 - locals.var_qgod_dn2)), (locals.var_mfactor * (locals.var_qbdld_dn4 - locals.var_qgod_dn4)), (locals.var_mfactor * (locals.var_qbdld_dn5 - locals.var_qgod_dn5)), (locals.var_mfactor * (locals.var_qbdld_dn6 - locals.var_qgod_dn6)), (locals.var_mfactor * (locals.var_qbdld_dn7 - locals.var_qgod_dn7)), (locals.var_mfactor * (locals.var_qbdld_dn8 - locals.var_qgod_dn8)), (locals.var_mfactor * (locals.var_qbdld_dn9 - locals.var_qgod_dn9)), (locals.var_mfactor * (locals.var_qbdld_dn10 - locals.var_qgod_dn10)), (locals.var_mfactor * (locals.var_qbdld_dn11 - locals.var_qgod_dn11)), (locals.var_mfactor * (locals.var_qbdld_dn14 - locals.var_qgod_dn14)),)
    } else {
        (locals.var_qdov, locals.var_qdov_dn0, locals.var_qdov_dn2, locals.var_qdov_dn4, locals.var_qdov_dn5, locals.var_qdov_dn6, locals.var_qdov_dn7, locals.var_qdov_dn8, locals.var_qdov_dn9, locals.var_qdov_dn10, locals.var_qdov_dn11, locals.var_qdov_dn14,)
    }
};
        locals.var_qdov = assign102100_e154062;
        locals.var_qdov_dn0 = assign102100_e154062_d_n0;
        locals.var_qdov_dn2 = assign102100_e154062_d_n2;
        locals.var_qdov_dn4 = assign102100_e154062_d_n4;
        locals.var_qdov_dn5 = assign102100_e154062_d_n5;
        locals.var_qdov_dn6 = assign102100_e154062_d_n6;
        locals.var_qdov_dn7 = assign102100_e154062_d_n7;
        locals.var_qdov_dn8 = assign102100_e154062_d_n8;
        locals.var_qdov_dn9 = assign102100_e154062_d_n9;
        locals.var_qdov_dn10 = assign102100_e154062_d_n10;
        locals.var_qdov_dn11 = assign102100_e154062_d_n11;
        locals.var_qdov_dn14 = assign102100_e154062_d_n14;

        let (assign102110_e154071, assign102110_e154071_d_n0, assign102110_e154071_d_n2, assign102110_e154071_d_n4, assign102110_e154071_d_n5, assign102110_e154071_d_n6, assign102110_e154071_d_n7, assign102110_e154071_d_n8, assign102110_e154071_d_n9, assign102110_e154071_d_n10, assign102110_e154071_d_n11, assign102110_e154071_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102110_e154066: f64 = locals.var_qbsld;
        let assign102110_e154068: f64 = (assign102110_e154066 - locals.var_qgos);
        let assign102110_e154069: f64 = (locals.var_mfactor * assign102110_e154068);
        (assign102110_e154069, (locals.var_mfactor * (locals.var_qbsld_dn0 - locals.var_qgos_dn0)), (locals.var_mfactor * (locals.var_qbsld_dn2 - locals.var_qgos_dn2)), (locals.var_mfactor * (locals.var_qbsld_dn4 - locals.var_qgos_dn4)), (locals.var_mfactor * (locals.var_qbsld_dn5 - locals.var_qgos_dn5)), (locals.var_mfactor * (locals.var_qbsld_dn6 - locals.var_qgos_dn6)), (locals.var_mfactor * (locals.var_qbsld_dn7 - locals.var_qgos_dn7)), (locals.var_mfactor * (locals.var_qbsld_dn8 - locals.var_qgos_dn8)), (locals.var_mfactor * (locals.var_qbsld_dn9 - locals.var_qgos_dn9)), (locals.var_mfactor * (locals.var_qbsld_dn10 - locals.var_qgos_dn10)), (locals.var_mfactor * (locals.var_qbsld_dn11 - locals.var_qgos_dn11)), (locals.var_mfactor * (locals.var_qbsld_dn14 - locals.var_qgos_dn14)),)
    } else {
        (locals.var_qsov, locals.var_qsov_dn0, locals.var_qsov_dn2, locals.var_qsov_dn4, locals.var_qsov_dn5, locals.var_qsov_dn6, locals.var_qsov_dn7, locals.var_qsov_dn8, locals.var_qsov_dn9, locals.var_qsov_dn10, locals.var_qsov_dn11, locals.var_qsov_dn14,)
    }
};
        locals.var_qsov = assign102110_e154071;
        locals.var_qsov_dn0 = assign102110_e154071_d_n0;
        locals.var_qsov_dn2 = assign102110_e154071_d_n2;
        locals.var_qsov_dn4 = assign102110_e154071_d_n4;
        locals.var_qsov_dn5 = assign102110_e154071_d_n5;
        locals.var_qsov_dn6 = assign102110_e154071_d_n6;
        locals.var_qsov_dn7 = assign102110_e154071_d_n7;
        locals.var_qsov_dn8 = assign102110_e154071_d_n8;
        locals.var_qsov_dn9 = assign102110_e154071_d_n9;
        locals.var_qsov_dn10 = assign102110_e154071_d_n10;
        locals.var_qsov_dn11 = assign102110_e154071_d_n11;
        locals.var_qsov_dn14 = assign102110_e154071_d_n14;

        let (assign102120_e154084, assign102120_e154084_d_n0, assign102120_e154084_d_n2, assign102120_e154084_d_n4, assign102120_e154084_d_n5, assign102120_e154084_d_n6, assign102120_e154084_d_n7, assign102120_e154084_d_n8, assign102120_e154084_d_n9, assign102120_e154084_d_n10, assign102120_e154084_d_n11, assign102120_e154084_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102120_e154076: f64 = locals.var_qy;
        let assign102120_e154078: f64 = (assign102120_e154076 - locals.var_qovd_add);
        let assign102120_e154080: f64 = (assign102120_e154078 - locals.var_qovs_add);
        let assign102120_e154081: f64 = (locals.var_mfactor * assign102120_e154080);
        let assign102120_e154082: f64 = (locals.var_qge + assign102120_e154081);
        (assign102120_e154082, (locals.var_qge_dn0 + (locals.var_mfactor * ((locals.var_qy_dn0 - locals.var_qovd_add_dn0) - locals.var_qovs_add_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((locals.var_qy_dn2 - locals.var_qovd_add_dn2) - locals.var_qovs_add_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((locals.var_qy_dn4 - locals.var_qovd_add_dn4) - locals.var_qovs_add_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * ((locals.var_qy_dn5 - locals.var_qovd_add_dn5) - locals.var_qovs_add_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * ((locals.var_qy_dn6 - locals.var_qovd_add_dn6) - locals.var_qovs_add_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((locals.var_qy_dn7 - locals.var_qovd_add_dn7) - locals.var_qovs_add_dn7))), (locals.var_qge_dn8 + (locals.var_mfactor * ((locals.var_qy_dn8 - locals.var_qovd_add_dn8) - locals.var_qovs_add_dn8))), (locals.var_qge_dn9 + (locals.var_mfactor * ((locals.var_qy_dn9 - locals.var_qovd_add_dn9) - locals.var_qovs_add_dn9))), (locals.var_qge_dn10 + (locals.var_mfactor * ((locals.var_qy_dn10 - locals.var_qovd_add_dn10) - locals.var_qovs_add_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((locals.var_qy_dn11 - locals.var_qovd_add_dn11) - locals.var_qovs_add_dn11))), (locals.var_qge_dn14 + (locals.var_mfactor * ((locals.var_qy_dn14 - locals.var_qovd_add_dn14) - locals.var_qovs_add_dn14))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn14,)
    }
};
        locals.var_qge = assign102120_e154084;
        locals.var_qge_dn0 = assign102120_e154084_d_n0;
        locals.var_qge_dn2 = assign102120_e154084_d_n2;
        locals.var_qge_dn4 = assign102120_e154084_d_n4;
        locals.var_qge_dn5 = assign102120_e154084_d_n5;
        locals.var_qge_dn6 = assign102120_e154084_d_n6;
        locals.var_qge_dn7 = assign102120_e154084_d_n7;
        locals.var_qge_dn8 = assign102120_e154084_d_n8;
        locals.var_qge_dn9 = assign102120_e154084_d_n9;
        locals.var_qge_dn10 = assign102120_e154084_d_n10;
        locals.var_qge_dn11 = assign102120_e154084_d_n11;
        locals.var_qge_dn14 = assign102120_e154084_d_n14;

        let (assign102130_e154095, assign102130_e154095_d_n0, assign102130_e154095_d_n2, assign102130_e154095_d_n4, assign102130_e154095_d_n5, assign102130_e154095_d_n6, assign102130_e154095_d_n7, assign102130_e154095_d_n8, assign102130_e154095_d_n9, assign102130_e154095_d_n10, assign102130_e154095_d_n11, assign102130_e154095_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102130_e154089: f64 = (-locals.var_qy);
        let assign102130_e154091: f64 = (assign102130_e154089 + locals.var_qbdld_add);
        let assign102130_e154092: f64 = (locals.var_mfactor * assign102130_e154091);
        let assign102130_e154093: f64 = (locals.var_qde + assign102130_e154092);
        (assign102130_e154093, (locals.var_qde_dn0 + (locals.var_mfactor * ((-locals.var_qy_dn0) + locals.var_qbdld_add_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((-locals.var_qy_dn2) + locals.var_qbdld_add_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * ((-locals.var_qy_dn4) + locals.var_qbdld_add_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * ((-locals.var_qy_dn5) + locals.var_qbdld_add_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * ((-locals.var_qy_dn6) + locals.var_qbdld_add_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((-locals.var_qy_dn7) + locals.var_qbdld_add_dn7))), (locals.var_qde_dn8 + (locals.var_mfactor * ((-locals.var_qy_dn8) + locals.var_qbdld_add_dn8))), (locals.var_qde_dn9 + (locals.var_mfactor * ((-locals.var_qy_dn9) + locals.var_qbdld_add_dn9))), (locals.var_qde_dn10 + (locals.var_mfactor * ((-locals.var_qy_dn10) + locals.var_qbdld_add_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((-locals.var_qy_dn11) + locals.var_qbdld_add_dn11))), (locals.var_qde_dn14 + (locals.var_mfactor * ((-locals.var_qy_dn14) + locals.var_qbdld_add_dn14))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14,)
    }
};
        locals.var_qde = assign102130_e154095;
        locals.var_qde_dn0 = assign102130_e154095_d_n0;
        locals.var_qde_dn2 = assign102130_e154095_d_n2;
        locals.var_qde_dn4 = assign102130_e154095_d_n4;
        locals.var_qde_dn5 = assign102130_e154095_d_n5;
        locals.var_qde_dn6 = assign102130_e154095_d_n6;
        locals.var_qde_dn7 = assign102130_e154095_d_n7;
        locals.var_qde_dn8 = assign102130_e154095_d_n8;
        locals.var_qde_dn9 = assign102130_e154095_d_n9;
        locals.var_qde_dn10 = assign102130_e154095_d_n10;
        locals.var_qde_dn11 = assign102130_e154095_d_n11;
        locals.var_qde_dn14 = assign102130_e154095_d_n14;

        let (assign102140_e154104, assign102140_e154104_d_n0, assign102140_e154104_d_n2, assign102140_e154104_d_n4, assign102140_e154104_d_n5, assign102140_e154104_d_n6, assign102140_e154104_d_n7, assign102140_e154104_d_n8, assign102140_e154104_d_n9, assign102140_e154104_d_n10, assign102140_e154104_d_n11, assign102140_e154104_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102140_e154100: f64 = locals.var_qbsld_add;
        let assign102140_e154101: f64 = (locals.var_mfactor * assign102140_e154100);
        let assign102140_e154102: f64 = (locals.var_qse + assign102140_e154101);
        (assign102140_e154102, (locals.var_qse_dn0 + (locals.var_mfactor * locals.var_qbsld_add_dn0)), (locals.var_qse_dn2 + (locals.var_mfactor * locals.var_qbsld_add_dn2)), (locals.var_qse_dn4 + (locals.var_mfactor * locals.var_qbsld_add_dn4)), (locals.var_qse_dn5 + (locals.var_mfactor * locals.var_qbsld_add_dn5)), (locals.var_qse_dn6 + (locals.var_mfactor * locals.var_qbsld_add_dn6)), (locals.var_qse_dn7 + (locals.var_mfactor * locals.var_qbsld_add_dn7)), (locals.var_qse_dn8 + (locals.var_mfactor * locals.var_qbsld_add_dn8)), (locals.var_qse_dn9 + (locals.var_mfactor * locals.var_qbsld_add_dn9)), (locals.var_qse_dn10 + (locals.var_mfactor * locals.var_qbsld_add_dn10)), (locals.var_qse_dn11 + (locals.var_mfactor * locals.var_qbsld_add_dn11)), (locals.var_qse_dn14 + (locals.var_mfactor * locals.var_qbsld_add_dn14)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn14,)
    }
};
        locals.var_qse = assign102140_e154104;
        locals.var_qse_dn0 = assign102140_e154104_d_n0;
        locals.var_qse_dn2 = assign102140_e154104_d_n2;
        locals.var_qse_dn4 = assign102140_e154104_d_n4;
        locals.var_qse_dn5 = assign102140_e154104_d_n5;
        locals.var_qse_dn6 = assign102140_e154104_d_n6;
        locals.var_qse_dn7 = assign102140_e154104_d_n7;
        locals.var_qse_dn8 = assign102140_e154104_d_n8;
        locals.var_qse_dn9 = assign102140_e154104_d_n9;
        locals.var_qse_dn10 = assign102140_e154104_d_n10;
        locals.var_qse_dn11 = assign102140_e154104_d_n11;
        locals.var_qse_dn14 = assign102140_e154104_d_n14;

        let (assign102150_e154113, assign102150_e154113_d_n0, assign102150_e154113_d_n2, assign102150_e154113_d_n4, assign102150_e154113_d_n5, assign102150_e154113_d_n6, assign102150_e154113_d_n7, assign102150_e154113_d_n8, assign102150_e154113_d_n9, assign102150_e154113_d_n10, assign102150_e154113_d_n11, assign102150_e154113_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102150_e154108: f64 = (-locals.var_qovdext);
        let assign102150_e154110: f64 = (assign102150_e154108 - locals.var_qovsext);
        let assign102150_e154111: f64 = (locals.var_mfactor * assign102150_e154110);
        (assign102150_e154111, (locals.var_mfactor * ((-locals.var_qovdext_dn0) - locals.var_qovsext_dn0)), (locals.var_mfactor * ((-locals.var_qovdext_dn2) - locals.var_qovsext_dn2)), (locals.var_mfactor * ((-locals.var_qovdext_dn4) - locals.var_qovsext_dn4)), (locals.var_mfactor * ((-locals.var_qovdext_dn5) - locals.var_qovsext_dn5)), (locals.var_mfactor * ((-locals.var_qovdext_dn6) - locals.var_qovsext_dn6)), (locals.var_mfactor * ((-locals.var_qovdext_dn7) - locals.var_qovsext_dn7)), (locals.var_mfactor * ((-locals.var_qovdext_dn8) - locals.var_qovsext_dn8)), (locals.var_mfactor * ((-locals.var_qovdext_dn9) - locals.var_qovsext_dn9)), (locals.var_mfactor * ((-locals.var_qovdext_dn10) - locals.var_qovsext_dn10)), (locals.var_mfactor * ((-locals.var_qovdext_dn11) - locals.var_qovsext_dn11)), (locals.var_mfactor * ((-locals.var_qovdext_dn14) - locals.var_qovsext_dn14)),)
    } else {
        (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14,)
    }
};
        locals.var_qgexte = assign102150_e154113;
        locals.var_qgexte_dn0 = assign102150_e154113_d_n0;
        locals.var_qgexte_dn2 = assign102150_e154113_d_n2;
        locals.var_qgexte_dn4 = assign102150_e154113_d_n4;
        locals.var_qgexte_dn5 = assign102150_e154113_d_n5;
        locals.var_qgexte_dn6 = assign102150_e154113_d_n6;
        locals.var_qgexte_dn7 = assign102150_e154113_d_n7;
        locals.var_qgexte_dn8 = assign102150_e154113_d_n8;
        locals.var_qgexte_dn9 = assign102150_e154113_d_n9;
        locals.var_qgexte_dn10 = assign102150_e154113_d_n10;
        locals.var_qgexte_dn11 = assign102150_e154113_d_n11;
        locals.var_qgexte_dn14 = assign102150_e154113_d_n14;

        let (assign102160_e154119, assign102160_e154119_d_n0, assign102160_e154119_d_n2, assign102160_e154119_d_n4, assign102160_e154119_d_n5, assign102160_e154119_d_n6, assign102160_e154119_d_n7, assign102160_e154119_d_n8, assign102160_e154119_d_n9, assign102160_e154119_d_n10, assign102160_e154119_d_n11, assign102160_e154119_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102160_e154117: f64 = (locals.var_mfactor * locals.var_qbdldext);
        (assign102160_e154117, (locals.var_mfactor * locals.var_qbdldext_dn0), (locals.var_mfactor * locals.var_qbdldext_dn2), (locals.var_mfactor * locals.var_qbdldext_dn4), (locals.var_mfactor * locals.var_qbdldext_dn5), (locals.var_mfactor * locals.var_qbdldext_dn6), (locals.var_mfactor * locals.var_qbdldext_dn7), (locals.var_mfactor * locals.var_qbdldext_dn8), (locals.var_mfactor * locals.var_qbdldext_dn9), (locals.var_mfactor * locals.var_qbdldext_dn10), (locals.var_mfactor * locals.var_qbdldext_dn11), (locals.var_mfactor * locals.var_qbdldext_dn14),)
    } else {
        (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14,)
    }
};
        locals.var_qdexte = assign102160_e154119;
        locals.var_qdexte_dn0 = assign102160_e154119_d_n0;
        locals.var_qdexte_dn2 = assign102160_e154119_d_n2;
        locals.var_qdexte_dn4 = assign102160_e154119_d_n4;
        locals.var_qdexte_dn5 = assign102160_e154119_d_n5;
        locals.var_qdexte_dn6 = assign102160_e154119_d_n6;
        locals.var_qdexte_dn7 = assign102160_e154119_d_n7;
        locals.var_qdexte_dn8 = assign102160_e154119_d_n8;
        locals.var_qdexte_dn9 = assign102160_e154119_d_n9;
        locals.var_qdexte_dn10 = assign102160_e154119_d_n10;
        locals.var_qdexte_dn11 = assign102160_e154119_d_n11;
        locals.var_qdexte_dn14 = assign102160_e154119_d_n14;

        let (assign102170_e154125, assign102170_e154125_d_n0, assign102170_e154125_d_n2, assign102170_e154125_d_n4, assign102170_e154125_d_n5, assign102170_e154125_d_n6, assign102170_e154125_d_n7, assign102170_e154125_d_n8, assign102170_e154125_d_n9, assign102170_e154125_d_n10, assign102170_e154125_d_n11, assign102170_e154125_d_n14,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102170_e154123: f64 = (locals.var_mfactor * locals.var_qbsldext);
        (assign102170_e154123, (locals.var_mfactor * locals.var_qbsldext_dn0), (locals.var_mfactor * locals.var_qbsldext_dn2), (locals.var_mfactor * locals.var_qbsldext_dn4), (locals.var_mfactor * locals.var_qbsldext_dn5), (locals.var_mfactor * locals.var_qbsldext_dn6), (locals.var_mfactor * locals.var_qbsldext_dn7), (locals.var_mfactor * locals.var_qbsldext_dn8), (locals.var_mfactor * locals.var_qbsldext_dn9), (locals.var_mfactor * locals.var_qbsldext_dn10), (locals.var_mfactor * locals.var_qbsldext_dn11), (locals.var_mfactor * locals.var_qbsldext_dn14),)
    } else {
        (locals.var_qsexte, locals.var_qsexte_dn0, locals.var_qsexte_dn2, locals.var_qsexte_dn4, locals.var_qsexte_dn5, locals.var_qsexte_dn6, locals.var_qsexte_dn7, locals.var_qsexte_dn8, locals.var_qsexte_dn9, locals.var_qsexte_dn10, locals.var_qsexte_dn11, locals.var_qsexte_dn14,)
    }
};
        locals.var_qsexte = assign102170_e154125;
        locals.var_qsexte_dn0 = assign102170_e154125_d_n0;
        locals.var_qsexte_dn2 = assign102170_e154125_d_n2;
        locals.var_qsexte_dn4 = assign102170_e154125_d_n4;
        locals.var_qsexte_dn5 = assign102170_e154125_d_n5;
        locals.var_qsexte_dn6 = assign102170_e154125_d_n6;
        locals.var_qsexte_dn7 = assign102170_e154125_d_n7;
        locals.var_qsexte_dn8 = assign102170_e154125_d_n8;
        locals.var_qsexte_dn9 = assign102170_e154125_d_n9;
        locals.var_qsexte_dn10 = assign102170_e154125_d_n10;
        locals.var_qsexte_dn11 = assign102170_e154125_d_n11;
        locals.var_qsexte_dn14 = assign102170_e154125_d_n14;

        let (assign102180_e154136, assign102180_e154136_d_n0, assign102180_e154136_d_n2, assign102180_e154136_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102180_e154130: f64 = (-locals.var_qfd);
        let assign102180_e154132: f64 = (assign102180_e154130 - locals.var_qgdo);
        let assign102180_e154133: f64 = (locals.var_mfactor * assign102180_e154132);
        let assign102180_e154134: f64 = (locals.var_qdp + assign102180_e154133);
        (assign102180_e154134, (locals.var_qdp_dn0 + (locals.var_mfactor * ((-locals.var_qfd_dn0) - locals.var_qgdo_dn0))), (locals.var_qdp_dn2 + (locals.var_mfactor * ((-locals.var_qfd_dn2) - locals.var_qgdo_dn2))), (locals.var_qdp_dn7 + (locals.var_mfactor * ((-locals.var_qfd_dn7) - locals.var_qgdo_dn7))),)
    } else {
        (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7,)
    }
};
        locals.var_qdp = assign102180_e154136;
        locals.var_qdp_dn0 = assign102180_e154136_d_n0;
        locals.var_qdp_dn2 = assign102180_e154136_d_n2;
        locals.var_qdp_dn7 = assign102180_e154136_d_n7;

    }

    pub(super) fn stamp_transient_block_376(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign102190_e154147, assign102190_e154147_d_n2, assign102190_e154147_d_n7,) = {
    if (locals.var_guard2335 != 0.0) {
        let assign102190_e154141: f64 = (-locals.var_qfs);
        let assign102190_e154143: f64 = (assign102190_e154141 - locals.var_qgso);
        let assign102190_e154144: f64 = (locals.var_mfactor * assign102190_e154143);
        let assign102190_e154145: f64 = (locals.var_qsp + assign102190_e154144);
        (assign102190_e154145, (locals.var_qsp_dn2 + (locals.var_mfactor * ((-locals.var_qfs_dn2) - locals.var_qgso_dn2))), (locals.var_qsp_dn7 + (locals.var_mfactor * ((-locals.var_qfs_dn7) - locals.var_qgso_dn7))),)
    } else {
        (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7,)
    }
};
        locals.var_qsp = assign102190_e154147;
        locals.var_qsp_dn2 = assign102190_e154147_d_n2;
        locals.var_qsp_dn7 = assign102190_e154147_d_n7;

        let assign102200_e154151: f64 = (locals.var_isub + locals.var_isubibpc);
        let assign102200_e154152: f64 = (locals.var_mfactor * assign102200_e154151);
        locals.var_isube = assign102200_e154152;
        locals.var_isube_dn0 = (locals.var_mfactor * (locals.var_isub_dn0 + locals.var_isubibpc_dn0));
        locals.var_isube_dn2 = (locals.var_mfactor * (locals.var_isub_dn2 + locals.var_isubibpc_dn2));
        locals.var_isube_dn4 = (locals.var_mfactor * (locals.var_isub_dn4 + locals.var_isubibpc_dn4));
        locals.var_isube_dn5 = (locals.var_mfactor * (locals.var_isub_dn5 + locals.var_isubibpc_dn5));
        locals.var_isube_dn6 = (locals.var_mfactor * (locals.var_isub_dn6 + locals.var_isubibpc_dn6));
        locals.var_isube_dn7 = (locals.var_mfactor * (locals.var_isub_dn7 + locals.var_isubibpc_dn7));
        locals.var_isube_dn8 = (locals.var_mfactor * (locals.var_isub_dn8 + locals.var_isubibpc_dn8));
        locals.var_isube_dn9 = (locals.var_mfactor * (locals.var_isub_dn9 + locals.var_isubibpc_dn9));
        locals.var_isube_dn10 = (locals.var_mfactor * (locals.var_isub_dn10 + locals.var_isubibpc_dn10));
        locals.var_isube_dn11 = (locals.var_mfactor * (locals.var_isub_dn11 + locals.var_isubibpc_dn11));
        locals.var_isube_dn14 = (locals.var_mfactor * (locals.var_isub_dn14 + locals.var_isubibpc_dn14));

        let assign102210_e154155: f64 = (locals.var_mfactor * locals.var_isubld);
        locals.var_isublde = assign102210_e154155;
        locals.var_isublde_dn0 = (locals.var_mfactor * locals.var_isubld_dn0);
        locals.var_isublde_dn2 = (locals.var_mfactor * locals.var_isubld_dn2);
        locals.var_isublde_dn4 = (locals.var_mfactor * locals.var_isubld_dn4);
        locals.var_isublde_dn5 = (locals.var_mfactor * locals.var_isubld_dn5);
        locals.var_isublde_dn6 = (locals.var_mfactor * locals.var_isubld_dn6);
        locals.var_isublde_dn7 = (locals.var_mfactor * locals.var_isubld_dn7);
        locals.var_isublde_dn8 = (locals.var_mfactor * locals.var_isubld_dn8);
        locals.var_isublde_dn9 = (locals.var_mfactor * locals.var_isubld_dn9);
        locals.var_isublde_dn10 = (locals.var_mfactor * locals.var_isubld_dn10);
        locals.var_isublde_dn11 = (locals.var_mfactor * locals.var_isubld_dn11);
        locals.var_isublde_dn14 = (locals.var_mfactor * locals.var_isubld_dn14);

        let assign102220_e154158: f64 = (-locals.var_igb);
        let assign102220_e154159: f64 = (locals.var_mfactor * assign102220_e154158);
        locals.var_igbe = assign102220_e154159;
        locals.var_igbe_dn0 = (locals.var_mfactor * (-locals.var_igb_dn0));
        locals.var_igbe_dn2 = (locals.var_mfactor * (-locals.var_igb_dn2));
        locals.var_igbe_dn4 = (locals.var_mfactor * (-locals.var_igb_dn4));
        locals.var_igbe_dn5 = (locals.var_mfactor * (-locals.var_igb_dn5));
        locals.var_igbe_dn6 = (locals.var_mfactor * (-locals.var_igb_dn6));
        locals.var_igbe_dn7 = (locals.var_mfactor * (-locals.var_igb_dn7));
        locals.var_igbe_dn8 = (locals.var_mfactor * (-locals.var_igb_dn8));
        locals.var_igbe_dn9 = (locals.var_mfactor * (-locals.var_igb_dn9));
        locals.var_igbe_dn10 = (locals.var_mfactor * (-locals.var_igb_dn10));
        locals.var_igbe_dn11 = (locals.var_mfactor * (-locals.var_igb_dn11));
        locals.var_igbe_dn14 = (locals.var_mfactor * (-locals.var_igb_dn14));

        let assign102230_e154162: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2336 = assign102230_e154162;

        let (assign102240_e154172, assign102240_e154172_d_n0, assign102240_e154172_d_n2, assign102240_e154172_d_n4, assign102240_e154172_d_n5, assign102240_e154172_d_n6, assign102240_e154172_d_n7, assign102240_e154172_d_n8, assign102240_e154172_d_n9, assign102240_e154172_d_n10, assign102240_e154172_d_n11, assign102240_e154172_d_n14,) = {
    if (locals.var_guard2336 != 0.0) {
        let assign102240_e154167: f64 = (p.p252 * locals.var_igate);
        let assign102240_e154169: f64 = (assign102240_e154167 - locals.var_igd);
        let assign102240_e154170: f64 = (locals.var_mfactor * assign102240_e154169);
        (assign102240_e154170, (locals.var_mfactor * ((p.p252 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn4) - locals.var_igd_dn4)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn5) - locals.var_igd_dn5)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn8) - locals.var_igd_dn8)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn9) - locals.var_igd_dn9)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn14) - locals.var_igd_dn14)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn4, locals.var_igde_dn5, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn8, locals.var_igde_dn9, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn14,)
    }
};
        locals.var_igde = assign102240_e154172;
        locals.var_igde_dn0 = assign102240_e154172_d_n0;
        locals.var_igde_dn2 = assign102240_e154172_d_n2;
        locals.var_igde_dn4 = assign102240_e154172_d_n4;
        locals.var_igde_dn5 = assign102240_e154172_d_n5;
        locals.var_igde_dn6 = assign102240_e154172_d_n6;
        locals.var_igde_dn7 = assign102240_e154172_d_n7;
        locals.var_igde_dn8 = assign102240_e154172_d_n8;
        locals.var_igde_dn9 = assign102240_e154172_d_n9;
        locals.var_igde_dn10 = assign102240_e154172_d_n10;
        locals.var_igde_dn11 = assign102240_e154172_d_n11;
        locals.var_igde_dn14 = assign102240_e154172_d_n14;

        let (assign102250_e154185, assign102250_e154185_d_n0, assign102250_e154185_d_n2, assign102250_e154185_d_n4, assign102250_e154185_d_n5, assign102250_e154185_d_n6, assign102250_e154185_d_n7, assign102250_e154185_d_n8, assign102250_e154185_d_n9, assign102250_e154185_d_n10, assign102250_e154185_d_n11, assign102250_e154185_d_n14,) = {
    if (locals.var_guard2336 == 0.0) {
        let assign102250_e154178: f64 = (1.0 - p.p252);
        let assign102250_e154180: f64 = (assign102250_e154178 * locals.var_igate);
        let assign102250_e154182: f64 = (assign102250_e154180 - locals.var_igs);
        let assign102250_e154183: f64 = (locals.var_mfactor * assign102250_e154182);
        (assign102250_e154183, (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn4) - locals.var_igs_dn4)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn5) - locals.var_igs_dn5)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn8) - locals.var_igs_dn8)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn9) - locals.var_igs_dn9)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign102250_e154178 * locals.var_igate_dn14) - locals.var_igs_dn14)),)
    } else {
        (locals.var_igde, locals.var_igde_dn0, locals.var_igde_dn2, locals.var_igde_dn4, locals.var_igde_dn5, locals.var_igde_dn6, locals.var_igde_dn7, locals.var_igde_dn8, locals.var_igde_dn9, locals.var_igde_dn10, locals.var_igde_dn11, locals.var_igde_dn14,)
    }
};
        locals.var_igde = assign102250_e154185;
        locals.var_igde_dn0 = assign102250_e154185_d_n0;
        locals.var_igde_dn2 = assign102250_e154185_d_n2;
        locals.var_igde_dn4 = assign102250_e154185_d_n4;
        locals.var_igde_dn5 = assign102250_e154185_d_n5;
        locals.var_igde_dn6 = assign102250_e154185_d_n6;
        locals.var_igde_dn7 = assign102250_e154185_d_n7;
        locals.var_igde_dn8 = assign102250_e154185_d_n8;
        locals.var_igde_dn9 = assign102250_e154185_d_n9;
        locals.var_igde_dn10 = assign102250_e154185_d_n10;
        locals.var_igde_dn11 = assign102250_e154185_d_n11;
        locals.var_igde_dn14 = assign102250_e154185_d_n14;

        let assign102260_e154188: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2337 = assign102260_e154188;

        let (assign102270_e154200, assign102270_e154200_d_n0, assign102270_e154200_d_n2, assign102270_e154200_d_n4, assign102270_e154200_d_n5, assign102270_e154200_d_n6, assign102270_e154200_d_n7, assign102270_e154200_d_n8, assign102270_e154200_d_n9, assign102270_e154200_d_n10, assign102270_e154200_d_n11, assign102270_e154200_d_n14,) = {
    if (locals.var_guard2337 != 0.0) {
        let assign102270_e154193: f64 = (1.0 - p.p252);
        let assign102270_e154195: f64 = (assign102270_e154193 * locals.var_igate);
        let assign102270_e154197: f64 = (assign102270_e154195 - locals.var_igs);
        let assign102270_e154198: f64 = (locals.var_mfactor * assign102270_e154197);
        (assign102270_e154198, (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn0) - locals.var_igs_dn0)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn2) - locals.var_igs_dn2)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn4) - locals.var_igs_dn4)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn5) - locals.var_igs_dn5)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn6) - locals.var_igs_dn6)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn7) - locals.var_igs_dn7)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn8) - locals.var_igs_dn8)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn9) - locals.var_igs_dn9)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn10) - locals.var_igs_dn10)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn11) - locals.var_igs_dn11)), (locals.var_mfactor * ((assign102270_e154193 * locals.var_igate_dn14) - locals.var_igs_dn14)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn4, locals.var_igse_dn5, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn8, locals.var_igse_dn9, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn14,)
    }
};
        locals.var_igse = assign102270_e154200;
        locals.var_igse_dn0 = assign102270_e154200_d_n0;
        locals.var_igse_dn2 = assign102270_e154200_d_n2;
        locals.var_igse_dn4 = assign102270_e154200_d_n4;
        locals.var_igse_dn5 = assign102270_e154200_d_n5;
        locals.var_igse_dn6 = assign102270_e154200_d_n6;
        locals.var_igse_dn7 = assign102270_e154200_d_n7;
        locals.var_igse_dn8 = assign102270_e154200_d_n8;
        locals.var_igse_dn9 = assign102270_e154200_d_n9;
        locals.var_igse_dn10 = assign102270_e154200_d_n10;
        locals.var_igse_dn11 = assign102270_e154200_d_n11;
        locals.var_igse_dn14 = assign102270_e154200_d_n14;

        let (assign102280_e154211, assign102280_e154211_d_n0, assign102280_e154211_d_n2, assign102280_e154211_d_n4, assign102280_e154211_d_n5, assign102280_e154211_d_n6, assign102280_e154211_d_n7, assign102280_e154211_d_n8, assign102280_e154211_d_n9, assign102280_e154211_d_n10, assign102280_e154211_d_n11, assign102280_e154211_d_n14,) = {
    if (locals.var_guard2337 == 0.0) {
        let assign102280_e154206: f64 = (p.p252 * locals.var_igate);
        let assign102280_e154208: f64 = (assign102280_e154206 - locals.var_igd);
        let assign102280_e154209: f64 = (locals.var_mfactor * assign102280_e154208);
        (assign102280_e154209, (locals.var_mfactor * ((p.p252 * locals.var_igate_dn0) - locals.var_igd_dn0)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn2) - locals.var_igd_dn2)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn4) - locals.var_igd_dn4)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn5) - locals.var_igd_dn5)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn6) - locals.var_igd_dn6)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn7) - locals.var_igd_dn7)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn8) - locals.var_igd_dn8)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn9) - locals.var_igd_dn9)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn10) - locals.var_igd_dn10)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn11) - locals.var_igd_dn11)), (locals.var_mfactor * ((p.p252 * locals.var_igate_dn14) - locals.var_igd_dn14)),)
    } else {
        (locals.var_igse, locals.var_igse_dn0, locals.var_igse_dn2, locals.var_igse_dn4, locals.var_igse_dn5, locals.var_igse_dn6, locals.var_igse_dn7, locals.var_igse_dn8, locals.var_igse_dn9, locals.var_igse_dn10, locals.var_igse_dn11, locals.var_igse_dn14,)
    }
};
        locals.var_igse = assign102280_e154211;
        locals.var_igse_dn0 = assign102280_e154211_d_n0;
        locals.var_igse_dn2 = assign102280_e154211_d_n2;
        locals.var_igse_dn4 = assign102280_e154211_d_n4;
        locals.var_igse_dn5 = assign102280_e154211_d_n5;
        locals.var_igse_dn6 = assign102280_e154211_d_n6;
        locals.var_igse_dn7 = assign102280_e154211_d_n7;
        locals.var_igse_dn8 = assign102280_e154211_d_n8;
        locals.var_igse_dn9 = assign102280_e154211_d_n9;
        locals.var_igse_dn10 = assign102280_e154211_d_n10;
        locals.var_igse_dn11 = assign102280_e154211_d_n11;
        locals.var_igse_dn14 = assign102280_e154211_d_n14;

        let assign102290_e154214: f64 = (locals.var_mfactor * locals.var_igidl);
        locals.var_igidle = assign102290_e154214;
        locals.var_igidle_dn0 = (locals.var_mfactor * locals.var_igidl_dn0);
        locals.var_igidle_dn2 = (locals.var_mfactor * locals.var_igidl_dn2);
        locals.var_igidle_dn4 = (locals.var_mfactor * locals.var_igidl_dn4);
        locals.var_igidle_dn5 = (locals.var_mfactor * locals.var_igidl_dn5);
        locals.var_igidle_dn6 = (locals.var_mfactor * locals.var_igidl_dn6);
        locals.var_igidle_dn7 = (locals.var_mfactor * locals.var_igidl_dn7);
        locals.var_igidle_dn8 = (locals.var_mfactor * locals.var_igidl_dn8);
        locals.var_igidle_dn9 = (locals.var_mfactor * locals.var_igidl_dn9);
        locals.var_igidle_dn10 = (locals.var_mfactor * locals.var_igidl_dn10);
        locals.var_igidle_dn11 = (locals.var_mfactor * locals.var_igidl_dn11);
        locals.var_igidle_dn14 = (locals.var_mfactor * locals.var_igidl_dn14);

        let assign102300_e154217: f64 = (locals.var_mfactor * locals.var_igisl);
        locals.var_igisle = assign102300_e154217;
        locals.var_igisle_dn0 = (locals.var_mfactor * locals.var_igisl_dn0);
        locals.var_igisle_dn2 = (locals.var_mfactor * locals.var_igisl_dn2);
        locals.var_igisle_dn4 = (locals.var_mfactor * locals.var_igisl_dn4);
        locals.var_igisle_dn5 = (locals.var_mfactor * locals.var_igisl_dn5);
        locals.var_igisle_dn6 = (locals.var_mfactor * locals.var_igisl_dn6);
        locals.var_igisle_dn7 = (locals.var_mfactor * locals.var_igisl_dn7);
        locals.var_igisle_dn8 = (locals.var_mfactor * locals.var_igisl_dn8);
        locals.var_igisle_dn9 = (locals.var_mfactor * locals.var_igisl_dn9);
        locals.var_igisle_dn10 = (locals.var_mfactor * locals.var_igisl_dn10);
        locals.var_igisle_dn11 = (locals.var_mfactor * locals.var_igisl_dn11);
        locals.var_igisle_dn14 = (locals.var_mfactor * locals.var_igisl_dn14);

        let assign102330_e154222: f64 = (4.0 * 1.3806226e-23);
        let assign102330_e154224: f64 = (assign102330_e154222 * locals.var_ttemp);
        let assign102330_e154226: f64 = assign102330_e154224;
        locals.var_whi_noise = assign102330_e154226;
        locals.var_whi_noise_dn0 = (assign102330_e154222 * locals.var_ttemp_dn0);
        locals.var_whi_noise_dn2 = (assign102330_e154222 * locals.var_ttemp_dn2);
        locals.var_whi_noise_dn4 = (assign102330_e154222 * locals.var_ttemp_dn4);
        locals.var_whi_noise_dn5 = (assign102330_e154222 * locals.var_ttemp_dn5);
        locals.var_whi_noise_dn6 = (assign102330_e154222 * locals.var_ttemp_dn6);
        locals.var_whi_noise_dn7 = (assign102330_e154222 * locals.var_ttemp_dn7);
        locals.var_whi_noise_dn8 = (assign102330_e154222 * locals.var_ttemp_dn8);
        locals.var_whi_noise_dn9 = (assign102330_e154222 * locals.var_ttemp_dn9);
        locals.var_whi_noise_dn10 = (assign102330_e154222 * locals.var_ttemp_dn10);
        locals.var_whi_noise_dn11 = (assign102330_e154222 * locals.var_ttemp_dn11);
        locals.var_whi_noise_dn14 = (assign102330_e154222 * locals.var_ttemp_dn14);

        let assign102350_e154232: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign102350_e154232;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn7 = (locals.var_mfactor * locals.var_nthrml_dn7);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn9 = (locals.var_mfactor * locals.var_nthrml_dn9);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn14 = (locals.var_mfactor * locals.var_nthrml_dn14);

        let assign102360_e154235: f64 = locals.var_qge_dn6;
        locals.var_cgdbd = assign102360_e154235;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn14 = 0.0;

        let assign102370_e154238: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign102370_e154238;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p87 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn14 = (p.p87 * locals.var_cgdbd_dn14);

        let assign102380_e154241: f64 = locals.var_qge_dn8;
        locals.var_cgsbd = assign102380_e154241;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn14 = 0.0;

        let assign102390_e154244: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign102390_e154244;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p87 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn14 = (p.p87 * locals.var_cgsbd_dn14);

        let (assign102400_e154250, assign102400_e154250_d_n0, assign102400_e154250_d_n2, assign102400_e154250_d_n4, assign102400_e154250_d_n5, assign102400_e154250_d_n6, assign102400_e154250_d_n7, assign102400_e154250_d_n8, assign102400_e154250_d_n9, assign102400_e154250_d_n10, assign102400_e154250_d_n11, assign102400_e154250_d_n14,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14,)
    }
};
        locals.var_cgsb = assign102400_e154250;
        locals.var_cgsb_dn0 = assign102400_e154250_d_n0;
        locals.var_cgsb_dn2 = assign102400_e154250_d_n2;
        locals.var_cgsb_dn4 = assign102400_e154250_d_n4;
        locals.var_cgsb_dn5 = assign102400_e154250_d_n5;
        locals.var_cgsb_dn6 = assign102400_e154250_d_n6;
        locals.var_cgsb_dn7 = assign102400_e154250_d_n7;
        locals.var_cgsb_dn8 = assign102400_e154250_d_n8;
        locals.var_cgsb_dn9 = assign102400_e154250_d_n9;
        locals.var_cgsb_dn10 = assign102400_e154250_d_n10;
        locals.var_cgsb_dn11 = assign102400_e154250_d_n11;
        locals.var_cgsb_dn14 = assign102400_e154250_d_n14;

        locals.var_noiigate = 0.0;
        locals.var_noiigate_dn0 = 0.0;
        locals.var_noiigate_dn2 = 0.0;
        locals.var_noiigate_dn4 = 0.0;
        locals.var_noiigate_dn5 = 0.0;
        locals.var_noiigate_dn6 = 0.0;
        locals.var_noiigate_dn7 = 0.0;
        locals.var_noiigate_dn8 = 0.0;
        locals.var_noiigate_dn9 = 0.0;
        locals.var_noiigate_dn10 = 0.0;
        locals.var_noiigate_dn11 = 0.0;
        locals.var_noiigate_dn14 = 0.0;

        locals.var_noicross = 0.0;
        locals.var_noicross_dn0 = 0.0;
        locals.var_noicross_dn2 = 0.0;
        locals.var_noicross_dn4 = 0.0;
        locals.var_noicross_dn5 = 0.0;
        locals.var_noicross_dn6 = 0.0;
        locals.var_noicross_dn7 = 0.0;
        locals.var_noicross_dn8 = 0.0;
        locals.var_noicross_dn9 = 0.0;
        locals.var_noicross_dn10 = 0.0;
        locals.var_noicross_dn11 = 0.0;
        locals.var_noicross_dn14 = 0.0;

        let assign102430_e154270: f64 = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2338 = assign102430_e154270;

        let (assign102440_e154280, assign102440_e154280_d_n0, assign102440_e154280_d_n2, assign102440_e154280_d_n4, assign102440_e154280_d_n5, assign102440_e154280_d_n6, assign102440_e154280_d_n7, assign102440_e154280_d_n8, assign102440_e154280_d_n9, assign102440_e154280_d_n10, assign102440_e154280_d_n11, assign102440_e154280_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102440_e154274: f64 = (1e-6 * locals.var_cox);
        let assign102440_e154276: f64 = (assign102440_e154274 * locals.var_weffcv_nf);
        let assign102440_e154278: f64 = (assign102440_e154276 * locals.var_leff);
        (assign102440_e154278, (((1e-6 * locals.var_cox_dn0) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn2) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn4) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn5) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn6) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn7) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn8) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn9) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn10) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn11) * locals.var_weffcv_nf) * locals.var_leff), (((1e-6 * locals.var_cox_dn14) * locals.var_weffcv_nf) * locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102440_e154280;
        locals.var_t0_dn0 = assign102440_e154280_d_n0;
        locals.var_t0_dn2 = assign102440_e154280_d_n2;
        locals.var_t0_dn4 = assign102440_e154280_d_n4;
        locals.var_t0_dn5 = assign102440_e154280_d_n5;
        locals.var_t0_dn6 = assign102440_e154280_d_n6;
        locals.var_t0_dn7 = assign102440_e154280_d_n7;
        locals.var_t0_dn8 = assign102440_e154280_d_n8;
        locals.var_t0_dn9 = assign102440_e154280_d_n9;
        locals.var_t0_dn10 = assign102440_e154280_d_n10;
        locals.var_t0_dn11 = assign102440_e154280_d_n11;
        locals.var_t0_dn14 = assign102440_e154280_d_n14;

        let (assign102450_e154286, assign102450_e154286_d_n0, assign102450_e154286_d_n2, assign102450_e154286_d_n4, assign102450_e154286_d_n5, assign102450_e154286_d_n6, assign102450_e154286_d_n7, assign102450_e154286_d_n8, assign102450_e154286_d_n9, assign102450_e154286_d_n10, assign102450_e154286_d_n11, assign102450_e154286_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102450_e154284: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign102450_e154284, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn7 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn9 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign102450_e154286;
        locals.var_t10_dn0 = assign102450_e154286_d_n0;
        locals.var_t10_dn2 = assign102450_e154286_d_n2;
        locals.var_t10_dn4 = assign102450_e154286_d_n4;
        locals.var_t10_dn5 = assign102450_e154286_d_n5;
        locals.var_t10_dn6 = assign102450_e154286_d_n6;
        locals.var_t10_dn7 = assign102450_e154286_d_n7;
        locals.var_t10_dn8 = assign102450_e154286_d_n8;
        locals.var_t10_dn9 = assign102450_e154286_d_n9;
        locals.var_t10_dn10 = assign102450_e154286_d_n10;
        locals.var_t10_dn11 = assign102450_e154286_d_n11;
        locals.var_t10_dn14 = assign102450_e154286_d_n14;

        let (assign102460_e154300, assign102460_e154300_d_n0, assign102460_e154300_d_n2, assign102460_e154300_d_n4, assign102460_e154300_d_n5, assign102460_e154300_d_n6, assign102460_e154300_d_n7, assign102460_e154300_d_n8, assign102460_e154300_d_n9, assign102460_e154300_d_n10, assign102460_e154300_d_n11, assign102460_e154300_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102460_e154290: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign102460_e154292: f64 = (assign102460_e154290 * locals.var_beta_inv);
        let assign102460_e154294: f64 = (assign102460_e154292 * locals.var_t10);
        let assign102460_e154296: f64 = (assign102460_e154294 * locals.var_t10);
        let assign102460_e154298: f64 = (assign102460_e154296 / locals.var_gds0_ign);
        (assign102460_e154298, ((((((((assign102460_e154290 * locals.var_beta_inv_dn0) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn0)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn2) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn2)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn4)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn5) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn5)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn6) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn6)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn7) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn7)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn7)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn7)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn8) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn8)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn9) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn9)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn9)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn9)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn10) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn10)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn11) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn11)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign102460_e154290 * locals.var_beta_inv_dn14) * locals.var_t10) + (assign102460_e154292 * locals.var_t10_dn14)) * locals.var_t10) + (assign102460_e154294 * locals.var_t10_dn14)) * locals.var_gds0_ign) - (assign102460_e154296 * locals.var_gds0_ign_dn14)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn7, locals.var_nign0_dn8, locals.var_nign0_dn9, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn14,)
    }
};
        locals.var_nign0 = assign102460_e154300;
        locals.var_nign0_dn0 = assign102460_e154300_d_n0;
        locals.var_nign0_dn2 = assign102460_e154300_d_n2;
        locals.var_nign0_dn4 = assign102460_e154300_d_n4;
        locals.var_nign0_dn5 = assign102460_e154300_d_n5;
        locals.var_nign0_dn6 = assign102460_e154300_d_n6;
        locals.var_nign0_dn7 = assign102460_e154300_d_n7;
        locals.var_nign0_dn8 = assign102460_e154300_d_n8;
        locals.var_nign0_dn9 = assign102460_e154300_d_n9;
        locals.var_nign0_dn10 = assign102460_e154300_d_n10;
        locals.var_nign0_dn11 = assign102460_e154300_d_n11;
        locals.var_nign0_dn14 = assign102460_e154300_d_n14;

        let assign102470_e154304: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154309: f64 = (10.0 * 2.220446049250313e-16);
        let assign102470_e154311: f64 = if ((locals.var_kusai00l > assign102470_e154304) && (locals.var_vds > assign102470_e154309)) { 1.0 } else { 0.0 };
        locals.var_guard2339 = assign102470_e154311;

        let (assign102480_e154319, assign102480_e154319_d_n0, assign102480_e154319_d_n2, assign102480_e154319_d_n4, assign102480_e154319_d_n5, assign102480_e154319_d_n6, assign102480_e154319_d_n7, assign102480_e154319_d_n8, assign102480_e154319_d_n9, assign102480_e154319_d_n10, assign102480_e154319_d_n11, assign102480_e154319_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102480_e154317: f64 = (locals.var_muun / locals.var_mu);
        (assign102480_e154317, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn7 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn7)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn9 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn9)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn14 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn14)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn7, locals.var_mumoda_dn8, locals.var_mumoda_dn9, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn14,)
    }
};
        locals.var_mumoda = assign102480_e154319;
        locals.var_mumoda_dn0 = assign102480_e154319_d_n0;
        locals.var_mumoda_dn2 = assign102480_e154319_d_n2;
        locals.var_mumoda_dn4 = assign102480_e154319_d_n4;
        locals.var_mumoda_dn5 = assign102480_e154319_d_n5;
        locals.var_mumoda_dn6 = assign102480_e154319_d_n6;
        locals.var_mumoda_dn7 = assign102480_e154319_d_n7;
        locals.var_mumoda_dn8 = assign102480_e154319_d_n8;
        locals.var_mumoda_dn9 = assign102480_e154319_d_n9;
        locals.var_mumoda_dn10 = assign102480_e154319_d_n10;
        locals.var_mumoda_dn11 = assign102480_e154319_d_n11;
        locals.var_mumoda_dn14 = assign102480_e154319_d_n14;

        let (assign102490_e154331, assign102490_e154331_d_n0, assign102490_e154331_d_n2, assign102490_e154331_d_n4, assign102490_e154331_d_n5, assign102490_e154331_d_n6, assign102490_e154331_d_n7, assign102490_e154331_d_n8, assign102490_e154331_d_n9, assign102490_e154331_d_n10, assign102490_e154331_d_n11, assign102490_e154331_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102490_e154325: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign102490_e154327: f64 = (assign102490_e154325 - locals.var_mumoda);
        let assign102490_e154329: f64 = (assign102490_e154327 / locals.var_vds);
        (assign102490_e154329, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn7) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn7)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn9) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn9)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn14) * locals.var_vds) - (assign102490_e154327 * locals.var_vds_dn14)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn7, locals.var_mumodb_dn8, locals.var_mumodb_dn9, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn14,)
    }
};
        locals.var_mumodb = assign102490_e154331;
        locals.var_mumodb_dn0 = assign102490_e154331_d_n0;
        locals.var_mumodb_dn2 = assign102490_e154331_d_n2;
        locals.var_mumodb_dn4 = assign102490_e154331_d_n4;
        locals.var_mumodb_dn5 = assign102490_e154331_d_n5;
        locals.var_mumodb_dn6 = assign102490_e154331_d_n6;
        locals.var_mumodb_dn7 = assign102490_e154331_d_n7;
        locals.var_mumodb_dn8 = assign102490_e154331_d_n8;
        locals.var_mumodb_dn9 = assign102490_e154331_d_n9;
        locals.var_mumodb_dn10 = assign102490_e154331_d_n10;
        locals.var_mumodb_dn11 = assign102490_e154331_d_n11;
        locals.var_mumodb_dn14 = assign102490_e154331_d_n14;

        let (assign102500_e154353, assign102500_e154353_d_n0, assign102500_e154353_d_n2, assign102500_e154353_d_n4, assign102500_e154353_d_n5, assign102500_e154353_d_n6, assign102500_e154353_d_n7, assign102500_e154353_d_n8, assign102500_e154353_d_n9, assign102500_e154353_d_n10, assign102500_e154353_d_n11, assign102500_e154353_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 != 0.0)) {
        let assign102500_e154338: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign102500_e154342: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign102500_e154343: f64 = (locals.var_kusai00 + assign102500_e154342);
        let assign102500_e154345: f64 = (assign102500_e154343 + locals.var_kusail);
        let assign102500_e154346: f64 = (assign102500_e154338 * assign102500_e154345);
        let assign102500_e154349: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign102500_e154350: f64 = (assign102500_e154346 / assign102500_e154349);
        let assign102500_e154351: f64 = (locals.var_mumoda + assign102500_e154350);
        (assign102500_e154351, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn7 + ((((((0.6666666666666667 * locals.var_mumodb_dn7) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn7 + ((locals.var_vgvt_dn7 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn7))) + locals.var_kusail_dn7))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn7 + locals.var_sqrtkusail_dn7))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn9 + ((((((0.6666666666666667 * locals.var_mumodb_dn9) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn9 + ((locals.var_vgvt_dn9 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn9))) + locals.var_kusail_dn9))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn9 + locals.var_sqrtkusail_dn9))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign102500_e154349 * assign102500_e154349))), (locals.var_mumoda_dn14 + ((((((0.6666666666666667 * locals.var_mumodb_dn14) * assign102500_e154345) + (assign102500_e154338 * ((locals.var_kusai00_dn14 + ((locals.var_vgvt_dn14 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn14))) + locals.var_kusail_dn14))) * assign102500_e154349) - (assign102500_e154346 * (locals.var_vgvt_dn14 + locals.var_sqrtkusail_dn14))) / (assign102500_e154349 * assign102500_e154349))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102500_e154353;
        locals.var_correct_w1_dn0 = assign102500_e154353_d_n0;
        locals.var_correct_w1_dn2 = assign102500_e154353_d_n2;
        locals.var_correct_w1_dn4 = assign102500_e154353_d_n4;
        locals.var_correct_w1_dn5 = assign102500_e154353_d_n5;
        locals.var_correct_w1_dn6 = assign102500_e154353_d_n6;
        locals.var_correct_w1_dn7 = assign102500_e154353_d_n7;
        locals.var_correct_w1_dn8 = assign102500_e154353_d_n8;
        locals.var_correct_w1_dn9 = assign102500_e154353_d_n9;
        locals.var_correct_w1_dn10 = assign102500_e154353_d_n10;
        locals.var_correct_w1_dn11 = assign102500_e154353_d_n11;
        locals.var_correct_w1_dn14 = assign102500_e154353_d_n14;

        let (assign102510_e154362, assign102510_e154362_d_n0, assign102510_e154362_d_n2, assign102510_e154362_d_n4, assign102510_e154362_d_n5, assign102510_e154362_d_n6, assign102510_e154362_d_n7, assign102510_e154362_d_n8, assign102510_e154362_d_n9, assign102510_e154362_d_n10, assign102510_e154362_d_n11, assign102510_e154362_d_n14,) = {
    if ((locals.var_guard2338 != 0.0) && (locals.var_guard2339 == 0.0)) {
        let assign102510_e154360: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign102510_e154360, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn7 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn7)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn9 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn9)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn14 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn14)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn7, locals.var_correct_w1_dn8, locals.var_correct_w1_dn9, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn14,)
    }
};
        locals.var_correct_w1 = assign102510_e154362;
        locals.var_correct_w1_dn0 = assign102510_e154362_d_n0;
        locals.var_correct_w1_dn2 = assign102510_e154362_d_n2;
        locals.var_correct_w1_dn4 = assign102510_e154362_d_n4;
        locals.var_correct_w1_dn5 = assign102510_e154362_d_n5;
        locals.var_correct_w1_dn6 = assign102510_e154362_d_n6;
        locals.var_correct_w1_dn7 = assign102510_e154362_d_n7;
        locals.var_correct_w1_dn8 = assign102510_e154362_d_n8;
        locals.var_correct_w1_dn9 = assign102510_e154362_d_n9;
        locals.var_correct_w1_dn10 = assign102510_e154362_d_n10;
        locals.var_correct_w1_dn11 = assign102510_e154362_d_n11;
        locals.var_correct_w1_dn14 = assign102510_e154362_d_n14;

        let (assign102520_e154372, assign102520_e154372_d_n0, assign102520_e154372_d_n2, assign102520_e154372_d_n4, assign102520_e154372_d_n5, assign102520_e154372_d_n6, assign102520_e154372_d_n7, assign102520_e154372_d_n8, assign102520_e154372_d_n9, assign102520_e154372_d_n10, assign102520_e154372_d_n11, assign102520_e154372_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102520_e154366: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign102520_e154368: f64 = (assign102520_e154366 * locals.var_kusai_ig);
        let assign102520_e154370: f64 = (assign102520_e154368 * locals.var_correct_w1);
        (assign102520_e154370, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn7) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn7)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn7)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn9) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn9)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn9)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn14) * locals.var_kusai_ig) + (assign102520_e154366 * locals.var_kusai_ig_dn14)) * locals.var_correct_w1) + (assign102520_e154368 * locals.var_correct_w1_dn14)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102520_e154372;
        locals.var_noiigate_dn0 = assign102520_e154372_d_n0;
        locals.var_noiigate_dn2 = assign102520_e154372_d_n2;
        locals.var_noiigate_dn4 = assign102520_e154372_d_n4;
        locals.var_noiigate_dn5 = assign102520_e154372_d_n5;
        locals.var_noiigate_dn6 = assign102520_e154372_d_n6;
        locals.var_noiigate_dn7 = assign102520_e154372_d_n7;
        locals.var_noiigate_dn8 = assign102520_e154372_d_n8;
        locals.var_noiigate_dn9 = assign102520_e154372_d_n9;
        locals.var_noiigate_dn10 = assign102520_e154372_d_n10;
        locals.var_noiigate_dn11 = assign102520_e154372_d_n11;
        locals.var_noiigate_dn14 = assign102520_e154372_d_n14;

    }

    pub(super) fn stamp_transient_block_377(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign102530_e154376, assign102530_e154376_d_n0, assign102530_e154376_d_n2, assign102530_e154376_d_n4, assign102530_e154376_d_n5, assign102530_e154376_d_n6, assign102530_e154376_d_n7, assign102530_e154376_d_n8, assign102530_e154376_d_n9, assign102530_e154376_d_n10, assign102530_e154376_d_n11, assign102530_e154376_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        (locals.var_crl_f, locals.var_crl_f_dn0, locals.var_crl_f_dn2, locals.var_crl_f_dn4, locals.var_crl_f_dn5, locals.var_crl_f_dn6, locals.var_crl_f_dn7, locals.var_crl_f_dn8, locals.var_crl_f_dn9, locals.var_crl_f_dn10, locals.var_crl_f_dn11, locals.var_crl_f_dn14,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
    }
};
        locals.var_noicross = assign102530_e154376;
        locals.var_noicross_dn0 = assign102530_e154376_d_n0;
        locals.var_noicross_dn2 = assign102530_e154376_d_n2;
        locals.var_noicross_dn4 = assign102530_e154376_d_n4;
        locals.var_noicross_dn5 = assign102530_e154376_d_n5;
        locals.var_noicross_dn6 = assign102530_e154376_d_n6;
        locals.var_noicross_dn7 = assign102530_e154376_d_n7;
        locals.var_noicross_dn8 = assign102530_e154376_d_n8;
        locals.var_noicross_dn9 = assign102530_e154376_d_n9;
        locals.var_noicross_dn10 = assign102530_e154376_d_n10;
        locals.var_noicross_dn11 = assign102530_e154376_d_n11;
        locals.var_noicross_dn14 = assign102530_e154376_d_n14;

        let (assign102540_e154385, assign102540_e154385_d_n0, assign102540_e154385_d_n2, assign102540_e154385_d_n4, assign102540_e154385_d_n5, assign102540_e154385_d_n6, assign102540_e154385_d_n7, assign102540_e154385_d_n8, assign102540_e154385_d_n9, assign102540_e154385_d_n10, assign102540_e154385_d_n11, assign102540_e154385_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let (assign102540_e154383, assign102540_e154383_d_n0, assign102540_e154383_d_n2, assign102540_e154383_d_n4, assign102540_e154383_d_n5, assign102540_e154383_d_n6, assign102540_e154383_d_n7, assign102540_e154383_d_n8, assign102540_e154383_d_n9, assign102540_e154383_d_n10, assign102540_e154383_d_n11, assign102540_e154383_d_n14,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            }
        };
        (assign102540_e154383, assign102540_e154383_d_n0, assign102540_e154383_d_n2, assign102540_e154383_d_n4, assign102540_e154383_d_n5, assign102540_e154383_d_n6, assign102540_e154383_d_n7, assign102540_e154383_d_n8, assign102540_e154383_d_n9, assign102540_e154383_d_n10, assign102540_e154383_d_n11, assign102540_e154383_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102540_e154385;
        locals.var_noiigate_dn0 = assign102540_e154385_d_n0;
        locals.var_noiigate_dn2 = assign102540_e154385_d_n2;
        locals.var_noiigate_dn4 = assign102540_e154385_d_n4;
        locals.var_noiigate_dn5 = assign102540_e154385_d_n5;
        locals.var_noiigate_dn6 = assign102540_e154385_d_n6;
        locals.var_noiigate_dn7 = assign102540_e154385_d_n7;
        locals.var_noiigate_dn8 = assign102540_e154385_d_n8;
        locals.var_noiigate_dn9 = assign102540_e154385_d_n9;
        locals.var_noiigate_dn10 = assign102540_e154385_d_n10;
        locals.var_noiigate_dn11 = assign102540_e154385_d_n11;
        locals.var_noiigate_dn14 = assign102540_e154385_d_n14;

        let (assign102550_e154395, assign102550_e154395_d_n0, assign102550_e154395_d_n2, assign102550_e154395_d_n4, assign102550_e154395_d_n5, assign102550_e154395_d_n6, assign102550_e154395_d_n7, assign102550_e154395_d_n8, assign102550_e154395_d_n9, assign102550_e154395_d_n10, assign102550_e154395_d_n11, assign102550_e154395_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102550_e154388: f64 = (-locals.var_t10);
        let (assign102550_e154393, assign102550_e154393_d_n0, assign102550_e154393_d_n2, assign102550_e154393_d_n4, assign102550_e154393_d_n5, assign102550_e154393_d_n6, assign102550_e154393_d_n7, assign102550_e154393_d_n8, assign102550_e154393_d_n9, assign102550_e154393_d_n10, assign102550_e154393_d_n11, assign102550_e154393_d_n14,) = {
            if (assign102550_e154388 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102550_e154393, assign102550_e154393_d_n0, assign102550_e154393_d_n2, assign102550_e154393_d_n4, assign102550_e154393_d_n5, assign102550_e154393_d_n6, assign102550_e154393_d_n7, assign102550_e154393_d_n8, assign102550_e154393_d_n9, assign102550_e154393_d_n10, assign102550_e154393_d_n11, assign102550_e154393_d_n14,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn7, locals.var_noiigate_dn8, locals.var_noiigate_dn9, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn14,)
    }
};
        locals.var_noiigate = assign102550_e154395;
        locals.var_noiigate_dn0 = assign102550_e154395_d_n0;
        locals.var_noiigate_dn2 = assign102550_e154395_d_n2;
        locals.var_noiigate_dn4 = assign102550_e154395_d_n4;
        locals.var_noiigate_dn5 = assign102550_e154395_d_n5;
        locals.var_noiigate_dn6 = assign102550_e154395_d_n6;
        locals.var_noiigate_dn7 = assign102550_e154395_d_n7;
        locals.var_noiigate_dn8 = assign102550_e154395_d_n8;
        locals.var_noiigate_dn9 = assign102550_e154395_d_n9;
        locals.var_noiigate_dn10 = assign102550_e154395_d_n10;
        locals.var_noiigate_dn11 = assign102550_e154395_d_n11;
        locals.var_noiigate_dn14 = assign102550_e154395_d_n14;

        let (assign102560_e154405, assign102560_e154405_d_n0, assign102560_e154405_d_n2, assign102560_e154405_d_n4, assign102560_e154405_d_n5, assign102560_e154405_d_n6, assign102560_e154405_d_n7, assign102560_e154405_d_n8, assign102560_e154405_d_n9, assign102560_e154405_d_n10, assign102560_e154405_d_n11, assign102560_e154405_d_n14,) = {
    if (locals.var_guard2338 != 0.0) {
        let assign102560_e154398: f64 = (-locals.var_t10);
        let (assign102560_e154403, assign102560_e154403_d_n0, assign102560_e154403_d_n2, assign102560_e154403_d_n4, assign102560_e154403_d_n5, assign102560_e154403_d_n6, assign102560_e154403_d_n7, assign102560_e154403_d_n8, assign102560_e154403_d_n9, assign102560_e154403_d_n10, assign102560_e154403_d_n11, assign102560_e154403_d_n14,) = {
            if (assign102560_e154398 > locals.var_t0) {
                (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign102560_e154403, assign102560_e154403_d_n0, assign102560_e154403_d_n2, assign102560_e154403_d_n4, assign102560_e154403_d_n5, assign102560_e154403_d_n6, assign102560_e154403_d_n7, assign102560_e154403_d_n8, assign102560_e154403_d_n9, assign102560_e154403_d_n10, assign102560_e154403_d_n11, assign102560_e154403_d_n14,)
    } else {
        (locals.var_noicross, locals.var_noicross_dn0, locals.var_noicross_dn2, locals.var_noicross_dn4, locals.var_noicross_dn5, locals.var_noicross_dn6, locals.var_noicross_dn7, locals.var_noicross_dn8, locals.var_noicross_dn9, locals.var_noicross_dn10, locals.var_noicross_dn11, locals.var_noicross_dn14,)
    }
};
        locals.var_noicross = assign102560_e154405;
        locals.var_noicross_dn0 = assign102560_e154405_d_n0;
        locals.var_noicross_dn2 = assign102560_e154405_d_n2;
        locals.var_noicross_dn4 = assign102560_e154405_d_n4;
        locals.var_noicross_dn5 = assign102560_e154405_d_n5;
        locals.var_noicross_dn6 = assign102560_e154405_d_n6;
        locals.var_noicross_dn7 = assign102560_e154405_d_n7;
        locals.var_noicross_dn8 = assign102560_e154405_d_n8;
        locals.var_noicross_dn9 = assign102560_e154405_d_n9;
        locals.var_noicross_dn10 = assign102560_e154405_d_n10;
        locals.var_noicross_dn11 = assign102560_e154405_d_n11;
        locals.var_noicross_dn14 = assign102560_e154405_d_n14;

        let assign102570_e154408: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign102570_e154408;
        locals.var_sid_dn0 = ((locals.var_whi_noise_dn0 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn0));
        locals.var_sid_dn2 = ((locals.var_whi_noise_dn2 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn2));
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = ((locals.var_whi_noise_dn5 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn5));
        locals.var_sid_dn6 = ((locals.var_whi_noise_dn6 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn6));
        locals.var_sid_dn7 = ((locals.var_whi_noise_dn7 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn7));
        locals.var_sid_dn8 = ((locals.var_whi_noise_dn8 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn8));
        locals.var_sid_dn9 = ((locals.var_whi_noise_dn9 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn9));
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = ((locals.var_whi_noise_dn11 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn11));
        locals.var_sid_dn14 = ((locals.var_whi_noise_dn14 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn14));

        locals.var_ci = locals.var_noicross;
        locals.var_ci_dn0 = locals.var_noicross_dn0;
        locals.var_ci_dn2 = locals.var_noicross_dn2;
        locals.var_ci_dn4 = locals.var_noicross_dn4;
        locals.var_ci_dn5 = locals.var_noicross_dn5;
        locals.var_ci_dn6 = locals.var_noicross_dn6;
        locals.var_ci_dn7 = locals.var_noicross_dn7;
        locals.var_ci_dn8 = locals.var_noicross_dn8;
        locals.var_ci_dn9 = locals.var_noicross_dn9;
        locals.var_ci_dn10 = locals.var_noicross_dn10;
        locals.var_ci_dn11 = locals.var_noicross_dn11;
        locals.var_ci_dn14 = locals.var_noicross_dn14;

        let (assign102590_e154422, assign102590_e154422_d_n0, assign102590_e154422_d_n2, assign102590_e154422_d_n4, assign102590_e154422_d_n5, assign102590_e154422_d_n6, assign102590_e154422_d_n7, assign102590_e154422_d_n8, assign102590_e154422_d_n9, assign102590_e154422_d_n10, assign102590_e154422_d_n11, assign102590_e154422_d_n14,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign102590_e154419: f64 = (locals.var_noiigate / locals.var_sid);
        let assign102590_e154420: f64 = (assign102590_e154419).sqrt();
        (assign102590_e154420, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn9 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn9)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)), ((((locals.var_noiigate_dn14 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn14)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign102590_e154420)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign102590_e154422;
        locals.var_sigrat_dn0 = assign102590_e154422_d_n0;
        locals.var_sigrat_dn2 = assign102590_e154422_d_n2;
        locals.var_sigrat_dn4 = assign102590_e154422_d_n4;
        locals.var_sigrat_dn5 = assign102590_e154422_d_n5;
        locals.var_sigrat_dn6 = assign102590_e154422_d_n6;
        locals.var_sigrat_dn7 = assign102590_e154422_d_n7;
        locals.var_sigrat_dn8 = assign102590_e154422_d_n8;
        locals.var_sigrat_dn9 = assign102590_e154422_d_n9;
        locals.var_sigrat_dn10 = assign102590_e154422_d_n10;
        locals.var_sigrat_dn11 = assign102590_e154422_d_n11;
        locals.var_sigrat_dn14 = assign102590_e154422_d_n14;

        let (assign102600_e154434, assign102600_e154434_d_n0, assign102600_e154434_d_n2, assign102600_e154434_d_n4, assign102600_e154434_d_n5, assign102600_e154434_d_n6, assign102600_e154434_d_n7, assign102600_e154434_d_n8, assign102600_e154434_d_n9, assign102600_e154434_d_n10, assign102600_e154434_d_n11, assign102600_e154434_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102600_e154429: f64 = (1.0 - locals.var_qdrat);
        let assign102600_e154430: f64 = (locals.var_sigrat * assign102600_e154429);
        (assign102600_e154430, ((locals.var_sigrat_dn0 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102600_e154429) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    } else {
        let assign102600_e154433: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102600_e154433, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    }
};
        locals.var_sigrat_s = assign102600_e154434;
        locals.var_sigrat_s_dn0 = assign102600_e154434_d_n0;
        locals.var_sigrat_s_dn2 = assign102600_e154434_d_n2;
        locals.var_sigrat_s_dn4 = assign102600_e154434_d_n4;
        locals.var_sigrat_s_dn5 = assign102600_e154434_d_n5;
        locals.var_sigrat_s_dn6 = assign102600_e154434_d_n6;
        locals.var_sigrat_s_dn7 = assign102600_e154434_d_n7;
        locals.var_sigrat_s_dn8 = assign102600_e154434_d_n8;
        locals.var_sigrat_s_dn9 = assign102600_e154434_d_n9;
        locals.var_sigrat_s_dn10 = assign102600_e154434_d_n10;
        locals.var_sigrat_s_dn11 = assign102600_e154434_d_n11;
        locals.var_sigrat_s_dn14 = assign102600_e154434_d_n14;

        let (assign102610_e154446, assign102610_e154446_d_n0, assign102610_e154446_d_n2, assign102610_e154446_d_n4, assign102610_e154446_d_n5, assign102610_e154446_d_n6, assign102610_e154446_d_n7, assign102610_e154446_d_n8, assign102610_e154446_d_n9, assign102610_e154446_d_n10, assign102610_e154446_d_n11, assign102610_e154446_d_n14,) = {
    if (locals.var_mode > 0.0) {
        let assign102610_e154440: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign102610_e154440, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn4 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn4)), ((locals.var_sigrat_dn5 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn5)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn8 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn8)), ((locals.var_sigrat_dn9 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn9)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn14 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn14)),)
    } else {
        let assign102610_e154444: f64 = (1.0 - locals.var_qdrat);
        let assign102610_e154445: f64 = (locals.var_sigrat * assign102610_e154444);
        (assign102610_e154445, ((locals.var_sigrat_dn0 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn4 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn4))), ((locals.var_sigrat_dn5 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn5))), ((locals.var_sigrat_dn6 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn8 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn8))), ((locals.var_sigrat_dn9 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn9))), ((locals.var_sigrat_dn10 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn14 * assign102610_e154444) + (locals.var_sigrat * (-locals.var_qdrat_dn14))),)
    }
};
        locals.var_sigrat_d = assign102610_e154446;
        locals.var_sigrat_d_dn0 = assign102610_e154446_d_n0;
        locals.var_sigrat_d_dn2 = assign102610_e154446_d_n2;
        locals.var_sigrat_d_dn4 = assign102610_e154446_d_n4;
        locals.var_sigrat_d_dn5 = assign102610_e154446_d_n5;
        locals.var_sigrat_d_dn6 = assign102610_e154446_d_n6;
        locals.var_sigrat_d_dn7 = assign102610_e154446_d_n7;
        locals.var_sigrat_d_dn8 = assign102610_e154446_d_n8;
        locals.var_sigrat_d_dn9 = assign102610_e154446_d_n9;
        locals.var_sigrat_d_dn10 = assign102610_e154446_d_n10;
        locals.var_sigrat_d_dn11 = assign102610_e154446_d_n11;
        locals.var_sigrat_d_dn14 = assign102610_e154446_d_n14;

        locals.var_rsde = 0.0;
        locals.var_rsde_dn0 = 0.0;
        locals.var_rsde_dn2 = 0.0;
        locals.var_rsde_dn4 = 0.0;
        locals.var_rsde_dn5 = 0.0;
        locals.var_rsde_dn6 = 0.0;
        locals.var_rsde_dn7 = 0.0;
        locals.var_rsde_dn8 = 0.0;
        locals.var_rsde_dn9 = 0.0;
        locals.var_rsde_dn10 = 0.0;
        locals.var_rsde_dn11 = 0.0;
        locals.var_rsde_dn14 = 0.0;

        locals.var_rdde = 0.0;
        locals.var_rdde_dn0 = 0.0;
        locals.var_rdde_dn2 = 0.0;
        locals.var_rdde_dn4 = 0.0;
        locals.var_rdde_dn5 = 0.0;
        locals.var_rdde_dn6 = 0.0;
        locals.var_rdde_dn7 = 0.0;
        locals.var_rdde_dn8 = 0.0;
        locals.var_rdde_dn9 = 0.0;
        locals.var_rdde_dn10 = 0.0;
        locals.var_rdde_dn11 = 0.0;
        locals.var_rdde_dn14 = 0.0;

        let assign102640_e154451: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2340 = assign102640_e154451;

        let assign102650_e154454: f64 = if locals.var_flg_rs == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2341 = assign102650_e154454;

        let assign102660_e154461: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2342 = assign102660_e154461;

        let (assign102670_e154477, assign102670_e154477_d_n0, assign102670_e154477_d_n2, assign102670_e154477_d_n4, assign102670_e154477_d_n5, assign102670_e154477_d_n6, assign102670_e154477_d_n7, assign102670_e154477_d_n8, assign102670_e154477_d_n9, assign102670_e154477_d_n10, assign102670_e154477_d_n11, assign102670_e154477_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let (assign102670_e154475, assign102670_e154475_d_n0, assign102670_e154475_d_n2, assign102670_e154475_d_n4, assign102670_e154475_d_n5, assign102670_e154475_d_n6, assign102670_e154475_d_n7, assign102670_e154475_d_n8, assign102670_e154475_d_n9, assign102670_e154475_d_n10, assign102670_e154475_d_n11, assign102670_e154475_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102670_e154474: f64 = (locals.var_tratio).powf(p.p416);
                (assign102670_e154474, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn11)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn14)) } } else { (assign102670_e154474 * (p.p416 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign102670_e154475, assign102670_e154475_d_n0, assign102670_e154475_d_n2, assign102670_e154475_d_n4, assign102670_e154475_d_n5, assign102670_e154475_d_n6, assign102670_e154475_d_n7, assign102670_e154475_d_n8, assign102670_e154475_d_n9, assign102670_e154475_d_n10, assign102670_e154475_d_n11, assign102670_e154475_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102670_e154477;
        locals.var_t1_dn0 = assign102670_e154477_d_n0;
        locals.var_t1_dn2 = assign102670_e154477_d_n2;
        locals.var_t1_dn4 = assign102670_e154477_d_n4;
        locals.var_t1_dn5 = assign102670_e154477_d_n5;
        locals.var_t1_dn6 = assign102670_e154477_d_n6;
        locals.var_t1_dn7 = assign102670_e154477_d_n7;
        locals.var_t1_dn8 = assign102670_e154477_d_n8;
        locals.var_t1_dn9 = assign102670_e154477_d_n9;
        locals.var_t1_dn10 = assign102670_e154477_d_n10;
        locals.var_t1_dn11 = assign102670_e154477_d_n11;
        locals.var_t1_dn14 = assign102670_e154477_d_n14;

        let (assign102680_e154488, assign102680_e154488_d_n0, assign102680_e154488_d_n2, assign102680_e154488_d_n4, assign102680_e154488_d_n5, assign102680_e154488_d_n6, assign102680_e154488_d_n7, assign102680_e154488_d_n8, assign102680_e154488_d_n9, assign102680_e154488_d_n10, assign102680_e154488_d_n11, assign102680_e154488_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102680_e154486: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign102680_e154486, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn11, locals.var_rrdrmues_dn14,)
    }
};
        locals.var_rrdrmues = assign102680_e154488;
        locals.var_rrdrmues_dn0 = assign102680_e154488_d_n0;
        locals.var_rrdrmues_dn2 = assign102680_e154488_d_n2;
        locals.var_rrdrmues_dn4 = assign102680_e154488_d_n4;
        locals.var_rrdrmues_dn5 = assign102680_e154488_d_n5;
        locals.var_rrdrmues_dn6 = assign102680_e154488_d_n6;
        locals.var_rrdrmues_dn7 = assign102680_e154488_d_n7;
        locals.var_rrdrmues_dn8 = assign102680_e154488_d_n8;
        locals.var_rrdrmues_dn9 = assign102680_e154488_d_n9;
        locals.var_rrdrmues_dn10 = assign102680_e154488_d_n10;
        locals.var_rrdrmues_dn11 = assign102680_e154488_d_n11;
        locals.var_rrdrmues_dn14 = assign102680_e154488_d_n14;

        let (assign102690_e154513, assign102690_e154513_d_n0, assign102690_e154513_d_n2, assign102690_e154513_d_n4, assign102690_e154513_d_n5, assign102690_e154513_d_n6, assign102690_e154513_d_n7, assign102690_e154513_d_n8, assign102690_e154513_d_n9, assign102690_e154513_d_n10, assign102690_e154513_d_n11, assign102690_e154513_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102690_e154498: f64 = (0.4 * locals.var_tratio);
        let assign102690_e154499: f64 = (1.8 + assign102690_e154498);
        let assign102690_e154502: f64 = (0.1 * locals.var_tratio);
        let assign102690_e154504: f64 = (assign102690_e154502 * locals.var_tratio);
        let assign102690_e154505: f64 = (assign102690_e154499 + assign102690_e154504);
        let assign102690_e154509: f64 = (1.0 - locals.var_tratio);
        let assign102690_e154510: f64 = (p.p418 * assign102690_e154509);
        let assign102690_e154511: f64 = (assign102690_e154505 - assign102690_e154510);
        (assign102690_e154511, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn11))) - (p.p418 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign102690_e154502 * locals.var_tratio_dn14))) - (p.p418 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign102690_e154513;
        locals.var_t0_dn0 = assign102690_e154513_d_n0;
        locals.var_t0_dn2 = assign102690_e154513_d_n2;
        locals.var_t0_dn4 = assign102690_e154513_d_n4;
        locals.var_t0_dn5 = assign102690_e154513_d_n5;
        locals.var_t0_dn6 = assign102690_e154513_d_n6;
        locals.var_t0_dn7 = assign102690_e154513_d_n7;
        locals.var_t0_dn8 = assign102690_e154513_d_n8;
        locals.var_t0_dn9 = assign102690_e154513_d_n9;
        locals.var_t0_dn10 = assign102690_e154513_d_n10;
        locals.var_t0_dn11 = assign102690_e154513_d_n11;
        locals.var_t0_dn14 = assign102690_e154513_d_n14;

        let (assign102700_e154524, assign102700_e154524_d_n0, assign102700_e154524_d_n2, assign102700_e154524_d_n4, assign102700_e154524_d_n5, assign102700_e154524_d_n6, assign102700_e154524_d_n7, assign102700_e154524_d_n8, assign102700_e154524_d_n9, assign102700_e154524_d_n10, assign102700_e154524_d_n11, assign102700_e154524_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102700_e154522: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign102700_e154522, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn11, locals.var_rrdrvmaxs_dn14,)
    }
};
        locals.var_rrdrvmaxs = assign102700_e154524;
        locals.var_rrdrvmaxs_dn0 = assign102700_e154524_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign102700_e154524_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign102700_e154524_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign102700_e154524_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign102700_e154524_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign102700_e154524_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign102700_e154524_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign102700_e154524_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign102700_e154524_d_n10;
        locals.var_rrdrvmaxs_dn11 = assign102700_e154524_d_n11;
        locals.var_rrdrvmaxs_dn14 = assign102700_e154524_d_n14;

        let (assign102710_e154539, assign102710_e154539_d_n0, assign102710_e154539_d_n2, assign102710_e154539_d_n4, assign102710_e154539_d_n5, assign102710_e154539_d_n6, assign102710_e154539_d_n7, assign102710_e154539_d_n8, assign102710_e154539_d_n9, assign102710_e154539_d_n10, assign102710_e154539_d_n11, assign102710_e154539_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 != 0.0)) {
        let assign102710_e154535: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign102710_e154536: f64 = (p.p439 * assign102710_e154535);
        let assign102710_e154537: f64 = (locals.var_uc_rdrbb_s + assign102710_e154536);
        (assign102710_e154537, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn11 + (p.p439 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_s_dn14 + (p.p439 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn11, locals.var_uc_rdrbb_s_dn14,)
    }
};
        locals.var_uc_rdrbb_s = assign102710_e154539;
        locals.var_uc_rdrbb_s_dn0 = assign102710_e154539_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign102710_e154539_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign102710_e154539_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign102710_e154539_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign102710_e154539_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign102710_e154539_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign102710_e154539_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign102710_e154539_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign102710_e154539_d_n10;
        locals.var_uc_rdrbb_s_dn11 = assign102710_e154539_d_n11;
        locals.var_uc_rdrbb_s_dn14 = assign102710_e154539_d_n14;

        let (assign102720_e154551, assign102720_e154551_d_n0, assign102720_e154551_d_n2, assign102720_e154551_d_n4, assign102720_e154551_d_n5, assign102720_e154551_d_n6, assign102720_e154551_d_n7, assign102720_e154551_d_n8, assign102720_e154551_d_n9, assign102720_e154551_d_n10, assign102720_e154551_d_n11, assign102720_e154551_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2342 == 0.0)) {
        let assign102720_e154547: f64 = ctx_temp;
        let assign102720_e154549: f64 = (assign102720_e154547 + p.p11);
        (assign102720_e154549, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign102720_e154551;
        locals.var_ttemp_dn0 = assign102720_e154551_d_n0;
        locals.var_ttemp_dn2 = assign102720_e154551_d_n2;
        locals.var_ttemp_dn4 = assign102720_e154551_d_n4;
        locals.var_ttemp_dn5 = assign102720_e154551_d_n5;
        locals.var_ttemp_dn6 = assign102720_e154551_d_n6;
        locals.var_ttemp_dn7 = assign102720_e154551_d_n7;
        locals.var_ttemp_dn8 = assign102720_e154551_d_n8;
        locals.var_ttemp_dn9 = assign102720_e154551_d_n9;
        locals.var_ttemp_dn10 = assign102720_e154551_d_n10;
        locals.var_ttemp_dn11 = assign102720_e154551_d_n11;
        locals.var_ttemp_dn14 = assign102720_e154551_d_n14;

        let (assign102730_e154560,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102730_e154558: f64 = (locals.var_weff_ld * p.p7);
        (assign102730_e154558,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign102730_e154560;

        let (assign102740_e154567,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (p.p71,)
    } else {
        (locals.var_ldrifte_s,)
    }
};
        locals.var_ldrifte_s = assign102740_e154567;

        let (assign102750_e154574,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_novers,)
    }
};
        locals.var_novers = assign102750_e154574;

        let (assign102760_e154583, assign102760_e154583_d_n0, assign102760_e154583_d_n2, assign102760_e154583_d_n4, assign102760_e154583_d_n5, assign102760_e154583_d_n6, assign102760_e154583_d_n7, assign102760_e154583_d_n8, assign102760_e154583_d_n9, assign102760_e154583_d_n10, assign102760_e154583_d_n11, assign102760_e154583_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102760_e154581: f64 = (locals.var_rrdrmues * locals.var_rdrmuele);
        (assign102760_e154581, (locals.var_rrdrmues_dn0 * locals.var_rdrmuele), (locals.var_rrdrmues_dn2 * locals.var_rdrmuele), (locals.var_rrdrmues_dn4 * locals.var_rdrmuele), (locals.var_rrdrmues_dn5 * locals.var_rdrmuele), (locals.var_rrdrmues_dn6 * locals.var_rdrmuele), (locals.var_rrdrmues_dn7 * locals.var_rdrmuele), (locals.var_rrdrmues_dn8 * locals.var_rdrmuele), (locals.var_rrdrmues_dn9 * locals.var_rdrmuele), (locals.var_rrdrmues_dn10 * locals.var_rdrmuele), (locals.var_rrdrmues_dn11 * locals.var_rdrmuele), (locals.var_rrdrmues_dn14 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0_s, locals.var_mu0_s_dn0, locals.var_mu0_s_dn2, locals.var_mu0_s_dn4, locals.var_mu0_s_dn5, locals.var_mu0_s_dn6, locals.var_mu0_s_dn7, locals.var_mu0_s_dn8, locals.var_mu0_s_dn9, locals.var_mu0_s_dn10, locals.var_mu0_s_dn11, locals.var_mu0_s_dn14,)
    }
};
        locals.var_mu0_s = assign102760_e154583;
        locals.var_mu0_s_dn0 = assign102760_e154583_d_n0;
        locals.var_mu0_s_dn2 = assign102760_e154583_d_n2;
        locals.var_mu0_s_dn4 = assign102760_e154583_d_n4;
        locals.var_mu0_s_dn5 = assign102760_e154583_d_n5;
        locals.var_mu0_s_dn6 = assign102760_e154583_d_n6;
        locals.var_mu0_s_dn7 = assign102760_e154583_d_n7;
        locals.var_mu0_s_dn8 = assign102760_e154583_d_n8;
        locals.var_mu0_s_dn9 = assign102760_e154583_d_n9;
        locals.var_mu0_s_dn10 = assign102760_e154583_d_n10;
        locals.var_mu0_s_dn11 = assign102760_e154583_d_n11;
        locals.var_mu0_s_dn14 = assign102760_e154583_d_n14;

        let (assign102770_e154596, assign102770_e154596_d_n0, assign102770_e154596_d_n2, assign102770_e154596_d_n4, assign102770_e154596_d_n5, assign102770_e154596_d_n6, assign102770_e154596_d_n7, assign102770_e154596_d_n8, assign102770_e154596_d_n9, assign102770_e154596_d_n10, assign102770_e154596_d_n11, assign102770_e154596_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102770_e154590: f64 = (locals.var_rrdrvmaxs * locals.var_rdrvmaxwe);
        let assign102770_e154592: f64 = (assign102770_e154590 * locals.var_rdrvmaxle);
        let assign102770_e154594: f64 = (assign102770_e154592 + 1e-25);
        (assign102770_e154594, ((locals.var_rrdrvmaxs_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmaxs_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe_s, locals.var_vmaxe_s_dn0, locals.var_vmaxe_s_dn2, locals.var_vmaxe_s_dn4, locals.var_vmaxe_s_dn5, locals.var_vmaxe_s_dn6, locals.var_vmaxe_s_dn7, locals.var_vmaxe_s_dn8, locals.var_vmaxe_s_dn9, locals.var_vmaxe_s_dn10, locals.var_vmaxe_s_dn11, locals.var_vmaxe_s_dn14,)
    }
};
        locals.var_vmaxe_s = assign102770_e154596;
        locals.var_vmaxe_s_dn0 = assign102770_e154596_d_n0;
        locals.var_vmaxe_s_dn2 = assign102770_e154596_d_n2;
        locals.var_vmaxe_s_dn4 = assign102770_e154596_d_n4;
        locals.var_vmaxe_s_dn5 = assign102770_e154596_d_n5;
        locals.var_vmaxe_s_dn6 = assign102770_e154596_d_n6;
        locals.var_vmaxe_s_dn7 = assign102770_e154596_d_n7;
        locals.var_vmaxe_s_dn8 = assign102770_e154596_d_n8;
        locals.var_vmaxe_s_dn9 = assign102770_e154596_d_n9;
        locals.var_vmaxe_s_dn10 = assign102770_e154596_d_n10;
        locals.var_vmaxe_s_dn11 = assign102770_e154596_d_n11;
        locals.var_vmaxe_s_dn14 = assign102770_e154596_d_n14;

        let (assign102780_e154605, assign102780_e154605_d_n2, assign102780_e154605_d_n8,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102780_e154603: f64 = (locals.var_vsps / locals.var_ldrifte_s);
        (assign102780_e154603, (locals.var_vsps_dn2 / locals.var_ldrifte_s), (locals.var_vsps_dn8 / locals.var_ldrifte_s),)
    } else {
        (locals.var_edri_s, locals.var_edri_s_dn2, locals.var_edri_s_dn8,)
    }
};
        locals.var_edri_s = assign102780_e154605;
        locals.var_edri_s_dn2 = assign102780_e154605_d_n2;
        locals.var_edri_s_dn8 = assign102780_e154605_d_n8;

        let (assign102790_e154614, assign102790_e154614_d_n0, assign102790_e154614_d_n2, assign102790_e154614_d_n4, assign102790_e154614_d_n5, assign102790_e154614_d_n6, assign102790_e154614_d_n7, assign102790_e154614_d_n8, assign102790_e154614_d_n9, assign102790_e154614_d_n10, assign102790_e154614_d_n11, assign102790_e154614_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102790_e154612: f64 = (locals.var_mu0_s * locals.var_edri_s);
        (assign102790_e154612, (locals.var_mu0_s_dn0 * locals.var_edri_s), ((locals.var_mu0_s_dn2 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn2)), (locals.var_mu0_s_dn4 * locals.var_edri_s), (locals.var_mu0_s_dn5 * locals.var_edri_s), (locals.var_mu0_s_dn6 * locals.var_edri_s), (locals.var_mu0_s_dn7 * locals.var_edri_s), ((locals.var_mu0_s_dn8 * locals.var_edri_s) + (locals.var_mu0_s * locals.var_edri_s_dn8)), (locals.var_mu0_s_dn9 * locals.var_edri_s), (locals.var_mu0_s_dn10 * locals.var_edri_s), (locals.var_mu0_s_dn11 * locals.var_edri_s), (locals.var_mu0_s_dn14 * locals.var_edri_s),)
    } else {
        (locals.var_vdri_s, locals.var_vdri_s_dn0, locals.var_vdri_s_dn2, locals.var_vdri_s_dn4, locals.var_vdri_s_dn5, locals.var_vdri_s_dn6, locals.var_vdri_s_dn7, locals.var_vdri_s_dn8, locals.var_vdri_s_dn9, locals.var_vdri_s_dn10, locals.var_vdri_s_dn11, locals.var_vdri_s_dn14,)
    }
};
        locals.var_vdri_s = assign102790_e154614;
        locals.var_vdri_s_dn0 = assign102790_e154614_d_n0;
        locals.var_vdri_s_dn2 = assign102790_e154614_d_n2;
        locals.var_vdri_s_dn4 = assign102790_e154614_d_n4;
        locals.var_vdri_s_dn5 = assign102790_e154614_d_n5;
        locals.var_vdri_s_dn6 = assign102790_e154614_d_n6;
        locals.var_vdri_s_dn7 = assign102790_e154614_d_n7;
        locals.var_vdri_s_dn8 = assign102790_e154614_d_n8;
        locals.var_vdri_s_dn9 = assign102790_e154614_d_n9;
        locals.var_vdri_s_dn10 = assign102790_e154614_d_n10;
        locals.var_vdri_s_dn11 = assign102790_e154614_d_n11;
        locals.var_vdri_s_dn14 = assign102790_e154614_d_n14;

        let assign102800_e154617: f64 = if locals.var_vsps >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2343 = assign102800_e154617;

        let (assign102810_e154628, assign102810_e154628_d_n0, assign102810_e154628_d_n2, assign102810_e154628_d_n4, assign102810_e154628_d_n5, assign102810_e154628_d_n6, assign102810_e154628_d_n7, assign102810_e154628_d_n8, assign102810_e154628_d_n9, assign102810_e154628_d_n10, assign102810_e154628_d_n11, assign102810_e154628_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2343 != 0.0)) {
        let assign102810_e154626: f64 = (locals.var_vdri_s / locals.var_vmaxe_s);
        (assign102810_e154626, (((locals.var_vdri_s_dn0 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn2 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn4 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn5 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn6 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn7 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn8 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn9 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn10 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn11 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), (((locals.var_vdri_s_dn14 * locals.var_vmaxe_s) - (locals.var_vdri_s * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102810_e154628;
        locals.var_t1_dn0 = assign102810_e154628_d_n0;
        locals.var_t1_dn2 = assign102810_e154628_d_n2;
        locals.var_t1_dn4 = assign102810_e154628_d_n4;
        locals.var_t1_dn5 = assign102810_e154628_d_n5;
        locals.var_t1_dn6 = assign102810_e154628_d_n6;
        locals.var_t1_dn7 = assign102810_e154628_d_n7;
        locals.var_t1_dn8 = assign102810_e154628_d_n8;
        locals.var_t1_dn9 = assign102810_e154628_d_n9;
        locals.var_t1_dn10 = assign102810_e154628_d_n10;
        locals.var_t1_dn11 = assign102810_e154628_d_n11;
        locals.var_t1_dn14 = assign102810_e154628_d_n14;

    }

    pub(super) fn stamp_transient_block_378(
        locals: &mut StampLocals,
    ) {
        let (assign102820_e154641, assign102820_e154641_d_n0, assign102820_e154641_d_n2, assign102820_e154641_d_n4, assign102820_e154641_d_n5, assign102820_e154641_d_n6, assign102820_e154641_d_n7, assign102820_e154641_d_n8, assign102820_e154641_d_n9, assign102820_e154641_d_n10, assign102820_e154641_d_n11, assign102820_e154641_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2343 == 0.0)) {
        let assign102820_e154637: f64 = (-locals.var_vdri_s);
        let assign102820_e154639: f64 = (assign102820_e154637 / locals.var_vmaxe_s);
        (assign102820_e154639, ((((-locals.var_vdri_s_dn0) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn0)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn2) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn2)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn4) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn4)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn5) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn5)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn6) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn6)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn7) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn7)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn8) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn8)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn9) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn9)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn10) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn10)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn11) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn11)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)), ((((-locals.var_vdri_s_dn14) * locals.var_vmaxe_s) - (assign102820_e154637 * locals.var_vmaxe_s_dn14)) / (locals.var_vmaxe_s * locals.var_vmaxe_s)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102820_e154641;
        locals.var_t1_dn0 = assign102820_e154641_d_n0;
        locals.var_t1_dn2 = assign102820_e154641_d_n2;
        locals.var_t1_dn4 = assign102820_e154641_d_n4;
        locals.var_t1_dn5 = assign102820_e154641_d_n5;
        locals.var_t1_dn6 = assign102820_e154641_d_n6;
        locals.var_t1_dn7 = assign102820_e154641_d_n7;
        locals.var_t1_dn8 = assign102820_e154641_d_n8;
        locals.var_t1_dn9 = assign102820_e154641_d_n9;
        locals.var_t1_dn10 = assign102820_e154641_d_n10;
        locals.var_t1_dn11 = assign102820_e154641_d_n11;
        locals.var_t1_dn14 = assign102820_e154641_d_n14;

        let assign102830_e154645: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154646: f64 = (1.0 - assign102830_e154645);
        let assign102830_e154653: f64 = (10.0 * 2.220446049250313e-16);
        let assign102830_e154654: f64 = (1.0 + assign102830_e154653);
        let assign102830_e154656: f64 = if ((assign102830_e154646 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102830_e154654)) { 1.0 } else { 0.0 };
        locals.var_guard2344 = assign102830_e154656;

        let (assign102840_e154665, assign102840_e154665_d_n0, assign102840_e154665_d_n2, assign102840_e154665_d_n4, assign102840_e154665_d_n5, assign102840_e154665_d_n6, assign102840_e154665_d_n7, assign102840_e154665_d_n8, assign102840_e154665_d_n9, assign102840_e154665_d_n10, assign102840_e154665_d_n11, assign102840_e154665_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102840_e154665;
        locals.var_t3_dn0 = assign102840_e154665_d_n0;
        locals.var_t3_dn2 = assign102840_e154665_d_n2;
        locals.var_t3_dn4 = assign102840_e154665_d_n4;
        locals.var_t3_dn5 = assign102840_e154665_d_n5;
        locals.var_t3_dn6 = assign102840_e154665_d_n6;
        locals.var_t3_dn7 = assign102840_e154665_d_n7;
        locals.var_t3_dn8 = assign102840_e154665_d_n8;
        locals.var_t3_dn9 = assign102840_e154665_d_n9;
        locals.var_t3_dn10 = assign102840_e154665_d_n10;
        locals.var_t3_dn11 = assign102840_e154665_d_n11;
        locals.var_t3_dn14 = assign102840_e154665_d_n14;

        let assign102850_e154669: f64 = (10.0 * 2.220446049250313e-16);
        let assign102850_e154670: f64 = (2.0 - assign102850_e154669);
        let assign102850_e154677: f64 = (10.0 * 2.220446049250313e-16);
        let assign102850_e154678: f64 = (2.0 + assign102850_e154677);
        let assign102850_e154680: f64 = if ((assign102850_e154670 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102850_e154678)) { 1.0 } else { 0.0 };
        locals.var_guard2345 = assign102850_e154680;

        let (assign102860_e154692, assign102860_e154692_d_n0, assign102860_e154692_d_n2, assign102860_e154692_d_n4, assign102860_e154692_d_n5, assign102860_e154692_d_n6, assign102860_e154692_d_n7, assign102860_e154692_d_n8, assign102860_e154692_d_n9, assign102860_e154692_d_n10, assign102860_e154692_d_n11, assign102860_e154692_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102860_e154692;
        locals.var_t3_dn0 = assign102860_e154692_d_n0;
        locals.var_t3_dn2 = assign102860_e154692_d_n2;
        locals.var_t3_dn4 = assign102860_e154692_d_n4;
        locals.var_t3_dn5 = assign102860_e154692_d_n5;
        locals.var_t3_dn6 = assign102860_e154692_d_n6;
        locals.var_t3_dn7 = assign102860_e154692_d_n7;
        locals.var_t3_dn8 = assign102860_e154692_d_n8;
        locals.var_t3_dn9 = assign102860_e154692_d_n9;
        locals.var_t3_dn10 = assign102860_e154692_d_n10;
        locals.var_t3_dn11 = assign102860_e154692_d_n11;
        locals.var_t3_dn14 = assign102860_e154692_d_n14;

        let (assign102870_e154709, assign102870_e154709_d_n0, assign102870_e154709_d_n2, assign102870_e154709_d_n4, assign102870_e154709_d_n5, assign102870_e154709_d_n6, assign102870_e154709_d_n7, assign102870_e154709_d_n8, assign102870_e154709_d_n9, assign102870_e154709_d_n10, assign102870_e154709_d_n11, assign102870_e154709_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2344 == 0.0)) && (locals.var_guard2345 == 0.0)) {
        let assign102870_e154706: f64 = (locals.var_uc_rdrbb_s - 1.0);
        let assign102870_e154707: f64 = (locals.var_t1).powf(assign102870_e154706);
        (assign102870_e154707, if locals.var_uc_rdrbb_s_dn0 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn0)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn0 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn2 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn2)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn2 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn4 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn4)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn4 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn5 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn5)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn5 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn6 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn6)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn6 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn7 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn7)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn7 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn8 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn8)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn8 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn9 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn9)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn9 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn10 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn10)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn10 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn11 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn11)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn11 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_s_dn14 == 0.0 && ((assign102870_e154706) as f64).is_finite() && ((assign102870_e154706) as f64).fract() == 0.0 { if assign102870_e154706 == 0.0 { 0.0 } else { (assign102870_e154706 * ((locals.var_t1).powf(assign102870_e154706 - 1.0) * locals.var_t1_dn14)) } } else { (assign102870_e154707 * ((locals.var_uc_rdrbb_s_dn14 * (locals.var_t1).ln()) + (assign102870_e154706 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign102870_e154709;
        locals.var_t3_dn0 = assign102870_e154709_d_n0;
        locals.var_t3_dn2 = assign102870_e154709_d_n2;
        locals.var_t3_dn4 = assign102870_e154709_d_n4;
        locals.var_t3_dn5 = assign102870_e154709_d_n5;
        locals.var_t3_dn6 = assign102870_e154709_d_n6;
        locals.var_t3_dn7 = assign102870_e154709_d_n7;
        locals.var_t3_dn8 = assign102870_e154709_d_n8;
        locals.var_t3_dn9 = assign102870_e154709_d_n9;
        locals.var_t3_dn10 = assign102870_e154709_d_n10;
        locals.var_t3_dn11 = assign102870_e154709_d_n11;
        locals.var_t3_dn14 = assign102870_e154709_d_n14;

        let (assign102880_e154718, assign102880_e154718_d_n0, assign102880_e154718_d_n2, assign102880_e154718_d_n4, assign102880_e154718_d_n5, assign102880_e154718_d_n6, assign102880_e154718_d_n7, assign102880_e154718_d_n8, assign102880_e154718_d_n9, assign102880_e154718_d_n10, assign102880_e154718_d_n11, assign102880_e154718_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102880_e154716: f64 = (locals.var_t1 * locals.var_t3);
        (assign102880_e154716, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign102880_e154718;
        locals.var_t2_dn0 = assign102880_e154718_d_n0;
        locals.var_t2_dn2 = assign102880_e154718_d_n2;
        locals.var_t2_dn4 = assign102880_e154718_d_n4;
        locals.var_t2_dn5 = assign102880_e154718_d_n5;
        locals.var_t2_dn6 = assign102880_e154718_d_n6;
        locals.var_t2_dn7 = assign102880_e154718_d_n7;
        locals.var_t2_dn8 = assign102880_e154718_d_n8;
        locals.var_t2_dn9 = assign102880_e154718_d_n9;
        locals.var_t2_dn10 = assign102880_e154718_d_n10;
        locals.var_t2_dn11 = assign102880_e154718_d_n11;
        locals.var_t2_dn14 = assign102880_e154718_d_n14;

        let (assign102890_e154727, assign102890_e154727_d_n0, assign102890_e154727_d_n2, assign102890_e154727_d_n4, assign102890_e154727_d_n5, assign102890_e154727_d_n6, assign102890_e154727_d_n7, assign102890_e154727_d_n8, assign102890_e154727_d_n9, assign102890_e154727_d_n10, assign102890_e154727_d_n11, assign102890_e154727_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102890_e154725: f64 = (1.0 + locals.var_t2);
        (assign102890_e154725, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign102890_e154727;
        locals.var_t4_dn0 = assign102890_e154727_d_n0;
        locals.var_t4_dn2 = assign102890_e154727_d_n2;
        locals.var_t4_dn4 = assign102890_e154727_d_n4;
        locals.var_t4_dn5 = assign102890_e154727_d_n5;
        locals.var_t4_dn6 = assign102890_e154727_d_n6;
        locals.var_t4_dn7 = assign102890_e154727_d_n7;
        locals.var_t4_dn8 = assign102890_e154727_d_n8;
        locals.var_t4_dn9 = assign102890_e154727_d_n9;
        locals.var_t4_dn10 = assign102890_e154727_d_n10;
        locals.var_t4_dn11 = assign102890_e154727_d_n11;
        locals.var_t4_dn14 = assign102890_e154727_d_n14;

        let assign102900_e154731: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154732: f64 = (1.0 - assign102900_e154731);
        let assign102900_e154739: f64 = (10.0 * 2.220446049250313e-16);
        let assign102900_e154740: f64 = (1.0 + assign102900_e154739);
        let assign102900_e154742: f64 = if ((assign102900_e154732 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102900_e154740)) { 1.0 } else { 0.0 };
        locals.var_guard2346 = assign102900_e154742;

        let (assign102910_e154753, assign102910_e154753_d_n0, assign102910_e154753_d_n2, assign102910_e154753_d_n4, assign102910_e154753_d_n5, assign102910_e154753_d_n6, assign102910_e154753_d_n7, assign102910_e154753_d_n8, assign102910_e154753_d_n9, assign102910_e154753_d_n10, assign102910_e154753_d_n11, assign102910_e154753_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 != 0.0)) {
        let assign102910_e154751: f64 = (1.0 / locals.var_t4);
        (assign102910_e154751, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102910_e154753;
        locals.var_t5_dn0 = assign102910_e154753_d_n0;
        locals.var_t5_dn2 = assign102910_e154753_d_n2;
        locals.var_t5_dn4 = assign102910_e154753_d_n4;
        locals.var_t5_dn5 = assign102910_e154753_d_n5;
        locals.var_t5_dn6 = assign102910_e154753_d_n6;
        locals.var_t5_dn7 = assign102910_e154753_d_n7;
        locals.var_t5_dn8 = assign102910_e154753_d_n8;
        locals.var_t5_dn9 = assign102910_e154753_d_n9;
        locals.var_t5_dn10 = assign102910_e154753_d_n10;
        locals.var_t5_dn11 = assign102910_e154753_d_n11;
        locals.var_t5_dn14 = assign102910_e154753_d_n14;

        let assign102920_e154757: f64 = (10.0 * 2.220446049250313e-16);
        let assign102920_e154758: f64 = (2.0 - assign102920_e154757);
        let assign102920_e154765: f64 = (10.0 * 2.220446049250313e-16);
        let assign102920_e154766: f64 = (2.0 + assign102920_e154765);
        let assign102920_e154768: f64 = if ((assign102920_e154758 <= locals.var_uc_rdrbb_s) && (locals.var_uc_rdrbb_s <= assign102920_e154766)) { 1.0 } else { 0.0 };
        locals.var_guard2347 = assign102920_e154768;

        let (assign102930_e154783, assign102930_e154783_d_n0, assign102930_e154783_d_n2, assign102930_e154783_d_n4, assign102930_e154783_d_n5, assign102930_e154783_d_n6, assign102930_e154783_d_n7, assign102930_e154783_d_n8, assign102930_e154783_d_n9, assign102930_e154783_d_n10, assign102930_e154783_d_n11, assign102930_e154783_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 != 0.0)) {
        let assign102930_e154780: f64 = (locals.var_t4).sqrt();
        let assign102930_e154781: f64 = (1.0 / assign102930_e154780);
        (assign102930_e154781, (-((locals.var_t4_dn0 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn2 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn4 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn5 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn6 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn7 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn8 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn9 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn10 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn11 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))), (-((locals.var_t4_dn14 / (2.0 * assign102930_e154780)) / (assign102930_e154780 * assign102930_e154780))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102930_e154783;
        locals.var_t5_dn0 = assign102930_e154783_d_n0;
        locals.var_t5_dn2 = assign102930_e154783_d_n2;
        locals.var_t5_dn4 = assign102930_e154783_d_n4;
        locals.var_t5_dn5 = assign102930_e154783_d_n5;
        locals.var_t5_dn6 = assign102930_e154783_d_n6;
        locals.var_t5_dn7 = assign102930_e154783_d_n7;
        locals.var_t5_dn8 = assign102930_e154783_d_n8;
        locals.var_t5_dn9 = assign102930_e154783_d_n9;
        locals.var_t5_dn10 = assign102930_e154783_d_n10;
        locals.var_t5_dn11 = assign102930_e154783_d_n11;
        locals.var_t5_dn14 = assign102930_e154783_d_n14;

        let (assign102940_e154808, assign102940_e154808_d_n0, assign102940_e154808_d_n2, assign102940_e154808_d_n4, assign102940_e154808_d_n5, assign102940_e154808_d_n6, assign102940_e154808_d_n7, assign102940_e154808_d_n8, assign102940_e154808_d_n9, assign102940_e154808_d_n10, assign102940_e154808_d_n11, assign102940_e154808_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) {
        let (assign102940_e154806, assign102940_e154806_d_n0, assign102940_e154806_d_n2, assign102940_e154806_d_n4, assign102940_e154806_d_n5, assign102940_e154806_d_n6, assign102940_e154806_d_n7, assign102940_e154806_d_n8, assign102940_e154806_d_n9, assign102940_e154806_d_n10, assign102940_e154806_d_n11, assign102940_e154806_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign102940_e154800: f64 = (-1.0);
                let assign102940_e154802: f64 = (assign102940_e154800 / locals.var_uc_rdrbb_s);
                let assign102940_e154804: f64 = (assign102940_e154802 - 1.0);
                let assign102940_e154805: f64 = (locals.var_t4).powf(assign102940_e154804);
                (assign102940_e154805, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn0)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn0) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn2)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn2) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn4)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn4) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn5)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn5) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn6)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn6) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn7)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn7) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn8)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn8) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn9)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn9) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn10)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn10) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn11)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn11) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) == 0.0 && ((assign102940_e154804) as f64).is_finite() && ((assign102940_e154804) as f64).fract() == 0.0 { if assign102940_e154804 == 0.0 { 0.0 } else { (assign102940_e154804 * ((locals.var_t4).powf(assign102940_e154804 - 1.0) * locals.var_t4_dn14)) } } else { (assign102940_e154805 * (((-((assign102940_e154800 * locals.var_uc_rdrbb_s_dn14) / (locals.var_uc_rdrbb_s * locals.var_uc_rdrbb_s))) * (locals.var_t4).ln()) + (assign102940_e154804 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign102940_e154806, assign102940_e154806_d_n0, assign102940_e154806_d_n2, assign102940_e154806_d_n4, assign102940_e154806_d_n5, assign102940_e154806_d_n6, assign102940_e154806_d_n7, assign102940_e154806_d_n8, assign102940_e154806_d_n9, assign102940_e154806_d_n10, assign102940_e154806_d_n11, assign102940_e154806_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign102940_e154808;
        locals.var_t6_dn0 = assign102940_e154808_d_n0;
        locals.var_t6_dn2 = assign102940_e154808_d_n2;
        locals.var_t6_dn4 = assign102940_e154808_d_n4;
        locals.var_t6_dn5 = assign102940_e154808_d_n5;
        locals.var_t6_dn6 = assign102940_e154808_d_n6;
        locals.var_t6_dn7 = assign102940_e154808_d_n7;
        locals.var_t6_dn8 = assign102940_e154808_d_n8;
        locals.var_t6_dn9 = assign102940_e154808_d_n9;
        locals.var_t6_dn10 = assign102940_e154808_d_n10;
        locals.var_t6_dn11 = assign102940_e154808_d_n11;
        locals.var_t6_dn14 = assign102940_e154808_d_n14;

        let (assign102950_e154823, assign102950_e154823_d_n0, assign102950_e154823_d_n2, assign102950_e154823_d_n4, assign102950_e154823_d_n5, assign102950_e154823_d_n6, assign102950_e154823_d_n7, assign102950_e154823_d_n8, assign102950_e154823_d_n9, assign102950_e154823_d_n10, assign102950_e154823_d_n11, assign102950_e154823_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2346 == 0.0)) && (locals.var_guard2347 == 0.0)) {
        let assign102950_e154821: f64 = (locals.var_t4 * locals.var_t6);
        (assign102950_e154821, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign102950_e154823;
        locals.var_t5_dn0 = assign102950_e154823_d_n0;
        locals.var_t5_dn2 = assign102950_e154823_d_n2;
        locals.var_t5_dn4 = assign102950_e154823_d_n4;
        locals.var_t5_dn5 = assign102950_e154823_d_n5;
        locals.var_t5_dn6 = assign102950_e154823_d_n6;
        locals.var_t5_dn7 = assign102950_e154823_d_n7;
        locals.var_t5_dn8 = assign102950_e154823_d_n8;
        locals.var_t5_dn9 = assign102950_e154823_d_n9;
        locals.var_t5_dn10 = assign102950_e154823_d_n10;
        locals.var_t5_dn11 = assign102950_e154823_d_n11;
        locals.var_t5_dn14 = assign102950_e154823_d_n14;

        let (assign102960_e154832, assign102960_e154832_d_n0, assign102960_e154832_d_n2, assign102960_e154832_d_n4, assign102960_e154832_d_n5, assign102960_e154832_d_n6, assign102960_e154832_d_n7, assign102960_e154832_d_n8, assign102960_e154832_d_n9, assign102960_e154832_d_n10, assign102960_e154832_d_n11, assign102960_e154832_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102960_e154830: f64 = (locals.var_mu0_s * locals.var_t5);
        (assign102960_e154830, ((locals.var_mu0_s_dn0 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn0)), ((locals.var_mu0_s_dn2 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn2)), ((locals.var_mu0_s_dn4 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn4)), ((locals.var_mu0_s_dn5 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn5)), ((locals.var_mu0_s_dn6 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn6)), ((locals.var_mu0_s_dn7 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn7)), ((locals.var_mu0_s_dn8 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn8)), ((locals.var_mu0_s_dn9 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn9)), ((locals.var_mu0_s_dn10 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn10)), ((locals.var_mu0_s_dn11 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn11)), ((locals.var_mu0_s_dn14 * locals.var_t5) + (locals.var_mu0_s * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu_s, locals.var_mu_s_dn0, locals.var_mu_s_dn2, locals.var_mu_s_dn4, locals.var_mu_s_dn5, locals.var_mu_s_dn6, locals.var_mu_s_dn7, locals.var_mu_s_dn8, locals.var_mu_s_dn9, locals.var_mu_s_dn10, locals.var_mu_s_dn11, locals.var_mu_s_dn14,)
    }
};
        locals.var_mu_s = assign102960_e154832;
        locals.var_mu_s_dn0 = assign102960_e154832_d_n0;
        locals.var_mu_s_dn2 = assign102960_e154832_d_n2;
        locals.var_mu_s_dn4 = assign102960_e154832_d_n4;
        locals.var_mu_s_dn5 = assign102960_e154832_d_n5;
        locals.var_mu_s_dn6 = assign102960_e154832_d_n6;
        locals.var_mu_s_dn7 = assign102960_e154832_d_n7;
        locals.var_mu_s_dn8 = assign102960_e154832_d_n8;
        locals.var_mu_s_dn9 = assign102960_e154832_d_n9;
        locals.var_mu_s_dn10 = assign102960_e154832_d_n10;
        locals.var_mu_s_dn11 = assign102960_e154832_d_n11;
        locals.var_mu_s_dn14 = assign102960_e154832_d_n14;

        let (assign102970_e154839,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_novers,)
    } else {
        (locals.var_carr_s,)
    }
};
        locals.var_carr_s = assign102970_e154839;

        let (assign102980_e154846,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        (locals.var_xmax_s,)
    } else {
        (locals.var_xov_s,)
    }
};
        locals.var_xov_s = assign102980_e154846;

        let (assign102990_e154855, assign102990_e154855_d_n0, assign102990_e154855_d_n2, assign102990_e154855_d_n4, assign102990_e154855_d_n5, assign102990_e154855_d_n6, assign102990_e154855_d_n7, assign102990_e154855_d_n8, assign102990_e154855_d_n9, assign102990_e154855_d_n10, assign102990_e154855_d_n11, assign102990_e154855_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign102990_e154853: f64 = (1.6021918e-19 / locals.var_ldrifte_s);
        (assign102990_e154853, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign102990_e154855;
        locals.var_t1_dn0 = assign102990_e154855_d_n0;
        locals.var_t1_dn2 = assign102990_e154855_d_n2;
        locals.var_t1_dn4 = assign102990_e154855_d_n4;
        locals.var_t1_dn5 = assign102990_e154855_d_n5;
        locals.var_t1_dn6 = assign102990_e154855_d_n6;
        locals.var_t1_dn7 = assign102990_e154855_d_n7;
        locals.var_t1_dn8 = assign102990_e154855_d_n8;
        locals.var_t1_dn9 = assign102990_e154855_d_n9;
        locals.var_t1_dn10 = assign102990_e154855_d_n10;
        locals.var_t1_dn11 = assign102990_e154855_d_n11;
        locals.var_t1_dn14 = assign102990_e154855_d_n14;

        let (assign103000_e154868, assign103000_e154868_d_n0, assign103000_e154868_d_n2, assign103000_e154868_d_n4, assign103000_e154868_d_n5, assign103000_e154868_d_n6, assign103000_e154868_d_n7, assign103000_e154868_d_n8, assign103000_e154868_d_n9, assign103000_e154868_d_n10, assign103000_e154868_d_n11, assign103000_e154868_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103000_e154862: f64 = (locals.var_t1 * locals.var_xov_s);
        let assign103000_e154864: f64 = (assign103000_e154862 * locals.var_mu_s);
        let assign103000_e154866: f64 = (assign103000_e154864 * locals.var_carr_s);
        (assign103000_e154866, ((((locals.var_t1_dn0 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn0)) * locals.var_carr_s), ((((locals.var_t1_dn2 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn2)) * locals.var_carr_s), ((((locals.var_t1_dn4 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn4)) * locals.var_carr_s), ((((locals.var_t1_dn5 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn5)) * locals.var_carr_s), ((((locals.var_t1_dn6 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn6)) * locals.var_carr_s), ((((locals.var_t1_dn7 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn7)) * locals.var_carr_s), ((((locals.var_t1_dn8 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn8)) * locals.var_carr_s), ((((locals.var_t1_dn9 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn9)) * locals.var_carr_s), ((((locals.var_t1_dn10 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn10)) * locals.var_carr_s), ((((locals.var_t1_dn11 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn11)) * locals.var_carr_s), ((((locals.var_t1_dn14 * locals.var_xov_s) * locals.var_mu_s) + (assign103000_e154862 * locals.var_mu_s_dn14)) * locals.var_carr_s),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103000_e154868;
        locals.var_gd_s_dn0 = assign103000_e154868_d_n0;
        locals.var_gd_s_dn2 = assign103000_e154868_d_n2;
        locals.var_gd_s_dn4 = assign103000_e154868_d_n4;
        locals.var_gd_s_dn5 = assign103000_e154868_d_n5;
        locals.var_gd_s_dn6 = assign103000_e154868_d_n6;
        locals.var_gd_s_dn7 = assign103000_e154868_d_n7;
        locals.var_gd_s_dn8 = assign103000_e154868_d_n8;
        locals.var_gd_s_dn9 = assign103000_e154868_d_n9;
        locals.var_gd_s_dn10 = assign103000_e154868_d_n10;
        locals.var_gd_s_dn11 = assign103000_e154868_d_n11;
        locals.var_gd_s_dn14 = assign103000_e154868_d_n14;

        let assign103010_e154872: f64 = 1e-25;
        let assign103010_e154877: f64 = if ((locals.var_gd_s < assign103010_e154872) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2348 = assign103010_e154877;

        let (assign103020_e154890, assign103020_e154890_d_n0, assign103020_e154890_d_n2, assign103020_e154890_d_n4, assign103020_e154890_d_n5, assign103020_e154890_d_n6, assign103020_e154890_d_n7, assign103020_e154890_d_n8, assign103020_e154890_d_n9, assign103020_e154890_d_n10, assign103020_e154890_d_n11, assign103020_e154890_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103020_e154886: f64 = 1e-25;
        let assign103020_e154888: f64 = (assign103020_e154886 - locals.var_gd_s);
        (assign103020_e154888, (-locals.var_gd_s_dn0), (-locals.var_gd_s_dn2), (-locals.var_gd_s_dn4), (-locals.var_gd_s_dn5), (-locals.var_gd_s_dn6), (-locals.var_gd_s_dn7), (-locals.var_gd_s_dn8), (-locals.var_gd_s_dn9), (-locals.var_gd_s_dn10), (-locals.var_gd_s_dn11), (-locals.var_gd_s_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103020_e154890;
        locals.var_tmf1_dn0 = assign103020_e154890_d_n0;
        locals.var_tmf1_dn2 = assign103020_e154890_d_n2;
        locals.var_tmf1_dn4 = assign103020_e154890_d_n4;
        locals.var_tmf1_dn5 = assign103020_e154890_d_n5;
        locals.var_tmf1_dn6 = assign103020_e154890_d_n6;
        locals.var_tmf1_dn7 = assign103020_e154890_d_n7;
        locals.var_tmf1_dn8 = assign103020_e154890_d_n8;
        locals.var_tmf1_dn9 = assign103020_e154890_d_n9;
        locals.var_tmf1_dn10 = assign103020_e154890_d_n10;
        locals.var_tmf1_dn11 = assign103020_e154890_d_n11;
        locals.var_tmf1_dn14 = assign103020_e154890_d_n14;

        let (assign103030_e154901, assign103030_e154901_d_n0, assign103030_e154901_d_n2, assign103030_e154901_d_n4, assign103030_e154901_d_n5, assign103030_e154901_d_n6, assign103030_e154901_d_n7, assign103030_e154901_d_n8, assign103030_e154901_d_n9, assign103030_e154901_d_n10, assign103030_e154901_d_n11, assign103030_e154901_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103030_e154899: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign103030_e154899, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign103030_e154901;
        locals.var_x2_dn0 = assign103030_e154901_d_n0;
        locals.var_x2_dn2 = assign103030_e154901_d_n2;
        locals.var_x2_dn4 = assign103030_e154901_d_n4;
        locals.var_x2_dn5 = assign103030_e154901_d_n5;
        locals.var_x2_dn6 = assign103030_e154901_d_n6;
        locals.var_x2_dn7 = assign103030_e154901_d_n7;
        locals.var_x2_dn8 = assign103030_e154901_d_n8;
        locals.var_x2_dn9 = assign103030_e154901_d_n9;
        locals.var_x2_dn10 = assign103030_e154901_d_n10;
        locals.var_x2_dn11 = assign103030_e154901_d_n11;
        locals.var_x2_dn14 = assign103030_e154901_d_n14;

        let (assign103040_e154912, assign103040_e154912_d_n0, assign103040_e154912_d_n2, assign103040_e154912_d_n4, assign103040_e154912_d_n5, assign103040_e154912_d_n6, assign103040_e154912_d_n7, assign103040_e154912_d_n8, assign103040_e154912_d_n9, assign103040_e154912_d_n10, assign103040_e154912_d_n11, assign103040_e154912_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103040_e154910: f64 = (1e-25 * 1e-25);
        (assign103040_e154910, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign103040_e154912;
        locals.var_xmax2_dn0 = assign103040_e154912_d_n0;
        locals.var_xmax2_dn2 = assign103040_e154912_d_n2;
        locals.var_xmax2_dn4 = assign103040_e154912_d_n4;
        locals.var_xmax2_dn5 = assign103040_e154912_d_n5;
        locals.var_xmax2_dn6 = assign103040_e154912_d_n6;
        locals.var_xmax2_dn7 = assign103040_e154912_d_n7;
        locals.var_xmax2_dn8 = assign103040_e154912_d_n8;
        locals.var_xmax2_dn9 = assign103040_e154912_d_n9;
        locals.var_xmax2_dn10 = assign103040_e154912_d_n10;
        locals.var_xmax2_dn11 = assign103040_e154912_d_n11;
        locals.var_xmax2_dn14 = assign103040_e154912_d_n14;

        let (assign103050_e154921, assign103050_e154921_d_n0, assign103050_e154921_d_n2, assign103050_e154921_d_n4, assign103050_e154921_d_n5, assign103050_e154921_d_n6, assign103050_e154921_d_n7, assign103050_e154921_d_n8, assign103050_e154921_d_n9, assign103050_e154921_d_n10, assign103050_e154921_d_n11, assign103050_e154921_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103050_e154921;
        locals.var_xp_dn0 = assign103050_e154921_d_n0;
        locals.var_xp_dn2 = assign103050_e154921_d_n2;
        locals.var_xp_dn4 = assign103050_e154921_d_n4;
        locals.var_xp_dn5 = assign103050_e154921_d_n5;
        locals.var_xp_dn6 = assign103050_e154921_d_n6;
        locals.var_xp_dn7 = assign103050_e154921_d_n7;
        locals.var_xp_dn8 = assign103050_e154921_d_n8;
        locals.var_xp_dn9 = assign103050_e154921_d_n9;
        locals.var_xp_dn10 = assign103050_e154921_d_n10;
        locals.var_xp_dn11 = assign103050_e154921_d_n11;
        locals.var_xp_dn14 = assign103050_e154921_d_n14;

        let (assign103060_e154930, assign103060_e154930_d_n0, assign103060_e154930_d_n2, assign103060_e154930_d_n4, assign103060_e154930_d_n5, assign103060_e154930_d_n6, assign103060_e154930_d_n7, assign103060_e154930_d_n8, assign103060_e154930_d_n9, assign103060_e154930_d_n10, assign103060_e154930_d_n11, assign103060_e154930_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103060_e154930;
        locals.var_xmp_dn0 = assign103060_e154930_d_n0;
        locals.var_xmp_dn2 = assign103060_e154930_d_n2;
        locals.var_xmp_dn4 = assign103060_e154930_d_n4;
        locals.var_xmp_dn5 = assign103060_e154930_d_n5;
        locals.var_xmp_dn6 = assign103060_e154930_d_n6;
        locals.var_xmp_dn7 = assign103060_e154930_d_n7;
        locals.var_xmp_dn8 = assign103060_e154930_d_n8;
        locals.var_xmp_dn9 = assign103060_e154930_d_n9;
        locals.var_xmp_dn10 = assign103060_e154930_d_n10;
        locals.var_xmp_dn11 = assign103060_e154930_d_n11;
        locals.var_xmp_dn14 = assign103060_e154930_d_n14;

        let (assign103070_e154939,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103070_e154939;

        let (assign103080_e154948,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103080_e154948;

        let (assign103090_e154957, assign103090_e154957_d_n0, assign103090_e154957_d_n2, assign103090_e154957_d_n4, assign103090_e154957_d_n5, assign103090_e154957_d_n6, assign103090_e154957_d_n7, assign103090_e154957_d_n8, assign103090_e154957_d_n9, assign103090_e154957_d_n10, assign103090_e154957_d_n11, assign103090_e154957_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103090_e154957;
        locals.var_arg_dn0 = assign103090_e154957_d_n0;
        locals.var_arg_dn2 = assign103090_e154957_d_n2;
        locals.var_arg_dn4 = assign103090_e154957_d_n4;
        locals.var_arg_dn5 = assign103090_e154957_d_n5;
        locals.var_arg_dn6 = assign103090_e154957_d_n6;
        locals.var_arg_dn7 = assign103090_e154957_d_n7;
        locals.var_arg_dn8 = assign103090_e154957_d_n8;
        locals.var_arg_dn9 = assign103090_e154957_d_n9;
        locals.var_arg_dn10 = assign103090_e154957_d_n10;
        locals.var_arg_dn11 = assign103090_e154957_d_n11;
        locals.var_arg_dn14 = assign103090_e154957_d_n14;

        let (assign103100_e154966, assign103100_e154966_d_n0, assign103100_e154966_d_n2, assign103100_e154966_d_n4, assign103100_e154966_d_n5, assign103100_e154966_d_n6, assign103100_e154966_d_n7, assign103100_e154966_d_n8, assign103100_e154966_d_n9, assign103100_e154966_d_n10, assign103100_e154966_d_n11, assign103100_e154966_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103100_e154966;
        locals.var_dnm_dn0 = assign103100_e154966_d_n0;
        locals.var_dnm_dn2 = assign103100_e154966_d_n2;
        locals.var_dnm_dn4 = assign103100_e154966_d_n4;
        locals.var_dnm_dn5 = assign103100_e154966_d_n5;
        locals.var_dnm_dn6 = assign103100_e154966_d_n6;
        locals.var_dnm_dn7 = assign103100_e154966_d_n7;
        locals.var_dnm_dn8 = assign103100_e154966_d_n8;
        locals.var_dnm_dn9 = assign103100_e154966_d_n9;
        locals.var_dnm_dn10 = assign103100_e154966_d_n10;
        locals.var_dnm_dn11 = assign103100_e154966_d_n11;
        locals.var_dnm_dn14 = assign103100_e154966_d_n14;

    }

    pub(super) fn stamp_transient_block_379(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103110_e154977, assign103110_e154977_d_n0, assign103110_e154977_d_n2, assign103110_e154977_d_n4, assign103110_e154977_d_n5, assign103110_e154977_d_n6, assign103110_e154977_d_n7, assign103110_e154977_d_n8, assign103110_e154977_d_n9, assign103110_e154977_d_n10, assign103110_e154977_d_n11, assign103110_e154977_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103110_e154975: f64 = (locals.var_xp * locals.var_x2);
        (assign103110_e154975, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103110_e154977;
        locals.var_xp_dn0 = assign103110_e154977_d_n0;
        locals.var_xp_dn2 = assign103110_e154977_d_n2;
        locals.var_xp_dn4 = assign103110_e154977_d_n4;
        locals.var_xp_dn5 = assign103110_e154977_d_n5;
        locals.var_xp_dn6 = assign103110_e154977_d_n6;
        locals.var_xp_dn7 = assign103110_e154977_d_n7;
        locals.var_xp_dn8 = assign103110_e154977_d_n8;
        locals.var_xp_dn9 = assign103110_e154977_d_n9;
        locals.var_xp_dn10 = assign103110_e154977_d_n10;
        locals.var_xp_dn11 = assign103110_e154977_d_n11;
        locals.var_xp_dn14 = assign103110_e154977_d_n14;

        let (assign103120_e154988, assign103120_e154988_d_n0, assign103120_e154988_d_n2, assign103120_e154988_d_n4, assign103120_e154988_d_n5, assign103120_e154988_d_n6, assign103120_e154988_d_n7, assign103120_e154988_d_n8, assign103120_e154988_d_n9, assign103120_e154988_d_n10, assign103120_e154988_d_n11, assign103120_e154988_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103120_e154986: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103120_e154986, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103120_e154988;
        locals.var_xmp_dn0 = assign103120_e154988_d_n0;
        locals.var_xmp_dn2 = assign103120_e154988_d_n2;
        locals.var_xmp_dn4 = assign103120_e154988_d_n4;
        locals.var_xmp_dn5 = assign103120_e154988_d_n5;
        locals.var_xmp_dn6 = assign103120_e154988_d_n6;
        locals.var_xmp_dn7 = assign103120_e154988_d_n7;
        locals.var_xmp_dn8 = assign103120_e154988_d_n8;
        locals.var_xmp_dn9 = assign103120_e154988_d_n9;
        locals.var_xmp_dn10 = assign103120_e154988_d_n10;
        locals.var_xmp_dn11 = assign103120_e154988_d_n11;
        locals.var_xmp_dn14 = assign103120_e154988_d_n14;

        let (assign103130_e154999, assign103130_e154999_d_n0, assign103130_e154999_d_n2, assign103130_e154999_d_n4, assign103130_e154999_d_n5, assign103130_e154999_d_n6, assign103130_e154999_d_n7, assign103130_e154999_d_n8, assign103130_e154999_d_n9, assign103130_e154999_d_n10, assign103130_e154999_d_n11, assign103130_e154999_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103130_e154997: f64 = (locals.var_xp * locals.var_x2);
        (assign103130_e154997, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign103130_e154999;
        locals.var_xp_dn0 = assign103130_e154999_d_n0;
        locals.var_xp_dn2 = assign103130_e154999_d_n2;
        locals.var_xp_dn4 = assign103130_e154999_d_n4;
        locals.var_xp_dn5 = assign103130_e154999_d_n5;
        locals.var_xp_dn6 = assign103130_e154999_d_n6;
        locals.var_xp_dn7 = assign103130_e154999_d_n7;
        locals.var_xp_dn8 = assign103130_e154999_d_n8;
        locals.var_xp_dn9 = assign103130_e154999_d_n9;
        locals.var_xp_dn10 = assign103130_e154999_d_n10;
        locals.var_xp_dn11 = assign103130_e154999_d_n11;
        locals.var_xp_dn14 = assign103130_e154999_d_n14;

        let (assign103140_e155010, assign103140_e155010_d_n0, assign103140_e155010_d_n2, assign103140_e155010_d_n4, assign103140_e155010_d_n5, assign103140_e155010_d_n6, assign103140_e155010_d_n7, assign103140_e155010_d_n8, assign103140_e155010_d_n9, assign103140_e155010_d_n10, assign103140_e155010_d_n11, assign103140_e155010_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103140_e155008: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign103140_e155008, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign103140_e155010;
        locals.var_xmp_dn0 = assign103140_e155010_d_n0;
        locals.var_xmp_dn2 = assign103140_e155010_d_n2;
        locals.var_xmp_dn4 = assign103140_e155010_d_n4;
        locals.var_xmp_dn5 = assign103140_e155010_d_n5;
        locals.var_xmp_dn6 = assign103140_e155010_d_n6;
        locals.var_xmp_dn7 = assign103140_e155010_d_n7;
        locals.var_xmp_dn8 = assign103140_e155010_d_n8;
        locals.var_xmp_dn9 = assign103140_e155010_d_n9;
        locals.var_xmp_dn10 = assign103140_e155010_d_n10;
        locals.var_xmp_dn11 = assign103140_e155010_d_n11;
        locals.var_xmp_dn14 = assign103140_e155010_d_n14;

        let (assign103150_e155021, assign103150_e155021_d_n0, assign103150_e155021_d_n2, assign103150_e155021_d_n4, assign103150_e155021_d_n5, assign103150_e155021_d_n6, assign103150_e155021_d_n7, assign103150_e155021_d_n8, assign103150_e155021_d_n9, assign103150_e155021_d_n10, assign103150_e155021_d_n11, assign103150_e155021_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103150_e155019: f64 = (locals.var_xp + locals.var_xmp);
        (assign103150_e155019, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign103150_e155021;
        locals.var_arg_dn0 = assign103150_e155021_d_n0;
        locals.var_arg_dn2 = assign103150_e155021_d_n2;
        locals.var_arg_dn4 = assign103150_e155021_d_n4;
        locals.var_arg_dn5 = assign103150_e155021_d_n5;
        locals.var_arg_dn6 = assign103150_e155021_d_n6;
        locals.var_arg_dn7 = assign103150_e155021_d_n7;
        locals.var_arg_dn8 = assign103150_e155021_d_n8;
        locals.var_arg_dn9 = assign103150_e155021_d_n9;
        locals.var_arg_dn10 = assign103150_e155021_d_n10;
        locals.var_arg_dn11 = assign103150_e155021_d_n11;
        locals.var_arg_dn14 = assign103150_e155021_d_n14;

        let (assign103160_e155030, assign103160_e155030_d_n0, assign103160_e155030_d_n2, assign103160_e155030_d_n4, assign103160_e155030_d_n5, assign103160_e155030_d_n6, assign103160_e155030_d_n7, assign103160_e155030_d_n8, assign103160_e155030_d_n9, assign103160_e155030_d_n10, assign103160_e155030_d_n11, assign103160_e155030_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103160_e155030;
        locals.var_dnm_dn0 = assign103160_e155030_d_n0;
        locals.var_dnm_dn2 = assign103160_e155030_d_n2;
        locals.var_dnm_dn4 = assign103160_e155030_d_n4;
        locals.var_dnm_dn5 = assign103160_e155030_d_n5;
        locals.var_dnm_dn6 = assign103160_e155030_d_n6;
        locals.var_dnm_dn7 = assign103160_e155030_d_n7;
        locals.var_dnm_dn8 = assign103160_e155030_d_n8;
        locals.var_dnm_dn9 = assign103160_e155030_d_n9;
        locals.var_dnm_dn10 = assign103160_e155030_d_n10;
        locals.var_dnm_dn11 = assign103160_e155030_d_n11;
        locals.var_dnm_dn14 = assign103160_e155030_d_n14;

        let assign103170_e155045: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2349 = assign103170_e155045;

        let assign103180_e155048: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2350 = assign103180_e155048;

        let (assign103190_e155061,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103190_e155061;

        let assign103200_e155064: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2351 = assign103200_e155064;

        let (assign103210_e155080,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103210_e155080;

        let assign103220_e155083: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2352 = assign103220_e155083;

        let (assign103230_e155102,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 == 0.0)) && (locals.var_guard2352 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103230_e155102;

        let assign103240_e155105: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2353 = assign103240_e155105;

        let (assign103250_e155127,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_guard2350 == 0.0)) && (locals.var_guard2351 == 0.0)) && (locals.var_guard2352 == 0.0)) && (locals.var_guard2353 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign103250_e155127;

        let (assign103260_e155138,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign103260_e155138;

        let mut assign103270_loop_guard: usize = 0;
        while {
            let assign103270_cond_e155150: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign103270_cond_e155150 != 0.0
        } {
            assign103270_loop_guard += 1;
            assert!(assign103270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign103270_body0_e155162, assign103270_body0_e155162_d_n0, assign103270_body0_e155162_d_n2, assign103270_body0_e155162_d_n4, assign103270_body0_e155162_d_n5, assign103270_body0_e155162_d_n6, assign103270_body0_e155162_d_n7, assign103270_body0_e155162_d_n8, assign103270_body0_e155162_d_n9, assign103270_body0_e155162_d_n10, assign103270_body0_e155162_d_n11, assign103270_body0_e155162_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        let assign103270_body0_e155160: f64 = (locals.var_dnm).sqrt();
        (assign103270_body0_e155160, (locals.var_dnm_dn0 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn2 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn4 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn5 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn6 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn7 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn8 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn9 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn10 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn11 / (2.0 * assign103270_body0_e155160)), (locals.var_dnm_dn14 / (2.0 * assign103270_body0_e155160)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign103270_body0_e155162;
            locals.var_dnm_dn0 = assign103270_body0_e155162_d_n0;
            locals.var_dnm_dn2 = assign103270_body0_e155162_d_n2;
            locals.var_dnm_dn4 = assign103270_body0_e155162_d_n4;
            locals.var_dnm_dn5 = assign103270_body0_e155162_d_n5;
            locals.var_dnm_dn6 = assign103270_body0_e155162_d_n6;
            locals.var_dnm_dn7 = assign103270_body0_e155162_d_n7;
            locals.var_dnm_dn8 = assign103270_body0_e155162_d_n8;
            locals.var_dnm_dn9 = assign103270_body0_e155162_d_n9;
            locals.var_dnm_dn10 = assign103270_body0_e155162_d_n10;
            locals.var_dnm_dn11 = assign103270_body0_e155162_d_n11;
            locals.var_dnm_dn14 = assign103270_body0_e155162_d_n14;
            let (assign103270_body1_e155175,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 != 0.0)) {
        let assign103270_body1_e155173: f64 = (locals.var_m0 + 1.0);
        (assign103270_body1_e155173,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign103270_body1_e155175;
        }

        let (assign103280_e155198, assign103280_e155198_d_n0, assign103280_e155198_d_n2, assign103280_e155198_d_n4, assign103280_e155198_d_n5, assign103280_e155198_d_n6, assign103280_e155198_d_n7, assign103280_e155198_d_n8, assign103280_e155198_d_n9, assign103280_e155198_d_n10, assign103280_e155198_d_n11, assign103280_e155198_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) && (locals.var_guard2349 == 0.0)) {
        let (assign103280_e155196, assign103280_e155196_d_n0, assign103280_e155196_d_n2, assign103280_e155196_d_n4, assign103280_e155196_d_n5, assign103280_e155196_d_n6, assign103280_e155196_d_n7, assign103280_e155196_d_n8, assign103280_e155196_d_n9, assign103280_e155196_d_n10, assign103280_e155196_d_n11, assign103280_e155196_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103280_e155193: f64 = (2.0 * 2.0);
                let assign103280_e155194: f64 = (1.0 / assign103280_e155193);
                let assign103280_e155195: f64 = (locals.var_dnm).powf(assign103280_e155194);
                (assign103280_e155195, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn0)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn2)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn4)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn5)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn6)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn7)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn8)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn9)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn10)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn11)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign103280_e155194) as f64).is_finite() && ((assign103280_e155194) as f64).fract() == 0.0 { if assign103280_e155194 == 0.0 { 0.0 } else { (assign103280_e155194 * ((locals.var_dnm).powf(assign103280_e155194 - 1.0) * locals.var_dnm_dn14)) } } else { (assign103280_e155195 * (assign103280_e155194 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign103280_e155196, assign103280_e155196_d_n0, assign103280_e155196_d_n2, assign103280_e155196_d_n4, assign103280_e155196_d_n5, assign103280_e155196_d_n6, assign103280_e155196_d_n7, assign103280_e155196_d_n8, assign103280_e155196_d_n9, assign103280_e155196_d_n10, assign103280_e155196_d_n11, assign103280_e155196_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103280_e155198;
        locals.var_dnm_dn0 = assign103280_e155198_d_n0;
        locals.var_dnm_dn2 = assign103280_e155198_d_n2;
        locals.var_dnm_dn4 = assign103280_e155198_d_n4;
        locals.var_dnm_dn5 = assign103280_e155198_d_n5;
        locals.var_dnm_dn6 = assign103280_e155198_d_n6;
        locals.var_dnm_dn7 = assign103280_e155198_d_n7;
        locals.var_dnm_dn8 = assign103280_e155198_d_n8;
        locals.var_dnm_dn9 = assign103280_e155198_d_n9;
        locals.var_dnm_dn10 = assign103280_e155198_d_n10;
        locals.var_dnm_dn11 = assign103280_e155198_d_n11;
        locals.var_dnm_dn14 = assign103280_e155198_d_n14;

        let (assign103290_e155209, assign103290_e155209_d_n0, assign103290_e155209_d_n2, assign103290_e155209_d_n4, assign103290_e155209_d_n5, assign103290_e155209_d_n6, assign103290_e155209_d_n7, assign103290_e155209_d_n8, assign103290_e155209_d_n9, assign103290_e155209_d_n10, assign103290_e155209_d_n11, assign103290_e155209_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103290_e155207: f64 = (1.0 / locals.var_dnm);
        (assign103290_e155207, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign103290_e155209;
        locals.var_dnm_dn0 = assign103290_e155209_d_n0;
        locals.var_dnm_dn2 = assign103290_e155209_d_n2;
        locals.var_dnm_dn4 = assign103290_e155209_d_n4;
        locals.var_dnm_dn5 = assign103290_e155209_d_n5;
        locals.var_dnm_dn6 = assign103290_e155209_d_n6;
        locals.var_dnm_dn7 = assign103290_e155209_d_n7;
        locals.var_dnm_dn8 = assign103290_e155209_d_n8;
        locals.var_dnm_dn9 = assign103290_e155209_d_n9;
        locals.var_dnm_dn10 = assign103290_e155209_d_n10;
        locals.var_dnm_dn11 = assign103290_e155209_d_n11;
        locals.var_dnm_dn14 = assign103290_e155209_d_n14;

        let (assign103300_e155222, assign103300_e155222_d_n0, assign103300_e155222_d_n2, assign103300_e155222_d_n4, assign103300_e155222_d_n5, assign103300_e155222_d_n6, assign103300_e155222_d_n7, assign103300_e155222_d_n8, assign103300_e155222_d_n9, assign103300_e155222_d_n10, assign103300_e155222_d_n11, assign103300_e155222_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103300_e155218: f64 = (locals.var_tmf1 * 1e-25);
        let assign103300_e155220: f64 = (assign103300_e155218 * locals.var_dnm);
        (assign103300_e155220, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-25) * locals.var_dnm) + (assign103300_e155218 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign103300_e155222;
        locals.var_tmf0_dn0 = assign103300_e155222_d_n0;
        locals.var_tmf0_dn2 = assign103300_e155222_d_n2;
        locals.var_tmf0_dn4 = assign103300_e155222_d_n4;
        locals.var_tmf0_dn5 = assign103300_e155222_d_n5;
        locals.var_tmf0_dn6 = assign103300_e155222_d_n6;
        locals.var_tmf0_dn7 = assign103300_e155222_d_n7;
        locals.var_tmf0_dn8 = assign103300_e155222_d_n8;
        locals.var_tmf0_dn9 = assign103300_e155222_d_n9;
        locals.var_tmf0_dn10 = assign103300_e155222_d_n10;
        locals.var_tmf0_dn11 = assign103300_e155222_d_n11;
        locals.var_tmf0_dn14 = assign103300_e155222_d_n14;

        let (assign103310_e155237, assign103310_e155237_d_n0, assign103310_e155237_d_n2, assign103310_e155237_d_n4, assign103310_e155237_d_n5, assign103310_e155237_d_n6, assign103310_e155237_d_n7, assign103310_e155237_d_n8, assign103310_e155237_d_n9, assign103310_e155237_d_n10, assign103310_e155237_d_n11, assign103310_e155237_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103310_e155231: f64 = (1e-25 * locals.var_xmp);
        let assign103310_e155233: f64 = (assign103310_e155231 * locals.var_dnm);
        let assign103310_e155235: f64 = (assign103310_e155233 / locals.var_arg);
        (assign103310_e155235, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn0)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn2)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn4)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn5)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn6)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn7)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn8)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn9)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn10)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn11) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn11)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn14) * locals.var_dnm) + (assign103310_e155231 * locals.var_dnm_dn14)) * locals.var_arg) - (assign103310_e155233 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103310_e155237;
        locals.var_t0_dn0 = assign103310_e155237_d_n0;
        locals.var_t0_dn2 = assign103310_e155237_d_n2;
        locals.var_t0_dn4 = assign103310_e155237_d_n4;
        locals.var_t0_dn5 = assign103310_e155237_d_n5;
        locals.var_t0_dn6 = assign103310_e155237_d_n6;
        locals.var_t0_dn7 = assign103310_e155237_d_n7;
        locals.var_t0_dn8 = assign103310_e155237_d_n8;
        locals.var_t0_dn9 = assign103310_e155237_d_n9;
        locals.var_t0_dn10 = assign103310_e155237_d_n10;
        locals.var_t0_dn11 = assign103310_e155237_d_n11;
        locals.var_t0_dn14 = assign103310_e155237_d_n14;

        let (assign103320_e155250, assign103320_e155250_d_n0, assign103320_e155250_d_n2, assign103320_e155250_d_n4, assign103320_e155250_d_n5, assign103320_e155250_d_n6, assign103320_e155250_d_n7, assign103320_e155250_d_n8, assign103320_e155250_d_n9, assign103320_e155250_d_n10, assign103320_e155250_d_n11, assign103320_e155250_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        let assign103320_e155246: f64 = 1e-25;
        let assign103320_e155248: f64 = (assign103320_e155246 - locals.var_tmf0);
        (assign103320_e155248, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103320_e155250;
        locals.var_gd_s_dn0 = assign103320_e155250_d_n0;
        locals.var_gd_s_dn2 = assign103320_e155250_d_n2;
        locals.var_gd_s_dn4 = assign103320_e155250_d_n4;
        locals.var_gd_s_dn5 = assign103320_e155250_d_n5;
        locals.var_gd_s_dn6 = assign103320_e155250_d_n6;
        locals.var_gd_s_dn7 = assign103320_e155250_d_n7;
        locals.var_gd_s_dn8 = assign103320_e155250_d_n8;
        locals.var_gd_s_dn9 = assign103320_e155250_d_n9;
        locals.var_gd_s_dn10 = assign103320_e155250_d_n10;
        locals.var_gd_s_dn11 = assign103320_e155250_d_n11;
        locals.var_gd_s_dn14 = assign103320_e155250_d_n14;

        let (assign103330_e155259, assign103330_e155259_d_n0, assign103330_e155259_d_n2, assign103330_e155259_d_n4, assign103330_e155259_d_n5, assign103330_e155259_d_n6, assign103330_e155259_d_n7, assign103330_e155259_d_n8, assign103330_e155259_d_n9, assign103330_e155259_d_n10, assign103330_e155259_d_n11, assign103330_e155259_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103330_e155259;
        locals.var_t0_dn0 = assign103330_e155259_d_n0;
        locals.var_t0_dn2 = assign103330_e155259_d_n2;
        locals.var_t0_dn4 = assign103330_e155259_d_n4;
        locals.var_t0_dn5 = assign103330_e155259_d_n5;
        locals.var_t0_dn6 = assign103330_e155259_d_n6;
        locals.var_t0_dn7 = assign103330_e155259_d_n7;
        locals.var_t0_dn8 = assign103330_e155259_d_n8;
        locals.var_t0_dn9 = assign103330_e155259_d_n9;
        locals.var_t0_dn10 = assign103330_e155259_d_n10;
        locals.var_t0_dn11 = assign103330_e155259_d_n11;
        locals.var_t0_dn14 = assign103330_e155259_d_n14;

        let (assign103340_e155269, assign103340_e155269_d_n0, assign103340_e155269_d_n2, assign103340_e155269_d_n4, assign103340_e155269_d_n5, assign103340_e155269_d_n6, assign103340_e155269_d_n7, assign103340_e155269_d_n8, assign103340_e155269_d_n9, assign103340_e155269_d_n10, assign103340_e155269_d_n11, assign103340_e155269_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 == 0.0)) {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    } else {
        (locals.var_gd_s, locals.var_gd_s_dn0, locals.var_gd_s_dn2, locals.var_gd_s_dn4, locals.var_gd_s_dn5, locals.var_gd_s_dn6, locals.var_gd_s_dn7, locals.var_gd_s_dn8, locals.var_gd_s_dn9, locals.var_gd_s_dn10, locals.var_gd_s_dn11, locals.var_gd_s_dn14,)
    }
};
        locals.var_gd_s = assign103340_e155269;
        locals.var_gd_s_dn0 = assign103340_e155269_d_n0;
        locals.var_gd_s_dn2 = assign103340_e155269_d_n2;
        locals.var_gd_s_dn4 = assign103340_e155269_d_n4;
        locals.var_gd_s_dn5 = assign103340_e155269_d_n5;
        locals.var_gd_s_dn6 = assign103340_e155269_d_n6;
        locals.var_gd_s_dn7 = assign103340_e155269_d_n7;
        locals.var_gd_s_dn8 = assign103340_e155269_d_n8;
        locals.var_gd_s_dn9 = assign103340_e155269_d_n9;
        locals.var_gd_s_dn10 = assign103340_e155269_d_n10;
        locals.var_gd_s_dn11 = assign103340_e155269_d_n11;
        locals.var_gd_s_dn14 = assign103340_e155269_d_n14;

        let (assign103350_e155279, assign103350_e155279_d_n0, assign103350_e155279_d_n2, assign103350_e155279_d_n4, assign103350_e155279_d_n5, assign103350_e155279_d_n6, assign103350_e155279_d_n7, assign103350_e155279_d_n8, assign103350_e155279_d_n9, assign103350_e155279_d_n10, assign103350_e155279_d_n11, assign103350_e155279_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2348 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103350_e155279;
        locals.var_t0_dn0 = assign103350_e155279_d_n0;
        locals.var_t0_dn2 = assign103350_e155279_d_n2;
        locals.var_t0_dn4 = assign103350_e155279_d_n4;
        locals.var_t0_dn5 = assign103350_e155279_d_n5;
        locals.var_t0_dn6 = assign103350_e155279_d_n6;
        locals.var_t0_dn7 = assign103350_e155279_d_n7;
        locals.var_t0_dn8 = assign103350_e155279_d_n8;
        locals.var_t0_dn9 = assign103350_e155279_d_n9;
        locals.var_t0_dn10 = assign103350_e155279_d_n10;
        locals.var_t0_dn11 = assign103350_e155279_d_n11;
        locals.var_t0_dn14 = assign103350_e155279_d_n14;

        let (assign103360_e155288, assign103360_e155288_d_n0, assign103360_e155288_d_n2, assign103360_e155288_d_n4, assign103360_e155288_d_n5, assign103360_e155288_d_n6, assign103360_e155288_d_n7, assign103360_e155288_d_n8, assign103360_e155288_d_n9, assign103360_e155288_d_n10, assign103360_e155288_d_n11, assign103360_e155288_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103360_e155286: f64 = (1.0 / locals.var_gd_s);
        (assign103360_e155286, (-(locals.var_gd_s_dn0 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn2 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn4 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn5 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn6 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn7 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn8 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn9 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn10 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn11 / (locals.var_gd_s * locals.var_gd_s))), (-(locals.var_gd_s_dn14 / (locals.var_gd_s * locals.var_gd_s))),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103360_e155288;
        locals.var_rsd_dn0 = assign103360_e155288_d_n0;
        locals.var_rsd_dn2 = assign103360_e155288_d_n2;
        locals.var_rsd_dn4 = assign103360_e155288_d_n4;
        locals.var_rsd_dn5 = assign103360_e155288_d_n5;
        locals.var_rsd_dn6 = assign103360_e155288_d_n6;
        locals.var_rsd_dn7 = assign103360_e155288_d_n7;
        locals.var_rsd_dn8 = assign103360_e155288_d_n8;
        locals.var_rsd_dn9 = assign103360_e155288_d_n9;
        locals.var_rsd_dn10 = assign103360_e155288_d_n10;
        locals.var_rsd_dn11 = assign103360_e155288_d_n11;
        locals.var_rsd_dn14 = assign103360_e155288_d_n14;

        let (assign103370_e155297, assign103370_e155297_d_n0, assign103370_e155297_d_n2, assign103370_e155297_d_n4, assign103370_e155297_d_n5, assign103370_e155297_d_n6, assign103370_e155297_d_n7, assign103370_e155297_d_n8, assign103370_e155297_d_n9, assign103370_e155297_d_n10, assign103370_e155297_d_n11, assign103370_e155297_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103370_e155295: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign103370_e155295, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103370_e155297;
        locals.var_rsd_dn0 = assign103370_e155297_d_n0;
        locals.var_rsd_dn2 = assign103370_e155297_d_n2;
        locals.var_rsd_dn4 = assign103370_e155297_d_n4;
        locals.var_rsd_dn5 = assign103370_e155297_d_n5;
        locals.var_rsd_dn6 = assign103370_e155297_d_n6;
        locals.var_rsd_dn7 = assign103370_e155297_d_n7;
        locals.var_rsd_dn8 = assign103370_e155297_d_n8;
        locals.var_rsd_dn9 = assign103370_e155297_d_n9;
        locals.var_rsd_dn10 = assign103370_e155297_d_n10;
        locals.var_rsd_dn11 = assign103370_e155297_d_n11;
        locals.var_rsd_dn14 = assign103370_e155297_d_n14;

        let (assign103380_e155306, assign103380_e155306_d_n0, assign103380_e155306_d_n2, assign103380_e155306_d_n4, assign103380_e155306_d_n5, assign103380_e155306_d_n6, assign103380_e155306_d_n7, assign103380_e155306_d_n8, assign103380_e155306_d_n9, assign103380_e155306_d_n10, assign103380_e155306_d_n11, assign103380_e155306_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103380_e155304: f64 = (locals.var_rsd + locals.var_rs0);
        (assign103380_e155304, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103380_e155306;
        locals.var_rsd_dn0 = assign103380_e155306_d_n0;
        locals.var_rsd_dn2 = assign103380_e155306_d_n2;
        locals.var_rsd_dn4 = assign103380_e155306_d_n4;
        locals.var_rsd_dn5 = assign103380_e155306_d_n5;
        locals.var_rsd_dn6 = assign103380_e155306_d_n6;
        locals.var_rsd_dn7 = assign103380_e155306_d_n7;
        locals.var_rsd_dn8 = assign103380_e155306_d_n8;
        locals.var_rsd_dn9 = assign103380_e155306_d_n9;
        locals.var_rsd_dn10 = assign103380_e155306_d_n10;
        locals.var_rsd_dn11 = assign103380_e155306_d_n11;
        locals.var_rsd_dn14 = assign103380_e155306_d_n14;

        let assign103420_e155337: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2355 = assign103420_e155337;

        let (assign103430_e155346, assign103430_e155346_d_n0, assign103430_e155346_d_n2, assign103430_e155346_d_n4, assign103430_e155346_d_n5, assign103430_e155346_d_n6, assign103430_e155346_d_n7, assign103430_e155346_d_n8, assign103430_e155346_d_n9, assign103430_e155346_d_n10, assign103430_e155346_d_n11, assign103430_e155346_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) && (locals.var_guard2355 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign103430_e155346;
        locals.var_rsd_dn0 = assign103430_e155346_d_n0;
        locals.var_rsd_dn2 = assign103430_e155346_d_n2;
        locals.var_rsd_dn4 = assign103430_e155346_d_n4;
        locals.var_rsd_dn5 = assign103430_e155346_d_n5;
        locals.var_rsd_dn6 = assign103430_e155346_d_n6;
        locals.var_rsd_dn7 = assign103430_e155346_d_n7;
        locals.var_rsd_dn8 = assign103430_e155346_d_n8;
        locals.var_rsd_dn9 = assign103430_e155346_d_n9;
        locals.var_rsd_dn10 = assign103430_e155346_d_n10;
        locals.var_rsd_dn11 = assign103430_e155346_d_n11;
        locals.var_rsd_dn14 = assign103430_e155346_d_n14;

        let (assign103440_e155355, assign103440_e155355_d_n0, assign103440_e155355_d_n2, assign103440_e155355_d_n4, assign103440_e155355_d_n5, assign103440_e155355_d_n6, assign103440_e155355_d_n7, assign103440_e155355_d_n8, assign103440_e155355_d_n9, assign103440_e155355_d_n10, assign103440_e155355_d_n11, assign103440_e155355_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2341 == 0.0)) {
        let assign103440_e155353: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign103440_e155353, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn11 / locals.var_mfactor), (locals.var_rsd_dn14 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn11, locals.var_rsde_dn14,)
    }
};
        locals.var_rsde = assign103440_e155355;
        locals.var_rsde_dn0 = assign103440_e155355_d_n0;
        locals.var_rsde_dn2 = assign103440_e155355_d_n2;
        locals.var_rsde_dn4 = assign103440_e155355_d_n4;
        locals.var_rsde_dn5 = assign103440_e155355_d_n5;
        locals.var_rsde_dn6 = assign103440_e155355_d_n6;
        locals.var_rsde_dn7 = assign103440_e155355_d_n7;
        locals.var_rsde_dn8 = assign103440_e155355_d_n8;
        locals.var_rsde_dn9 = assign103440_e155355_d_n9;
        locals.var_rsde_dn10 = assign103440_e155355_d_n10;
        locals.var_rsde_dn11 = assign103440_e155355_d_n11;
        locals.var_rsde_dn14 = assign103440_e155355_d_n14;

        let assign103450_e155358: f64 = if locals.var_flg_rd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2360 = assign103450_e155358;

    }

    pub(super) fn stamp_transient_block_380(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign103460_e155365, assign103460_e155365_d_n6, assign103460_e155365_d_n8,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_vdsi, locals.var_vdsi_dn6, locals.var_vdsi_dn8,)
    } else {
        (locals.var_vds__blk2356, locals.var_vds__blk2356_dn6, locals.var_vds__blk2356_dn8,)
    }
};
        locals.var_vds__blk2356 = assign103460_e155365;
        locals.var_vds__blk2356_dn6 = assign103460_e155365_d_n6;
        locals.var_vds__blk2356_dn8 = assign103460_e155365_d_n8;

        let (assign103470_e155372, assign103470_e155372_d_n8, assign103470_e155372_d_n9,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_vbsi, locals.var_vbsi_dn8, locals.var_vbsi_dn9,)
    } else {
        (locals.var_vbs__blk2357, locals.var_vbs__blk2357_dn8, locals.var_vbs__blk2357_dn9,)
    }
};
        locals.var_vbs__blk2357 = assign103470_e155372;
        locals.var_vbs__blk2357_dn8 = assign103470_e155372_d_n8;
        locals.var_vbs__blk2357_dn9 = assign103470_e155372_d_n9;

        let assign103480_e155379: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2361 = assign103480_e155379;

        let (assign103490_e155395, assign103490_e155395_d_n0, assign103490_e155395_d_n2, assign103490_e155395_d_n4, assign103490_e155395_d_n5, assign103490_e155395_d_n6, assign103490_e155395_d_n7, assign103490_e155395_d_n8, assign103490_e155395_d_n9, assign103490_e155395_d_n10, assign103490_e155395_d_n11, assign103490_e155395_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let (assign103490_e155393, assign103490_e155393_d_n0, assign103490_e155393_d_n2, assign103490_e155393_d_n4, assign103490_e155393_d_n5, assign103490_e155393_d_n6, assign103490_e155393_d_n7, assign103490_e155393_d_n8, assign103490_e155393_d_n9, assign103490_e155393_d_n10, assign103490_e155393_d_n11, assign103490_e155393_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign103490_e155392: f64 = (locals.var_tratio).powf(p.p415);
                (assign103490_e155392, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn11)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn14)) } } else { (assign103490_e155392 * (p.p415 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign103490_e155393, assign103490_e155393_d_n0, assign103490_e155393_d_n2, assign103490_e155393_d_n4, assign103490_e155393_d_n5, assign103490_e155393_d_n6, assign103490_e155393_d_n7, assign103490_e155393_d_n8, assign103490_e155393_d_n9, assign103490_e155393_d_n10, assign103490_e155393_d_n11, assign103490_e155393_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103490_e155395;
        locals.var_t1_dn0 = assign103490_e155395_d_n0;
        locals.var_t1_dn2 = assign103490_e155395_d_n2;
        locals.var_t1_dn4 = assign103490_e155395_d_n4;
        locals.var_t1_dn5 = assign103490_e155395_d_n5;
        locals.var_t1_dn6 = assign103490_e155395_d_n6;
        locals.var_t1_dn7 = assign103490_e155395_d_n7;
        locals.var_t1_dn8 = assign103490_e155395_d_n8;
        locals.var_t1_dn9 = assign103490_e155395_d_n9;
        locals.var_t1_dn10 = assign103490_e155395_d_n10;
        locals.var_t1_dn11 = assign103490_e155395_d_n11;
        locals.var_t1_dn14 = assign103490_e155395_d_n14;

        let (assign103500_e155406, assign103500_e155406_d_n0, assign103500_e155406_d_n2, assign103500_e155406_d_n4, assign103500_e155406_d_n5, assign103500_e155406_d_n6, assign103500_e155406_d_n7, assign103500_e155406_d_n8, assign103500_e155406_d_n9, assign103500_e155406_d_n10, assign103500_e155406_d_n11, assign103500_e155406_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103500_e155404: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign103500_e155404, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn11, locals.var_rrdrmue_dn14,)
    }
};
        locals.var_rrdrmue = assign103500_e155406;
        locals.var_rrdrmue_dn0 = assign103500_e155406_d_n0;
        locals.var_rrdrmue_dn2 = assign103500_e155406_d_n2;
        locals.var_rrdrmue_dn4 = assign103500_e155406_d_n4;
        locals.var_rrdrmue_dn5 = assign103500_e155406_d_n5;
        locals.var_rrdrmue_dn6 = assign103500_e155406_d_n6;
        locals.var_rrdrmue_dn7 = assign103500_e155406_d_n7;
        locals.var_rrdrmue_dn8 = assign103500_e155406_d_n8;
        locals.var_rrdrmue_dn9 = assign103500_e155406_d_n9;
        locals.var_rrdrmue_dn10 = assign103500_e155406_d_n10;
        locals.var_rrdrmue_dn11 = assign103500_e155406_d_n11;
        locals.var_rrdrmue_dn14 = assign103500_e155406_d_n14;

        let (assign103510_e155431, assign103510_e155431_d_n0, assign103510_e155431_d_n2, assign103510_e155431_d_n4, assign103510_e155431_d_n5, assign103510_e155431_d_n6, assign103510_e155431_d_n7, assign103510_e155431_d_n8, assign103510_e155431_d_n9, assign103510_e155431_d_n10, assign103510_e155431_d_n11, assign103510_e155431_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103510_e155416: f64 = (0.4 * locals.var_tratio);
        let assign103510_e155417: f64 = (1.8 + assign103510_e155416);
        let assign103510_e155420: f64 = (0.1 * locals.var_tratio);
        let assign103510_e155422: f64 = (assign103510_e155420 * locals.var_tratio);
        let assign103510_e155423: f64 = (assign103510_e155417 + assign103510_e155422);
        let assign103510_e155427: f64 = (1.0 - locals.var_tratio);
        let assign103510_e155428: f64 = (p.p417 * assign103510_e155427);
        let assign103510_e155429: f64 = (assign103510_e155423 - assign103510_e155428);
        (assign103510_e155429, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn11))) - (p.p417 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign103510_e155420 * locals.var_tratio_dn14))) - (p.p417 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign103510_e155431;
        locals.var_t0_dn0 = assign103510_e155431_d_n0;
        locals.var_t0_dn2 = assign103510_e155431_d_n2;
        locals.var_t0_dn4 = assign103510_e155431_d_n4;
        locals.var_t0_dn5 = assign103510_e155431_d_n5;
        locals.var_t0_dn6 = assign103510_e155431_d_n6;
        locals.var_t0_dn7 = assign103510_e155431_d_n7;
        locals.var_t0_dn8 = assign103510_e155431_d_n8;
        locals.var_t0_dn9 = assign103510_e155431_d_n9;
        locals.var_t0_dn10 = assign103510_e155431_d_n10;
        locals.var_t0_dn11 = assign103510_e155431_d_n11;
        locals.var_t0_dn14 = assign103510_e155431_d_n14;

        let (assign103520_e155442, assign103520_e155442_d_n0, assign103520_e155442_d_n2, assign103520_e155442_d_n4, assign103520_e155442_d_n5, assign103520_e155442_d_n6, assign103520_e155442_d_n7, assign103520_e155442_d_n8, assign103520_e155442_d_n9, assign103520_e155442_d_n10, assign103520_e155442_d_n11, assign103520_e155442_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103520_e155440: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign103520_e155440, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn11, locals.var_rrdrvmax_dn14,)
    }
};
        locals.var_rrdrvmax = assign103520_e155442;
        locals.var_rrdrvmax_dn0 = assign103520_e155442_d_n0;
        locals.var_rrdrvmax_dn2 = assign103520_e155442_d_n2;
        locals.var_rrdrvmax_dn4 = assign103520_e155442_d_n4;
        locals.var_rrdrvmax_dn5 = assign103520_e155442_d_n5;
        locals.var_rrdrvmax_dn6 = assign103520_e155442_d_n6;
        locals.var_rrdrvmax_dn7 = assign103520_e155442_d_n7;
        locals.var_rrdrvmax_dn8 = assign103520_e155442_d_n8;
        locals.var_rrdrvmax_dn9 = assign103520_e155442_d_n9;
        locals.var_rrdrvmax_dn10 = assign103520_e155442_d_n10;
        locals.var_rrdrvmax_dn11 = assign103520_e155442_d_n11;
        locals.var_rrdrvmax_dn14 = assign103520_e155442_d_n14;

        let (assign103530_e155457, assign103530_e155457_d_n0, assign103530_e155457_d_n2, assign103530_e155457_d_n4, assign103530_e155457_d_n5, assign103530_e155457_d_n6, assign103530_e155457_d_n7, assign103530_e155457_d_n8, assign103530_e155457_d_n9, assign103530_e155457_d_n10, assign103530_e155457_d_n11, assign103530_e155457_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) {
        let assign103530_e155453: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign103530_e155454: f64 = (p.p438 * assign103530_e155453);
        let assign103530_e155455: f64 = (locals.var_uc_rdrbb + assign103530_e155454);
        (assign103530_e155455, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn11 + (p.p438 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_dn14 + (p.p438 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103530_e155457;
        locals.var_uc_rdrbb_dn0 = assign103530_e155457_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103530_e155457_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103530_e155457_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103530_e155457_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103530_e155457_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103530_e155457_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103530_e155457_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103530_e155457_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103530_e155457_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103530_e155457_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103530_e155457_d_n14;

        let assign103550_e155465: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard2363 = assign103550_e155465;

        let (assign103560_e155476, assign103560_e155476_d_n0, assign103560_e155476_d_n2, assign103560_e155476_d_n4, assign103560_e155476_d_n5, assign103560_e155476_d_n6, assign103560_e155476_d_n7, assign103560_e155476_d_n8, assign103560_e155476_d_n9, assign103560_e155476_d_n10, assign103560_e155476_d_n11, assign103560_e155476_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 != 0.0)) && (locals.var_guard2363 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign103560_e155476;
        locals.var_uc_rdrbb_dn0 = assign103560_e155476_d_n0;
        locals.var_uc_rdrbb_dn2 = assign103560_e155476_d_n2;
        locals.var_uc_rdrbb_dn4 = assign103560_e155476_d_n4;
        locals.var_uc_rdrbb_dn5 = assign103560_e155476_d_n5;
        locals.var_uc_rdrbb_dn6 = assign103560_e155476_d_n6;
        locals.var_uc_rdrbb_dn7 = assign103560_e155476_d_n7;
        locals.var_uc_rdrbb_dn8 = assign103560_e155476_d_n8;
        locals.var_uc_rdrbb_dn9 = assign103560_e155476_d_n9;
        locals.var_uc_rdrbb_dn10 = assign103560_e155476_d_n10;
        locals.var_uc_rdrbb_dn11 = assign103560_e155476_d_n11;
        locals.var_uc_rdrbb_dn14 = assign103560_e155476_d_n14;

        let (assign103570_e155488, assign103570_e155488_d_n0, assign103570_e155488_d_n2, assign103570_e155488_d_n4, assign103570_e155488_d_n5, assign103570_e155488_d_n6, assign103570_e155488_d_n7, assign103570_e155488_d_n8, assign103570_e155488_d_n9, assign103570_e155488_d_n10, assign103570_e155488_d_n11, assign103570_e155488_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2361 == 0.0)) {
        let assign103570_e155484: f64 = ctx_temp;
        let assign103570_e155486: f64 = (assign103570_e155484 + p.p11);
        (assign103570_e155486, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign103570_e155488;
        locals.var_ttemp_dn0 = assign103570_e155488_d_n0;
        locals.var_ttemp_dn2 = assign103570_e155488_d_n2;
        locals.var_ttemp_dn4 = assign103570_e155488_d_n4;
        locals.var_ttemp_dn5 = assign103570_e155488_d_n5;
        locals.var_ttemp_dn6 = assign103570_e155488_d_n6;
        locals.var_ttemp_dn7 = assign103570_e155488_d_n7;
        locals.var_ttemp_dn8 = assign103570_e155488_d_n8;
        locals.var_ttemp_dn9 = assign103570_e155488_d_n9;
        locals.var_ttemp_dn10 = assign103570_e155488_d_n10;
        locals.var_ttemp_dn11 = assign103570_e155488_d_n11;
        locals.var_ttemp_dn14 = assign103570_e155488_d_n14;

        let (assign103580_e155497,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103580_e155495: f64 = (locals.var_weff_ld * p.p7);
        (assign103580_e155495,)
    } else {
        (locals.var_weffld_nf,)
    }
};
        locals.var_weffld_nf = assign103580_e155497;

        let (assign103590_e155506,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103590_e155504: f64 = (p.p67 + p.p68);
        (assign103590_e155504,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign103590_e155506;

        let (assign103600_e155515,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103600_e155513: f64 = (locals.var_uc_xldld + 1e-12);
        (assign103600_e155513,)
    } else {
        (locals.var_rd_xldld,)
    }
};
        locals.var_rd_xldld = assign103600_e155515;

        let (assign103610_e155522,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_noverd,)
    }
};
        locals.var_noverd = assign103610_e155522;

        let (assign103620_e155537, assign103620_e155537_d_n0, assign103620_e155537_d_n2, assign103620_e155537_d_n4, assign103620_e155537_d_n5, assign103620_e155537_d_n6, assign103620_e155537_d_n7, assign103620_e155537_d_n8, assign103620_e155537_d_n9, assign103620_e155537_d_n10, assign103620_e155537_d_n11, assign103620_e155537_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103620_e155532: f64 = (p.p411 * locals.var_vbs__blk2357);
        let assign103620_e155533: f64 = (p.p410 - assign103620_e155532);
        let assign103620_e155534: f64 = (locals.var_vbs__blk2357 * assign103620_e155533);
        let assign103620_e155535: f64 = (1.0 + assign103620_e155534);
        (assign103620_e155535, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ((locals.var_vbs__blk2357_dn8 * assign103620_e155533) + (locals.var_vbs__blk2357 * (-(p.p411 * locals.var_vbs__blk2357_dn8)))), ((locals.var_vbs__blk2357_dn9 * assign103620_e155533) + (locals.var_vbs__blk2357 * (-(p.p411 * locals.var_vbs__blk2357_dn9)))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103620_e155537;
        locals.var_t1_dn0 = assign103620_e155537_d_n0;
        locals.var_t1_dn2 = assign103620_e155537_d_n2;
        locals.var_t1_dn4 = assign103620_e155537_d_n4;
        locals.var_t1_dn5 = assign103620_e155537_d_n5;
        locals.var_t1_dn6 = assign103620_e155537_d_n6;
        locals.var_t1_dn7 = assign103620_e155537_d_n7;
        locals.var_t1_dn8 = assign103620_e155537_d_n8;
        locals.var_t1_dn9 = assign103620_e155537_d_n9;
        locals.var_t1_dn10 = assign103620_e155537_d_n10;
        locals.var_t1_dn11 = assign103620_e155537_d_n11;
        locals.var_t1_dn14 = assign103620_e155537_d_n14;

        let (assign103630_e155553, assign103630_e155553_d_n0, assign103630_e155553_d_n2, assign103630_e155553_d_n4, assign103630_e155553_d_n5, assign103630_e155553_d_n6, assign103630_e155553_d_n7, assign103630_e155553_d_n8, assign103630_e155553_d_n9, assign103630_e155553_d_n10, assign103630_e155553_d_n11, assign103630_e155553_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103630_e155544: f64 = (locals.var_t1 * locals.var_t1);
        let assign103630_e155547: f64 = (4.0 * 0.1);
        let assign103630_e155549: f64 = (assign103630_e155547 * 0.1);
        let assign103630_e155550: f64 = (assign103630_e155544 + assign103630_e155549);
        let assign103630_e155551: f64 = (assign103630_e155550).sqrt();
        (assign103630_e155551, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign103630_e155551)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign103630_e155551)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103630_e155553;
        locals.var_tmf2_dn0 = assign103630_e155553_d_n0;
        locals.var_tmf2_dn2 = assign103630_e155553_d_n2;
        locals.var_tmf2_dn4 = assign103630_e155553_d_n4;
        locals.var_tmf2_dn5 = assign103630_e155553_d_n5;
        locals.var_tmf2_dn6 = assign103630_e155553_d_n6;
        locals.var_tmf2_dn7 = assign103630_e155553_d_n7;
        locals.var_tmf2_dn8 = assign103630_e155553_d_n8;
        locals.var_tmf2_dn9 = assign103630_e155553_d_n9;
        locals.var_tmf2_dn10 = assign103630_e155553_d_n10;
        locals.var_tmf2_dn11 = assign103630_e155553_d_n11;
        locals.var_tmf2_dn14 = assign103630_e155553_d_n14;

        let (assign103640_e155566, assign103640_e155566_d_n0, assign103640_e155566_d_n2, assign103640_e155566_d_n4, assign103640_e155566_d_n5, assign103640_e155566_d_n6, assign103640_e155566_d_n7, assign103640_e155566_d_n8, assign103640_e155566_d_n9, assign103640_e155566_d_n10, assign103640_e155566_d_n11, assign103640_e155566_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103640_e155562: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign103640_e155563: f64 = (1.0 + assign103640_e155562);
        let assign103640_e155564: f64 = (0.5 * assign103640_e155563);
        (assign103640_e155564, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103640_e155566;
        locals.var_t2_dn0 = assign103640_e155566_d_n0;
        locals.var_t2_dn2 = assign103640_e155566_d_n2;
        locals.var_t2_dn4 = assign103640_e155566_d_n4;
        locals.var_t2_dn5 = assign103640_e155566_d_n5;
        locals.var_t2_dn6 = assign103640_e155566_d_n6;
        locals.var_t2_dn7 = assign103640_e155566_d_n7;
        locals.var_t2_dn8 = assign103640_e155566_d_n8;
        locals.var_t2_dn9 = assign103640_e155566_d_n9;
        locals.var_t2_dn10 = assign103640_e155566_d_n10;
        locals.var_t2_dn11 = assign103640_e155566_d_n11;
        locals.var_t2_dn14 = assign103640_e155566_d_n14;

        let (assign103650_e155577, assign103650_e155577_d_n0, assign103650_e155577_d_n2, assign103650_e155577_d_n4, assign103650_e155577_d_n5, assign103650_e155577_d_n6, assign103650_e155577_d_n7, assign103650_e155577_d_n8, assign103650_e155577_d_n9, assign103650_e155577_d_n10, assign103650_e155577_d_n11, assign103650_e155577_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103650_e155574: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign103650_e155575: f64 = (0.5 * assign103650_e155574);
        (assign103650_e155575, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103650_e155577;
        locals.var_rdrmuevbs_dn0 = assign103650_e155577_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103650_e155577_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103650_e155577_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103650_e155577_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103650_e155577_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103650_e155577_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103650_e155577_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103650_e155577_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103650_e155577_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103650_e155577_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103650_e155577_d_n14;

        let assign103660_e155580: f64 = if locals.var_rdrmuevbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2364 = assign103660_e155580;

        let (assign103670_e155589, assign103670_e155589_d_n0, assign103670_e155589_d_n2, assign103670_e155589_d_n4, assign103670_e155589_d_n5, assign103670_e155589_d_n6, assign103670_e155589_d_n7, assign103670_e155589_d_n8, assign103670_e155589_d_n9, assign103670_e155589_d_n10, assign103670_e155589_d_n11, assign103670_e155589_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2364 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign103670_e155589;
        locals.var_rdrmuevbs_dn0 = assign103670_e155589_d_n0;
        locals.var_rdrmuevbs_dn2 = assign103670_e155589_d_n2;
        locals.var_rdrmuevbs_dn4 = assign103670_e155589_d_n4;
        locals.var_rdrmuevbs_dn5 = assign103670_e155589_d_n5;
        locals.var_rdrmuevbs_dn6 = assign103670_e155589_d_n6;
        locals.var_rdrmuevbs_dn7 = assign103670_e155589_d_n7;
        locals.var_rdrmuevbs_dn8 = assign103670_e155589_d_n8;
        locals.var_rdrmuevbs_dn9 = assign103670_e155589_d_n9;
        locals.var_rdrmuevbs_dn10 = assign103670_e155589_d_n10;
        locals.var_rdrmuevbs_dn11 = assign103670_e155589_d_n11;
        locals.var_rdrmuevbs_dn14 = assign103670_e155589_d_n14;

        let (assign103680_e155598, assign103680_e155598_d_n0, assign103680_e155598_d_n2, assign103680_e155598_d_n4, assign103680_e155598_d_n5, assign103680_e155598_d_n6, assign103680_e155598_d_n7, assign103680_e155598_d_n8, assign103680_e155598_d_n9, assign103680_e155598_d_n10, assign103680_e155598_d_n11, assign103680_e155598_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2364 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103680_e155598;
        locals.var_t2_dn0 = assign103680_e155598_d_n0;
        locals.var_t2_dn2 = assign103680_e155598_d_n2;
        locals.var_t2_dn4 = assign103680_e155598_d_n4;
        locals.var_t2_dn5 = assign103680_e155598_d_n5;
        locals.var_t2_dn6 = assign103680_e155598_d_n6;
        locals.var_t2_dn7 = assign103680_e155598_d_n7;
        locals.var_t2_dn8 = assign103680_e155598_d_n8;
        locals.var_t2_dn9 = assign103680_e155598_d_n9;
        locals.var_t2_dn10 = assign103680_e155598_d_n10;
        locals.var_t2_dn11 = assign103680_e155598_d_n11;
        locals.var_t2_dn14 = assign103680_e155598_d_n14;

        let (assign103690_e155609, assign103690_e155609_d_n0, assign103690_e155609_d_n2, assign103690_e155609_d_n4, assign103690_e155609_d_n5, assign103690_e155609_d_n6, assign103690_e155609_d_n7, assign103690_e155609_d_n8, assign103690_e155609_d_n9, assign103690_e155609_d_n10, assign103690_e155609_d_n11, assign103690_e155609_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103690_e155605: f64 = (locals.var_rrdrmue * locals.var_rdrmuele);
        let assign103690_e155607: f64 = (assign103690_e155605 * locals.var_rdrmuevbs);
        (assign103690_e155607, (((locals.var_rrdrmue_dn0 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn0)), (((locals.var_rrdrmue_dn2 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn2)), (((locals.var_rrdrmue_dn4 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn4)), (((locals.var_rrdrmue_dn5 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn5)), (((locals.var_rrdrmue_dn6 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn6)), (((locals.var_rrdrmue_dn7 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn7)), (((locals.var_rrdrmue_dn8 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn8)), (((locals.var_rrdrmue_dn9 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn9)), (((locals.var_rrdrmue_dn10 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn10)), (((locals.var_rrdrmue_dn11 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn11)), (((locals.var_rrdrmue_dn14 * locals.var_rdrmuele) * locals.var_rdrmuevbs) + (assign103690_e155605 * locals.var_rdrmuevbs_dn14)),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn8, locals.var_mu0_dn9, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn14,)
    }
};
        locals.var_mu0 = assign103690_e155609;
        locals.var_mu0_dn0 = assign103690_e155609_d_n0;
        locals.var_mu0_dn2 = assign103690_e155609_d_n2;
        locals.var_mu0_dn4 = assign103690_e155609_d_n4;
        locals.var_mu0_dn5 = assign103690_e155609_d_n5;
        locals.var_mu0_dn6 = assign103690_e155609_d_n6;
        locals.var_mu0_dn7 = assign103690_e155609_d_n7;
        locals.var_mu0_dn8 = assign103690_e155609_d_n8;
        locals.var_mu0_dn9 = assign103690_e155609_d_n9;
        locals.var_mu0_dn10 = assign103690_e155609_d_n10;
        locals.var_mu0_dn11 = assign103690_e155609_d_n11;
        locals.var_mu0_dn14 = assign103690_e155609_d_n14;

        let (assign103700_e155622, assign103700_e155622_d_n0, assign103700_e155622_d_n2, assign103700_e155622_d_n4, assign103700_e155622_d_n5, assign103700_e155622_d_n6, assign103700_e155622_d_n7, assign103700_e155622_d_n8, assign103700_e155622_d_n9, assign103700_e155622_d_n10, assign103700_e155622_d_n11, assign103700_e155622_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103700_e155616: f64 = (locals.var_rrdrvmax * locals.var_rdrvmaxwe);
        let assign103700_e155618: f64 = (assign103700_e155616 * locals.var_rdrvmaxle);
        let assign103700_e155620: f64 = (assign103700_e155618 + 1e-25);
        (assign103700_e155620, ((locals.var_rrdrvmax_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn9 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_rrdrvmax_dn14 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk2359, locals.var_vmaxe__blk2359_dn0, locals.var_vmaxe__blk2359_dn2, locals.var_vmaxe__blk2359_dn4, locals.var_vmaxe__blk2359_dn5, locals.var_vmaxe__blk2359_dn6, locals.var_vmaxe__blk2359_dn7, locals.var_vmaxe__blk2359_dn8, locals.var_vmaxe__blk2359_dn9, locals.var_vmaxe__blk2359_dn10, locals.var_vmaxe__blk2359_dn11, locals.var_vmaxe__blk2359_dn14,)
    }
};
        locals.var_vmaxe__blk2359 = assign103700_e155622;
        locals.var_vmaxe__blk2359_dn0 = assign103700_e155622_d_n0;
        locals.var_vmaxe__blk2359_dn2 = assign103700_e155622_d_n2;
        locals.var_vmaxe__blk2359_dn4 = assign103700_e155622_d_n4;
        locals.var_vmaxe__blk2359_dn5 = assign103700_e155622_d_n5;
        locals.var_vmaxe__blk2359_dn6 = assign103700_e155622_d_n6;
        locals.var_vmaxe__blk2359_dn7 = assign103700_e155622_d_n7;
        locals.var_vmaxe__blk2359_dn8 = assign103700_e155622_d_n8;
        locals.var_vmaxe__blk2359_dn9 = assign103700_e155622_d_n9;
        locals.var_vmaxe__blk2359_dn10 = assign103700_e155622_d_n10;
        locals.var_vmaxe__blk2359_dn11 = assign103700_e155622_d_n11;
        locals.var_vmaxe__blk2359_dn14 = assign103700_e155622_d_n14;

        let (assign103710_e155629,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (locals.var_uc_rdrcx,)
    } else {
        (locals.var_cx,)
    }
};
        locals.var_cx = assign103710_e155629;

        let (assign103720_e155636,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (p.p421,)
    } else {
        (locals.var_car,)
    }
};
        locals.var_car = assign103720_e155636;

        let (assign103730_e155645, assign103730_e155645_d_n0, assign103730_e155645_d_n2, assign103730_e155645_d_n4, assign103730_e155645_d_n5, assign103730_e155645_d_n6, assign103730_e155645_d_n7, assign103730_e155645_d_n8, assign103730_e155645_d_n9, assign103730_e155645_d_n10, assign103730_e155645_d_n11, assign103730_e155645_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103730_e155643: f64 = (locals.var_mu0 * 10000.0);
        (assign103730_e155643, (locals.var_mu0_dn0 * 10000.0), (locals.var_mu0_dn2 * 10000.0), (locals.var_mu0_dn4 * 10000.0), (locals.var_mu0_dn5 * 10000.0), (locals.var_mu0_dn6 * 10000.0), (locals.var_mu0_dn7 * 10000.0), (locals.var_mu0_dn8 * 10000.0), (locals.var_mu0_dn9 * 10000.0), (locals.var_mu0_dn10 * 10000.0), (locals.var_mu0_dn11 * 10000.0), (locals.var_mu0_dn14 * 10000.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103730_e155645;
        locals.var_t1_dn0 = assign103730_e155645_d_n0;
        locals.var_t1_dn2 = assign103730_e155645_d_n2;
        locals.var_t1_dn4 = assign103730_e155645_d_n4;
        locals.var_t1_dn5 = assign103730_e155645_d_n5;
        locals.var_t1_dn6 = assign103730_e155645_d_n6;
        locals.var_t1_dn7 = assign103730_e155645_d_n7;
        locals.var_t1_dn8 = assign103730_e155645_d_n8;
        locals.var_t1_dn9 = assign103730_e155645_d_n9;
        locals.var_t1_dn10 = assign103730_e155645_d_n10;
        locals.var_t1_dn11 = assign103730_e155645_d_n11;
        locals.var_t1_dn14 = assign103730_e155645_d_n14;

        let (assign103740_e155654, assign103740_e155654_d_n0, assign103740_e155654_d_n2, assign103740_e155654_d_n4, assign103740_e155654_d_n5, assign103740_e155654_d_n6, assign103740_e155654_d_n7, assign103740_e155654_d_n8, assign103740_e155654_d_n9, assign103740_e155654_d_n10, assign103740_e155654_d_n11, assign103740_e155654_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103740_e155652: f64 = (locals.var_vmaxe__blk2359 * 100.0);
        (assign103740_e155652, (locals.var_vmaxe__blk2359_dn0 * 100.0), (locals.var_vmaxe__blk2359_dn2 * 100.0), (locals.var_vmaxe__blk2359_dn4 * 100.0), (locals.var_vmaxe__blk2359_dn5 * 100.0), (locals.var_vmaxe__blk2359_dn6 * 100.0), (locals.var_vmaxe__blk2359_dn7 * 100.0), (locals.var_vmaxe__blk2359_dn8 * 100.0), (locals.var_vmaxe__blk2359_dn9 * 100.0), (locals.var_vmaxe__blk2359_dn10 * 100.0), (locals.var_vmaxe__blk2359_dn11 * 100.0), (locals.var_vmaxe__blk2359_dn14 * 100.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103740_e155654;
        locals.var_t2_dn0 = assign103740_e155654_d_n0;
        locals.var_t2_dn2 = assign103740_e155654_d_n2;
        locals.var_t2_dn4 = assign103740_e155654_d_n4;
        locals.var_t2_dn5 = assign103740_e155654_d_n5;
        locals.var_t2_dn6 = assign103740_e155654_d_n6;
        locals.var_t2_dn7 = assign103740_e155654_d_n7;
        locals.var_t2_dn8 = assign103740_e155654_d_n8;
        locals.var_t2_dn9 = assign103740_e155654_d_n9;
        locals.var_t2_dn10 = assign103740_e155654_d_n10;
        locals.var_t2_dn11 = assign103740_e155654_d_n11;
        locals.var_t2_dn14 = assign103740_e155654_d_n14;

        let assign103770_e155675: f64 = if locals.var_vddp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2367 = assign103770_e155675;

        let (assign103780_e155691, assign103780_e155691_d_n0, assign103780_e155691_d_n2, assign103780_e155691_d_n4, assign103780_e155691_d_n5, assign103780_e155691_d_n6, assign103780_e155691_d_n7, assign103780_e155691_d_n8, assign103780_e155691_d_n9, assign103780_e155691_d_n10, assign103780_e155691_d_n11, assign103780_e155691_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103780_e155684: f64 = (-locals.var_vddp);
        let assign103780_e155686: f64 = (assign103780_e155684 / 2.0);
        let assign103780_e155687: f64 = (2.0 * assign103780_e155686);
        let assign103780_e155689: f64 = (assign103780_e155687 / p.p262);
        (assign103780_e155689, ((2.0 * ((-locals.var_vddp_dn0) / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * ((-locals.var_vddp_dn6) / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103780_e155691;
        locals.var_tmf1_dn0 = assign103780_e155691_d_n0;
        locals.var_tmf1_dn2 = assign103780_e155691_d_n2;
        locals.var_tmf1_dn4 = assign103780_e155691_d_n4;
        locals.var_tmf1_dn5 = assign103780_e155691_d_n5;
        locals.var_tmf1_dn6 = assign103780_e155691_d_n6;
        locals.var_tmf1_dn7 = assign103780_e155691_d_n7;
        locals.var_tmf1_dn8 = assign103780_e155691_d_n8;
        locals.var_tmf1_dn9 = assign103780_e155691_d_n9;
        locals.var_tmf1_dn10 = assign103780_e155691_d_n10;
        locals.var_tmf1_dn11 = assign103780_e155691_d_n11;
        locals.var_tmf1_dn14 = assign103780_e155691_d_n14;

    }

    pub(super) fn stamp_transient_block_381(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign103790_e155736, assign103790_e155736_d_n0, assign103790_e155736_d_n2, assign103790_e155736_d_n4, assign103790_e155736_d_n5, assign103790_e155736_d_n6, assign103790_e155736_d_n7, assign103790_e155736_d_n8, assign103790_e155736_d_n9, assign103790_e155736_d_n10, assign103790_e155736_d_n11, assign103790_e155736_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103790_e155702: f64 = (1.0 / 2.0);
        let assign103790_e155706: f64 = (1.0 / 6.0);
        let assign103790_e155710: f64 = (1.0 / 24.0);
        let assign103790_e155714: f64 = (1.0 / 120.0);
        let assign103790_e155718: f64 = (1.0 / 720.0);
        let assign103790_e155722: f64 = (1.0 / 5040.0);
        let assign103790_e155723: f64 = (locals.var_tmf1 * assign103790_e155722);
        let assign103790_e155724: f64 = (assign103790_e155718 + assign103790_e155723);
        let assign103790_e155725: f64 = (locals.var_tmf1 * assign103790_e155724);
        let assign103790_e155726: f64 = (assign103790_e155714 + assign103790_e155725);
        let assign103790_e155727: f64 = (locals.var_tmf1 * assign103790_e155726);
        let assign103790_e155728: f64 = (assign103790_e155710 + assign103790_e155727);
        let assign103790_e155729: f64 = (locals.var_tmf1 * assign103790_e155728);
        let assign103790_e155730: f64 = (assign103790_e155706 + assign103790_e155729);
        let assign103790_e155731: f64 = (locals.var_tmf1 * assign103790_e155730);
        let assign103790_e155732: f64 = (assign103790_e155702 + assign103790_e155731);
        let assign103790_e155733: f64 = (locals.var_tmf1 * assign103790_e155732);
        let assign103790_e155734: f64 = (1.0 + assign103790_e155733);
        (assign103790_e155734, ((locals.var_tmf1_dn0 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn2 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn4 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn5 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn6 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn7 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn8 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn9 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn10 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn11 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103790_e155722))))))))))), ((locals.var_tmf1_dn14 * assign103790_e155732) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155730) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155728) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155726) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103790_e155724) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103790_e155722))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103790_e155736;
        locals.var_tmf2_dn0 = assign103790_e155736_d_n0;
        locals.var_tmf2_dn2 = assign103790_e155736_d_n2;
        locals.var_tmf2_dn4 = assign103790_e155736_d_n4;
        locals.var_tmf2_dn5 = assign103790_e155736_d_n5;
        locals.var_tmf2_dn6 = assign103790_e155736_d_n6;
        locals.var_tmf2_dn7 = assign103790_e155736_d_n7;
        locals.var_tmf2_dn8 = assign103790_e155736_d_n8;
        locals.var_tmf2_dn9 = assign103790_e155736_d_n9;
        locals.var_tmf2_dn10 = assign103790_e155736_d_n10;
        locals.var_tmf2_dn11 = assign103790_e155736_d_n11;
        locals.var_tmf2_dn14 = assign103790_e155736_d_n14;

        let (assign103800_e155777, assign103800_e155777_d_n0, assign103800_e155777_d_n2, assign103800_e155777_d_n4, assign103800_e155777_d_n5, assign103800_e155777_d_n6, assign103800_e155777_d_n7, assign103800_e155777_d_n8, assign103800_e155777_d_n9, assign103800_e155777_d_n10, assign103800_e155777_d_n11, assign103800_e155777_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103800_e155745: f64 = (1.0 / 2.0);
        let assign103800_e155749: f64 = (1.0 / 3.0);
        let assign103800_e155753: f64 = (1.0 / 8.0);
        let assign103800_e155757: f64 = (1.0 / 30.0);
        let assign103800_e155761: f64 = (1.0 / 144.0);
        let assign103800_e155765: f64 = (1.0 / 840.0);
        let assign103800_e155766: f64 = (locals.var_tmf1 * assign103800_e155765);
        let assign103800_e155767: f64 = (assign103800_e155761 + assign103800_e155766);
        let assign103800_e155768: f64 = (locals.var_tmf1 * assign103800_e155767);
        let assign103800_e155769: f64 = (assign103800_e155757 + assign103800_e155768);
        let assign103800_e155770: f64 = (locals.var_tmf1 * assign103800_e155769);
        let assign103800_e155771: f64 = (assign103800_e155753 + assign103800_e155770);
        let assign103800_e155772: f64 = (locals.var_tmf1 * assign103800_e155771);
        let assign103800_e155773: f64 = (assign103800_e155749 + assign103800_e155772);
        let assign103800_e155774: f64 = (locals.var_tmf1 * assign103800_e155773);
        let assign103800_e155775: f64 = (assign103800_e155745 + assign103800_e155774);
        (assign103800_e155775, ((locals.var_tmf1_dn0 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103800_e155765))))))))), ((locals.var_tmf1_dn2 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103800_e155765))))))))), ((locals.var_tmf1_dn4 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103800_e155765))))))))), ((locals.var_tmf1_dn5 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103800_e155765))))))))), ((locals.var_tmf1_dn6 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103800_e155765))))))))), ((locals.var_tmf1_dn7 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103800_e155765))))))))), ((locals.var_tmf1_dn8 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103800_e155765))))))))), ((locals.var_tmf1_dn9 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103800_e155765))))))))), ((locals.var_tmf1_dn10 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103800_e155765))))))))), ((locals.var_tmf1_dn11 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103800_e155765))))))))), ((locals.var_tmf1_dn14 * assign103800_e155773) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155771) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155769) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103800_e155767) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103800_e155765))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103800_e155777;
        locals.var_tmf3_dn0 = assign103800_e155777_d_n0;
        locals.var_tmf3_dn2 = assign103800_e155777_d_n2;
        locals.var_tmf3_dn4 = assign103800_e155777_d_n4;
        locals.var_tmf3_dn5 = assign103800_e155777_d_n5;
        locals.var_tmf3_dn6 = assign103800_e155777_d_n6;
        locals.var_tmf3_dn7 = assign103800_e155777_d_n7;
        locals.var_tmf3_dn8 = assign103800_e155777_d_n8;
        locals.var_tmf3_dn9 = assign103800_e155777_d_n9;
        locals.var_tmf3_dn10 = assign103800_e155777_d_n10;
        locals.var_tmf3_dn11 = assign103800_e155777_d_n11;
        locals.var_tmf3_dn14 = assign103800_e155777_d_n14;

        let (assign103810_e155788, assign103810_e155788_d_n0, assign103810_e155788_d_n2, assign103810_e155788_d_n4, assign103810_e155788_d_n5, assign103810_e155788_d_n6, assign103810_e155788_d_n7, assign103810_e155788_d_n8, assign103810_e155788_d_n9, assign103810_e155788_d_n10, assign103810_e155788_d_n11, assign103810_e155788_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103810_e155786: f64 = (p.p262 / locals.var_tmf2);
        (assign103810_e155786, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103810_e155788;
        locals.var_vzadd_dn0 = assign103810_e155788_d_n0;
        locals.var_vzadd_dn2 = assign103810_e155788_d_n2;
        locals.var_vzadd_dn4 = assign103810_e155788_d_n4;
        locals.var_vzadd_dn5 = assign103810_e155788_d_n5;
        locals.var_vzadd_dn6 = assign103810_e155788_d_n6;
        locals.var_vzadd_dn7 = assign103810_e155788_d_n7;
        locals.var_vzadd_dn8 = assign103810_e155788_d_n8;
        locals.var_vzadd_dn9 = assign103810_e155788_d_n9;
        locals.var_vzadd_dn10 = assign103810_e155788_d_n10;
        locals.var_vzadd_dn11 = assign103810_e155788_d_n11;
        locals.var_vzadd_dn14 = assign103810_e155788_d_n14;

        let (assign103820_e155804, assign103820_e155804_d_n0, assign103820_e155804_d_n2, assign103820_e155804_d_n4, assign103820_e155804_d_n5, assign103820_e155804_d_n6, assign103820_e155804_d_n7, assign103820_e155804_d_n8, assign103820_e155804_d_n9, assign103820_e155804_d_n10, assign103820_e155804_d_n11, assign103820_e155804_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103820_e155796: f64 = (-2.0);
        let assign103820_e155798: f64 = (assign103820_e155796 * locals.var_tmf3);
        let assign103820_e155801: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103820_e155802: f64 = (assign103820_e155798 / assign103820_e155801);
        (assign103820_e155802, ((((assign103820_e155796 * locals.var_tmf3_dn0) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn2) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn4) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn5) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn6) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn7) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn8) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn9) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn10) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn11) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103820_e155801 * assign103820_e155801)), ((((assign103820_e155796 * locals.var_tmf3_dn14) * assign103820_e155801) - (assign103820_e155798 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103820_e155801 * assign103820_e155801)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103820_e155804;
        locals.var_t2_dn0 = assign103820_e155804_d_n0;
        locals.var_t2_dn2 = assign103820_e155804_d_n2;
        locals.var_t2_dn4 = assign103820_e155804_d_n4;
        locals.var_t2_dn5 = assign103820_e155804_d_n5;
        locals.var_t2_dn6 = assign103820_e155804_d_n6;
        locals.var_t2_dn7 = assign103820_e155804_d_n7;
        locals.var_t2_dn8 = assign103820_e155804_d_n8;
        locals.var_t2_dn9 = assign103820_e155804_d_n9;
        locals.var_t2_dn10 = assign103820_e155804_d_n10;
        locals.var_t2_dn11 = assign103820_e155804_d_n11;
        locals.var_t2_dn14 = assign103820_e155804_d_n14;

        let assign103830_e155807: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2368 = assign103830_e155807;

        let (assign103840_e155818, assign103840_e155818_d_n0, assign103840_e155818_d_n2, assign103840_e155818_d_n4, assign103840_e155818_d_n5, assign103840_e155818_d_n6, assign103840_e155818_d_n7, assign103840_e155818_d_n8, assign103840_e155818_d_n9, assign103840_e155818_d_n10, assign103840_e155818_d_n11, assign103840_e155818_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) && (locals.var_guard2368 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103840_e155818;
        locals.var_vzadd_dn0 = assign103840_e155818_d_n0;
        locals.var_vzadd_dn2 = assign103840_e155818_d_n2;
        locals.var_vzadd_dn4 = assign103840_e155818_d_n4;
        locals.var_vzadd_dn5 = assign103840_e155818_d_n5;
        locals.var_vzadd_dn6 = assign103840_e155818_d_n6;
        locals.var_vzadd_dn7 = assign103840_e155818_d_n7;
        locals.var_vzadd_dn8 = assign103840_e155818_d_n8;
        locals.var_vzadd_dn9 = assign103840_e155818_d_n9;
        locals.var_vzadd_dn10 = assign103840_e155818_d_n10;
        locals.var_vzadd_dn11 = assign103840_e155818_d_n11;
        locals.var_vzadd_dn14 = assign103840_e155818_d_n14;

        let (assign103850_e155831, assign103850_e155831_d_n0, assign103850_e155831_d_n2, assign103850_e155831_d_n4, assign103850_e155831_d_n5, assign103850_e155831_d_n6, assign103850_e155831_d_n7, assign103850_e155831_d_n8, assign103850_e155831_d_n9, assign103850_e155831_d_n10, assign103850_e155831_d_n11, assign103850_e155831_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 != 0.0)) {
        let assign103850_e155828: f64 = (2.0 * locals.var_vzadd);
        let assign103850_e155829: f64 = (locals.var_vddp - assign103850_e155828);
        (assign103850_e155829, (locals.var_vddp_dn0 - (2.0 * locals.var_vzadd_dn0)), (-(2.0 * locals.var_vzadd_dn2)), (-(2.0 * locals.var_vzadd_dn4)), (-(2.0 * locals.var_vzadd_dn5)), (locals.var_vddp_dn6 - (2.0 * locals.var_vzadd_dn6)), (-(2.0 * locals.var_vzadd_dn7)), (-(2.0 * locals.var_vzadd_dn8)), (-(2.0 * locals.var_vzadd_dn9)), (-(2.0 * locals.var_vzadd_dn10)), (-(2.0 * locals.var_vzadd_dn11)), (-(2.0 * locals.var_vzadd_dn14)),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103850_e155831;
        locals.var_vddpz_dn0 = assign103850_e155831_d_n0;
        locals.var_vddpz_dn2 = assign103850_e155831_d_n2;
        locals.var_vddpz_dn4 = assign103850_e155831_d_n4;
        locals.var_vddpz_dn5 = assign103850_e155831_d_n5;
        locals.var_vddpz_dn6 = assign103850_e155831_d_n6;
        locals.var_vddpz_dn7 = assign103850_e155831_d_n7;
        locals.var_vddpz_dn8 = assign103850_e155831_d_n8;
        locals.var_vddpz_dn9 = assign103850_e155831_d_n9;
        locals.var_vddpz_dn10 = assign103850_e155831_d_n10;
        locals.var_vddpz_dn11 = assign103850_e155831_d_n11;
        locals.var_vddpz_dn14 = assign103850_e155831_d_n14;

        let (assign103860_e155847, assign103860_e155847_d_n0, assign103860_e155847_d_n2, assign103860_e155847_d_n4, assign103860_e155847_d_n5, assign103860_e155847_d_n6, assign103860_e155847_d_n7, assign103860_e155847_d_n8, assign103860_e155847_d_n9, assign103860_e155847_d_n10, assign103860_e155847_d_n11, assign103860_e155847_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103860_e155842: f64 = (locals.var_vddp / 2.0);
        let assign103860_e155843: f64 = (2.0 * assign103860_e155842);
        let assign103860_e155845: f64 = (assign103860_e155843 / p.p262);
        (assign103860_e155845, ((2.0 * (locals.var_vddp_dn0 / 2.0)) / p.p262), 0.0, 0.0, 0.0, ((2.0 * (locals.var_vddp_dn6 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign103860_e155847;
        locals.var_tmf1_dn0 = assign103860_e155847_d_n0;
        locals.var_tmf1_dn2 = assign103860_e155847_d_n2;
        locals.var_tmf1_dn4 = assign103860_e155847_d_n4;
        locals.var_tmf1_dn5 = assign103860_e155847_d_n5;
        locals.var_tmf1_dn6 = assign103860_e155847_d_n6;
        locals.var_tmf1_dn7 = assign103860_e155847_d_n7;
        locals.var_tmf1_dn8 = assign103860_e155847_d_n8;
        locals.var_tmf1_dn9 = assign103860_e155847_d_n9;
        locals.var_tmf1_dn10 = assign103860_e155847_d_n10;
        locals.var_tmf1_dn11 = assign103860_e155847_d_n11;
        locals.var_tmf1_dn14 = assign103860_e155847_d_n14;

        let (assign103870_e155893, assign103870_e155893_d_n0, assign103870_e155893_d_n2, assign103870_e155893_d_n4, assign103870_e155893_d_n5, assign103870_e155893_d_n6, assign103870_e155893_d_n7, assign103870_e155893_d_n8, assign103870_e155893_d_n9, assign103870_e155893_d_n10, assign103870_e155893_d_n11, assign103870_e155893_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103870_e155859: f64 = (1.0 / 2.0);
        let assign103870_e155863: f64 = (1.0 / 6.0);
        let assign103870_e155867: f64 = (1.0 / 24.0);
        let assign103870_e155871: f64 = (1.0 / 120.0);
        let assign103870_e155875: f64 = (1.0 / 720.0);
        let assign103870_e155879: f64 = (1.0 / 5040.0);
        let assign103870_e155880: f64 = (locals.var_tmf1 * assign103870_e155879);
        let assign103870_e155881: f64 = (assign103870_e155875 + assign103870_e155880);
        let assign103870_e155882: f64 = (locals.var_tmf1 * assign103870_e155881);
        let assign103870_e155883: f64 = (assign103870_e155871 + assign103870_e155882);
        let assign103870_e155884: f64 = (locals.var_tmf1 * assign103870_e155883);
        let assign103870_e155885: f64 = (assign103870_e155867 + assign103870_e155884);
        let assign103870_e155886: f64 = (locals.var_tmf1 * assign103870_e155885);
        let assign103870_e155887: f64 = (assign103870_e155863 + assign103870_e155886);
        let assign103870_e155888: f64 = (locals.var_tmf1 * assign103870_e155887);
        let assign103870_e155889: f64 = (assign103870_e155859 + assign103870_e155888);
        let assign103870_e155890: f64 = (locals.var_tmf1 * assign103870_e155889);
        let assign103870_e155891: f64 = (1.0 + assign103870_e155890);
        (assign103870_e155891, ((locals.var_tmf1_dn0 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn2 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn4 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn5 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn6 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn7 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn8 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn9 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn10 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn11 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103870_e155879))))))))))), ((locals.var_tmf1_dn14 * assign103870_e155889) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155887) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155885) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155883) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103870_e155881) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103870_e155879))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign103870_e155893;
        locals.var_tmf2_dn0 = assign103870_e155893_d_n0;
        locals.var_tmf2_dn2 = assign103870_e155893_d_n2;
        locals.var_tmf2_dn4 = assign103870_e155893_d_n4;
        locals.var_tmf2_dn5 = assign103870_e155893_d_n5;
        locals.var_tmf2_dn6 = assign103870_e155893_d_n6;
        locals.var_tmf2_dn7 = assign103870_e155893_d_n7;
        locals.var_tmf2_dn8 = assign103870_e155893_d_n8;
        locals.var_tmf2_dn9 = assign103870_e155893_d_n9;
        locals.var_tmf2_dn10 = assign103870_e155893_d_n10;
        locals.var_tmf2_dn11 = assign103870_e155893_d_n11;
        locals.var_tmf2_dn14 = assign103870_e155893_d_n14;

        let (assign103880_e155935, assign103880_e155935_d_n0, assign103880_e155935_d_n2, assign103880_e155935_d_n4, assign103880_e155935_d_n5, assign103880_e155935_d_n6, assign103880_e155935_d_n7, assign103880_e155935_d_n8, assign103880_e155935_d_n9, assign103880_e155935_d_n10, assign103880_e155935_d_n11, assign103880_e155935_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103880_e155903: f64 = (1.0 / 2.0);
        let assign103880_e155907: f64 = (1.0 / 3.0);
        let assign103880_e155911: f64 = (1.0 / 8.0);
        let assign103880_e155915: f64 = (1.0 / 30.0);
        let assign103880_e155919: f64 = (1.0 / 144.0);
        let assign103880_e155923: f64 = (1.0 / 840.0);
        let assign103880_e155924: f64 = (locals.var_tmf1 * assign103880_e155923);
        let assign103880_e155925: f64 = (assign103880_e155919 + assign103880_e155924);
        let assign103880_e155926: f64 = (locals.var_tmf1 * assign103880_e155925);
        let assign103880_e155927: f64 = (assign103880_e155915 + assign103880_e155926);
        let assign103880_e155928: f64 = (locals.var_tmf1 * assign103880_e155927);
        let assign103880_e155929: f64 = (assign103880_e155911 + assign103880_e155928);
        let assign103880_e155930: f64 = (locals.var_tmf1 * assign103880_e155929);
        let assign103880_e155931: f64 = (assign103880_e155907 + assign103880_e155930);
        let assign103880_e155932: f64 = (locals.var_tmf1 * assign103880_e155931);
        let assign103880_e155933: f64 = (assign103880_e155903 + assign103880_e155932);
        (assign103880_e155933, ((locals.var_tmf1_dn0 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign103880_e155923))))))))), ((locals.var_tmf1_dn2 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign103880_e155923))))))))), ((locals.var_tmf1_dn4 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign103880_e155923))))))))), ((locals.var_tmf1_dn5 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign103880_e155923))))))))), ((locals.var_tmf1_dn6 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign103880_e155923))))))))), ((locals.var_tmf1_dn7 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign103880_e155923))))))))), ((locals.var_tmf1_dn8 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign103880_e155923))))))))), ((locals.var_tmf1_dn9 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign103880_e155923))))))))), ((locals.var_tmf1_dn10 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign103880_e155923))))))))), ((locals.var_tmf1_dn11 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign103880_e155923))))))))), ((locals.var_tmf1_dn14 * assign103880_e155931) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155929) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155927) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign103880_e155925) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign103880_e155923))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign103880_e155935;
        locals.var_tmf3_dn0 = assign103880_e155935_d_n0;
        locals.var_tmf3_dn2 = assign103880_e155935_d_n2;
        locals.var_tmf3_dn4 = assign103880_e155935_d_n4;
        locals.var_tmf3_dn5 = assign103880_e155935_d_n5;
        locals.var_tmf3_dn6 = assign103880_e155935_d_n6;
        locals.var_tmf3_dn7 = assign103880_e155935_d_n7;
        locals.var_tmf3_dn8 = assign103880_e155935_d_n8;
        locals.var_tmf3_dn9 = assign103880_e155935_d_n9;
        locals.var_tmf3_dn10 = assign103880_e155935_d_n10;
        locals.var_tmf3_dn11 = assign103880_e155935_d_n11;
        locals.var_tmf3_dn14 = assign103880_e155935_d_n14;

        let (assign103890_e155947, assign103890_e155947_d_n0, assign103890_e155947_d_n2, assign103890_e155947_d_n4, assign103890_e155947_d_n5, assign103890_e155947_d_n6, assign103890_e155947_d_n7, assign103890_e155947_d_n8, assign103890_e155947_d_n9, assign103890_e155947_d_n10, assign103890_e155947_d_n11, assign103890_e155947_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103890_e155945: f64 = (p.p262 / locals.var_tmf2);
        (assign103890_e155945, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103890_e155947;
        locals.var_vzadd_dn0 = assign103890_e155947_d_n0;
        locals.var_vzadd_dn2 = assign103890_e155947_d_n2;
        locals.var_vzadd_dn4 = assign103890_e155947_d_n4;
        locals.var_vzadd_dn5 = assign103890_e155947_d_n5;
        locals.var_vzadd_dn6 = assign103890_e155947_d_n6;
        locals.var_vzadd_dn7 = assign103890_e155947_d_n7;
        locals.var_vzadd_dn8 = assign103890_e155947_d_n8;
        locals.var_vzadd_dn9 = assign103890_e155947_d_n9;
        locals.var_vzadd_dn10 = assign103890_e155947_d_n10;
        locals.var_vzadd_dn11 = assign103890_e155947_d_n11;
        locals.var_vzadd_dn14 = assign103890_e155947_d_n14;

        let (assign103900_e155964, assign103900_e155964_d_n0, assign103900_e155964_d_n2, assign103900_e155964_d_n4, assign103900_e155964_d_n5, assign103900_e155964_d_n6, assign103900_e155964_d_n7, assign103900_e155964_d_n8, assign103900_e155964_d_n9, assign103900_e155964_d_n10, assign103900_e155964_d_n11, assign103900_e155964_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103900_e155956: f64 = (-2.0);
        let assign103900_e155958: f64 = (assign103900_e155956 * locals.var_tmf3);
        let assign103900_e155961: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign103900_e155962: f64 = (assign103900_e155958 / assign103900_e155961);
        (assign103900_e155962, ((((assign103900_e155956 * locals.var_tmf3_dn0) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn2) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn4) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn5) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn6) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn7) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn8) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn9) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn10) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn11) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign103900_e155961 * assign103900_e155961)), ((((assign103900_e155956 * locals.var_tmf3_dn14) * assign103900_e155961) - (assign103900_e155958 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign103900_e155961 * assign103900_e155961)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign103900_e155964;
        locals.var_t2_dn0 = assign103900_e155964_d_n0;
        locals.var_t2_dn2 = assign103900_e155964_d_n2;
        locals.var_t2_dn4 = assign103900_e155964_d_n4;
        locals.var_t2_dn5 = assign103900_e155964_d_n5;
        locals.var_t2_dn6 = assign103900_e155964_d_n6;
        locals.var_t2_dn7 = assign103900_e155964_d_n7;
        locals.var_t2_dn8 = assign103900_e155964_d_n8;
        locals.var_t2_dn9 = assign103900_e155964_d_n9;
        locals.var_t2_dn10 = assign103900_e155964_d_n10;
        locals.var_t2_dn11 = assign103900_e155964_d_n11;
        locals.var_t2_dn14 = assign103900_e155964_d_n14;

        let assign103910_e155967: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard2369 = assign103910_e155967;

        let (assign103920_e155979, assign103920_e155979_d_n0, assign103920_e155979_d_n2, assign103920_e155979_d_n4, assign103920_e155979_d_n5, assign103920_e155979_d_n6, assign103920_e155979_d_n7, assign103920_e155979_d_n8, assign103920_e155979_d_n9, assign103920_e155979_d_n10, assign103920_e155979_d_n11, assign103920_e155979_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) && (locals.var_guard2369 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign103920_e155979;
        locals.var_vzadd_dn0 = assign103920_e155979_d_n0;
        locals.var_vzadd_dn2 = assign103920_e155979_d_n2;
        locals.var_vzadd_dn4 = assign103920_e155979_d_n4;
        locals.var_vzadd_dn5 = assign103920_e155979_d_n5;
        locals.var_vzadd_dn6 = assign103920_e155979_d_n6;
        locals.var_vzadd_dn7 = assign103920_e155979_d_n7;
        locals.var_vzadd_dn8 = assign103920_e155979_d_n8;
        locals.var_vzadd_dn9 = assign103920_e155979_d_n9;
        locals.var_vzadd_dn10 = assign103920_e155979_d_n10;
        locals.var_vzadd_dn11 = assign103920_e155979_d_n11;
        locals.var_vzadd_dn14 = assign103920_e155979_d_n14;

        let (assign103930_e155993, assign103930_e155993_d_n0, assign103930_e155993_d_n2, assign103930_e155993_d_n4, assign103930_e155993_d_n5, assign103930_e155993_d_n6, assign103930_e155993_d_n7, assign103930_e155993_d_n8, assign103930_e155993_d_n9, assign103930_e155993_d_n10, assign103930_e155993_d_n11, assign103930_e155993_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2367 == 0.0)) {
        let assign103930_e155990: f64 = (2.0 * locals.var_vzadd);
        let assign103930_e155991: f64 = (locals.var_vddp + assign103930_e155990);
        (assign103930_e155991, (locals.var_vddp_dn0 + (2.0 * locals.var_vzadd_dn0)), (2.0 * locals.var_vzadd_dn2), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (locals.var_vddp_dn6 + (2.0 * locals.var_vzadd_dn6)), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vddpz, locals.var_vddpz_dn0, locals.var_vddpz_dn2, locals.var_vddpz_dn4, locals.var_vddpz_dn5, locals.var_vddpz_dn6, locals.var_vddpz_dn7, locals.var_vddpz_dn8, locals.var_vddpz_dn9, locals.var_vddpz_dn10, locals.var_vddpz_dn11, locals.var_vddpz_dn14,)
    }
};
        locals.var_vddpz = assign103930_e155993;
        locals.var_vddpz_dn0 = assign103930_e155993_d_n0;
        locals.var_vddpz_dn2 = assign103930_e155993_d_n2;
        locals.var_vddpz_dn4 = assign103930_e155993_d_n4;
        locals.var_vddpz_dn5 = assign103930_e155993_d_n5;
        locals.var_vddpz_dn6 = assign103930_e155993_d_n6;
        locals.var_vddpz_dn7 = assign103930_e155993_d_n7;
        locals.var_vddpz_dn8 = assign103930_e155993_d_n8;
        locals.var_vddpz_dn9 = assign103930_e155993_d_n9;
        locals.var_vddpz_dn10 = assign103930_e155993_d_n10;
        locals.var_vddpz_dn11 = assign103930_e155993_d_n11;
        locals.var_vddpz_dn14 = assign103930_e155993_d_n14;

        let (assign103940_e156002, assign103940_e156002_d_n0, assign103940_e156002_d_n2, assign103940_e156002_d_n4, assign103940_e156002_d_n5, assign103940_e156002_d_n6, assign103940_e156002_d_n7, assign103940_e156002_d_n8, assign103940_e156002_d_n9, assign103940_e156002_d_n10, assign103940_e156002_d_n11, assign103940_e156002_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103940_e156000: f64 = (locals.var_vddpz / locals.var_ldrifte);
        (assign103940_e156000, (locals.var_vddpz_dn0 / locals.var_ldrifte), (locals.var_vddpz_dn2 / locals.var_ldrifte), (locals.var_vddpz_dn4 / locals.var_ldrifte), (locals.var_vddpz_dn5 / locals.var_ldrifte), (locals.var_vddpz_dn6 / locals.var_ldrifte), (locals.var_vddpz_dn7 / locals.var_ldrifte), (locals.var_vddpz_dn8 / locals.var_ldrifte), (locals.var_vddpz_dn9 / locals.var_ldrifte), (locals.var_vddpz_dn10 / locals.var_ldrifte), (locals.var_vddpz_dn11 / locals.var_ldrifte), (locals.var_vddpz_dn14 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn4, locals.var_edri_dn5, locals.var_edri_dn6, locals.var_edri_dn7, locals.var_edri_dn8, locals.var_edri_dn9, locals.var_edri_dn10, locals.var_edri_dn11, locals.var_edri_dn14,)
    }
};
        locals.var_edri = assign103940_e156002;
        locals.var_edri_dn0 = assign103940_e156002_d_n0;
        locals.var_edri_dn2 = assign103940_e156002_d_n2;
        locals.var_edri_dn4 = assign103940_e156002_d_n4;
        locals.var_edri_dn5 = assign103940_e156002_d_n5;
        locals.var_edri_dn6 = assign103940_e156002_d_n6;
        locals.var_edri_dn7 = assign103940_e156002_d_n7;
        locals.var_edri_dn8 = assign103940_e156002_d_n8;
        locals.var_edri_dn9 = assign103940_e156002_d_n9;
        locals.var_edri_dn10 = assign103940_e156002_d_n10;
        locals.var_edri_dn11 = assign103940_e156002_d_n11;
        locals.var_edri_dn14 = assign103940_e156002_d_n14;

        let (assign103950_e156011, assign103950_e156011_d_n0, assign103950_e156011_d_n2, assign103950_e156011_d_n4, assign103950_e156011_d_n5, assign103950_e156011_d_n6, assign103950_e156011_d_n7, assign103950_e156011_d_n8, assign103950_e156011_d_n9, assign103950_e156011_d_n10, assign103950_e156011_d_n11, assign103950_e156011_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign103950_e156009: f64 = (locals.var_mu0 * locals.var_edri);
        (assign103950_e156009, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn4 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn4)), ((locals.var_mu0_dn5 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn5)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), ((locals.var_mu0_dn8 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn8)), ((locals.var_mu0_dn9 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn9)), ((locals.var_mu0_dn10 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn10)), ((locals.var_mu0_dn11 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn11)), ((locals.var_mu0_dn14 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn14)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn8, locals.var_vdri_dn9, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn14,)
    }
};
        locals.var_vdri = assign103950_e156011;
        locals.var_vdri_dn0 = assign103950_e156011_d_n0;
        locals.var_vdri_dn2 = assign103950_e156011_d_n2;
        locals.var_vdri_dn4 = assign103950_e156011_d_n4;
        locals.var_vdri_dn5 = assign103950_e156011_d_n5;
        locals.var_vdri_dn6 = assign103950_e156011_d_n6;
        locals.var_vdri_dn7 = assign103950_e156011_d_n7;
        locals.var_vdri_dn8 = assign103950_e156011_d_n8;
        locals.var_vdri_dn9 = assign103950_e156011_d_n9;
        locals.var_vdri_dn10 = assign103950_e156011_d_n10;
        locals.var_vdri_dn11 = assign103950_e156011_d_n11;
        locals.var_vdri_dn14 = assign103950_e156011_d_n14;

        let assign103960_e156014: f64 = if locals.var_vddp >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2370 = assign103960_e156014;

        let (assign103970_e156025, assign103970_e156025_d_n0, assign103970_e156025_d_n2, assign103970_e156025_d_n4, assign103970_e156025_d_n5, assign103970_e156025_d_n6, assign103970_e156025_d_n7, assign103970_e156025_d_n8, assign103970_e156025_d_n9, assign103970_e156025_d_n10, assign103970_e156025_d_n11, assign103970_e156025_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2370 != 0.0)) {
        let assign103970_e156023: f64 = (locals.var_vdri / locals.var_vmaxe__blk2359);
        (assign103970_e156023, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn0)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn2)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn4)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn5)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn6)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn7)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn8)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn9 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn9)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn10)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn11)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), (((locals.var_vdri_dn14 * locals.var_vmaxe__blk2359) - (locals.var_vdri * locals.var_vmaxe__blk2359_dn14)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103970_e156025;
        locals.var_t1_dn0 = assign103970_e156025_d_n0;
        locals.var_t1_dn2 = assign103970_e156025_d_n2;
        locals.var_t1_dn4 = assign103970_e156025_d_n4;
        locals.var_t1_dn5 = assign103970_e156025_d_n5;
        locals.var_t1_dn6 = assign103970_e156025_d_n6;
        locals.var_t1_dn7 = assign103970_e156025_d_n7;
        locals.var_t1_dn8 = assign103970_e156025_d_n8;
        locals.var_t1_dn9 = assign103970_e156025_d_n9;
        locals.var_t1_dn10 = assign103970_e156025_d_n10;
        locals.var_t1_dn11 = assign103970_e156025_d_n11;
        locals.var_t1_dn14 = assign103970_e156025_d_n14;

        let (assign103980_e156038, assign103980_e156038_d_n0, assign103980_e156038_d_n2, assign103980_e156038_d_n4, assign103980_e156038_d_n5, assign103980_e156038_d_n6, assign103980_e156038_d_n7, assign103980_e156038_d_n8, assign103980_e156038_d_n9, assign103980_e156038_d_n10, assign103980_e156038_d_n11, assign103980_e156038_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2370 == 0.0)) {
        let assign103980_e156034: f64 = (-locals.var_vdri);
        let assign103980_e156036: f64 = (assign103980_e156034 / locals.var_vmaxe__blk2359);
        (assign103980_e156036, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn0)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn2)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn4)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn5)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn6)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn7)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn8)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn9) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn9)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn10)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn11)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)), ((((-locals.var_vdri_dn14) * locals.var_vmaxe__blk2359) - (assign103980_e156034 * locals.var_vmaxe__blk2359_dn14)) / (locals.var_vmaxe__blk2359 * locals.var_vmaxe__blk2359)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign103980_e156038;
        locals.var_t1_dn0 = assign103980_e156038_d_n0;
        locals.var_t1_dn2 = assign103980_e156038_d_n2;
        locals.var_t1_dn4 = assign103980_e156038_d_n4;
        locals.var_t1_dn5 = assign103980_e156038_d_n5;
        locals.var_t1_dn6 = assign103980_e156038_d_n6;
        locals.var_t1_dn7 = assign103980_e156038_d_n7;
        locals.var_t1_dn8 = assign103980_e156038_d_n8;
        locals.var_t1_dn9 = assign103980_e156038_d_n9;
        locals.var_t1_dn10 = assign103980_e156038_d_n10;
        locals.var_t1_dn11 = assign103980_e156038_d_n11;
        locals.var_t1_dn14 = assign103980_e156038_d_n14;

        let assign103990_e156042: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156043: f64 = (1.0 - assign103990_e156042);
        let assign103990_e156050: f64 = (10.0 * 2.220446049250313e-16);
        let assign103990_e156051: f64 = (1.0 + assign103990_e156050);
        let assign103990_e156053: f64 = if ((assign103990_e156043 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign103990_e156051)) { 1.0 } else { 0.0 };
        locals.var_guard2371 = assign103990_e156053;

        let (assign104000_e156062, assign104000_e156062_d_n0, assign104000_e156062_d_n2, assign104000_e156062_d_n4, assign104000_e156062_d_n5, assign104000_e156062_d_n6, assign104000_e156062_d_n7, assign104000_e156062_d_n8, assign104000_e156062_d_n9, assign104000_e156062_d_n10, assign104000_e156062_d_n11, assign104000_e156062_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104000_e156062;
        locals.var_t3_dn0 = assign104000_e156062_d_n0;
        locals.var_t3_dn2 = assign104000_e156062_d_n2;
        locals.var_t3_dn4 = assign104000_e156062_d_n4;
        locals.var_t3_dn5 = assign104000_e156062_d_n5;
        locals.var_t3_dn6 = assign104000_e156062_d_n6;
        locals.var_t3_dn7 = assign104000_e156062_d_n7;
        locals.var_t3_dn8 = assign104000_e156062_d_n8;
        locals.var_t3_dn9 = assign104000_e156062_d_n9;
        locals.var_t3_dn10 = assign104000_e156062_d_n10;
        locals.var_t3_dn11 = assign104000_e156062_d_n11;
        locals.var_t3_dn14 = assign104000_e156062_d_n14;

        let assign104010_e156066: f64 = (10.0 * 2.220446049250313e-16);
        let assign104010_e156067: f64 = (2.0 - assign104010_e156066);
        let assign104010_e156074: f64 = (10.0 * 2.220446049250313e-16);
        let assign104010_e156075: f64 = (2.0 + assign104010_e156074);
        let assign104010_e156077: f64 = if ((assign104010_e156067 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104010_e156075)) { 1.0 } else { 0.0 };
        locals.var_guard2372 = assign104010_e156077;

        let (assign104020_e156089, assign104020_e156089_d_n0, assign104020_e156089_d_n2, assign104020_e156089_d_n4, assign104020_e156089_d_n5, assign104020_e156089_d_n6, assign104020_e156089_d_n7, assign104020_e156089_d_n8, assign104020_e156089_d_n9, assign104020_e156089_d_n10, assign104020_e156089_d_n11, assign104020_e156089_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104020_e156089;
        locals.var_t3_dn0 = assign104020_e156089_d_n0;
        locals.var_t3_dn2 = assign104020_e156089_d_n2;
        locals.var_t3_dn4 = assign104020_e156089_d_n4;
        locals.var_t3_dn5 = assign104020_e156089_d_n5;
        locals.var_t3_dn6 = assign104020_e156089_d_n6;
        locals.var_t3_dn7 = assign104020_e156089_d_n7;
        locals.var_t3_dn8 = assign104020_e156089_d_n8;
        locals.var_t3_dn9 = assign104020_e156089_d_n9;
        locals.var_t3_dn10 = assign104020_e156089_d_n10;
        locals.var_t3_dn11 = assign104020_e156089_d_n11;
        locals.var_t3_dn14 = assign104020_e156089_d_n14;

    }

    pub(super) fn stamp_transient_block_382(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104030_e156106, assign104030_e156106_d_n0, assign104030_e156106_d_n2, assign104030_e156106_d_n4, assign104030_e156106_d_n5, assign104030_e156106_d_n6, assign104030_e156106_d_n7, assign104030_e156106_d_n8, assign104030_e156106_d_n9, assign104030_e156106_d_n10, assign104030_e156106_d_n11, assign104030_e156106_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2371 == 0.0)) && (locals.var_guard2372 == 0.0)) {
        let assign104030_e156103: f64 = (locals.var_uc_rdrbb - 1.0);
        let assign104030_e156104: f64 = (locals.var_t1).powf(assign104030_e156103);
        (assign104030_e156104, if locals.var_uc_rdrbb_dn0 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn0)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn0 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn2 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn2)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn2 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn4 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn4)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn4 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn5 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn5)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn5 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn6 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn6)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn6 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn7 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn7)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn7 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn8 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn8)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn8 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn9 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn9)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn9 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn10 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn10)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn10 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn11 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn11)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn11 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_uc_rdrbb_dn14 == 0.0 && ((assign104030_e156103) as f64).is_finite() && ((assign104030_e156103) as f64).fract() == 0.0 { if assign104030_e156103 == 0.0 { 0.0 } else { (assign104030_e156103 * ((locals.var_t1).powf(assign104030_e156103 - 1.0) * locals.var_t1_dn14)) } } else { (assign104030_e156104 * ((locals.var_uc_rdrbb_dn14 * (locals.var_t1).ln()) + (assign104030_e156103 * (locals.var_t1_dn14 / locals.var_t1)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign104030_e156106;
        locals.var_t3_dn0 = assign104030_e156106_d_n0;
        locals.var_t3_dn2 = assign104030_e156106_d_n2;
        locals.var_t3_dn4 = assign104030_e156106_d_n4;
        locals.var_t3_dn5 = assign104030_e156106_d_n5;
        locals.var_t3_dn6 = assign104030_e156106_d_n6;
        locals.var_t3_dn7 = assign104030_e156106_d_n7;
        locals.var_t3_dn8 = assign104030_e156106_d_n8;
        locals.var_t3_dn9 = assign104030_e156106_d_n9;
        locals.var_t3_dn10 = assign104030_e156106_d_n10;
        locals.var_t3_dn11 = assign104030_e156106_d_n11;
        locals.var_t3_dn14 = assign104030_e156106_d_n14;

        let (assign104040_e156115, assign104040_e156115_d_n0, assign104040_e156115_d_n2, assign104040_e156115_d_n4, assign104040_e156115_d_n5, assign104040_e156115_d_n6, assign104040_e156115_d_n7, assign104040_e156115_d_n8, assign104040_e156115_d_n9, assign104040_e156115_d_n10, assign104040_e156115_d_n11, assign104040_e156115_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104040_e156113: f64 = (locals.var_t1 * locals.var_t3);
        (assign104040_e156113, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign104040_e156115;
        locals.var_t2_dn0 = assign104040_e156115_d_n0;
        locals.var_t2_dn2 = assign104040_e156115_d_n2;
        locals.var_t2_dn4 = assign104040_e156115_d_n4;
        locals.var_t2_dn5 = assign104040_e156115_d_n5;
        locals.var_t2_dn6 = assign104040_e156115_d_n6;
        locals.var_t2_dn7 = assign104040_e156115_d_n7;
        locals.var_t2_dn8 = assign104040_e156115_d_n8;
        locals.var_t2_dn9 = assign104040_e156115_d_n9;
        locals.var_t2_dn10 = assign104040_e156115_d_n10;
        locals.var_t2_dn11 = assign104040_e156115_d_n11;
        locals.var_t2_dn14 = assign104040_e156115_d_n14;

        let (assign104050_e156124, assign104050_e156124_d_n0, assign104050_e156124_d_n2, assign104050_e156124_d_n4, assign104050_e156124_d_n5, assign104050_e156124_d_n6, assign104050_e156124_d_n7, assign104050_e156124_d_n8, assign104050_e156124_d_n9, assign104050_e156124_d_n10, assign104050_e156124_d_n11, assign104050_e156124_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104050_e156122: f64 = (1.0 + locals.var_t2);
        (assign104050_e156122, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104050_e156124;
        locals.var_t4_dn0 = assign104050_e156124_d_n0;
        locals.var_t4_dn2 = assign104050_e156124_d_n2;
        locals.var_t4_dn4 = assign104050_e156124_d_n4;
        locals.var_t4_dn5 = assign104050_e156124_d_n5;
        locals.var_t4_dn6 = assign104050_e156124_d_n6;
        locals.var_t4_dn7 = assign104050_e156124_d_n7;
        locals.var_t4_dn8 = assign104050_e156124_d_n8;
        locals.var_t4_dn9 = assign104050_e156124_d_n9;
        locals.var_t4_dn10 = assign104050_e156124_d_n10;
        locals.var_t4_dn11 = assign104050_e156124_d_n11;
        locals.var_t4_dn14 = assign104050_e156124_d_n14;

        let assign104060_e156128: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156129: f64 = (1.0 - assign104060_e156128);
        let assign104060_e156136: f64 = (10.0 * 2.220446049250313e-16);
        let assign104060_e156137: f64 = (1.0 + assign104060_e156136);
        let assign104060_e156139: f64 = if ((assign104060_e156129 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104060_e156137)) { 1.0 } else { 0.0 };
        locals.var_guard2373 = assign104060_e156139;

        let (assign104070_e156150, assign104070_e156150_d_n0, assign104070_e156150_d_n2, assign104070_e156150_d_n4, assign104070_e156150_d_n5, assign104070_e156150_d_n6, assign104070_e156150_d_n7, assign104070_e156150_d_n8, assign104070_e156150_d_n9, assign104070_e156150_d_n10, assign104070_e156150_d_n11, assign104070_e156150_d_n14,) = {
    if (((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104070_e156148: f64 = (1.0 / locals.var_t4);
        (assign104070_e156148, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104070_e156150;
        locals.var_t5_dn0 = assign104070_e156150_d_n0;
        locals.var_t5_dn2 = assign104070_e156150_d_n2;
        locals.var_t5_dn4 = assign104070_e156150_d_n4;
        locals.var_t5_dn5 = assign104070_e156150_d_n5;
        locals.var_t5_dn6 = assign104070_e156150_d_n6;
        locals.var_t5_dn7 = assign104070_e156150_d_n7;
        locals.var_t5_dn8 = assign104070_e156150_d_n8;
        locals.var_t5_dn9 = assign104070_e156150_d_n9;
        locals.var_t5_dn10 = assign104070_e156150_d_n10;
        locals.var_t5_dn11 = assign104070_e156150_d_n11;
        locals.var_t5_dn14 = assign104070_e156150_d_n14;

        let assign104080_e156154: f64 = (10.0 * 2.220446049250313e-16);
        let assign104080_e156155: f64 = (2.0 - assign104080_e156154);
        let assign104080_e156162: f64 = (10.0 * 2.220446049250313e-16);
        let assign104080_e156163: f64 = (2.0 + assign104080_e156162);
        let assign104080_e156165: f64 = if ((assign104080_e156155 <= locals.var_uc_rdrbb) && (locals.var_uc_rdrbb <= assign104080_e156163)) { 1.0 } else { 0.0 };
        locals.var_guard2374 = assign104080_e156165;

        let (assign104090_e156180, assign104090_e156180_d_n0, assign104090_e156180_d_n2, assign104090_e156180_d_n4, assign104090_e156180_d_n5, assign104090_e156180_d_n6, assign104090_e156180_d_n7, assign104090_e156180_d_n8, assign104090_e156180_d_n9, assign104090_e156180_d_n10, assign104090_e156180_d_n11, assign104090_e156180_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 != 0.0)) {
        let assign104090_e156177: f64 = (locals.var_t4).sqrt();
        let assign104090_e156178: f64 = (1.0 / assign104090_e156177);
        (assign104090_e156178, (-((locals.var_t4_dn0 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn2 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn4 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn5 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn6 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn7 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn8 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn9 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn10 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn11 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))), (-((locals.var_t4_dn14 / (2.0 * assign104090_e156177)) / (assign104090_e156177 * assign104090_e156177))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104090_e156180;
        locals.var_t5_dn0 = assign104090_e156180_d_n0;
        locals.var_t5_dn2 = assign104090_e156180_d_n2;
        locals.var_t5_dn4 = assign104090_e156180_d_n4;
        locals.var_t5_dn5 = assign104090_e156180_d_n5;
        locals.var_t5_dn6 = assign104090_e156180_d_n6;
        locals.var_t5_dn7 = assign104090_e156180_d_n7;
        locals.var_t5_dn8 = assign104090_e156180_d_n8;
        locals.var_t5_dn9 = assign104090_e156180_d_n9;
        locals.var_t5_dn10 = assign104090_e156180_d_n10;
        locals.var_t5_dn11 = assign104090_e156180_d_n11;
        locals.var_t5_dn14 = assign104090_e156180_d_n14;

        let (assign104100_e156205, assign104100_e156205_d_n0, assign104100_e156205_d_n2, assign104100_e156205_d_n4, assign104100_e156205_d_n5, assign104100_e156205_d_n6, assign104100_e156205_d_n7, assign104100_e156205_d_n8, assign104100_e156205_d_n9, assign104100_e156205_d_n10, assign104100_e156205_d_n11, assign104100_e156205_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 == 0.0)) {
        let (assign104100_e156203, assign104100_e156203_d_n0, assign104100_e156203_d_n2, assign104100_e156203_d_n4, assign104100_e156203_d_n5, assign104100_e156203_d_n6, assign104100_e156203_d_n7, assign104100_e156203_d_n8, assign104100_e156203_d_n9, assign104100_e156203_d_n10, assign104100_e156203_d_n11, assign104100_e156203_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104100_e156197: f64 = (-1.0);
                let assign104100_e156199: f64 = (assign104100_e156197 / locals.var_uc_rdrbb);
                let assign104100_e156201: f64 = (assign104100_e156199 - 1.0);
                let assign104100_e156202: f64 = (locals.var_t4).powf(assign104100_e156201);
                (assign104100_e156202, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn0)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn0) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn0 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn2)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn2) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn2 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn4)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn4) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn4 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn5)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn5) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn5 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn6)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn6) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn6 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn7)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn7) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn7 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn8)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn8) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn8 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn9)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn9) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn9 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn10)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn10) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn10 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn11)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn11) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn11 / locals.var_t4)))) }, if (-((assign104100_e156197 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) == 0.0 && ((assign104100_e156201) as f64).is_finite() && ((assign104100_e156201) as f64).fract() == 0.0 { if assign104100_e156201 == 0.0 { 0.0 } else { (assign104100_e156201 * ((locals.var_t4).powf(assign104100_e156201 - 1.0) * locals.var_t4_dn14)) } } else { (assign104100_e156202 * (((-((assign104100_e156197 * locals.var_uc_rdrbb_dn14) / (locals.var_uc_rdrbb * locals.var_uc_rdrbb))) * (locals.var_t4).ln()) + (assign104100_e156201 * (locals.var_t4_dn14 / locals.var_t4)))) },)
            }
        };
        (assign104100_e156203, assign104100_e156203_d_n0, assign104100_e156203_d_n2, assign104100_e156203_d_n4, assign104100_e156203_d_n5, assign104100_e156203_d_n6, assign104100_e156203_d_n7, assign104100_e156203_d_n8, assign104100_e156203_d_n9, assign104100_e156203_d_n10, assign104100_e156203_d_n11, assign104100_e156203_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign104100_e156205;
        locals.var_t6_dn0 = assign104100_e156205_d_n0;
        locals.var_t6_dn2 = assign104100_e156205_d_n2;
        locals.var_t6_dn4 = assign104100_e156205_d_n4;
        locals.var_t6_dn5 = assign104100_e156205_d_n5;
        locals.var_t6_dn6 = assign104100_e156205_d_n6;
        locals.var_t6_dn7 = assign104100_e156205_d_n7;
        locals.var_t6_dn8 = assign104100_e156205_d_n8;
        locals.var_t6_dn9 = assign104100_e156205_d_n9;
        locals.var_t6_dn10 = assign104100_e156205_d_n10;
        locals.var_t6_dn11 = assign104100_e156205_d_n11;
        locals.var_t6_dn14 = assign104100_e156205_d_n14;

        let (assign104110_e156220, assign104110_e156220_d_n0, assign104110_e156220_d_n2, assign104110_e156220_d_n4, assign104110_e156220_d_n5, assign104110_e156220_d_n6, assign104110_e156220_d_n7, assign104110_e156220_d_n8, assign104110_e156220_d_n9, assign104110_e156220_d_n10, assign104110_e156220_d_n11, assign104110_e156220_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2373 == 0.0)) && (locals.var_guard2374 == 0.0)) {
        let assign104110_e156218: f64 = (locals.var_t4 * locals.var_t6);
        (assign104110_e156218, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104110_e156220;
        locals.var_t5_dn0 = assign104110_e156220_d_n0;
        locals.var_t5_dn2 = assign104110_e156220_d_n2;
        locals.var_t5_dn4 = assign104110_e156220_d_n4;
        locals.var_t5_dn5 = assign104110_e156220_d_n5;
        locals.var_t5_dn6 = assign104110_e156220_d_n6;
        locals.var_t5_dn7 = assign104110_e156220_d_n7;
        locals.var_t5_dn8 = assign104110_e156220_d_n8;
        locals.var_t5_dn9 = assign104110_e156220_d_n9;
        locals.var_t5_dn10 = assign104110_e156220_d_n10;
        locals.var_t5_dn11 = assign104110_e156220_d_n11;
        locals.var_t5_dn14 = assign104110_e156220_d_n14;

        let (assign104120_e156229, assign104120_e156229_d_n0, assign104120_e156229_d_n2, assign104120_e156229_d_n4, assign104120_e156229_d_n5, assign104120_e156229_d_n6, assign104120_e156229_d_n7, assign104120_e156229_d_n8, assign104120_e156229_d_n9, assign104120_e156229_d_n10, assign104120_e156229_d_n11, assign104120_e156229_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104120_e156227: f64 = (locals.var_mu0 * locals.var_t5);
        (assign104120_e156227, ((locals.var_mu0_dn0 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn0)), ((locals.var_mu0_dn2 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn2)), ((locals.var_mu0_dn4 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn4)), ((locals.var_mu0_dn5 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn5)), ((locals.var_mu0_dn6 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn6)), ((locals.var_mu0_dn7 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn7)), ((locals.var_mu0_dn8 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn8)), ((locals.var_mu0_dn9 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn9)), ((locals.var_mu0_dn10 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn10)), ((locals.var_mu0_dn11 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn11)), ((locals.var_mu0_dn14 * locals.var_t5) + (locals.var_mu0 * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu__blk2358, locals.var_mu__blk2358_dn0, locals.var_mu__blk2358_dn2, locals.var_mu__blk2358_dn4, locals.var_mu__blk2358_dn5, locals.var_mu__blk2358_dn6, locals.var_mu__blk2358_dn7, locals.var_mu__blk2358_dn8, locals.var_mu__blk2358_dn9, locals.var_mu__blk2358_dn10, locals.var_mu__blk2358_dn11, locals.var_mu__blk2358_dn14,)
    }
};
        locals.var_mu__blk2358 = assign104120_e156229;
        locals.var_mu__blk2358_dn0 = assign104120_e156229_d_n0;
        locals.var_mu__blk2358_dn2 = assign104120_e156229_d_n2;
        locals.var_mu__blk2358_dn4 = assign104120_e156229_d_n4;
        locals.var_mu__blk2358_dn5 = assign104120_e156229_d_n5;
        locals.var_mu__blk2358_dn6 = assign104120_e156229_d_n6;
        locals.var_mu__blk2358_dn7 = assign104120_e156229_d_n7;
        locals.var_mu__blk2358_dn8 = assign104120_e156229_d_n8;
        locals.var_mu__blk2358_dn9 = assign104120_e156229_d_n9;
        locals.var_mu__blk2358_dn10 = assign104120_e156229_d_n10;
        locals.var_mu__blk2358_dn11 = assign104120_e156229_d_n11;
        locals.var_mu__blk2358_dn14 = assign104120_e156229_d_n14;

        let (assign104130_e156238, assign104130_e156238_d_n0, assign104130_e156238_d_n2, assign104130_e156238_d_n4, assign104130_e156238_d_n5, assign104130_e156238_d_n6, assign104130_e156238_d_n7, assign104130_e156238_d_n8, assign104130_e156238_d_n9, assign104130_e156238_d_n10, assign104130_e156238_d_n11, assign104130_e156238_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104130_e156236: f64 = (1.0 + locals.var_t1);
        (assign104130_e156236, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104130_e156238;
        locals.var_t4_dn0 = assign104130_e156238_d_n0;
        locals.var_t4_dn2 = assign104130_e156238_d_n2;
        locals.var_t4_dn4 = assign104130_e156238_d_n4;
        locals.var_t4_dn5 = assign104130_e156238_d_n5;
        locals.var_t4_dn6 = assign104130_e156238_d_n6;
        locals.var_t4_dn7 = assign104130_e156238_d_n7;
        locals.var_t4_dn8 = assign104130_e156238_d_n8;
        locals.var_t4_dn9 = assign104130_e156238_d_n9;
        locals.var_t4_dn10 = assign104130_e156238_d_n10;
        locals.var_t4_dn11 = assign104130_e156238_d_n11;
        locals.var_t4_dn14 = assign104130_e156238_d_n14;

        let (assign104140_e156247, assign104140_e156247_d_n0, assign104140_e156247_d_n2, assign104140_e156247_d_n4, assign104140_e156247_d_n5, assign104140_e156247_d_n6, assign104140_e156247_d_n7, assign104140_e156247_d_n8, assign104140_e156247_d_n9, assign104140_e156247_d_n10, assign104140_e156247_d_n11, assign104140_e156247_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104140_e156245: f64 = (1.0 / locals.var_t4);
        (assign104140_e156245, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104140_e156247;
        locals.var_t5_dn0 = assign104140_e156247_d_n0;
        locals.var_t5_dn2 = assign104140_e156247_d_n2;
        locals.var_t5_dn4 = assign104140_e156247_d_n4;
        locals.var_t5_dn5 = assign104140_e156247_d_n5;
        locals.var_t5_dn6 = assign104140_e156247_d_n6;
        locals.var_t5_dn7 = assign104140_e156247_d_n7;
        locals.var_t5_dn8 = assign104140_e156247_d_n8;
        locals.var_t5_dn9 = assign104140_e156247_d_n9;
        locals.var_t5_dn10 = assign104140_e156247_d_n10;
        locals.var_t5_dn11 = assign104140_e156247_d_n11;
        locals.var_t5_dn14 = assign104140_e156247_d_n14;

        let (assign104150_e156266, assign104150_e156266_d_n0, assign104150_e156266_d_n2, assign104150_e156266_d_n4, assign104150_e156266_d_n5, assign104150_e156266_d_n6, assign104150_e156266_d_n7, assign104150_e156266_d_n8, assign104150_e156266_d_n9, assign104150_e156266_d_n10, assign104150_e156266_d_n11, assign104150_e156266_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104150_e156256: f64 = (1.0 - locals.var_t5);
        let assign104150_e156257: f64 = (locals.var_car * assign104150_e156256);
        let assign104150_e156259: f64 = (assign104150_e156257 * locals.var_vddpz);
        let assign104150_e156262: f64 = (locals.var_ldrifte - p.p423);
        let assign104150_e156263: f64 = (assign104150_e156259 / assign104150_e156262);
        let assign104150_e156264: f64 = (1.0 + assign104150_e156263);
        (assign104150_e156264, ((((locals.var_car * (-locals.var_t5_dn0)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn0)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn2)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn2)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn4)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn4)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn5)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn5)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn6)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn6)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn7)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn7)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn8)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn8)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn9)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn9)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn10)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn10)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn11)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn11)) / assign104150_e156262), ((((locals.var_car * (-locals.var_t5_dn14)) * locals.var_vddpz) + (assign104150_e156257 * locals.var_vddpz_dn14)) / assign104150_e156262),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign104150_e156266;
        locals.var_t4_dn0 = assign104150_e156266_d_n0;
        locals.var_t4_dn2 = assign104150_e156266_d_n2;
        locals.var_t4_dn4 = assign104150_e156266_d_n4;
        locals.var_t4_dn5 = assign104150_e156266_d_n5;
        locals.var_t4_dn6 = assign104150_e156266_d_n6;
        locals.var_t4_dn7 = assign104150_e156266_d_n7;
        locals.var_t4_dn8 = assign104150_e156266_d_n8;
        locals.var_t4_dn9 = assign104150_e156266_d_n9;
        locals.var_t4_dn10 = assign104150_e156266_d_n10;
        locals.var_t4_dn11 = assign104150_e156266_d_n11;
        locals.var_t4_dn14 = assign104150_e156266_d_n14;

        let (assign104160_e156277, assign104160_e156277_d_n0, assign104160_e156277_d_n2, assign104160_e156277_d_n4, assign104160_e156277_d_n5, assign104160_e156277_d_n6, assign104160_e156277_d_n7, assign104160_e156277_d_n8, assign104160_e156277_d_n9, assign104160_e156277_d_n10, assign104160_e156277_d_n11, assign104160_e156277_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104160_e156273: f64 = locals.var_t4;
        let assign104160_e156275: f64 = (assign104160_e156273 - 0.001);
        (assign104160_e156275, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104160_e156277;
        locals.var_tmf1_dn0 = assign104160_e156277_d_n0;
        locals.var_tmf1_dn2 = assign104160_e156277_d_n2;
        locals.var_tmf1_dn4 = assign104160_e156277_d_n4;
        locals.var_tmf1_dn5 = assign104160_e156277_d_n5;
        locals.var_tmf1_dn6 = assign104160_e156277_d_n6;
        locals.var_tmf1_dn7 = assign104160_e156277_d_n7;
        locals.var_tmf1_dn8 = assign104160_e156277_d_n8;
        locals.var_tmf1_dn9 = assign104160_e156277_d_n9;
        locals.var_tmf1_dn10 = assign104160_e156277_d_n10;
        locals.var_tmf1_dn11 = assign104160_e156277_d_n11;
        locals.var_tmf1_dn14 = assign104160_e156277_d_n14;

        let (assign104170_e156288, assign104170_e156288_d_n0, assign104170_e156288_d_n2, assign104170_e156288_d_n4, assign104170_e156288_d_n5, assign104170_e156288_d_n6, assign104170_e156288_d_n7, assign104170_e156288_d_n8, assign104170_e156288_d_n9, assign104170_e156288_d_n10, assign104170_e156288_d_n11, assign104170_e156288_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104170_e156288;
        locals.var_tmf2_dn0 = assign104170_e156288_d_n0;
        locals.var_tmf2_dn2 = assign104170_e156288_d_n2;
        locals.var_tmf2_dn4 = assign104170_e156288_d_n4;
        locals.var_tmf2_dn5 = assign104170_e156288_d_n5;
        locals.var_tmf2_dn6 = assign104170_e156288_d_n6;
        locals.var_tmf2_dn7 = assign104170_e156288_d_n7;
        locals.var_tmf2_dn8 = assign104170_e156288_d_n8;
        locals.var_tmf2_dn9 = assign104170_e156288_d_n9;
        locals.var_tmf2_dn10 = assign104170_e156288_d_n10;
        locals.var_tmf2_dn11 = assign104170_e156288_d_n11;
        locals.var_tmf2_dn14 = assign104170_e156288_d_n14;

        let (assign104180_e156301, assign104180_e156301_d_n0, assign104180_e156301_d_n2, assign104180_e156301_d_n4, assign104180_e156301_d_n5, assign104180_e156301_d_n6, assign104180_e156301_d_n7, assign104180_e156301_d_n8, assign104180_e156301_d_n9, assign104180_e156301_d_n10, assign104180_e156301_d_n11, assign104180_e156301_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let (assign104180_e156299, assign104180_e156299_d_n0, assign104180_e156299_d_n2, assign104180_e156299_d_n4, assign104180_e156299_d_n5, assign104180_e156299_d_n6, assign104180_e156299_d_n7, assign104180_e156299_d_n8, assign104180_e156299_d_n9, assign104180_e156299_d_n10, assign104180_e156299_d_n11, assign104180_e156299_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104180_e156298: f64 = (-locals.var_tmf2);
                (assign104180_e156298, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104180_e156299, assign104180_e156299_d_n0, assign104180_e156299_d_n2, assign104180_e156299_d_n4, assign104180_e156299_d_n5, assign104180_e156299_d_n6, assign104180_e156299_d_n7, assign104180_e156299_d_n8, assign104180_e156299_d_n9, assign104180_e156299_d_n10, assign104180_e156299_d_n11, assign104180_e156299_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104180_e156301;
        locals.var_tmf2_dn0 = assign104180_e156301_d_n0;
        locals.var_tmf2_dn2 = assign104180_e156301_d_n2;
        locals.var_tmf2_dn4 = assign104180_e156301_d_n4;
        locals.var_tmf2_dn5 = assign104180_e156301_d_n5;
        locals.var_tmf2_dn6 = assign104180_e156301_d_n6;
        locals.var_tmf2_dn7 = assign104180_e156301_d_n7;
        locals.var_tmf2_dn8 = assign104180_e156301_d_n8;
        locals.var_tmf2_dn9 = assign104180_e156301_d_n9;
        locals.var_tmf2_dn10 = assign104180_e156301_d_n10;
        locals.var_tmf2_dn11 = assign104180_e156301_d_n11;
        locals.var_tmf2_dn14 = assign104180_e156301_d_n14;

        let (assign104190_e156313, assign104190_e156313_d_n0, assign104190_e156313_d_n2, assign104190_e156313_d_n4, assign104190_e156313_d_n5, assign104190_e156313_d_n6, assign104190_e156313_d_n7, assign104190_e156313_d_n8, assign104190_e156313_d_n9, assign104190_e156313_d_n10, assign104190_e156313_d_n11, assign104190_e156313_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104190_e156308: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104190_e156310: f64 = (assign104190_e156308 + locals.var_tmf2);
        let assign104190_e156311: f64 = (assign104190_e156310).sqrt();
        (assign104190_e156311, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104190_e156311)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104190_e156311)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104190_e156313;
        locals.var_tmf2_dn0 = assign104190_e156313_d_n0;
        locals.var_tmf2_dn2 = assign104190_e156313_d_n2;
        locals.var_tmf2_dn4 = assign104190_e156313_d_n4;
        locals.var_tmf2_dn5 = assign104190_e156313_d_n5;
        locals.var_tmf2_dn6 = assign104190_e156313_d_n6;
        locals.var_tmf2_dn7 = assign104190_e156313_d_n7;
        locals.var_tmf2_dn8 = assign104190_e156313_d_n8;
        locals.var_tmf2_dn9 = assign104190_e156313_d_n9;
        locals.var_tmf2_dn10 = assign104190_e156313_d_n10;
        locals.var_tmf2_dn11 = assign104190_e156313_d_n11;
        locals.var_tmf2_dn14 = assign104190_e156313_d_n14;

        let (assign104200_e156326, assign104200_e156326_d_n0, assign104200_e156326_d_n2, assign104200_e156326_d_n4, assign104200_e156326_d_n5, assign104200_e156326_d_n6, assign104200_e156326_d_n7, assign104200_e156326_d_n8, assign104200_e156326_d_n9, assign104200_e156326_d_n10, assign104200_e156326_d_n11, assign104200_e156326_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104200_e156322: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104200_e156323: f64 = (1.0 + assign104200_e156322);
        let assign104200_e156324: f64 = (0.5 * assign104200_e156323);
        (assign104200_e156324, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104200_e156326;
        locals.var_t0_dn0 = assign104200_e156326_d_n0;
        locals.var_t0_dn2 = assign104200_e156326_d_n2;
        locals.var_t0_dn4 = assign104200_e156326_d_n4;
        locals.var_t0_dn5 = assign104200_e156326_d_n5;
        locals.var_t0_dn6 = assign104200_e156326_d_n6;
        locals.var_t0_dn7 = assign104200_e156326_d_n7;
        locals.var_t0_dn8 = assign104200_e156326_d_n8;
        locals.var_t0_dn9 = assign104200_e156326_d_n9;
        locals.var_t0_dn10 = assign104200_e156326_d_n10;
        locals.var_t0_dn11 = assign104200_e156326_d_n11;
        locals.var_t0_dn14 = assign104200_e156326_d_n14;

        let (assign104210_e156339, assign104210_e156339_d_n0, assign104210_e156339_d_n2, assign104210_e156339_d_n4, assign104210_e156339_d_n5, assign104210_e156339_d_n6, assign104210_e156339_d_n7, assign104210_e156339_d_n8, assign104210_e156339_d_n9, assign104210_e156339_d_n10, assign104210_e156339_d_n11, assign104210_e156339_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104210_e156335: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104210_e156336: f64 = (0.5 * assign104210_e156335);
        let assign104210_e156337: f64 = assign104210_e156336;
        (assign104210_e156337, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign104210_e156339;
        locals.var_t5_dn0 = assign104210_e156339_d_n0;
        locals.var_t5_dn2 = assign104210_e156339_d_n2;
        locals.var_t5_dn4 = assign104210_e156339_d_n4;
        locals.var_t5_dn5 = assign104210_e156339_d_n5;
        locals.var_t5_dn6 = assign104210_e156339_d_n6;
        locals.var_t5_dn7 = assign104210_e156339_d_n7;
        locals.var_t5_dn8 = assign104210_e156339_d_n8;
        locals.var_t5_dn9 = assign104210_e156339_d_n9;
        locals.var_t5_dn10 = assign104210_e156339_d_n10;
        locals.var_t5_dn11 = assign104210_e156339_d_n11;
        locals.var_t5_dn14 = assign104210_e156339_d_n14;

        let (assign104220_e156348, assign104220_e156348_d_n0, assign104220_e156348_d_n2, assign104220_e156348_d_n4, assign104220_e156348_d_n5, assign104220_e156348_d_n6, assign104220_e156348_d_n7, assign104220_e156348_d_n8, assign104220_e156348_d_n9, assign104220_e156348_d_n10, assign104220_e156348_d_n11, assign104220_e156348_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104220_e156346: f64 = (locals.var_noverd * locals.var_t5);
        (assign104220_e156346, (locals.var_noverd * locals.var_t5_dn0), (locals.var_noverd * locals.var_t5_dn2), (locals.var_noverd * locals.var_t5_dn4), (locals.var_noverd * locals.var_t5_dn5), (locals.var_noverd * locals.var_t5_dn6), (locals.var_noverd * locals.var_t5_dn7), (locals.var_noverd * locals.var_t5_dn8), (locals.var_noverd * locals.var_t5_dn9), (locals.var_noverd * locals.var_t5_dn10), (locals.var_noverd * locals.var_t5_dn11), (locals.var_noverd * locals.var_t5_dn14),)
    } else {
        (locals.var_carr1, locals.var_carr1_dn0, locals.var_carr1_dn2, locals.var_carr1_dn4, locals.var_carr1_dn5, locals.var_carr1_dn6, locals.var_carr1_dn7, locals.var_carr1_dn8, locals.var_carr1_dn9, locals.var_carr1_dn10, locals.var_carr1_dn11, locals.var_carr1_dn14,)
    }
};
        locals.var_carr1 = assign104220_e156348;
        locals.var_carr1_dn0 = assign104220_e156348_d_n0;
        locals.var_carr1_dn2 = assign104220_e156348_d_n2;
        locals.var_carr1_dn4 = assign104220_e156348_d_n4;
        locals.var_carr1_dn5 = assign104220_e156348_d_n5;
        locals.var_carr1_dn6 = assign104220_e156348_d_n6;
        locals.var_carr1_dn7 = assign104220_e156348_d_n7;
        locals.var_carr1_dn8 = assign104220_e156348_d_n8;
        locals.var_carr1_dn9 = assign104220_e156348_d_n9;
        locals.var_carr1_dn10 = assign104220_e156348_d_n10;
        locals.var_carr1_dn11 = assign104220_e156348_d_n11;
        locals.var_carr1_dn14 = assign104220_e156348_d_n14;

        let (assign104230_e156359, assign104230_e156359_d_n0, assign104230_e156359_d_n2, assign104230_e156359_d_n4, assign104230_e156359_d_n5, assign104230_e156359_d_n6, assign104230_e156359_d_n7, assign104230_e156359_d_n8, assign104230_e156359_d_n9, assign104230_e156359_d_n10, assign104230_e156359_d_n11, assign104230_e156359_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104230_e156355: f64 = (locals.var_rd_qbuld / 1.6021918e-19);
        let assign104230_e156357: f64 = (assign104230_e156355 * p.p430);
        (assign104230_e156357, ((locals.var_rd_qbuld_dn0 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn2 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn4 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn5 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn6 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn7 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn8 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn9 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn10 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn11 / 1.6021918e-19) * p.p430), ((locals.var_rd_qbuld_dn14 / 1.6021918e-19) * p.p430),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104230_e156359;
        locals.var_carr2_dn0 = assign104230_e156359_d_n0;
        locals.var_carr2_dn2 = assign104230_e156359_d_n2;
        locals.var_carr2_dn4 = assign104230_e156359_d_n4;
        locals.var_carr2_dn5 = assign104230_e156359_d_n5;
        locals.var_carr2_dn6 = assign104230_e156359_d_n6;
        locals.var_carr2_dn7 = assign104230_e156359_d_n7;
        locals.var_carr2_dn8 = assign104230_e156359_d_n8;
        locals.var_carr2_dn9 = assign104230_e156359_d_n9;
        locals.var_carr2_dn10 = assign104230_e156359_d_n10;
        locals.var_carr2_dn11 = assign104230_e156359_d_n11;
        locals.var_carr2_dn14 = assign104230_e156359_d_n14;

        let (assign104240_e156372, assign104240_e156372_d_n0, assign104240_e156372_d_n2, assign104240_e156372_d_n4, assign104240_e156372_d_n5, assign104240_e156372_d_n6, assign104240_e156372_d_n7, assign104240_e156372_d_n8, assign104240_e156372_d_n9, assign104240_e156372_d_n10, assign104240_e156372_d_n11, assign104240_e156372_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104240_e156366: f64 = (locals.var_carr1 - locals.var_carr2);
        let assign104240_e156369: f64 = (locals.var_carr1 * 0.001);
        let assign104240_e156370: f64 = (assign104240_e156366 - assign104240_e156369);
        (assign104240_e156370, ((locals.var_carr1_dn0 - locals.var_carr2_dn0) - (locals.var_carr1_dn0 * 0.001)), ((locals.var_carr1_dn2 - locals.var_carr2_dn2) - (locals.var_carr1_dn2 * 0.001)), ((locals.var_carr1_dn4 - locals.var_carr2_dn4) - (locals.var_carr1_dn4 * 0.001)), ((locals.var_carr1_dn5 - locals.var_carr2_dn5) - (locals.var_carr1_dn5 * 0.001)), ((locals.var_carr1_dn6 - locals.var_carr2_dn6) - (locals.var_carr1_dn6 * 0.001)), ((locals.var_carr1_dn7 - locals.var_carr2_dn7) - (locals.var_carr1_dn7 * 0.001)), ((locals.var_carr1_dn8 - locals.var_carr2_dn8) - (locals.var_carr1_dn8 * 0.001)), ((locals.var_carr1_dn9 - locals.var_carr2_dn9) - (locals.var_carr1_dn9 * 0.001)), ((locals.var_carr1_dn10 - locals.var_carr2_dn10) - (locals.var_carr1_dn10 * 0.001)), ((locals.var_carr1_dn11 - locals.var_carr2_dn11) - (locals.var_carr1_dn11 * 0.001)), ((locals.var_carr1_dn14 - locals.var_carr2_dn14) - (locals.var_carr1_dn14 * 0.001)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104240_e156372;
        locals.var_tmf1_dn0 = assign104240_e156372_d_n0;
        locals.var_tmf1_dn2 = assign104240_e156372_d_n2;
        locals.var_tmf1_dn4 = assign104240_e156372_d_n4;
        locals.var_tmf1_dn5 = assign104240_e156372_d_n5;
        locals.var_tmf1_dn6 = assign104240_e156372_d_n6;
        locals.var_tmf1_dn7 = assign104240_e156372_d_n7;
        locals.var_tmf1_dn8 = assign104240_e156372_d_n8;
        locals.var_tmf1_dn9 = assign104240_e156372_d_n9;
        locals.var_tmf1_dn10 = assign104240_e156372_d_n10;
        locals.var_tmf1_dn11 = assign104240_e156372_d_n11;
        locals.var_tmf1_dn14 = assign104240_e156372_d_n14;

        let (assign104250_e156385, assign104250_e156385_d_n0, assign104250_e156385_d_n2, assign104250_e156385_d_n4, assign104250_e156385_d_n5, assign104250_e156385_d_n6, assign104250_e156385_d_n7, assign104250_e156385_d_n8, assign104250_e156385_d_n9, assign104250_e156385_d_n10, assign104250_e156385_d_n11, assign104250_e156385_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104250_e156379: f64 = (4.0 * locals.var_carr1);
        let assign104250_e156382: f64 = (locals.var_carr1 * 0.001);
        let assign104250_e156383: f64 = (assign104250_e156379 * assign104250_e156382);
        (assign104250_e156383, (((4.0 * locals.var_carr1_dn0) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn0 * 0.001))), (((4.0 * locals.var_carr1_dn2) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn2 * 0.001))), (((4.0 * locals.var_carr1_dn4) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn4 * 0.001))), (((4.0 * locals.var_carr1_dn5) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn5 * 0.001))), (((4.0 * locals.var_carr1_dn6) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn6 * 0.001))), (((4.0 * locals.var_carr1_dn7) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn7 * 0.001))), (((4.0 * locals.var_carr1_dn8) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn8 * 0.001))), (((4.0 * locals.var_carr1_dn9) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn9 * 0.001))), (((4.0 * locals.var_carr1_dn10) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn10 * 0.001))), (((4.0 * locals.var_carr1_dn11) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn11 * 0.001))), (((4.0 * locals.var_carr1_dn14) * assign104250_e156382) + (assign104250_e156379 * (locals.var_carr1_dn14 * 0.001))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104250_e156385;
        locals.var_tmf2_dn0 = assign104250_e156385_d_n0;
        locals.var_tmf2_dn2 = assign104250_e156385_d_n2;
        locals.var_tmf2_dn4 = assign104250_e156385_d_n4;
        locals.var_tmf2_dn5 = assign104250_e156385_d_n5;
        locals.var_tmf2_dn6 = assign104250_e156385_d_n6;
        locals.var_tmf2_dn7 = assign104250_e156385_d_n7;
        locals.var_tmf2_dn8 = assign104250_e156385_d_n8;
        locals.var_tmf2_dn9 = assign104250_e156385_d_n9;
        locals.var_tmf2_dn10 = assign104250_e156385_d_n10;
        locals.var_tmf2_dn11 = assign104250_e156385_d_n11;
        locals.var_tmf2_dn14 = assign104250_e156385_d_n14;

    }

    pub(super) fn stamp_transient_block_383(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104260_e156398, assign104260_e156398_d_n0, assign104260_e156398_d_n2, assign104260_e156398_d_n4, assign104260_e156398_d_n5, assign104260_e156398_d_n6, assign104260_e156398_d_n7, assign104260_e156398_d_n8, assign104260_e156398_d_n9, assign104260_e156398_d_n10, assign104260_e156398_d_n11, assign104260_e156398_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let (assign104260_e156396, assign104260_e156396_d_n0, assign104260_e156396_d_n2, assign104260_e156396_d_n4, assign104260_e156396_d_n5, assign104260_e156396_d_n6, assign104260_e156396_d_n7, assign104260_e156396_d_n8, assign104260_e156396_d_n9, assign104260_e156396_d_n10, assign104260_e156396_d_n11, assign104260_e156396_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign104260_e156395: f64 = (-locals.var_tmf2);
                (assign104260_e156395, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign104260_e156396, assign104260_e156396_d_n0, assign104260_e156396_d_n2, assign104260_e156396_d_n4, assign104260_e156396_d_n5, assign104260_e156396_d_n6, assign104260_e156396_d_n7, assign104260_e156396_d_n8, assign104260_e156396_d_n9, assign104260_e156396_d_n10, assign104260_e156396_d_n11, assign104260_e156396_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104260_e156398;
        locals.var_tmf2_dn0 = assign104260_e156398_d_n0;
        locals.var_tmf2_dn2 = assign104260_e156398_d_n2;
        locals.var_tmf2_dn4 = assign104260_e156398_d_n4;
        locals.var_tmf2_dn5 = assign104260_e156398_d_n5;
        locals.var_tmf2_dn6 = assign104260_e156398_d_n6;
        locals.var_tmf2_dn7 = assign104260_e156398_d_n7;
        locals.var_tmf2_dn8 = assign104260_e156398_d_n8;
        locals.var_tmf2_dn9 = assign104260_e156398_d_n9;
        locals.var_tmf2_dn10 = assign104260_e156398_d_n10;
        locals.var_tmf2_dn11 = assign104260_e156398_d_n11;
        locals.var_tmf2_dn14 = assign104260_e156398_d_n14;

        let (assign104270_e156410, assign104270_e156410_d_n0, assign104270_e156410_d_n2, assign104270_e156410_d_n4, assign104270_e156410_d_n5, assign104270_e156410_d_n6, assign104270_e156410_d_n7, assign104270_e156410_d_n8, assign104270_e156410_d_n9, assign104270_e156410_d_n10, assign104270_e156410_d_n11, assign104270_e156410_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104270_e156405: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104270_e156407: f64 = (assign104270_e156405 + locals.var_tmf2);
        let assign104270_e156408: f64 = (assign104270_e156407).sqrt();
        (assign104270_e156408, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign104270_e156408)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign104270_e156408)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign104270_e156410;
        locals.var_tmf2_dn0 = assign104270_e156410_d_n0;
        locals.var_tmf2_dn2 = assign104270_e156410_d_n2;
        locals.var_tmf2_dn4 = assign104270_e156410_d_n4;
        locals.var_tmf2_dn5 = assign104270_e156410_d_n5;
        locals.var_tmf2_dn6 = assign104270_e156410_d_n6;
        locals.var_tmf2_dn7 = assign104270_e156410_d_n7;
        locals.var_tmf2_dn8 = assign104270_e156410_d_n8;
        locals.var_tmf2_dn9 = assign104270_e156410_d_n9;
        locals.var_tmf2_dn10 = assign104270_e156410_d_n10;
        locals.var_tmf2_dn11 = assign104270_e156410_d_n11;
        locals.var_tmf2_dn14 = assign104270_e156410_d_n14;

        let (assign104280_e156423, assign104280_e156423_d_n0, assign104280_e156423_d_n2, assign104280_e156423_d_n4, assign104280_e156423_d_n5, assign104280_e156423_d_n6, assign104280_e156423_d_n7, assign104280_e156423_d_n8, assign104280_e156423_d_n9, assign104280_e156423_d_n10, assign104280_e156423_d_n11, assign104280_e156423_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104280_e156419: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104280_e156420: f64 = (1.0 + assign104280_e156419);
        let assign104280_e156421: f64 = (0.5 * assign104280_e156420);
        (assign104280_e156421, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign104280_e156423;
        locals.var_t0_dn0 = assign104280_e156423_d_n0;
        locals.var_t0_dn2 = assign104280_e156423_d_n2;
        locals.var_t0_dn4 = assign104280_e156423_d_n4;
        locals.var_t0_dn5 = assign104280_e156423_d_n5;
        locals.var_t0_dn6 = assign104280_e156423_d_n6;
        locals.var_t0_dn7 = assign104280_e156423_d_n7;
        locals.var_t0_dn8 = assign104280_e156423_d_n8;
        locals.var_t0_dn9 = assign104280_e156423_d_n9;
        locals.var_t0_dn10 = assign104280_e156423_d_n10;
        locals.var_t0_dn11 = assign104280_e156423_d_n11;
        locals.var_t0_dn14 = assign104280_e156423_d_n14;

        let (assign104290_e156436, assign104290_e156436_d_n0, assign104290_e156436_d_n2, assign104290_e156436_d_n4, assign104290_e156436_d_n5, assign104290_e156436_d_n6, assign104290_e156436_d_n7, assign104290_e156436_d_n8, assign104290_e156436_d_n9, assign104290_e156436_d_n10, assign104290_e156436_d_n11, assign104290_e156436_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104290_e156432: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104290_e156433: f64 = (0.5 * assign104290_e156432);
        let assign104290_e156434: f64 = (locals.var_carr1 - assign104290_e156433);
        (assign104290_e156434, (locals.var_carr1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_carr1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_carr1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_carr1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_carr1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_carr1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_carr1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_carr1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_carr1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_carr1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_carr1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_carr2, locals.var_carr2_dn0, locals.var_carr2_dn2, locals.var_carr2_dn4, locals.var_carr2_dn5, locals.var_carr2_dn6, locals.var_carr2_dn7, locals.var_carr2_dn8, locals.var_carr2_dn9, locals.var_carr2_dn10, locals.var_carr2_dn11, locals.var_carr2_dn14,)
    }
};
        locals.var_carr2 = assign104290_e156436;
        locals.var_carr2_dn0 = assign104290_e156436_d_n0;
        locals.var_carr2_dn2 = assign104290_e156436_d_n2;
        locals.var_carr2_dn4 = assign104290_e156436_d_n4;
        locals.var_carr2_dn5 = assign104290_e156436_d_n5;
        locals.var_carr2_dn6 = assign104290_e156436_d_n6;
        locals.var_carr2_dn7 = assign104290_e156436_d_n7;
        locals.var_carr2_dn8 = assign104290_e156436_d_n8;
        locals.var_carr2_dn9 = assign104290_e156436_d_n9;
        locals.var_carr2_dn10 = assign104290_e156436_d_n10;
        locals.var_carr2_dn11 = assign104290_e156436_d_n11;
        locals.var_carr2_dn14 = assign104290_e156436_d_n14;

        let (assign104300_e156445, assign104300_e156445_d_n0, assign104300_e156445_d_n2, assign104300_e156445_d_n4, assign104300_e156445_d_n5, assign104300_e156445_d_n6, assign104300_e156445_d_n7, assign104300_e156445_d_n8, assign104300_e156445_d_n9, assign104300_e156445_d_n10, assign104300_e156445_d_n11, assign104300_e156445_d_n14,) = {
    if ((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) {
        let assign104300_e156443: f64 = (locals.var_carr1 - locals.var_carr2);
        (assign104300_e156443, (locals.var_carr1_dn0 - locals.var_carr2_dn0), (locals.var_carr1_dn2 - locals.var_carr2_dn2), (locals.var_carr1_dn4 - locals.var_carr2_dn4), (locals.var_carr1_dn5 - locals.var_carr2_dn5), (locals.var_carr1_dn6 - locals.var_carr2_dn6), (locals.var_carr1_dn7 - locals.var_carr2_dn7), (locals.var_carr1_dn8 - locals.var_carr2_dn8), (locals.var_carr1_dn9 - locals.var_carr2_dn9), (locals.var_carr1_dn10 - locals.var_carr2_dn10), (locals.var_carr1_dn11 - locals.var_carr2_dn11), (locals.var_carr1_dn14 - locals.var_carr2_dn14),)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    }
};
        locals.var_carr = assign104300_e156445;
        locals.var_carr_dn0 = assign104300_e156445_d_n0;
        locals.var_carr_dn2 = assign104300_e156445_d_n2;
        locals.var_carr_dn4 = assign104300_e156445_d_n4;
        locals.var_carr_dn5 = assign104300_e156445_d_n5;
        locals.var_carr_dn6 = assign104300_e156445_d_n6;
        locals.var_carr_dn7 = assign104300_e156445_d_n7;
        locals.var_carr_dn8 = assign104300_e156445_d_n8;
        locals.var_carr_dn9 = assign104300_e156445_d_n9;
        locals.var_carr_dn10 = assign104300_e156445_d_n10;
        locals.var_carr_dn11 = assign104300_e156445_d_n11;
        locals.var_carr_dn14 = assign104300_e156445_d_n14;

        let assign104310_e156452: f64 = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2375 = assign104310_e156452;

        let assign104320_e156456: f64 = (locals.var_noverd * p.p440);
        let assign104320_e156459: f64 = (locals.var_noverd * p.p441);
        let assign104320_e156460: f64 = (assign104320_e156456 - assign104320_e156459);
        let assign104320_e156464: f64 = (locals.var_noverd * p.p441);
        let assign104320_e156467: f64 = if ((locals.var_carr > assign104320_e156460) && (assign104320_e156464 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2376 = assign104320_e156467;

        let (assign104330_e156486, assign104330_e156486_d_n0, assign104330_e156486_d_n2, assign104330_e156486_d_n4, assign104330_e156486_d_n5, assign104330_e156486_d_n6, assign104330_e156486_d_n7, assign104330_e156486_d_n8, assign104330_e156486_d_n9, assign104330_e156486_d_n10, assign104330_e156486_d_n11, assign104330_e156486_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104330_e156479: f64 = (locals.var_noverd * p.p440);
        let assign104330_e156480: f64 = (locals.var_carr - assign104330_e156479);
        let assign104330_e156483: f64 = (locals.var_noverd * p.p441);
        let assign104330_e156484: f64 = (assign104330_e156480 + assign104330_e156483);
        (assign104330_e156484, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn11, locals.var_carr_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign104330_e156486;
        locals.var_tmf1_dn0 = assign104330_e156486_d_n0;
        locals.var_tmf1_dn2 = assign104330_e156486_d_n2;
        locals.var_tmf1_dn4 = assign104330_e156486_d_n4;
        locals.var_tmf1_dn5 = assign104330_e156486_d_n5;
        locals.var_tmf1_dn6 = assign104330_e156486_d_n6;
        locals.var_tmf1_dn7 = assign104330_e156486_d_n7;
        locals.var_tmf1_dn8 = assign104330_e156486_d_n8;
        locals.var_tmf1_dn9 = assign104330_e156486_d_n9;
        locals.var_tmf1_dn10 = assign104330_e156486_d_n10;
        locals.var_tmf1_dn11 = assign104330_e156486_d_n11;
        locals.var_tmf1_dn14 = assign104330_e156486_d_n14;

        let (assign104340_e156499, assign104340_e156499_d_n0, assign104340_e156499_d_n2, assign104340_e156499_d_n4, assign104340_e156499_d_n5, assign104340_e156499_d_n6, assign104340_e156499_d_n7, assign104340_e156499_d_n8, assign104340_e156499_d_n9, assign104340_e156499_d_n10, assign104340_e156499_d_n11, assign104340_e156499_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104340_e156497: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104340_e156497, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign104340_e156499;
        locals.var_x2_dn0 = assign104340_e156499_d_n0;
        locals.var_x2_dn2 = assign104340_e156499_d_n2;
        locals.var_x2_dn4 = assign104340_e156499_d_n4;
        locals.var_x2_dn5 = assign104340_e156499_d_n5;
        locals.var_x2_dn6 = assign104340_e156499_d_n6;
        locals.var_x2_dn7 = assign104340_e156499_d_n7;
        locals.var_x2_dn8 = assign104340_e156499_d_n8;
        locals.var_x2_dn9 = assign104340_e156499_d_n9;
        locals.var_x2_dn10 = assign104340_e156499_d_n10;
        locals.var_x2_dn11 = assign104340_e156499_d_n11;
        locals.var_x2_dn14 = assign104340_e156499_d_n14;

        let (assign104350_e156516, assign104350_e156516_d_n0, assign104350_e156516_d_n2, assign104350_e156516_d_n4, assign104350_e156516_d_n5, assign104350_e156516_d_n6, assign104350_e156516_d_n7, assign104350_e156516_d_n8, assign104350_e156516_d_n9, assign104350_e156516_d_n10, assign104350_e156516_d_n11, assign104350_e156516_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104350_e156510: f64 = (locals.var_noverd * p.p441);
        let assign104350_e156513: f64 = (locals.var_noverd * p.p441);
        let assign104350_e156514: f64 = (assign104350_e156510 * assign104350_e156513);
        (assign104350_e156514, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign104350_e156516;
        locals.var_xmax2_dn0 = assign104350_e156516_d_n0;
        locals.var_xmax2_dn2 = assign104350_e156516_d_n2;
        locals.var_xmax2_dn4 = assign104350_e156516_d_n4;
        locals.var_xmax2_dn5 = assign104350_e156516_d_n5;
        locals.var_xmax2_dn6 = assign104350_e156516_d_n6;
        locals.var_xmax2_dn7 = assign104350_e156516_d_n7;
        locals.var_xmax2_dn8 = assign104350_e156516_d_n8;
        locals.var_xmax2_dn9 = assign104350_e156516_d_n9;
        locals.var_xmax2_dn10 = assign104350_e156516_d_n10;
        locals.var_xmax2_dn11 = assign104350_e156516_d_n11;
        locals.var_xmax2_dn14 = assign104350_e156516_d_n14;

        let (assign104360_e156527, assign104360_e156527_d_n0, assign104360_e156527_d_n2, assign104360_e156527_d_n4, assign104360_e156527_d_n5, assign104360_e156527_d_n6, assign104360_e156527_d_n7, assign104360_e156527_d_n8, assign104360_e156527_d_n9, assign104360_e156527_d_n10, assign104360_e156527_d_n11, assign104360_e156527_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign104360_e156527;
        locals.var_xp_dn0 = assign104360_e156527_d_n0;
        locals.var_xp_dn2 = assign104360_e156527_d_n2;
        locals.var_xp_dn4 = assign104360_e156527_d_n4;
        locals.var_xp_dn5 = assign104360_e156527_d_n5;
        locals.var_xp_dn6 = assign104360_e156527_d_n6;
        locals.var_xp_dn7 = assign104360_e156527_d_n7;
        locals.var_xp_dn8 = assign104360_e156527_d_n8;
        locals.var_xp_dn9 = assign104360_e156527_d_n9;
        locals.var_xp_dn10 = assign104360_e156527_d_n10;
        locals.var_xp_dn11 = assign104360_e156527_d_n11;
        locals.var_xp_dn14 = assign104360_e156527_d_n14;

        let (assign104370_e156538, assign104370_e156538_d_n0, assign104370_e156538_d_n2, assign104370_e156538_d_n4, assign104370_e156538_d_n5, assign104370_e156538_d_n6, assign104370_e156538_d_n7, assign104370_e156538_d_n8, assign104370_e156538_d_n9, assign104370_e156538_d_n10, assign104370_e156538_d_n11, assign104370_e156538_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign104370_e156538;
        locals.var_xmp_dn0 = assign104370_e156538_d_n0;
        locals.var_xmp_dn2 = assign104370_e156538_d_n2;
        locals.var_xmp_dn4 = assign104370_e156538_d_n4;
        locals.var_xmp_dn5 = assign104370_e156538_d_n5;
        locals.var_xmp_dn6 = assign104370_e156538_d_n6;
        locals.var_xmp_dn7 = assign104370_e156538_d_n7;
        locals.var_xmp_dn8 = assign104370_e156538_d_n8;
        locals.var_xmp_dn9 = assign104370_e156538_d_n9;
        locals.var_xmp_dn10 = assign104370_e156538_d_n10;
        locals.var_xmp_dn11 = assign104370_e156538_d_n11;
        locals.var_xmp_dn14 = assign104370_e156538_d_n14;

        let (assign104380_e156549,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104380_e156549;

        let (assign104390_e156560,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104390_e156560;

        let (assign104400_e156571, assign104400_e156571_d_n0, assign104400_e156571_d_n2, assign104400_e156571_d_n4, assign104400_e156571_d_n5, assign104400_e156571_d_n6, assign104400_e156571_d_n7, assign104400_e156571_d_n8, assign104400_e156571_d_n9, assign104400_e156571_d_n10, assign104400_e156571_d_n11, assign104400_e156571_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104400_e156571;
        locals.var_arg_dn0 = assign104400_e156571_d_n0;
        locals.var_arg_dn2 = assign104400_e156571_d_n2;
        locals.var_arg_dn4 = assign104400_e156571_d_n4;
        locals.var_arg_dn5 = assign104400_e156571_d_n5;
        locals.var_arg_dn6 = assign104400_e156571_d_n6;
        locals.var_arg_dn7 = assign104400_e156571_d_n7;
        locals.var_arg_dn8 = assign104400_e156571_d_n8;
        locals.var_arg_dn9 = assign104400_e156571_d_n9;
        locals.var_arg_dn10 = assign104400_e156571_d_n10;
        locals.var_arg_dn11 = assign104400_e156571_d_n11;
        locals.var_arg_dn14 = assign104400_e156571_d_n14;

        let (assign104410_e156582, assign104410_e156582_d_n0, assign104410_e156582_d_n2, assign104410_e156582_d_n4, assign104410_e156582_d_n5, assign104410_e156582_d_n6, assign104410_e156582_d_n7, assign104410_e156582_d_n8, assign104410_e156582_d_n9, assign104410_e156582_d_n10, assign104410_e156582_d_n11, assign104410_e156582_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104410_e156582;
        locals.var_dnm_dn0 = assign104410_e156582_d_n0;
        locals.var_dnm_dn2 = assign104410_e156582_d_n2;
        locals.var_dnm_dn4 = assign104410_e156582_d_n4;
        locals.var_dnm_dn5 = assign104410_e156582_d_n5;
        locals.var_dnm_dn6 = assign104410_e156582_d_n6;
        locals.var_dnm_dn7 = assign104410_e156582_d_n7;
        locals.var_dnm_dn8 = assign104410_e156582_d_n8;
        locals.var_dnm_dn9 = assign104410_e156582_d_n9;
        locals.var_dnm_dn10 = assign104410_e156582_d_n10;
        locals.var_dnm_dn11 = assign104410_e156582_d_n11;
        locals.var_dnm_dn14 = assign104410_e156582_d_n14;

        let (assign104420_e156593,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104420_e156593;

        let mut assign104430_loop_guard: usize = 0;
        while {
            let assign104430_cond_e156605: f64 = if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104430_cond_e156605 != 0.0
        } {
            assign104430_loop_guard += 1;
            assert!(assign104430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104430_body0_e156618, assign104430_body0_e156618_d_n0, assign104430_body0_e156618_d_n2, assign104430_body0_e156618_d_n4, assign104430_body0_e156618_d_n5, assign104430_body0_e156618_d_n6, assign104430_body0_e156618_d_n7, assign104430_body0_e156618_d_n8, assign104430_body0_e156618_d_n9, assign104430_body0_e156618_d_n10, assign104430_body0_e156618_d_n11, assign104430_body0_e156618_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body0_e156616: f64 = (locals.var_xp * locals.var_x2);
        (assign104430_body0_e156616, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign104430_body0_e156618;
            locals.var_xp_dn0 = assign104430_body0_e156618_d_n0;
            locals.var_xp_dn2 = assign104430_body0_e156618_d_n2;
            locals.var_xp_dn4 = assign104430_body0_e156618_d_n4;
            locals.var_xp_dn5 = assign104430_body0_e156618_d_n5;
            locals.var_xp_dn6 = assign104430_body0_e156618_d_n6;
            locals.var_xp_dn7 = assign104430_body0_e156618_d_n7;
            locals.var_xp_dn8 = assign104430_body0_e156618_d_n8;
            locals.var_xp_dn9 = assign104430_body0_e156618_d_n9;
            locals.var_xp_dn10 = assign104430_body0_e156618_d_n10;
            locals.var_xp_dn11 = assign104430_body0_e156618_d_n11;
            locals.var_xp_dn14 = assign104430_body0_e156618_d_n14;
            let (assign104430_body1_e156631, assign104430_body1_e156631_d_n0, assign104430_body1_e156631_d_n2, assign104430_body1_e156631_d_n4, assign104430_body1_e156631_d_n5, assign104430_body1_e156631_d_n6, assign104430_body1_e156631_d_n7, assign104430_body1_e156631_d_n8, assign104430_body1_e156631_d_n9, assign104430_body1_e156631_d_n10, assign104430_body1_e156631_d_n11, assign104430_body1_e156631_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body1_e156629: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign104430_body1_e156629, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign104430_body1_e156631;
            locals.var_xmp_dn0 = assign104430_body1_e156631_d_n0;
            locals.var_xmp_dn2 = assign104430_body1_e156631_d_n2;
            locals.var_xmp_dn4 = assign104430_body1_e156631_d_n4;
            locals.var_xmp_dn5 = assign104430_body1_e156631_d_n5;
            locals.var_xmp_dn6 = assign104430_body1_e156631_d_n6;
            locals.var_xmp_dn7 = assign104430_body1_e156631_d_n7;
            locals.var_xmp_dn8 = assign104430_body1_e156631_d_n8;
            locals.var_xmp_dn9 = assign104430_body1_e156631_d_n9;
            locals.var_xmp_dn10 = assign104430_body1_e156631_d_n10;
            locals.var_xmp_dn11 = assign104430_body1_e156631_d_n11;
            locals.var_xmp_dn14 = assign104430_body1_e156631_d_n14;
            let (assign104430_body2_e156644,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104430_body2_e156642: f64 = (locals.var_m0 + 1.0);
        (assign104430_body2_e156642,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104430_body2_e156644;
        }

        let (assign104440_e156657, assign104440_e156657_d_n0, assign104440_e156657_d_n2, assign104440_e156657_d_n4, assign104440_e156657_d_n5, assign104440_e156657_d_n6, assign104440_e156657_d_n7, assign104440_e156657_d_n8, assign104440_e156657_d_n9, assign104440_e156657_d_n10, assign104440_e156657_d_n11, assign104440_e156657_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        let assign104440_e156655: f64 = (locals.var_xp + locals.var_xmp);
        (assign104440_e156655, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign104440_e156657;
        locals.var_arg_dn0 = assign104440_e156657_d_n0;
        locals.var_arg_dn2 = assign104440_e156657_d_n2;
        locals.var_arg_dn4 = assign104440_e156657_d_n4;
        locals.var_arg_dn5 = assign104440_e156657_d_n5;
        locals.var_arg_dn6 = assign104440_e156657_d_n6;
        locals.var_arg_dn7 = assign104440_e156657_d_n7;
        locals.var_arg_dn8 = assign104440_e156657_d_n8;
        locals.var_arg_dn9 = assign104440_e156657_d_n9;
        locals.var_arg_dn10 = assign104440_e156657_d_n10;
        locals.var_arg_dn11 = assign104440_e156657_d_n11;
        locals.var_arg_dn14 = assign104440_e156657_d_n14;

        let (assign104450_e156668, assign104450_e156668_d_n0, assign104450_e156668_d_n2, assign104450_e156668_d_n4, assign104450_e156668_d_n5, assign104450_e156668_d_n6, assign104450_e156668_d_n7, assign104450_e156668_d_n8, assign104450_e156668_d_n9, assign104450_e156668_d_n10, assign104450_e156668_d_n11, assign104450_e156668_d_n14,) = {
    if ((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign104450_e156668;
        locals.var_dnm_dn0 = assign104450_e156668_d_n0;
        locals.var_dnm_dn2 = assign104450_e156668_d_n2;
        locals.var_dnm_dn4 = assign104450_e156668_d_n4;
        locals.var_dnm_dn5 = assign104450_e156668_d_n5;
        locals.var_dnm_dn6 = assign104450_e156668_d_n6;
        locals.var_dnm_dn7 = assign104450_e156668_d_n7;
        locals.var_dnm_dn8 = assign104450_e156668_d_n8;
        locals.var_dnm_dn9 = assign104450_e156668_d_n9;
        locals.var_dnm_dn10 = assign104450_e156668_d_n10;
        locals.var_dnm_dn11 = assign104450_e156668_d_n11;
        locals.var_dnm_dn14 = assign104450_e156668_d_n14;

        let assign104460_e156683: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2377 = assign104460_e156683;

        let assign104470_e156686: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2378 = assign104470_e156686;

        let (assign104480_e156701,) = {
    if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104480_e156701;

        let assign104490_e156704: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2379 = assign104490_e156704;

        let (assign104500_e156722,) = {
    if (((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104500_e156722;

        let assign104510_e156725: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2380 = assign104510_e156725;

        let (assign104520_e156746,) = {
    if ((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104520_e156746;

        let assign104530_e156749: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2381 = assign104530_e156749;

        let (assign104540_e156773,) = {
    if (((((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_guard2378 == 0.0)) && (locals.var_guard2379 == 0.0)) && (locals.var_guard2380 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104540_e156773;

        let (assign104550_e156786,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104550_e156786;

        let mut assign104560_loop_guard: usize = 0;
        while {
            let assign104560_cond_e156800: f64 = if ((((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign104560_cond_e156800 != 0.0
        } {
            assign104560_loop_guard += 1;
            assert!(assign104560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104560_body0_e156814, assign104560_body0_e156814_d_n0, assign104560_body0_e156814_d_n2, assign104560_body0_e156814_d_n4, assign104560_body0_e156814_d_n5, assign104560_body0_e156814_d_n6, assign104560_body0_e156814_d_n7, assign104560_body0_e156814_d_n8, assign104560_body0_e156814_d_n9, assign104560_body0_e156814_d_n10, assign104560_body0_e156814_d_n11, assign104560_body0_e156814_d_n14,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        let assign104560_body0_e156812: f64 = (locals.var_dnm).sqrt();
        (assign104560_body0_e156812, (locals.var_dnm_dn0 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn2 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn4 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn5 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn6 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn7 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn8 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn9 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn10 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn11 / (2.0 * assign104560_body0_e156812)), (locals.var_dnm_dn14 / (2.0 * assign104560_body0_e156812)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign104560_body0_e156814;
            locals.var_dnm_dn0 = assign104560_body0_e156814_d_n0;
            locals.var_dnm_dn2 = assign104560_body0_e156814_d_n2;
            locals.var_dnm_dn4 = assign104560_body0_e156814_d_n4;
            locals.var_dnm_dn5 = assign104560_body0_e156814_d_n5;
            locals.var_dnm_dn6 = assign104560_body0_e156814_d_n6;
            locals.var_dnm_dn7 = assign104560_body0_e156814_d_n7;
            locals.var_dnm_dn8 = assign104560_body0_e156814_d_n8;
            locals.var_dnm_dn9 = assign104560_body0_e156814_d_n9;
            locals.var_dnm_dn10 = assign104560_body0_e156814_d_n10;
            locals.var_dnm_dn11 = assign104560_body0_e156814_d_n11;
            locals.var_dnm_dn14 = assign104560_body0_e156814_d_n14;
            let (assign104560_body1_e156829,) = {
    if (((((locals.var_guard2340 != 0.0) && (locals.var_guard2360 == 0.0)) && (locals.var_guard2375 != 0.0)) && (locals.var_guard2376 != 0.0)) && (locals.var_guard2377 != 0.0)) {
        let assign104560_body1_e156827: f64 = (locals.var_m0 + 1.0);
        (assign104560_body1_e156827,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104560_body1_e156829;
        }

    }
}
