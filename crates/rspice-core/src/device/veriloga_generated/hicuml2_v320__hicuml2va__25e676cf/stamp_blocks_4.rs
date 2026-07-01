#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq1_e170: f64 = (locals.var_qdeix + locals.var_qjei);
        let eq1_e170_d_n0: f64 = (locals.var_qdeix_dn0 + locals.var_qjei_dn0);
        let eq1_e170_d_n1: f64 = (locals.var_qdeix_dn1 + locals.var_qjei_dn1);
        let eq1_e170_d_n3: f64 = (locals.var_qdeix_dn3 + locals.var_qjei_dn3);
        let eq1_e170_d_n4: f64 = (locals.var_qdeix_dn4 + locals.var_qjei_dn4);
        let eq1_e170_d_n5: f64 = (locals.var_qdeix_dn5 + locals.var_qjei_dn5);
        let eq1_e170_d_n6: f64 = (locals.var_qdeix_dn6 + locals.var_qjei_dn6);
        let eq1_e170_d_n7: f64 = (locals.var_qdeix_dn7 + locals.var_qjei_dn7);
        let eq1_e170_d_n8: f64 = (locals.var_qdeix_dn8 + locals.var_qjei_dn8);
        let eq1_e170_d_n9: f64 = (locals.var_qdeix_dn9 + locals.var_qjei_dn9);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n12: f64 = (p.p148 * locals.var_qdeix_dn12);
        let eq1_e172_q: f64 = eq1_e171;
        let eq1_reactive_node_derivatives: [f64; 15] = [eq1_e171_d_n0, eq1_e171_d_n1, 0.0, eq1_e171_d_n3, eq1_e171_d_n4, eq1_e171_d_n5, eq1_e171_d_n6, eq1_e171_d_n7, eq1_e171_d_n8, eq1_e171_d_n9, 0.0, 0.0, eq1_e171_d_n12, 0.0, 0.0];
        let eq1_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let eq3_e185: f64 = (locals.var_qdci + locals.var_qjci);
        let eq3_e185_d_n0: f64 = (locals.var_qdci_dn0 + locals.var_qjci_dn0);
        let eq3_e185_d_n1: f64 = (locals.var_qdci_dn1 + locals.var_qjci_dn1);
        let eq3_e185_d_n3: f64 = (locals.var_qdci_dn3 + locals.var_qjci_dn3);
        let eq3_e185_d_n4: f64 = (locals.var_qdci_dn4 + locals.var_qjci_dn4);
        let eq3_e185_d_n5: f64 = (locals.var_qdci_dn5 + locals.var_qjci_dn5);
        let eq3_e185_d_n6: f64 = (locals.var_qdci_dn6 + locals.var_qjci_dn6);
        let eq3_e185_d_n7: f64 = (locals.var_qdci_dn7 + locals.var_qjci_dn7);
        let eq3_e185_d_n8: f64 = (locals.var_qdci_dn8 + locals.var_qjci_dn8);
        let eq3_e185_d_n9: f64 = (locals.var_qdci_dn9 + locals.var_qjci_dn9);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e187_q: f64 = eq3_e186;
        let eq3_reactive_node_derivatives: [f64; 15] = [eq3_e186_d_n0, eq3_e186_d_n1, 0.0, eq3_e186_d_n3, eq3_e186_d_n4, eq3_e186_d_n5, eq3_e186_d_n6, eq3_e186_d_n7, eq3_e186_d_n8, eq3_e186_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq3_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq3_reactive_node_derivatives,
            branches,
            &eq3_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_q,) = {
    if ((locals.var_guard233 != 0.0) && (locals.var_guard234 != 0.0)) {
        let eq7_e204_q: f64 = locals.var_qrbi;
        (locals.var_qrbi, locals.var_qrbi_dn0, locals.var_qrbi_dn1, locals.var_qrbi_dn3, locals.var_qrbi_dn4, locals.var_qrbi_dn5, locals.var_qrbi_dn6, locals.var_qrbi_dn7, locals.var_qrbi_dn8, locals.var_qrbi_dn9, eq7_e204_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, 0.0, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq7_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e238: f64 = (p.p148 * locals.var_qjep);
        let eq13_e238_d_n0: f64 = (p.p148 * locals.var_qjep_dn0);
        let eq13_e238_d_n1: f64 = (p.p148 * locals.var_qjep_dn1);
        let eq13_e238_d_n3: f64 = (p.p148 * locals.var_qjep_dn3);
        let eq13_e238_d_n4: f64 = (p.p148 * locals.var_qjep_dn4);
        let eq13_e238_d_n5: f64 = (p.p148 * locals.var_qjep_dn5);
        let eq13_e238_d_n6: f64 = (p.p148 * locals.var_qjep_dn6);
        let eq13_e238_d_n7: f64 = (p.p148 * locals.var_qjep_dn7);
        let eq13_e238_d_n8: f64 = (p.p148 * locals.var_qjep_dn8);
        let eq13_e238_d_n9: f64 = (p.p148 * locals.var_qjep_dn9);
        let eq13_e239_q: f64 = eq13_e238;
        let eq13_reactive_node_derivatives: [f64; 15] = [eq13_e238_d_n0, eq13_e238_d_n1, 0.0, eq13_e238_d_n3, eq13_e238_d_n4, eq13_e238_d_n5, eq13_e238_d_n6, eq13_e238_d_n7, eq13_e238_d_n8, eq13_e238_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq13_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e246: f64 = (locals.var_qjcx0_t_p + locals.var_qdsu);
        let eq15_e246_d_n4: f64 = (locals.var_qjcx0_t_p_dn4 + locals.var_qdsu_dn4);
        let eq15_e246_d_n5: f64 = (locals.var_qjcx0_t_p_dn5 + locals.var_qdsu_dn5);
        let eq15_e246_d_n7: f64 = (locals.var_qjcx0_t_p_dn7 + locals.var_qdsu_dn7);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * locals.var_qjcx0_t_p_dn0);
        let eq15_e247_d_n1: f64 = (p.p148 * locals.var_qjcx0_t_p_dn1);
        let eq15_e247_d_n3: f64 = (p.p148 * locals.var_qjcx0_t_p_dn3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * locals.var_qjcx0_t_p_dn6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * locals.var_qjcx0_t_p_dn8);
        let eq15_e247_d_n9: f64 = (p.p148 * locals.var_qjcx0_t_p_dn9);
        let eq15_e248_q: f64 = eq15_e247;
        let eq15_reactive_node_derivatives: [f64; 15] = [eq15_e247_d_n0, eq15_e247_d_n1, 0.0, eq15_e247_d_n3, eq15_e247_d_n4, eq15_e247_d_n5, eq15_e247_d_n6, eq15_e247_d_n7, eq15_e247_d_n8, eq15_e247_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e255: f64 = (p.p148 * locals.var_qjcx0_t_x);
        let eq17_e255_d_n0: f64 = (p.p148 * locals.var_qjcx0_t_x_dn0);
        let eq17_e255_d_n1: f64 = (p.p148 * locals.var_qjcx0_t_x_dn1);
        let eq17_e255_d_n3: f64 = (p.p148 * locals.var_qjcx0_t_x_dn3);
        let eq17_e255_d_n4: f64 = (p.p148 * locals.var_qjcx0_t_x_dn4);
        let eq17_e255_d_n5: f64 = (p.p148 * locals.var_qjcx0_t_x_dn5);
        let eq17_e255_d_n6: f64 = (p.p148 * locals.var_qjcx0_t_x_dn6);
        let eq17_e255_d_n7: f64 = (p.p148 * locals.var_qjcx0_t_x_dn7);
        let eq17_e255_d_n8: f64 = (p.p148 * locals.var_qjcx0_t_x_dn8);
        let eq17_e255_d_n9: f64 = (p.p148 * locals.var_qjcx0_t_x_dn9);
        let eq17_e256_q: f64 = eq17_e255;
        let eq17_reactive_node_derivatives: [f64; 15] = [eq17_e255_d_n0, eq17_e255_d_n1, 0.0, eq17_e255_d_n3, eq17_e255_d_n4, eq17_e255_d_n5, eq17_e255_d_n6, eq17_e255_d_n7, eq17_e255_d_n8, eq17_e255_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq17_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e343: f64 = (p.p148 * locals.var_qjs);
        let eq33_e343_d_n0: f64 = (p.p148 * locals.var_qjs_dn0);
        let eq33_e343_d_n1: f64 = (p.p148 * locals.var_qjs_dn1);
        let eq33_e343_d_n3: f64 = (p.p148 * locals.var_qjs_dn3);
        let eq33_e343_d_n4: f64 = (p.p148 * locals.var_qjs_dn4);
        let eq33_e343_d_n5: f64 = (p.p148 * locals.var_qjs_dn5);
        let eq33_e343_d_n6: f64 = (p.p148 * locals.var_qjs_dn6);
        let eq33_e343_d_n7: f64 = (p.p148 * locals.var_qjs_dn7);
        let eq33_e343_d_n8: f64 = (p.p148 * locals.var_qjs_dn8);
        let eq33_e343_d_n9: f64 = (p.p148 * locals.var_qjs_dn9);
        let eq33_e344_q: f64 = eq33_e343;
        let eq33_reactive_node_derivatives: [f64; 15] = [eq33_e343_d_n0, eq33_e343_d_n1, 0.0, eq33_e343_d_n3, eq33_e343_d_n4, eq33_e343_d_n5, eq33_e343_d_n6, eq33_e343_d_n7, eq33_e343_d_n8, eq33_e343_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e347: f64 = (p.p148 * locals.var_qscp);
        let eq34_e347_d_n0: f64 = (p.p148 * locals.var_qscp_dn0);
        let eq34_e347_d_n1: f64 = (p.p148 * locals.var_qscp_dn1);
        let eq34_e347_d_n3: f64 = (p.p148 * locals.var_qscp_dn3);
        let eq34_e347_d_n4: f64 = (p.p148 * locals.var_qscp_dn4);
        let eq34_e347_d_n5: f64 = (p.p148 * locals.var_qscp_dn5);
        let eq34_e347_d_n6: f64 = (p.p148 * locals.var_qscp_dn6);
        let eq34_e347_d_n7: f64 = (p.p148 * locals.var_qscp_dn7);
        let eq34_e347_d_n8: f64 = (p.p148 * locals.var_qscp_dn8);
        let eq34_e347_d_n9: f64 = (p.p148 * locals.var_qscp_dn9);
        let eq34_e348_q: f64 = eq34_e347;
        let eq34_reactive_node_derivatives: [f64; 15] = [eq34_e347_d_n0, eq34_e347_d_n1, 0.0, eq34_e347_d_n3, eq34_e347_d_n4, eq34_e347_d_n5, eq34_e347_d_n6, eq34_e347_d_n7, eq34_e347_d_n8, eq34_e347_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9, eq36_e363_q,) = {
    if ((locals.var_guard242 != 0.0) && (locals.var_guard243 != 0.0)) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e361_q: f64 = eq36_e360;
        (eq36_e360, (-p.p103), p.p103, eq36_e361_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * (eq36_e363_d_n3),
            nodes[9],
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq39_e385, eq39_e385_d_n4, eq39_e385_q,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e383_q: f64 = eq39_e382;
        (eq39_e382, p.p145, eq39_e383_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq39_e385_d_n4),
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n13, eq65_e534_q, eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9,) = {
    if (locals.var_guard258 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_n_w;
        let eq65_e527: f64 = (locals.var_n_2 * __rspice_inv_cse_0);
        let eq65_e527_d_n0: f64 = (locals.var_n_2_dn0 * __rspice_inv_cse_0);
        let eq65_e527_d_n1: f64 = (locals.var_n_2_dn1 * __rspice_inv_cse_0);
        let eq65_e527_d_n3: f64 = (locals.var_n_2_dn3 * __rspice_inv_cse_0);
        let eq65_e527_d_n4: f64 = (locals.var_n_2_dn4 * __rspice_inv_cse_0);
        let eq65_e527_d_n5: f64 = (locals.var_n_2_dn5 * __rspice_inv_cse_0);
        let eq65_e527_d_n6: f64 = (locals.var_n_2_dn6 * __rspice_inv_cse_0);
        let eq65_e527_d_n7: f64 = (locals.var_n_2_dn7 * __rspice_inv_cse_0);
        let eq65_e527_d_n8: f64 = (locals.var_n_2_dn8 * __rspice_inv_cse_0);
        let eq65_e527_d_n9: f64 = (locals.var_n_2_dn9 * __rspice_inv_cse_0);
        let eq65_e530: f64 = (locals.var_n_w * (nv13 - 0.0));
        let eq65_e531_q: f64 = eq65_e530;
        let eq65_e532: f64 = (eq65_e527 * eq65_e530);
        let eq65_e532_d_n0: f64 = (eq65_e527_d_n0 * eq65_e530);
        let eq65_e532_d_n1: f64 = (eq65_e527_d_n1 * eq65_e530);
        let eq65_e532_d_n3: f64 = (eq65_e527_d_n3 * eq65_e530);
        let eq65_e532_d_n4: f64 = (eq65_e527_d_n4 * eq65_e530);
        let eq65_e532_d_n5: f64 = (eq65_e527_d_n5 * eq65_e530);
        let eq65_e532_d_n6: f64 = (eq65_e527_d_n6 * eq65_e530);
        let eq65_e532_d_n7: f64 = (eq65_e527_d_n7 * eq65_e530);
        let eq65_e532_d_n8: f64 = (eq65_e527_d_n8 * eq65_e530);
        let eq65_e532_d_n9: f64 = (eq65_e527_d_n9 * eq65_e530);
        let eq65_e532_d_n13: f64 = (eq65_e527 * locals.var_n_w);
        let eq65_e532_q: f64 = (eq65_e527 * eq65_e531_q);
        let eq65_e532_q_d_n0: f64 = (eq65_e527_d_n0 * eq65_e531_q);
        let eq65_e532_q_d_n1: f64 = (eq65_e527_d_n1 * eq65_e531_q);
        let eq65_e532_q_d_n3: f64 = (eq65_e527_d_n3 * eq65_e531_q);
        let eq65_e532_q_d_n4: f64 = (eq65_e527_d_n4 * eq65_e531_q);
        let eq65_e532_q_d_n5: f64 = (eq65_e527_d_n5 * eq65_e531_q);
        let eq65_e532_q_d_n6: f64 = (eq65_e527_d_n6 * eq65_e531_q);
        let eq65_e532_q_d_n7: f64 = (eq65_e527_d_n7 * eq65_e531_q);
        let eq65_e532_q_d_n8: f64 = (eq65_e527_d_n8 * eq65_e531_q);
        let eq65_e532_q_d_n9: f64 = (eq65_e527_d_n9 * eq65_e531_q);
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n13, eq65_e532_q, eq65_e532_q_d_n0, eq65_e532_q_d_n1, eq65_e532_q_d_n3, eq65_e532_q_d_n4, eq65_e532_q_d_n5, eq65_e532_q_d_n6, eq65_e532_q_d_n7, eq65_e532_q_d_n8, eq65_e532_q_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 15] = [eq65_e534_q_d_n0, eq65_e534_q_d_n1, 0.0, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, 0.0, 0.0, 0.0, eq65_e534_d_n13, 0.0];
        let eq65_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq65_reactive_node_derivatives,
            branches,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n14, eq66_e545_q, eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9,) = {
    if (locals.var_guard258 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_n_w;
        let eq66_e538: f64 = (locals.var_n_1 * __rspice_inv_cse_1);
        let eq66_e538_d_n0: f64 = (locals.var_n_1_dn0 * __rspice_inv_cse_1);
        let eq66_e538_d_n1: f64 = (locals.var_n_1_dn1 * __rspice_inv_cse_1);
        let eq66_e538_d_n3: f64 = (locals.var_n_1_dn3 * __rspice_inv_cse_1);
        let eq66_e538_d_n4: f64 = (locals.var_n_1_dn4 * __rspice_inv_cse_1);
        let eq66_e538_d_n5: f64 = (locals.var_n_1_dn5 * __rspice_inv_cse_1);
        let eq66_e538_d_n6: f64 = (locals.var_n_1_dn6 * __rspice_inv_cse_1);
        let eq66_e538_d_n7: f64 = (locals.var_n_1_dn7 * __rspice_inv_cse_1);
        let eq66_e538_d_n8: f64 = (locals.var_n_1_dn8 * __rspice_inv_cse_1);
        let eq66_e538_d_n9: f64 = (locals.var_n_1_dn9 * __rspice_inv_cse_1);
        let eq66_e541: f64 = (locals.var_n_w * (nv14 - 0.0));
        let eq66_e542_q: f64 = eq66_e541;
        let eq66_e543: f64 = (eq66_e538 * eq66_e541);
        let eq66_e543_d_n0: f64 = (eq66_e538_d_n0 * eq66_e541);
        let eq66_e543_d_n1: f64 = (eq66_e538_d_n1 * eq66_e541);
        let eq66_e543_d_n3: f64 = (eq66_e538_d_n3 * eq66_e541);
        let eq66_e543_d_n4: f64 = (eq66_e538_d_n4 * eq66_e541);
        let eq66_e543_d_n5: f64 = (eq66_e538_d_n5 * eq66_e541);
        let eq66_e543_d_n6: f64 = (eq66_e538_d_n6 * eq66_e541);
        let eq66_e543_d_n7: f64 = (eq66_e538_d_n7 * eq66_e541);
        let eq66_e543_d_n8: f64 = (eq66_e538_d_n8 * eq66_e541);
        let eq66_e543_d_n9: f64 = (eq66_e538_d_n9 * eq66_e541);
        let eq66_e543_d_n14: f64 = (eq66_e538 * locals.var_n_w);
        let eq66_e543_q: f64 = (eq66_e538 * eq66_e542_q);
        let eq66_e543_q_d_n0: f64 = (eq66_e538_d_n0 * eq66_e542_q);
        let eq66_e543_q_d_n1: f64 = (eq66_e538_d_n1 * eq66_e542_q);
        let eq66_e543_q_d_n3: f64 = (eq66_e538_d_n3 * eq66_e542_q);
        let eq66_e543_q_d_n4: f64 = (eq66_e538_d_n4 * eq66_e542_q);
        let eq66_e543_q_d_n5: f64 = (eq66_e538_d_n5 * eq66_e542_q);
        let eq66_e543_q_d_n6: f64 = (eq66_e538_d_n6 * eq66_e542_q);
        let eq66_e543_q_d_n7: f64 = (eq66_e538_d_n7 * eq66_e542_q);
        let eq66_e543_q_d_n8: f64 = (eq66_e538_d_n8 * eq66_e542_q);
        let eq66_e543_q_d_n9: f64 = (eq66_e538_d_n9 * eq66_e542_q);
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n14, eq66_e543_q, eq66_e543_q_d_n0, eq66_e543_q_d_n1, eq66_e543_q_d_n3, eq66_e543_q_d_n4, eq66_e543_q_d_n5, eq66_e543_q_d_n6, eq66_e543_q_d_n7, eq66_e543_q_d_n8, eq66_e543_q_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 15] = [eq66_e545_q_d_n0, eq66_e545_q_d_n1, 0.0, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, 0.0, 0.0, 0.0, 0.0, eq66_e545_d_n14];
        let eq66_reactive_branch_derivatives: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
