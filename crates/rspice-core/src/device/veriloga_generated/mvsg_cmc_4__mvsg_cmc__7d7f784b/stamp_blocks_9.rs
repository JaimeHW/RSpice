#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq130_e1574, eq130_e1574_d_n2, eq130_e1574_d_n4, eq130_e1574_d_n7, eq130_e1574_d_n12, eq130_e1574_d_n13,) = {
    if (locals.var_guard311 == 0.0) {
        let eq130_e1567: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 126, locals.var_qgdfps4);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 127, eq130_e1570);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n2: f64 = ((locals.var_qgdfps4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq130_e1572_d_n12: f64 = ((locals.var_qgdfps4_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq130_e1572, eq130_e1572_d_n2, (locals.var_qgdfps4_dn4 * ddt_scale), (locals.var_qgdfps4_dn7 * ddt_scale), eq130_e1572_d_n12, (locals.var_qgdfps4_dn13 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq130_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq130_e1574_d_n2), multiplicity * (eq130_e1574_d_n4), multiplicity * (eq130_e1574_d_n7), multiplicity * (eq130_e1574_d_n12), multiplicity * (eq130_e1574_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq131_e1585, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n7, eq131_e1585_d_n12, eq131_e1585_d_n13,) = {
    if (locals.var_guard311 == 0.0) {
        let eq131_e1578: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 128, locals.var_qcfps4);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 129, eq131_e1581);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n7: f64 = ((locals.var_qcfps4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq131_e1583_d_n13: f64 = ((locals.var_qcfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq131_e1583, (locals.var_qcfps4_dn2 * ddt_scale), (locals.var_qcfps4_dn3 * ddt_scale), (locals.var_qcfps4_dn4 * ddt_scale), eq131_e1583_d_n7, (locals.var_qcfps4_dn12 * ddt_scale), eq131_e1583_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(13),
            multiplicity * (eq131_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq131_e1585_d_n2), multiplicity * (eq131_e1585_d_n3), multiplicity * (eq131_e1585_d_n4), multiplicity * (eq131_e1585_d_n7), multiplicity * (eq131_e1585_d_n12), multiplicity * (eq131_e1585_d_n13)],
            [],
            [],
            1.0,
        );
        let eq134_e1597: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 130, locals.var_qbfps4);
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1601: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 131, eq134_e1600);
        let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);
        let eq134_e1602_d_n3: f64 = ((locals.var_qbfps4_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq134_e1602_d_n13: f64 = ((locals.var_qbfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq134_value: f64 = eq134_e1602;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(13),
            multiplicity * (eq134_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * ((locals.var_qbfps4_dn2 * ddt_scale)), multiplicity * (eq134_e1602_d_n3), multiplicity * ((locals.var_qbfps4_dn4 * ddt_scale)), multiplicity * ((locals.var_qbfps4_dn7 * ddt_scale)), multiplicity * ((locals.var_qbfps4_dn12 * ddt_scale)), multiplicity * (eq134_e1602_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq142_e1656, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n28, eq142_e1656_d_n29,) = {
    if (locals.var_guard416 == 0.0) {
        let eq142_e1649: f64 = (locals.var_ids - (nv29 - 0.0));
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 132, eq142_e1652);
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);
        let eq142_e1654_d_n28: f64 = (-(p.p323 * ddt_scale));
        (eq142_e1654, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn22, locals.var_ids_dn23, locals.var_ids_dn25, locals.var_ids_dn26, eq142_e1654_d_n28, (-1.0),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(28),
            None,
            multiplicity * (eq142_value),
            [4, 5, 8, 9, 22, 23, 25, 26, 28, 29],
            [multiplicity * (eq142_e1656_d_n4), multiplicity * (eq142_e1656_d_n5), multiplicity * (eq142_e1656_d_n8), multiplicity * (eq142_e1656_d_n9), multiplicity * (eq142_e1656_d_n22), multiplicity * (eq142_e1656_d_n23), multiplicity * (eq142_e1656_d_n25), multiplicity * (eq142_e1656_d_n26), multiplicity * (eq142_e1656_d_n28), multiplicity * (eq142_e1656_d_n29)],
            [],
            [],
            1.0,
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (locals.var_guard416 == 0.0) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 133, eq143_e1666);
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1667);
        let eq143_e1668_d_n29: f64 = ((-1.0) - (eq143_e1664 * ddt_scale));
        (eq143_e1668, 1.0, eq143_e1668_d_n29,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1670;
        stamper.stamp_current_node2_local(
            Some(29),
            None,
            multiplicity * (eq143_value),
            28,
            multiplicity * (eq143_e1670_d_n28),
            29,
            multiplicity * (eq143_e1670_d_n29),
        );
        let eq145_e1681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 134, locals.var_qgs);
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 135, eq145_e1684);
        let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);
        let eq145_e1686_d_n8: f64 = ((locals.var_qgs_dn8 * ddt_scale) + (p.p355 * ddt_scale));
        let eq145_e1686_d_n9: f64 = ((locals.var_qgs_dn9 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq145_value: f64 = eq145_e1686;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq145_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((locals.var_qgs_dn4 * ddt_scale)), multiplicity * ((locals.var_qgs_dn5 * ddt_scale)), multiplicity * (eq145_e1686_d_n8), multiplicity * (eq145_e1686_d_n9), multiplicity * ((locals.var_qgs_dn22 * ddt_scale)), multiplicity * ((locals.var_qgs_dn23 * ddt_scale)), multiplicity * ((locals.var_qgs_dn25 * ddt_scale)), multiplicity * ((locals.var_qgs_dn26 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq146_e1688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 136, locals.var_qgd);
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1692: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 137, eq146_e1691);
        let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);
        let eq146_e1693_d_n5: f64 = ((locals.var_qgd_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq146_e1693_d_n8: f64 = ((locals.var_qgd_dn8 * ddt_scale) + (p.p355 * ddt_scale));
        let eq146_value: f64 = eq146_e1693;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq146_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((locals.var_qgd_dn4 * ddt_scale)), multiplicity * (eq146_e1693_d_n5), multiplicity * (eq146_e1693_d_n8), multiplicity * ((locals.var_qgd_dn9 * ddt_scale)), multiplicity * ((locals.var_qgd_dn22 * ddt_scale)), multiplicity * ((locals.var_qgd_dn23 * ddt_scale)), multiplicity * ((locals.var_qgd_dn25 * ddt_scale)), multiplicity * ((locals.var_qgd_dn26 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq157_e1796, eq157_e1796_d_n7, eq157_e1796_d_n8,) = {
    if (locals.var_guard461 != 0.0) {
        let eq157_e1794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 138, locals.var_qsch);
        (eq157_e1794, (locals.var_qsch_dn7 * ddt_scale), (locals.var_qsch_dn8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (eq157_value),
            7,
            multiplicity * (eq157_e1796_d_n7),
            8,
            multiplicity * (eq157_e1796_d_n8),
        );
        let (eq194_e2167, eq194_e2167_d_n4,) = {
    if (locals.var_guard523 != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 145, eq194_e2164);
        (eq194_e2165, (p.p321 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2167;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq194_value),
            4,
            multiplicity * (eq194_e2167_d_n4),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_q,) = {
    if (locals.var_guard12 != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e419_q: f64 = eq8_e418;
        (eq8_e418, (-p.p330), p.p330, eq8_e419_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[21]),
            Some(nodes[20]),
            nodes[20],
            multiplicity * (eq8_e421_d_n20),
            nodes[21],
            multiplicity * (eq8_e421_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20, eq9_e428_q,) = {
    if (locals.var_guard12 != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e426_q: f64 = eq9_e425;
        (eq9_e425, p.p332, eq9_e426_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[20]),
            None,
            nodes[20],
            multiplicity * (eq9_e428_d_n20),
        );
        let (eq17_e564, eq17_e564_d_n4, eq17_e564_d_n23, eq17_e564_q, eq17_e564_q_d_n4,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let eq17_e543_q: f64 = locals.var_vdloutput;
        let eq17_e544: f64 = (p.p341 * locals.var_vdloutput);
        let eq17_e544_d_n23: f64 = (p.p341 * locals.var_vdloutput_dn23);
        let eq17_e544_q: f64 = (p.p341 * eq17_e543_q);
        let eq17_e549: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n4: f64 = (p.p342 * locals.var_tdut_dn4);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n4: f64 = (p.p344 * locals.var_tdut_dn4);
        let eq17_e559: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * locals.var_tdut_dn4));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n4: f64 = (eq17_e544 * eq17_e561_d_n4);
        let eq17_e562_d_n23: f64 = (eq17_e544_d_n23 * eq17_e561);
        let eq17_e562_q: f64 = (eq17_e544_q * eq17_e561);
        let eq17_e562_q_d_n4: f64 = (eq17_e544_q * eq17_e561_d_n4);
        (eq17_e562, eq17_e562_d_n4, eq17_e562_d_n23, eq17_e562_q, eq17_e562_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[23]),
            None,
            nodes[4],
            multiplicity * (eq17_e564_q_d_n4),
            nodes[23],
            multiplicity * (eq17_e564_d_n23),
        );
        let (eq22_e682, eq22_e682_d_n4, eq22_e682_d_n26, eq22_e682_q, eq22_e682_q_d_n4,) = {
    if ((locals.var_guard12 == 0.0) && (locals.var_guard13 != 0.0)) {
        let eq22_e661_q: f64 = locals.var_vgloutput;
        let eq22_e662: f64 = (p.p341 * locals.var_vgloutput);
        let eq22_e662_d_n26: f64 = (p.p341 * locals.var_vgloutput_dn26);
        let eq22_e662_q: f64 = (p.p341 * eq22_e661_q);
        let eq22_e667: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq22_e668: f64 = (p.p343 * eq22_e667);
        let eq22_e668_d_n4: f64 = (p.p343 * locals.var_tdut_dn4);
        let eq22_e669: f64 = (1.0 + eq22_e668);
        let eq22_e673: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq22_e674: f64 = (p.p345 * eq22_e673);
        let eq22_e674_d_n4: f64 = (p.p345 * locals.var_tdut_dn4);
        let eq22_e677: f64 = (locals.var_tdut - locals.var_tnomk);
        let eq22_e678: f64 = (eq22_e674 * eq22_e677);
        let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * locals.var_tdut_dn4));
        let eq22_e679: f64 = (eq22_e669 + eq22_e678);
        let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);
        let eq22_e680: f64 = (eq22_e662 * eq22_e679);
        let eq22_e680_d_n4: f64 = (eq22_e662 * eq22_e679_d_n4);
        let eq22_e680_d_n26: f64 = (eq22_e662_d_n26 * eq22_e679);
        let eq22_e680_q: f64 = (eq22_e662_q * eq22_e679);
        let eq22_e680_q_d_n4: f64 = (eq22_e662_q * eq22_e679_d_n4);
        (eq22_e680, eq22_e680_d_n4, eq22_e680_d_n26, eq22_e680_q, eq22_e680_q_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[26]),
            None,
            nodes[4],
            multiplicity * (eq22_e682_q_d_n4),
            nodes[26],
            multiplicity * (eq22_e682_d_n26),
        );
        let (eq33_e769, eq33_e769_d_n2, eq33_e769_d_n4, eq33_e769_d_n7, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_q,) = {
    if (locals.var_guard59 != 0.0) {
        let eq33_e762_q: f64 = locals.var_qgsfp4;
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e766_q: f64 = eq33_e765;
        let eq33_e767: f64 = (locals.var_qgsfp4 + eq33_e765);
        let eq33_e767_d_n7: f64 = (locals.var_qgsfp4_dn7 + p.p355);
        let eq33_e767_d_n16: f64 = (locals.var_qgsfp4_dn16 + (-p.p355));
        let eq33_e767_q: f64 = (eq33_e762_q + eq33_e766_q);
        (eq33_e767, locals.var_qgsfp4_dn2, locals.var_qgsfp4_dn4, eq33_e767_d_n7, eq33_e767_d_n16, locals.var_qgsfp4_dn17, eq33_e767_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq33_e769_d_n2, 0.0, eq33_e769_d_n4, 0.0, 0.0, eq33_e769_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq33_e769_d_n16, eq33_e769_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n2, eq34_e779_d_n4, eq34_e779_d_n7, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_q,) = {
    if (locals.var_guard59 != 0.0) {
        let eq34_e772_q: f64 = locals.var_qgdfp4;
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e776_q: f64 = eq34_e775;
        let eq34_e777: f64 = (locals.var_qgdfp4 + eq34_e775);
        let eq34_e777_d_n7: f64 = (locals.var_qgdfp4_dn7 + p.p355);
        let eq34_e777_d_n17: f64 = (locals.var_qgdfp4_dn17 + (-p.p355));
        let eq34_e777_q: f64 = (eq34_e772_q + eq34_e776_q);
        (eq34_e777, locals.var_qgdfp4_dn2, locals.var_qgdfp4_dn4, eq34_e777_d_n7, locals.var_qgdfp4_dn16, eq34_e777_d_n17, eq34_e777_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq34_e779_d_n2, 0.0, eq34_e779_d_n4, 0.0, 0.0, eq34_e779_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq34_e779_d_n16, eq34_e779_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[17]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n7, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_q,) = {
    if (locals.var_guard59 != 0.0) {
        let eq35_e782_q: f64 = locals.var_qcfp4;
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e786_q: f64 = eq35_e785;
        let eq35_e787: f64 = (locals.var_qcfp4 + eq35_e785);
        let eq35_e787_d_n2: f64 = (locals.var_qcfp4_dn2 + p.p355);
        let eq35_e787_d_n16: f64 = (locals.var_qcfp4_dn16 + (-p.p355));
        let eq35_e787_q: f64 = (eq35_e782_q + eq35_e786_q);
        (eq35_e787, eq35_e787_d_n2, locals.var_qcfp4_dn3, locals.var_qcfp4_dn4, locals.var_qcfp4_dn7, eq35_e787_d_n16, locals.var_qcfp4_dn17, eq35_e787_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, 0.0, 0.0, eq35_e789_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq35_e789_d_n16, eq35_e789_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e803, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n7, eq37_e803_d_n9, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_q,) = {
    if (locals.var_guard59 != 0.0) {
        let eq37_e796_q: f64 = locals.var_qsfp4;
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e800_q: f64 = eq37_e799;
        let eq37_e801: f64 = (locals.var_qsfp4 + eq37_e799);
        let eq37_e801_d_n7: f64 = (locals.var_qsfp4_dn7 + p.p355);
        let eq37_e801_q: f64 = (eq37_e796_q + eq37_e800_q);
        (eq37_e801, locals.var_qsfp4_dn2, locals.var_qsfp4_dn3, locals.var_qsfp4_dn4, eq37_e801_d_n7, (-p.p355), locals.var_qsfp4_dn16, locals.var_qsfp4_dn17, eq37_e801_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, 0.0, 0.0, eq37_e803_d_n7, 0.0, eq37_e803_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq37_e803_d_n16, eq37_e803_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq37_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n2, eq38_e814_d_n4, eq38_e814_d_n7, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_q,) = {
    if (locals.var_guard59 == 0.0) {
        let eq38_e807_q: f64 = locals.var_qgsfp4;
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e811_q: f64 = eq38_e810;
        let eq38_e812: f64 = (locals.var_qgsfp4 + eq38_e810);
        let eq38_e812_d_n2: f64 = (locals.var_qgsfp4_dn2 + p.p355);
        let eq38_e812_d_n16: f64 = (locals.var_qgsfp4_dn16 + (-p.p355));
        let eq38_e812_q: f64 = (eq38_e807_q + eq38_e811_q);
        (eq38_e812, eq38_e812_d_n2, locals.var_qgsfp4_dn4, locals.var_qgsfp4_dn7, eq38_e812_d_n16, locals.var_qgsfp4_dn17, eq38_e812_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq38_e814_d_n2, 0.0, eq38_e814_d_n4, 0.0, 0.0, eq38_e814_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq38_e814_d_n16, eq38_e814_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n2, eq39_e825_d_n4, eq39_e825_d_n7, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_q,) = {
    if (locals.var_guard59 == 0.0) {
        let eq39_e818_q: f64 = locals.var_qgdfp4;
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e822_q: f64 = eq39_e821;
        let eq39_e823: f64 = (locals.var_qgdfp4 + eq39_e821);
        let eq39_e823_d_n2: f64 = (locals.var_qgdfp4_dn2 + p.p355);
        let eq39_e823_d_n17: f64 = (locals.var_qgdfp4_dn17 + (-p.p355));
        let eq39_e823_q: f64 = (eq39_e818_q + eq39_e822_q);
        (eq39_e823, eq39_e823_d_n2, locals.var_qgdfp4_dn4, locals.var_qgdfp4_dn7, locals.var_qgdfp4_dn16, eq39_e823_d_n17, eq39_e823_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq39_e825_d_n2, 0.0, eq39_e825_d_n4, 0.0, 0.0, eq39_e825_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq39_e825_d_n16, eq39_e825_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq39_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n7, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_q,) = {
    if (locals.var_guard59 == 0.0) {
        let eq40_e829_q: f64 = locals.var_qcfp4;
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e833_q: f64 = eq40_e832;
        let eq40_e834: f64 = (locals.var_qcfp4 + eq40_e832);
        let eq40_e834_d_n7: f64 = (locals.var_qcfp4_dn7 + p.p355);
        let eq40_e834_d_n16: f64 = (locals.var_qcfp4_dn16 + (-p.p355));
        let eq40_e834_q: f64 = (eq40_e829_q + eq40_e833_q);
        (eq40_e834, locals.var_qcfp4_dn2, locals.var_qcfp4_dn3, locals.var_qcfp4_dn4, eq40_e834_d_n7, eq40_e834_d_n16, locals.var_qcfp4_dn17, eq40_e834_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, 0.0, 0.0, eq40_e836_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq40_e836_d_n16, eq40_e836_d_n17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq40_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e848_q: f64 = locals.var_qbfp4;
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e852_q: f64 = eq43_e851;
        let eq43_e853: f64 = (locals.var_qbfp4 + eq43_e851);
        let eq43_e853_d_n3: f64 = (locals.var_qbfp4_dn3 + p.p355);
        let eq43_e853_d_n16: f64 = (locals.var_qbfp4_dn16 + (-p.p355));
        let eq43_e853_q: f64 = (eq43_e848_q + eq43_e852_q);
        let eq43_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfp4_dn2, eq43_e853_d_n3, locals.var_qbfp4_dn4, 0.0, 0.0, locals.var_qbfp4_dn7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq43_e853_d_n16, locals.var_qbfp4_dn17, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e876, eq46_e876_d_n2, eq46_e876_d_n4, eq46_e876_d_n7, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_q,) = {
    if (locals.var_guard95 != 0.0) {
        let eq46_e869_q: f64 = locals.var_qgsfp3;
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e873_q: f64 = eq46_e872;
        let eq46_e874: f64 = (locals.var_qgsfp3 + eq46_e872);
        let eq46_e874_d_n7: f64 = (locals.var_qgsfp3_dn7 + p.p355);
        let eq46_e874_d_n15: f64 = (locals.var_qgsfp3_dn15 + (-p.p355));
        let eq46_e874_q: f64 = (eq46_e869_q + eq46_e873_q);
        (eq46_e874, locals.var_qgsfp3_dn2, locals.var_qgsfp3_dn4, eq46_e874_d_n7, eq46_e874_d_n15, locals.var_qgsfp3_dn16, eq46_e874_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq46_e876_d_n2, 0.0, eq46_e876_d_n4, 0.0, 0.0, eq46_e876_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq46_e876_d_n15, eq46_e876_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n2, eq47_e886_d_n4, eq47_e886_d_n7, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_q,) = {
    if (locals.var_guard95 != 0.0) {
        let eq47_e879_q: f64 = locals.var_qgdfp3;
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e883_q: f64 = eq47_e882;
        let eq47_e884: f64 = (locals.var_qgdfp3 + eq47_e882);
        let eq47_e884_d_n7: f64 = (locals.var_qgdfp3_dn7 + p.p355);
        let eq47_e884_d_n16: f64 = (locals.var_qgdfp3_dn16 + (-p.p355));
        let eq47_e884_q: f64 = (eq47_e879_q + eq47_e883_q);
        (eq47_e884, locals.var_qgdfp3_dn2, locals.var_qgdfp3_dn4, eq47_e884_d_n7, locals.var_qgdfp3_dn15, eq47_e884_d_n16, eq47_e884_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq47_e886_d_n2, 0.0, eq47_e886_d_n4, 0.0, 0.0, eq47_e886_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq47_e886_d_n15, eq47_e886_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n7, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_q,) = {
    if (locals.var_guard95 != 0.0) {
        let eq48_e889_q: f64 = locals.var_qcfp3;
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e893_q: f64 = eq48_e892;
        let eq48_e894: f64 = (locals.var_qcfp3 + eq48_e892);
        let eq48_e894_d_n2: f64 = (locals.var_qcfp3_dn2 + p.p355);
        let eq48_e894_d_n15: f64 = (locals.var_qcfp3_dn15 + (-p.p355));
        let eq48_e894_q: f64 = (eq48_e889_q + eq48_e893_q);
        (eq48_e894, eq48_e894_d_n2, locals.var_qcfp3_dn3, locals.var_qcfp3_dn4, locals.var_qcfp3_dn7, eq48_e894_d_n15, locals.var_qcfp3_dn16, eq48_e894_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, 0.0, 0.0, eq48_e896_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq48_e896_d_n15, eq48_e896_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e910, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n7, eq50_e910_d_n9, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_q,) = {
    if (locals.var_guard95 != 0.0) {
        let eq50_e903_q: f64 = locals.var_qsfp3;
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e907_q: f64 = eq50_e906;
        let eq50_e908: f64 = (locals.var_qsfp3 + eq50_e906);
        let eq50_e908_d_n7: f64 = (locals.var_qsfp3_dn7 + p.p355);
        let eq50_e908_q: f64 = (eq50_e903_q + eq50_e907_q);
        (eq50_e908, locals.var_qsfp3_dn2, locals.var_qsfp3_dn3, locals.var_qsfp3_dn4, eq50_e908_d_n7, (-p.p355), locals.var_qsfp3_dn15, locals.var_qsfp3_dn16, eq50_e908_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, 0.0, 0.0, eq50_e910_d_n7, 0.0, eq50_e910_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, eq50_e910_d_n15, eq50_e910_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n2, eq51_e921_d_n4, eq51_e921_d_n7, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_q,) = {
    if (locals.var_guard95 == 0.0) {
        let eq51_e914_q: f64 = locals.var_qgsfp3;
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e918_q: f64 = eq51_e917;
        let eq51_e919: f64 = (locals.var_qgsfp3 + eq51_e917);
        let eq51_e919_d_n2: f64 = (locals.var_qgsfp3_dn2 + p.p355);
        let eq51_e919_d_n15: f64 = (locals.var_qgsfp3_dn15 + (-p.p355));
        let eq51_e919_q: f64 = (eq51_e914_q + eq51_e918_q);
        (eq51_e919, eq51_e919_d_n2, locals.var_qgsfp3_dn4, locals.var_qgsfp3_dn7, eq51_e919_d_n15, locals.var_qgsfp3_dn16, eq51_e919_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq51_e921_d_n2, 0.0, eq51_e921_d_n4, 0.0, 0.0, eq51_e921_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq51_e921_d_n15, eq51_e921_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n2, eq52_e932_d_n4, eq52_e932_d_n7, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_q,) = {
    if (locals.var_guard95 == 0.0) {
        let eq52_e925_q: f64 = locals.var_qgdfp3;
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e929_q: f64 = eq52_e928;
        let eq52_e930: f64 = (locals.var_qgdfp3 + eq52_e928);
        let eq52_e930_d_n2: f64 = (locals.var_qgdfp3_dn2 + p.p355);
        let eq52_e930_d_n16: f64 = (locals.var_qgdfp3_dn16 + (-p.p355));
        let eq52_e930_q: f64 = (eq52_e925_q + eq52_e929_q);
        (eq52_e930, eq52_e930_d_n2, locals.var_qgdfp3_dn4, locals.var_qgdfp3_dn7, locals.var_qgdfp3_dn15, eq52_e930_d_n16, eq52_e930_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq52_e932_d_n2, 0.0, eq52_e932_d_n4, 0.0, 0.0, eq52_e932_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq52_e932_d_n15, eq52_e932_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq52_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n7, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_q,) = {
    if (locals.var_guard95 == 0.0) {
        let eq53_e936_q: f64 = locals.var_qcfp3;
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e940_q: f64 = eq53_e939;
        let eq53_e941: f64 = (locals.var_qcfp3 + eq53_e939);
        let eq53_e941_d_n7: f64 = (locals.var_qcfp3_dn7 + p.p355);
        let eq53_e941_d_n15: f64 = (locals.var_qcfp3_dn15 + (-p.p355));
        let eq53_e941_q: f64 = (eq53_e936_q + eq53_e940_q);
        (eq53_e941, locals.var_qcfp3_dn2, locals.var_qcfp3_dn3, locals.var_qcfp3_dn4, eq53_e941_d_n7, eq53_e941_d_n15, locals.var_qcfp3_dn16, eq53_e941_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, 0.0, 0.0, eq53_e943_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq53_e943_d_n15, eq53_e943_d_n16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e955_q: f64 = locals.var_qbfp3;
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e959_q: f64 = eq56_e958;
        let eq56_e960: f64 = (locals.var_qbfp3 + eq56_e958);
        let eq56_e960_d_n3: f64 = (locals.var_qbfp3_dn3 + p.p355);
        let eq56_e960_d_n15: f64 = (locals.var_qbfp3_dn15 + (-p.p355));
        let eq56_e960_q: f64 = (eq56_e955_q + eq56_e959_q);
        let eq56_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfp3_dn2, eq56_e960_d_n3, locals.var_qbfp3_dn4, 0.0, 0.0, locals.var_qbfp3_dn7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq56_e960_d_n15, locals.var_qbfp3_dn16, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq56_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq59_e983, eq59_e983_d_n2, eq59_e983_d_n4, eq59_e983_d_n7, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_q,) = {
    if (locals.var_guard131 != 0.0) {
        let eq59_e976_q: f64 = locals.var_qgsfp2;
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e980_q: f64 = eq59_e979;
        let eq59_e981: f64 = (locals.var_qgsfp2 + eq59_e979);
        let eq59_e981_d_n7: f64 = (locals.var_qgsfp2_dn7 + p.p355);
        let eq59_e981_d_n14: f64 = (locals.var_qgsfp2_dn14 + (-p.p355));
        let eq59_e981_q: f64 = (eq59_e976_q + eq59_e980_q);
        (eq59_e981, locals.var_qgsfp2_dn2, locals.var_qgsfp2_dn4, eq59_e981_d_n7, eq59_e981_d_n14, locals.var_qgsfp2_dn15, eq59_e981_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq59_e983_d_n2, 0.0, eq59_e983_d_n4, 0.0, 0.0, eq59_e983_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq59_e983_d_n14, eq59_e983_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq59_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq59_reactive_node_derivatives,
            branches,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq60_e993, eq60_e993_d_n2, eq60_e993_d_n4, eq60_e993_d_n7, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_q,) = {
    if (locals.var_guard131 != 0.0) {
        let eq60_e986_q: f64 = locals.var_qgdfp2;
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e990_q: f64 = eq60_e989;
        let eq60_e991: f64 = (locals.var_qgdfp2 + eq60_e989);
        let eq60_e991_d_n7: f64 = (locals.var_qgdfp2_dn7 + p.p355);
        let eq60_e991_d_n15: f64 = (locals.var_qgdfp2_dn15 + (-p.p355));
        let eq60_e991_q: f64 = (eq60_e986_q + eq60_e990_q);
        (eq60_e991, locals.var_qgdfp2_dn2, locals.var_qgdfp2_dn4, eq60_e991_d_n7, locals.var_qgdfp2_dn14, eq60_e991_d_n15, eq60_e991_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq60_e993_d_n2, 0.0, eq60_e993_d_n4, 0.0, 0.0, eq60_e993_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq60_e993_d_n14, eq60_e993_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq60_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq60_reactive_node_derivatives,
            branches,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n7, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_q,) = {
    if (locals.var_guard131 != 0.0) {
        let eq61_e996_q: f64 = locals.var_qcfp2;
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e1000_q: f64 = eq61_e999;
        let eq61_e1001: f64 = (locals.var_qcfp2 + eq61_e999);
        let eq61_e1001_d_n2: f64 = (locals.var_qcfp2_dn2 + p.p355);
        let eq61_e1001_d_n14: f64 = (locals.var_qcfp2_dn14 + (-p.p355));
        let eq61_e1001_q: f64 = (eq61_e996_q + eq61_e1000_q);
        (eq61_e1001, eq61_e1001_d_n2, locals.var_qcfp2_dn3, locals.var_qcfp2_dn4, locals.var_qcfp2_dn7, eq61_e1001_d_n14, locals.var_qcfp2_dn15, eq61_e1001_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, 0.0, 0.0, eq61_e1003_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq61_e1003_d_n14, eq61_e1003_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq61_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1017, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n7, eq63_e1017_d_n9, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_q,) = {
    if (locals.var_guard131 != 0.0) {
        let eq63_e1010_q: f64 = locals.var_qsfp2;
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1014_q: f64 = eq63_e1013;
        let eq63_e1015: f64 = (locals.var_qsfp2 + eq63_e1013);
        let eq63_e1015_d_n7: f64 = (locals.var_qsfp2_dn7 + p.p355);
        let eq63_e1015_q: f64 = (eq63_e1010_q + eq63_e1014_q);
        (eq63_e1015, locals.var_qsfp2_dn2, locals.var_qsfp2_dn3, locals.var_qsfp2_dn4, eq63_e1015_d_n7, (-p.p355), locals.var_qsfp2_dn14, locals.var_qsfp2_dn15, eq63_e1015_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, 0.0, 0.0, eq63_e1017_d_n7, 0.0, eq63_e1017_d_n9, 0.0, 0.0, 0.0, 0.0, eq63_e1017_d_n14, eq63_e1017_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq63_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n2, eq64_e1028_d_n4, eq64_e1028_d_n7, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_q,) = {
    if (locals.var_guard131 == 0.0) {
        let eq64_e1021_q: f64 = locals.var_qgsfp2;
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1025_q: f64 = eq64_e1024;
        let eq64_e1026: f64 = (locals.var_qgsfp2 + eq64_e1024);
        let eq64_e1026_d_n2: f64 = (locals.var_qgsfp2_dn2 + p.p355);
        let eq64_e1026_d_n14: f64 = (locals.var_qgsfp2_dn14 + (-p.p355));
        let eq64_e1026_q: f64 = (eq64_e1021_q + eq64_e1025_q);
        (eq64_e1026, eq64_e1026_d_n2, locals.var_qgsfp2_dn4, locals.var_qgsfp2_dn7, eq64_e1026_d_n14, locals.var_qgsfp2_dn15, eq64_e1026_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq64_e1028_d_n2, 0.0, eq64_e1028_d_n4, 0.0, 0.0, eq64_e1028_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq64_e1028_d_n14, eq64_e1028_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq64_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq64_reactive_node_derivatives,
            branches,
            &eq64_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n2, eq65_e1039_d_n4, eq65_e1039_d_n7, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_q,) = {
    if (locals.var_guard131 == 0.0) {
        let eq65_e1032_q: f64 = locals.var_qgdfp2;
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1036_q: f64 = eq65_e1035;
        let eq65_e1037: f64 = (locals.var_qgdfp2 + eq65_e1035);
        let eq65_e1037_d_n2: f64 = (locals.var_qgdfp2_dn2 + p.p355);
        let eq65_e1037_d_n15: f64 = (locals.var_qgdfp2_dn15 + (-p.p355));
        let eq65_e1037_q: f64 = (eq65_e1032_q + eq65_e1036_q);
        (eq65_e1037, eq65_e1037_d_n2, locals.var_qgdfp2_dn4, locals.var_qgdfp2_dn7, locals.var_qgdfp2_dn14, eq65_e1037_d_n15, eq65_e1037_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq65_e1039_d_n2, 0.0, eq65_e1039_d_n4, 0.0, 0.0, eq65_e1039_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq65_e1039_d_n14, eq65_e1039_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq65_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq65_reactive_node_derivatives,
            branches,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n7, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_q,) = {
    if (locals.var_guard131 == 0.0) {
        let eq66_e1043_q: f64 = locals.var_qcfp2;
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1047_q: f64 = eq66_e1046;
        let eq66_e1048: f64 = (locals.var_qcfp2 + eq66_e1046);
        let eq66_e1048_d_n7: f64 = (locals.var_qcfp2_dn7 + p.p355);
        let eq66_e1048_d_n14: f64 = (locals.var_qcfp2_dn14 + (-p.p355));
        let eq66_e1048_q: f64 = (eq66_e1043_q + eq66_e1047_q);
        (eq66_e1048, locals.var_qcfp2_dn2, locals.var_qcfp2_dn3, locals.var_qcfp2_dn4, eq66_e1048_d_n7, eq66_e1048_d_n14, locals.var_qcfp2_dn15, eq66_e1048_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, 0.0, 0.0, eq66_e1050_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq66_e1050_d_n14, eq66_e1050_d_n15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq66_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
        let eq69_e1062_q: f64 = locals.var_qbfp2;
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1066_q: f64 = eq69_e1065;
        let eq69_e1067: f64 = (locals.var_qbfp2 + eq69_e1065);
        let eq69_e1067_d_n3: f64 = (locals.var_qbfp2_dn3 + p.p355);
        let eq69_e1067_d_n14: f64 = (locals.var_qbfp2_dn14 + (-p.p355));
        let eq69_e1067_q: f64 = (eq69_e1062_q + eq69_e1066_q);
        let eq69_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfp2_dn2, eq69_e1067_d_n3, locals.var_qbfp2_dn4, 0.0, 0.0, locals.var_qbfp2_dn7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq69_e1067_d_n14, locals.var_qbfp2_dn15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq69_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1090, eq72_e1090_d_n2, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n7, eq72_e1090_d_n14, eq72_e1090_q,) = {
    if (locals.var_guard167 != 0.0) {
        let eq72_e1083_q: f64 = locals.var_qgsfp1;
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1087_q: f64 = eq72_e1086;
        let eq72_e1088: f64 = (locals.var_qgsfp1 + eq72_e1086);
        let eq72_e1088_d_n5: f64 = (locals.var_qgsfp1_dn5 + (-p.p355));
        let eq72_e1088_d_n7: f64 = (locals.var_qgsfp1_dn7 + p.p355);
        let eq72_e1088_q: f64 = (eq72_e1083_q + eq72_e1087_q);
        (eq72_e1088, locals.var_qgsfp1_dn2, locals.var_qgsfp1_dn4, eq72_e1088_d_n5, eq72_e1088_d_n7, locals.var_qgsfp1_dn14, eq72_e1088_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq72_e1090_d_n2, 0.0, eq72_e1090_d_n4, eq72_e1090_d_n5, 0.0, eq72_e1090_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq72_e1090_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq72_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n2, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n7, eq73_e1100_d_n14, eq73_e1100_q,) = {
    if (locals.var_guard167 != 0.0) {
        let eq73_e1093_q: f64 = locals.var_qgdfp1;
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1097_q: f64 = eq73_e1096;
        let eq73_e1098: f64 = (locals.var_qgdfp1 + eq73_e1096);
        let eq73_e1098_d_n7: f64 = (locals.var_qgdfp1_dn7 + p.p355);
        let eq73_e1098_d_n14: f64 = (locals.var_qgdfp1_dn14 + (-p.p355));
        let eq73_e1098_q: f64 = (eq73_e1093_q + eq73_e1097_q);
        (eq73_e1098, locals.var_qgdfp1_dn2, locals.var_qgdfp1_dn4, locals.var_qgdfp1_dn5, eq73_e1098_d_n7, eq73_e1098_d_n14, eq73_e1098_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq73_e1100_d_n2, 0.0, eq73_e1100_d_n4, eq73_e1100_d_n5, 0.0, eq73_e1100_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq73_e1100_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq73_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1110, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n7, eq74_e1110_d_n14, eq74_e1110_q,) = {
    if (locals.var_guard167 != 0.0) {
        let eq74_e1103_q: f64 = locals.var_qcfp1;
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1107_q: f64 = eq74_e1106;
        let eq74_e1108: f64 = (locals.var_qcfp1 + eq74_e1106);
        let eq74_e1108_d_n2: f64 = (locals.var_qcfp1_dn2 + p.p355);
        let eq74_e1108_d_n5: f64 = (locals.var_qcfp1_dn5 + (-p.p355));
        let eq74_e1108_q: f64 = (eq74_e1103_q + eq74_e1107_q);
        (eq74_e1108, eq74_e1108_d_n2, locals.var_qcfp1_dn3, locals.var_qcfp1_dn4, eq74_e1108_d_n5, locals.var_qcfp1_dn7, locals.var_qcfp1_dn14, eq74_e1108_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, 0.0, eq74_e1110_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq74_e1110_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq74_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1124, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n7, eq76_e1124_d_n9, eq76_e1124_d_n14, eq76_e1124_q,) = {
    if (locals.var_guard167 != 0.0) {
        let eq76_e1117_q: f64 = locals.var_qsfp1;
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1121_q: f64 = eq76_e1120;
        let eq76_e1122: f64 = (locals.var_qsfp1 + eq76_e1120);
        let eq76_e1122_d_n7: f64 = (locals.var_qsfp1_dn7 + p.p355);
        let eq76_e1122_q: f64 = (eq76_e1117_q + eq76_e1121_q);
        (eq76_e1122, locals.var_qsfp1_dn2, locals.var_qsfp1_dn3, locals.var_qsfp1_dn4, locals.var_qsfp1_dn5, eq76_e1122_d_n7, (-p.p355), locals.var_qsfp1_dn14, eq76_e1122_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, 0.0, eq76_e1124_d_n7, 0.0, eq76_e1124_d_n9, 0.0, 0.0, 0.0, 0.0, eq76_e1124_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq76_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n2, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n7, eq77_e1135_d_n14, eq77_e1135_q,) = {
    if (locals.var_guard167 == 0.0) {
        let eq77_e1128_q: f64 = locals.var_qgsfp1;
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1132_q: f64 = eq77_e1131;
        let eq77_e1133: f64 = (locals.var_qgsfp1 + eq77_e1131);
        let eq77_e1133_d_n2: f64 = (locals.var_qgsfp1_dn2 + p.p355);
        let eq77_e1133_d_n5: f64 = (locals.var_qgsfp1_dn5 + (-p.p355));
        let eq77_e1133_q: f64 = (eq77_e1128_q + eq77_e1132_q);
        (eq77_e1133, eq77_e1133_d_n2, locals.var_qgsfp1_dn4, eq77_e1133_d_n5, locals.var_qgsfp1_dn7, locals.var_qgsfp1_dn14, eq77_e1133_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq77_e1135_d_n2, 0.0, eq77_e1135_d_n4, eq77_e1135_d_n5, 0.0, eq77_e1135_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq77_e1135_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq77_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n2, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n7, eq78_e1146_d_n14, eq78_e1146_q,) = {
    if (locals.var_guard167 == 0.0) {
        let eq78_e1139_q: f64 = locals.var_qgdfp1;
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1143_q: f64 = eq78_e1142;
        let eq78_e1144: f64 = (locals.var_qgdfp1 + eq78_e1142);
        let eq78_e1144_d_n2: f64 = (locals.var_qgdfp1_dn2 + p.p355);
        let eq78_e1144_d_n14: f64 = (locals.var_qgdfp1_dn14 + (-p.p355));
        let eq78_e1144_q: f64 = (eq78_e1139_q + eq78_e1143_q);
        (eq78_e1144, eq78_e1144_d_n2, locals.var_qgdfp1_dn4, locals.var_qgdfp1_dn5, locals.var_qgdfp1_dn7, eq78_e1144_d_n14, eq78_e1144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq78_e1146_d_n2, 0.0, eq78_e1146_d_n4, eq78_e1146_d_n5, 0.0, eq78_e1146_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq78_e1146_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq78_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq78_reactive_node_derivatives,
            branches,
            &eq78_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n7, eq79_e1157_d_n14, eq79_e1157_q,) = {
    if (locals.var_guard167 == 0.0) {
        let eq79_e1150_q: f64 = locals.var_qcfp1;
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1154_q: f64 = eq79_e1153;
        let eq79_e1155: f64 = (locals.var_qcfp1 + eq79_e1153);
        let eq79_e1155_d_n5: f64 = (locals.var_qcfp1_dn5 + (-p.p355));
        let eq79_e1155_d_n7: f64 = (locals.var_qcfp1_dn7 + p.p355);
        let eq79_e1155_q: f64 = (eq79_e1150_q + eq79_e1154_q);
        (eq79_e1155, locals.var_qcfp1_dn2, locals.var_qcfp1_dn3, locals.var_qcfp1_dn4, eq79_e1155_d_n5, eq79_e1155_d_n7, locals.var_qcfp1_dn14, eq79_e1155_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, 0.0, eq79_e1157_d_n7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, eq79_e1157_d_n14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq79_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq79_reactive_node_derivatives,
            branches,
            &eq79_reactive_branch_derivatives,
            multiplicity,
        );
        let eq82_e1169_q: f64 = locals.var_qbfp1;
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1173_q: f64 = eq82_e1172;
        let eq82_e1174: f64 = (locals.var_qbfp1 + eq82_e1172);
        let eq82_e1174_d_n3: f64 = (locals.var_qbfp1_dn3 + p.p355);
        let eq82_e1174_d_n5: f64 = (locals.var_qbfp1_dn5 + (-p.p355));
        let eq82_e1174_q: f64 = (eq82_e1169_q + eq82_e1173_q);
        let eq82_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfp1_dn2, eq82_e1174_d_n3, locals.var_qbfp1_dn4, eq82_e1174_d_n5, 0.0, locals.var_qbfp1_dn7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_qbfp1_dn14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq82_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq82_reactive_node_derivatives,
            branches,
            &eq82_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq85_e1197, eq85_e1197_d_n2, eq85_e1197_d_n4, eq85_e1197_d_n7, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_q,) = {
    if (locals.var_guard203 != 0.0) {
        let eq85_e1190_q: f64 = locals.var_qgsfps1;
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1194_q: f64 = eq85_e1193;
        let eq85_e1195: f64 = (locals.var_qgsfps1 + eq85_e1193);
        let eq85_e1195_d_n7: f64 = (locals.var_qgsfps1_dn7 + p.p355);
        let eq85_e1195_d_n10: f64 = (locals.var_qgsfps1_dn10 + (-p.p355));
        let eq85_e1195_q: f64 = (eq85_e1190_q + eq85_e1194_q);
        (eq85_e1195, locals.var_qgsfps1_dn2, locals.var_qgsfps1_dn4, eq85_e1195_d_n7, locals.var_qgsfps1_dn9, eq85_e1195_d_n10, eq85_e1195_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq85_e1197_d_n2, 0.0, eq85_e1197_d_n4, 0.0, 0.0, eq85_e1197_d_n7, 0.0, eq85_e1197_d_n9, eq85_e1197_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq85_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq85_reactive_node_derivatives,
            branches,
            &eq85_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n2, eq86_e1207_d_n4, eq86_e1207_d_n7, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_q,) = {
    if (locals.var_guard203 != 0.0) {
        let eq86_e1200_q: f64 = locals.var_qgdfps1;
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1204_q: f64 = eq86_e1203;
        let eq86_e1205: f64 = (locals.var_qgdfps1 + eq86_e1203);
        let eq86_e1205_d_n7: f64 = (locals.var_qgdfps1_dn7 + p.p355);
        let eq86_e1205_d_n9: f64 = (locals.var_qgdfps1_dn9 + (-p.p355));
        let eq86_e1205_q: f64 = (eq86_e1200_q + eq86_e1204_q);
        (eq86_e1205, locals.var_qgdfps1_dn2, locals.var_qgdfps1_dn4, eq86_e1205_d_n7, eq86_e1205_d_n9, locals.var_qgdfps1_dn10, eq86_e1205_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq86_e1207_d_n2, 0.0, eq86_e1207_d_n4, 0.0, 0.0, eq86_e1207_d_n7, 0.0, eq86_e1207_d_n9, eq86_e1207_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq86_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq86_reactive_node_derivatives,
            branches,
            &eq86_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n7, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_q,) = {
    if (locals.var_guard203 != 0.0) {
        let eq87_e1210_q: f64 = locals.var_qcfps1;
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1214_q: f64 = eq87_e1213;
        let eq87_e1215: f64 = (locals.var_qcfps1 + eq87_e1213);
        let eq87_e1215_d_n2: f64 = (locals.var_qcfps1_dn2 + p.p355);
        let eq87_e1215_d_n10: f64 = (locals.var_qcfps1_dn10 + (-p.p355));
        let eq87_e1215_q: f64 = (eq87_e1210_q + eq87_e1214_q);
        (eq87_e1215, eq87_e1215_d_n2, locals.var_qcfps1_dn3, locals.var_qcfps1_dn4, locals.var_qcfps1_dn7, locals.var_qcfps1_dn9, eq87_e1215_d_n10, eq87_e1215_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, 0.0, 0.0, eq87_e1217_d_n7, 0.0, eq87_e1217_d_n9, eq87_e1217_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq87_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq87_reactive_node_derivatives,
            branches,
            &eq87_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1231, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n7, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_q,) = {
    if (locals.var_guard203 != 0.0) {
        let eq89_e1224_q: f64 = locals.var_qsfps1;
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1228_q: f64 = eq89_e1227;
        let eq89_e1229: f64 = (locals.var_qsfps1 + eq89_e1227);
        let eq89_e1229_d_n7: f64 = (locals.var_qsfps1_dn7 + p.p355);
        let eq89_e1229_d_n9: f64 = (locals.var_qsfps1_dn9 + (-p.p355));
        let eq89_e1229_q: f64 = (eq89_e1224_q + eq89_e1228_q);
        (eq89_e1229, locals.var_qsfps1_dn2, locals.var_qsfps1_dn3, locals.var_qsfps1_dn4, eq89_e1229_d_n7, eq89_e1229_d_n9, locals.var_qsfps1_dn10, eq89_e1229_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, 0.0, 0.0, eq89_e1231_d_n7, 0.0, eq89_e1231_d_n9, eq89_e1231_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq89_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq89_reactive_node_derivatives,
            branches,
            &eq89_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n2, eq90_e1242_d_n4, eq90_e1242_d_n7, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_q,) = {
    if (locals.var_guard203 == 0.0) {
        let eq90_e1235_q: f64 = locals.var_qgsfps1;
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1239_q: f64 = eq90_e1238;
        let eq90_e1240: f64 = (locals.var_qgsfps1 + eq90_e1238);
        let eq90_e1240_d_n2: f64 = (locals.var_qgsfps1_dn2 + p.p355);
        let eq90_e1240_d_n10: f64 = (locals.var_qgsfps1_dn10 + (-p.p355));
        let eq90_e1240_q: f64 = (eq90_e1235_q + eq90_e1239_q);
        (eq90_e1240, eq90_e1240_d_n2, locals.var_qgsfps1_dn4, locals.var_qgsfps1_dn7, locals.var_qgsfps1_dn9, eq90_e1240_d_n10, eq90_e1240_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq90_e1242_d_n2, 0.0, eq90_e1242_d_n4, 0.0, 0.0, eq90_e1242_d_n7, 0.0, eq90_e1242_d_n9, eq90_e1242_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq90_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq90_reactive_node_derivatives,
            branches,
            &eq90_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq91_e1253, eq91_e1253_d_n2, eq91_e1253_d_n4, eq91_e1253_d_n7, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_q,) = {
    if (locals.var_guard203 == 0.0) {
        let eq91_e1246_q: f64 = locals.var_qgdfps1;
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1250_q: f64 = eq91_e1249;
        let eq91_e1251: f64 = (locals.var_qgdfps1 + eq91_e1249);
        let eq91_e1251_d_n2: f64 = (locals.var_qgdfps1_dn2 + p.p355);
        let eq91_e1251_d_n9: f64 = (locals.var_qgdfps1_dn9 + (-p.p355));
        let eq91_e1251_q: f64 = (eq91_e1246_q + eq91_e1250_q);
        (eq91_e1251, eq91_e1251_d_n2, locals.var_qgdfps1_dn4, locals.var_qgdfps1_dn7, eq91_e1251_d_n9, locals.var_qgdfps1_dn10, eq91_e1251_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq91_e1253_d_n2, 0.0, eq91_e1253_d_n4, 0.0, 0.0, eq91_e1253_d_n7, 0.0, eq91_e1253_d_n9, eq91_e1253_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq91_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[9]),
            nodes,
            &eq91_reactive_node_derivatives,
            branches,
            &eq91_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq92_e1264, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n7, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_q,) = {
    if (locals.var_guard203 == 0.0) {
        let eq92_e1257_q: f64 = locals.var_qcfps1;
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1261_q: f64 = eq92_e1260;
        let eq92_e1262: f64 = (locals.var_qcfps1 + eq92_e1260);
        let eq92_e1262_d_n7: f64 = (locals.var_qcfps1_dn7 + p.p355);
        let eq92_e1262_d_n10: f64 = (locals.var_qcfps1_dn10 + (-p.p355));
        let eq92_e1262_q: f64 = (eq92_e1257_q + eq92_e1261_q);
        (eq92_e1262, locals.var_qcfps1_dn2, locals.var_qcfps1_dn3, locals.var_qcfps1_dn4, eq92_e1262_d_n7, locals.var_qcfps1_dn9, eq92_e1262_d_n10, eq92_e1262_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, 0.0, 0.0, eq92_e1264_d_n7, 0.0, eq92_e1264_d_n9, eq92_e1264_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq92_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq92_reactive_node_derivatives,
            branches,
            &eq92_reactive_branch_derivatives,
            multiplicity,
        );
        let eq95_e1276_q: f64 = locals.var_qbfps1;
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1280_q: f64 = eq95_e1279;
        let eq95_e1281: f64 = (locals.var_qbfps1 + eq95_e1279);
        let eq95_e1281_d_n3: f64 = (locals.var_qbfps1_dn3 + p.p355);
        let eq95_e1281_d_n10: f64 = (locals.var_qbfps1_dn10 + (-p.p355));
        let eq95_e1281_q: f64 = (eq95_e1276_q + eq95_e1280_q);
        let eq95_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfps1_dn2, eq95_e1281_d_n3, locals.var_qbfps1_dn4, 0.0, 0.0, locals.var_qbfps1_dn7, 0.0, locals.var_qbfps1_dn9, eq95_e1281_d_n10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq95_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq95_reactive_node_derivatives,
            branches,
            &eq95_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq98_e1304, eq98_e1304_d_n2, eq98_e1304_d_n4, eq98_e1304_d_n7, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_q,) = {
    if (locals.var_guard239 != 0.0) {
        let eq98_e1297_q: f64 = locals.var_qgsfps2;
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1301_q: f64 = eq98_e1300;
        let eq98_e1302: f64 = (locals.var_qgsfps2 + eq98_e1300);
        let eq98_e1302_d_n7: f64 = (locals.var_qgsfps2_dn7 + p.p355);
        let eq98_e1302_d_n11: f64 = (locals.var_qgsfps2_dn11 + (-p.p355));
        let eq98_e1302_q: f64 = (eq98_e1297_q + eq98_e1301_q);
        (eq98_e1302, locals.var_qgsfps2_dn2, locals.var_qgsfps2_dn4, eq98_e1302_d_n7, locals.var_qgsfps2_dn10, eq98_e1302_d_n11, eq98_e1302_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq98_e1304_d_n2, 0.0, eq98_e1304_d_n4, 0.0, 0.0, eq98_e1304_d_n7, 0.0, 0.0, eq98_e1304_d_n10, eq98_e1304_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq98_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq98_reactive_node_derivatives,
            branches,
            &eq98_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n2, eq99_e1314_d_n4, eq99_e1314_d_n7, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_q,) = {
    if (locals.var_guard239 != 0.0) {
        let eq99_e1307_q: f64 = locals.var_qgdfps2;
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1311_q: f64 = eq99_e1310;
        let eq99_e1312: f64 = (locals.var_qgdfps2 + eq99_e1310);
        let eq99_e1312_d_n7: f64 = (locals.var_qgdfps2_dn7 + p.p355);
        let eq99_e1312_d_n10: f64 = (locals.var_qgdfps2_dn10 + (-p.p355));
        let eq99_e1312_q: f64 = (eq99_e1307_q + eq99_e1311_q);
        (eq99_e1312, locals.var_qgdfps2_dn2, locals.var_qgdfps2_dn4, eq99_e1312_d_n7, eq99_e1312_d_n10, locals.var_qgdfps2_dn11, eq99_e1312_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq99_e1314_d_n2, 0.0, eq99_e1314_d_n4, 0.0, 0.0, eq99_e1314_d_n7, 0.0, 0.0, eq99_e1314_d_n10, eq99_e1314_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq99_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq99_reactive_node_derivatives,
            branches,
            &eq99_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq100_e1324, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n7, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_q,) = {
    if (locals.var_guard239 != 0.0) {
        let eq100_e1317_q: f64 = locals.var_qcfps2;
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1321_q: f64 = eq100_e1320;
        let eq100_e1322: f64 = (locals.var_qcfps2 + eq100_e1320);
        let eq100_e1322_d_n2: f64 = (locals.var_qcfps2_dn2 + p.p355);
        let eq100_e1322_d_n11: f64 = (locals.var_qcfps2_dn11 + (-p.p355));
        let eq100_e1322_q: f64 = (eq100_e1317_q + eq100_e1321_q);
        (eq100_e1322, eq100_e1322_d_n2, locals.var_qcfps2_dn3, locals.var_qcfps2_dn4, locals.var_qcfps2_dn7, locals.var_qcfps2_dn10, eq100_e1322_d_n11, eq100_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, 0.0, 0.0, eq100_e1324_d_n7, 0.0, 0.0, eq100_e1324_d_n10, eq100_e1324_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq100_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq100_reactive_node_derivatives,
            branches,
            &eq100_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq102_e1338, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n7, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_q,) = {
    if (locals.var_guard239 != 0.0) {
        let eq102_e1331_q: f64 = locals.var_qsfps2;
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1335_q: f64 = eq102_e1334;
        let eq102_e1336: f64 = (locals.var_qsfps2 + eq102_e1334);
        let eq102_e1336_d_n7: f64 = (locals.var_qsfps2_dn7 + p.p355);
        let eq102_e1336_q: f64 = (eq102_e1331_q + eq102_e1335_q);
        (eq102_e1336, locals.var_qsfps2_dn2, locals.var_qsfps2_dn3, locals.var_qsfps2_dn4, eq102_e1336_d_n7, (-p.p355), locals.var_qsfps2_dn10, locals.var_qsfps2_dn11, eq102_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, 0.0, 0.0, eq102_e1338_d_n7, 0.0, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq102_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq102_reactive_node_derivatives,
            branches,
            &eq102_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1349, eq103_e1349_d_n2, eq103_e1349_d_n4, eq103_e1349_d_n7, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_q,) = {
    if (locals.var_guard239 == 0.0) {
        let eq103_e1342_q: f64 = locals.var_qgsfps2;
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1346_q: f64 = eq103_e1345;
        let eq103_e1347: f64 = (locals.var_qgsfps2 + eq103_e1345);
        let eq103_e1347_d_n2: f64 = (locals.var_qgsfps2_dn2 + p.p355);
        let eq103_e1347_d_n11: f64 = (locals.var_qgsfps2_dn11 + (-p.p355));
        let eq103_e1347_q: f64 = (eq103_e1342_q + eq103_e1346_q);
        (eq103_e1347, eq103_e1347_d_n2, locals.var_qgsfps2_dn4, locals.var_qgsfps2_dn7, locals.var_qgsfps2_dn10, eq103_e1347_d_n11, eq103_e1347_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq103_e1349_d_n2, 0.0, eq103_e1349_d_n4, 0.0, 0.0, eq103_e1349_d_n7, 0.0, 0.0, eq103_e1349_d_n10, eq103_e1349_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq103_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq103_reactive_node_derivatives,
            branches,
            &eq103_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1360, eq104_e1360_d_n2, eq104_e1360_d_n4, eq104_e1360_d_n7, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_q,) = {
    if (locals.var_guard239 == 0.0) {
        let eq104_e1353_q: f64 = locals.var_qgdfps2;
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1357_q: f64 = eq104_e1356;
        let eq104_e1358: f64 = (locals.var_qgdfps2 + eq104_e1356);
        let eq104_e1358_d_n2: f64 = (locals.var_qgdfps2_dn2 + p.p355);
        let eq104_e1358_d_n10: f64 = (locals.var_qgdfps2_dn10 + (-p.p355));
        let eq104_e1358_q: f64 = (eq104_e1353_q + eq104_e1357_q);
        (eq104_e1358, eq104_e1358_d_n2, locals.var_qgdfps2_dn4, locals.var_qgdfps2_dn7, eq104_e1358_d_n10, locals.var_qgdfps2_dn11, eq104_e1358_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq104_e1360_d_n2, 0.0, eq104_e1360_d_n4, 0.0, 0.0, eq104_e1360_d_n7, 0.0, 0.0, eq104_e1360_d_n10, eq104_e1360_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq104_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq104_reactive_node_derivatives,
            branches,
            &eq104_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n7, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_q,) = {
    if (locals.var_guard239 == 0.0) {
        let eq105_e1364_q: f64 = locals.var_qcfps2;
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1368_q: f64 = eq105_e1367;
        let eq105_e1369: f64 = (locals.var_qcfps2 + eq105_e1367);
        let eq105_e1369_d_n7: f64 = (locals.var_qcfps2_dn7 + p.p355);
        let eq105_e1369_d_n11: f64 = (locals.var_qcfps2_dn11 + (-p.p355));
        let eq105_e1369_q: f64 = (eq105_e1364_q + eq105_e1368_q);
        (eq105_e1369, locals.var_qcfps2_dn2, locals.var_qcfps2_dn3, locals.var_qcfps2_dn4, eq105_e1369_d_n7, locals.var_qcfps2_dn10, eq105_e1369_d_n11, eq105_e1369_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, 0.0, 0.0, eq105_e1371_d_n7, 0.0, 0.0, eq105_e1371_d_n10, eq105_e1371_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq105_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq105_reactive_node_derivatives,
            branches,
            &eq105_reactive_branch_derivatives,
            multiplicity,
        );
        let eq108_e1383_q: f64 = locals.var_qbfps2;
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1387_q: f64 = eq108_e1386;
        let eq108_e1388: f64 = (locals.var_qbfps2 + eq108_e1386);
        let eq108_e1388_d_n3: f64 = (locals.var_qbfps2_dn3 + p.p355);
        let eq108_e1388_d_n11: f64 = (locals.var_qbfps2_dn11 + (-p.p355));
        let eq108_e1388_q: f64 = (eq108_e1383_q + eq108_e1387_q);
        let eq108_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfps2_dn2, eq108_e1388_d_n3, locals.var_qbfps2_dn4, 0.0, 0.0, locals.var_qbfps2_dn7, 0.0, 0.0, locals.var_qbfps2_dn10, eq108_e1388_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq108_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            nodes,
            &eq108_reactive_node_derivatives,
            branches,
            &eq108_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1411, eq111_e1411_d_n2, eq111_e1411_d_n4, eq111_e1411_d_n7, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_q,) = {
    if (locals.var_guard275 != 0.0) {
        let eq111_e1404_q: f64 = locals.var_qgsfps3;
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1408_q: f64 = eq111_e1407;
        let eq111_e1409: f64 = (locals.var_qgsfps3 + eq111_e1407);
        let eq111_e1409_d_n7: f64 = (locals.var_qgsfps3_dn7 + p.p355);
        let eq111_e1409_d_n12: f64 = (locals.var_qgsfps3_dn12 + (-p.p355));
        let eq111_e1409_q: f64 = (eq111_e1404_q + eq111_e1408_q);
        (eq111_e1409, locals.var_qgsfps3_dn2, locals.var_qgsfps3_dn4, eq111_e1409_d_n7, locals.var_qgsfps3_dn11, eq111_e1409_d_n12, eq111_e1409_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq111_e1411_d_n2, 0.0, eq111_e1411_d_n4, 0.0, 0.0, eq111_e1411_d_n7, 0.0, 0.0, 0.0, eq111_e1411_d_n11, eq111_e1411_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq111_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n2, eq112_e1421_d_n4, eq112_e1421_d_n7, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_q,) = {
    if (locals.var_guard275 != 0.0) {
        let eq112_e1414_q: f64 = locals.var_qgdfps3;
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1418_q: f64 = eq112_e1417;
        let eq112_e1419: f64 = (locals.var_qgdfps3 + eq112_e1417);
        let eq112_e1419_d_n7: f64 = (locals.var_qgdfps3_dn7 + p.p355);
        let eq112_e1419_d_n11: f64 = (locals.var_qgdfps3_dn11 + (-p.p355));
        let eq112_e1419_q: f64 = (eq112_e1414_q + eq112_e1418_q);
        (eq112_e1419, locals.var_qgdfps3_dn2, locals.var_qgdfps3_dn4, eq112_e1419_d_n7, eq112_e1419_d_n11, locals.var_qgdfps3_dn12, eq112_e1419_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq112_e1421_d_n2, 0.0, eq112_e1421_d_n4, 0.0, 0.0, eq112_e1421_d_n7, 0.0, 0.0, 0.0, eq112_e1421_d_n11, eq112_e1421_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq112_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq112_reactive_node_derivatives,
            branches,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n7, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_q,) = {
    if (locals.var_guard275 != 0.0) {
        let eq113_e1424_q: f64 = locals.var_qcfps3;
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1428_q: f64 = eq113_e1427;
        let eq113_e1429: f64 = (locals.var_qcfps3 + eq113_e1427);
        let eq113_e1429_d_n2: f64 = (locals.var_qcfps3_dn2 + p.p355);
        let eq113_e1429_d_n12: f64 = (locals.var_qcfps3_dn12 + (-p.p355));
        let eq113_e1429_q: f64 = (eq113_e1424_q + eq113_e1428_q);
        (eq113_e1429, eq113_e1429_d_n2, locals.var_qcfps3_dn3, locals.var_qcfps3_dn4, locals.var_qcfps3_dn7, locals.var_qcfps3_dn11, eq113_e1429_d_n12, eq113_e1429_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, 0.0, 0.0, eq113_e1431_d_n7, 0.0, 0.0, 0.0, eq113_e1431_d_n11, eq113_e1431_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq113_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq113_reactive_node_derivatives,
            branches,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq115_e1445, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n7, eq115_e1445_d_n9, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_q,) = {
    if (locals.var_guard275 != 0.0) {
        let eq115_e1438_q: f64 = locals.var_qsfps3;
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1442_q: f64 = eq115_e1441;
        let eq115_e1443: f64 = (locals.var_qsfps3 + eq115_e1441);
        let eq115_e1443_d_n7: f64 = (locals.var_qsfps3_dn7 + p.p355);
        let eq115_e1443_q: f64 = (eq115_e1438_q + eq115_e1442_q);
        (eq115_e1443, locals.var_qsfps3_dn2, locals.var_qsfps3_dn3, locals.var_qsfps3_dn4, eq115_e1443_d_n7, (-p.p355), locals.var_qsfps3_dn11, locals.var_qsfps3_dn12, eq115_e1443_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, 0.0, 0.0, eq115_e1445_d_n7, 0.0, eq115_e1445_d_n9, 0.0, eq115_e1445_d_n11, eq115_e1445_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq115_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq115_reactive_node_derivatives,
            branches,
            &eq115_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq116_e1456, eq116_e1456_d_n2, eq116_e1456_d_n4, eq116_e1456_d_n7, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_q,) = {
    if (locals.var_guard275 == 0.0) {
        let eq116_e1449_q: f64 = locals.var_qgsfps3;
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1453_q: f64 = eq116_e1452;
        let eq116_e1454: f64 = (locals.var_qgsfps3 + eq116_e1452);
        let eq116_e1454_d_n2: f64 = (locals.var_qgsfps3_dn2 + p.p355);
        let eq116_e1454_d_n12: f64 = (locals.var_qgsfps3_dn12 + (-p.p355));
        let eq116_e1454_q: f64 = (eq116_e1449_q + eq116_e1453_q);
        (eq116_e1454, eq116_e1454_d_n2, locals.var_qgsfps3_dn4, locals.var_qgsfps3_dn7, locals.var_qgsfps3_dn11, eq116_e1454_d_n12, eq116_e1454_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq116_e1456_d_n2, 0.0, eq116_e1456_d_n4, 0.0, 0.0, eq116_e1456_d_n7, 0.0, 0.0, 0.0, eq116_e1456_d_n11, eq116_e1456_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq116_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq116_reactive_node_derivatives,
            branches,
            &eq116_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n2, eq117_e1467_d_n4, eq117_e1467_d_n7, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_q,) = {
    if (locals.var_guard275 == 0.0) {
        let eq117_e1460_q: f64 = locals.var_qgdfps3;
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1464_q: f64 = eq117_e1463;
        let eq117_e1465: f64 = (locals.var_qgdfps3 + eq117_e1463);
        let eq117_e1465_d_n2: f64 = (locals.var_qgdfps3_dn2 + p.p355);
        let eq117_e1465_d_n11: f64 = (locals.var_qgdfps3_dn11 + (-p.p355));
        let eq117_e1465_q: f64 = (eq117_e1460_q + eq117_e1464_q);
        (eq117_e1465, eq117_e1465_d_n2, locals.var_qgdfps3_dn4, locals.var_qgdfps3_dn7, eq117_e1465_d_n11, locals.var_qgdfps3_dn12, eq117_e1465_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq117_e1467_d_n2, 0.0, eq117_e1467_d_n4, 0.0, 0.0, eq117_e1467_d_n7, 0.0, 0.0, 0.0, eq117_e1467_d_n11, eq117_e1467_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq117_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq117_reactive_node_derivatives,
            branches,
            &eq117_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n7, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_q,) = {
    if (locals.var_guard275 == 0.0) {
        let eq118_e1471_q: f64 = locals.var_qcfps3;
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1475_q: f64 = eq118_e1474;
        let eq118_e1476: f64 = (locals.var_qcfps3 + eq118_e1474);
        let eq118_e1476_d_n7: f64 = (locals.var_qcfps3_dn7 + p.p355);
        let eq118_e1476_d_n12: f64 = (locals.var_qcfps3_dn12 + (-p.p355));
        let eq118_e1476_q: f64 = (eq118_e1471_q + eq118_e1475_q);
        (eq118_e1476, locals.var_qcfps3_dn2, locals.var_qcfps3_dn3, locals.var_qcfps3_dn4, eq118_e1476_d_n7, locals.var_qcfps3_dn11, eq118_e1476_d_n12, eq118_e1476_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, 0.0, 0.0, eq118_e1478_d_n7, 0.0, 0.0, 0.0, eq118_e1478_d_n11, eq118_e1478_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq118_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq118_reactive_node_derivatives,
            branches,
            &eq118_reactive_branch_derivatives,
            multiplicity,
        );
        let eq121_e1490_q: f64 = locals.var_qbfps3;
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1494_q: f64 = eq121_e1493;
        let eq121_e1495: f64 = (locals.var_qbfps3 + eq121_e1493);
        let eq121_e1495_d_n3: f64 = (locals.var_qbfps3_dn3 + p.p355);
        let eq121_e1495_d_n12: f64 = (locals.var_qbfps3_dn12 + (-p.p355));
        let eq121_e1495_q: f64 = (eq121_e1490_q + eq121_e1494_q);
        let eq121_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfps3_dn2, eq121_e1495_d_n3, locals.var_qbfps3_dn4, 0.0, 0.0, locals.var_qbfps3_dn7, 0.0, 0.0, 0.0, locals.var_qbfps3_dn11, eq121_e1495_d_n12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq121_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1518, eq124_e1518_d_n2, eq124_e1518_d_n4, eq124_e1518_d_n7, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_q,) = {
    if (locals.var_guard311 != 0.0) {
        let eq124_e1511_q: f64 = locals.var_qgsfps4;
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1515_q: f64 = eq124_e1514;
        let eq124_e1516: f64 = (locals.var_qgsfps4 + eq124_e1514);
        let eq124_e1516_d_n7: f64 = (locals.var_qgsfps4_dn7 + p.p355);
        let eq124_e1516_d_n13: f64 = (locals.var_qgsfps4_dn13 + (-p.p355));
        let eq124_e1516_q: f64 = (eq124_e1511_q + eq124_e1515_q);
        (eq124_e1516, locals.var_qgsfps4_dn2, locals.var_qgsfps4_dn4, eq124_e1516_d_n7, locals.var_qgsfps4_dn12, eq124_e1516_d_n13, eq124_e1516_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq124_e1518_d_n2, 0.0, eq124_e1518_d_n4, 0.0, 0.0, eq124_e1518_d_n7, 0.0, 0.0, 0.0, 0.0, eq124_e1518_d_n12, eq124_e1518_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq124_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n2, eq125_e1528_d_n4, eq125_e1528_d_n7, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_q,) = {
    if (locals.var_guard311 != 0.0) {
        let eq125_e1521_q: f64 = locals.var_qgdfps4;
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1525_q: f64 = eq125_e1524;
        let eq125_e1526: f64 = (locals.var_qgdfps4 + eq125_e1524);
        let eq125_e1526_d_n7: f64 = (locals.var_qgdfps4_dn7 + p.p355);
        let eq125_e1526_d_n12: f64 = (locals.var_qgdfps4_dn12 + (-p.p355));
        let eq125_e1526_q: f64 = (eq125_e1521_q + eq125_e1525_q);
        (eq125_e1526, locals.var_qgdfps4_dn2, locals.var_qgdfps4_dn4, eq125_e1526_d_n7, eq125_e1526_d_n12, locals.var_qgdfps4_dn13, eq125_e1526_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq125_e1528_d_n2, 0.0, eq125_e1528_d_n4, 0.0, 0.0, eq125_e1528_d_n7, 0.0, 0.0, 0.0, 0.0, eq125_e1528_d_n12, eq125_e1528_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq125_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq126_e1538, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n7, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_q,) = {
    if (locals.var_guard311 != 0.0) {
        let eq126_e1531_q: f64 = locals.var_qcfps4;
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1535_q: f64 = eq126_e1534;
        let eq126_e1536: f64 = (locals.var_qcfps4 + eq126_e1534);
        let eq126_e1536_d_n2: f64 = (locals.var_qcfps4_dn2 + p.p355);
        let eq126_e1536_d_n13: f64 = (locals.var_qcfps4_dn13 + (-p.p355));
        let eq126_e1536_q: f64 = (eq126_e1531_q + eq126_e1535_q);
        (eq126_e1536, eq126_e1536_d_n2, locals.var_qcfps4_dn3, locals.var_qcfps4_dn4, locals.var_qcfps4_dn7, locals.var_qcfps4_dn12, eq126_e1536_d_n13, eq126_e1536_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, 0.0, 0.0, eq126_e1538_d_n7, 0.0, 0.0, 0.0, 0.0, eq126_e1538_d_n12, eq126_e1538_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq126_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1552, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n7, eq128_e1552_d_n9, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_q,) = {
    if (locals.var_guard311 != 0.0) {
        let eq128_e1545_q: f64 = locals.var_qsfps4;
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1549_q: f64 = eq128_e1548;
        let eq128_e1550: f64 = (locals.var_qsfps4 + eq128_e1548);
        let eq128_e1550_d_n7: f64 = (locals.var_qsfps4_dn7 + p.p355);
        let eq128_e1550_q: f64 = (eq128_e1545_q + eq128_e1549_q);
        (eq128_e1550, locals.var_qsfps4_dn2, locals.var_qsfps4_dn3, locals.var_qsfps4_dn4, eq128_e1550_d_n7, (-p.p355), locals.var_qsfps4_dn12, locals.var_qsfps4_dn13, eq128_e1550_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, 0.0, 0.0, eq128_e1552_d_n7, 0.0, eq128_e1552_d_n9, 0.0, 0.0, eq128_e1552_d_n12, eq128_e1552_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq128_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1563, eq129_e1563_d_n2, eq129_e1563_d_n4, eq129_e1563_d_n7, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_q,) = {
    if (locals.var_guard311 == 0.0) {
        let eq129_e1556_q: f64 = locals.var_qgsfps4;
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1560_q: f64 = eq129_e1559;
        let eq129_e1561: f64 = (locals.var_qgsfps4 + eq129_e1559);
        let eq129_e1561_d_n2: f64 = (locals.var_qgsfps4_dn2 + p.p355);
        let eq129_e1561_d_n13: f64 = (locals.var_qgsfps4_dn13 + (-p.p355));
        let eq129_e1561_q: f64 = (eq129_e1556_q + eq129_e1560_q);
        (eq129_e1561, eq129_e1561_d_n2, locals.var_qgsfps4_dn4, locals.var_qgsfps4_dn7, locals.var_qgsfps4_dn12, eq129_e1561_d_n13, eq129_e1561_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq129_e1563_d_n2, 0.0, eq129_e1563_d_n4, 0.0, 0.0, eq129_e1563_d_n7, 0.0, 0.0, 0.0, 0.0, eq129_e1563_d_n12, eq129_e1563_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq129_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n2, eq130_e1574_d_n4, eq130_e1574_d_n7, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_q,) = {
    if (locals.var_guard311 == 0.0) {
        let eq130_e1567_q: f64 = locals.var_qgdfps4;
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1571_q: f64 = eq130_e1570;
        let eq130_e1572: f64 = (locals.var_qgdfps4 + eq130_e1570);
        let eq130_e1572_d_n2: f64 = (locals.var_qgdfps4_dn2 + p.p355);
        let eq130_e1572_d_n12: f64 = (locals.var_qgdfps4_dn12 + (-p.p355));
        let eq130_e1572_q: f64 = (eq130_e1567_q + eq130_e1571_q);
        (eq130_e1572, eq130_e1572_d_n2, locals.var_qgdfps4_dn4, locals.var_qgdfps4_dn7, eq130_e1572_d_n12, locals.var_qgdfps4_dn13, eq130_e1572_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq130_e1574_d_n2, 0.0, eq130_e1574_d_n4, 0.0, 0.0, eq130_e1574_d_n7, 0.0, 0.0, 0.0, 0.0, eq130_e1574_d_n12, eq130_e1574_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq130_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1585, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n7, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_q,) = {
    if (locals.var_guard311 == 0.0) {
        let eq131_e1578_q: f64 = locals.var_qcfps4;
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1582_q: f64 = eq131_e1581;
        let eq131_e1583: f64 = (locals.var_qcfps4 + eq131_e1581);
        let eq131_e1583_d_n7: f64 = (locals.var_qcfps4_dn7 + p.p355);
        let eq131_e1583_d_n13: f64 = (locals.var_qcfps4_dn13 + (-p.p355));
        let eq131_e1583_q: f64 = (eq131_e1578_q + eq131_e1582_q);
        (eq131_e1583, locals.var_qcfps4_dn2, locals.var_qcfps4_dn3, locals.var_qcfps4_dn4, eq131_e1583_d_n7, locals.var_qcfps4_dn12, eq131_e1583_d_n13, eq131_e1583_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, 0.0, 0.0, eq131_e1585_d_n7, 0.0, 0.0, 0.0, 0.0, eq131_e1585_d_n12, eq131_e1585_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq131_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let eq134_e1597_q: f64 = locals.var_qbfps4;
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1601_q: f64 = eq134_e1600;
        let eq134_e1602: f64 = (locals.var_qbfps4 + eq134_e1600);
        let eq134_e1602_d_n3: f64 = (locals.var_qbfps4_dn3 + p.p355);
        let eq134_e1602_d_n13: f64 = (locals.var_qbfps4_dn13 + (-p.p355));
        let eq134_e1602_q: f64 = (eq134_e1597_q + eq134_e1601_q);
        let eq134_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, locals.var_qbfps4_dn2, eq134_e1602_d_n3, locals.var_qbfps4_dn4, 0.0, 0.0, locals.var_qbfps4_dn7, 0.0, 0.0, 0.0, 0.0, locals.var_qbfps4_dn12, eq134_e1602_d_n13, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq134_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_q,) = {
    if (locals.var_guard416 == 0.0) {
        let eq142_e1649: f64 = (locals.var_ids - (nv29 - 0.0));
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1653_q: f64 = eq142_e1652;
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1652);
        let eq142_e1654_q: f64 = (-eq142_e1653_q);
        (eq142_e1654, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn22, locals.var_ids_dn23, locals.var_ids_dn25, locals.var_ids_dn26, (-p.p323), (-1.0), eq142_e1654_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[28]),
            None,
            nodes[28],
            multiplicity * (eq142_e1656_d_n28),
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29, eq143_e1670_q, eq143_e1670_q_d_n29,) = {
    if (locals.var_guard416 == 0.0) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667_q: f64 = eq143_e1666;
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1666);
        let eq143_e1668_d_n29: f64 = ((-1.0) - eq143_e1664);
        let eq143_e1668_q: f64 = (-eq143_e1667_q);
        (eq143_e1668, 1.0, eq143_e1668_d_n29, eq143_e1668_q, (-eq143_e1664),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[29]),
            None,
            nodes[29],
            multiplicity * (eq143_e1670_q_d_n29),
        );
        let eq145_e1681_q: f64 = locals.var_qgs;
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1685_q: f64 = eq145_e1684;
        let eq145_e1686: f64 = (locals.var_qgs + eq145_e1684);
        let eq145_e1686_d_n8: f64 = (locals.var_qgs_dn8 + p.p355);
        let eq145_e1686_d_n9: f64 = (locals.var_qgs_dn9 + (-p.p355));
        let eq145_e1686_q: f64 = (eq145_e1681_q + eq145_e1685_q);
        let eq145_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, 0.0, 0.0, locals.var_qgs_dn4, locals.var_qgs_dn5, 0.0, 0.0, eq145_e1686_d_n8, eq145_e1686_d_n9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_qgs_dn22, locals.var_qgs_dn23, 0.0, locals.var_qgs_dn25, locals.var_qgs_dn26, 0.0, 0.0, 0.0];
        let eq145_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
        let eq146_e1688_q: f64 = locals.var_qgd;
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1692_q: f64 = eq146_e1691;
        let eq146_e1693: f64 = (locals.var_qgd + eq146_e1691);
        let eq146_e1693_d_n5: f64 = (locals.var_qgd_dn5 + (-p.p355));
        let eq146_e1693_d_n8: f64 = (locals.var_qgd_dn8 + p.p355);
        let eq146_e1693_q: f64 = (eq146_e1688_q + eq146_e1692_q);
        let eq146_reactive_node_derivatives: [f64; 30] = [0.0, 0.0, 0.0, 0.0, locals.var_qgd_dn4, eq146_e1693_d_n5, 0.0, 0.0, eq146_e1693_d_n8, locals.var_qgd_dn9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_qgd_dn22, locals.var_qgd_dn23, 0.0, locals.var_qgd_dn25, locals.var_qgd_dn26, 0.0, 0.0, 0.0];
        let eq146_reactive_branch_derivatives: [f64; 36] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_q,) = {
    if (locals.var_guard461 != 0.0) {
        let eq157_e1794_q: f64 = locals.var_qsch;
        (locals.var_qsch, locals.var_qsch_dn7, locals.var_qsch_dn8, eq157_e1794_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes[7],
            multiplicity * (eq157_e1796_d_n7),
            nodes[8],
            multiplicity * (eq157_e1796_d_n8),
        );
        let (eq194_e2167, eq194_e2167_d_n4, eq194_e2167_q,) = {
    if (locals.var_guard523 != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2165_q: f64 = eq194_e2164;
        (eq194_e2164, p.p321, eq194_e2165_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq194_e2167_d_n4),
        );
    }
}
