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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e233: f64 = (locals.var_qte + locals.var_qbe);
        let eq10_e233_d_n0: f64 = (locals.var_qte_dn0 + locals.var_qbe_dn0);
        let eq10_e233_d_n1: f64 = (locals.var_qte_dn1 + locals.var_qbe_dn1);
        let eq10_e233_d_n3: f64 = (locals.var_qte_dn3 + locals.var_qbe_dn3);
        let eq10_e233_d_n4: f64 = (locals.var_qte_dn4 + locals.var_qbe_dn4);
        let eq10_e233_d_n5: f64 = (locals.var_qte_dn5 + locals.var_qbe_dn5);
        let eq10_e233_d_n6: f64 = (locals.var_qte_dn6 + locals.var_qbe_dn6);
        let eq10_e233_d_n7: f64 = (locals.var_qte_dn7 + locals.var_qbe_dn7);
        let eq10_e233_d_n8: f64 = (locals.var_qte_dn8 + locals.var_qbe_dn8);
        let eq10_e233_d_n9: f64 = (locals.var_qte_dn9 + locals.var_qbe_dn9);
        let eq10_e235: f64 = (eq10_e233 + locals.var_qe);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + locals.var_qe_dn0);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + locals.var_qe_dn1);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + locals.var_qe_dn3);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + locals.var_qe_dn4);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + locals.var_qe_dn5);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + locals.var_qe_dn6);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + locals.var_qe_dn7);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + locals.var_qe_dn8);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + locals.var_qe_dn9);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e237_q: f64 = eq10_e236;
        let eq10_e239: f64 = (eq10_e236 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_q: f64 = (eq10_e237_q * p.p1);
        let eq10_reactive_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, 0.0, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, 0.0];
        let eq10_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * locals.var_qte_s);
        let eq11_e242_d_n0: f64 = (p.p3 * locals.var_qte_s_dn0);
        let eq11_e242_d_n1: f64 = (p.p3 * locals.var_qte_s_dn1);
        let eq11_e242_d_n3: f64 = (p.p3 * locals.var_qte_s_dn3);
        let eq11_e242_d_n4: f64 = (p.p3 * locals.var_qte_s_dn4);
        let eq11_e242_d_n5: f64 = (p.p3 * locals.var_qte_s_dn5);
        let eq11_e242_d_n6: f64 = (p.p3 * locals.var_qte_s_dn6);
        let eq11_e242_d_n7: f64 = (p.p3 * locals.var_qte_s_dn7);
        let eq11_e242_d_n8: f64 = (p.p3 * locals.var_qte_s_dn8);
        let eq11_e242_d_n9: f64 = (p.p3 * locals.var_qte_s_dn9);
        let eq11_e243_q: f64 = eq11_e242;
        let eq11_e245: f64 = (eq11_e242 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_q: f64 = (eq11_e243_q * p.p1);
        let eq11_reactive_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, 0.0, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, 0.0];
        let eq11_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (locals.var_qtc + locals.var_qbc);
        let eq12_e249_d_n0: f64 = (locals.var_qtc_dn0 + locals.var_qbc_dn0);
        let eq12_e249_d_n1: f64 = (locals.var_qtc_dn1 + locals.var_qbc_dn1);
        let eq12_e249_d_n3: f64 = (locals.var_qtc_dn3 + locals.var_qbc_dn3);
        let eq12_e249_d_n4: f64 = (locals.var_qtc_dn4 + locals.var_qbc_dn4);
        let eq12_e249_d_n5: f64 = (locals.var_qtc_dn5 + locals.var_qbc_dn5);
        let eq12_e249_d_n6: f64 = (locals.var_qtc_dn6 + locals.var_qbc_dn6);
        let eq12_e249_d_n7: f64 = (locals.var_qtc_dn7 + locals.var_qbc_dn7);
        let eq12_e249_d_n8: f64 = (locals.var_qtc_dn8 + locals.var_qbc_dn8);
        let eq12_e249_d_n9: f64 = (locals.var_qtc_dn9 + locals.var_qbc_dn9);
        let eq12_e251: f64 = (eq12_e249 + locals.var_qepi);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + locals.var_qepi_dn0);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + locals.var_qepi_dn1);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + locals.var_qepi_dn3);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + locals.var_qepi_dn4);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + locals.var_qepi_dn5);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + locals.var_qepi_dn6);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + locals.var_qepi_dn7);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + locals.var_qepi_dn8);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + locals.var_qepi_dn9);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e253_q: f64 = eq12_e252;
        let eq12_e255: f64 = (eq12_e252 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_q: f64 = (eq12_e253_q * p.p1);
        let eq12_reactive_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, 0.0, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, 0.0];
        let eq12_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * locals.var_qb1b2);
        let eq13_e258_d_n0: f64 = (p.p3 * locals.var_qb1b2_dn0);
        let eq13_e258_d_n1: f64 = (p.p3 * locals.var_qb1b2_dn1);
        let eq13_e258_d_n3: f64 = (p.p3 * locals.var_qb1b2_dn3);
        let eq13_e258_d_n4: f64 = (p.p3 * locals.var_qb1b2_dn4);
        let eq13_e258_d_n5: f64 = (p.p3 * locals.var_qb1b2_dn5);
        let eq13_e258_d_n6: f64 = (p.p3 * locals.var_qb1b2_dn6);
        let eq13_e258_d_n7: f64 = (p.p3 * locals.var_qb1b2_dn7);
        let eq13_e258_d_n8: f64 = (p.p3 * locals.var_qb1b2_dn8);
        let eq13_e258_d_n9: f64 = (p.p3 * locals.var_qb1b2_dn9);
        let eq13_e259_q: f64 = eq13_e258;
        let eq13_e261: f64 = (eq13_e258 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_q: f64 = (eq13_e259_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, 0.0, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, 0.0];
        let eq13_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * locals.var_vbe);
        let eq14_e266_d_n1: f64 = (eq14_e264 * locals.var_vbe_dn1);
        let eq14_e266_d_n2: f64 = (eq14_e264 * locals.var_vbe_dn2);
        let eq14_e267_q: f64 = eq14_e266;
        let eq14_e269: f64 = (eq14_e266 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_q: f64 = (eq14_e267_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq14_e269_d_n1),
            nodes[2],
            multiplicity * (eq14_e269_d_n2),
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * locals.var_vbc);
        let eq15_e274_d_n0: f64 = (eq15_e272 * locals.var_vbc_dn0);
        let eq15_e274_d_n1: f64 = (eq15_e272 * locals.var_vbc_dn1);
        let eq15_e275_q: f64 = eq15_e274;
        let eq15_e277: f64 = (eq15_e274 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_q: f64 = (eq15_e275_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq15_e277_d_n0),
            nodes[1],
            multiplicity * (eq15_e277_d_n1),
        );
        let eq18_e293: f64 = (locals.var_xqtex + locals.var_xqex);
        let eq18_e293_d_n0: f64 = (locals.var_xqtex_dn0 + locals.var_xqex_dn0);
        let eq18_e293_d_n1: f64 = (locals.var_xqtex_dn1 + locals.var_xqex_dn1);
        let eq18_e293_d_n3: f64 = (locals.var_xqtex_dn3 + locals.var_xqex_dn3);
        let eq18_e293_d_n4: f64 = (locals.var_xqtex_dn4 + locals.var_xqex_dn4);
        let eq18_e293_d_n5: f64 = (locals.var_xqtex_dn5 + locals.var_xqex_dn5);
        let eq18_e293_d_n6: f64 = (locals.var_xqtex_dn6 + locals.var_xqex_dn6);
        let eq18_e293_d_n7: f64 = (locals.var_xqtex_dn7 + locals.var_xqex_dn7);
        let eq18_e293_d_n8: f64 = (locals.var_xqtex_dn8 + locals.var_xqex_dn8);
        let eq18_e293_d_n9: f64 = (locals.var_xqtex_dn9 + locals.var_xqex_dn9);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e295_q: f64 = eq18_e294;
        let eq18_e297: f64 = (eq18_e294 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_q: f64 = (eq18_e295_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, 0.0, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, 0.0];
        let eq18_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e312: f64 = (locals.var_qtex + locals.var_qex);
        let eq20_e312_d_n0: f64 = (locals.var_qtex_dn0 + locals.var_qex_dn0);
        let eq20_e312_d_n1: f64 = (locals.var_qtex_dn1 + locals.var_qex_dn1);
        let eq20_e312_d_n3: f64 = (locals.var_qtex_dn3 + locals.var_qex_dn3);
        let eq20_e312_d_n4: f64 = (locals.var_qtex_dn4 + locals.var_qex_dn4);
        let eq20_e312_d_n5: f64 = (locals.var_qtex_dn5 + locals.var_qex_dn5);
        let eq20_e312_d_n6: f64 = (locals.var_qtex_dn6 + locals.var_qex_dn6);
        let eq20_e312_d_n7: f64 = (locals.var_qtex_dn7 + locals.var_qex_dn7);
        let eq20_e312_d_n8: f64 = (locals.var_qtex_dn8 + locals.var_qex_dn8);
        let eq20_e312_d_n9: f64 = (locals.var_qtex_dn9 + locals.var_qex_dn9);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, 0.0, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e355_q: f64 = (nv10 - 0.0);
        let eq27_e356: f64 = (locals.var_taun * (nv10 - 0.0));
        let eq27_e356_d_n0: f64 = (locals.var_taun_dn0 * (nv10 - 0.0));
        let eq27_e356_d_n1: f64 = (locals.var_taun_dn1 * (nv10 - 0.0));
        let eq27_e356_d_n3: f64 = (locals.var_taun_dn3 * (nv10 - 0.0));
        let eq27_e356_d_n4: f64 = (locals.var_taun_dn4 * (nv10 - 0.0));
        let eq27_e356_d_n5: f64 = (locals.var_taun_dn5 * (nv10 - 0.0));
        let eq27_e356_d_n6: f64 = (locals.var_taun_dn6 * (nv10 - 0.0));
        let eq27_e356_d_n7: f64 = (locals.var_taun_dn7 * (nv10 - 0.0));
        let eq27_e356_d_n8: f64 = (locals.var_taun_dn8 * (nv10 - 0.0));
        let eq27_e356_d_n9: f64 = (locals.var_taun_dn9 * (nv10 - 0.0));
        let eq27_e356_q: f64 = (locals.var_taun * eq27_e355_q);
        let eq27_e356_q_d_n0: f64 = (locals.var_taun_dn0 * eq27_e355_q);
        let eq27_e356_q_d_n1: f64 = (locals.var_taun_dn1 * eq27_e355_q);
        let eq27_e356_q_d_n3: f64 = (locals.var_taun_dn3 * eq27_e355_q);
        let eq27_e356_q_d_n4: f64 = (locals.var_taun_dn4 * eq27_e355_q);
        let eq27_e356_q_d_n5: f64 = (locals.var_taun_dn5 * eq27_e355_q);
        let eq27_e356_q_d_n6: f64 = (locals.var_taun_dn6 * eq27_e355_q);
        let eq27_e356_q_d_n7: f64 = (locals.var_taun_dn7 * eq27_e355_q);
        let eq27_e356_q_d_n8: f64 = (locals.var_taun_dn8 * eq27_e355_q);
        let eq27_e356_q_d_n9: f64 = (locals.var_taun_dn9 * eq27_e355_q);
        let eq27_reactive_node_derivatives: [f64; 11] = [eq27_e356_q_d_n0, eq27_e356_q_d_n1, 0.0, eq27_e356_q_d_n3, eq27_e356_q_d_n4, eq27_e356_q_d_n5, eq27_e356_q_d_n6, eq27_e356_q_d_n7, eq27_e356_q_d_n8, eq27_e356_q_d_n9, locals.var_taun];
        let eq27_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
