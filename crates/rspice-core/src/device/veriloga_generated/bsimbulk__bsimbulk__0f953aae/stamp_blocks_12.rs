#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        var_devsign: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let __rspice_deriv_cse_0: f64 = ((-p.p28) * s.v[781]);
        let __rspice_deriv_cse_1: f64 = (p.p28 * s.v[781]);
        let __rspice_deriv_cse_2: f64 = ((p.p29 * s.dn[334][0]) * ddt_scale);
        let __rspice_deriv_cse_3: f64 = ((p.p29 * s.dn[334][1]) * ddt_scale);
        let __rspice_deriv_cse_4: f64 = ((p.p29 * s.dn[334][2]) * ddt_scale);
        let __rspice_deriv_cse_5: f64 = ((p.p29 * s.dn[334][3]) * ddt_scale);
        let __rspice_deriv_cse_6: f64 = ((p.p29 * s.dn[334][4]) * ddt_scale);
        let __rspice_deriv_cse_7: f64 = ((p.p29 * s.dn[334][5]) * ddt_scale);
        let __rspice_deriv_cse_8: f64 = ((p.p29 * s.dn[334][6]) * ddt_scale);
        let __rspice_deriv_cse_9: f64 = ((p.p29 * s.dn[334][7]) * ddt_scale);
        let __rspice_deriv_cse_10: f64 = ((p.p29 * s.dn[334][8]) * ddt_scale);
        let __rspice_deriv_cse_11: f64 = ((p.p29 * s.dn[334][9]) * ddt_scale);
        let __rspice_deriv_cse_12: f64 = ((p.p29 * s.dn[334][10]) * ddt_scale);
        let __rspice_deriv_cse_13: f64 = ((p.p29 * s.dn[334][11]) * ddt_scale);
        let __rspice_deriv_cse_14: f64 = ((p.p29 * s.dn[334][12]) * ddt_scale);
        let __rspice_deriv_cse_15: f64 = ((p.p29 * s.dn[334][13]) * ddt_scale);
        let __rspice_deriv_cse_16: f64 = ((p.p29 * s.dn[334][14]) * ddt_scale);
        let __rspice_deriv_cse_17: f64 = ((p.p29 * s.dn[334][15]) * ddt_scale);
        let __rspice_deriv_cse_18: f64 = ((p.p29 * s.dn[334][16]) * ddt_scale);
        let __rspice_deriv_cse_19: f64 = ((p.p29 * s.db[334][0]) * ddt_scale);
        let __rspice_deriv_cse_20: f64 = ((p.p29 * s.db[334][1]) * ddt_scale);
        let __rspice_deriv_cse_21: f64 = ((p.p29 * s.db[334][2]) * ddt_scale);
        let __rspice_deriv_cse_22: f64 = ((p.p29 * s.db[334][3]) * ddt_scale);
        let __rspice_deriv_cse_23: f64 = ((p.p29 * s.db[334][4]) * ddt_scale);
        let __rspice_deriv_cse_24: f64 = ((p.p29 * s.db[334][5]) * ddt_scale);
        let __rspice_deriv_cse_25: f64 = ((p.p29 * s.db[334][6]) * ddt_scale);
        let __rspice_deriv_cse_26: f64 = ((p.p29 * s.db[334][7]) * ddt_scale);
        let __rspice_deriv_cse_27: f64 = ((p.p29 * s.db[334][8]) * ddt_scale);
        let __rspice_deriv_cse_28: f64 = ((p.p29 * s.db[334][9]) * ddt_scale);
        let __rspice_deriv_cse_29: f64 = ((p.p29 * s.db[334][10]) * ddt_scale);
        let __rspice_deriv_cse_30: f64 = ((p.p29 * s.db[334][11]) * ddt_scale);
        let __rspice_deriv_cse_31: f64 = ((p.p29 * s.db[334][12]) * ddt_scale);
        let __rspice_deriv_cse_32: f64 = ((p.p29 * s.db[334][13]) * ddt_scale);
        let (eq74_e1883, eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16, eq74_e1883_d_b0, eq74_e1883_d_b1, eq74_e1883_d_b2, eq74_e1883_d_b3, eq74_e1883_d_b4, eq74_e1883_d_b5, eq74_e1883_d_b6, eq74_e1883_d_b7, eq74_e1883_d_b8, eq74_e1883_d_b9, eq74_e1883_d_b10, eq74_e1883_d_b11, eq74_e1883_d_b12, eq74_e1883_d_b13,) = {
    if (!s.b[1627]) {
        let eq74_e1873: f64 = (var_devsign * p.p28);
        let eq74_e1875: f64 = (eq74_e1873 * s.v[303]);
        let eq74_e1878: f64 = ((nv11 - nv7) * p.p28);
        let eq74_e1880: f64 = (eq74_e1878 * s.v[781]);
        let eq74_e1881: f64 = (eq74_e1875 + eq74_e1880);
        let eq74_e1881_d_n7: f64 = ((eq74_e1873 * s.dn[303][7]) + __rspice_deriv_cse_0);
        let eq74_e1881_d_n11: f64 = ((eq74_e1873 * s.dn[303][11]) + __rspice_deriv_cse_1);
        (eq74_e1881, (eq74_e1873 * s.dn[303][0]), (eq74_e1873 * s.dn[303][1]), (eq74_e1873 * s.dn[303][2]), (eq74_e1873 * s.dn[303][3]), (eq74_e1873 * s.dn[303][4]), (eq74_e1873 * s.dn[303][5]), (eq74_e1873 * s.dn[303][6]), eq74_e1881_d_n7, (eq74_e1873 * s.dn[303][8]), (eq74_e1873 * s.dn[303][9]), (eq74_e1873 * s.dn[303][10]), eq74_e1881_d_n11, (eq74_e1873 * s.dn[303][12]), (eq74_e1873 * s.dn[303][13]), (eq74_e1873 * s.dn[303][14]), (eq74_e1873 * s.dn[303][15]), (eq74_e1873 * s.dn[303][16]), (eq74_e1873 * s.db[303][0]), (eq74_e1873 * s.db[303][1]), (eq74_e1873 * s.db[303][2]), (eq74_e1873 * s.db[303][3]), (eq74_e1873 * s.db[303][4]), (eq74_e1873 * s.db[303][5]), (eq74_e1873 * s.db[303][6]), (eq74_e1873 * s.db[303][7]), (eq74_e1873 * s.db[303][8]), (eq74_e1873 * s.db[303][9]), (eq74_e1873 * s.db[303][10]), (eq74_e1873 * s.db[303][11]), (eq74_e1873 * s.db[303][12]), (eq74_e1873 * s.db[303][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1883;
        let eq74_node_derivatives: [f64; 17] = [eq74_e1883_d_n0, eq74_e1883_d_n1, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14, eq74_e1883_d_n15, eq74_e1883_d_n16];
        let eq74_branch_derivatives: [f64; 14] = [eq74_e1883_d_b0, eq74_e1883_d_b1, eq74_e1883_d_b2, eq74_e1883_d_b3, eq74_e1883_d_b4, eq74_e1883_d_b5, eq74_e1883_d_b6, eq74_e1883_d_b7, eq74_e1883_d_b8, eq74_e1883_d_b9, eq74_e1883_d_b10, eq74_e1883_d_b11, eq74_e1883_d_b12, eq74_e1883_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq74_value),
            &eq74_node_derivatives,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1898, eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16, eq75_e1898_d_b0, eq75_e1898_d_b1, eq75_e1898_d_b2, eq75_e1898_d_b3, eq75_e1898_d_b4, eq75_e1898_d_b5, eq75_e1898_d_b6, eq75_e1898_d_b7, eq75_e1898_d_b8, eq75_e1898_d_b9, eq75_e1898_d_b10, eq75_e1898_d_b11, eq75_e1898_d_b12, eq75_e1898_d_b13,) = {
    if (!s.b[1627]) {
        let eq75_e1888: f64 = (var_devsign * p.p28);
        let eq75_e1890: f64 = (eq75_e1888 * s.v[304]);
        let eq75_e1893: f64 = ((nv11 - nv5) * p.p28);
        let eq75_e1895: f64 = (eq75_e1893 * s.v[781]);
        let eq75_e1896: f64 = (eq75_e1890 + eq75_e1895);
        let eq75_e1896_d_n5: f64 = ((eq75_e1888 * s.dn[304][5]) + __rspice_deriv_cse_0);
        let eq75_e1896_d_n11: f64 = ((eq75_e1888 * s.dn[304][11]) + __rspice_deriv_cse_1);
        (eq75_e1896, (eq75_e1888 * s.dn[304][0]), (eq75_e1888 * s.dn[304][1]), (eq75_e1888 * s.dn[304][2]), (eq75_e1888 * s.dn[304][3]), (eq75_e1888 * s.dn[304][4]), eq75_e1896_d_n5, (eq75_e1888 * s.dn[304][6]), (eq75_e1888 * s.dn[304][7]), (eq75_e1888 * s.dn[304][8]), (eq75_e1888 * s.dn[304][9]), (eq75_e1888 * s.dn[304][10]), eq75_e1896_d_n11, (eq75_e1888 * s.dn[304][12]), (eq75_e1888 * s.dn[304][13]), (eq75_e1888 * s.dn[304][14]), (eq75_e1888 * s.dn[304][15]), (eq75_e1888 * s.dn[304][16]), (eq75_e1888 * s.db[304][0]), (eq75_e1888 * s.db[304][1]), (eq75_e1888 * s.db[304][2]), (eq75_e1888 * s.db[304][3]), (eq75_e1888 * s.db[304][4]), (eq75_e1888 * s.db[304][5]), (eq75_e1888 * s.db[304][6]), (eq75_e1888 * s.db[304][7]), (eq75_e1888 * s.db[304][8]), (eq75_e1888 * s.db[304][9]), (eq75_e1888 * s.db[304][10]), (eq75_e1888 * s.db[304][11]), (eq75_e1888 * s.db[304][12]), (eq75_e1888 * s.db[304][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1898;
        let eq75_node_derivatives: [f64; 17] = [eq75_e1898_d_n0, eq75_e1898_d_n1, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14, eq75_e1898_d_n15, eq75_e1898_d_n16];
        let eq75_branch_derivatives: [f64; 14] = [eq75_e1898_d_b0, eq75_e1898_d_b1, eq75_e1898_d_b2, eq75_e1898_d_b3, eq75_e1898_d_b4, eq75_e1898_d_b5, eq75_e1898_d_b6, eq75_e1898_d_b7, eq75_e1898_d_b8, eq75_e1898_d_b9, eq75_e1898_d_b10, eq75_e1898_d_b11, eq75_e1898_d_b12, eq75_e1898_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq75_value),
            &eq75_node_derivatives,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13,) = {
    if (!s.b[1627]) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1905: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq76_e1904);
        let eq76_e1905_d_n0: f64 = ((p.p29 * s.dn[330][0]) * ddt_scale);
        let eq76_e1905_d_n1: f64 = ((p.p29 * s.dn[330][1]) * ddt_scale);
        let eq76_e1905_d_n2: f64 = ((p.p29 * s.dn[330][2]) * ddt_scale);
        let eq76_e1905_d_n3: f64 = ((p.p29 * s.dn[330][3]) * ddt_scale);
        let eq76_e1905_d_n4: f64 = ((p.p29 * s.dn[330][4]) * ddt_scale);
        let eq76_e1905_d_n5: f64 = ((p.p29 * s.dn[330][5]) * ddt_scale);
        let eq76_e1905_d_n6: f64 = ((p.p29 * s.dn[330][6]) * ddt_scale);
        let eq76_e1905_d_n7: f64 = ((p.p29 * s.dn[330][7]) * ddt_scale);
        let eq76_e1905_d_n8: f64 = ((p.p29 * s.dn[330][8]) * ddt_scale);
        let eq76_e1905_d_n9: f64 = ((p.p29 * s.dn[330][9]) * ddt_scale);
        let eq76_e1905_d_n10: f64 = ((p.p29 * s.dn[330][10]) * ddt_scale);
        let eq76_e1905_d_n11: f64 = ((p.p29 * s.dn[330][11]) * ddt_scale);
        let eq76_e1905_d_n12: f64 = ((p.p29 * s.dn[330][12]) * ddt_scale);
        let eq76_e1905_d_n13: f64 = ((p.p29 * s.dn[330][13]) * ddt_scale);
        let eq76_e1905_d_n14: f64 = ((p.p29 * s.dn[330][14]) * ddt_scale);
        let eq76_e1905_d_n15: f64 = ((p.p29 * s.dn[330][15]) * ddt_scale);
        let eq76_e1905_d_n16: f64 = ((p.p29 * s.dn[330][16]) * ddt_scale);
        let eq76_e1905_d_b0: f64 = ((p.p29 * s.db[330][0]) * ddt_scale);
        let eq76_e1905_d_b1: f64 = ((p.p29 * s.db[330][1]) * ddt_scale);
        let eq76_e1905_d_b2: f64 = ((p.p29 * s.db[330][2]) * ddt_scale);
        let eq76_e1905_d_b3: f64 = ((p.p29 * s.db[330][3]) * ddt_scale);
        let eq76_e1905_d_b4: f64 = ((p.p29 * s.db[330][4]) * ddt_scale);
        let eq76_e1905_d_b5: f64 = ((p.p29 * s.db[330][5]) * ddt_scale);
        let eq76_e1905_d_b6: f64 = ((p.p29 * s.db[330][6]) * ddt_scale);
        let eq76_e1905_d_b7: f64 = ((p.p29 * s.db[330][7]) * ddt_scale);
        let eq76_e1905_d_b8: f64 = ((p.p29 * s.db[330][8]) * ddt_scale);
        let eq76_e1905_d_b9: f64 = ((p.p29 * s.db[330][9]) * ddt_scale);
        let eq76_e1905_d_b10: f64 = ((p.p29 * s.db[330][10]) * ddt_scale);
        let eq76_e1905_d_b11: f64 = ((p.p29 * s.db[330][11]) * ddt_scale);
        let eq76_e1905_d_b12: f64 = ((p.p29 * s.db[330][12]) * ddt_scale);
        let eq76_e1905_d_b13: f64 = ((p.p29 * s.db[330][13]) * ddt_scale);
        let eq76_e1906: f64 = (var_devsign * eq76_e1905);
        let eq76_e1906_d_n0: f64 = (var_devsign * eq76_e1905_d_n0);
        let eq76_e1906_d_n1: f64 = (var_devsign * eq76_e1905_d_n1);
        let eq76_e1906_d_n2: f64 = (var_devsign * eq76_e1905_d_n2);
        let eq76_e1906_d_n3: f64 = (var_devsign * eq76_e1905_d_n3);
        let eq76_e1906_d_n4: f64 = (var_devsign * eq76_e1905_d_n4);
        let eq76_e1906_d_n5: f64 = (var_devsign * eq76_e1905_d_n5);
        let eq76_e1906_d_n6: f64 = (var_devsign * eq76_e1905_d_n6);
        let eq76_e1906_d_n7: f64 = (var_devsign * eq76_e1905_d_n7);
        let eq76_e1906_d_n8: f64 = (var_devsign * eq76_e1905_d_n8);
        let eq76_e1906_d_n9: f64 = (var_devsign * eq76_e1905_d_n9);
        let eq76_e1906_d_n10: f64 = (var_devsign * eq76_e1905_d_n10);
        let eq76_e1906_d_n11: f64 = (var_devsign * eq76_e1905_d_n11);
        let eq76_e1906_d_n12: f64 = (var_devsign * eq76_e1905_d_n12);
        let eq76_e1906_d_n13: f64 = (var_devsign * eq76_e1905_d_n13);
        let eq76_e1906_d_n14: f64 = (var_devsign * eq76_e1905_d_n14);
        let eq76_e1906_d_n15: f64 = (var_devsign * eq76_e1905_d_n15);
        let eq76_e1906_d_n16: f64 = (var_devsign * eq76_e1905_d_n16);
        let eq76_e1906_d_b0: f64 = (var_devsign * eq76_e1905_d_b0);
        let eq76_e1906_d_b1: f64 = (var_devsign * eq76_e1905_d_b1);
        let eq76_e1906_d_b2: f64 = (var_devsign * eq76_e1905_d_b2);
        let eq76_e1906_d_b3: f64 = (var_devsign * eq76_e1905_d_b3);
        let eq76_e1906_d_b4: f64 = (var_devsign * eq76_e1905_d_b4);
        let eq76_e1906_d_b5: f64 = (var_devsign * eq76_e1905_d_b5);
        let eq76_e1906_d_b6: f64 = (var_devsign * eq76_e1905_d_b6);
        let eq76_e1906_d_b7: f64 = (var_devsign * eq76_e1905_d_b7);
        let eq76_e1906_d_b8: f64 = (var_devsign * eq76_e1905_d_b8);
        let eq76_e1906_d_b9: f64 = (var_devsign * eq76_e1905_d_b9);
        let eq76_e1906_d_b10: f64 = (var_devsign * eq76_e1905_d_b10);
        let eq76_e1906_d_b11: f64 = (var_devsign * eq76_e1905_d_b11);
        let eq76_e1906_d_b12: f64 = (var_devsign * eq76_e1905_d_b12);
        let eq76_e1906_d_b13: f64 = (var_devsign * eq76_e1905_d_b13);
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_d_b0, eq76_e1906_d_b1, eq76_e1906_d_b2, eq76_e1906_d_b3, eq76_e1906_d_b4, eq76_e1906_d_b5, eq76_e1906_d_b6, eq76_e1906_d_b7, eq76_e1906_d_b8, eq76_e1906_d_b9, eq76_e1906_d_b10, eq76_e1906_d_b11, eq76_e1906_d_b12, eq76_e1906_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivatives: [f64; 17] = [eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16];
        let eq76_branch_derivatives: [f64; 14] = [eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13,) = {
    if (!s.b[1627]) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1915: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq77_e1914);
        let eq77_e1916: f64 = (var_devsign * eq77_e1915);
        let eq77_e1916_d_n0: f64 = (var_devsign * __rspice_deriv_cse_2);
        let eq77_e1916_d_n1: f64 = (var_devsign * __rspice_deriv_cse_3);
        let eq77_e1916_d_n2: f64 = (var_devsign * __rspice_deriv_cse_4);
        let eq77_e1916_d_n3: f64 = (var_devsign * __rspice_deriv_cse_5);
        let eq77_e1916_d_n4: f64 = (var_devsign * __rspice_deriv_cse_6);
        let eq77_e1916_d_n5: f64 = (var_devsign * __rspice_deriv_cse_7);
        let eq77_e1916_d_n6: f64 = (var_devsign * __rspice_deriv_cse_8);
        let eq77_e1916_d_n7: f64 = (var_devsign * __rspice_deriv_cse_9);
        let eq77_e1916_d_n8: f64 = (var_devsign * __rspice_deriv_cse_10);
        let eq77_e1916_d_n9: f64 = (var_devsign * __rspice_deriv_cse_11);
        let eq77_e1916_d_n10: f64 = (var_devsign * __rspice_deriv_cse_12);
        let eq77_e1916_d_n11: f64 = (var_devsign * __rspice_deriv_cse_13);
        let eq77_e1916_d_n12: f64 = (var_devsign * __rspice_deriv_cse_14);
        let eq77_e1916_d_n13: f64 = (var_devsign * __rspice_deriv_cse_15);
        let eq77_e1916_d_n14: f64 = (var_devsign * __rspice_deriv_cse_16);
        let eq77_e1916_d_n15: f64 = (var_devsign * __rspice_deriv_cse_17);
        let eq77_e1916_d_n16: f64 = (var_devsign * __rspice_deriv_cse_18);
        let eq77_e1916_d_b0: f64 = (var_devsign * __rspice_deriv_cse_19);
        let eq77_e1916_d_b1: f64 = (var_devsign * __rspice_deriv_cse_20);
        let eq77_e1916_d_b2: f64 = (var_devsign * __rspice_deriv_cse_21);
        let eq77_e1916_d_b3: f64 = (var_devsign * __rspice_deriv_cse_22);
        let eq77_e1916_d_b4: f64 = (var_devsign * __rspice_deriv_cse_23);
        let eq77_e1916_d_b5: f64 = (var_devsign * __rspice_deriv_cse_24);
        let eq77_e1916_d_b6: f64 = (var_devsign * __rspice_deriv_cse_25);
        let eq77_e1916_d_b7: f64 = (var_devsign * __rspice_deriv_cse_26);
        let eq77_e1916_d_b8: f64 = (var_devsign * __rspice_deriv_cse_27);
        let eq77_e1916_d_b9: f64 = (var_devsign * __rspice_deriv_cse_28);
        let eq77_e1916_d_b10: f64 = (var_devsign * __rspice_deriv_cse_29);
        let eq77_e1916_d_b11: f64 = (var_devsign * __rspice_deriv_cse_30);
        let eq77_e1916_d_b12: f64 = (var_devsign * __rspice_deriv_cse_31);
        let eq77_e1916_d_b13: f64 = (var_devsign * __rspice_deriv_cse_32);
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_d_b0, eq77_e1916_d_b1, eq77_e1916_d_b2, eq77_e1916_d_b3, eq77_e1916_d_b4, eq77_e1916_d_b5, eq77_e1916_d_b6, eq77_e1916_d_b7, eq77_e1916_d_b8, eq77_e1916_d_b9, eq77_e1916_d_b10, eq77_e1916_d_b11, eq77_e1916_d_b12, eq77_e1916_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivatives: [f64; 17] = [eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16];
        let eq77_branch_derivatives: [f64; 14] = [eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivatives,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq81_e1959, eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16, eq81_e1959_d_b0, eq81_e1959_d_b1, eq81_e1959_d_b2, eq81_e1959_d_b3, eq81_e1959_d_b4, eq81_e1959_d_b5, eq81_e1959_d_b6, eq81_e1959_d_b7, eq81_e1959_d_b8, eq81_e1959_d_b9, eq81_e1959_d_b10, eq81_e1959_d_b11, eq81_e1959_d_b12, eq81_e1959_d_b13,) = {
    if s.b[1630] {
        let eq81_e1945: f64 = (var_devsign * p.p28);
        let eq81_e1947: f64 = (eq81_e1945 * s.v[304]);
        let eq81_e1950: f64 = (1.0 - p.p1128);
        let eq81_e1952: f64 = (eq81_e1950 * p.p28);
        let eq81_e1954: f64 = (eq81_e1952 * (nv13 - nv5));
        let eq81_e1956: f64 = (eq81_e1954 * s.v[781]);
        let eq81_e1956_d_n5: f64 = ((-eq81_e1952) * s.v[781]);
        let eq81_e1956_d_n13: f64 = (eq81_e1952 * s.v[781]);
        let eq81_e1957: f64 = (eq81_e1947 + eq81_e1956);
        let eq81_e1957_d_n5: f64 = ((eq81_e1945 * s.dn[304][5]) + eq81_e1956_d_n5);
        let eq81_e1957_d_n13: f64 = ((eq81_e1945 * s.dn[304][13]) + eq81_e1956_d_n13);
        (eq81_e1957, (eq81_e1945 * s.dn[304][0]), (eq81_e1945 * s.dn[304][1]), (eq81_e1945 * s.dn[304][2]), (eq81_e1945 * s.dn[304][3]), (eq81_e1945 * s.dn[304][4]), eq81_e1957_d_n5, (eq81_e1945 * s.dn[304][6]), (eq81_e1945 * s.dn[304][7]), (eq81_e1945 * s.dn[304][8]), (eq81_e1945 * s.dn[304][9]), (eq81_e1945 * s.dn[304][10]), (eq81_e1945 * s.dn[304][11]), (eq81_e1945 * s.dn[304][12]), eq81_e1957_d_n13, (eq81_e1945 * s.dn[304][14]), (eq81_e1945 * s.dn[304][15]), (eq81_e1945 * s.dn[304][16]), (eq81_e1945 * s.db[304][0]), (eq81_e1945 * s.db[304][1]), (eq81_e1945 * s.db[304][2]), (eq81_e1945 * s.db[304][3]), (eq81_e1945 * s.db[304][4]), (eq81_e1945 * s.db[304][5]), (eq81_e1945 * s.db[304][6]), (eq81_e1945 * s.db[304][7]), (eq81_e1945 * s.db[304][8]), (eq81_e1945 * s.db[304][9]), (eq81_e1945 * s.db[304][10]), (eq81_e1945 * s.db[304][11]), (eq81_e1945 * s.db[304][12]), (eq81_e1945 * s.db[304][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq81_value: f64 = eq81_e1959;
        let eq81_node_derivatives: [f64; 17] = [eq81_e1959_d_n0, eq81_e1959_d_n1, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14, eq81_e1959_d_n15, eq81_e1959_d_n16];
        let eq81_branch_derivatives: [f64; 14] = [eq81_e1959_d_b0, eq81_e1959_d_b1, eq81_e1959_d_b2, eq81_e1959_d_b3, eq81_e1959_d_b4, eq81_e1959_d_b5, eq81_e1959_d_b6, eq81_e1959_d_b7, eq81_e1959_d_b8, eq81_e1959_d_b9, eq81_e1959_d_b10, eq81_e1959_d_b11, eq81_e1959_d_b12, eq81_e1959_d_b13];
        stamper.stamp_current_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq81_value),
            &eq81_node_derivatives,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e1975, eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16, eq82_e1975_d_b0, eq82_e1975_d_b1, eq82_e1975_d_b2, eq82_e1975_d_b3, eq82_e1975_d_b4, eq82_e1975_d_b5, eq82_e1975_d_b6, eq82_e1975_d_b7, eq82_e1975_d_b8, eq82_e1975_d_b9, eq82_e1975_d_b10, eq82_e1975_d_b11, eq82_e1975_d_b12, eq82_e1975_d_b13,) = {
    if s.b[1630] {
        let eq82_e1963: f64 = (var_devsign * p.p28);
        let eq82_e1965: f64 = (eq82_e1963 * s.v[305]);
        let eq82_e1968: f64 = (p.p1128 * p.p28);
        let eq82_e1970: f64 = (eq82_e1968 * (nv13 - nv14));
        let eq82_e1972: f64 = (eq82_e1970 * s.v[781]);
        let eq82_e1972_d_n13: f64 = (eq82_e1968 * s.v[781]);
        let eq82_e1972_d_n14: f64 = ((-eq82_e1968) * s.v[781]);
        let eq82_e1973: f64 = (eq82_e1965 + eq82_e1972);
        let eq82_e1973_d_n13: f64 = ((eq82_e1963 * s.dn[305][13]) + eq82_e1972_d_n13);
        let eq82_e1973_d_n14: f64 = ((eq82_e1963 * s.dn[305][14]) + eq82_e1972_d_n14);
        (eq82_e1973, (eq82_e1963 * s.dn[305][0]), (eq82_e1963 * s.dn[305][1]), (eq82_e1963 * s.dn[305][2]), (eq82_e1963 * s.dn[305][3]), (eq82_e1963 * s.dn[305][4]), (eq82_e1963 * s.dn[305][5]), (eq82_e1963 * s.dn[305][6]), (eq82_e1963 * s.dn[305][7]), (eq82_e1963 * s.dn[305][8]), (eq82_e1963 * s.dn[305][9]), (eq82_e1963 * s.dn[305][10]), (eq82_e1963 * s.dn[305][11]), (eq82_e1963 * s.dn[305][12]), eq82_e1973_d_n13, eq82_e1973_d_n14, (eq82_e1963 * s.dn[305][15]), (eq82_e1963 * s.dn[305][16]), (eq82_e1963 * s.db[305][0]), (eq82_e1963 * s.db[305][1]), (eq82_e1963 * s.db[305][2]), (eq82_e1963 * s.db[305][3]), (eq82_e1963 * s.db[305][4]), (eq82_e1963 * s.db[305][5]), (eq82_e1963 * s.db[305][6]), (eq82_e1963 * s.db[305][7]), (eq82_e1963 * s.db[305][8]), (eq82_e1963 * s.db[305][9]), (eq82_e1963 * s.db[305][10]), (eq82_e1963 * s.db[305][11]), (eq82_e1963 * s.db[305][12]), (eq82_e1963 * s.db[305][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1975;
        let eq82_node_derivatives: [f64; 17] = [eq82_e1975_d_n0, eq82_e1975_d_n1, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14, eq82_e1975_d_n15, eq82_e1975_d_n16];
        let eq82_branch_derivatives: [f64; 14] = [eq82_e1975_d_b0, eq82_e1975_d_b1, eq82_e1975_d_b2, eq82_e1975_d_b3, eq82_e1975_d_b4, eq82_e1975_d_b5, eq82_e1975_d_b6, eq82_e1975_d_b7, eq82_e1975_d_b8, eq82_e1975_d_b9, eq82_e1975_d_b10, eq82_e1975_d_b11, eq82_e1975_d_b12, eq82_e1975_d_b13];
        stamper.stamp_current_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq82_value),
            &eq82_node_derivatives,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13,) = {
    if s.b[1630] {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1981: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq83_e1980);
        let eq83_e1982: f64 = (var_devsign * eq83_e1981);
        let eq83_e1982_d_n0: f64 = (var_devsign * __rspice_deriv_cse_2);
        let eq83_e1982_d_n1: f64 = (var_devsign * __rspice_deriv_cse_3);
        let eq83_e1982_d_n2: f64 = (var_devsign * __rspice_deriv_cse_4);
        let eq83_e1982_d_n3: f64 = (var_devsign * __rspice_deriv_cse_5);
        let eq83_e1982_d_n4: f64 = (var_devsign * __rspice_deriv_cse_6);
        let eq83_e1982_d_n5: f64 = (var_devsign * __rspice_deriv_cse_7);
        let eq83_e1982_d_n6: f64 = (var_devsign * __rspice_deriv_cse_8);
        let eq83_e1982_d_n7: f64 = (var_devsign * __rspice_deriv_cse_9);
        let eq83_e1982_d_n8: f64 = (var_devsign * __rspice_deriv_cse_10);
        let eq83_e1982_d_n9: f64 = (var_devsign * __rspice_deriv_cse_11);
        let eq83_e1982_d_n10: f64 = (var_devsign * __rspice_deriv_cse_12);
        let eq83_e1982_d_n11: f64 = (var_devsign * __rspice_deriv_cse_13);
        let eq83_e1982_d_n12: f64 = (var_devsign * __rspice_deriv_cse_14);
        let eq83_e1982_d_n13: f64 = (var_devsign * __rspice_deriv_cse_15);
        let eq83_e1982_d_n14: f64 = (var_devsign * __rspice_deriv_cse_16);
        let eq83_e1982_d_n15: f64 = (var_devsign * __rspice_deriv_cse_17);
        let eq83_e1982_d_n16: f64 = (var_devsign * __rspice_deriv_cse_18);
        let eq83_e1982_d_b0: f64 = (var_devsign * __rspice_deriv_cse_19);
        let eq83_e1982_d_b1: f64 = (var_devsign * __rspice_deriv_cse_20);
        let eq83_e1982_d_b2: f64 = (var_devsign * __rspice_deriv_cse_21);
        let eq83_e1982_d_b3: f64 = (var_devsign * __rspice_deriv_cse_22);
        let eq83_e1982_d_b4: f64 = (var_devsign * __rspice_deriv_cse_23);
        let eq83_e1982_d_b5: f64 = (var_devsign * __rspice_deriv_cse_24);
        let eq83_e1982_d_b6: f64 = (var_devsign * __rspice_deriv_cse_25);
        let eq83_e1982_d_b7: f64 = (var_devsign * __rspice_deriv_cse_26);
        let eq83_e1982_d_b8: f64 = (var_devsign * __rspice_deriv_cse_27);
        let eq83_e1982_d_b9: f64 = (var_devsign * __rspice_deriv_cse_28);
        let eq83_e1982_d_b10: f64 = (var_devsign * __rspice_deriv_cse_29);
        let eq83_e1982_d_b11: f64 = (var_devsign * __rspice_deriv_cse_30);
        let eq83_e1982_d_b12: f64 = (var_devsign * __rspice_deriv_cse_31);
        let eq83_e1982_d_b13: f64 = (var_devsign * __rspice_deriv_cse_32);
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_d_b0, eq83_e1982_d_b1, eq83_e1982_d_b2, eq83_e1982_d_b3, eq83_e1982_d_b4, eq83_e1982_d_b5, eq83_e1982_d_b6, eq83_e1982_d_b7, eq83_e1982_d_b8, eq83_e1982_d_b9, eq83_e1982_d_b10, eq83_e1982_d_b11, eq83_e1982_d_b12, eq83_e1982_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivatives: [f64; 17] = [eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16];
        let eq83_branch_derivatives: [f64; 14] = [eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13];
        stamper.stamp_current_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq83_value),
            &eq83_node_derivatives,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13,) = {
    if s.b[1630] {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq84_e1989);
        let eq84_e1990_d_n0: f64 = ((p.p29 * s.dn[338][0]) * ddt_scale);
        let eq84_e1990_d_n1: f64 = ((p.p29 * s.dn[338][1]) * ddt_scale);
        let eq84_e1990_d_n2: f64 = ((p.p29 * s.dn[338][2]) * ddt_scale);
        let eq84_e1990_d_n3: f64 = ((p.p29 * s.dn[338][3]) * ddt_scale);
        let eq84_e1990_d_n4: f64 = ((p.p29 * s.dn[338][4]) * ddt_scale);
        let eq84_e1990_d_n5: f64 = ((p.p29 * s.dn[338][5]) * ddt_scale);
        let eq84_e1990_d_n6: f64 = ((p.p29 * s.dn[338][6]) * ddt_scale);
        let eq84_e1990_d_n7: f64 = ((p.p29 * s.dn[338][7]) * ddt_scale);
        let eq84_e1990_d_n8: f64 = ((p.p29 * s.dn[338][8]) * ddt_scale);
        let eq84_e1990_d_n9: f64 = ((p.p29 * s.dn[338][9]) * ddt_scale);
        let eq84_e1990_d_n10: f64 = ((p.p29 * s.dn[338][10]) * ddt_scale);
        let eq84_e1990_d_n11: f64 = ((p.p29 * s.dn[338][11]) * ddt_scale);
        let eq84_e1990_d_n12: f64 = ((p.p29 * s.dn[338][12]) * ddt_scale);
        let eq84_e1990_d_n13: f64 = ((p.p29 * s.dn[338][13]) * ddt_scale);
        let eq84_e1990_d_n14: f64 = ((p.p29 * s.dn[338][14]) * ddt_scale);
        let eq84_e1990_d_n15: f64 = ((p.p29 * s.dn[338][15]) * ddt_scale);
        let eq84_e1990_d_n16: f64 = ((p.p29 * s.dn[338][16]) * ddt_scale);
        let eq84_e1990_d_b0: f64 = ((p.p29 * s.db[338][0]) * ddt_scale);
        let eq84_e1990_d_b1: f64 = ((p.p29 * s.db[338][1]) * ddt_scale);
        let eq84_e1990_d_b2: f64 = ((p.p29 * s.db[338][2]) * ddt_scale);
        let eq84_e1990_d_b3: f64 = ((p.p29 * s.db[338][3]) * ddt_scale);
        let eq84_e1990_d_b4: f64 = ((p.p29 * s.db[338][4]) * ddt_scale);
        let eq84_e1990_d_b5: f64 = ((p.p29 * s.db[338][5]) * ddt_scale);
        let eq84_e1990_d_b6: f64 = ((p.p29 * s.db[338][6]) * ddt_scale);
        let eq84_e1990_d_b7: f64 = ((p.p29 * s.db[338][7]) * ddt_scale);
        let eq84_e1990_d_b8: f64 = ((p.p29 * s.db[338][8]) * ddt_scale);
        let eq84_e1990_d_b9: f64 = ((p.p29 * s.db[338][9]) * ddt_scale);
        let eq84_e1990_d_b10: f64 = ((p.p29 * s.db[338][10]) * ddt_scale);
        let eq84_e1990_d_b11: f64 = ((p.p29 * s.db[338][11]) * ddt_scale);
        let eq84_e1990_d_b12: f64 = ((p.p29 * s.db[338][12]) * ddt_scale);
        let eq84_e1990_d_b13: f64 = ((p.p29 * s.db[338][13]) * ddt_scale);
        let eq84_e1991: f64 = (var_devsign * eq84_e1990);
        let eq84_e1991_d_n0: f64 = (var_devsign * eq84_e1990_d_n0);
        let eq84_e1991_d_n1: f64 = (var_devsign * eq84_e1990_d_n1);
        let eq84_e1991_d_n2: f64 = (var_devsign * eq84_e1990_d_n2);
        let eq84_e1991_d_n3: f64 = (var_devsign * eq84_e1990_d_n3);
        let eq84_e1991_d_n4: f64 = (var_devsign * eq84_e1990_d_n4);
        let eq84_e1991_d_n5: f64 = (var_devsign * eq84_e1990_d_n5);
        let eq84_e1991_d_n6: f64 = (var_devsign * eq84_e1990_d_n6);
        let eq84_e1991_d_n7: f64 = (var_devsign * eq84_e1990_d_n7);
        let eq84_e1991_d_n8: f64 = (var_devsign * eq84_e1990_d_n8);
        let eq84_e1991_d_n9: f64 = (var_devsign * eq84_e1990_d_n9);
        let eq84_e1991_d_n10: f64 = (var_devsign * eq84_e1990_d_n10);
        let eq84_e1991_d_n11: f64 = (var_devsign * eq84_e1990_d_n11);
        let eq84_e1991_d_n12: f64 = (var_devsign * eq84_e1990_d_n12);
        let eq84_e1991_d_n13: f64 = (var_devsign * eq84_e1990_d_n13);
        let eq84_e1991_d_n14: f64 = (var_devsign * eq84_e1990_d_n14);
        let eq84_e1991_d_n15: f64 = (var_devsign * eq84_e1990_d_n15);
        let eq84_e1991_d_n16: f64 = (var_devsign * eq84_e1990_d_n16);
        let eq84_e1991_d_b0: f64 = (var_devsign * eq84_e1990_d_b0);
        let eq84_e1991_d_b1: f64 = (var_devsign * eq84_e1990_d_b1);
        let eq84_e1991_d_b2: f64 = (var_devsign * eq84_e1990_d_b2);
        let eq84_e1991_d_b3: f64 = (var_devsign * eq84_e1990_d_b3);
        let eq84_e1991_d_b4: f64 = (var_devsign * eq84_e1990_d_b4);
        let eq84_e1991_d_b5: f64 = (var_devsign * eq84_e1990_d_b5);
        let eq84_e1991_d_b6: f64 = (var_devsign * eq84_e1990_d_b6);
        let eq84_e1991_d_b7: f64 = (var_devsign * eq84_e1990_d_b7);
        let eq84_e1991_d_b8: f64 = (var_devsign * eq84_e1990_d_b8);
        let eq84_e1991_d_b9: f64 = (var_devsign * eq84_e1990_d_b9);
        let eq84_e1991_d_b10: f64 = (var_devsign * eq84_e1990_d_b10);
        let eq84_e1991_d_b11: f64 = (var_devsign * eq84_e1990_d_b11);
        let eq84_e1991_d_b12: f64 = (var_devsign * eq84_e1990_d_b12);
        let eq84_e1991_d_b13: f64 = (var_devsign * eq84_e1990_d_b13);
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_d_b0, eq84_e1991_d_b1, eq84_e1991_d_b2, eq84_e1991_d_b3, eq84_e1991_d_b4, eq84_e1991_d_b5, eq84_e1991_d_b6, eq84_e1991_d_b7, eq84_e1991_d_b8, eq84_e1991_d_b9, eq84_e1991_d_b10, eq84_e1991_d_b11, eq84_e1991_d_b12, eq84_e1991_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivatives: [f64; 17] = [eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16];
        let eq84_branch_derivatives: [f64; 14] = [eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13];
        stamper.stamp_current_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq84_value),
            &eq84_node_derivatives,
            &eq84_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16, eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13, eq8_e1290_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq8_e1279: f64 = (s.v[378] * s.v[46]);
        let eq8_e1281: f64 = (eq8_e1279 * s.v[29]);
        let eq8_e1281_d_n0: f64 = ((s.dn[378][0] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n1: f64 = ((s.dn[378][1] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n2: f64 = ((s.dn[378][2] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n3: f64 = ((s.dn[378][3] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n4: f64 = ((s.dn[378][4] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n5: f64 = ((s.dn[378][5] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n6: f64 = ((s.dn[378][6] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n7: f64 = ((s.dn[378][7] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n8: f64 = ((s.dn[378][8] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n9: f64 = ((s.dn[378][9] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n10: f64 = ((s.dn[378][10] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n11: f64 = ((s.dn[378][11] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n12: f64 = ((s.dn[378][12] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n13: f64 = ((s.dn[378][13] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n14: f64 = ((s.dn[378][14] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n15: f64 = ((s.dn[378][15] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_n16: f64 = ((s.dn[378][16] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b0: f64 = ((s.db[378][0] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b1: f64 = ((s.db[378][1] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b2: f64 = ((s.db[378][2] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b3: f64 = ((s.db[378][3] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b4: f64 = ((s.db[378][4] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b5: f64 = ((s.db[378][5] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b6: f64 = ((s.db[378][6] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b7: f64 = ((s.db[378][7] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b8: f64 = ((s.db[378][8] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b9: f64 = ((s.db[378][9] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b10: f64 = ((s.db[378][10] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b11: f64 = ((s.db[378][11] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b12: f64 = ((s.db[378][12] * s.v[46]) * s.v[29]);
        let eq8_e1281_d_b13: f64 = ((s.db[378][13] * s.v[46]) * s.v[29]);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n1: f64 = (eq8_e1281_d_n1 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1283_d_n15: f64 = (eq8_e1281_d_n15 * p.p2);
        let eq8_e1283_d_n16: f64 = (eq8_e1281_d_n16 * p.p2);
        let eq8_e1283_d_b0: f64 = (eq8_e1281_d_b0 * p.p2);
        let eq8_e1283_d_b1: f64 = (eq8_e1281_d_b1 * p.p2);
        let eq8_e1283_d_b2: f64 = (eq8_e1281_d_b2 * p.p2);
        let eq8_e1283_d_b3: f64 = (eq8_e1281_d_b3 * p.p2);
        let eq8_e1283_d_b4: f64 = (eq8_e1281_d_b4 * p.p2);
        let eq8_e1283_d_b5: f64 = (eq8_e1281_d_b5 * p.p2);
        let eq8_e1283_d_b6: f64 = (eq8_e1281_d_b6 * p.p2);
        let eq8_e1283_d_b7: f64 = (eq8_e1281_d_b7 * p.p2);
        let eq8_e1283_d_b8: f64 = (eq8_e1281_d_b8 * p.p2);
        let eq8_e1283_d_b9: f64 = (eq8_e1281_d_b9 * p.p2);
        let eq8_e1283_d_b10: f64 = (eq8_e1281_d_b10 * p.p2);
        let eq8_e1283_d_b11: f64 = (eq8_e1281_d_b11 * p.p2);
        let eq8_e1283_d_b12: f64 = (eq8_e1281_d_b12 * p.p2);
        let eq8_e1283_d_b13: f64 = (eq8_e1281_d_b13 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * s.v[30]);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * s.v[30]);
        let eq8_e1285_d_n1: f64 = (eq8_e1283_d_n1 * s.v[30]);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * s.v[30]);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * s.v[30]);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * s.v[30]);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * s.v[30]);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * s.v[30]);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * s.v[30]);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * s.v[30]);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * s.v[30]);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * s.v[30]);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * s.v[30]);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * s.v[30]);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * s.v[30]);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * s.v[30]);
        let eq8_e1285_d_n15: f64 = (eq8_e1283_d_n15 * s.v[30]);
        let eq8_e1285_d_n16: f64 = (eq8_e1283_d_n16 * s.v[30]);
        let eq8_e1285_d_b0: f64 = (eq8_e1283_d_b0 * s.v[30]);
        let eq8_e1285_d_b1: f64 = (eq8_e1283_d_b1 * s.v[30]);
        let eq8_e1285_d_b2: f64 = (eq8_e1283_d_b2 * s.v[30]);
        let eq8_e1285_d_b3: f64 = (eq8_e1283_d_b3 * s.v[30]);
        let eq8_e1285_d_b4: f64 = (eq8_e1283_d_b4 * s.v[30]);
        let eq8_e1285_d_b5: f64 = (eq8_e1283_d_b5 * s.v[30]);
        let eq8_e1285_d_b6: f64 = (eq8_e1283_d_b6 * s.v[30]);
        let eq8_e1285_d_b7: f64 = (eq8_e1283_d_b7 * s.v[30]);
        let eq8_e1285_d_b8: f64 = (eq8_e1283_d_b8 * s.v[30]);
        let eq8_e1285_d_b9: f64 = (eq8_e1283_d_b9 * s.v[30]);
        let eq8_e1285_d_b10: f64 = (eq8_e1283_d_b10 * s.v[30]);
        let eq8_e1285_d_b11: f64 = (eq8_e1283_d_b11 * s.v[30]);
        let eq8_e1285_d_b12: f64 = (eq8_e1283_d_b12 * s.v[30]);
        let eq8_e1285_d_b13: f64 = (eq8_e1283_d_b13 * s.v[30]);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n1: f64 = (eq8_e1285_d_n1 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1287_d_n15: f64 = ((eq8_e1285_d_n15 * (nv15 - 0.0)) + eq8_e1285);
        let eq8_e1287_d_n16: f64 = (eq8_e1285_d_n16 * (nv15 - 0.0));
        let eq8_e1287_d_b0: f64 = (eq8_e1285_d_b0 * (nv15 - 0.0));
        let eq8_e1287_d_b1: f64 = (eq8_e1285_d_b1 * (nv15 - 0.0));
        let eq8_e1287_d_b2: f64 = (eq8_e1285_d_b2 * (nv15 - 0.0));
        let eq8_e1287_d_b3: f64 = (eq8_e1285_d_b3 * (nv15 - 0.0));
        let eq8_e1287_d_b4: f64 = (eq8_e1285_d_b4 * (nv15 - 0.0));
        let eq8_e1287_d_b5: f64 = (eq8_e1285_d_b5 * (nv15 - 0.0));
        let eq8_e1287_d_b6: f64 = (eq8_e1285_d_b6 * (nv15 - 0.0));
        let eq8_e1287_d_b7: f64 = (eq8_e1285_d_b7 * (nv15 - 0.0));
        let eq8_e1287_d_b8: f64 = (eq8_e1285_d_b8 * (nv15 - 0.0));
        let eq8_e1287_d_b9: f64 = (eq8_e1285_d_b9 * (nv15 - 0.0));
        let eq8_e1287_d_b10: f64 = (eq8_e1285_d_b10 * (nv15 - 0.0));
        let eq8_e1287_d_b11: f64 = (eq8_e1285_d_b11 * (nv15 - 0.0));
        let eq8_e1287_d_b12: f64 = (eq8_e1285_d_b12 * (nv15 - 0.0));
        let eq8_e1287_d_b13: f64 = (eq8_e1285_d_b13 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n1, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1287_d_n15, eq8_e1287_d_n16, eq8_e1287_d_b0, eq8_e1287_d_b1, eq8_e1287_d_b2, eq8_e1287_d_b3, eq8_e1287_d_b4, eq8_e1287_d_b5, eq8_e1287_d_b6, eq8_e1287_d_b7, eq8_e1287_d_b8, eq8_e1287_d_b9, eq8_e1287_d_b10, eq8_e1287_d_b11, eq8_e1287_d_b12, eq8_e1287_d_b13, eq8_e1288_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, eq8_e1290_d_n1, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_d_n16];
        let eq8_reactive_branch_derivatives: [f64; 14] = [eq8_e1290_d_b0, eq8_e1290_d_b1, eq8_e1290_d_b2, eq8_e1290_d_b3, eq8_e1290_d_b4, eq8_e1290_d_b5, eq8_e1290_d_b6, eq8_e1290_d_b7, eq8_e1290_d_b8, eq8_e1290_d_b9, eq8_e1290_d_b10, eq8_e1290_d_b11, eq8_e1290_d_b12, eq8_e1290_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16, eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13, eq11_e1344_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1329_d_b0: f64 = ((s.db[57][0] * s.v[378]) + (eq11_e1327 * s.db[378][0]));
        let eq11_e1329_d_b1: f64 = ((s.db[57][1] * s.v[378]) + (eq11_e1327 * s.db[378][1]));
        let eq11_e1329_d_b2: f64 = ((s.db[57][2] * s.v[378]) + (eq11_e1327 * s.db[378][2]));
        let eq11_e1329_d_b3: f64 = ((s.db[57][3] * s.v[378]) + (eq11_e1327 * s.db[378][3]));
        let eq11_e1329_d_b4: f64 = ((s.db[57][4] * s.v[378]) + (eq11_e1327 * s.db[378][4]));
        let eq11_e1329_d_b5: f64 = ((s.db[57][5] * s.v[378]) + (eq11_e1327 * s.db[378][5]));
        let eq11_e1329_d_b6: f64 = ((s.db[57][6] * s.v[378]) + (eq11_e1327 * s.db[378][6]));
        let eq11_e1329_d_b7: f64 = ((s.db[57][7] * s.v[378]) + (eq11_e1327 * s.db[378][7]));
        let eq11_e1329_d_b8: f64 = ((s.db[57][8] * s.v[378]) + (eq11_e1327 * s.db[378][8]));
        let eq11_e1329_d_b9: f64 = ((s.db[57][9] * s.v[378]) + (eq11_e1327 * s.db[378][9]));
        let eq11_e1329_d_b10: f64 = ((s.db[57][10] * s.v[378]) + (eq11_e1327 * s.db[378][10]));
        let eq11_e1329_d_b11: f64 = ((s.db[57][11] * s.v[378]) + (eq11_e1327 * s.db[378][11]));
        let eq11_e1329_d_b12: f64 = ((s.db[57][12] * s.v[378]) + (eq11_e1327 * s.db[378][12]));
        let eq11_e1329_d_b13: f64 = ((s.db[57][13] * s.v[378]) + (eq11_e1327 * s.db[378][13]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1331_d_b0: f64 = (eq11_e1329_d_b0 * s.v[46]);
        let eq11_e1331_d_b1: f64 = (eq11_e1329_d_b1 * s.v[46]);
        let eq11_e1331_d_b2: f64 = (eq11_e1329_d_b2 * s.v[46]);
        let eq11_e1331_d_b3: f64 = (eq11_e1329_d_b3 * s.v[46]);
        let eq11_e1331_d_b4: f64 = (eq11_e1329_d_b4 * s.v[46]);
        let eq11_e1331_d_b5: f64 = (eq11_e1329_d_b5 * s.v[46]);
        let eq11_e1331_d_b6: f64 = (eq11_e1329_d_b6 * s.v[46]);
        let eq11_e1331_d_b7: f64 = (eq11_e1329_d_b7 * s.v[46]);
        let eq11_e1331_d_b8: f64 = (eq11_e1329_d_b8 * s.v[46]);
        let eq11_e1331_d_b9: f64 = (eq11_e1329_d_b9 * s.v[46]);
        let eq11_e1331_d_b10: f64 = (eq11_e1329_d_b10 * s.v[46]);
        let eq11_e1331_d_b11: f64 = (eq11_e1329_d_b11 * s.v[46]);
        let eq11_e1331_d_b12: f64 = (eq11_e1329_d_b12 * s.v[46]);
        let eq11_e1331_d_b13: f64 = (eq11_e1329_d_b13 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1333_d_b0: f64 = (eq11_e1331_d_b0 * s.v[29]);
        let eq11_e1333_d_b1: f64 = (eq11_e1331_d_b1 * s.v[29]);
        let eq11_e1333_d_b2: f64 = (eq11_e1331_d_b2 * s.v[29]);
        let eq11_e1333_d_b3: f64 = (eq11_e1331_d_b3 * s.v[29]);
        let eq11_e1333_d_b4: f64 = (eq11_e1331_d_b4 * s.v[29]);
        let eq11_e1333_d_b5: f64 = (eq11_e1331_d_b5 * s.v[29]);
        let eq11_e1333_d_b6: f64 = (eq11_e1331_d_b6 * s.v[29]);
        let eq11_e1333_d_b7: f64 = (eq11_e1331_d_b7 * s.v[29]);
        let eq11_e1333_d_b8: f64 = (eq11_e1331_d_b8 * s.v[29]);
        let eq11_e1333_d_b9: f64 = (eq11_e1331_d_b9 * s.v[29]);
        let eq11_e1333_d_b10: f64 = (eq11_e1331_d_b10 * s.v[29]);
        let eq11_e1333_d_b11: f64 = (eq11_e1331_d_b11 * s.v[29]);
        let eq11_e1333_d_b12: f64 = (eq11_e1331_d_b12 * s.v[29]);
        let eq11_e1333_d_b13: f64 = (eq11_e1331_d_b13 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1335_d_b0: f64 = (eq11_e1333_d_b0 * p.p2);
        let eq11_e1335_d_b1: f64 = (eq11_e1333_d_b1 * p.p2);
        let eq11_e1335_d_b2: f64 = (eq11_e1333_d_b2 * p.p2);
        let eq11_e1335_d_b3: f64 = (eq11_e1333_d_b3 * p.p2);
        let eq11_e1335_d_b4: f64 = (eq11_e1333_d_b4 * p.p2);
        let eq11_e1335_d_b5: f64 = (eq11_e1333_d_b5 * p.p2);
        let eq11_e1335_d_b6: f64 = (eq11_e1333_d_b6 * p.p2);
        let eq11_e1335_d_b7: f64 = (eq11_e1333_d_b7 * p.p2);
        let eq11_e1335_d_b8: f64 = (eq11_e1333_d_b8 * p.p2);
        let eq11_e1335_d_b9: f64 = (eq11_e1333_d_b9 * p.p2);
        let eq11_e1335_d_b10: f64 = (eq11_e1333_d_b10 * p.p2);
        let eq11_e1335_d_b11: f64 = (eq11_e1333_d_b11 * p.p2);
        let eq11_e1335_d_b12: f64 = (eq11_e1333_d_b12 * p.p2);
        let eq11_e1335_d_b13: f64 = (eq11_e1333_d_b13 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1337_d_b0: f64 = (eq11_e1335_d_b0 * s.v[30]);
        let eq11_e1337_d_b1: f64 = (eq11_e1335_d_b1 * s.v[30]);
        let eq11_e1337_d_b2: f64 = (eq11_e1335_d_b2 * s.v[30]);
        let eq11_e1337_d_b3: f64 = (eq11_e1335_d_b3 * s.v[30]);
        let eq11_e1337_d_b4: f64 = (eq11_e1335_d_b4 * s.v[30]);
        let eq11_e1337_d_b5: f64 = (eq11_e1335_d_b5 * s.v[30]);
        let eq11_e1337_d_b6: f64 = (eq11_e1335_d_b6 * s.v[30]);
        let eq11_e1337_d_b7: f64 = (eq11_e1335_d_b7 * s.v[30]);
        let eq11_e1337_d_b8: f64 = (eq11_e1335_d_b8 * s.v[30]);
        let eq11_e1337_d_b9: f64 = (eq11_e1335_d_b9 * s.v[30]);
        let eq11_e1337_d_b10: f64 = (eq11_e1335_d_b10 * s.v[30]);
        let eq11_e1337_d_b11: f64 = (eq11_e1335_d_b11 * s.v[30]);
        let eq11_e1337_d_b12: f64 = (eq11_e1335_d_b12 * s.v[30]);
        let eq11_e1337_d_b13: f64 = (eq11_e1335_d_b13 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1339_d_b0: f64 = (eq11_e1337_d_b0 * (nv15 - 0.0));
        let eq11_e1339_d_b1: f64 = (eq11_e1337_d_b1 * (nv15 - 0.0));
        let eq11_e1339_d_b2: f64 = (eq11_e1337_d_b2 * (nv15 - 0.0));
        let eq11_e1339_d_b3: f64 = (eq11_e1337_d_b3 * (nv15 - 0.0));
        let eq11_e1339_d_b4: f64 = (eq11_e1337_d_b4 * (nv15 - 0.0));
        let eq11_e1339_d_b5: f64 = (eq11_e1337_d_b5 * (nv15 - 0.0));
        let eq11_e1339_d_b6: f64 = (eq11_e1337_d_b6 * (nv15 - 0.0));
        let eq11_e1339_d_b7: f64 = (eq11_e1337_d_b7 * (nv15 - 0.0));
        let eq11_e1339_d_b8: f64 = (eq11_e1337_d_b8 * (nv15 - 0.0));
        let eq11_e1339_d_b9: f64 = (eq11_e1337_d_b9 * (nv15 - 0.0));
        let eq11_e1339_d_b10: f64 = (eq11_e1337_d_b10 * (nv15 - 0.0));
        let eq11_e1339_d_b11: f64 = (eq11_e1337_d_b11 * (nv15 - 0.0));
        let eq11_e1339_d_b12: f64 = (eq11_e1337_d_b12 * (nv15 - 0.0));
        let eq11_e1339_d_b13: f64 = (eq11_e1337_d_b13 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1340_d_b0: f64 = (0.5 * eq11_e1339_d_b0);
        let eq11_e1340_d_b1: f64 = (0.5 * eq11_e1339_d_b1);
        let eq11_e1340_d_b2: f64 = (0.5 * eq11_e1339_d_b2);
        let eq11_e1340_d_b3: f64 = (0.5 * eq11_e1339_d_b3);
        let eq11_e1340_d_b4: f64 = (0.5 * eq11_e1339_d_b4);
        let eq11_e1340_d_b5: f64 = (0.5 * eq11_e1339_d_b5);
        let eq11_e1340_d_b6: f64 = (0.5 * eq11_e1339_d_b6);
        let eq11_e1340_d_b7: f64 = (0.5 * eq11_e1339_d_b7);
        let eq11_e1340_d_b8: f64 = (0.5 * eq11_e1339_d_b8);
        let eq11_e1340_d_b9: f64 = (0.5 * eq11_e1339_d_b9);
        let eq11_e1340_d_b10: f64 = (0.5 * eq11_e1339_d_b10);
        let eq11_e1340_d_b11: f64 = (0.5 * eq11_e1339_d_b11);
        let eq11_e1340_d_b12: f64 = (0.5 * eq11_e1339_d_b12);
        let eq11_e1340_d_b13: f64 = (0.5 * eq11_e1339_d_b13);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        let eq11_e1342_d_b0: f64 = (p.p29 * eq11_e1340_d_b0);
        let eq11_e1342_d_b1: f64 = (p.p29 * eq11_e1340_d_b1);
        let eq11_e1342_d_b2: f64 = (p.p29 * eq11_e1340_d_b2);
        let eq11_e1342_d_b3: f64 = (p.p29 * eq11_e1340_d_b3);
        let eq11_e1342_d_b4: f64 = (p.p29 * eq11_e1340_d_b4);
        let eq11_e1342_d_b5: f64 = (p.p29 * eq11_e1340_d_b5);
        let eq11_e1342_d_b6: f64 = (p.p29 * eq11_e1340_d_b6);
        let eq11_e1342_d_b7: f64 = (p.p29 * eq11_e1340_d_b7);
        let eq11_e1342_d_b8: f64 = (p.p29 * eq11_e1340_d_b8);
        let eq11_e1342_d_b9: f64 = (p.p29 * eq11_e1340_d_b9);
        let eq11_e1342_d_b10: f64 = (p.p29 * eq11_e1340_d_b10);
        let eq11_e1342_d_b11: f64 = (p.p29 * eq11_e1340_d_b11);
        let eq11_e1342_d_b12: f64 = (p.p29 * eq11_e1340_d_b12);
        let eq11_e1342_d_b13: f64 = (p.p29 * eq11_e1340_d_b13);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16, eq11_e1342_d_b0, eq11_e1342_d_b1, eq11_e1342_d_b2, eq11_e1342_d_b3, eq11_e1342_d_b4, eq11_e1342_d_b5, eq11_e1342_d_b6, eq11_e1342_d_b7, eq11_e1342_d_b8, eq11_e1342_d_b9, eq11_e1342_d_b10, eq11_e1342_d_b11, eq11_e1342_d_b12, eq11_e1342_d_b13, eq11_e1342_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16];
        let eq11_reactive_branch_derivatives: [f64; 14] = [eq11_e1344_d_b0, eq11_e1344_d_b1, eq11_e1344_d_b2, eq11_e1344_d_b3, eq11_e1344_d_b4, eq11_e1344_d_b5, eq11_e1344_d_b6, eq11_e1344_d_b7, eq11_e1344_d_b8, eq11_e1344_d_b9, eq11_e1344_d_b10, eq11_e1344_d_b11, eq11_e1344_d_b12, eq11_e1344_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16, eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13, eq12_e1370_q,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = (((-s.dn[57][0]) * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = (((-s.dn[57][1]) * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = (((-s.dn[57][2]) * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = (((-s.dn[57][3]) * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = (((-s.dn[57][4]) * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = (((-s.dn[57][5]) * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = (((-s.dn[57][6]) * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = (((-s.dn[57][7]) * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = (((-s.dn[57][8]) * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = (((-s.dn[57][9]) * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = (((-s.dn[57][10]) * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = (((-s.dn[57][11]) * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = (((-s.dn[57][12]) * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = (((-s.dn[57][13]) * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = (((-s.dn[57][14]) * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = (((-s.dn[57][15]) * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = (((-s.dn[57][16]) * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1355_d_b0: f64 = (((-s.db[57][0]) * s.v[378]) + (eq12_e1353 * s.db[378][0]));
        let eq12_e1355_d_b1: f64 = (((-s.db[57][1]) * s.v[378]) + (eq12_e1353 * s.db[378][1]));
        let eq12_e1355_d_b2: f64 = (((-s.db[57][2]) * s.v[378]) + (eq12_e1353 * s.db[378][2]));
        let eq12_e1355_d_b3: f64 = (((-s.db[57][3]) * s.v[378]) + (eq12_e1353 * s.db[378][3]));
        let eq12_e1355_d_b4: f64 = (((-s.db[57][4]) * s.v[378]) + (eq12_e1353 * s.db[378][4]));
        let eq12_e1355_d_b5: f64 = (((-s.db[57][5]) * s.v[378]) + (eq12_e1353 * s.db[378][5]));
        let eq12_e1355_d_b6: f64 = (((-s.db[57][6]) * s.v[378]) + (eq12_e1353 * s.db[378][6]));
        let eq12_e1355_d_b7: f64 = (((-s.db[57][7]) * s.v[378]) + (eq12_e1353 * s.db[378][7]));
        let eq12_e1355_d_b8: f64 = (((-s.db[57][8]) * s.v[378]) + (eq12_e1353 * s.db[378][8]));
        let eq12_e1355_d_b9: f64 = (((-s.db[57][9]) * s.v[378]) + (eq12_e1353 * s.db[378][9]));
        let eq12_e1355_d_b10: f64 = (((-s.db[57][10]) * s.v[378]) + (eq12_e1353 * s.db[378][10]));
        let eq12_e1355_d_b11: f64 = (((-s.db[57][11]) * s.v[378]) + (eq12_e1353 * s.db[378][11]));
        let eq12_e1355_d_b12: f64 = (((-s.db[57][12]) * s.v[378]) + (eq12_e1353 * s.db[378][12]));
        let eq12_e1355_d_b13: f64 = (((-s.db[57][13]) * s.v[378]) + (eq12_e1353 * s.db[378][13]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1357_d_b0: f64 = (eq12_e1355_d_b0 * s.v[46]);
        let eq12_e1357_d_b1: f64 = (eq12_e1355_d_b1 * s.v[46]);
        let eq12_e1357_d_b2: f64 = (eq12_e1355_d_b2 * s.v[46]);
        let eq12_e1357_d_b3: f64 = (eq12_e1355_d_b3 * s.v[46]);
        let eq12_e1357_d_b4: f64 = (eq12_e1355_d_b4 * s.v[46]);
        let eq12_e1357_d_b5: f64 = (eq12_e1355_d_b5 * s.v[46]);
        let eq12_e1357_d_b6: f64 = (eq12_e1355_d_b6 * s.v[46]);
        let eq12_e1357_d_b7: f64 = (eq12_e1355_d_b7 * s.v[46]);
        let eq12_e1357_d_b8: f64 = (eq12_e1355_d_b8 * s.v[46]);
        let eq12_e1357_d_b9: f64 = (eq12_e1355_d_b9 * s.v[46]);
        let eq12_e1357_d_b10: f64 = (eq12_e1355_d_b10 * s.v[46]);
        let eq12_e1357_d_b11: f64 = (eq12_e1355_d_b11 * s.v[46]);
        let eq12_e1357_d_b12: f64 = (eq12_e1355_d_b12 * s.v[46]);
        let eq12_e1357_d_b13: f64 = (eq12_e1355_d_b13 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1359_d_b0: f64 = (eq12_e1357_d_b0 * s.v[29]);
        let eq12_e1359_d_b1: f64 = (eq12_e1357_d_b1 * s.v[29]);
        let eq12_e1359_d_b2: f64 = (eq12_e1357_d_b2 * s.v[29]);
        let eq12_e1359_d_b3: f64 = (eq12_e1357_d_b3 * s.v[29]);
        let eq12_e1359_d_b4: f64 = (eq12_e1357_d_b4 * s.v[29]);
        let eq12_e1359_d_b5: f64 = (eq12_e1357_d_b5 * s.v[29]);
        let eq12_e1359_d_b6: f64 = (eq12_e1357_d_b6 * s.v[29]);
        let eq12_e1359_d_b7: f64 = (eq12_e1357_d_b7 * s.v[29]);
        let eq12_e1359_d_b8: f64 = (eq12_e1357_d_b8 * s.v[29]);
        let eq12_e1359_d_b9: f64 = (eq12_e1357_d_b9 * s.v[29]);
        let eq12_e1359_d_b10: f64 = (eq12_e1357_d_b10 * s.v[29]);
        let eq12_e1359_d_b11: f64 = (eq12_e1357_d_b11 * s.v[29]);
        let eq12_e1359_d_b12: f64 = (eq12_e1357_d_b12 * s.v[29]);
        let eq12_e1359_d_b13: f64 = (eq12_e1357_d_b13 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1361_d_b0: f64 = (eq12_e1359_d_b0 * p.p2);
        let eq12_e1361_d_b1: f64 = (eq12_e1359_d_b1 * p.p2);
        let eq12_e1361_d_b2: f64 = (eq12_e1359_d_b2 * p.p2);
        let eq12_e1361_d_b3: f64 = (eq12_e1359_d_b3 * p.p2);
        let eq12_e1361_d_b4: f64 = (eq12_e1359_d_b4 * p.p2);
        let eq12_e1361_d_b5: f64 = (eq12_e1359_d_b5 * p.p2);
        let eq12_e1361_d_b6: f64 = (eq12_e1359_d_b6 * p.p2);
        let eq12_e1361_d_b7: f64 = (eq12_e1359_d_b7 * p.p2);
        let eq12_e1361_d_b8: f64 = (eq12_e1359_d_b8 * p.p2);
        let eq12_e1361_d_b9: f64 = (eq12_e1359_d_b9 * p.p2);
        let eq12_e1361_d_b10: f64 = (eq12_e1359_d_b10 * p.p2);
        let eq12_e1361_d_b11: f64 = (eq12_e1359_d_b11 * p.p2);
        let eq12_e1361_d_b12: f64 = (eq12_e1359_d_b12 * p.p2);
        let eq12_e1361_d_b13: f64 = (eq12_e1359_d_b13 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1363_d_b0: f64 = (eq12_e1361_d_b0 * s.v[30]);
        let eq12_e1363_d_b1: f64 = (eq12_e1361_d_b1 * s.v[30]);
        let eq12_e1363_d_b2: f64 = (eq12_e1361_d_b2 * s.v[30]);
        let eq12_e1363_d_b3: f64 = (eq12_e1361_d_b3 * s.v[30]);
        let eq12_e1363_d_b4: f64 = (eq12_e1361_d_b4 * s.v[30]);
        let eq12_e1363_d_b5: f64 = (eq12_e1361_d_b5 * s.v[30]);
        let eq12_e1363_d_b6: f64 = (eq12_e1361_d_b6 * s.v[30]);
        let eq12_e1363_d_b7: f64 = (eq12_e1361_d_b7 * s.v[30]);
        let eq12_e1363_d_b8: f64 = (eq12_e1361_d_b8 * s.v[30]);
        let eq12_e1363_d_b9: f64 = (eq12_e1361_d_b9 * s.v[30]);
        let eq12_e1363_d_b10: f64 = (eq12_e1361_d_b10 * s.v[30]);
        let eq12_e1363_d_b11: f64 = (eq12_e1361_d_b11 * s.v[30]);
        let eq12_e1363_d_b12: f64 = (eq12_e1361_d_b12 * s.v[30]);
        let eq12_e1363_d_b13: f64 = (eq12_e1361_d_b13 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1365_d_b0: f64 = (eq12_e1363_d_b0 * (nv15 - 0.0));
        let eq12_e1365_d_b1: f64 = (eq12_e1363_d_b1 * (nv15 - 0.0));
        let eq12_e1365_d_b2: f64 = (eq12_e1363_d_b2 * (nv15 - 0.0));
        let eq12_e1365_d_b3: f64 = (eq12_e1363_d_b3 * (nv15 - 0.0));
        let eq12_e1365_d_b4: f64 = (eq12_e1363_d_b4 * (nv15 - 0.0));
        let eq12_e1365_d_b5: f64 = (eq12_e1363_d_b5 * (nv15 - 0.0));
        let eq12_e1365_d_b6: f64 = (eq12_e1363_d_b6 * (nv15 - 0.0));
        let eq12_e1365_d_b7: f64 = (eq12_e1363_d_b7 * (nv15 - 0.0));
        let eq12_e1365_d_b8: f64 = (eq12_e1363_d_b8 * (nv15 - 0.0));
        let eq12_e1365_d_b9: f64 = (eq12_e1363_d_b9 * (nv15 - 0.0));
        let eq12_e1365_d_b10: f64 = (eq12_e1363_d_b10 * (nv15 - 0.0));
        let eq12_e1365_d_b11: f64 = (eq12_e1363_d_b11 * (nv15 - 0.0));
        let eq12_e1365_d_b12: f64 = (eq12_e1363_d_b12 * (nv15 - 0.0));
        let eq12_e1365_d_b13: f64 = (eq12_e1363_d_b13 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1366_d_b0: f64 = (0.5 * eq12_e1365_d_b0);
        let eq12_e1366_d_b1: f64 = (0.5 * eq12_e1365_d_b1);
        let eq12_e1366_d_b2: f64 = (0.5 * eq12_e1365_d_b2);
        let eq12_e1366_d_b3: f64 = (0.5 * eq12_e1365_d_b3);
        let eq12_e1366_d_b4: f64 = (0.5 * eq12_e1365_d_b4);
        let eq12_e1366_d_b5: f64 = (0.5 * eq12_e1365_d_b5);
        let eq12_e1366_d_b6: f64 = (0.5 * eq12_e1365_d_b6);
        let eq12_e1366_d_b7: f64 = (0.5 * eq12_e1365_d_b7);
        let eq12_e1366_d_b8: f64 = (0.5 * eq12_e1365_d_b8);
        let eq12_e1366_d_b9: f64 = (0.5 * eq12_e1365_d_b9);
        let eq12_e1366_d_b10: f64 = (0.5 * eq12_e1365_d_b10);
        let eq12_e1366_d_b11: f64 = (0.5 * eq12_e1365_d_b11);
        let eq12_e1366_d_b12: f64 = (0.5 * eq12_e1365_d_b12);
        let eq12_e1366_d_b13: f64 = (0.5 * eq12_e1365_d_b13);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        let eq12_e1368_d_b0: f64 = (p.p29 * eq12_e1366_d_b0);
        let eq12_e1368_d_b1: f64 = (p.p29 * eq12_e1366_d_b1);
        let eq12_e1368_d_b2: f64 = (p.p29 * eq12_e1366_d_b2);
        let eq12_e1368_d_b3: f64 = (p.p29 * eq12_e1366_d_b3);
        let eq12_e1368_d_b4: f64 = (p.p29 * eq12_e1366_d_b4);
        let eq12_e1368_d_b5: f64 = (p.p29 * eq12_e1366_d_b5);
        let eq12_e1368_d_b6: f64 = (p.p29 * eq12_e1366_d_b6);
        let eq12_e1368_d_b7: f64 = (p.p29 * eq12_e1366_d_b7);
        let eq12_e1368_d_b8: f64 = (p.p29 * eq12_e1366_d_b8);
        let eq12_e1368_d_b9: f64 = (p.p29 * eq12_e1366_d_b9);
        let eq12_e1368_d_b10: f64 = (p.p29 * eq12_e1366_d_b10);
        let eq12_e1368_d_b11: f64 = (p.p29 * eq12_e1366_d_b11);
        let eq12_e1368_d_b12: f64 = (p.p29 * eq12_e1366_d_b12);
        let eq12_e1368_d_b13: f64 = (p.p29 * eq12_e1366_d_b13);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16, eq12_e1368_d_b0, eq12_e1368_d_b1, eq12_e1368_d_b2, eq12_e1368_d_b3, eq12_e1368_d_b4, eq12_e1368_d_b5, eq12_e1368_d_b6, eq12_e1368_d_b7, eq12_e1368_d_b8, eq12_e1368_d_b9, eq12_e1368_d_b10, eq12_e1368_d_b11, eq12_e1368_d_b12, eq12_e1368_d_b13, eq12_e1368_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16];
        let eq12_reactive_branch_derivatives: [f64; 14] = [eq12_e1370_d_b0, eq12_e1370_d_b1, eq12_e1370_d_b2, eq12_e1370_d_b3, eq12_e1370_d_b4, eq12_e1370_d_b5, eq12_e1370_d_b6, eq12_e1370_d_b7, eq12_e1370_d_b8, eq12_e1370_d_b9, eq12_e1370_d_b10, eq12_e1370_d_b11, eq12_e1370_d_b12, eq12_e1370_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428_q: f64 = s.v[787];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            nodes,
            &s.dn[787],
            branches,
            &s.db[787],
            multiplicity,
        );
        let eq20_e1430_q: f64 = s.v[785];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &s.dn[785],
            branches,
            &s.db[785],
            multiplicity,
        );
        let eq21_e1432_q: f64 = s.v[786];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &s.dn[786],
            branches,
            &s.db[786],
            multiplicity,
        );
        let eq22_e1435: f64 = (-s.v[187]);
        let eq22_e1437: f64 = (eq22_e1435 * s.v[223]);
        let eq22_e1437_d_n0: f64 = (((-s.dn[187][0]) * s.v[223]) + (eq22_e1435 * s.dn[223][0]));
        let eq22_e1437_d_n1: f64 = (((-s.dn[187][1]) * s.v[223]) + (eq22_e1435 * s.dn[223][1]));
        let eq22_e1437_d_n2: f64 = (((-s.dn[187][2]) * s.v[223]) + (eq22_e1435 * s.dn[223][2]));
        let eq22_e1437_d_n3: f64 = (((-s.dn[187][3]) * s.v[223]) + (eq22_e1435 * s.dn[223][3]));
        let eq22_e1437_d_n4: f64 = (((-s.dn[187][4]) * s.v[223]) + (eq22_e1435 * s.dn[223][4]));
        let eq22_e1437_d_n5: f64 = (((-s.dn[187][5]) * s.v[223]) + (eq22_e1435 * s.dn[223][5]));
        let eq22_e1437_d_n6: f64 = (((-s.dn[187][6]) * s.v[223]) + (eq22_e1435 * s.dn[223][6]));
        let eq22_e1437_d_n7: f64 = (((-s.dn[187][7]) * s.v[223]) + (eq22_e1435 * s.dn[223][7]));
        let eq22_e1437_d_n8: f64 = (((-s.dn[187][8]) * s.v[223]) + (eq22_e1435 * s.dn[223][8]));
        let eq22_e1437_d_n9: f64 = (((-s.dn[187][9]) * s.v[223]) + (eq22_e1435 * s.dn[223][9]));
        let eq22_e1437_d_n10: f64 = (((-s.dn[187][10]) * s.v[223]) + (eq22_e1435 * s.dn[223][10]));
        let eq22_e1437_d_n11: f64 = (((-s.dn[187][11]) * s.v[223]) + (eq22_e1435 * s.dn[223][11]));
        let eq22_e1437_d_n12: f64 = (((-s.dn[187][12]) * s.v[223]) + (eq22_e1435 * s.dn[223][12]));
        let eq22_e1437_d_n13: f64 = (((-s.dn[187][13]) * s.v[223]) + (eq22_e1435 * s.dn[223][13]));
        let eq22_e1437_d_n14: f64 = (((-s.dn[187][14]) * s.v[223]) + (eq22_e1435 * s.dn[223][14]));
        let eq22_e1437_d_n15: f64 = (((-s.dn[187][15]) * s.v[223]) + (eq22_e1435 * s.dn[223][15]));
        let eq22_e1437_d_n16: f64 = (((-s.dn[187][16]) * s.v[223]) + (eq22_e1435 * s.dn[223][16]));
        let eq22_e1437_d_b0: f64 = (((-s.db[187][0]) * s.v[223]) + (eq22_e1435 * s.db[223][0]));
        let eq22_e1437_d_b1: f64 = (((-s.db[187][1]) * s.v[223]) + (eq22_e1435 * s.db[223][1]));
        let eq22_e1437_d_b2: f64 = (((-s.db[187][2]) * s.v[223]) + (eq22_e1435 * s.db[223][2]));
        let eq22_e1437_d_b3: f64 = (((-s.db[187][3]) * s.v[223]) + (eq22_e1435 * s.db[223][3]));
        let eq22_e1437_d_b4: f64 = (((-s.db[187][4]) * s.v[223]) + (eq22_e1435 * s.db[223][4]));
        let eq22_e1437_d_b5: f64 = (((-s.db[187][5]) * s.v[223]) + (eq22_e1435 * s.db[223][5]));
        let eq22_e1437_d_b6: f64 = (((-s.db[187][6]) * s.v[223]) + (eq22_e1435 * s.db[223][6]));
        let eq22_e1437_d_b7: f64 = (((-s.db[187][7]) * s.v[223]) + (eq22_e1435 * s.db[223][7]));
        let eq22_e1437_d_b8: f64 = (((-s.db[187][8]) * s.v[223]) + (eq22_e1435 * s.db[223][8]));
        let eq22_e1437_d_b9: f64 = (((-s.db[187][9]) * s.v[223]) + (eq22_e1435 * s.db[223][9]));
        let eq22_e1437_d_b10: f64 = (((-s.db[187][10]) * s.v[223]) + (eq22_e1435 * s.db[223][10]));
        let eq22_e1437_d_b11: f64 = (((-s.db[187][11]) * s.v[223]) + (eq22_e1435 * s.db[223][11]));
        let eq22_e1437_d_b12: f64 = (((-s.db[187][12]) * s.v[223]) + (eq22_e1435 * s.db[223][12]));
        let eq22_e1437_d_b13: f64 = (((-s.db[187][13]) * s.v[223]) + (eq22_e1435 * s.db[223][13]));
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_e1439_d_b0: f64 = (p.p29 * eq22_e1437_d_b0);
        let eq22_e1439_d_b1: f64 = (p.p29 * eq22_e1437_d_b1);
        let eq22_e1439_d_b2: f64 = (p.p29 * eq22_e1437_d_b2);
        let eq22_e1439_d_b3: f64 = (p.p29 * eq22_e1437_d_b3);
        let eq22_e1439_d_b4: f64 = (p.p29 * eq22_e1437_d_b4);
        let eq22_e1439_d_b5: f64 = (p.p29 * eq22_e1437_d_b5);
        let eq22_e1439_d_b6: f64 = (p.p29 * eq22_e1437_d_b6);
        let eq22_e1439_d_b7: f64 = (p.p29 * eq22_e1437_d_b7);
        let eq22_e1439_d_b8: f64 = (p.p29 * eq22_e1437_d_b8);
        let eq22_e1439_d_b9: f64 = (p.p29 * eq22_e1437_d_b9);
        let eq22_e1439_d_b10: f64 = (p.p29 * eq22_e1437_d_b10);
        let eq22_e1439_d_b11: f64 = (p.p29 * eq22_e1437_d_b11);
        let eq22_e1439_d_b12: f64 = (p.p29 * eq22_e1437_d_b12);
        let eq22_e1439_d_b13: f64 = (p.p29 * eq22_e1437_d_b13);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, eq22_e1439_d_n1, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, eq22_e1439_d_n15, eq22_e1439_d_n16];
        let eq22_reactive_branch_derivatives: [f64; 14] = [eq22_e1439_d_b0, eq22_e1439_d_b1, eq22_e1439_d_b2, eq22_e1439_d_b3, eq22_e1439_d_b4, eq22_e1439_d_b5, eq22_e1439_d_b6, eq22_e1439_d_b7, eq22_e1439_d_b8, eq22_e1439_d_b9, eq22_e1439_d_b10, eq22_e1439_d_b11, eq22_e1439_d_b12, eq22_e1439_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-s.v[187]);
        let eq23_e1444: f64 = (eq23_e1442 * s.v[224]);
        let eq23_e1444_d_n0: f64 = (((-s.dn[187][0]) * s.v[224]) + (eq23_e1442 * s.dn[224][0]));
        let eq23_e1444_d_n1: f64 = (((-s.dn[187][1]) * s.v[224]) + (eq23_e1442 * s.dn[224][1]));
        let eq23_e1444_d_n2: f64 = (((-s.dn[187][2]) * s.v[224]) + (eq23_e1442 * s.dn[224][2]));
        let eq23_e1444_d_n3: f64 = (((-s.dn[187][3]) * s.v[224]) + (eq23_e1442 * s.dn[224][3]));
        let eq23_e1444_d_n4: f64 = (((-s.dn[187][4]) * s.v[224]) + (eq23_e1442 * s.dn[224][4]));
        let eq23_e1444_d_n5: f64 = (((-s.dn[187][5]) * s.v[224]) + (eq23_e1442 * s.dn[224][5]));
        let eq23_e1444_d_n6: f64 = (((-s.dn[187][6]) * s.v[224]) + (eq23_e1442 * s.dn[224][6]));
        let eq23_e1444_d_n7: f64 = (((-s.dn[187][7]) * s.v[224]) + (eq23_e1442 * s.dn[224][7]));
        let eq23_e1444_d_n8: f64 = (((-s.dn[187][8]) * s.v[224]) + (eq23_e1442 * s.dn[224][8]));
        let eq23_e1444_d_n9: f64 = (((-s.dn[187][9]) * s.v[224]) + (eq23_e1442 * s.dn[224][9]));
        let eq23_e1444_d_n10: f64 = (((-s.dn[187][10]) * s.v[224]) + (eq23_e1442 * s.dn[224][10]));
        let eq23_e1444_d_n11: f64 = (((-s.dn[187][11]) * s.v[224]) + (eq23_e1442 * s.dn[224][11]));
        let eq23_e1444_d_n12: f64 = (((-s.dn[187][12]) * s.v[224]) + (eq23_e1442 * s.dn[224][12]));
        let eq23_e1444_d_n13: f64 = (((-s.dn[187][13]) * s.v[224]) + (eq23_e1442 * s.dn[224][13]));
        let eq23_e1444_d_n14: f64 = (((-s.dn[187][14]) * s.v[224]) + (eq23_e1442 * s.dn[224][14]));
        let eq23_e1444_d_n15: f64 = (((-s.dn[187][15]) * s.v[224]) + (eq23_e1442 * s.dn[224][15]));
        let eq23_e1444_d_n16: f64 = (((-s.dn[187][16]) * s.v[224]) + (eq23_e1442 * s.dn[224][16]));
        let eq23_e1444_d_b0: f64 = (((-s.db[187][0]) * s.v[224]) + (eq23_e1442 * s.db[224][0]));
        let eq23_e1444_d_b1: f64 = (((-s.db[187][1]) * s.v[224]) + (eq23_e1442 * s.db[224][1]));
        let eq23_e1444_d_b2: f64 = (((-s.db[187][2]) * s.v[224]) + (eq23_e1442 * s.db[224][2]));
        let eq23_e1444_d_b3: f64 = (((-s.db[187][3]) * s.v[224]) + (eq23_e1442 * s.db[224][3]));
        let eq23_e1444_d_b4: f64 = (((-s.db[187][4]) * s.v[224]) + (eq23_e1442 * s.db[224][4]));
        let eq23_e1444_d_b5: f64 = (((-s.db[187][5]) * s.v[224]) + (eq23_e1442 * s.db[224][5]));
        let eq23_e1444_d_b6: f64 = (((-s.db[187][6]) * s.v[224]) + (eq23_e1442 * s.db[224][6]));
        let eq23_e1444_d_b7: f64 = (((-s.db[187][7]) * s.v[224]) + (eq23_e1442 * s.db[224][7]));
        let eq23_e1444_d_b8: f64 = (((-s.db[187][8]) * s.v[224]) + (eq23_e1442 * s.db[224][8]));
        let eq23_e1444_d_b9: f64 = (((-s.db[187][9]) * s.v[224]) + (eq23_e1442 * s.db[224][9]));
        let eq23_e1444_d_b10: f64 = (((-s.db[187][10]) * s.v[224]) + (eq23_e1442 * s.db[224][10]));
        let eq23_e1444_d_b11: f64 = (((-s.db[187][11]) * s.v[224]) + (eq23_e1442 * s.db[224][11]));
        let eq23_e1444_d_b12: f64 = (((-s.db[187][12]) * s.v[224]) + (eq23_e1442 * s.db[224][12]));
        let eq23_e1444_d_b13: f64 = (((-s.db[187][13]) * s.v[224]) + (eq23_e1442 * s.db[224][13]));
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_e1446_d_b0: f64 = (p.p29 * eq23_e1444_d_b0);
        let eq23_e1446_d_b1: f64 = (p.p29 * eq23_e1444_d_b1);
        let eq23_e1446_d_b2: f64 = (p.p29 * eq23_e1444_d_b2);
        let eq23_e1446_d_b3: f64 = (p.p29 * eq23_e1444_d_b3);
        let eq23_e1446_d_b4: f64 = (p.p29 * eq23_e1444_d_b4);
        let eq23_e1446_d_b5: f64 = (p.p29 * eq23_e1444_d_b5);
        let eq23_e1446_d_b6: f64 = (p.p29 * eq23_e1444_d_b6);
        let eq23_e1446_d_b7: f64 = (p.p29 * eq23_e1444_d_b7);
        let eq23_e1446_d_b8: f64 = (p.p29 * eq23_e1444_d_b8);
        let eq23_e1446_d_b9: f64 = (p.p29 * eq23_e1444_d_b9);
        let eq23_e1446_d_b10: f64 = (p.p29 * eq23_e1444_d_b10);
        let eq23_e1446_d_b11: f64 = (p.p29 * eq23_e1444_d_b11);
        let eq23_e1446_d_b12: f64 = (p.p29 * eq23_e1444_d_b12);
        let eq23_e1446_d_b13: f64 = (p.p29 * eq23_e1444_d_b13);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, eq23_e1446_d_n1, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, eq23_e1446_d_n15, eq23_e1446_d_n16];
        let eq23_reactive_branch_derivatives: [f64; 14] = [eq23_e1446_d_b0, eq23_e1446_d_b1, eq23_e1446_d_b2, eq23_e1446_d_b3, eq23_e1446_d_b4, eq23_e1446_d_b5, eq23_e1446_d_b6, eq23_e1446_d_b7, eq23_e1446_d_b8, eq23_e1446_d_b9, eq23_e1446_d_b10, eq23_e1446_d_b11, eq23_e1446_d_b12, eq23_e1446_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq24_e1449: f64 = (-s.v[187]);
        let eq24_e1451: f64 = (eq24_e1449 * s.v[221]);
        let eq24_e1451_d_n0: f64 = (((-s.dn[187][0]) * s.v[221]) + (eq24_e1449 * s.dn[221][0]));
        let eq24_e1451_d_n1: f64 = (((-s.dn[187][1]) * s.v[221]) + (eq24_e1449 * s.dn[221][1]));
        let eq24_e1451_d_n2: f64 = (((-s.dn[187][2]) * s.v[221]) + (eq24_e1449 * s.dn[221][2]));
        let eq24_e1451_d_n3: f64 = (((-s.dn[187][3]) * s.v[221]) + (eq24_e1449 * s.dn[221][3]));
        let eq24_e1451_d_n4: f64 = (((-s.dn[187][4]) * s.v[221]) + (eq24_e1449 * s.dn[221][4]));
        let eq24_e1451_d_n5: f64 = (((-s.dn[187][5]) * s.v[221]) + (eq24_e1449 * s.dn[221][5]));
        let eq24_e1451_d_n6: f64 = (((-s.dn[187][6]) * s.v[221]) + (eq24_e1449 * s.dn[221][6]));
        let eq24_e1451_d_n7: f64 = (((-s.dn[187][7]) * s.v[221]) + (eq24_e1449 * s.dn[221][7]));
        let eq24_e1451_d_n8: f64 = (((-s.dn[187][8]) * s.v[221]) + (eq24_e1449 * s.dn[221][8]));
        let eq24_e1451_d_n9: f64 = (((-s.dn[187][9]) * s.v[221]) + (eq24_e1449 * s.dn[221][9]));
        let eq24_e1451_d_n10: f64 = (((-s.dn[187][10]) * s.v[221]) + (eq24_e1449 * s.dn[221][10]));
        let eq24_e1451_d_n11: f64 = (((-s.dn[187][11]) * s.v[221]) + (eq24_e1449 * s.dn[221][11]));
        let eq24_e1451_d_n12: f64 = (((-s.dn[187][12]) * s.v[221]) + (eq24_e1449 * s.dn[221][12]));
        let eq24_e1451_d_n13: f64 = (((-s.dn[187][13]) * s.v[221]) + (eq24_e1449 * s.dn[221][13]));
        let eq24_e1451_d_n14: f64 = (((-s.dn[187][14]) * s.v[221]) + (eq24_e1449 * s.dn[221][14]));
        let eq24_e1451_d_n15: f64 = (((-s.dn[187][15]) * s.v[221]) + (eq24_e1449 * s.dn[221][15]));
        let eq24_e1451_d_n16: f64 = (((-s.dn[187][16]) * s.v[221]) + (eq24_e1449 * s.dn[221][16]));
        let eq24_e1451_d_b0: f64 = (((-s.db[187][0]) * s.v[221]) + (eq24_e1449 * s.db[221][0]));
        let eq24_e1451_d_b1: f64 = (((-s.db[187][1]) * s.v[221]) + (eq24_e1449 * s.db[221][1]));
        let eq24_e1451_d_b2: f64 = (((-s.db[187][2]) * s.v[221]) + (eq24_e1449 * s.db[221][2]));
        let eq24_e1451_d_b3: f64 = (((-s.db[187][3]) * s.v[221]) + (eq24_e1449 * s.db[221][3]));
        let eq24_e1451_d_b4: f64 = (((-s.db[187][4]) * s.v[221]) + (eq24_e1449 * s.db[221][4]));
        let eq24_e1451_d_b5: f64 = (((-s.db[187][5]) * s.v[221]) + (eq24_e1449 * s.db[221][5]));
        let eq24_e1451_d_b6: f64 = (((-s.db[187][6]) * s.v[221]) + (eq24_e1449 * s.db[221][6]));
        let eq24_e1451_d_b7: f64 = (((-s.db[187][7]) * s.v[221]) + (eq24_e1449 * s.db[221][7]));
        let eq24_e1451_d_b8: f64 = (((-s.db[187][8]) * s.v[221]) + (eq24_e1449 * s.db[221][8]));
        let eq24_e1451_d_b9: f64 = (((-s.db[187][9]) * s.v[221]) + (eq24_e1449 * s.db[221][9]));
        let eq24_e1451_d_b10: f64 = (((-s.db[187][10]) * s.v[221]) + (eq24_e1449 * s.db[221][10]));
        let eq24_e1451_d_b11: f64 = (((-s.db[187][11]) * s.v[221]) + (eq24_e1449 * s.db[221][11]));
        let eq24_e1451_d_b12: f64 = (((-s.db[187][12]) * s.v[221]) + (eq24_e1449 * s.db[221][12]));
        let eq24_e1451_d_b13: f64 = (((-s.db[187][13]) * s.v[221]) + (eq24_e1449 * s.db[221][13]));
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
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
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_e1453_d_b0: f64 = (p.p29 * eq24_e1451_d_b0);
        let eq24_e1453_d_b1: f64 = (p.p29 * eq24_e1451_d_b1);
        let eq24_e1453_d_b2: f64 = (p.p29 * eq24_e1451_d_b2);
        let eq24_e1453_d_b3: f64 = (p.p29 * eq24_e1451_d_b3);
        let eq24_e1453_d_b4: f64 = (p.p29 * eq24_e1451_d_b4);
        let eq24_e1453_d_b5: f64 = (p.p29 * eq24_e1451_d_b5);
        let eq24_e1453_d_b6: f64 = (p.p29 * eq24_e1451_d_b6);
        let eq24_e1453_d_b7: f64 = (p.p29 * eq24_e1451_d_b7);
        let eq24_e1453_d_b8: f64 = (p.p29 * eq24_e1451_d_b8);
        let eq24_e1453_d_b9: f64 = (p.p29 * eq24_e1451_d_b9);
        let eq24_e1453_d_b10: f64 = (p.p29 * eq24_e1451_d_b10);
        let eq24_e1453_d_b11: f64 = (p.p29 * eq24_e1451_d_b11);
        let eq24_e1453_d_b12: f64 = (p.p29 * eq24_e1451_d_b12);
        let eq24_e1453_d_b13: f64 = (p.p29 * eq24_e1451_d_b13);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, eq24_e1453_d_n1, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, eq24_e1453_d_n15, eq24_e1453_d_n16];
        let eq24_reactive_branch_derivatives: [f64; 14] = [eq24_e1453_d_b0, eq24_e1453_d_b1, eq24_e1453_d_b2, eq24_e1453_d_b3, eq24_e1453_d_b4, eq24_e1453_d_b5, eq24_e1453_d_b6, eq24_e1453_d_b7, eq24_e1453_d_b8, eq24_e1453_d_b9, eq24_e1453_d_b10, eq24_e1453_d_b11, eq24_e1453_d_b12, eq24_e1453_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16, eq55_e1708_d_b0, eq55_e1708_d_b1, eq55_e1708_d_b2, eq55_e1708_d_b3, eq55_e1708_d_b4, eq55_e1708_d_b5, eq55_e1708_d_b6, eq55_e1708_d_b7, eq55_e1708_d_b8, eq55_e1708_d_b9, eq55_e1708_d_b10, eq55_e1708_d_b11, eq55_e1708_d_b12, eq55_e1708_d_b13, eq55_e1708_q, eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16, eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13,) = {
    if s.b[1621] {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1699_d_b0: f64 = ((s.db[390][0] * s.v[747]) + (s.v[390] * s.db[747][0]));
        let eq55_e1699_d_b1: f64 = ((s.db[390][1] * s.v[747]) + (s.v[390] * s.db[747][1]));
        let eq55_e1699_d_b2: f64 = ((s.db[390][2] * s.v[747]) + (s.v[390] * s.db[747][2]));
        let eq55_e1699_d_b3: f64 = ((s.db[390][3] * s.v[747]) + (s.v[390] * s.db[747][3]));
        let eq55_e1699_d_b4: f64 = ((s.db[390][4] * s.v[747]) + (s.v[390] * s.db[747][4]));
        let eq55_e1699_d_b5: f64 = ((s.db[390][5] * s.v[747]) + (s.v[390] * s.db[747][5]));
        let eq55_e1699_d_b6: f64 = ((s.db[390][6] * s.v[747]) + (s.v[390] * s.db[747][6]));
        let eq55_e1699_d_b7: f64 = ((s.db[390][7] * s.v[747]) + (s.v[390] * s.db[747][7]));
        let eq55_e1699_d_b8: f64 = ((s.db[390][8] * s.v[747]) + (s.v[390] * s.db[747][8]));
        let eq55_e1699_d_b9: f64 = ((s.db[390][9] * s.v[747]) + (s.v[390] * s.db[747][9]));
        let eq55_e1699_d_b10: f64 = ((s.db[390][10] * s.v[747]) + (s.v[390] * s.db[747][10]));
        let eq55_e1699_d_b11: f64 = ((s.db[390][11] * s.v[747]) + (s.v[390] * s.db[747][11]));
        let eq55_e1699_d_b12: f64 = ((s.db[390][12] * s.v[747]) + (s.v[390] * s.db[747][12]));
        let eq55_e1699_d_b13: f64 = ((s.db[390][13] * s.v[747]) + (s.v[390] * s.db[747][13]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1702_d_b0: f64 = ((s.db[390][0] * s.v[748]) + (s.v[390] * s.db[748][0]));
        let eq55_e1702_d_b1: f64 = ((s.db[390][1] * s.v[748]) + (s.v[390] * s.db[748][1]));
        let eq55_e1702_d_b2: f64 = ((s.db[390][2] * s.v[748]) + (s.v[390] * s.db[748][2]));
        let eq55_e1702_d_b3: f64 = ((s.db[390][3] * s.v[748]) + (s.v[390] * s.db[748][3]));
        let eq55_e1702_d_b4: f64 = ((s.db[390][4] * s.v[748]) + (s.v[390] * s.db[748][4]));
        let eq55_e1702_d_b5: f64 = ((s.db[390][5] * s.v[748]) + (s.v[390] * s.db[748][5]));
        let eq55_e1702_d_b6: f64 = ((s.db[390][6] * s.v[748]) + (s.v[390] * s.db[748][6]));
        let eq55_e1702_d_b7: f64 = ((s.db[390][7] * s.v[748]) + (s.v[390] * s.db[748][7]));
        let eq55_e1702_d_b8: f64 = ((s.db[390][8] * s.v[748]) + (s.v[390] * s.db[748][8]));
        let eq55_e1702_d_b9: f64 = ((s.db[390][9] * s.v[748]) + (s.v[390] * s.db[748][9]));
        let eq55_e1702_d_b10: f64 = ((s.db[390][10] * s.v[748]) + (s.v[390] * s.db[748][10]));
        let eq55_e1702_d_b11: f64 = ((s.db[390][11] * s.v[748]) + (s.v[390] * s.db[748][11]));
        let eq55_e1702_d_b12: f64 = ((s.db[390][12] * s.v[748]) + (s.v[390] * s.db[748][12]));
        let eq55_e1702_d_b13: f64 = ((s.db[390][13] * s.v[748]) + (s.v[390] * s.db[748][13]));
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1702_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1702_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1702_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1702_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1702_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1702_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1702_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1702_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1702_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1702_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1702_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1702_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1702_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1702_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1702_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1702_d_n16);
        let eq55_e1704_d_b0: f64 = (eq55_e1699_d_b0 + eq55_e1702_d_b0);
        let eq55_e1704_d_b1: f64 = (eq55_e1699_d_b1 + eq55_e1702_d_b1);
        let eq55_e1704_d_b2: f64 = (eq55_e1699_d_b2 + eq55_e1702_d_b2);
        let eq55_e1704_d_b3: f64 = (eq55_e1699_d_b3 + eq55_e1702_d_b3);
        let eq55_e1704_d_b4: f64 = (eq55_e1699_d_b4 + eq55_e1702_d_b4);
        let eq55_e1704_d_b5: f64 = (eq55_e1699_d_b5 + eq55_e1702_d_b5);
        let eq55_e1704_d_b6: f64 = (eq55_e1699_d_b6 + eq55_e1702_d_b6);
        let eq55_e1704_d_b7: f64 = (eq55_e1699_d_b7 + eq55_e1702_d_b7);
        let eq55_e1704_d_b8: f64 = (eq55_e1699_d_b8 + eq55_e1702_d_b8);
        let eq55_e1704_d_b9: f64 = (eq55_e1699_d_b9 + eq55_e1702_d_b9);
        let eq55_e1704_d_b10: f64 = (eq55_e1699_d_b10 + eq55_e1702_d_b10);
        let eq55_e1704_d_b11: f64 = (eq55_e1699_d_b11 + eq55_e1702_d_b11);
        let eq55_e1704_d_b12: f64 = (eq55_e1699_d_b12 + eq55_e1702_d_b12);
        let eq55_e1704_d_b13: f64 = (eq55_e1699_d_b13 + eq55_e1702_d_b13);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        let eq55_e1706_d_b0: f64 = (eq55_e1704_d_b0 - s.db[749][0]);
        let eq55_e1706_d_b1: f64 = (eq55_e1704_d_b1 - s.db[749][1]);
        let eq55_e1706_d_b2: f64 = (eq55_e1704_d_b2 - s.db[749][2]);
        let eq55_e1706_d_b3: f64 = (eq55_e1704_d_b3 - s.db[749][3]);
        let eq55_e1706_d_b4: f64 = (eq55_e1704_d_b4 - s.db[749][4]);
        let eq55_e1706_d_b5: f64 = (eq55_e1704_d_b5 - s.db[749][5]);
        let eq55_e1706_d_b6: f64 = (eq55_e1704_d_b6 - s.db[749][6]);
        let eq55_e1706_d_b7: f64 = (eq55_e1704_d_b7 - s.db[749][7]);
        let eq55_e1706_d_b8: f64 = (eq55_e1704_d_b8 - s.db[749][8]);
        let eq55_e1706_d_b9: f64 = (eq55_e1704_d_b9 - s.db[749][9]);
        let eq55_e1706_d_b10: f64 = (eq55_e1704_d_b10 - s.db[749][10]);
        let eq55_e1706_d_b11: f64 = (eq55_e1704_d_b11 - s.db[749][11]);
        let eq55_e1706_d_b12: f64 = (eq55_e1704_d_b12 - s.db[749][12]);
        let eq55_e1706_d_b13: f64 = (eq55_e1704_d_b13 - s.db[749][13]);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16, eq55_e1706_d_b0, eq55_e1706_d_b1, eq55_e1706_d_b2, eq55_e1706_d_b3, eq55_e1706_d_b4, eq55_e1706_d_b5, eq55_e1706_d_b6, eq55_e1706_d_b7, eq55_e1706_d_b8, eq55_e1706_d_b9, eq55_e1706_d_b10, eq55_e1706_d_b11, eq55_e1706_d_b12, eq55_e1706_d_b13, eq55_e1706_q, eq55_e1702_d_n0, eq55_e1702_d_n1, eq55_e1702_d_n2, eq55_e1702_d_n3, eq55_e1702_d_n4, eq55_e1702_d_n5, eq55_e1702_d_n6, eq55_e1702_d_n7, eq55_e1702_d_n8, eq55_e1702_d_n9, eq55_e1702_d_n10, eq55_e1702_d_n11, eq55_e1702_d_n12, eq55_e1702_d_n13, eq55_e1702_d_n14, eq55_e1702_d_n15, eq55_e1702_d_n16, eq55_e1702_d_b0, eq55_e1702_d_b1, eq55_e1702_d_b2, eq55_e1702_d_b3, eq55_e1702_d_b4, eq55_e1702_d_b5, eq55_e1702_d_b6, eq55_e1702_d_b7, eq55_e1702_d_b8, eq55_e1702_d_b9, eq55_e1702_d_b10, eq55_e1702_d_b11, eq55_e1702_d_b12, eq55_e1702_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 14] = [eq55_e1708_q_d_b0, eq55_e1708_q_d_b1, eq55_e1708_q_d_b2, eq55_e1708_q_d_b3, eq55_e1708_q_d_b4, eq55_e1708_q_d_b5, eq55_e1708_q_d_b6, eq55_e1708_q_d_b7, eq55_e1708_q_d_b8, eq55_e1708_q_d_b9, eq55_e1708_q_d_b10, eq55_e1708_q_d_b11, eq55_e1708_q_d_b12, eq55_e1708_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16, eq71_e1841_d_b0, eq71_e1841_d_b1, eq71_e1841_d_b2, eq71_e1841_d_b3, eq71_e1841_d_b4, eq71_e1841_d_b5, eq71_e1841_d_b6, eq71_e1841_d_b7, eq71_e1841_d_b8, eq71_e1841_d_b9, eq71_e1841_d_b10, eq71_e1841_d_b11, eq71_e1841_d_b12, eq71_e1841_d_b13, eq71_e1841_q, eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16, eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13,) = {
    if s.b[1627] {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (s.v[187] * eq71_e1837);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1837) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq71_e1839_d_b0: f64 = ((s.db[187][0] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq71_e1839_d_b1: f64 = ((s.db[187][1] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq71_e1839_d_b2: f64 = ((s.db[187][2] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq71_e1839_d_b3: f64 = ((s.db[187][3] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq71_e1839_d_b4: f64 = ((s.db[187][4] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq71_e1839_d_b5: f64 = ((s.db[187][5] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq71_e1839_d_b6: f64 = ((s.db[187][6] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq71_e1839_d_b7: f64 = ((s.db[187][7] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq71_e1839_d_b8: f64 = ((s.db[187][8] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq71_e1839_d_b9: f64 = ((s.db[187][9] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq71_e1839_d_b10: f64 = ((s.db[187][10] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq71_e1839_d_b11: f64 = ((s.db[187][11] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq71_e1839_d_b12: f64 = ((s.db[187][12] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq71_e1839_d_b13: f64 = ((s.db[187][13] * eq71_e1837) + (s.v[187] * (p.p29 * s.db[330][13])));
        let eq71_e1839_q: f64 = (s.v[187] * eq71_e1838_q);
        let eq71_e1839_q_d_n0: f64 = ((s.dn[187][0] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq71_e1839_q_d_n1: f64 = ((s.dn[187][1] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq71_e1839_q_d_n2: f64 = ((s.dn[187][2] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq71_e1839_q_d_n3: f64 = ((s.dn[187][3] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq71_e1839_q_d_n4: f64 = ((s.dn[187][4] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq71_e1839_q_d_n5: f64 = ((s.dn[187][5] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq71_e1839_q_d_n6: f64 = ((s.dn[187][6] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq71_e1839_q_d_n7: f64 = ((s.dn[187][7] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq71_e1839_q_d_n8: f64 = ((s.dn[187][8] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq71_e1839_q_d_n9: f64 = ((s.dn[187][9] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq71_e1839_q_d_n10: f64 = ((s.dn[187][10] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq71_e1839_q_d_n11: f64 = ((s.dn[187][11] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq71_e1839_q_d_n12: f64 = ((s.dn[187][12] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq71_e1839_q_d_n13: f64 = ((s.dn[187][13] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq71_e1839_q_d_n14: f64 = ((s.dn[187][14] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq71_e1839_q_d_n15: f64 = ((s.dn[187][15] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq71_e1839_q_d_n16: f64 = ((s.dn[187][16] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq71_e1839_q_d_b0: f64 = ((s.db[187][0] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq71_e1839_q_d_b1: f64 = ((s.db[187][1] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq71_e1839_q_d_b2: f64 = ((s.db[187][2] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq71_e1839_q_d_b3: f64 = ((s.db[187][3] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq71_e1839_q_d_b4: f64 = ((s.db[187][4] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq71_e1839_q_d_b5: f64 = ((s.db[187][5] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq71_e1839_q_d_b6: f64 = ((s.db[187][6] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq71_e1839_q_d_b7: f64 = ((s.db[187][7] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq71_e1839_q_d_b8: f64 = ((s.db[187][8] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq71_e1839_q_d_b9: f64 = ((s.db[187][9] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq71_e1839_q_d_b10: f64 = ((s.db[187][10] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq71_e1839_q_d_b11: f64 = ((s.db[187][11] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq71_e1839_q_d_b12: f64 = ((s.db[187][12] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq71_e1839_q_d_b13: f64 = ((s.db[187][13] * eq71_e1838_q) + (s.v[187] * (p.p29 * s.db[330][13])));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16, eq71_e1839_d_b0, eq71_e1839_d_b1, eq71_e1839_d_b2, eq71_e1839_d_b3, eq71_e1839_d_b4, eq71_e1839_d_b5, eq71_e1839_d_b6, eq71_e1839_d_b7, eq71_e1839_d_b8, eq71_e1839_d_b9, eq71_e1839_d_b10, eq71_e1839_d_b11, eq71_e1839_d_b12, eq71_e1839_d_b13, eq71_e1839_q, eq71_e1839_q_d_n0, eq71_e1839_q_d_n1, eq71_e1839_q_d_n2, eq71_e1839_q_d_n3, eq71_e1839_q_d_n4, eq71_e1839_q_d_n5, eq71_e1839_q_d_n6, eq71_e1839_q_d_n7, eq71_e1839_q_d_n8, eq71_e1839_q_d_n9, eq71_e1839_q_d_n10, eq71_e1839_q_d_n11, eq71_e1839_q_d_n12, eq71_e1839_q_d_n13, eq71_e1839_q_d_n14, eq71_e1839_q_d_n15, eq71_e1839_q_d_n16, eq71_e1839_q_d_b0, eq71_e1839_q_d_b1, eq71_e1839_q_d_b2, eq71_e1839_q_d_b3, eq71_e1839_q_d_b4, eq71_e1839_q_d_b5, eq71_e1839_q_d_b6, eq71_e1839_q_d_b7, eq71_e1839_q_d_b8, eq71_e1839_q_d_b9, eq71_e1839_q_d_b10, eq71_e1839_q_d_b11, eq71_e1839_q_d_b12, eq71_e1839_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16];
        let eq71_reactive_branch_derivatives: [f64; 14] = [eq71_e1841_q_d_b0, eq71_e1841_q_d_b1, eq71_e1841_q_d_b2, eq71_e1841_q_d_b3, eq71_e1841_q_d_b4, eq71_e1841_q_d_b5, eq71_e1841_q_d_b6, eq71_e1841_q_d_b7, eq71_e1841_q_d_b8, eq71_e1841_q_d_b9, eq71_e1841_q_d_b10, eq71_e1841_q_d_b11, eq71_e1841_q_d_b12, eq71_e1841_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16, eq73_e1868_d_b0, eq73_e1868_d_b1, eq73_e1868_d_b2, eq73_e1868_d_b3, eq73_e1868_d_b4, eq73_e1868_d_b5, eq73_e1868_d_b6, eq73_e1868_d_b7, eq73_e1868_d_b8, eq73_e1868_d_b9, eq73_e1868_d_b10, eq73_e1868_d_b11, eq73_e1868_d_b12, eq73_e1868_d_b13, eq73_e1868_q, eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16, eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13,) = {
    if (s.b[1627] && s.b[1628]) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (s.v[187] * eq73_e1864);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1864) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq73_e1866_d_b0: f64 = ((s.db[187][0] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq73_e1866_d_b1: f64 = ((s.db[187][1] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq73_e1866_d_b2: f64 = ((s.db[187][2] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq73_e1866_d_b3: f64 = ((s.db[187][3] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq73_e1866_d_b4: f64 = ((s.db[187][4] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq73_e1866_d_b5: f64 = ((s.db[187][5] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq73_e1866_d_b6: f64 = ((s.db[187][6] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq73_e1866_d_b7: f64 = ((s.db[187][7] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq73_e1866_d_b8: f64 = ((s.db[187][8] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq73_e1866_d_b9: f64 = ((s.db[187][9] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq73_e1866_d_b10: f64 = ((s.db[187][10] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq73_e1866_d_b11: f64 = ((s.db[187][11] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq73_e1866_d_b12: f64 = ((s.db[187][12] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq73_e1866_d_b13: f64 = ((s.db[187][13] * eq73_e1864) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq73_e1866_q: f64 = (s.v[187] * eq73_e1865_q);
        let eq73_e1866_q_d_n0: f64 = ((s.dn[187][0] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq73_e1866_q_d_n1: f64 = ((s.dn[187][1] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq73_e1866_q_d_n2: f64 = ((s.dn[187][2] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq73_e1866_q_d_n3: f64 = ((s.dn[187][3] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq73_e1866_q_d_n4: f64 = ((s.dn[187][4] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq73_e1866_q_d_n5: f64 = ((s.dn[187][5] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq73_e1866_q_d_n6: f64 = ((s.dn[187][6] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq73_e1866_q_d_n7: f64 = ((s.dn[187][7] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq73_e1866_q_d_n8: f64 = ((s.dn[187][8] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq73_e1866_q_d_n9: f64 = ((s.dn[187][9] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq73_e1866_q_d_n10: f64 = ((s.dn[187][10] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq73_e1866_q_d_n11: f64 = ((s.dn[187][11] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq73_e1866_q_d_n12: f64 = ((s.dn[187][12] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq73_e1866_q_d_n13: f64 = ((s.dn[187][13] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq73_e1866_q_d_n14: f64 = ((s.dn[187][14] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq73_e1866_q_d_n15: f64 = ((s.dn[187][15] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq73_e1866_q_d_n16: f64 = ((s.dn[187][16] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq73_e1866_q_d_b0: f64 = ((s.db[187][0] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq73_e1866_q_d_b1: f64 = ((s.db[187][1] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq73_e1866_q_d_b2: f64 = ((s.db[187][2] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq73_e1866_q_d_b3: f64 = ((s.db[187][3] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq73_e1866_q_d_b4: f64 = ((s.db[187][4] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq73_e1866_q_d_b5: f64 = ((s.db[187][5] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq73_e1866_q_d_b6: f64 = ((s.db[187][6] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq73_e1866_q_d_b7: f64 = ((s.db[187][7] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq73_e1866_q_d_b8: f64 = ((s.db[187][8] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq73_e1866_q_d_b9: f64 = ((s.db[187][9] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq73_e1866_q_d_b10: f64 = ((s.db[187][10] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq73_e1866_q_d_b11: f64 = ((s.db[187][11] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq73_e1866_q_d_b12: f64 = ((s.db[187][12] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq73_e1866_q_d_b13: f64 = ((s.db[187][13] * eq73_e1865_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16, eq73_e1866_d_b0, eq73_e1866_d_b1, eq73_e1866_d_b2, eq73_e1866_d_b3, eq73_e1866_d_b4, eq73_e1866_d_b5, eq73_e1866_d_b6, eq73_e1866_d_b7, eq73_e1866_d_b8, eq73_e1866_d_b9, eq73_e1866_d_b10, eq73_e1866_d_b11, eq73_e1866_d_b12, eq73_e1866_d_b13, eq73_e1866_q, eq73_e1866_q_d_n0, eq73_e1866_q_d_n1, eq73_e1866_q_d_n2, eq73_e1866_q_d_n3, eq73_e1866_q_d_n4, eq73_e1866_q_d_n5, eq73_e1866_q_d_n6, eq73_e1866_q_d_n7, eq73_e1866_q_d_n8, eq73_e1866_q_d_n9, eq73_e1866_q_d_n10, eq73_e1866_q_d_n11, eq73_e1866_q_d_n12, eq73_e1866_q_d_n13, eq73_e1866_q_d_n14, eq73_e1866_q_d_n15, eq73_e1866_q_d_n16, eq73_e1866_q_d_b0, eq73_e1866_q_d_b1, eq73_e1866_q_d_b2, eq73_e1866_q_d_b3, eq73_e1866_q_d_b4, eq73_e1866_q_d_b5, eq73_e1866_q_d_b6, eq73_e1866_q_d_b7, eq73_e1866_q_d_b8, eq73_e1866_q_d_b9, eq73_e1866_q_d_b10, eq73_e1866_q_d_b11, eq73_e1866_q_d_b12, eq73_e1866_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16];
        let eq73_reactive_branch_derivatives: [f64; 14] = [eq73_e1868_q_d_b0, eq73_e1868_q_d_b1, eq73_e1868_q_d_b2, eq73_e1868_q_d_b3, eq73_e1868_q_d_b4, eq73_e1868_q_d_b5, eq73_e1868_q_d_b6, eq73_e1868_q_d_b7, eq73_e1868_q_d_b8, eq73_e1868_q_d_b9, eq73_e1868_q_d_b10, eq73_e1868_q_d_b11, eq73_e1868_q_d_b12, eq73_e1868_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_d_b0, eq76_e1908_d_b1, eq76_e1908_d_b2, eq76_e1908_d_b3, eq76_e1908_d_b4, eq76_e1908_d_b5, eq76_e1908_d_b6, eq76_e1908_d_b7, eq76_e1908_d_b8, eq76_e1908_d_b9, eq76_e1908_d_b10, eq76_e1908_d_b11, eq76_e1908_d_b12, eq76_e1908_d_b13, eq76_e1908_q, eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16, eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (s.v[187] * eq76_e1904);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1904) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq76_e1906_d_b0: f64 = ((s.db[187][0] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq76_e1906_d_b1: f64 = ((s.db[187][1] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq76_e1906_d_b2: f64 = ((s.db[187][2] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq76_e1906_d_b3: f64 = ((s.db[187][3] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq76_e1906_d_b4: f64 = ((s.db[187][4] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq76_e1906_d_b5: f64 = ((s.db[187][5] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq76_e1906_d_b6: f64 = ((s.db[187][6] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq76_e1906_d_b7: f64 = ((s.db[187][7] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq76_e1906_d_b8: f64 = ((s.db[187][8] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq76_e1906_d_b9: f64 = ((s.db[187][9] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq76_e1906_d_b10: f64 = ((s.db[187][10] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq76_e1906_d_b11: f64 = ((s.db[187][11] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq76_e1906_d_b12: f64 = ((s.db[187][12] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq76_e1906_d_b13: f64 = ((s.db[187][13] * eq76_e1904) + (s.v[187] * (p.p29 * s.db[330][13])));
        let eq76_e1906_q: f64 = (s.v[187] * eq76_e1905_q);
        let eq76_e1906_q_d_n0: f64 = ((s.dn[187][0] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][0])));
        let eq76_e1906_q_d_n1: f64 = ((s.dn[187][1] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][1])));
        let eq76_e1906_q_d_n2: f64 = ((s.dn[187][2] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][2])));
        let eq76_e1906_q_d_n3: f64 = ((s.dn[187][3] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][3])));
        let eq76_e1906_q_d_n4: f64 = ((s.dn[187][4] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][4])));
        let eq76_e1906_q_d_n5: f64 = ((s.dn[187][5] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][5])));
        let eq76_e1906_q_d_n6: f64 = ((s.dn[187][6] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][6])));
        let eq76_e1906_q_d_n7: f64 = ((s.dn[187][7] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][7])));
        let eq76_e1906_q_d_n8: f64 = ((s.dn[187][8] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][8])));
        let eq76_e1906_q_d_n9: f64 = ((s.dn[187][9] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][9])));
        let eq76_e1906_q_d_n10: f64 = ((s.dn[187][10] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][10])));
        let eq76_e1906_q_d_n11: f64 = ((s.dn[187][11] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][11])));
        let eq76_e1906_q_d_n12: f64 = ((s.dn[187][12] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][12])));
        let eq76_e1906_q_d_n13: f64 = ((s.dn[187][13] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][13])));
        let eq76_e1906_q_d_n14: f64 = ((s.dn[187][14] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][14])));
        let eq76_e1906_q_d_n15: f64 = ((s.dn[187][15] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][15])));
        let eq76_e1906_q_d_n16: f64 = ((s.dn[187][16] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.dn[330][16])));
        let eq76_e1906_q_d_b0: f64 = ((s.db[187][0] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][0])));
        let eq76_e1906_q_d_b1: f64 = ((s.db[187][1] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][1])));
        let eq76_e1906_q_d_b2: f64 = ((s.db[187][2] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][2])));
        let eq76_e1906_q_d_b3: f64 = ((s.db[187][3] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][3])));
        let eq76_e1906_q_d_b4: f64 = ((s.db[187][4] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][4])));
        let eq76_e1906_q_d_b5: f64 = ((s.db[187][5] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][5])));
        let eq76_e1906_q_d_b6: f64 = ((s.db[187][6] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][6])));
        let eq76_e1906_q_d_b7: f64 = ((s.db[187][7] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][7])));
        let eq76_e1906_q_d_b8: f64 = ((s.db[187][8] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][8])));
        let eq76_e1906_q_d_b9: f64 = ((s.db[187][9] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][9])));
        let eq76_e1906_q_d_b10: f64 = ((s.db[187][10] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][10])));
        let eq76_e1906_q_d_b11: f64 = ((s.db[187][11] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][11])));
        let eq76_e1906_q_d_b12: f64 = ((s.db[187][12] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][12])));
        let eq76_e1906_q_d_b13: f64 = ((s.db[187][13] * eq76_e1905_q) + (s.v[187] * (p.p29 * s.db[330][13])));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_d_b0, eq76_e1906_d_b1, eq76_e1906_d_b2, eq76_e1906_d_b3, eq76_e1906_d_b4, eq76_e1906_d_b5, eq76_e1906_d_b6, eq76_e1906_d_b7, eq76_e1906_d_b8, eq76_e1906_d_b9, eq76_e1906_d_b10, eq76_e1906_d_b11, eq76_e1906_d_b12, eq76_e1906_d_b13, eq76_e1906_q, eq76_e1906_q_d_n0, eq76_e1906_q_d_n1, eq76_e1906_q_d_n2, eq76_e1906_q_d_n3, eq76_e1906_q_d_n4, eq76_e1906_q_d_n5, eq76_e1906_q_d_n6, eq76_e1906_q_d_n7, eq76_e1906_q_d_n8, eq76_e1906_q_d_n9, eq76_e1906_q_d_n10, eq76_e1906_q_d_n11, eq76_e1906_q_d_n12, eq76_e1906_q_d_n13, eq76_e1906_q_d_n14, eq76_e1906_q_d_n15, eq76_e1906_q_d_n16, eq76_e1906_q_d_b0, eq76_e1906_q_d_b1, eq76_e1906_q_d_b2, eq76_e1906_q_d_b3, eq76_e1906_q_d_b4, eq76_e1906_q_d_b5, eq76_e1906_q_d_b6, eq76_e1906_q_d_b7, eq76_e1906_q_d_b8, eq76_e1906_q_d_b9, eq76_e1906_q_d_b10, eq76_e1906_q_d_b11, eq76_e1906_q_d_b12, eq76_e1906_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16];
        let eq76_reactive_branch_derivatives: [f64; 14] = [eq76_e1908_q_d_b0, eq76_e1908_q_d_b1, eq76_e1908_q_d_b2, eq76_e1908_q_d_b3, eq76_e1908_q_d_b4, eq76_e1908_q_d_b5, eq76_e1908_q_d_b6, eq76_e1908_q_d_b7, eq76_e1908_q_d_b8, eq76_e1908_q_d_b9, eq76_e1908_q_d_b10, eq76_e1908_q_d_b11, eq76_e1908_q_d_b12, eq76_e1908_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_d_b0, eq77_e1918_d_b1, eq77_e1918_d_b2, eq77_e1918_d_b3, eq77_e1918_d_b4, eq77_e1918_d_b5, eq77_e1918_d_b6, eq77_e1918_d_b7, eq77_e1918_d_b8, eq77_e1918_d_b9, eq77_e1918_d_b10, eq77_e1918_d_b11, eq77_e1918_d_b12, eq77_e1918_d_b13, eq77_e1918_q, eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16, eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13,) = {
    if (!s.b[1627]) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (s.v[187] * eq77_e1914);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1914) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq77_e1916_d_b0: f64 = ((s.db[187][0] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq77_e1916_d_b1: f64 = ((s.db[187][1] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq77_e1916_d_b2: f64 = ((s.db[187][2] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq77_e1916_d_b3: f64 = ((s.db[187][3] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq77_e1916_d_b4: f64 = ((s.db[187][4] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq77_e1916_d_b5: f64 = ((s.db[187][5] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq77_e1916_d_b6: f64 = ((s.db[187][6] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq77_e1916_d_b7: f64 = ((s.db[187][7] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq77_e1916_d_b8: f64 = ((s.db[187][8] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq77_e1916_d_b9: f64 = ((s.db[187][9] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq77_e1916_d_b10: f64 = ((s.db[187][10] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq77_e1916_d_b11: f64 = ((s.db[187][11] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq77_e1916_d_b12: f64 = ((s.db[187][12] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq77_e1916_d_b13: f64 = ((s.db[187][13] * eq77_e1914) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq77_e1916_q: f64 = (s.v[187] * eq77_e1915_q);
        let eq77_e1916_q_d_n0: f64 = ((s.dn[187][0] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq77_e1916_q_d_n1: f64 = ((s.dn[187][1] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq77_e1916_q_d_n2: f64 = ((s.dn[187][2] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq77_e1916_q_d_n3: f64 = ((s.dn[187][3] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq77_e1916_q_d_n4: f64 = ((s.dn[187][4] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq77_e1916_q_d_n5: f64 = ((s.dn[187][5] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq77_e1916_q_d_n6: f64 = ((s.dn[187][6] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq77_e1916_q_d_n7: f64 = ((s.dn[187][7] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq77_e1916_q_d_n8: f64 = ((s.dn[187][8] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq77_e1916_q_d_n9: f64 = ((s.dn[187][9] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq77_e1916_q_d_n10: f64 = ((s.dn[187][10] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq77_e1916_q_d_n11: f64 = ((s.dn[187][11] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq77_e1916_q_d_n12: f64 = ((s.dn[187][12] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq77_e1916_q_d_n13: f64 = ((s.dn[187][13] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq77_e1916_q_d_n14: f64 = ((s.dn[187][14] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq77_e1916_q_d_n15: f64 = ((s.dn[187][15] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq77_e1916_q_d_n16: f64 = ((s.dn[187][16] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq77_e1916_q_d_b0: f64 = ((s.db[187][0] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq77_e1916_q_d_b1: f64 = ((s.db[187][1] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq77_e1916_q_d_b2: f64 = ((s.db[187][2] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq77_e1916_q_d_b3: f64 = ((s.db[187][3] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq77_e1916_q_d_b4: f64 = ((s.db[187][4] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq77_e1916_q_d_b5: f64 = ((s.db[187][5] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq77_e1916_q_d_b6: f64 = ((s.db[187][6] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq77_e1916_q_d_b7: f64 = ((s.db[187][7] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq77_e1916_q_d_b8: f64 = ((s.db[187][8] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq77_e1916_q_d_b9: f64 = ((s.db[187][9] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq77_e1916_q_d_b10: f64 = ((s.db[187][10] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq77_e1916_q_d_b11: f64 = ((s.db[187][11] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq77_e1916_q_d_b12: f64 = ((s.db[187][12] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq77_e1916_q_d_b13: f64 = ((s.db[187][13] * eq77_e1915_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_d_b0, eq77_e1916_d_b1, eq77_e1916_d_b2, eq77_e1916_d_b3, eq77_e1916_d_b4, eq77_e1916_d_b5, eq77_e1916_d_b6, eq77_e1916_d_b7, eq77_e1916_d_b8, eq77_e1916_d_b9, eq77_e1916_d_b10, eq77_e1916_d_b11, eq77_e1916_d_b12, eq77_e1916_d_b13, eq77_e1916_q, eq77_e1916_q_d_n0, eq77_e1916_q_d_n1, eq77_e1916_q_d_n2, eq77_e1916_q_d_n3, eq77_e1916_q_d_n4, eq77_e1916_q_d_n5, eq77_e1916_q_d_n6, eq77_e1916_q_d_n7, eq77_e1916_q_d_n8, eq77_e1916_q_d_n9, eq77_e1916_q_d_n10, eq77_e1916_q_d_n11, eq77_e1916_q_d_n12, eq77_e1916_q_d_n13, eq77_e1916_q_d_n14, eq77_e1916_q_d_n15, eq77_e1916_q_d_n16, eq77_e1916_q_d_b0, eq77_e1916_q_d_b1, eq77_e1916_q_d_b2, eq77_e1916_q_d_b3, eq77_e1916_q_d_b4, eq77_e1916_q_d_b5, eq77_e1916_q_d_b6, eq77_e1916_q_d_b7, eq77_e1916_q_d_b8, eq77_e1916_q_d_b9, eq77_e1916_q_d_b10, eq77_e1916_q_d_b11, eq77_e1916_q_d_b12, eq77_e1916_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16];
        let eq77_reactive_branch_derivatives: [f64; 14] = [eq77_e1918_q_d_b0, eq77_e1918_q_d_b1, eq77_e1918_q_d_b2, eq77_e1918_q_d_b3, eq77_e1918_q_d_b4, eq77_e1918_q_d_b5, eq77_e1918_q_d_b6, eq77_e1918_q_d_b7, eq77_e1918_q_d_b8, eq77_e1918_q_d_b9, eq77_e1918_q_d_b10, eq77_e1918_q_d_b11, eq77_e1918_q_d_b12, eq77_e1918_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_d_b0, eq83_e1984_d_b1, eq83_e1984_d_b2, eq83_e1984_d_b3, eq83_e1984_d_b4, eq83_e1984_d_b5, eq83_e1984_d_b6, eq83_e1984_d_b7, eq83_e1984_d_b8, eq83_e1984_d_b9, eq83_e1984_d_b10, eq83_e1984_d_b11, eq83_e1984_d_b12, eq83_e1984_d_b13, eq83_e1984_q, eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16, eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13,) = {
    if s.b[1630] {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (s.v[187] * eq83_e1980);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1980) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq83_e1982_d_b0: f64 = ((s.db[187][0] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq83_e1982_d_b1: f64 = ((s.db[187][1] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq83_e1982_d_b2: f64 = ((s.db[187][2] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq83_e1982_d_b3: f64 = ((s.db[187][3] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq83_e1982_d_b4: f64 = ((s.db[187][4] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq83_e1982_d_b5: f64 = ((s.db[187][5] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq83_e1982_d_b6: f64 = ((s.db[187][6] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq83_e1982_d_b7: f64 = ((s.db[187][7] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq83_e1982_d_b8: f64 = ((s.db[187][8] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq83_e1982_d_b9: f64 = ((s.db[187][9] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq83_e1982_d_b10: f64 = ((s.db[187][10] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq83_e1982_d_b11: f64 = ((s.db[187][11] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq83_e1982_d_b12: f64 = ((s.db[187][12] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq83_e1982_d_b13: f64 = ((s.db[187][13] * eq83_e1980) + (s.v[187] * (p.p29 * s.db[334][13])));
        let eq83_e1982_q: f64 = (s.v[187] * eq83_e1981_q);
        let eq83_e1982_q_d_n0: f64 = ((s.dn[187][0] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][0])));
        let eq83_e1982_q_d_n1: f64 = ((s.dn[187][1] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][1])));
        let eq83_e1982_q_d_n2: f64 = ((s.dn[187][2] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][2])));
        let eq83_e1982_q_d_n3: f64 = ((s.dn[187][3] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][3])));
        let eq83_e1982_q_d_n4: f64 = ((s.dn[187][4] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][4])));
        let eq83_e1982_q_d_n5: f64 = ((s.dn[187][5] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][5])));
        let eq83_e1982_q_d_n6: f64 = ((s.dn[187][6] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][6])));
        let eq83_e1982_q_d_n7: f64 = ((s.dn[187][7] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][7])));
        let eq83_e1982_q_d_n8: f64 = ((s.dn[187][8] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][8])));
        let eq83_e1982_q_d_n9: f64 = ((s.dn[187][9] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][9])));
        let eq83_e1982_q_d_n10: f64 = ((s.dn[187][10] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][10])));
        let eq83_e1982_q_d_n11: f64 = ((s.dn[187][11] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][11])));
        let eq83_e1982_q_d_n12: f64 = ((s.dn[187][12] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][12])));
        let eq83_e1982_q_d_n13: f64 = ((s.dn[187][13] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][13])));
        let eq83_e1982_q_d_n14: f64 = ((s.dn[187][14] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][14])));
        let eq83_e1982_q_d_n15: f64 = ((s.dn[187][15] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][15])));
        let eq83_e1982_q_d_n16: f64 = ((s.dn[187][16] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.dn[334][16])));
        let eq83_e1982_q_d_b0: f64 = ((s.db[187][0] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][0])));
        let eq83_e1982_q_d_b1: f64 = ((s.db[187][1] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][1])));
        let eq83_e1982_q_d_b2: f64 = ((s.db[187][2] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][2])));
        let eq83_e1982_q_d_b3: f64 = ((s.db[187][3] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][3])));
        let eq83_e1982_q_d_b4: f64 = ((s.db[187][4] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][4])));
        let eq83_e1982_q_d_b5: f64 = ((s.db[187][5] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][5])));
        let eq83_e1982_q_d_b6: f64 = ((s.db[187][6] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][6])));
        let eq83_e1982_q_d_b7: f64 = ((s.db[187][7] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][7])));
        let eq83_e1982_q_d_b8: f64 = ((s.db[187][8] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][8])));
        let eq83_e1982_q_d_b9: f64 = ((s.db[187][9] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][9])));
        let eq83_e1982_q_d_b10: f64 = ((s.db[187][10] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][10])));
        let eq83_e1982_q_d_b11: f64 = ((s.db[187][11] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][11])));
        let eq83_e1982_q_d_b12: f64 = ((s.db[187][12] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][12])));
        let eq83_e1982_q_d_b13: f64 = ((s.db[187][13] * eq83_e1981_q) + (s.v[187] * (p.p29 * s.db[334][13])));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_d_b0, eq83_e1982_d_b1, eq83_e1982_d_b2, eq83_e1982_d_b3, eq83_e1982_d_b4, eq83_e1982_d_b5, eq83_e1982_d_b6, eq83_e1982_d_b7, eq83_e1982_d_b8, eq83_e1982_d_b9, eq83_e1982_d_b10, eq83_e1982_d_b11, eq83_e1982_d_b12, eq83_e1982_d_b13, eq83_e1982_q, eq83_e1982_q_d_n0, eq83_e1982_q_d_n1, eq83_e1982_q_d_n2, eq83_e1982_q_d_n3, eq83_e1982_q_d_n4, eq83_e1982_q_d_n5, eq83_e1982_q_d_n6, eq83_e1982_q_d_n7, eq83_e1982_q_d_n8, eq83_e1982_q_d_n9, eq83_e1982_q_d_n10, eq83_e1982_q_d_n11, eq83_e1982_q_d_n12, eq83_e1982_q_d_n13, eq83_e1982_q_d_n14, eq83_e1982_q_d_n15, eq83_e1982_q_d_n16, eq83_e1982_q_d_b0, eq83_e1982_q_d_b1, eq83_e1982_q_d_b2, eq83_e1982_q_d_b3, eq83_e1982_q_d_b4, eq83_e1982_q_d_b5, eq83_e1982_q_d_b6, eq83_e1982_q_d_b7, eq83_e1982_q_d_b8, eq83_e1982_q_d_b9, eq83_e1982_q_d_b10, eq83_e1982_q_d_b11, eq83_e1982_q_d_b12, eq83_e1982_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16];
        let eq83_reactive_branch_derivatives: [f64; 14] = [eq83_e1984_q_d_b0, eq83_e1984_q_d_b1, eq83_e1984_q_d_b2, eq83_e1984_q_d_b3, eq83_e1984_q_d_b4, eq83_e1984_q_d_b5, eq83_e1984_q_d_b6, eq83_e1984_q_d_b7, eq83_e1984_q_d_b8, eq83_e1984_q_d_b9, eq83_e1984_q_d_b10, eq83_e1984_q_d_b11, eq83_e1984_q_d_b12, eq83_e1984_q_d_b13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            nodes,
            &eq83_reactive_node_derivatives,
            branches,
            &eq83_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_d_b0, eq84_e1993_d_b1, eq84_e1993_d_b2, eq84_e1993_d_b3, eq84_e1993_d_b4, eq84_e1993_d_b5, eq84_e1993_d_b6, eq84_e1993_d_b7, eq84_e1993_d_b8, eq84_e1993_d_b9, eq84_e1993_d_b10, eq84_e1993_d_b11, eq84_e1993_d_b12, eq84_e1993_d_b13, eq84_e1993_q, eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16, eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13,) = {
    if s.b[1630] {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (s.v[187] * eq84_e1989);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][0])));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][1])));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][2])));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][3])));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][4])));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][5])));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][6])));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][7])));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][8])));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][9])));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][10])));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][11])));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][12])));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][13])));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][14])));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][15])));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1989) + (s.v[187] * (p.p29 * s.dn[338][16])));
        let eq84_e1991_d_b0: f64 = ((s.db[187][0] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][0])));
        let eq84_e1991_d_b1: f64 = ((s.db[187][1] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][1])));
        let eq84_e1991_d_b2: f64 = ((s.db[187][2] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][2])));
        let eq84_e1991_d_b3: f64 = ((s.db[187][3] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][3])));
        let eq84_e1991_d_b4: f64 = ((s.db[187][4] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][4])));
        let eq84_e1991_d_b5: f64 = ((s.db[187][5] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][5])));
        let eq84_e1991_d_b6: f64 = ((s.db[187][6] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][6])));
        let eq84_e1991_d_b7: f64 = ((s.db[187][7] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][7])));
        let eq84_e1991_d_b8: f64 = ((s.db[187][8] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][8])));
        let eq84_e1991_d_b9: f64 = ((s.db[187][9] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][9])));
        let eq84_e1991_d_b10: f64 = ((s.db[187][10] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][10])));
        let eq84_e1991_d_b11: f64 = ((s.db[187][11] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][11])));
        let eq84_e1991_d_b12: f64 = ((s.db[187][12] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][12])));
        let eq84_e1991_d_b13: f64 = ((s.db[187][13] * eq84_e1989) + (s.v[187] * (p.p29 * s.db[338][13])));
        let eq84_e1991_q: f64 = (s.v[187] * eq84_e1990_q);
        let eq84_e1991_q_d_n0: f64 = ((s.dn[187][0] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][0])));
        let eq84_e1991_q_d_n1: f64 = ((s.dn[187][1] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][1])));
        let eq84_e1991_q_d_n2: f64 = ((s.dn[187][2] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][2])));
        let eq84_e1991_q_d_n3: f64 = ((s.dn[187][3] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][3])));
        let eq84_e1991_q_d_n4: f64 = ((s.dn[187][4] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][4])));
        let eq84_e1991_q_d_n5: f64 = ((s.dn[187][5] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][5])));
        let eq84_e1991_q_d_n6: f64 = ((s.dn[187][6] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][6])));
        let eq84_e1991_q_d_n7: f64 = ((s.dn[187][7] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][7])));
        let eq84_e1991_q_d_n8: f64 = ((s.dn[187][8] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][8])));
        let eq84_e1991_q_d_n9: f64 = ((s.dn[187][9] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][9])));
        let eq84_e1991_q_d_n10: f64 = ((s.dn[187][10] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][10])));
        let eq84_e1991_q_d_n11: f64 = ((s.dn[187][11] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][11])));
        let eq84_e1991_q_d_n12: f64 = ((s.dn[187][12] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][12])));
        let eq84_e1991_q_d_n13: f64 = ((s.dn[187][13] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][13])));
        let eq84_e1991_q_d_n14: f64 = ((s.dn[187][14] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][14])));
        let eq84_e1991_q_d_n15: f64 = ((s.dn[187][15] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][15])));
        let eq84_e1991_q_d_n16: f64 = ((s.dn[187][16] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.dn[338][16])));
        let eq84_e1991_q_d_b0: f64 = ((s.db[187][0] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][0])));
        let eq84_e1991_q_d_b1: f64 = ((s.db[187][1] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][1])));
        let eq84_e1991_q_d_b2: f64 = ((s.db[187][2] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][2])));
        let eq84_e1991_q_d_b3: f64 = ((s.db[187][3] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][3])));
        let eq84_e1991_q_d_b4: f64 = ((s.db[187][4] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][4])));
        let eq84_e1991_q_d_b5: f64 = ((s.db[187][5] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][5])));
        let eq84_e1991_q_d_b6: f64 = ((s.db[187][6] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][6])));
        let eq84_e1991_q_d_b7: f64 = ((s.db[187][7] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][7])));
        let eq84_e1991_q_d_b8: f64 = ((s.db[187][8] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][8])));
        let eq84_e1991_q_d_b9: f64 = ((s.db[187][9] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][9])));
        let eq84_e1991_q_d_b10: f64 = ((s.db[187][10] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][10])));
        let eq84_e1991_q_d_b11: f64 = ((s.db[187][11] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][11])));
        let eq84_e1991_q_d_b12: f64 = ((s.db[187][12] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][12])));
        let eq84_e1991_q_d_b13: f64 = ((s.db[187][13] * eq84_e1990_q) + (s.v[187] * (p.p29 * s.db[338][13])));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_d_b0, eq84_e1991_d_b1, eq84_e1991_d_b2, eq84_e1991_d_b3, eq84_e1991_d_b4, eq84_e1991_d_b5, eq84_e1991_d_b6, eq84_e1991_d_b7, eq84_e1991_d_b8, eq84_e1991_d_b9, eq84_e1991_d_b10, eq84_e1991_d_b11, eq84_e1991_d_b12, eq84_e1991_d_b13, eq84_e1991_q, eq84_e1991_q_d_n0, eq84_e1991_q_d_n1, eq84_e1991_q_d_n2, eq84_e1991_q_d_n3, eq84_e1991_q_d_n4, eq84_e1991_q_d_n5, eq84_e1991_q_d_n6, eq84_e1991_q_d_n7, eq84_e1991_q_d_n8, eq84_e1991_q_d_n9, eq84_e1991_q_d_n10, eq84_e1991_q_d_n11, eq84_e1991_q_d_n12, eq84_e1991_q_d_n13, eq84_e1991_q_d_n14, eq84_e1991_q_d_n15, eq84_e1991_q_d_n16, eq84_e1991_q_d_b0, eq84_e1991_q_d_b1, eq84_e1991_q_d_b2, eq84_e1991_q_d_b3, eq84_e1991_q_d_b4, eq84_e1991_q_d_b5, eq84_e1991_q_d_b6, eq84_e1991_q_d_b7, eq84_e1991_q_d_b8, eq84_e1991_q_d_b9, eq84_e1991_q_d_b10, eq84_e1991_q_d_b11, eq84_e1991_q_d_b12, eq84_e1991_q_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16];
        let eq84_reactive_branch_derivatives: [f64; 14] = [eq84_e1993_q_d_b0, eq84_e1993_q_d_b1, eq84_e1993_q_d_b2, eq84_e1993_q_d_b3, eq84_e1993_q_d_b4, eq84_e1993_q_d_b5, eq84_e1993_q_d_b6, eq84_e1993_q_d_b7, eq84_e1993_q_d_b8, eq84_e1993_q_d_b9, eq84_e1993_q_d_b10, eq84_e1993_q_d_b11, eq84_e1993_q_d_b12, eq84_e1993_q_d_b13];
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
