#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq24_e1449: f64 = (-locals.var_devsign);
        let eq24_e1451: f64 = (eq24_e1449 * locals.var_qovb);
        let eq24_e1451_d_n0: f64 = (eq24_e1449 * locals.var_qovb_dn0);
        let eq24_e1451_d_n2: f64 = (eq24_e1449 * locals.var_qovb_dn2);
        let eq24_e1451_d_n3: f64 = (eq24_e1449 * locals.var_qovb_dn3);
        let eq24_e1451_d_n4: f64 = (eq24_e1449 * locals.var_qovb_dn4);
        let eq24_e1451_d_n5: f64 = (eq24_e1449 * locals.var_qovb_dn5);
        let eq24_e1451_d_n6: f64 = (eq24_e1449 * locals.var_qovb_dn6);
        let eq24_e1451_d_n7: f64 = (eq24_e1449 * locals.var_qovb_dn7);
        let eq24_e1451_d_n8: f64 = (eq24_e1449 * locals.var_qovb_dn8);
        let eq24_e1451_d_n9: f64 = (eq24_e1449 * locals.var_qovb_dn9);
        let eq24_e1451_d_n10: f64 = (eq24_e1449 * locals.var_qovb_dn10);
        let eq24_e1451_d_n11: f64 = (eq24_e1449 * locals.var_qovb_dn11);
        let eq24_e1451_d_n12: f64 = (eq24_e1449 * locals.var_qovb_dn12);
        let eq24_e1451_d_n13: f64 = (eq24_e1449 * locals.var_qovb_dn13);
        let eq24_e1451_d_n14: f64 = (eq24_e1449 * locals.var_qovb_dn14);
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, 0.0, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, 0.0, 0.0];
        let eq24_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_q, eq55_e1708_q_d_n4,) = {
    if (locals.var_guard763 != 0.0) {
        let eq55_e1699: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq55_e1699_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq55_e1702: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq55_e1702_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - locals.var_pdiss);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - locals.var_pdiss_dn4);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq55_e1706_d_n4, (-locals.var_pdiss_dn5), (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), (-locals.var_pdiss_dn12), (-locals.var_pdiss_dn13), (-locals.var_pdiss_dn14), eq55_e1706_q, eq55_e1702_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq55_e1708_q_d_n4),
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_q,) = {
    if (locals.var_guard769 != 0.0) {
        let eq71_e1837: f64 = (p.p29 * locals.var_qbsj);
        let eq71_e1837_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq71_e1837_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq71_e1837_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq71_e1837_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq71_e1837_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq71_e1837_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq71_e1837_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq71_e1837_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq71_e1837_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq71_e1837_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq71_e1837_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq71_e1837_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq71_e1837_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq71_e1837_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (locals.var_devsign * eq71_e1837);
        let eq71_e1839_d_n0: f64 = (locals.var_devsign * eq71_e1837_d_n0);
        let eq71_e1839_d_n2: f64 = (locals.var_devsign * eq71_e1837_d_n2);
        let eq71_e1839_d_n3: f64 = (locals.var_devsign * eq71_e1837_d_n3);
        let eq71_e1839_d_n4: f64 = (locals.var_devsign * eq71_e1837_d_n4);
        let eq71_e1839_d_n5: f64 = (locals.var_devsign * eq71_e1837_d_n5);
        let eq71_e1839_d_n6: f64 = (locals.var_devsign * eq71_e1837_d_n6);
        let eq71_e1839_d_n7: f64 = (locals.var_devsign * eq71_e1837_d_n7);
        let eq71_e1839_d_n8: f64 = (locals.var_devsign * eq71_e1837_d_n8);
        let eq71_e1839_d_n9: f64 = (locals.var_devsign * eq71_e1837_d_n9);
        let eq71_e1839_d_n10: f64 = (locals.var_devsign * eq71_e1837_d_n10);
        let eq71_e1839_d_n11: f64 = (locals.var_devsign * eq71_e1837_d_n11);
        let eq71_e1839_d_n12: f64 = (locals.var_devsign * eq71_e1837_d_n12);
        let eq71_e1839_d_n13: f64 = (locals.var_devsign * eq71_e1837_d_n13);
        let eq71_e1839_d_n14: f64 = (locals.var_devsign * eq71_e1837_d_n14);
        let eq71_e1839_q: f64 = (locals.var_devsign * eq71_e1838_q);
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_d_n0, 0.0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, 0.0, 0.0];
        let eq71_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_q,) = {
    if ((locals.var_guard769 != 0.0) && (locals.var_guard770 != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * locals.var_qbdj);
        let eq73_e1864_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq73_e1864_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq73_e1864_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq73_e1864_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq73_e1864_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq73_e1864_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq73_e1864_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq73_e1864_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq73_e1864_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq73_e1864_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq73_e1864_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq73_e1864_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq73_e1864_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq73_e1864_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (locals.var_devsign * eq73_e1864);
        let eq73_e1866_d_n0: f64 = (locals.var_devsign * eq73_e1864_d_n0);
        let eq73_e1866_d_n2: f64 = (locals.var_devsign * eq73_e1864_d_n2);
        let eq73_e1866_d_n3: f64 = (locals.var_devsign * eq73_e1864_d_n3);
        let eq73_e1866_d_n4: f64 = (locals.var_devsign * eq73_e1864_d_n4);
        let eq73_e1866_d_n5: f64 = (locals.var_devsign * eq73_e1864_d_n5);
        let eq73_e1866_d_n6: f64 = (locals.var_devsign * eq73_e1864_d_n6);
        let eq73_e1866_d_n7: f64 = (locals.var_devsign * eq73_e1864_d_n7);
        let eq73_e1866_d_n8: f64 = (locals.var_devsign * eq73_e1864_d_n8);
        let eq73_e1866_d_n9: f64 = (locals.var_devsign * eq73_e1864_d_n9);
        let eq73_e1866_d_n10: f64 = (locals.var_devsign * eq73_e1864_d_n10);
        let eq73_e1866_d_n11: f64 = (locals.var_devsign * eq73_e1864_d_n11);
        let eq73_e1866_d_n12: f64 = (locals.var_devsign * eq73_e1864_d_n12);
        let eq73_e1866_d_n13: f64 = (locals.var_devsign * eq73_e1864_d_n13);
        let eq73_e1866_d_n14: f64 = (locals.var_devsign * eq73_e1864_d_n14);
        let eq73_e1866_q: f64 = (locals.var_devsign * eq73_e1865_q);
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_d_n0, 0.0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, 0.0, 0.0];
        let eq73_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_q,) = {
    if (locals.var_guard769 == 0.0) {
        let eq76_e1904: f64 = (p.p29 * locals.var_qbsj);
        let eq76_e1904_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq76_e1904_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq76_e1904_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq76_e1904_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq76_e1904_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq76_e1904_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq76_e1904_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq76_e1904_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq76_e1904_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq76_e1904_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq76_e1904_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq76_e1904_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq76_e1904_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq76_e1904_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (locals.var_devsign * eq76_e1904);
        let eq76_e1906_d_n0: f64 = (locals.var_devsign * eq76_e1904_d_n0);
        let eq76_e1906_d_n2: f64 = (locals.var_devsign * eq76_e1904_d_n2);
        let eq76_e1906_d_n3: f64 = (locals.var_devsign * eq76_e1904_d_n3);
        let eq76_e1906_d_n4: f64 = (locals.var_devsign * eq76_e1904_d_n4);
        let eq76_e1906_d_n5: f64 = (locals.var_devsign * eq76_e1904_d_n5);
        let eq76_e1906_d_n6: f64 = (locals.var_devsign * eq76_e1904_d_n6);
        let eq76_e1906_d_n7: f64 = (locals.var_devsign * eq76_e1904_d_n7);
        let eq76_e1906_d_n8: f64 = (locals.var_devsign * eq76_e1904_d_n8);
        let eq76_e1906_d_n9: f64 = (locals.var_devsign * eq76_e1904_d_n9);
        let eq76_e1906_d_n10: f64 = (locals.var_devsign * eq76_e1904_d_n10);
        let eq76_e1906_d_n11: f64 = (locals.var_devsign * eq76_e1904_d_n11);
        let eq76_e1906_d_n12: f64 = (locals.var_devsign * eq76_e1904_d_n12);
        let eq76_e1906_d_n13: f64 = (locals.var_devsign * eq76_e1904_d_n13);
        let eq76_e1906_d_n14: f64 = (locals.var_devsign * eq76_e1904_d_n14);
        let eq76_e1906_q: f64 = (locals.var_devsign * eq76_e1905_q);
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_d_n0, 0.0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, 0.0, 0.0];
        let eq76_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_q,) = {
    if (locals.var_guard769 == 0.0) {
        let eq77_e1914: f64 = (p.p29 * locals.var_qbdj);
        let eq77_e1914_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq77_e1914_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq77_e1914_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq77_e1914_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq77_e1914_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq77_e1914_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq77_e1914_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq77_e1914_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq77_e1914_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq77_e1914_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq77_e1914_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq77_e1914_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq77_e1914_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq77_e1914_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (locals.var_devsign * eq77_e1914);
        let eq77_e1916_d_n0: f64 = (locals.var_devsign * eq77_e1914_d_n0);
        let eq77_e1916_d_n2: f64 = (locals.var_devsign * eq77_e1914_d_n2);
        let eq77_e1916_d_n3: f64 = (locals.var_devsign * eq77_e1914_d_n3);
        let eq77_e1916_d_n4: f64 = (locals.var_devsign * eq77_e1914_d_n4);
        let eq77_e1916_d_n5: f64 = (locals.var_devsign * eq77_e1914_d_n5);
        let eq77_e1916_d_n6: f64 = (locals.var_devsign * eq77_e1914_d_n6);
        let eq77_e1916_d_n7: f64 = (locals.var_devsign * eq77_e1914_d_n7);
        let eq77_e1916_d_n8: f64 = (locals.var_devsign * eq77_e1914_d_n8);
        let eq77_e1916_d_n9: f64 = (locals.var_devsign * eq77_e1914_d_n9);
        let eq77_e1916_d_n10: f64 = (locals.var_devsign * eq77_e1914_d_n10);
        let eq77_e1916_d_n11: f64 = (locals.var_devsign * eq77_e1914_d_n11);
        let eq77_e1916_d_n12: f64 = (locals.var_devsign * eq77_e1914_d_n12);
        let eq77_e1916_d_n13: f64 = (locals.var_devsign * eq77_e1914_d_n13);
        let eq77_e1916_d_n14: f64 = (locals.var_devsign * eq77_e1914_d_n14);
        let eq77_e1916_q: f64 = (locals.var_devsign * eq77_e1915_q);
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_d_n0, 0.0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, 0.0, 0.0];
        let eq77_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_q,) = {
    if (locals.var_guard772 != 0.0) {
        let eq83_e1980: f64 = (p.p29 * locals.var_qbdj);
        let eq83_e1980_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq83_e1980_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq83_e1980_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq83_e1980_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq83_e1980_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq83_e1980_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq83_e1980_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq83_e1980_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq83_e1980_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq83_e1980_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq83_e1980_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq83_e1980_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq83_e1980_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq83_e1980_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (locals.var_devsign * eq83_e1980);
        let eq83_e1982_d_n0: f64 = (locals.var_devsign * eq83_e1980_d_n0);
        let eq83_e1982_d_n2: f64 = (locals.var_devsign * eq83_e1980_d_n2);
        let eq83_e1982_d_n3: f64 = (locals.var_devsign * eq83_e1980_d_n3);
        let eq83_e1982_d_n4: f64 = (locals.var_devsign * eq83_e1980_d_n4);
        let eq83_e1982_d_n5: f64 = (locals.var_devsign * eq83_e1980_d_n5);
        let eq83_e1982_d_n6: f64 = (locals.var_devsign * eq83_e1980_d_n6);
        let eq83_e1982_d_n7: f64 = (locals.var_devsign * eq83_e1980_d_n7);
        let eq83_e1982_d_n8: f64 = (locals.var_devsign * eq83_e1980_d_n8);
        let eq83_e1982_d_n9: f64 = (locals.var_devsign * eq83_e1980_d_n9);
        let eq83_e1982_d_n10: f64 = (locals.var_devsign * eq83_e1980_d_n10);
        let eq83_e1982_d_n11: f64 = (locals.var_devsign * eq83_e1980_d_n11);
        let eq83_e1982_d_n12: f64 = (locals.var_devsign * eq83_e1980_d_n12);
        let eq83_e1982_d_n13: f64 = (locals.var_devsign * eq83_e1980_d_n13);
        let eq83_e1982_d_n14: f64 = (locals.var_devsign * eq83_e1980_d_n14);
        let eq83_e1982_q: f64 = (locals.var_devsign * eq83_e1981_q);
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_d_n0, 0.0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, 0.0, 0.0];
        let eq83_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq83_reactive_node_derivatives,
            branches,
            &eq83_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_q,) = {
    if (locals.var_guard772 != 0.0) {
        let eq84_e1989: f64 = (p.p29 * locals.var_qbdj_ext);
        let eq84_e1989_d_n0: f64 = (p.p29 * locals.var_qbdj_ext_dn0);
        let eq84_e1989_d_n2: f64 = (p.p29 * locals.var_qbdj_ext_dn2);
        let eq84_e1989_d_n3: f64 = (p.p29 * locals.var_qbdj_ext_dn3);
        let eq84_e1989_d_n4: f64 = (p.p29 * locals.var_qbdj_ext_dn4);
        let eq84_e1989_d_n5: f64 = (p.p29 * locals.var_qbdj_ext_dn5);
        let eq84_e1989_d_n6: f64 = (p.p29 * locals.var_qbdj_ext_dn6);
        let eq84_e1989_d_n7: f64 = (p.p29 * locals.var_qbdj_ext_dn7);
        let eq84_e1989_d_n8: f64 = (p.p29 * locals.var_qbdj_ext_dn8);
        let eq84_e1989_d_n9: f64 = (p.p29 * locals.var_qbdj_ext_dn9);
        let eq84_e1989_d_n10: f64 = (p.p29 * locals.var_qbdj_ext_dn10);
        let eq84_e1989_d_n11: f64 = (p.p29 * locals.var_qbdj_ext_dn11);
        let eq84_e1989_d_n12: f64 = (p.p29 * locals.var_qbdj_ext_dn12);
        let eq84_e1989_d_n13: f64 = (p.p29 * locals.var_qbdj_ext_dn13);
        let eq84_e1989_d_n14: f64 = (p.p29 * locals.var_qbdj_ext_dn14);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (locals.var_devsign * eq84_e1989);
        let eq84_e1991_d_n0: f64 = (locals.var_devsign * eq84_e1989_d_n0);
        let eq84_e1991_d_n2: f64 = (locals.var_devsign * eq84_e1989_d_n2);
        let eq84_e1991_d_n3: f64 = (locals.var_devsign * eq84_e1989_d_n3);
        let eq84_e1991_d_n4: f64 = (locals.var_devsign * eq84_e1989_d_n4);
        let eq84_e1991_d_n5: f64 = (locals.var_devsign * eq84_e1989_d_n5);
        let eq84_e1991_d_n6: f64 = (locals.var_devsign * eq84_e1989_d_n6);
        let eq84_e1991_d_n7: f64 = (locals.var_devsign * eq84_e1989_d_n7);
        let eq84_e1991_d_n8: f64 = (locals.var_devsign * eq84_e1989_d_n8);
        let eq84_e1991_d_n9: f64 = (locals.var_devsign * eq84_e1989_d_n9);
        let eq84_e1991_d_n10: f64 = (locals.var_devsign * eq84_e1989_d_n10);
        let eq84_e1991_d_n11: f64 = (locals.var_devsign * eq84_e1989_d_n11);
        let eq84_e1991_d_n12: f64 = (locals.var_devsign * eq84_e1989_d_n12);
        let eq84_e1991_d_n13: f64 = (locals.var_devsign * eq84_e1989_d_n13);
        let eq84_e1991_d_n14: f64 = (locals.var_devsign * eq84_e1989_d_n14);
        let eq84_e1991_q: f64 = (locals.var_devsign * eq84_e1990_q);
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_d_n0, 0.0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, 0.0, 0.0];
        let eq84_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            nodes,
            &eq84_reactive_node_derivatives,
            branches,
            &eq84_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
