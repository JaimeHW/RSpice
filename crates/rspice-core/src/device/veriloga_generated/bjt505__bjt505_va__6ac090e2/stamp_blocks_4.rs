#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_qb1b2: f64,
        var_qb1b2_db0: f64,
        var_qb1b2_db1: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn11: f64,
        var_qb1b2_dn2: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_db0: f64,
        var_qbc_db1: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn11: f64,
        var_qbc_dn2: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_db0: f64,
        var_qbe_db1: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn11: f64,
        var_qbe_dn2: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_db0: f64,
        var_qe_db1: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn10: f64,
        var_qe_dn11: f64,
        var_qe_dn2: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_db0: f64,
        var_qepi_db1: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn11: f64,
        var_qepi_dn2: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_db0: f64,
        var_qex_db1: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn2: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_db0: f64,
        var_qtc_db1: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn11: f64,
        var_qtc_dn2: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_db0: f64,
        var_qte_db1: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn11: f64,
        var_qte_dn2: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_db0: f64,
        var_qte_s_db1: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn10: f64,
        var_qte_s_dn11: f64,
        var_qte_s_dn2: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_db0: f64,
        var_qtex_db1: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn11: f64,
        var_qtex_dn2: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_qts: f64,
        var_qts_db0: f64,
        var_qts_db1: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn11: f64,
        var_qts_dn2: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_vbc: f64,
        var_vbc_db0: f64,
        var_vbc_db1: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbc_dn10: f64,
        var_vbc_dn11: f64,
        var_vbc_dn2: f64,
        var_vbc_dn3: f64,
        var_vbc_dn4: f64,
        var_vbc_dn5: f64,
        var_vbc_dn6: f64,
        var_vbc_dn7: f64,
        var_vbc_dn8: f64,
        var_vbc_dn9: f64,
        var_vbe: f64,
        var_vbe_db0: f64,
        var_vbe_db1: f64,
        var_vbe_dn0: f64,
        var_vbe_dn1: f64,
        var_vbe_dn10: f64,
        var_vbe_dn11: f64,
        var_vbe_dn2: f64,
        var_vbe_dn3: f64,
        var_vbe_dn4: f64,
        var_vbe_dn5: f64,
        var_vbe_dn6: f64,
        var_vbe_dn7: f64,
        var_vbe_dn8: f64,
        var_vbe_dn9: f64,
        var_xqex: f64,
        var_xqex_db0: f64,
        var_xqex_db1: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn11: f64,
        var_xqex_dn2: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_db0: f64,
        var_xqtex_db1: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn10: f64,
        var_xqtex_dn11: f64,
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let eq14_e266: f64 = (var_qte + var_qbe);
        let eq14_e266_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq14_e266_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq14_e266_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq14_e266_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq14_e266_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq14_e266_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq14_e266_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq14_e266_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq14_e266_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq14_e266_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq14_e266_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq14_e266_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq14_e266_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq14_e266_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq14_e268: f64 = (eq14_e266 + var_qe);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + var_qe_dn0);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + var_qe_dn1);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + var_qe_dn2);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + var_qe_dn3);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + var_qe_dn4);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + var_qe_dn5);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + var_qe_dn6);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + var_qe_dn7);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + var_qe_dn8);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + var_qe_dn9);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + var_qe_dn10);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + var_qe_dn11);
        let eq14_e268_d_b0: f64 = (eq14_e266_d_b0 + var_qe_db0);
        let eq14_e268_d_b1: f64 = (eq14_e266_d_b1 + var_qe_db1);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e269_d_b0: f64 = (p.p3 * eq14_e268_d_b0);
        let eq14_e269_d_b1: f64 = (p.p3 * eq14_e268_d_b1);
        let eq14_e270_q: f64 = eq14_e269;
        let eq14_e272: f64 = (eq14_e269 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_e272_d_b0: f64 = (eq14_e269_d_b0 * p.p1);
        let eq14_e272_d_b1: f64 = (eq14_e269_d_b1 * p.p1);
        let eq14_e272_q: f64 = (eq14_e270_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e272_d_b0, eq14_e272_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * var_qte_s);
        let eq15_e275_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq15_e275_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq15_e275_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq15_e275_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq15_e275_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq15_e275_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq15_e275_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq15_e275_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq15_e275_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq15_e275_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq15_e275_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq15_e275_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq15_e275_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq15_e275_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq15_e276_q: f64 = eq15_e275;
        let eq15_e278: f64 = (eq15_e275 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_e278_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e278_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_e278_q: f64 = (eq15_e276_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e278_d_b0, eq15_e278_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e282: f64 = (var_qtc + var_qbc);
        let eq16_e282_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq16_e282_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq16_e282_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq16_e282_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq16_e282_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq16_e282_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq16_e282_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq16_e282_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq16_e282_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq16_e282_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq16_e282_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq16_e282_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq16_e282_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq16_e282_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq16_e284: f64 = (eq16_e282 + var_qepi);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + var_qepi_dn0);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + var_qepi_dn1);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + var_qepi_dn2);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + var_qepi_dn3);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + var_qepi_dn4);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + var_qepi_dn5);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + var_qepi_dn6);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + var_qepi_dn7);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + var_qepi_dn8);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + var_qepi_dn9);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + var_qepi_dn10);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + var_qepi_dn11);
        let eq16_e284_d_b0: f64 = (eq16_e282_d_b0 + var_qepi_db0);
        let eq16_e284_d_b1: f64 = (eq16_e282_d_b1 + var_qepi_db1);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e285_d_b0: f64 = (p.p3 * eq16_e284_d_b0);
        let eq16_e285_d_b1: f64 = (p.p3 * eq16_e284_d_b1);
        let eq16_e286_q: f64 = eq16_e285;
        let eq16_e288: f64 = (eq16_e285 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_e288_d_b0: f64 = (eq16_e285_d_b0 * p.p1);
        let eq16_e288_d_b1: f64 = (eq16_e285_d_b1 * p.p1);
        let eq16_e288_q: f64 = (eq16_e286_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e288_d_b0, eq16_e288_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * var_qts);
        let eq17_e291_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq17_e291_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq17_e291_d_n2: f64 = (p.p3 * var_qts_dn2);
        let eq17_e291_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq17_e291_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq17_e291_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq17_e291_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq17_e291_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq17_e291_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq17_e291_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq17_e291_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq17_e291_d_n11: f64 = (p.p3 * var_qts_dn11);
        let eq17_e291_d_b0: f64 = (p.p3 * var_qts_db0);
        let eq17_e291_d_b1: f64 = (p.p3 * var_qts_db1);
        let eq17_e292_q: f64 = eq17_e291;
        let eq17_e294: f64 = (eq17_e291 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_e294_d_b0: f64 = (eq17_e291_d_b0 * p.p1);
        let eq17_e294_d_b1: f64 = (eq17_e291_d_b1 * p.p1);
        let eq17_e294_q: f64 = (eq17_e292_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e294_d_b0, eq17_e294_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * var_qb1b2);
        let eq18_e297_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq18_e297_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq18_e297_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq18_e297_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq18_e297_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq18_e297_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq18_e297_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq18_e297_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq18_e297_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq18_e297_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq18_e297_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq18_e297_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq18_e297_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq18_e297_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq18_e298_q: f64 = eq18_e297;
        let eq18_e300: f64 = (eq18_e297 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_e300_d_b0: f64 = (eq18_e297_d_b0 * p.p1);
        let eq18_e300_d_b1: f64 = (eq18_e297_d_b1 * p.p1);
        let eq18_e300_q: f64 = (eq18_e298_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e300_d_b0, eq18_e300_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * var_vbe);
        let eq19_e305_d_n0: f64 = (eq19_e303 * var_vbe_dn0);
        let eq19_e305_d_n1: f64 = (eq19_e303 * var_vbe_dn1);
        let eq19_e305_d_n2: f64 = (eq19_e303 * var_vbe_dn2);
        let eq19_e305_d_n3: f64 = (eq19_e303 * var_vbe_dn3);
        let eq19_e305_d_n4: f64 = (eq19_e303 * var_vbe_dn4);
        let eq19_e305_d_n5: f64 = (eq19_e303 * var_vbe_dn5);
        let eq19_e305_d_n6: f64 = (eq19_e303 * var_vbe_dn6);
        let eq19_e305_d_n7: f64 = (eq19_e303 * var_vbe_dn7);
        let eq19_e305_d_n8: f64 = (eq19_e303 * var_vbe_dn8);
        let eq19_e305_d_n9: f64 = (eq19_e303 * var_vbe_dn9);
        let eq19_e305_d_n10: f64 = (eq19_e303 * var_vbe_dn10);
        let eq19_e305_d_n11: f64 = (eq19_e303 * var_vbe_dn11);
        let eq19_e305_d_b0: f64 = (eq19_e303 * var_vbe_db0);
        let eq19_e305_d_b1: f64 = (eq19_e303 * var_vbe_db1);
        let eq19_e306_q: f64 = eq19_e305;
        let eq19_e308: f64 = (eq19_e305 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_e308_d_b0: f64 = (eq19_e305_d_b0 * p.p1);
        let eq19_e308_d_b1: f64 = (eq19_e305_d_b1 * p.p1);
        let eq19_e308_q: f64 = (eq19_e306_q * p.p1);
        let eq19_reactive_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * var_vbc);
        let eq20_e313_d_n0: f64 = (eq20_e311 * var_vbc_dn0);
        let eq20_e313_d_n1: f64 = (eq20_e311 * var_vbc_dn1);
        let eq20_e313_d_n2: f64 = (eq20_e311 * var_vbc_dn2);
        let eq20_e313_d_n3: f64 = (eq20_e311 * var_vbc_dn3);
        let eq20_e313_d_n4: f64 = (eq20_e311 * var_vbc_dn4);
        let eq20_e313_d_n5: f64 = (eq20_e311 * var_vbc_dn5);
        let eq20_e313_d_n6: f64 = (eq20_e311 * var_vbc_dn6);
        let eq20_e313_d_n7: f64 = (eq20_e311 * var_vbc_dn7);
        let eq20_e313_d_n8: f64 = (eq20_e311 * var_vbc_dn8);
        let eq20_e313_d_n9: f64 = (eq20_e311 * var_vbc_dn9);
        let eq20_e313_d_n10: f64 = (eq20_e311 * var_vbc_dn10);
        let eq20_e313_d_n11: f64 = (eq20_e311 * var_vbc_dn11);
        let eq20_e313_d_b0: f64 = (eq20_e311 * var_vbc_db0);
        let eq20_e313_d_b1: f64 = (eq20_e311 * var_vbc_db1);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_e316_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
        let eq20_e316_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e332: f64 = (var_xqtex + var_xqex);
        let eq23_e332_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq23_e332_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq23_e332_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq23_e332_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq23_e332_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq23_e332_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq23_e332_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq23_e332_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq23_e332_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq23_e332_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq23_e332_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq23_e332_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq23_e332_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq23_e332_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e333_d_b0: f64 = (p.p3 * eq23_e332_d_b0);
        let eq23_e333_d_b1: f64 = (p.p3 * eq23_e332_d_b1);
        let eq23_e334_q: f64 = eq23_e333;
        let eq23_e336: f64 = (eq23_e333 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_e336_d_b0: f64 = (eq23_e333_d_b0 * p.p1);
        let eq23_e336_d_b1: f64 = (eq23_e333_d_b1 * p.p1);
        let eq23_e336_q: f64 = (eq23_e334_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e336_d_b0, eq23_e336_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (var_qtex + var_qex);
        let eq25_e351_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq25_e351_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq25_e351_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq25_e351_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq25_e351_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq25_e351_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq25_e351_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq25_e351_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq25_e351_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq25_e351_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq25_e351_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq25_e351_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq25_e351_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq25_e351_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e352_d_b0: f64 = (p.p3 * eq25_e351_d_b0);
        let eq25_e352_d_b1: f64 = (p.p3 * eq25_e351_d_b1);
        let eq25_e353_q: f64 = eq25_e352;
        let eq25_e355: f64 = (eq25_e352 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_e355_d_b0: f64 = (eq25_e352_d_b0 * p.p1);
        let eq25_e355_d_b1: f64 = (eq25_e352_d_b1 * p.p1);
        let eq25_e355_q: f64 = (eq25_e353_q * p.p1);
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_reactive_branch_derivatives: [f64; 2] = [eq25_e355_d_b0, eq25_e355_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_taun: f64,
        var_taun_db0: f64,
        var_taun_db1: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn11: f64,
        var_taun_dn2: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq32_e394_q: f64 = (nv11 - 0.0);
        let eq32_e395: f64 = (var_taun * (nv11 - 0.0));
        let eq32_e395_d_n0: f64 = (var_taun_dn0 * (nv11 - 0.0));
        let eq32_e395_d_n1: f64 = (var_taun_dn1 * (nv11 - 0.0));
        let eq32_e395_d_n2: f64 = (var_taun_dn2 * (nv11 - 0.0));
        let eq32_e395_d_n3: f64 = (var_taun_dn3 * (nv11 - 0.0));
        let eq32_e395_d_n4: f64 = (var_taun_dn4 * (nv11 - 0.0));
        let eq32_e395_d_n5: f64 = (var_taun_dn5 * (nv11 - 0.0));
        let eq32_e395_d_n6: f64 = (var_taun_dn6 * (nv11 - 0.0));
        let eq32_e395_d_n7: f64 = (var_taun_dn7 * (nv11 - 0.0));
        let eq32_e395_d_n8: f64 = (var_taun_dn8 * (nv11 - 0.0));
        let eq32_e395_d_n9: f64 = (var_taun_dn9 * (nv11 - 0.0));
        let eq32_e395_d_n10: f64 = (var_taun_dn10 * (nv11 - 0.0));
        let eq32_e395_d_n11: f64 = ((var_taun_dn11 * (nv11 - 0.0)) + var_taun);
        let eq32_e395_d_b0: f64 = (var_taun_db0 * (nv11 - 0.0));
        let eq32_e395_d_b1: f64 = (var_taun_db1 * (nv11 - 0.0));
        let eq32_e395_q: f64 = (var_taun * eq32_e394_q);
        let eq32_e395_q_d_n0: f64 = (var_taun_dn0 * eq32_e394_q);
        let eq32_e395_q_d_n1: f64 = (var_taun_dn1 * eq32_e394_q);
        let eq32_e395_q_d_n2: f64 = (var_taun_dn2 * eq32_e394_q);
        let eq32_e395_q_d_n3: f64 = (var_taun_dn3 * eq32_e394_q);
        let eq32_e395_q_d_n4: f64 = (var_taun_dn4 * eq32_e394_q);
        let eq32_e395_q_d_n5: f64 = (var_taun_dn5 * eq32_e394_q);
        let eq32_e395_q_d_n6: f64 = (var_taun_dn6 * eq32_e394_q);
        let eq32_e395_q_d_n7: f64 = (var_taun_dn7 * eq32_e394_q);
        let eq32_e395_q_d_n8: f64 = (var_taun_dn8 * eq32_e394_q);
        let eq32_e395_q_d_n9: f64 = (var_taun_dn9 * eq32_e394_q);
        let eq32_e395_q_d_n10: f64 = (var_taun_dn10 * eq32_e394_q);
        let eq32_e395_q_d_n11: f64 = ((var_taun_dn11 * eq32_e394_q) + var_taun);
        let eq32_e395_q_d_b0: f64 = (var_taun_db0 * eq32_e394_q);
        let eq32_e395_q_d_b1: f64 = (var_taun_db1 * eq32_e394_q);
        let eq32_reactive_node_derivatives: [f64; 12] = [eq32_e395_q_d_n0, eq32_e395_q_d_n1, eq32_e395_q_d_n2, eq32_e395_q_d_n3, eq32_e395_q_d_n4, eq32_e395_q_d_n5, eq32_e395_q_d_n6, eq32_e395_q_d_n7, eq32_e395_q_d_n8, eq32_e395_q_d_n9, eq32_e395_q_d_n10, eq32_e395_q_d_n11];
        let eq32_reactive_branch_derivatives: [f64; 2] = [eq32_e395_q_d_b0, eq32_e395_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
