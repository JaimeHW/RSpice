#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[173][11] * ddt_scale) + ((-p.p355) * ddt_scale));
        let __rspice_deriv_cse_1: f64 = ((s.dn[174][10] * ddt_scale) + ((-p.p355) * ddt_scale));
        let __rspice_deriv_cse_2: f64 = ((s.dn[175][11] * ddt_scale) + ((-p.p355) * ddt_scale));
        let __rspice_deriv_cse_3: f64 = ((s.dn[179][12] * ddt_scale) + ((-p.p355) * ddt_scale));
        let __rspice_deriv_cse_4: f64 = ((s.dn[180][11] * ddt_scale) + ((-p.p355) * ddt_scale));
        let __rspice_deriv_cse_5: f64 = ((s.dn[181][12] * ddt_scale) + ((-p.p355) * ddt_scale));
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29, eq98_e1304_d_b0, eq98_e1304_d_b1, eq98_e1304_d_b2, eq98_e1304_d_b3, eq98_e1304_d_b4, eq98_e1304_d_b5, eq98_e1304_d_b6, eq98_e1304_d_b7, eq98_e1304_d_b8, eq98_e1304_d_b9, eq98_e1304_d_b10, eq98_e1304_d_b11, eq98_e1304_d_b12, eq98_e1304_d_b13, eq98_e1304_d_b14, eq98_e1304_d_b15, eq98_e1304_d_b16, eq98_e1304_d_b17, eq98_e1304_d_b18, eq98_e1304_d_b19, eq98_e1304_d_b20, eq98_e1304_d_b21, eq98_e1304_d_b22, eq98_e1304_d_b23, eq98_e1304_d_b24, eq98_e1304_d_b25, eq98_e1304_d_b26, eq98_e1304_d_b27, eq98_e1304_d_b28, eq98_e1304_d_b29, eq98_e1304_d_b30, eq98_e1304_d_b31, eq98_e1304_d_b32, eq98_e1304_d_b33, eq98_e1304_d_b34, eq98_e1304_d_b35,) = {
    if s.b[1201] {
        let eq98_e1297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 84, s.v[173]);
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 85, eq98_e1300);
        let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);
        let eq98_e1302_d_n7: f64 = ((s.dn[173][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq98_e1302, (s.dn[173][0] * ddt_scale), (s.dn[173][1] * ddt_scale), (s.dn[173][2] * ddt_scale), (s.dn[173][3] * ddt_scale), (s.dn[173][4] * ddt_scale), (s.dn[173][5] * ddt_scale), (s.dn[173][6] * ddt_scale), eq98_e1302_d_n7, (s.dn[173][8] * ddt_scale), (s.dn[173][9] * ddt_scale), (s.dn[173][10] * ddt_scale), __rspice_deriv_cse_0, (s.dn[173][12] * ddt_scale), (s.dn[173][13] * ddt_scale), (s.dn[173][14] * ddt_scale), (s.dn[173][15] * ddt_scale), (s.dn[173][16] * ddt_scale), (s.dn[173][17] * ddt_scale), (s.dn[173][18] * ddt_scale), (s.dn[173][19] * ddt_scale), (s.dn[173][20] * ddt_scale), (s.dn[173][21] * ddt_scale), (s.dn[173][22] * ddt_scale), (s.dn[173][23] * ddt_scale), (s.dn[173][24] * ddt_scale), (s.dn[173][25] * ddt_scale), (s.dn[173][26] * ddt_scale), (s.dn[173][27] * ddt_scale), (s.dn[173][28] * ddt_scale), (s.dn[173][29] * ddt_scale), (s.db[173][0] * ddt_scale), (s.db[173][1] * ddt_scale), (s.db[173][2] * ddt_scale), (s.db[173][3] * ddt_scale), (s.db[173][4] * ddt_scale), (s.db[173][5] * ddt_scale), (s.db[173][6] * ddt_scale), (s.db[173][7] * ddt_scale), (s.db[173][8] * ddt_scale), (s.db[173][9] * ddt_scale), (s.db[173][10] * ddt_scale), (s.db[173][11] * ddt_scale), (s.db[173][12] * ddt_scale), (s.db[173][13] * ddt_scale), (s.db[173][14] * ddt_scale), (s.db[173][15] * ddt_scale), (s.db[173][16] * ddt_scale), (s.db[173][17] * ddt_scale), (s.db[173][18] * ddt_scale), (s.db[173][19] * ddt_scale), (s.db[173][20] * ddt_scale), (s.db[173][21] * ddt_scale), (s.db[173][22] * ddt_scale), (s.db[173][23] * ddt_scale), (s.db[173][24] * ddt_scale), (s.db[173][25] * ddt_scale), (s.db[173][26] * ddt_scale), (s.db[173][27] * ddt_scale), (s.db[173][28] * ddt_scale), (s.db[173][29] * ddt_scale), (s.db[173][30] * ddt_scale), (s.db[173][31] * ddt_scale), (s.db[173][32] * ddt_scale), (s.db[173][33] * ddt_scale), (s.db[173][34] * ddt_scale), (s.db[173][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_value: f64 = eq98_e1304;
        let eq98_node_derivatives: [f64; 30] = [eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29];
        let eq98_branch_derivatives: [f64; 36] = [eq98_e1304_d_b0, eq98_e1304_d_b1, eq98_e1304_d_b2, eq98_e1304_d_b3, eq98_e1304_d_b4, eq98_e1304_d_b5, eq98_e1304_d_b6, eq98_e1304_d_b7, eq98_e1304_d_b8, eq98_e1304_d_b9, eq98_e1304_d_b10, eq98_e1304_d_b11, eq98_e1304_d_b12, eq98_e1304_d_b13, eq98_e1304_d_b14, eq98_e1304_d_b15, eq98_e1304_d_b16, eq98_e1304_d_b17, eq98_e1304_d_b18, eq98_e1304_d_b19, eq98_e1304_d_b20, eq98_e1304_d_b21, eq98_e1304_d_b22, eq98_e1304_d_b23, eq98_e1304_d_b24, eq98_e1304_d_b25, eq98_e1304_d_b26, eq98_e1304_d_b27, eq98_e1304_d_b28, eq98_e1304_d_b29, eq98_e1304_d_b30, eq98_e1304_d_b31, eq98_e1304_d_b32, eq98_e1304_d_b33, eq98_e1304_d_b34, eq98_e1304_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq98_value),
            &eq98_node_derivatives,
            &eq98_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29, eq99_e1314_d_b0, eq99_e1314_d_b1, eq99_e1314_d_b2, eq99_e1314_d_b3, eq99_e1314_d_b4, eq99_e1314_d_b5, eq99_e1314_d_b6, eq99_e1314_d_b7, eq99_e1314_d_b8, eq99_e1314_d_b9, eq99_e1314_d_b10, eq99_e1314_d_b11, eq99_e1314_d_b12, eq99_e1314_d_b13, eq99_e1314_d_b14, eq99_e1314_d_b15, eq99_e1314_d_b16, eq99_e1314_d_b17, eq99_e1314_d_b18, eq99_e1314_d_b19, eq99_e1314_d_b20, eq99_e1314_d_b21, eq99_e1314_d_b22, eq99_e1314_d_b23, eq99_e1314_d_b24, eq99_e1314_d_b25, eq99_e1314_d_b26, eq99_e1314_d_b27, eq99_e1314_d_b28, eq99_e1314_d_b29, eq99_e1314_d_b30, eq99_e1314_d_b31, eq99_e1314_d_b32, eq99_e1314_d_b33, eq99_e1314_d_b34, eq99_e1314_d_b35,) = {
    if s.b[1201] {
        let eq99_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 86, s.v[174]);
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 87, eq99_e1310);
        let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);
        let eq99_e1312_d_n7: f64 = ((s.dn[174][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq99_e1312, (s.dn[174][0] * ddt_scale), (s.dn[174][1] * ddt_scale), (s.dn[174][2] * ddt_scale), (s.dn[174][3] * ddt_scale), (s.dn[174][4] * ddt_scale), (s.dn[174][5] * ddt_scale), (s.dn[174][6] * ddt_scale), eq99_e1312_d_n7, (s.dn[174][8] * ddt_scale), (s.dn[174][9] * ddt_scale), __rspice_deriv_cse_1, (s.dn[174][11] * ddt_scale), (s.dn[174][12] * ddt_scale), (s.dn[174][13] * ddt_scale), (s.dn[174][14] * ddt_scale), (s.dn[174][15] * ddt_scale), (s.dn[174][16] * ddt_scale), (s.dn[174][17] * ddt_scale), (s.dn[174][18] * ddt_scale), (s.dn[174][19] * ddt_scale), (s.dn[174][20] * ddt_scale), (s.dn[174][21] * ddt_scale), (s.dn[174][22] * ddt_scale), (s.dn[174][23] * ddt_scale), (s.dn[174][24] * ddt_scale), (s.dn[174][25] * ddt_scale), (s.dn[174][26] * ddt_scale), (s.dn[174][27] * ddt_scale), (s.dn[174][28] * ddt_scale), (s.dn[174][29] * ddt_scale), (s.db[174][0] * ddt_scale), (s.db[174][1] * ddt_scale), (s.db[174][2] * ddt_scale), (s.db[174][3] * ddt_scale), (s.db[174][4] * ddt_scale), (s.db[174][5] * ddt_scale), (s.db[174][6] * ddt_scale), (s.db[174][7] * ddt_scale), (s.db[174][8] * ddt_scale), (s.db[174][9] * ddt_scale), (s.db[174][10] * ddt_scale), (s.db[174][11] * ddt_scale), (s.db[174][12] * ddt_scale), (s.db[174][13] * ddt_scale), (s.db[174][14] * ddt_scale), (s.db[174][15] * ddt_scale), (s.db[174][16] * ddt_scale), (s.db[174][17] * ddt_scale), (s.db[174][18] * ddt_scale), (s.db[174][19] * ddt_scale), (s.db[174][20] * ddt_scale), (s.db[174][21] * ddt_scale), (s.db[174][22] * ddt_scale), (s.db[174][23] * ddt_scale), (s.db[174][24] * ddt_scale), (s.db[174][25] * ddt_scale), (s.db[174][26] * ddt_scale), (s.db[174][27] * ddt_scale), (s.db[174][28] * ddt_scale), (s.db[174][29] * ddt_scale), (s.db[174][30] * ddt_scale), (s.db[174][31] * ddt_scale), (s.db[174][32] * ddt_scale), (s.db[174][33] * ddt_scale), (s.db[174][34] * ddt_scale), (s.db[174][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_value: f64 = eq99_e1314;
        let eq99_node_derivatives: [f64; 30] = [eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29];
        let eq99_branch_derivatives: [f64; 36] = [eq99_e1314_d_b0, eq99_e1314_d_b1, eq99_e1314_d_b2, eq99_e1314_d_b3, eq99_e1314_d_b4, eq99_e1314_d_b5, eq99_e1314_d_b6, eq99_e1314_d_b7, eq99_e1314_d_b8, eq99_e1314_d_b9, eq99_e1314_d_b10, eq99_e1314_d_b11, eq99_e1314_d_b12, eq99_e1314_d_b13, eq99_e1314_d_b14, eq99_e1314_d_b15, eq99_e1314_d_b16, eq99_e1314_d_b17, eq99_e1314_d_b18, eq99_e1314_d_b19, eq99_e1314_d_b20, eq99_e1314_d_b21, eq99_e1314_d_b22, eq99_e1314_d_b23, eq99_e1314_d_b24, eq99_e1314_d_b25, eq99_e1314_d_b26, eq99_e1314_d_b27, eq99_e1314_d_b28, eq99_e1314_d_b29, eq99_e1314_d_b30, eq99_e1314_d_b31, eq99_e1314_d_b32, eq99_e1314_d_b33, eq99_e1314_d_b34, eq99_e1314_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq99_value),
            &eq99_node_derivatives,
            &eq99_branch_derivatives,
            multiplicity,
        );
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29, eq100_e1324_d_b0, eq100_e1324_d_b1, eq100_e1324_d_b2, eq100_e1324_d_b3, eq100_e1324_d_b4, eq100_e1324_d_b5, eq100_e1324_d_b6, eq100_e1324_d_b7, eq100_e1324_d_b8, eq100_e1324_d_b9, eq100_e1324_d_b10, eq100_e1324_d_b11, eq100_e1324_d_b12, eq100_e1324_d_b13, eq100_e1324_d_b14, eq100_e1324_d_b15, eq100_e1324_d_b16, eq100_e1324_d_b17, eq100_e1324_d_b18, eq100_e1324_d_b19, eq100_e1324_d_b20, eq100_e1324_d_b21, eq100_e1324_d_b22, eq100_e1324_d_b23, eq100_e1324_d_b24, eq100_e1324_d_b25, eq100_e1324_d_b26, eq100_e1324_d_b27, eq100_e1324_d_b28, eq100_e1324_d_b29, eq100_e1324_d_b30, eq100_e1324_d_b31, eq100_e1324_d_b32, eq100_e1324_d_b33, eq100_e1324_d_b34, eq100_e1324_d_b35,) = {
    if s.b[1201] {
        let eq100_e1317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 88, s.v[175]);
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 89, eq100_e1320);
        let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);
        let eq100_e1322_d_n2: f64 = ((s.dn[175][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq100_e1322, (s.dn[175][0] * ddt_scale), (s.dn[175][1] * ddt_scale), eq100_e1322_d_n2, (s.dn[175][3] * ddt_scale), (s.dn[175][4] * ddt_scale), (s.dn[175][5] * ddt_scale), (s.dn[175][6] * ddt_scale), (s.dn[175][7] * ddt_scale), (s.dn[175][8] * ddt_scale), (s.dn[175][9] * ddt_scale), (s.dn[175][10] * ddt_scale), __rspice_deriv_cse_2, (s.dn[175][12] * ddt_scale), (s.dn[175][13] * ddt_scale), (s.dn[175][14] * ddt_scale), (s.dn[175][15] * ddt_scale), (s.dn[175][16] * ddt_scale), (s.dn[175][17] * ddt_scale), (s.dn[175][18] * ddt_scale), (s.dn[175][19] * ddt_scale), (s.dn[175][20] * ddt_scale), (s.dn[175][21] * ddt_scale), (s.dn[175][22] * ddt_scale), (s.dn[175][23] * ddt_scale), (s.dn[175][24] * ddt_scale), (s.dn[175][25] * ddt_scale), (s.dn[175][26] * ddt_scale), (s.dn[175][27] * ddt_scale), (s.dn[175][28] * ddt_scale), (s.dn[175][29] * ddt_scale), (s.db[175][0] * ddt_scale), (s.db[175][1] * ddt_scale), (s.db[175][2] * ddt_scale), (s.db[175][3] * ddt_scale), (s.db[175][4] * ddt_scale), (s.db[175][5] * ddt_scale), (s.db[175][6] * ddt_scale), (s.db[175][7] * ddt_scale), (s.db[175][8] * ddt_scale), (s.db[175][9] * ddt_scale), (s.db[175][10] * ddt_scale), (s.db[175][11] * ddt_scale), (s.db[175][12] * ddt_scale), (s.db[175][13] * ddt_scale), (s.db[175][14] * ddt_scale), (s.db[175][15] * ddt_scale), (s.db[175][16] * ddt_scale), (s.db[175][17] * ddt_scale), (s.db[175][18] * ddt_scale), (s.db[175][19] * ddt_scale), (s.db[175][20] * ddt_scale), (s.db[175][21] * ddt_scale), (s.db[175][22] * ddt_scale), (s.db[175][23] * ddt_scale), (s.db[175][24] * ddt_scale), (s.db[175][25] * ddt_scale), (s.db[175][26] * ddt_scale), (s.db[175][27] * ddt_scale), (s.db[175][28] * ddt_scale), (s.db[175][29] * ddt_scale), (s.db[175][30] * ddt_scale), (s.db[175][31] * ddt_scale), (s.db[175][32] * ddt_scale), (s.db[175][33] * ddt_scale), (s.db[175][34] * ddt_scale), (s.db[175][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1324;
        let eq100_node_derivatives: [f64; 30] = [eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29];
        let eq100_branch_derivatives: [f64; 36] = [eq100_e1324_d_b0, eq100_e1324_d_b1, eq100_e1324_d_b2, eq100_e1324_d_b3, eq100_e1324_d_b4, eq100_e1324_d_b5, eq100_e1324_d_b6, eq100_e1324_d_b7, eq100_e1324_d_b8, eq100_e1324_d_b9, eq100_e1324_d_b10, eq100_e1324_d_b11, eq100_e1324_d_b12, eq100_e1324_d_b13, eq100_e1324_d_b14, eq100_e1324_d_b15, eq100_e1324_d_b16, eq100_e1324_d_b17, eq100_e1324_d_b18, eq100_e1324_d_b19, eq100_e1324_d_b20, eq100_e1324_d_b21, eq100_e1324_d_b22, eq100_e1324_d_b23, eq100_e1324_d_b24, eq100_e1324_d_b25, eq100_e1324_d_b26, eq100_e1324_d_b27, eq100_e1324_d_b28, eq100_e1324_d_b29, eq100_e1324_d_b30, eq100_e1324_d_b31, eq100_e1324_d_b32, eq100_e1324_d_b33, eq100_e1324_d_b34, eq100_e1324_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(11),
            multiplicity * (eq100_value),
            &eq100_node_derivatives,
            &eq100_branch_derivatives,
            multiplicity,
        );
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29, eq102_e1338_d_b0, eq102_e1338_d_b1, eq102_e1338_d_b2, eq102_e1338_d_b3, eq102_e1338_d_b4, eq102_e1338_d_b5, eq102_e1338_d_b6, eq102_e1338_d_b7, eq102_e1338_d_b8, eq102_e1338_d_b9, eq102_e1338_d_b10, eq102_e1338_d_b11, eq102_e1338_d_b12, eq102_e1338_d_b13, eq102_e1338_d_b14, eq102_e1338_d_b15, eq102_e1338_d_b16, eq102_e1338_d_b17, eq102_e1338_d_b18, eq102_e1338_d_b19, eq102_e1338_d_b20, eq102_e1338_d_b21, eq102_e1338_d_b22, eq102_e1338_d_b23, eq102_e1338_d_b24, eq102_e1338_d_b25, eq102_e1338_d_b26, eq102_e1338_d_b27, eq102_e1338_d_b28, eq102_e1338_d_b29, eq102_e1338_d_b30, eq102_e1338_d_b31, eq102_e1338_d_b32, eq102_e1338_d_b33, eq102_e1338_d_b34, eq102_e1338_d_b35,) = {
    if s.b[1201] {
        let eq102_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 90, s.v[177]);
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 91, eq102_e1334);
        let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);
        let eq102_e1336_d_n7: f64 = ((s.dn[177][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq102_e1336_d_n9: f64 = ((s.dn[177][9] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq102_e1336, (s.dn[177][0] * ddt_scale), (s.dn[177][1] * ddt_scale), (s.dn[177][2] * ddt_scale), (s.dn[177][3] * ddt_scale), (s.dn[177][4] * ddt_scale), (s.dn[177][5] * ddt_scale), (s.dn[177][6] * ddt_scale), eq102_e1336_d_n7, (s.dn[177][8] * ddt_scale), eq102_e1336_d_n9, (s.dn[177][10] * ddt_scale), (s.dn[177][11] * ddt_scale), (s.dn[177][12] * ddt_scale), (s.dn[177][13] * ddt_scale), (s.dn[177][14] * ddt_scale), (s.dn[177][15] * ddt_scale), (s.dn[177][16] * ddt_scale), (s.dn[177][17] * ddt_scale), (s.dn[177][18] * ddt_scale), (s.dn[177][19] * ddt_scale), (s.dn[177][20] * ddt_scale), (s.dn[177][21] * ddt_scale), (s.dn[177][22] * ddt_scale), (s.dn[177][23] * ddt_scale), (s.dn[177][24] * ddt_scale), (s.dn[177][25] * ddt_scale), (s.dn[177][26] * ddt_scale), (s.dn[177][27] * ddt_scale), (s.dn[177][28] * ddt_scale), (s.dn[177][29] * ddt_scale), (s.db[177][0] * ddt_scale), (s.db[177][1] * ddt_scale), (s.db[177][2] * ddt_scale), (s.db[177][3] * ddt_scale), (s.db[177][4] * ddt_scale), (s.db[177][5] * ddt_scale), (s.db[177][6] * ddt_scale), (s.db[177][7] * ddt_scale), (s.db[177][8] * ddt_scale), (s.db[177][9] * ddt_scale), (s.db[177][10] * ddt_scale), (s.db[177][11] * ddt_scale), (s.db[177][12] * ddt_scale), (s.db[177][13] * ddt_scale), (s.db[177][14] * ddt_scale), (s.db[177][15] * ddt_scale), (s.db[177][16] * ddt_scale), (s.db[177][17] * ddt_scale), (s.db[177][18] * ddt_scale), (s.db[177][19] * ddt_scale), (s.db[177][20] * ddt_scale), (s.db[177][21] * ddt_scale), (s.db[177][22] * ddt_scale), (s.db[177][23] * ddt_scale), (s.db[177][24] * ddt_scale), (s.db[177][25] * ddt_scale), (s.db[177][26] * ddt_scale), (s.db[177][27] * ddt_scale), (s.db[177][28] * ddt_scale), (s.db[177][29] * ddt_scale), (s.db[177][30] * ddt_scale), (s.db[177][31] * ddt_scale), (s.db[177][32] * ddt_scale), (s.db[177][33] * ddt_scale), (s.db[177][34] * ddt_scale), (s.db[177][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1338;
        let eq102_node_derivatives: [f64; 30] = [eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29];
        let eq102_branch_derivatives: [f64; 36] = [eq102_e1338_d_b0, eq102_e1338_d_b1, eq102_e1338_d_b2, eq102_e1338_d_b3, eq102_e1338_d_b4, eq102_e1338_d_b5, eq102_e1338_d_b6, eq102_e1338_d_b7, eq102_e1338_d_b8, eq102_e1338_d_b9, eq102_e1338_d_b10, eq102_e1338_d_b11, eq102_e1338_d_b12, eq102_e1338_d_b13, eq102_e1338_d_b14, eq102_e1338_d_b15, eq102_e1338_d_b16, eq102_e1338_d_b17, eq102_e1338_d_b18, eq102_e1338_d_b19, eq102_e1338_d_b20, eq102_e1338_d_b21, eq102_e1338_d_b22, eq102_e1338_d_b23, eq102_e1338_d_b24, eq102_e1338_d_b25, eq102_e1338_d_b26, eq102_e1338_d_b27, eq102_e1338_d_b28, eq102_e1338_d_b29, eq102_e1338_d_b30, eq102_e1338_d_b31, eq102_e1338_d_b32, eq102_e1338_d_b33, eq102_e1338_d_b34, eq102_e1338_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq102_value),
            &eq102_node_derivatives,
            &eq102_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29, eq103_e1349_d_b0, eq103_e1349_d_b1, eq103_e1349_d_b2, eq103_e1349_d_b3, eq103_e1349_d_b4, eq103_e1349_d_b5, eq103_e1349_d_b6, eq103_e1349_d_b7, eq103_e1349_d_b8, eq103_e1349_d_b9, eq103_e1349_d_b10, eq103_e1349_d_b11, eq103_e1349_d_b12, eq103_e1349_d_b13, eq103_e1349_d_b14, eq103_e1349_d_b15, eq103_e1349_d_b16, eq103_e1349_d_b17, eq103_e1349_d_b18, eq103_e1349_d_b19, eq103_e1349_d_b20, eq103_e1349_d_b21, eq103_e1349_d_b22, eq103_e1349_d_b23, eq103_e1349_d_b24, eq103_e1349_d_b25, eq103_e1349_d_b26, eq103_e1349_d_b27, eq103_e1349_d_b28, eq103_e1349_d_b29, eq103_e1349_d_b30, eq103_e1349_d_b31, eq103_e1349_d_b32, eq103_e1349_d_b33, eq103_e1349_d_b34, eq103_e1349_d_b35,) = {
    if (!s.b[1201]) {
        let eq103_e1342: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 92, s.v[173]);
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 93, eq103_e1345);
        let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);
        let eq103_e1347_d_n2: f64 = ((s.dn[173][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq103_e1347, (s.dn[173][0] * ddt_scale), (s.dn[173][1] * ddt_scale), eq103_e1347_d_n2, (s.dn[173][3] * ddt_scale), (s.dn[173][4] * ddt_scale), (s.dn[173][5] * ddt_scale), (s.dn[173][6] * ddt_scale), (s.dn[173][7] * ddt_scale), (s.dn[173][8] * ddt_scale), (s.dn[173][9] * ddt_scale), (s.dn[173][10] * ddt_scale), __rspice_deriv_cse_0, (s.dn[173][12] * ddt_scale), (s.dn[173][13] * ddt_scale), (s.dn[173][14] * ddt_scale), (s.dn[173][15] * ddt_scale), (s.dn[173][16] * ddt_scale), (s.dn[173][17] * ddt_scale), (s.dn[173][18] * ddt_scale), (s.dn[173][19] * ddt_scale), (s.dn[173][20] * ddt_scale), (s.dn[173][21] * ddt_scale), (s.dn[173][22] * ddt_scale), (s.dn[173][23] * ddt_scale), (s.dn[173][24] * ddt_scale), (s.dn[173][25] * ddt_scale), (s.dn[173][26] * ddt_scale), (s.dn[173][27] * ddt_scale), (s.dn[173][28] * ddt_scale), (s.dn[173][29] * ddt_scale), (s.db[173][0] * ddt_scale), (s.db[173][1] * ddt_scale), (s.db[173][2] * ddt_scale), (s.db[173][3] * ddt_scale), (s.db[173][4] * ddt_scale), (s.db[173][5] * ddt_scale), (s.db[173][6] * ddt_scale), (s.db[173][7] * ddt_scale), (s.db[173][8] * ddt_scale), (s.db[173][9] * ddt_scale), (s.db[173][10] * ddt_scale), (s.db[173][11] * ddt_scale), (s.db[173][12] * ddt_scale), (s.db[173][13] * ddt_scale), (s.db[173][14] * ddt_scale), (s.db[173][15] * ddt_scale), (s.db[173][16] * ddt_scale), (s.db[173][17] * ddt_scale), (s.db[173][18] * ddt_scale), (s.db[173][19] * ddt_scale), (s.db[173][20] * ddt_scale), (s.db[173][21] * ddt_scale), (s.db[173][22] * ddt_scale), (s.db[173][23] * ddt_scale), (s.db[173][24] * ddt_scale), (s.db[173][25] * ddt_scale), (s.db[173][26] * ddt_scale), (s.db[173][27] * ddt_scale), (s.db[173][28] * ddt_scale), (s.db[173][29] * ddt_scale), (s.db[173][30] * ddt_scale), (s.db[173][31] * ddt_scale), (s.db[173][32] * ddt_scale), (s.db[173][33] * ddt_scale), (s.db[173][34] * ddt_scale), (s.db[173][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1349;
        let eq103_node_derivatives: [f64; 30] = [eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29];
        let eq103_branch_derivatives: [f64; 36] = [eq103_e1349_d_b0, eq103_e1349_d_b1, eq103_e1349_d_b2, eq103_e1349_d_b3, eq103_e1349_d_b4, eq103_e1349_d_b5, eq103_e1349_d_b6, eq103_e1349_d_b7, eq103_e1349_d_b8, eq103_e1349_d_b9, eq103_e1349_d_b10, eq103_e1349_d_b11, eq103_e1349_d_b12, eq103_e1349_d_b13, eq103_e1349_d_b14, eq103_e1349_d_b15, eq103_e1349_d_b16, eq103_e1349_d_b17, eq103_e1349_d_b18, eq103_e1349_d_b19, eq103_e1349_d_b20, eq103_e1349_d_b21, eq103_e1349_d_b22, eq103_e1349_d_b23, eq103_e1349_d_b24, eq103_e1349_d_b25, eq103_e1349_d_b26, eq103_e1349_d_b27, eq103_e1349_d_b28, eq103_e1349_d_b29, eq103_e1349_d_b30, eq103_e1349_d_b31, eq103_e1349_d_b32, eq103_e1349_d_b33, eq103_e1349_d_b34, eq103_e1349_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(11),
            multiplicity * (eq103_value),
            &eq103_node_derivatives,
            &eq103_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29, eq104_e1360_d_b0, eq104_e1360_d_b1, eq104_e1360_d_b2, eq104_e1360_d_b3, eq104_e1360_d_b4, eq104_e1360_d_b5, eq104_e1360_d_b6, eq104_e1360_d_b7, eq104_e1360_d_b8, eq104_e1360_d_b9, eq104_e1360_d_b10, eq104_e1360_d_b11, eq104_e1360_d_b12, eq104_e1360_d_b13, eq104_e1360_d_b14, eq104_e1360_d_b15, eq104_e1360_d_b16, eq104_e1360_d_b17, eq104_e1360_d_b18, eq104_e1360_d_b19, eq104_e1360_d_b20, eq104_e1360_d_b21, eq104_e1360_d_b22, eq104_e1360_d_b23, eq104_e1360_d_b24, eq104_e1360_d_b25, eq104_e1360_d_b26, eq104_e1360_d_b27, eq104_e1360_d_b28, eq104_e1360_d_b29, eq104_e1360_d_b30, eq104_e1360_d_b31, eq104_e1360_d_b32, eq104_e1360_d_b33, eq104_e1360_d_b34, eq104_e1360_d_b35,) = {
    if (!s.b[1201]) {
        let eq104_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 94, s.v[174]);
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 95, eq104_e1356);
        let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);
        let eq104_e1358_d_n2: f64 = ((s.dn[174][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq104_e1358, (s.dn[174][0] * ddt_scale), (s.dn[174][1] * ddt_scale), eq104_e1358_d_n2, (s.dn[174][3] * ddt_scale), (s.dn[174][4] * ddt_scale), (s.dn[174][5] * ddt_scale), (s.dn[174][6] * ddt_scale), (s.dn[174][7] * ddt_scale), (s.dn[174][8] * ddt_scale), (s.dn[174][9] * ddt_scale), __rspice_deriv_cse_1, (s.dn[174][11] * ddt_scale), (s.dn[174][12] * ddt_scale), (s.dn[174][13] * ddt_scale), (s.dn[174][14] * ddt_scale), (s.dn[174][15] * ddt_scale), (s.dn[174][16] * ddt_scale), (s.dn[174][17] * ddt_scale), (s.dn[174][18] * ddt_scale), (s.dn[174][19] * ddt_scale), (s.dn[174][20] * ddt_scale), (s.dn[174][21] * ddt_scale), (s.dn[174][22] * ddt_scale), (s.dn[174][23] * ddt_scale), (s.dn[174][24] * ddt_scale), (s.dn[174][25] * ddt_scale), (s.dn[174][26] * ddt_scale), (s.dn[174][27] * ddt_scale), (s.dn[174][28] * ddt_scale), (s.dn[174][29] * ddt_scale), (s.db[174][0] * ddt_scale), (s.db[174][1] * ddt_scale), (s.db[174][2] * ddt_scale), (s.db[174][3] * ddt_scale), (s.db[174][4] * ddt_scale), (s.db[174][5] * ddt_scale), (s.db[174][6] * ddt_scale), (s.db[174][7] * ddt_scale), (s.db[174][8] * ddt_scale), (s.db[174][9] * ddt_scale), (s.db[174][10] * ddt_scale), (s.db[174][11] * ddt_scale), (s.db[174][12] * ddt_scale), (s.db[174][13] * ddt_scale), (s.db[174][14] * ddt_scale), (s.db[174][15] * ddt_scale), (s.db[174][16] * ddt_scale), (s.db[174][17] * ddt_scale), (s.db[174][18] * ddt_scale), (s.db[174][19] * ddt_scale), (s.db[174][20] * ddt_scale), (s.db[174][21] * ddt_scale), (s.db[174][22] * ddt_scale), (s.db[174][23] * ddt_scale), (s.db[174][24] * ddt_scale), (s.db[174][25] * ddt_scale), (s.db[174][26] * ddt_scale), (s.db[174][27] * ddt_scale), (s.db[174][28] * ddt_scale), (s.db[174][29] * ddt_scale), (s.db[174][30] * ddt_scale), (s.db[174][31] * ddt_scale), (s.db[174][32] * ddt_scale), (s.db[174][33] * ddt_scale), (s.db[174][34] * ddt_scale), (s.db[174][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_value: f64 = eq104_e1360;
        let eq104_node_derivatives: [f64; 30] = [eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29];
        let eq104_branch_derivatives: [f64; 36] = [eq104_e1360_d_b0, eq104_e1360_d_b1, eq104_e1360_d_b2, eq104_e1360_d_b3, eq104_e1360_d_b4, eq104_e1360_d_b5, eq104_e1360_d_b6, eq104_e1360_d_b7, eq104_e1360_d_b8, eq104_e1360_d_b9, eq104_e1360_d_b10, eq104_e1360_d_b11, eq104_e1360_d_b12, eq104_e1360_d_b13, eq104_e1360_d_b14, eq104_e1360_d_b15, eq104_e1360_d_b16, eq104_e1360_d_b17, eq104_e1360_d_b18, eq104_e1360_d_b19, eq104_e1360_d_b20, eq104_e1360_d_b21, eq104_e1360_d_b22, eq104_e1360_d_b23, eq104_e1360_d_b24, eq104_e1360_d_b25, eq104_e1360_d_b26, eq104_e1360_d_b27, eq104_e1360_d_b28, eq104_e1360_d_b29, eq104_e1360_d_b30, eq104_e1360_d_b31, eq104_e1360_d_b32, eq104_e1360_d_b33, eq104_e1360_d_b34, eq104_e1360_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(10),
            multiplicity * (eq104_value),
            &eq104_node_derivatives,
            &eq104_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29, eq105_e1371_d_b0, eq105_e1371_d_b1, eq105_e1371_d_b2, eq105_e1371_d_b3, eq105_e1371_d_b4, eq105_e1371_d_b5, eq105_e1371_d_b6, eq105_e1371_d_b7, eq105_e1371_d_b8, eq105_e1371_d_b9, eq105_e1371_d_b10, eq105_e1371_d_b11, eq105_e1371_d_b12, eq105_e1371_d_b13, eq105_e1371_d_b14, eq105_e1371_d_b15, eq105_e1371_d_b16, eq105_e1371_d_b17, eq105_e1371_d_b18, eq105_e1371_d_b19, eq105_e1371_d_b20, eq105_e1371_d_b21, eq105_e1371_d_b22, eq105_e1371_d_b23, eq105_e1371_d_b24, eq105_e1371_d_b25, eq105_e1371_d_b26, eq105_e1371_d_b27, eq105_e1371_d_b28, eq105_e1371_d_b29, eq105_e1371_d_b30, eq105_e1371_d_b31, eq105_e1371_d_b32, eq105_e1371_d_b33, eq105_e1371_d_b34, eq105_e1371_d_b35,) = {
    if (!s.b[1201]) {
        let eq105_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 96, s.v[175]);
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 97, eq105_e1367);
        let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);
        let eq105_e1369_d_n7: f64 = ((s.dn[175][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq105_e1369, (s.dn[175][0] * ddt_scale), (s.dn[175][1] * ddt_scale), (s.dn[175][2] * ddt_scale), (s.dn[175][3] * ddt_scale), (s.dn[175][4] * ddt_scale), (s.dn[175][5] * ddt_scale), (s.dn[175][6] * ddt_scale), eq105_e1369_d_n7, (s.dn[175][8] * ddt_scale), (s.dn[175][9] * ddt_scale), (s.dn[175][10] * ddt_scale), __rspice_deriv_cse_2, (s.dn[175][12] * ddt_scale), (s.dn[175][13] * ddt_scale), (s.dn[175][14] * ddt_scale), (s.dn[175][15] * ddt_scale), (s.dn[175][16] * ddt_scale), (s.dn[175][17] * ddt_scale), (s.dn[175][18] * ddt_scale), (s.dn[175][19] * ddt_scale), (s.dn[175][20] * ddt_scale), (s.dn[175][21] * ddt_scale), (s.dn[175][22] * ddt_scale), (s.dn[175][23] * ddt_scale), (s.dn[175][24] * ddt_scale), (s.dn[175][25] * ddt_scale), (s.dn[175][26] * ddt_scale), (s.dn[175][27] * ddt_scale), (s.dn[175][28] * ddt_scale), (s.dn[175][29] * ddt_scale), (s.db[175][0] * ddt_scale), (s.db[175][1] * ddt_scale), (s.db[175][2] * ddt_scale), (s.db[175][3] * ddt_scale), (s.db[175][4] * ddt_scale), (s.db[175][5] * ddt_scale), (s.db[175][6] * ddt_scale), (s.db[175][7] * ddt_scale), (s.db[175][8] * ddt_scale), (s.db[175][9] * ddt_scale), (s.db[175][10] * ddt_scale), (s.db[175][11] * ddt_scale), (s.db[175][12] * ddt_scale), (s.db[175][13] * ddt_scale), (s.db[175][14] * ddt_scale), (s.db[175][15] * ddt_scale), (s.db[175][16] * ddt_scale), (s.db[175][17] * ddt_scale), (s.db[175][18] * ddt_scale), (s.db[175][19] * ddt_scale), (s.db[175][20] * ddt_scale), (s.db[175][21] * ddt_scale), (s.db[175][22] * ddt_scale), (s.db[175][23] * ddt_scale), (s.db[175][24] * ddt_scale), (s.db[175][25] * ddt_scale), (s.db[175][26] * ddt_scale), (s.db[175][27] * ddt_scale), (s.db[175][28] * ddt_scale), (s.db[175][29] * ddt_scale), (s.db[175][30] * ddt_scale), (s.db[175][31] * ddt_scale), (s.db[175][32] * ddt_scale), (s.db[175][33] * ddt_scale), (s.db[175][34] * ddt_scale), (s.db[175][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e1371;
        let eq105_node_derivatives: [f64; 30] = [eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29];
        let eq105_branch_derivatives: [f64; 36] = [eq105_e1371_d_b0, eq105_e1371_d_b1, eq105_e1371_d_b2, eq105_e1371_d_b3, eq105_e1371_d_b4, eq105_e1371_d_b5, eq105_e1371_d_b6, eq105_e1371_d_b7, eq105_e1371_d_b8, eq105_e1371_d_b9, eq105_e1371_d_b10, eq105_e1371_d_b11, eq105_e1371_d_b12, eq105_e1371_d_b13, eq105_e1371_d_b14, eq105_e1371_d_b15, eq105_e1371_d_b16, eq105_e1371_d_b17, eq105_e1371_d_b18, eq105_e1371_d_b19, eq105_e1371_d_b20, eq105_e1371_d_b21, eq105_e1371_d_b22, eq105_e1371_d_b23, eq105_e1371_d_b24, eq105_e1371_d_b25, eq105_e1371_d_b26, eq105_e1371_d_b27, eq105_e1371_d_b28, eq105_e1371_d_b29, eq105_e1371_d_b30, eq105_e1371_d_b31, eq105_e1371_d_b32, eq105_e1371_d_b33, eq105_e1371_d_b34, eq105_e1371_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq105_value),
            &eq105_node_derivatives,
            &eq105_branch_derivatives,
            multiplicity,
        );
        let eq108_e1383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 98, s.v[176]);
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 99, eq108_e1386);
        let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);
        let eq108_e1388_d_n3: f64 = ((s.dn[176][3] * ddt_scale) + (p.p355 * ddt_scale));
        let eq108_e1388_d_n11: f64 = ((s.dn[176][11] * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq108_value: f64 = eq108_e1388;
        let eq108_node_derivatives: [f64; 30] = [(s.dn[176][0] * ddt_scale), (s.dn[176][1] * ddt_scale), (s.dn[176][2] * ddt_scale), eq108_e1388_d_n3, (s.dn[176][4] * ddt_scale), (s.dn[176][5] * ddt_scale), (s.dn[176][6] * ddt_scale), (s.dn[176][7] * ddt_scale), (s.dn[176][8] * ddt_scale), (s.dn[176][9] * ddt_scale), (s.dn[176][10] * ddt_scale), eq108_e1388_d_n11, (s.dn[176][12] * ddt_scale), (s.dn[176][13] * ddt_scale), (s.dn[176][14] * ddt_scale), (s.dn[176][15] * ddt_scale), (s.dn[176][16] * ddt_scale), (s.dn[176][17] * ddt_scale), (s.dn[176][18] * ddt_scale), (s.dn[176][19] * ddt_scale), (s.dn[176][20] * ddt_scale), (s.dn[176][21] * ddt_scale), (s.dn[176][22] * ddt_scale), (s.dn[176][23] * ddt_scale), (s.dn[176][24] * ddt_scale), (s.dn[176][25] * ddt_scale), (s.dn[176][26] * ddt_scale), (s.dn[176][27] * ddt_scale), (s.dn[176][28] * ddt_scale), (s.dn[176][29] * ddt_scale)];
        let eq108_branch_derivatives: [f64; 36] = [(s.db[176][0] * ddt_scale), (s.db[176][1] * ddt_scale), (s.db[176][2] * ddt_scale), (s.db[176][3] * ddt_scale), (s.db[176][4] * ddt_scale), (s.db[176][5] * ddt_scale), (s.db[176][6] * ddt_scale), (s.db[176][7] * ddt_scale), (s.db[176][8] * ddt_scale), (s.db[176][9] * ddt_scale), (s.db[176][10] * ddt_scale), (s.db[176][11] * ddt_scale), (s.db[176][12] * ddt_scale), (s.db[176][13] * ddt_scale), (s.db[176][14] * ddt_scale), (s.db[176][15] * ddt_scale), (s.db[176][16] * ddt_scale), (s.db[176][17] * ddt_scale), (s.db[176][18] * ddt_scale), (s.db[176][19] * ddt_scale), (s.db[176][20] * ddt_scale), (s.db[176][21] * ddt_scale), (s.db[176][22] * ddt_scale), (s.db[176][23] * ddt_scale), (s.db[176][24] * ddt_scale), (s.db[176][25] * ddt_scale), (s.db[176][26] * ddt_scale), (s.db[176][27] * ddt_scale), (s.db[176][28] * ddt_scale), (s.db[176][29] * ddt_scale), (s.db[176][30] * ddt_scale), (s.db[176][31] * ddt_scale), (s.db[176][32] * ddt_scale), (s.db[176][33] * ddt_scale), (s.db[176][34] * ddt_scale), (s.db[176][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(11),
            multiplicity * (eq108_value),
            &eq108_node_derivatives,
            &eq108_branch_derivatives,
            multiplicity,
        );
        let (eq109_e1396, eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29, eq109_e1396_d_b0, eq109_e1396_d_b1, eq109_e1396_d_b2, eq109_e1396_d_b3, eq109_e1396_d_b4, eq109_e1396_d_b5, eq109_e1396_d_b6, eq109_e1396_d_b7, eq109_e1396_d_b8, eq109_e1396_d_b9, eq109_e1396_d_b10, eq109_e1396_d_b11, eq109_e1396_d_b12, eq109_e1396_d_b13, eq109_e1396_d_b14, eq109_e1396_d_b15, eq109_e1396_d_b16, eq109_e1396_d_b17, eq109_e1396_d_b18, eq109_e1396_d_b19, eq109_e1396_d_b20, eq109_e1396_d_b21, eq109_e1396_d_b22, eq109_e1396_d_b23, eq109_e1396_d_b24, eq109_e1396_d_b25, eq109_e1396_d_b26, eq109_e1396_d_b27, eq109_e1396_d_b28, eq109_e1396_d_b29, eq109_e1396_d_b30, eq109_e1396_d_b31, eq109_e1396_d_b32, eq109_e1396_d_b33, eq109_e1396_d_b34, eq109_e1396_d_b35,) = {
    if s.b[1202] {
        let eq109_e1393: f64 = (s.v[0] * (nv11 - nv12));
        let eq109_e1394: f64 = (s.v[178] + eq109_e1393);
        let eq109_e1394_d_n11: f64 = (s.dn[178][11] + s.v[0]);
        let eq109_e1394_d_n12: f64 = (s.dn[178][12] + (-s.v[0]));
        (eq109_e1394, s.dn[178][0], s.dn[178][1], s.dn[178][2], s.dn[178][3], s.dn[178][4], s.dn[178][5], s.dn[178][6], s.dn[178][7], s.dn[178][8], s.dn[178][9], s.dn[178][10], eq109_e1394_d_n11, eq109_e1394_d_n12, s.dn[178][13], s.dn[178][14], s.dn[178][15], s.dn[178][16], s.dn[178][17], s.dn[178][18], s.dn[178][19], s.dn[178][20], s.dn[178][21], s.dn[178][22], s.dn[178][23], s.dn[178][24], s.dn[178][25], s.dn[178][26], s.dn[178][27], s.dn[178][28], s.dn[178][29], s.db[178][0], s.db[178][1], s.db[178][2], s.db[178][3], s.db[178][4], s.db[178][5], s.db[178][6], s.db[178][7], s.db[178][8], s.db[178][9], s.db[178][10], s.db[178][11], s.db[178][12], s.db[178][13], s.db[178][14], s.db[178][15], s.db[178][16], s.db[178][17], s.db[178][18], s.db[178][19], s.db[178][20], s.db[178][21], s.db[178][22], s.db[178][23], s.db[178][24], s.db[178][25], s.db[178][26], s.db[178][27], s.db[178][28], s.db[178][29], s.db[178][30], s.db[178][31], s.db[178][32], s.db[178][33], s.db[178][34], s.db[178][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e1396;
        let eq109_node_derivatives: [f64; 30] = [eq109_e1396_d_n0, eq109_e1396_d_n1, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n5, eq109_e1396_d_n6, eq109_e1396_d_n7, eq109_e1396_d_n8, eq109_e1396_d_n9, eq109_e1396_d_n10, eq109_e1396_d_n11, eq109_e1396_d_n12, eq109_e1396_d_n13, eq109_e1396_d_n14, eq109_e1396_d_n15, eq109_e1396_d_n16, eq109_e1396_d_n17, eq109_e1396_d_n18, eq109_e1396_d_n19, eq109_e1396_d_n20, eq109_e1396_d_n21, eq109_e1396_d_n22, eq109_e1396_d_n23, eq109_e1396_d_n24, eq109_e1396_d_n25, eq109_e1396_d_n26, eq109_e1396_d_n27, eq109_e1396_d_n28, eq109_e1396_d_n29];
        let eq109_branch_derivatives: [f64; 36] = [eq109_e1396_d_b0, eq109_e1396_d_b1, eq109_e1396_d_b2, eq109_e1396_d_b3, eq109_e1396_d_b4, eq109_e1396_d_b5, eq109_e1396_d_b6, eq109_e1396_d_b7, eq109_e1396_d_b8, eq109_e1396_d_b9, eq109_e1396_d_b10, eq109_e1396_d_b11, eq109_e1396_d_b12, eq109_e1396_d_b13, eq109_e1396_d_b14, eq109_e1396_d_b15, eq109_e1396_d_b16, eq109_e1396_d_b17, eq109_e1396_d_b18, eq109_e1396_d_b19, eq109_e1396_d_b20, eq109_e1396_d_b21, eq109_e1396_d_b22, eq109_e1396_d_b23, eq109_e1396_d_b24, eq109_e1396_d_b25, eq109_e1396_d_b26, eq109_e1396_d_b27, eq109_e1396_d_b28, eq109_e1396_d_b29, eq109_e1396_d_b30, eq109_e1396_d_b31, eq109_e1396_d_b32, eq109_e1396_d_b33, eq109_e1396_d_b34, eq109_e1396_d_b35];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq109_value),
            &eq109_node_derivatives,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29, eq111_e1411_d_b0, eq111_e1411_d_b1, eq111_e1411_d_b2, eq111_e1411_d_b3, eq111_e1411_d_b4, eq111_e1411_d_b5, eq111_e1411_d_b6, eq111_e1411_d_b7, eq111_e1411_d_b8, eq111_e1411_d_b9, eq111_e1411_d_b10, eq111_e1411_d_b11, eq111_e1411_d_b12, eq111_e1411_d_b13, eq111_e1411_d_b14, eq111_e1411_d_b15, eq111_e1411_d_b16, eq111_e1411_d_b17, eq111_e1411_d_b18, eq111_e1411_d_b19, eq111_e1411_d_b20, eq111_e1411_d_b21, eq111_e1411_d_b22, eq111_e1411_d_b23, eq111_e1411_d_b24, eq111_e1411_d_b25, eq111_e1411_d_b26, eq111_e1411_d_b27, eq111_e1411_d_b28, eq111_e1411_d_b29, eq111_e1411_d_b30, eq111_e1411_d_b31, eq111_e1411_d_b32, eq111_e1411_d_b33, eq111_e1411_d_b34, eq111_e1411_d_b35,) = {
    if s.b[1348] {
        let eq111_e1404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 100, s.v[179]);
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 101, eq111_e1407);
        let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);
        let eq111_e1409_d_n7: f64 = ((s.dn[179][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq111_e1409, (s.dn[179][0] * ddt_scale), (s.dn[179][1] * ddt_scale), (s.dn[179][2] * ddt_scale), (s.dn[179][3] * ddt_scale), (s.dn[179][4] * ddt_scale), (s.dn[179][5] * ddt_scale), (s.dn[179][6] * ddt_scale), eq111_e1409_d_n7, (s.dn[179][8] * ddt_scale), (s.dn[179][9] * ddt_scale), (s.dn[179][10] * ddt_scale), (s.dn[179][11] * ddt_scale), __rspice_deriv_cse_3, (s.dn[179][13] * ddt_scale), (s.dn[179][14] * ddt_scale), (s.dn[179][15] * ddt_scale), (s.dn[179][16] * ddt_scale), (s.dn[179][17] * ddt_scale), (s.dn[179][18] * ddt_scale), (s.dn[179][19] * ddt_scale), (s.dn[179][20] * ddt_scale), (s.dn[179][21] * ddt_scale), (s.dn[179][22] * ddt_scale), (s.dn[179][23] * ddt_scale), (s.dn[179][24] * ddt_scale), (s.dn[179][25] * ddt_scale), (s.dn[179][26] * ddt_scale), (s.dn[179][27] * ddt_scale), (s.dn[179][28] * ddt_scale), (s.dn[179][29] * ddt_scale), (s.db[179][0] * ddt_scale), (s.db[179][1] * ddt_scale), (s.db[179][2] * ddt_scale), (s.db[179][3] * ddt_scale), (s.db[179][4] * ddt_scale), (s.db[179][5] * ddt_scale), (s.db[179][6] * ddt_scale), (s.db[179][7] * ddt_scale), (s.db[179][8] * ddt_scale), (s.db[179][9] * ddt_scale), (s.db[179][10] * ddt_scale), (s.db[179][11] * ddt_scale), (s.db[179][12] * ddt_scale), (s.db[179][13] * ddt_scale), (s.db[179][14] * ddt_scale), (s.db[179][15] * ddt_scale), (s.db[179][16] * ddt_scale), (s.db[179][17] * ddt_scale), (s.db[179][18] * ddt_scale), (s.db[179][19] * ddt_scale), (s.db[179][20] * ddt_scale), (s.db[179][21] * ddt_scale), (s.db[179][22] * ddt_scale), (s.db[179][23] * ddt_scale), (s.db[179][24] * ddt_scale), (s.db[179][25] * ddt_scale), (s.db[179][26] * ddt_scale), (s.db[179][27] * ddt_scale), (s.db[179][28] * ddt_scale), (s.db[179][29] * ddt_scale), (s.db[179][30] * ddt_scale), (s.db[179][31] * ddt_scale), (s.db[179][32] * ddt_scale), (s.db[179][33] * ddt_scale), (s.db[179][34] * ddt_scale), (s.db[179][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1411;
        let eq111_node_derivatives: [f64; 30] = [eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29];
        let eq111_branch_derivatives: [f64; 36] = [eq111_e1411_d_b0, eq111_e1411_d_b1, eq111_e1411_d_b2, eq111_e1411_d_b3, eq111_e1411_d_b4, eq111_e1411_d_b5, eq111_e1411_d_b6, eq111_e1411_d_b7, eq111_e1411_d_b8, eq111_e1411_d_b9, eq111_e1411_d_b10, eq111_e1411_d_b11, eq111_e1411_d_b12, eq111_e1411_d_b13, eq111_e1411_d_b14, eq111_e1411_d_b15, eq111_e1411_d_b16, eq111_e1411_d_b17, eq111_e1411_d_b18, eq111_e1411_d_b19, eq111_e1411_d_b20, eq111_e1411_d_b21, eq111_e1411_d_b22, eq111_e1411_d_b23, eq111_e1411_d_b24, eq111_e1411_d_b25, eq111_e1411_d_b26, eq111_e1411_d_b27, eq111_e1411_d_b28, eq111_e1411_d_b29, eq111_e1411_d_b30, eq111_e1411_d_b31, eq111_e1411_d_b32, eq111_e1411_d_b33, eq111_e1411_d_b34, eq111_e1411_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq111_value),
            &eq111_node_derivatives,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29, eq112_e1421_d_b0, eq112_e1421_d_b1, eq112_e1421_d_b2, eq112_e1421_d_b3, eq112_e1421_d_b4, eq112_e1421_d_b5, eq112_e1421_d_b6, eq112_e1421_d_b7, eq112_e1421_d_b8, eq112_e1421_d_b9, eq112_e1421_d_b10, eq112_e1421_d_b11, eq112_e1421_d_b12, eq112_e1421_d_b13, eq112_e1421_d_b14, eq112_e1421_d_b15, eq112_e1421_d_b16, eq112_e1421_d_b17, eq112_e1421_d_b18, eq112_e1421_d_b19, eq112_e1421_d_b20, eq112_e1421_d_b21, eq112_e1421_d_b22, eq112_e1421_d_b23, eq112_e1421_d_b24, eq112_e1421_d_b25, eq112_e1421_d_b26, eq112_e1421_d_b27, eq112_e1421_d_b28, eq112_e1421_d_b29, eq112_e1421_d_b30, eq112_e1421_d_b31, eq112_e1421_d_b32, eq112_e1421_d_b33, eq112_e1421_d_b34, eq112_e1421_d_b35,) = {
    if s.b[1348] {
        let eq112_e1414: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 102, s.v[180]);
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 103, eq112_e1417);
        let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);
        let eq112_e1419_d_n7: f64 = ((s.dn[180][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq112_e1419, (s.dn[180][0] * ddt_scale), (s.dn[180][1] * ddt_scale), (s.dn[180][2] * ddt_scale), (s.dn[180][3] * ddt_scale), (s.dn[180][4] * ddt_scale), (s.dn[180][5] * ddt_scale), (s.dn[180][6] * ddt_scale), eq112_e1419_d_n7, (s.dn[180][8] * ddt_scale), (s.dn[180][9] * ddt_scale), (s.dn[180][10] * ddt_scale), __rspice_deriv_cse_4, (s.dn[180][12] * ddt_scale), (s.dn[180][13] * ddt_scale), (s.dn[180][14] * ddt_scale), (s.dn[180][15] * ddt_scale), (s.dn[180][16] * ddt_scale), (s.dn[180][17] * ddt_scale), (s.dn[180][18] * ddt_scale), (s.dn[180][19] * ddt_scale), (s.dn[180][20] * ddt_scale), (s.dn[180][21] * ddt_scale), (s.dn[180][22] * ddt_scale), (s.dn[180][23] * ddt_scale), (s.dn[180][24] * ddt_scale), (s.dn[180][25] * ddt_scale), (s.dn[180][26] * ddt_scale), (s.dn[180][27] * ddt_scale), (s.dn[180][28] * ddt_scale), (s.dn[180][29] * ddt_scale), (s.db[180][0] * ddt_scale), (s.db[180][1] * ddt_scale), (s.db[180][2] * ddt_scale), (s.db[180][3] * ddt_scale), (s.db[180][4] * ddt_scale), (s.db[180][5] * ddt_scale), (s.db[180][6] * ddt_scale), (s.db[180][7] * ddt_scale), (s.db[180][8] * ddt_scale), (s.db[180][9] * ddt_scale), (s.db[180][10] * ddt_scale), (s.db[180][11] * ddt_scale), (s.db[180][12] * ddt_scale), (s.db[180][13] * ddt_scale), (s.db[180][14] * ddt_scale), (s.db[180][15] * ddt_scale), (s.db[180][16] * ddt_scale), (s.db[180][17] * ddt_scale), (s.db[180][18] * ddt_scale), (s.db[180][19] * ddt_scale), (s.db[180][20] * ddt_scale), (s.db[180][21] * ddt_scale), (s.db[180][22] * ddt_scale), (s.db[180][23] * ddt_scale), (s.db[180][24] * ddt_scale), (s.db[180][25] * ddt_scale), (s.db[180][26] * ddt_scale), (s.db[180][27] * ddt_scale), (s.db[180][28] * ddt_scale), (s.db[180][29] * ddt_scale), (s.db[180][30] * ddt_scale), (s.db[180][31] * ddt_scale), (s.db[180][32] * ddt_scale), (s.db[180][33] * ddt_scale), (s.db[180][34] * ddt_scale), (s.db[180][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1421;
        let eq112_node_derivatives: [f64; 30] = [eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29];
        let eq112_branch_derivatives: [f64; 36] = [eq112_e1421_d_b0, eq112_e1421_d_b1, eq112_e1421_d_b2, eq112_e1421_d_b3, eq112_e1421_d_b4, eq112_e1421_d_b5, eq112_e1421_d_b6, eq112_e1421_d_b7, eq112_e1421_d_b8, eq112_e1421_d_b9, eq112_e1421_d_b10, eq112_e1421_d_b11, eq112_e1421_d_b12, eq112_e1421_d_b13, eq112_e1421_d_b14, eq112_e1421_d_b15, eq112_e1421_d_b16, eq112_e1421_d_b17, eq112_e1421_d_b18, eq112_e1421_d_b19, eq112_e1421_d_b20, eq112_e1421_d_b21, eq112_e1421_d_b22, eq112_e1421_d_b23, eq112_e1421_d_b24, eq112_e1421_d_b25, eq112_e1421_d_b26, eq112_e1421_d_b27, eq112_e1421_d_b28, eq112_e1421_d_b29, eq112_e1421_d_b30, eq112_e1421_d_b31, eq112_e1421_d_b32, eq112_e1421_d_b33, eq112_e1421_d_b34, eq112_e1421_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq112_value),
            &eq112_node_derivatives,
            &eq112_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29, eq113_e1431_d_b0, eq113_e1431_d_b1, eq113_e1431_d_b2, eq113_e1431_d_b3, eq113_e1431_d_b4, eq113_e1431_d_b5, eq113_e1431_d_b6, eq113_e1431_d_b7, eq113_e1431_d_b8, eq113_e1431_d_b9, eq113_e1431_d_b10, eq113_e1431_d_b11, eq113_e1431_d_b12, eq113_e1431_d_b13, eq113_e1431_d_b14, eq113_e1431_d_b15, eq113_e1431_d_b16, eq113_e1431_d_b17, eq113_e1431_d_b18, eq113_e1431_d_b19, eq113_e1431_d_b20, eq113_e1431_d_b21, eq113_e1431_d_b22, eq113_e1431_d_b23, eq113_e1431_d_b24, eq113_e1431_d_b25, eq113_e1431_d_b26, eq113_e1431_d_b27, eq113_e1431_d_b28, eq113_e1431_d_b29, eq113_e1431_d_b30, eq113_e1431_d_b31, eq113_e1431_d_b32, eq113_e1431_d_b33, eq113_e1431_d_b34, eq113_e1431_d_b35,) = {
    if s.b[1348] {
        let eq113_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 104, s.v[181]);
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 105, eq113_e1427);
        let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);
        let eq113_e1429_d_n2: f64 = ((s.dn[181][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq113_e1429, (s.dn[181][0] * ddt_scale), (s.dn[181][1] * ddt_scale), eq113_e1429_d_n2, (s.dn[181][3] * ddt_scale), (s.dn[181][4] * ddt_scale), (s.dn[181][5] * ddt_scale), (s.dn[181][6] * ddt_scale), (s.dn[181][7] * ddt_scale), (s.dn[181][8] * ddt_scale), (s.dn[181][9] * ddt_scale), (s.dn[181][10] * ddt_scale), (s.dn[181][11] * ddt_scale), __rspice_deriv_cse_5, (s.dn[181][13] * ddt_scale), (s.dn[181][14] * ddt_scale), (s.dn[181][15] * ddt_scale), (s.dn[181][16] * ddt_scale), (s.dn[181][17] * ddt_scale), (s.dn[181][18] * ddt_scale), (s.dn[181][19] * ddt_scale), (s.dn[181][20] * ddt_scale), (s.dn[181][21] * ddt_scale), (s.dn[181][22] * ddt_scale), (s.dn[181][23] * ddt_scale), (s.dn[181][24] * ddt_scale), (s.dn[181][25] * ddt_scale), (s.dn[181][26] * ddt_scale), (s.dn[181][27] * ddt_scale), (s.dn[181][28] * ddt_scale), (s.dn[181][29] * ddt_scale), (s.db[181][0] * ddt_scale), (s.db[181][1] * ddt_scale), (s.db[181][2] * ddt_scale), (s.db[181][3] * ddt_scale), (s.db[181][4] * ddt_scale), (s.db[181][5] * ddt_scale), (s.db[181][6] * ddt_scale), (s.db[181][7] * ddt_scale), (s.db[181][8] * ddt_scale), (s.db[181][9] * ddt_scale), (s.db[181][10] * ddt_scale), (s.db[181][11] * ddt_scale), (s.db[181][12] * ddt_scale), (s.db[181][13] * ddt_scale), (s.db[181][14] * ddt_scale), (s.db[181][15] * ddt_scale), (s.db[181][16] * ddt_scale), (s.db[181][17] * ddt_scale), (s.db[181][18] * ddt_scale), (s.db[181][19] * ddt_scale), (s.db[181][20] * ddt_scale), (s.db[181][21] * ddt_scale), (s.db[181][22] * ddt_scale), (s.db[181][23] * ddt_scale), (s.db[181][24] * ddt_scale), (s.db[181][25] * ddt_scale), (s.db[181][26] * ddt_scale), (s.db[181][27] * ddt_scale), (s.db[181][28] * ddt_scale), (s.db[181][29] * ddt_scale), (s.db[181][30] * ddt_scale), (s.db[181][31] * ddt_scale), (s.db[181][32] * ddt_scale), (s.db[181][33] * ddt_scale), (s.db[181][34] * ddt_scale), (s.db[181][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1431;
        let eq113_node_derivatives: [f64; 30] = [eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29];
        let eq113_branch_derivatives: [f64; 36] = [eq113_e1431_d_b0, eq113_e1431_d_b1, eq113_e1431_d_b2, eq113_e1431_d_b3, eq113_e1431_d_b4, eq113_e1431_d_b5, eq113_e1431_d_b6, eq113_e1431_d_b7, eq113_e1431_d_b8, eq113_e1431_d_b9, eq113_e1431_d_b10, eq113_e1431_d_b11, eq113_e1431_d_b12, eq113_e1431_d_b13, eq113_e1431_d_b14, eq113_e1431_d_b15, eq113_e1431_d_b16, eq113_e1431_d_b17, eq113_e1431_d_b18, eq113_e1431_d_b19, eq113_e1431_d_b20, eq113_e1431_d_b21, eq113_e1431_d_b22, eq113_e1431_d_b23, eq113_e1431_d_b24, eq113_e1431_d_b25, eq113_e1431_d_b26, eq113_e1431_d_b27, eq113_e1431_d_b28, eq113_e1431_d_b29, eq113_e1431_d_b30, eq113_e1431_d_b31, eq113_e1431_d_b32, eq113_e1431_d_b33, eq113_e1431_d_b34, eq113_e1431_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(12),
            multiplicity * (eq113_value),
            &eq113_node_derivatives,
            &eq113_branch_derivatives,
            multiplicity,
        );
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29, eq115_e1445_d_b0, eq115_e1445_d_b1, eq115_e1445_d_b2, eq115_e1445_d_b3, eq115_e1445_d_b4, eq115_e1445_d_b5, eq115_e1445_d_b6, eq115_e1445_d_b7, eq115_e1445_d_b8, eq115_e1445_d_b9, eq115_e1445_d_b10, eq115_e1445_d_b11, eq115_e1445_d_b12, eq115_e1445_d_b13, eq115_e1445_d_b14, eq115_e1445_d_b15, eq115_e1445_d_b16, eq115_e1445_d_b17, eq115_e1445_d_b18, eq115_e1445_d_b19, eq115_e1445_d_b20, eq115_e1445_d_b21, eq115_e1445_d_b22, eq115_e1445_d_b23, eq115_e1445_d_b24, eq115_e1445_d_b25, eq115_e1445_d_b26, eq115_e1445_d_b27, eq115_e1445_d_b28, eq115_e1445_d_b29, eq115_e1445_d_b30, eq115_e1445_d_b31, eq115_e1445_d_b32, eq115_e1445_d_b33, eq115_e1445_d_b34, eq115_e1445_d_b35,) = {
    if s.b[1348] {
        let eq115_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 106, s.v[183]);
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1442: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 107, eq115_e1441);
        let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);
        let eq115_e1443_d_n7: f64 = ((s.dn[183][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq115_e1443_d_n9: f64 = ((s.dn[183][9] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq115_e1443, (s.dn[183][0] * ddt_scale), (s.dn[183][1] * ddt_scale), (s.dn[183][2] * ddt_scale), (s.dn[183][3] * ddt_scale), (s.dn[183][4] * ddt_scale), (s.dn[183][5] * ddt_scale), (s.dn[183][6] * ddt_scale), eq115_e1443_d_n7, (s.dn[183][8] * ddt_scale), eq115_e1443_d_n9, (s.dn[183][10] * ddt_scale), (s.dn[183][11] * ddt_scale), (s.dn[183][12] * ddt_scale), (s.dn[183][13] * ddt_scale), (s.dn[183][14] * ddt_scale), (s.dn[183][15] * ddt_scale), (s.dn[183][16] * ddt_scale), (s.dn[183][17] * ddt_scale), (s.dn[183][18] * ddt_scale), (s.dn[183][19] * ddt_scale), (s.dn[183][20] * ddt_scale), (s.dn[183][21] * ddt_scale), (s.dn[183][22] * ddt_scale), (s.dn[183][23] * ddt_scale), (s.dn[183][24] * ddt_scale), (s.dn[183][25] * ddt_scale), (s.dn[183][26] * ddt_scale), (s.dn[183][27] * ddt_scale), (s.dn[183][28] * ddt_scale), (s.dn[183][29] * ddt_scale), (s.db[183][0] * ddt_scale), (s.db[183][1] * ddt_scale), (s.db[183][2] * ddt_scale), (s.db[183][3] * ddt_scale), (s.db[183][4] * ddt_scale), (s.db[183][5] * ddt_scale), (s.db[183][6] * ddt_scale), (s.db[183][7] * ddt_scale), (s.db[183][8] * ddt_scale), (s.db[183][9] * ddt_scale), (s.db[183][10] * ddt_scale), (s.db[183][11] * ddt_scale), (s.db[183][12] * ddt_scale), (s.db[183][13] * ddt_scale), (s.db[183][14] * ddt_scale), (s.db[183][15] * ddt_scale), (s.db[183][16] * ddt_scale), (s.db[183][17] * ddt_scale), (s.db[183][18] * ddt_scale), (s.db[183][19] * ddt_scale), (s.db[183][20] * ddt_scale), (s.db[183][21] * ddt_scale), (s.db[183][22] * ddt_scale), (s.db[183][23] * ddt_scale), (s.db[183][24] * ddt_scale), (s.db[183][25] * ddt_scale), (s.db[183][26] * ddt_scale), (s.db[183][27] * ddt_scale), (s.db[183][28] * ddt_scale), (s.db[183][29] * ddt_scale), (s.db[183][30] * ddt_scale), (s.db[183][31] * ddt_scale), (s.db[183][32] * ddt_scale), (s.db[183][33] * ddt_scale), (s.db[183][34] * ddt_scale), (s.db[183][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_value: f64 = eq115_e1445;
        let eq115_node_derivatives: [f64; 30] = [eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29];
        let eq115_branch_derivatives: [f64; 36] = [eq115_e1445_d_b0, eq115_e1445_d_b1, eq115_e1445_d_b2, eq115_e1445_d_b3, eq115_e1445_d_b4, eq115_e1445_d_b5, eq115_e1445_d_b6, eq115_e1445_d_b7, eq115_e1445_d_b8, eq115_e1445_d_b9, eq115_e1445_d_b10, eq115_e1445_d_b11, eq115_e1445_d_b12, eq115_e1445_d_b13, eq115_e1445_d_b14, eq115_e1445_d_b15, eq115_e1445_d_b16, eq115_e1445_d_b17, eq115_e1445_d_b18, eq115_e1445_d_b19, eq115_e1445_d_b20, eq115_e1445_d_b21, eq115_e1445_d_b22, eq115_e1445_d_b23, eq115_e1445_d_b24, eq115_e1445_d_b25, eq115_e1445_d_b26, eq115_e1445_d_b27, eq115_e1445_d_b28, eq115_e1445_d_b29, eq115_e1445_d_b30, eq115_e1445_d_b31, eq115_e1445_d_b32, eq115_e1445_d_b33, eq115_e1445_d_b34, eq115_e1445_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq115_value),
            &eq115_node_derivatives,
            &eq115_branch_derivatives,
            multiplicity,
        );
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29, eq116_e1456_d_b0, eq116_e1456_d_b1, eq116_e1456_d_b2, eq116_e1456_d_b3, eq116_e1456_d_b4, eq116_e1456_d_b5, eq116_e1456_d_b6, eq116_e1456_d_b7, eq116_e1456_d_b8, eq116_e1456_d_b9, eq116_e1456_d_b10, eq116_e1456_d_b11, eq116_e1456_d_b12, eq116_e1456_d_b13, eq116_e1456_d_b14, eq116_e1456_d_b15, eq116_e1456_d_b16, eq116_e1456_d_b17, eq116_e1456_d_b18, eq116_e1456_d_b19, eq116_e1456_d_b20, eq116_e1456_d_b21, eq116_e1456_d_b22, eq116_e1456_d_b23, eq116_e1456_d_b24, eq116_e1456_d_b25, eq116_e1456_d_b26, eq116_e1456_d_b27, eq116_e1456_d_b28, eq116_e1456_d_b29, eq116_e1456_d_b30, eq116_e1456_d_b31, eq116_e1456_d_b32, eq116_e1456_d_b33, eq116_e1456_d_b34, eq116_e1456_d_b35,) = {
    if (!s.b[1348]) {
        let eq116_e1449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 108, s.v[179]);
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1453: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 109, eq116_e1452);
        let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);
        let eq116_e1454_d_n2: f64 = ((s.dn[179][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq116_e1454, (s.dn[179][0] * ddt_scale), (s.dn[179][1] * ddt_scale), eq116_e1454_d_n2, (s.dn[179][3] * ddt_scale), (s.dn[179][4] * ddt_scale), (s.dn[179][5] * ddt_scale), (s.dn[179][6] * ddt_scale), (s.dn[179][7] * ddt_scale), (s.dn[179][8] * ddt_scale), (s.dn[179][9] * ddt_scale), (s.dn[179][10] * ddt_scale), (s.dn[179][11] * ddt_scale), __rspice_deriv_cse_3, (s.dn[179][13] * ddt_scale), (s.dn[179][14] * ddt_scale), (s.dn[179][15] * ddt_scale), (s.dn[179][16] * ddt_scale), (s.dn[179][17] * ddt_scale), (s.dn[179][18] * ddt_scale), (s.dn[179][19] * ddt_scale), (s.dn[179][20] * ddt_scale), (s.dn[179][21] * ddt_scale), (s.dn[179][22] * ddt_scale), (s.dn[179][23] * ddt_scale), (s.dn[179][24] * ddt_scale), (s.dn[179][25] * ddt_scale), (s.dn[179][26] * ddt_scale), (s.dn[179][27] * ddt_scale), (s.dn[179][28] * ddt_scale), (s.dn[179][29] * ddt_scale), (s.db[179][0] * ddt_scale), (s.db[179][1] * ddt_scale), (s.db[179][2] * ddt_scale), (s.db[179][3] * ddt_scale), (s.db[179][4] * ddt_scale), (s.db[179][5] * ddt_scale), (s.db[179][6] * ddt_scale), (s.db[179][7] * ddt_scale), (s.db[179][8] * ddt_scale), (s.db[179][9] * ddt_scale), (s.db[179][10] * ddt_scale), (s.db[179][11] * ddt_scale), (s.db[179][12] * ddt_scale), (s.db[179][13] * ddt_scale), (s.db[179][14] * ddt_scale), (s.db[179][15] * ddt_scale), (s.db[179][16] * ddt_scale), (s.db[179][17] * ddt_scale), (s.db[179][18] * ddt_scale), (s.db[179][19] * ddt_scale), (s.db[179][20] * ddt_scale), (s.db[179][21] * ddt_scale), (s.db[179][22] * ddt_scale), (s.db[179][23] * ddt_scale), (s.db[179][24] * ddt_scale), (s.db[179][25] * ddt_scale), (s.db[179][26] * ddt_scale), (s.db[179][27] * ddt_scale), (s.db[179][28] * ddt_scale), (s.db[179][29] * ddt_scale), (s.db[179][30] * ddt_scale), (s.db[179][31] * ddt_scale), (s.db[179][32] * ddt_scale), (s.db[179][33] * ddt_scale), (s.db[179][34] * ddt_scale), (s.db[179][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_value: f64 = eq116_e1456;
        let eq116_node_derivatives: [f64; 30] = [eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29];
        let eq116_branch_derivatives: [f64; 36] = [eq116_e1456_d_b0, eq116_e1456_d_b1, eq116_e1456_d_b2, eq116_e1456_d_b3, eq116_e1456_d_b4, eq116_e1456_d_b5, eq116_e1456_d_b6, eq116_e1456_d_b7, eq116_e1456_d_b8, eq116_e1456_d_b9, eq116_e1456_d_b10, eq116_e1456_d_b11, eq116_e1456_d_b12, eq116_e1456_d_b13, eq116_e1456_d_b14, eq116_e1456_d_b15, eq116_e1456_d_b16, eq116_e1456_d_b17, eq116_e1456_d_b18, eq116_e1456_d_b19, eq116_e1456_d_b20, eq116_e1456_d_b21, eq116_e1456_d_b22, eq116_e1456_d_b23, eq116_e1456_d_b24, eq116_e1456_d_b25, eq116_e1456_d_b26, eq116_e1456_d_b27, eq116_e1456_d_b28, eq116_e1456_d_b29, eq116_e1456_d_b30, eq116_e1456_d_b31, eq116_e1456_d_b32, eq116_e1456_d_b33, eq116_e1456_d_b34, eq116_e1456_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(12),
            multiplicity * (eq116_value),
            &eq116_node_derivatives,
            &eq116_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29, eq117_e1467_d_b0, eq117_e1467_d_b1, eq117_e1467_d_b2, eq117_e1467_d_b3, eq117_e1467_d_b4, eq117_e1467_d_b5, eq117_e1467_d_b6, eq117_e1467_d_b7, eq117_e1467_d_b8, eq117_e1467_d_b9, eq117_e1467_d_b10, eq117_e1467_d_b11, eq117_e1467_d_b12, eq117_e1467_d_b13, eq117_e1467_d_b14, eq117_e1467_d_b15, eq117_e1467_d_b16, eq117_e1467_d_b17, eq117_e1467_d_b18, eq117_e1467_d_b19, eq117_e1467_d_b20, eq117_e1467_d_b21, eq117_e1467_d_b22, eq117_e1467_d_b23, eq117_e1467_d_b24, eq117_e1467_d_b25, eq117_e1467_d_b26, eq117_e1467_d_b27, eq117_e1467_d_b28, eq117_e1467_d_b29, eq117_e1467_d_b30, eq117_e1467_d_b31, eq117_e1467_d_b32, eq117_e1467_d_b33, eq117_e1467_d_b34, eq117_e1467_d_b35,) = {
    if (!s.b[1348]) {
        let eq117_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 110, s.v[180]);
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1464: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 111, eq117_e1463);
        let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);
        let eq117_e1465_d_n2: f64 = ((s.dn[180][2] * ddt_scale) + (p.p355 * ddt_scale));
        (eq117_e1465, (s.dn[180][0] * ddt_scale), (s.dn[180][1] * ddt_scale), eq117_e1465_d_n2, (s.dn[180][3] * ddt_scale), (s.dn[180][4] * ddt_scale), (s.dn[180][5] * ddt_scale), (s.dn[180][6] * ddt_scale), (s.dn[180][7] * ddt_scale), (s.dn[180][8] * ddt_scale), (s.dn[180][9] * ddt_scale), (s.dn[180][10] * ddt_scale), __rspice_deriv_cse_4, (s.dn[180][12] * ddt_scale), (s.dn[180][13] * ddt_scale), (s.dn[180][14] * ddt_scale), (s.dn[180][15] * ddt_scale), (s.dn[180][16] * ddt_scale), (s.dn[180][17] * ddt_scale), (s.dn[180][18] * ddt_scale), (s.dn[180][19] * ddt_scale), (s.dn[180][20] * ddt_scale), (s.dn[180][21] * ddt_scale), (s.dn[180][22] * ddt_scale), (s.dn[180][23] * ddt_scale), (s.dn[180][24] * ddt_scale), (s.dn[180][25] * ddt_scale), (s.dn[180][26] * ddt_scale), (s.dn[180][27] * ddt_scale), (s.dn[180][28] * ddt_scale), (s.dn[180][29] * ddt_scale), (s.db[180][0] * ddt_scale), (s.db[180][1] * ddt_scale), (s.db[180][2] * ddt_scale), (s.db[180][3] * ddt_scale), (s.db[180][4] * ddt_scale), (s.db[180][5] * ddt_scale), (s.db[180][6] * ddt_scale), (s.db[180][7] * ddt_scale), (s.db[180][8] * ddt_scale), (s.db[180][9] * ddt_scale), (s.db[180][10] * ddt_scale), (s.db[180][11] * ddt_scale), (s.db[180][12] * ddt_scale), (s.db[180][13] * ddt_scale), (s.db[180][14] * ddt_scale), (s.db[180][15] * ddt_scale), (s.db[180][16] * ddt_scale), (s.db[180][17] * ddt_scale), (s.db[180][18] * ddt_scale), (s.db[180][19] * ddt_scale), (s.db[180][20] * ddt_scale), (s.db[180][21] * ddt_scale), (s.db[180][22] * ddt_scale), (s.db[180][23] * ddt_scale), (s.db[180][24] * ddt_scale), (s.db[180][25] * ddt_scale), (s.db[180][26] * ddt_scale), (s.db[180][27] * ddt_scale), (s.db[180][28] * ddt_scale), (s.db[180][29] * ddt_scale), (s.db[180][30] * ddt_scale), (s.db[180][31] * ddt_scale), (s.db[180][32] * ddt_scale), (s.db[180][33] * ddt_scale), (s.db[180][34] * ddt_scale), (s.db[180][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_value: f64 = eq117_e1467;
        let eq117_node_derivatives: [f64; 30] = [eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29];
        let eq117_branch_derivatives: [f64; 36] = [eq117_e1467_d_b0, eq117_e1467_d_b1, eq117_e1467_d_b2, eq117_e1467_d_b3, eq117_e1467_d_b4, eq117_e1467_d_b5, eq117_e1467_d_b6, eq117_e1467_d_b7, eq117_e1467_d_b8, eq117_e1467_d_b9, eq117_e1467_d_b10, eq117_e1467_d_b11, eq117_e1467_d_b12, eq117_e1467_d_b13, eq117_e1467_d_b14, eq117_e1467_d_b15, eq117_e1467_d_b16, eq117_e1467_d_b17, eq117_e1467_d_b18, eq117_e1467_d_b19, eq117_e1467_d_b20, eq117_e1467_d_b21, eq117_e1467_d_b22, eq117_e1467_d_b23, eq117_e1467_d_b24, eq117_e1467_d_b25, eq117_e1467_d_b26, eq117_e1467_d_b27, eq117_e1467_d_b28, eq117_e1467_d_b29, eq117_e1467_d_b30, eq117_e1467_d_b31, eq117_e1467_d_b32, eq117_e1467_d_b33, eq117_e1467_d_b34, eq117_e1467_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(11),
            multiplicity * (eq117_value),
            &eq117_node_derivatives,
            &eq117_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29, eq118_e1478_d_b0, eq118_e1478_d_b1, eq118_e1478_d_b2, eq118_e1478_d_b3, eq118_e1478_d_b4, eq118_e1478_d_b5, eq118_e1478_d_b6, eq118_e1478_d_b7, eq118_e1478_d_b8, eq118_e1478_d_b9, eq118_e1478_d_b10, eq118_e1478_d_b11, eq118_e1478_d_b12, eq118_e1478_d_b13, eq118_e1478_d_b14, eq118_e1478_d_b15, eq118_e1478_d_b16, eq118_e1478_d_b17, eq118_e1478_d_b18, eq118_e1478_d_b19, eq118_e1478_d_b20, eq118_e1478_d_b21, eq118_e1478_d_b22, eq118_e1478_d_b23, eq118_e1478_d_b24, eq118_e1478_d_b25, eq118_e1478_d_b26, eq118_e1478_d_b27, eq118_e1478_d_b28, eq118_e1478_d_b29, eq118_e1478_d_b30, eq118_e1478_d_b31, eq118_e1478_d_b32, eq118_e1478_d_b33, eq118_e1478_d_b34, eq118_e1478_d_b35,) = {
    if (!s.b[1348]) {
        let eq118_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 112, s.v[181]);
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1475: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 113, eq118_e1474);
        let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);
        let eq118_e1476_d_n7: f64 = ((s.dn[181][7] * ddt_scale) + (p.p355 * ddt_scale));
        (eq118_e1476, (s.dn[181][0] * ddt_scale), (s.dn[181][1] * ddt_scale), (s.dn[181][2] * ddt_scale), (s.dn[181][3] * ddt_scale), (s.dn[181][4] * ddt_scale), (s.dn[181][5] * ddt_scale), (s.dn[181][6] * ddt_scale), eq118_e1476_d_n7, (s.dn[181][8] * ddt_scale), (s.dn[181][9] * ddt_scale), (s.dn[181][10] * ddt_scale), (s.dn[181][11] * ddt_scale), __rspice_deriv_cse_5, (s.dn[181][13] * ddt_scale), (s.dn[181][14] * ddt_scale), (s.dn[181][15] * ddt_scale), (s.dn[181][16] * ddt_scale), (s.dn[181][17] * ddt_scale), (s.dn[181][18] * ddt_scale), (s.dn[181][19] * ddt_scale), (s.dn[181][20] * ddt_scale), (s.dn[181][21] * ddt_scale), (s.dn[181][22] * ddt_scale), (s.dn[181][23] * ddt_scale), (s.dn[181][24] * ddt_scale), (s.dn[181][25] * ddt_scale), (s.dn[181][26] * ddt_scale), (s.dn[181][27] * ddt_scale), (s.dn[181][28] * ddt_scale), (s.dn[181][29] * ddt_scale), (s.db[181][0] * ddt_scale), (s.db[181][1] * ddt_scale), (s.db[181][2] * ddt_scale), (s.db[181][3] * ddt_scale), (s.db[181][4] * ddt_scale), (s.db[181][5] * ddt_scale), (s.db[181][6] * ddt_scale), (s.db[181][7] * ddt_scale), (s.db[181][8] * ddt_scale), (s.db[181][9] * ddt_scale), (s.db[181][10] * ddt_scale), (s.db[181][11] * ddt_scale), (s.db[181][12] * ddt_scale), (s.db[181][13] * ddt_scale), (s.db[181][14] * ddt_scale), (s.db[181][15] * ddt_scale), (s.db[181][16] * ddt_scale), (s.db[181][17] * ddt_scale), (s.db[181][18] * ddt_scale), (s.db[181][19] * ddt_scale), (s.db[181][20] * ddt_scale), (s.db[181][21] * ddt_scale), (s.db[181][22] * ddt_scale), (s.db[181][23] * ddt_scale), (s.db[181][24] * ddt_scale), (s.db[181][25] * ddt_scale), (s.db[181][26] * ddt_scale), (s.db[181][27] * ddt_scale), (s.db[181][28] * ddt_scale), (s.db[181][29] * ddt_scale), (s.db[181][30] * ddt_scale), (s.db[181][31] * ddt_scale), (s.db[181][32] * ddt_scale), (s.db[181][33] * ddt_scale), (s.db[181][34] * ddt_scale), (s.db[181][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_value: f64 = eq118_e1478;
        let eq118_node_derivatives: [f64; 30] = [eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29];
        let eq118_branch_derivatives: [f64; 36] = [eq118_e1478_d_b0, eq118_e1478_d_b1, eq118_e1478_d_b2, eq118_e1478_d_b3, eq118_e1478_d_b4, eq118_e1478_d_b5, eq118_e1478_d_b6, eq118_e1478_d_b7, eq118_e1478_d_b8, eq118_e1478_d_b9, eq118_e1478_d_b10, eq118_e1478_d_b11, eq118_e1478_d_b12, eq118_e1478_d_b13, eq118_e1478_d_b14, eq118_e1478_d_b15, eq118_e1478_d_b16, eq118_e1478_d_b17, eq118_e1478_d_b18, eq118_e1478_d_b19, eq118_e1478_d_b20, eq118_e1478_d_b21, eq118_e1478_d_b22, eq118_e1478_d_b23, eq118_e1478_d_b24, eq118_e1478_d_b25, eq118_e1478_d_b26, eq118_e1478_d_b27, eq118_e1478_d_b28, eq118_e1478_d_b29, eq118_e1478_d_b30, eq118_e1478_d_b31, eq118_e1478_d_b32, eq118_e1478_d_b33, eq118_e1478_d_b34, eq118_e1478_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq118_value),
            &eq118_node_derivatives,
            &eq118_branch_derivatives,
            multiplicity,
        );
        let eq121_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 114, s.v[182]);
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1494: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 115, eq121_e1493);
        let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);
        let eq121_e1495_d_n3: f64 = ((s.dn[182][3] * ddt_scale) + (p.p355 * ddt_scale));
        let eq121_e1495_d_n12: f64 = ((s.dn[182][12] * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq121_value: f64 = eq121_e1495;
        let eq121_node_derivatives: [f64; 30] = [(s.dn[182][0] * ddt_scale), (s.dn[182][1] * ddt_scale), (s.dn[182][2] * ddt_scale), eq121_e1495_d_n3, (s.dn[182][4] * ddt_scale), (s.dn[182][5] * ddt_scale), (s.dn[182][6] * ddt_scale), (s.dn[182][7] * ddt_scale), (s.dn[182][8] * ddt_scale), (s.dn[182][9] * ddt_scale), (s.dn[182][10] * ddt_scale), (s.dn[182][11] * ddt_scale), eq121_e1495_d_n12, (s.dn[182][13] * ddt_scale), (s.dn[182][14] * ddt_scale), (s.dn[182][15] * ddt_scale), (s.dn[182][16] * ddt_scale), (s.dn[182][17] * ddt_scale), (s.dn[182][18] * ddt_scale), (s.dn[182][19] * ddt_scale), (s.dn[182][20] * ddt_scale), (s.dn[182][21] * ddt_scale), (s.dn[182][22] * ddt_scale), (s.dn[182][23] * ddt_scale), (s.dn[182][24] * ddt_scale), (s.dn[182][25] * ddt_scale), (s.dn[182][26] * ddt_scale), (s.dn[182][27] * ddt_scale), (s.dn[182][28] * ddt_scale), (s.dn[182][29] * ddt_scale)];
        let eq121_branch_derivatives: [f64; 36] = [(s.db[182][0] * ddt_scale), (s.db[182][1] * ddt_scale), (s.db[182][2] * ddt_scale), (s.db[182][3] * ddt_scale), (s.db[182][4] * ddt_scale), (s.db[182][5] * ddt_scale), (s.db[182][6] * ddt_scale), (s.db[182][7] * ddt_scale), (s.db[182][8] * ddt_scale), (s.db[182][9] * ddt_scale), (s.db[182][10] * ddt_scale), (s.db[182][11] * ddt_scale), (s.db[182][12] * ddt_scale), (s.db[182][13] * ddt_scale), (s.db[182][14] * ddt_scale), (s.db[182][15] * ddt_scale), (s.db[182][16] * ddt_scale), (s.db[182][17] * ddt_scale), (s.db[182][18] * ddt_scale), (s.db[182][19] * ddt_scale), (s.db[182][20] * ddt_scale), (s.db[182][21] * ddt_scale), (s.db[182][22] * ddt_scale), (s.db[182][23] * ddt_scale), (s.db[182][24] * ddt_scale), (s.db[182][25] * ddt_scale), (s.db[182][26] * ddt_scale), (s.db[182][27] * ddt_scale), (s.db[182][28] * ddt_scale), (s.db[182][29] * ddt_scale), (s.db[182][30] * ddt_scale), (s.db[182][31] * ddt_scale), (s.db[182][32] * ddt_scale), (s.db[182][33] * ddt_scale), (s.db[182][34] * ddt_scale), (s.db[182][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(12),
            multiplicity * (eq121_value),
            &eq121_node_derivatives,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1503, eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29, eq122_e1503_d_b0, eq122_e1503_d_b1, eq122_e1503_d_b2, eq122_e1503_d_b3, eq122_e1503_d_b4, eq122_e1503_d_b5, eq122_e1503_d_b6, eq122_e1503_d_b7, eq122_e1503_d_b8, eq122_e1503_d_b9, eq122_e1503_d_b10, eq122_e1503_d_b11, eq122_e1503_d_b12, eq122_e1503_d_b13, eq122_e1503_d_b14, eq122_e1503_d_b15, eq122_e1503_d_b16, eq122_e1503_d_b17, eq122_e1503_d_b18, eq122_e1503_d_b19, eq122_e1503_d_b20, eq122_e1503_d_b21, eq122_e1503_d_b22, eq122_e1503_d_b23, eq122_e1503_d_b24, eq122_e1503_d_b25, eq122_e1503_d_b26, eq122_e1503_d_b27, eq122_e1503_d_b28, eq122_e1503_d_b29, eq122_e1503_d_b30, eq122_e1503_d_b31, eq122_e1503_d_b32, eq122_e1503_d_b33, eq122_e1503_d_b34, eq122_e1503_d_b35,) = {
    if s.b[1349] {
        let eq122_e1500: f64 = (s.v[0] * (nv12 - nv13));
        let eq122_e1501: f64 = (s.v[184] + eq122_e1500);
        let eq122_e1501_d_n12: f64 = (s.dn[184][12] + s.v[0]);
        let eq122_e1501_d_n13: f64 = (s.dn[184][13] + (-s.v[0]));
        (eq122_e1501, s.dn[184][0], s.dn[184][1], s.dn[184][2], s.dn[184][3], s.dn[184][4], s.dn[184][5], s.dn[184][6], s.dn[184][7], s.dn[184][8], s.dn[184][9], s.dn[184][10], s.dn[184][11], eq122_e1501_d_n12, eq122_e1501_d_n13, s.dn[184][14], s.dn[184][15], s.dn[184][16], s.dn[184][17], s.dn[184][18], s.dn[184][19], s.dn[184][20], s.dn[184][21], s.dn[184][22], s.dn[184][23], s.dn[184][24], s.dn[184][25], s.dn[184][26], s.dn[184][27], s.dn[184][28], s.dn[184][29], s.db[184][0], s.db[184][1], s.db[184][2], s.db[184][3], s.db[184][4], s.db[184][5], s.db[184][6], s.db[184][7], s.db[184][8], s.db[184][9], s.db[184][10], s.db[184][11], s.db[184][12], s.db[184][13], s.db[184][14], s.db[184][15], s.db[184][16], s.db[184][17], s.db[184][18], s.db[184][19], s.db[184][20], s.db[184][21], s.db[184][22], s.db[184][23], s.db[184][24], s.db[184][25], s.db[184][26], s.db[184][27], s.db[184][28], s.db[184][29], s.db[184][30], s.db[184][31], s.db[184][32], s.db[184][33], s.db[184][34], s.db[184][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1503;
        let eq122_node_derivatives: [f64; 30] = [eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29];
        let eq122_branch_derivatives: [f64; 36] = [eq122_e1503_d_b0, eq122_e1503_d_b1, eq122_e1503_d_b2, eq122_e1503_d_b3, eq122_e1503_d_b4, eq122_e1503_d_b5, eq122_e1503_d_b6, eq122_e1503_d_b7, eq122_e1503_d_b8, eq122_e1503_d_b9, eq122_e1503_d_b10, eq122_e1503_d_b11, eq122_e1503_d_b12, eq122_e1503_d_b13, eq122_e1503_d_b14, eq122_e1503_d_b15, eq122_e1503_d_b16, eq122_e1503_d_b17, eq122_e1503_d_b18, eq122_e1503_d_b19, eq122_e1503_d_b20, eq122_e1503_d_b21, eq122_e1503_d_b22, eq122_e1503_d_b23, eq122_e1503_d_b24, eq122_e1503_d_b25, eq122_e1503_d_b26, eq122_e1503_d_b27, eq122_e1503_d_b28, eq122_e1503_d_b29, eq122_e1503_d_b30, eq122_e1503_d_b31, eq122_e1503_d_b32, eq122_e1503_d_b33, eq122_e1503_d_b34, eq122_e1503_d_b35];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(13),
            multiplicity * (eq122_value),
            &eq122_node_derivatives,
            &eq122_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29, eq124_e1518_d_b0, eq124_e1518_d_b1, eq124_e1518_d_b2, eq124_e1518_d_b3, eq124_e1518_d_b4, eq124_e1518_d_b5, eq124_e1518_d_b6, eq124_e1518_d_b7, eq124_e1518_d_b8, eq124_e1518_d_b9, eq124_e1518_d_b10, eq124_e1518_d_b11, eq124_e1518_d_b12, eq124_e1518_d_b13, eq124_e1518_d_b14, eq124_e1518_d_b15, eq124_e1518_d_b16, eq124_e1518_d_b17, eq124_e1518_d_b18, eq124_e1518_d_b19, eq124_e1518_d_b20, eq124_e1518_d_b21, eq124_e1518_d_b22, eq124_e1518_d_b23, eq124_e1518_d_b24, eq124_e1518_d_b25, eq124_e1518_d_b26, eq124_e1518_d_b27, eq124_e1518_d_b28, eq124_e1518_d_b29, eq124_e1518_d_b30, eq124_e1518_d_b31, eq124_e1518_d_b32, eq124_e1518_d_b33, eq124_e1518_d_b34, eq124_e1518_d_b35,) = {
    if s.b[1495] {
        let eq124_e1511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 116, s.v[185]);
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 117, eq124_e1514);
        let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);
        let eq124_e1516_d_n7: f64 = ((s.dn[185][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq124_e1516_d_n13: f64 = ((s.dn[185][13] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq124_e1516, (s.dn[185][0] * ddt_scale), (s.dn[185][1] * ddt_scale), (s.dn[185][2] * ddt_scale), (s.dn[185][3] * ddt_scale), (s.dn[185][4] * ddt_scale), (s.dn[185][5] * ddt_scale), (s.dn[185][6] * ddt_scale), eq124_e1516_d_n7, (s.dn[185][8] * ddt_scale), (s.dn[185][9] * ddt_scale), (s.dn[185][10] * ddt_scale), (s.dn[185][11] * ddt_scale), (s.dn[185][12] * ddt_scale), eq124_e1516_d_n13, (s.dn[185][14] * ddt_scale), (s.dn[185][15] * ddt_scale), (s.dn[185][16] * ddt_scale), (s.dn[185][17] * ddt_scale), (s.dn[185][18] * ddt_scale), (s.dn[185][19] * ddt_scale), (s.dn[185][20] * ddt_scale), (s.dn[185][21] * ddt_scale), (s.dn[185][22] * ddt_scale), (s.dn[185][23] * ddt_scale), (s.dn[185][24] * ddt_scale), (s.dn[185][25] * ddt_scale), (s.dn[185][26] * ddt_scale), (s.dn[185][27] * ddt_scale), (s.dn[185][28] * ddt_scale), (s.dn[185][29] * ddt_scale), (s.db[185][0] * ddt_scale), (s.db[185][1] * ddt_scale), (s.db[185][2] * ddt_scale), (s.db[185][3] * ddt_scale), (s.db[185][4] * ddt_scale), (s.db[185][5] * ddt_scale), (s.db[185][6] * ddt_scale), (s.db[185][7] * ddt_scale), (s.db[185][8] * ddt_scale), (s.db[185][9] * ddt_scale), (s.db[185][10] * ddt_scale), (s.db[185][11] * ddt_scale), (s.db[185][12] * ddt_scale), (s.db[185][13] * ddt_scale), (s.db[185][14] * ddt_scale), (s.db[185][15] * ddt_scale), (s.db[185][16] * ddt_scale), (s.db[185][17] * ddt_scale), (s.db[185][18] * ddt_scale), (s.db[185][19] * ddt_scale), (s.db[185][20] * ddt_scale), (s.db[185][21] * ddt_scale), (s.db[185][22] * ddt_scale), (s.db[185][23] * ddt_scale), (s.db[185][24] * ddt_scale), (s.db[185][25] * ddt_scale), (s.db[185][26] * ddt_scale), (s.db[185][27] * ddt_scale), (s.db[185][28] * ddt_scale), (s.db[185][29] * ddt_scale), (s.db[185][30] * ddt_scale), (s.db[185][31] * ddt_scale), (s.db[185][32] * ddt_scale), (s.db[185][33] * ddt_scale), (s.db[185][34] * ddt_scale), (s.db[185][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1518;
        let eq124_node_derivatives: [f64; 30] = [eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29];
        let eq124_branch_derivatives: [f64; 36] = [eq124_e1518_d_b0, eq124_e1518_d_b1, eq124_e1518_d_b2, eq124_e1518_d_b3, eq124_e1518_d_b4, eq124_e1518_d_b5, eq124_e1518_d_b6, eq124_e1518_d_b7, eq124_e1518_d_b8, eq124_e1518_d_b9, eq124_e1518_d_b10, eq124_e1518_d_b11, eq124_e1518_d_b12, eq124_e1518_d_b13, eq124_e1518_d_b14, eq124_e1518_d_b15, eq124_e1518_d_b16, eq124_e1518_d_b17, eq124_e1518_d_b18, eq124_e1518_d_b19, eq124_e1518_d_b20, eq124_e1518_d_b21, eq124_e1518_d_b22, eq124_e1518_d_b23, eq124_e1518_d_b24, eq124_e1518_d_b25, eq124_e1518_d_b26, eq124_e1518_d_b27, eq124_e1518_d_b28, eq124_e1518_d_b29, eq124_e1518_d_b30, eq124_e1518_d_b31, eq124_e1518_d_b32, eq124_e1518_d_b33, eq124_e1518_d_b34, eq124_e1518_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(13),
            multiplicity * (eq124_value),
            &eq124_node_derivatives,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29, eq125_e1528_d_b0, eq125_e1528_d_b1, eq125_e1528_d_b2, eq125_e1528_d_b3, eq125_e1528_d_b4, eq125_e1528_d_b5, eq125_e1528_d_b6, eq125_e1528_d_b7, eq125_e1528_d_b8, eq125_e1528_d_b9, eq125_e1528_d_b10, eq125_e1528_d_b11, eq125_e1528_d_b12, eq125_e1528_d_b13, eq125_e1528_d_b14, eq125_e1528_d_b15, eq125_e1528_d_b16, eq125_e1528_d_b17, eq125_e1528_d_b18, eq125_e1528_d_b19, eq125_e1528_d_b20, eq125_e1528_d_b21, eq125_e1528_d_b22, eq125_e1528_d_b23, eq125_e1528_d_b24, eq125_e1528_d_b25, eq125_e1528_d_b26, eq125_e1528_d_b27, eq125_e1528_d_b28, eq125_e1528_d_b29, eq125_e1528_d_b30, eq125_e1528_d_b31, eq125_e1528_d_b32, eq125_e1528_d_b33, eq125_e1528_d_b34, eq125_e1528_d_b35,) = {
    if s.b[1495] {
        let eq125_e1521: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 118, s.v[186]);
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1525: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 119, eq125_e1524);
        let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);
        let eq125_e1526_d_n7: f64 = ((s.dn[186][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq125_e1526_d_n12: f64 = ((s.dn[186][12] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq125_e1526, (s.dn[186][0] * ddt_scale), (s.dn[186][1] * ddt_scale), (s.dn[186][2] * ddt_scale), (s.dn[186][3] * ddt_scale), (s.dn[186][4] * ddt_scale), (s.dn[186][5] * ddt_scale), (s.dn[186][6] * ddt_scale), eq125_e1526_d_n7, (s.dn[186][8] * ddt_scale), (s.dn[186][9] * ddt_scale), (s.dn[186][10] * ddt_scale), (s.dn[186][11] * ddt_scale), eq125_e1526_d_n12, (s.dn[186][13] * ddt_scale), (s.dn[186][14] * ddt_scale), (s.dn[186][15] * ddt_scale), (s.dn[186][16] * ddt_scale), (s.dn[186][17] * ddt_scale), (s.dn[186][18] * ddt_scale), (s.dn[186][19] * ddt_scale), (s.dn[186][20] * ddt_scale), (s.dn[186][21] * ddt_scale), (s.dn[186][22] * ddt_scale), (s.dn[186][23] * ddt_scale), (s.dn[186][24] * ddt_scale), (s.dn[186][25] * ddt_scale), (s.dn[186][26] * ddt_scale), (s.dn[186][27] * ddt_scale), (s.dn[186][28] * ddt_scale), (s.dn[186][29] * ddt_scale), (s.db[186][0] * ddt_scale), (s.db[186][1] * ddt_scale), (s.db[186][2] * ddt_scale), (s.db[186][3] * ddt_scale), (s.db[186][4] * ddt_scale), (s.db[186][5] * ddt_scale), (s.db[186][6] * ddt_scale), (s.db[186][7] * ddt_scale), (s.db[186][8] * ddt_scale), (s.db[186][9] * ddt_scale), (s.db[186][10] * ddt_scale), (s.db[186][11] * ddt_scale), (s.db[186][12] * ddt_scale), (s.db[186][13] * ddt_scale), (s.db[186][14] * ddt_scale), (s.db[186][15] * ddt_scale), (s.db[186][16] * ddt_scale), (s.db[186][17] * ddt_scale), (s.db[186][18] * ddt_scale), (s.db[186][19] * ddt_scale), (s.db[186][20] * ddt_scale), (s.db[186][21] * ddt_scale), (s.db[186][22] * ddt_scale), (s.db[186][23] * ddt_scale), (s.db[186][24] * ddt_scale), (s.db[186][25] * ddt_scale), (s.db[186][26] * ddt_scale), (s.db[186][27] * ddt_scale), (s.db[186][28] * ddt_scale), (s.db[186][29] * ddt_scale), (s.db[186][30] * ddt_scale), (s.db[186][31] * ddt_scale), (s.db[186][32] * ddt_scale), (s.db[186][33] * ddt_scale), (s.db[186][34] * ddt_scale), (s.db[186][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1528;
        let eq125_node_derivatives: [f64; 30] = [eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29];
        let eq125_branch_derivatives: [f64; 36] = [eq125_e1528_d_b0, eq125_e1528_d_b1, eq125_e1528_d_b2, eq125_e1528_d_b3, eq125_e1528_d_b4, eq125_e1528_d_b5, eq125_e1528_d_b6, eq125_e1528_d_b7, eq125_e1528_d_b8, eq125_e1528_d_b9, eq125_e1528_d_b10, eq125_e1528_d_b11, eq125_e1528_d_b12, eq125_e1528_d_b13, eq125_e1528_d_b14, eq125_e1528_d_b15, eq125_e1528_d_b16, eq125_e1528_d_b17, eq125_e1528_d_b18, eq125_e1528_d_b19, eq125_e1528_d_b20, eq125_e1528_d_b21, eq125_e1528_d_b22, eq125_e1528_d_b23, eq125_e1528_d_b24, eq125_e1528_d_b25, eq125_e1528_d_b26, eq125_e1528_d_b27, eq125_e1528_d_b28, eq125_e1528_d_b29, eq125_e1528_d_b30, eq125_e1528_d_b31, eq125_e1528_d_b32, eq125_e1528_d_b33, eq125_e1528_d_b34, eq125_e1528_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq125_value),
            &eq125_node_derivatives,
            &eq125_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29, eq126_e1538_d_b0, eq126_e1538_d_b1, eq126_e1538_d_b2, eq126_e1538_d_b3, eq126_e1538_d_b4, eq126_e1538_d_b5, eq126_e1538_d_b6, eq126_e1538_d_b7, eq126_e1538_d_b8, eq126_e1538_d_b9, eq126_e1538_d_b10, eq126_e1538_d_b11, eq126_e1538_d_b12, eq126_e1538_d_b13, eq126_e1538_d_b14, eq126_e1538_d_b15, eq126_e1538_d_b16, eq126_e1538_d_b17, eq126_e1538_d_b18, eq126_e1538_d_b19, eq126_e1538_d_b20, eq126_e1538_d_b21, eq126_e1538_d_b22, eq126_e1538_d_b23, eq126_e1538_d_b24, eq126_e1538_d_b25, eq126_e1538_d_b26, eq126_e1538_d_b27, eq126_e1538_d_b28, eq126_e1538_d_b29, eq126_e1538_d_b30, eq126_e1538_d_b31, eq126_e1538_d_b32, eq126_e1538_d_b33, eq126_e1538_d_b34, eq126_e1538_d_b35,) = {
    if s.b[1495] {
        let eq126_e1531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 120, s.v[187]);
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1535: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 121, eq126_e1534);
        let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);
        let eq126_e1536_d_n2: f64 = ((s.dn[187][2] * ddt_scale) + (p.p355 * ddt_scale));
        let eq126_e1536_d_n13: f64 = ((s.dn[187][13] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq126_e1536, (s.dn[187][0] * ddt_scale), (s.dn[187][1] * ddt_scale), eq126_e1536_d_n2, (s.dn[187][3] * ddt_scale), (s.dn[187][4] * ddt_scale), (s.dn[187][5] * ddt_scale), (s.dn[187][6] * ddt_scale), (s.dn[187][7] * ddt_scale), (s.dn[187][8] * ddt_scale), (s.dn[187][9] * ddt_scale), (s.dn[187][10] * ddt_scale), (s.dn[187][11] * ddt_scale), (s.dn[187][12] * ddt_scale), eq126_e1536_d_n13, (s.dn[187][14] * ddt_scale), (s.dn[187][15] * ddt_scale), (s.dn[187][16] * ddt_scale), (s.dn[187][17] * ddt_scale), (s.dn[187][18] * ddt_scale), (s.dn[187][19] * ddt_scale), (s.dn[187][20] * ddt_scale), (s.dn[187][21] * ddt_scale), (s.dn[187][22] * ddt_scale), (s.dn[187][23] * ddt_scale), (s.dn[187][24] * ddt_scale), (s.dn[187][25] * ddt_scale), (s.dn[187][26] * ddt_scale), (s.dn[187][27] * ddt_scale), (s.dn[187][28] * ddt_scale), (s.dn[187][29] * ddt_scale), (s.db[187][0] * ddt_scale), (s.db[187][1] * ddt_scale), (s.db[187][2] * ddt_scale), (s.db[187][3] * ddt_scale), (s.db[187][4] * ddt_scale), (s.db[187][5] * ddt_scale), (s.db[187][6] * ddt_scale), (s.db[187][7] * ddt_scale), (s.db[187][8] * ddt_scale), (s.db[187][9] * ddt_scale), (s.db[187][10] * ddt_scale), (s.db[187][11] * ddt_scale), (s.db[187][12] * ddt_scale), (s.db[187][13] * ddt_scale), (s.db[187][14] * ddt_scale), (s.db[187][15] * ddt_scale), (s.db[187][16] * ddt_scale), (s.db[187][17] * ddt_scale), (s.db[187][18] * ddt_scale), (s.db[187][19] * ddt_scale), (s.db[187][20] * ddt_scale), (s.db[187][21] * ddt_scale), (s.db[187][22] * ddt_scale), (s.db[187][23] * ddt_scale), (s.db[187][24] * ddt_scale), (s.db[187][25] * ddt_scale), (s.db[187][26] * ddt_scale), (s.db[187][27] * ddt_scale), (s.db[187][28] * ddt_scale), (s.db[187][29] * ddt_scale), (s.db[187][30] * ddt_scale), (s.db[187][31] * ddt_scale), (s.db[187][32] * ddt_scale), (s.db[187][33] * ddt_scale), (s.db[187][34] * ddt_scale), (s.db[187][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1538;
        let eq126_node_derivatives: [f64; 30] = [eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29];
        let eq126_branch_derivatives: [f64; 36] = [eq126_e1538_d_b0, eq126_e1538_d_b1, eq126_e1538_d_b2, eq126_e1538_d_b3, eq126_e1538_d_b4, eq126_e1538_d_b5, eq126_e1538_d_b6, eq126_e1538_d_b7, eq126_e1538_d_b8, eq126_e1538_d_b9, eq126_e1538_d_b10, eq126_e1538_d_b11, eq126_e1538_d_b12, eq126_e1538_d_b13, eq126_e1538_d_b14, eq126_e1538_d_b15, eq126_e1538_d_b16, eq126_e1538_d_b17, eq126_e1538_d_b18, eq126_e1538_d_b19, eq126_e1538_d_b20, eq126_e1538_d_b21, eq126_e1538_d_b22, eq126_e1538_d_b23, eq126_e1538_d_b24, eq126_e1538_d_b25, eq126_e1538_d_b26, eq126_e1538_d_b27, eq126_e1538_d_b28, eq126_e1538_d_b29, eq126_e1538_d_b30, eq126_e1538_d_b31, eq126_e1538_d_b32, eq126_e1538_d_b33, eq126_e1538_d_b34, eq126_e1538_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(13),
            multiplicity * (eq126_value),
            &eq126_node_derivatives,
            &eq126_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29, eq128_e1552_d_b0, eq128_e1552_d_b1, eq128_e1552_d_b2, eq128_e1552_d_b3, eq128_e1552_d_b4, eq128_e1552_d_b5, eq128_e1552_d_b6, eq128_e1552_d_b7, eq128_e1552_d_b8, eq128_e1552_d_b9, eq128_e1552_d_b10, eq128_e1552_d_b11, eq128_e1552_d_b12, eq128_e1552_d_b13, eq128_e1552_d_b14, eq128_e1552_d_b15, eq128_e1552_d_b16, eq128_e1552_d_b17, eq128_e1552_d_b18, eq128_e1552_d_b19, eq128_e1552_d_b20, eq128_e1552_d_b21, eq128_e1552_d_b22, eq128_e1552_d_b23, eq128_e1552_d_b24, eq128_e1552_d_b25, eq128_e1552_d_b26, eq128_e1552_d_b27, eq128_e1552_d_b28, eq128_e1552_d_b29, eq128_e1552_d_b30, eq128_e1552_d_b31, eq128_e1552_d_b32, eq128_e1552_d_b33, eq128_e1552_d_b34, eq128_e1552_d_b35,) = {
    if s.b[1495] {
        let eq128_e1545: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 122, s.v[189]);
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1549: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 123, eq128_e1548);
        let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);
        let eq128_e1550_d_n7: f64 = ((s.dn[189][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq128_e1550_d_n9: f64 = ((s.dn[189][9] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq128_e1550, (s.dn[189][0] * ddt_scale), (s.dn[189][1] * ddt_scale), (s.dn[189][2] * ddt_scale), (s.dn[189][3] * ddt_scale), (s.dn[189][4] * ddt_scale), (s.dn[189][5] * ddt_scale), (s.dn[189][6] * ddt_scale), eq128_e1550_d_n7, (s.dn[189][8] * ddt_scale), eq128_e1550_d_n9, (s.dn[189][10] * ddt_scale), (s.dn[189][11] * ddt_scale), (s.dn[189][12] * ddt_scale), (s.dn[189][13] * ddt_scale), (s.dn[189][14] * ddt_scale), (s.dn[189][15] * ddt_scale), (s.dn[189][16] * ddt_scale), (s.dn[189][17] * ddt_scale), (s.dn[189][18] * ddt_scale), (s.dn[189][19] * ddt_scale), (s.dn[189][20] * ddt_scale), (s.dn[189][21] * ddt_scale), (s.dn[189][22] * ddt_scale), (s.dn[189][23] * ddt_scale), (s.dn[189][24] * ddt_scale), (s.dn[189][25] * ddt_scale), (s.dn[189][26] * ddt_scale), (s.dn[189][27] * ddt_scale), (s.dn[189][28] * ddt_scale), (s.dn[189][29] * ddt_scale), (s.db[189][0] * ddt_scale), (s.db[189][1] * ddt_scale), (s.db[189][2] * ddt_scale), (s.db[189][3] * ddt_scale), (s.db[189][4] * ddt_scale), (s.db[189][5] * ddt_scale), (s.db[189][6] * ddt_scale), (s.db[189][7] * ddt_scale), (s.db[189][8] * ddt_scale), (s.db[189][9] * ddt_scale), (s.db[189][10] * ddt_scale), (s.db[189][11] * ddt_scale), (s.db[189][12] * ddt_scale), (s.db[189][13] * ddt_scale), (s.db[189][14] * ddt_scale), (s.db[189][15] * ddt_scale), (s.db[189][16] * ddt_scale), (s.db[189][17] * ddt_scale), (s.db[189][18] * ddt_scale), (s.db[189][19] * ddt_scale), (s.db[189][20] * ddt_scale), (s.db[189][21] * ddt_scale), (s.db[189][22] * ddt_scale), (s.db[189][23] * ddt_scale), (s.db[189][24] * ddt_scale), (s.db[189][25] * ddt_scale), (s.db[189][26] * ddt_scale), (s.db[189][27] * ddt_scale), (s.db[189][28] * ddt_scale), (s.db[189][29] * ddt_scale), (s.db[189][30] * ddt_scale), (s.db[189][31] * ddt_scale), (s.db[189][32] * ddt_scale), (s.db[189][33] * ddt_scale), (s.db[189][34] * ddt_scale), (s.db[189][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1552;
        let eq128_node_derivatives: [f64; 30] = [eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29];
        let eq128_branch_derivatives: [f64; 36] = [eq128_e1552_d_b0, eq128_e1552_d_b1, eq128_e1552_d_b2, eq128_e1552_d_b3, eq128_e1552_d_b4, eq128_e1552_d_b5, eq128_e1552_d_b6, eq128_e1552_d_b7, eq128_e1552_d_b8, eq128_e1552_d_b9, eq128_e1552_d_b10, eq128_e1552_d_b11, eq128_e1552_d_b12, eq128_e1552_d_b13, eq128_e1552_d_b14, eq128_e1552_d_b15, eq128_e1552_d_b16, eq128_e1552_d_b17, eq128_e1552_d_b18, eq128_e1552_d_b19, eq128_e1552_d_b20, eq128_e1552_d_b21, eq128_e1552_d_b22, eq128_e1552_d_b23, eq128_e1552_d_b24, eq128_e1552_d_b25, eq128_e1552_d_b26, eq128_e1552_d_b27, eq128_e1552_d_b28, eq128_e1552_d_b29, eq128_e1552_d_b30, eq128_e1552_d_b31, eq128_e1552_d_b32, eq128_e1552_d_b33, eq128_e1552_d_b34, eq128_e1552_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq128_value),
            &eq128_node_derivatives,
            &eq128_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29, eq129_e1563_d_b0, eq129_e1563_d_b1, eq129_e1563_d_b2, eq129_e1563_d_b3, eq129_e1563_d_b4, eq129_e1563_d_b5, eq129_e1563_d_b6, eq129_e1563_d_b7, eq129_e1563_d_b8, eq129_e1563_d_b9, eq129_e1563_d_b10, eq129_e1563_d_b11, eq129_e1563_d_b12, eq129_e1563_d_b13, eq129_e1563_d_b14, eq129_e1563_d_b15, eq129_e1563_d_b16, eq129_e1563_d_b17, eq129_e1563_d_b18, eq129_e1563_d_b19, eq129_e1563_d_b20, eq129_e1563_d_b21, eq129_e1563_d_b22, eq129_e1563_d_b23, eq129_e1563_d_b24, eq129_e1563_d_b25, eq129_e1563_d_b26, eq129_e1563_d_b27, eq129_e1563_d_b28, eq129_e1563_d_b29, eq129_e1563_d_b30, eq129_e1563_d_b31, eq129_e1563_d_b32, eq129_e1563_d_b33, eq129_e1563_d_b34, eq129_e1563_d_b35,) = {
    if (!s.b[1495]) {
        let eq129_e1556: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 124, s.v[185]);
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 125, eq129_e1559);
        let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);
        let eq129_e1561_d_n2: f64 = ((s.dn[185][2] * ddt_scale) + (p.p355 * ddt_scale));
        let eq129_e1561_d_n13: f64 = ((s.dn[185][13] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq129_e1561, (s.dn[185][0] * ddt_scale), (s.dn[185][1] * ddt_scale), eq129_e1561_d_n2, (s.dn[185][3] * ddt_scale), (s.dn[185][4] * ddt_scale), (s.dn[185][5] * ddt_scale), (s.dn[185][6] * ddt_scale), (s.dn[185][7] * ddt_scale), (s.dn[185][8] * ddt_scale), (s.dn[185][9] * ddt_scale), (s.dn[185][10] * ddt_scale), (s.dn[185][11] * ddt_scale), (s.dn[185][12] * ddt_scale), eq129_e1561_d_n13, (s.dn[185][14] * ddt_scale), (s.dn[185][15] * ddt_scale), (s.dn[185][16] * ddt_scale), (s.dn[185][17] * ddt_scale), (s.dn[185][18] * ddt_scale), (s.dn[185][19] * ddt_scale), (s.dn[185][20] * ddt_scale), (s.dn[185][21] * ddt_scale), (s.dn[185][22] * ddt_scale), (s.dn[185][23] * ddt_scale), (s.dn[185][24] * ddt_scale), (s.dn[185][25] * ddt_scale), (s.dn[185][26] * ddt_scale), (s.dn[185][27] * ddt_scale), (s.dn[185][28] * ddt_scale), (s.dn[185][29] * ddt_scale), (s.db[185][0] * ddt_scale), (s.db[185][1] * ddt_scale), (s.db[185][2] * ddt_scale), (s.db[185][3] * ddt_scale), (s.db[185][4] * ddt_scale), (s.db[185][5] * ddt_scale), (s.db[185][6] * ddt_scale), (s.db[185][7] * ddt_scale), (s.db[185][8] * ddt_scale), (s.db[185][9] * ddt_scale), (s.db[185][10] * ddt_scale), (s.db[185][11] * ddt_scale), (s.db[185][12] * ddt_scale), (s.db[185][13] * ddt_scale), (s.db[185][14] * ddt_scale), (s.db[185][15] * ddt_scale), (s.db[185][16] * ddt_scale), (s.db[185][17] * ddt_scale), (s.db[185][18] * ddt_scale), (s.db[185][19] * ddt_scale), (s.db[185][20] * ddt_scale), (s.db[185][21] * ddt_scale), (s.db[185][22] * ddt_scale), (s.db[185][23] * ddt_scale), (s.db[185][24] * ddt_scale), (s.db[185][25] * ddt_scale), (s.db[185][26] * ddt_scale), (s.db[185][27] * ddt_scale), (s.db[185][28] * ddt_scale), (s.db[185][29] * ddt_scale), (s.db[185][30] * ddt_scale), (s.db[185][31] * ddt_scale), (s.db[185][32] * ddt_scale), (s.db[185][33] * ddt_scale), (s.db[185][34] * ddt_scale), (s.db[185][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1563;
        let eq129_node_derivatives: [f64; 30] = [eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29];
        let eq129_branch_derivatives: [f64; 36] = [eq129_e1563_d_b0, eq129_e1563_d_b1, eq129_e1563_d_b2, eq129_e1563_d_b3, eq129_e1563_d_b4, eq129_e1563_d_b5, eq129_e1563_d_b6, eq129_e1563_d_b7, eq129_e1563_d_b8, eq129_e1563_d_b9, eq129_e1563_d_b10, eq129_e1563_d_b11, eq129_e1563_d_b12, eq129_e1563_d_b13, eq129_e1563_d_b14, eq129_e1563_d_b15, eq129_e1563_d_b16, eq129_e1563_d_b17, eq129_e1563_d_b18, eq129_e1563_d_b19, eq129_e1563_d_b20, eq129_e1563_d_b21, eq129_e1563_d_b22, eq129_e1563_d_b23, eq129_e1563_d_b24, eq129_e1563_d_b25, eq129_e1563_d_b26, eq129_e1563_d_b27, eq129_e1563_d_b28, eq129_e1563_d_b29, eq129_e1563_d_b30, eq129_e1563_d_b31, eq129_e1563_d_b32, eq129_e1563_d_b33, eq129_e1563_d_b34, eq129_e1563_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(13),
            multiplicity * (eq129_value),
            &eq129_node_derivatives,
            &eq129_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29, eq130_e1574_d_b0, eq130_e1574_d_b1, eq130_e1574_d_b2, eq130_e1574_d_b3, eq130_e1574_d_b4, eq130_e1574_d_b5, eq130_e1574_d_b6, eq130_e1574_d_b7, eq130_e1574_d_b8, eq130_e1574_d_b9, eq130_e1574_d_b10, eq130_e1574_d_b11, eq130_e1574_d_b12, eq130_e1574_d_b13, eq130_e1574_d_b14, eq130_e1574_d_b15, eq130_e1574_d_b16, eq130_e1574_d_b17, eq130_e1574_d_b18, eq130_e1574_d_b19, eq130_e1574_d_b20, eq130_e1574_d_b21, eq130_e1574_d_b22, eq130_e1574_d_b23, eq130_e1574_d_b24, eq130_e1574_d_b25, eq130_e1574_d_b26, eq130_e1574_d_b27, eq130_e1574_d_b28, eq130_e1574_d_b29, eq130_e1574_d_b30, eq130_e1574_d_b31, eq130_e1574_d_b32, eq130_e1574_d_b33, eq130_e1574_d_b34, eq130_e1574_d_b35,) = {
    if (!s.b[1495]) {
        let eq130_e1567: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 126, s.v[186]);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 127, eq130_e1570);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n2: f64 = ((s.dn[186][2] * ddt_scale) + (p.p355 * ddt_scale));
        let eq130_e1572_d_n12: f64 = ((s.dn[186][12] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq130_e1572, (s.dn[186][0] * ddt_scale), (s.dn[186][1] * ddt_scale), eq130_e1572_d_n2, (s.dn[186][3] * ddt_scale), (s.dn[186][4] * ddt_scale), (s.dn[186][5] * ddt_scale), (s.dn[186][6] * ddt_scale), (s.dn[186][7] * ddt_scale), (s.dn[186][8] * ddt_scale), (s.dn[186][9] * ddt_scale), (s.dn[186][10] * ddt_scale), (s.dn[186][11] * ddt_scale), eq130_e1572_d_n12, (s.dn[186][13] * ddt_scale), (s.dn[186][14] * ddt_scale), (s.dn[186][15] * ddt_scale), (s.dn[186][16] * ddt_scale), (s.dn[186][17] * ddt_scale), (s.dn[186][18] * ddt_scale), (s.dn[186][19] * ddt_scale), (s.dn[186][20] * ddt_scale), (s.dn[186][21] * ddt_scale), (s.dn[186][22] * ddt_scale), (s.dn[186][23] * ddt_scale), (s.dn[186][24] * ddt_scale), (s.dn[186][25] * ddt_scale), (s.dn[186][26] * ddt_scale), (s.dn[186][27] * ddt_scale), (s.dn[186][28] * ddt_scale), (s.dn[186][29] * ddt_scale), (s.db[186][0] * ddt_scale), (s.db[186][1] * ddt_scale), (s.db[186][2] * ddt_scale), (s.db[186][3] * ddt_scale), (s.db[186][4] * ddt_scale), (s.db[186][5] * ddt_scale), (s.db[186][6] * ddt_scale), (s.db[186][7] * ddt_scale), (s.db[186][8] * ddt_scale), (s.db[186][9] * ddt_scale), (s.db[186][10] * ddt_scale), (s.db[186][11] * ddt_scale), (s.db[186][12] * ddt_scale), (s.db[186][13] * ddt_scale), (s.db[186][14] * ddt_scale), (s.db[186][15] * ddt_scale), (s.db[186][16] * ddt_scale), (s.db[186][17] * ddt_scale), (s.db[186][18] * ddt_scale), (s.db[186][19] * ddt_scale), (s.db[186][20] * ddt_scale), (s.db[186][21] * ddt_scale), (s.db[186][22] * ddt_scale), (s.db[186][23] * ddt_scale), (s.db[186][24] * ddt_scale), (s.db[186][25] * ddt_scale), (s.db[186][26] * ddt_scale), (s.db[186][27] * ddt_scale), (s.db[186][28] * ddt_scale), (s.db[186][29] * ddt_scale), (s.db[186][30] * ddt_scale), (s.db[186][31] * ddt_scale), (s.db[186][32] * ddt_scale), (s.db[186][33] * ddt_scale), (s.db[186][34] * ddt_scale), (s.db[186][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        let eq130_node_derivatives: [f64; 30] = [eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29];
        let eq130_branch_derivatives: [f64; 36] = [eq130_e1574_d_b0, eq130_e1574_d_b1, eq130_e1574_d_b2, eq130_e1574_d_b3, eq130_e1574_d_b4, eq130_e1574_d_b5, eq130_e1574_d_b6, eq130_e1574_d_b7, eq130_e1574_d_b8, eq130_e1574_d_b9, eq130_e1574_d_b10, eq130_e1574_d_b11, eq130_e1574_d_b12, eq130_e1574_d_b13, eq130_e1574_d_b14, eq130_e1574_d_b15, eq130_e1574_d_b16, eq130_e1574_d_b17, eq130_e1574_d_b18, eq130_e1574_d_b19, eq130_e1574_d_b20, eq130_e1574_d_b21, eq130_e1574_d_b22, eq130_e1574_d_b23, eq130_e1574_d_b24, eq130_e1574_d_b25, eq130_e1574_d_b26, eq130_e1574_d_b27, eq130_e1574_d_b28, eq130_e1574_d_b29, eq130_e1574_d_b30, eq130_e1574_d_b31, eq130_e1574_d_b32, eq130_e1574_d_b33, eq130_e1574_d_b34, eq130_e1574_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(12),
            multiplicity * (eq130_value),
            &eq130_node_derivatives,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29, eq131_e1585_d_b0, eq131_e1585_d_b1, eq131_e1585_d_b2, eq131_e1585_d_b3, eq131_e1585_d_b4, eq131_e1585_d_b5, eq131_e1585_d_b6, eq131_e1585_d_b7, eq131_e1585_d_b8, eq131_e1585_d_b9, eq131_e1585_d_b10, eq131_e1585_d_b11, eq131_e1585_d_b12, eq131_e1585_d_b13, eq131_e1585_d_b14, eq131_e1585_d_b15, eq131_e1585_d_b16, eq131_e1585_d_b17, eq131_e1585_d_b18, eq131_e1585_d_b19, eq131_e1585_d_b20, eq131_e1585_d_b21, eq131_e1585_d_b22, eq131_e1585_d_b23, eq131_e1585_d_b24, eq131_e1585_d_b25, eq131_e1585_d_b26, eq131_e1585_d_b27, eq131_e1585_d_b28, eq131_e1585_d_b29, eq131_e1585_d_b30, eq131_e1585_d_b31, eq131_e1585_d_b32, eq131_e1585_d_b33, eq131_e1585_d_b34, eq131_e1585_d_b35,) = {
    if (!s.b[1495]) {
        let eq131_e1578: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 128, s.v[187]);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 129, eq131_e1581);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n7: f64 = ((s.dn[187][7] * ddt_scale) + (p.p355 * ddt_scale));
        let eq131_e1583_d_n13: f64 = ((s.dn[187][13] * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq131_e1583, (s.dn[187][0] * ddt_scale), (s.dn[187][1] * ddt_scale), (s.dn[187][2] * ddt_scale), (s.dn[187][3] * ddt_scale), (s.dn[187][4] * ddt_scale), (s.dn[187][5] * ddt_scale), (s.dn[187][6] * ddt_scale), eq131_e1583_d_n7, (s.dn[187][8] * ddt_scale), (s.dn[187][9] * ddt_scale), (s.dn[187][10] * ddt_scale), (s.dn[187][11] * ddt_scale), (s.dn[187][12] * ddt_scale), eq131_e1583_d_n13, (s.dn[187][14] * ddt_scale), (s.dn[187][15] * ddt_scale), (s.dn[187][16] * ddt_scale), (s.dn[187][17] * ddt_scale), (s.dn[187][18] * ddt_scale), (s.dn[187][19] * ddt_scale), (s.dn[187][20] * ddt_scale), (s.dn[187][21] * ddt_scale), (s.dn[187][22] * ddt_scale), (s.dn[187][23] * ddt_scale), (s.dn[187][24] * ddt_scale), (s.dn[187][25] * ddt_scale), (s.dn[187][26] * ddt_scale), (s.dn[187][27] * ddt_scale), (s.dn[187][28] * ddt_scale), (s.dn[187][29] * ddt_scale), (s.db[187][0] * ddt_scale), (s.db[187][1] * ddt_scale), (s.db[187][2] * ddt_scale), (s.db[187][3] * ddt_scale), (s.db[187][4] * ddt_scale), (s.db[187][5] * ddt_scale), (s.db[187][6] * ddt_scale), (s.db[187][7] * ddt_scale), (s.db[187][8] * ddt_scale), (s.db[187][9] * ddt_scale), (s.db[187][10] * ddt_scale), (s.db[187][11] * ddt_scale), (s.db[187][12] * ddt_scale), (s.db[187][13] * ddt_scale), (s.db[187][14] * ddt_scale), (s.db[187][15] * ddt_scale), (s.db[187][16] * ddt_scale), (s.db[187][17] * ddt_scale), (s.db[187][18] * ddt_scale), (s.db[187][19] * ddt_scale), (s.db[187][20] * ddt_scale), (s.db[187][21] * ddt_scale), (s.db[187][22] * ddt_scale), (s.db[187][23] * ddt_scale), (s.db[187][24] * ddt_scale), (s.db[187][25] * ddt_scale), (s.db[187][26] * ddt_scale), (s.db[187][27] * ddt_scale), (s.db[187][28] * ddt_scale), (s.db[187][29] * ddt_scale), (s.db[187][30] * ddt_scale), (s.db[187][31] * ddt_scale), (s.db[187][32] * ddt_scale), (s.db[187][33] * ddt_scale), (s.db[187][34] * ddt_scale), (s.db[187][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        let eq131_node_derivatives: [f64; 30] = [eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29];
        let eq131_branch_derivatives: [f64; 36] = [eq131_e1585_d_b0, eq131_e1585_d_b1, eq131_e1585_d_b2, eq131_e1585_d_b3, eq131_e1585_d_b4, eq131_e1585_d_b5, eq131_e1585_d_b6, eq131_e1585_d_b7, eq131_e1585_d_b8, eq131_e1585_d_b9, eq131_e1585_d_b10, eq131_e1585_d_b11, eq131_e1585_d_b12, eq131_e1585_d_b13, eq131_e1585_d_b14, eq131_e1585_d_b15, eq131_e1585_d_b16, eq131_e1585_d_b17, eq131_e1585_d_b18, eq131_e1585_d_b19, eq131_e1585_d_b20, eq131_e1585_d_b21, eq131_e1585_d_b22, eq131_e1585_d_b23, eq131_e1585_d_b24, eq131_e1585_d_b25, eq131_e1585_d_b26, eq131_e1585_d_b27, eq131_e1585_d_b28, eq131_e1585_d_b29, eq131_e1585_d_b30, eq131_e1585_d_b31, eq131_e1585_d_b32, eq131_e1585_d_b33, eq131_e1585_d_b34, eq131_e1585_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(13),
            multiplicity * (eq131_value),
            &eq131_node_derivatives,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let eq134_e1597: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 130, s.v[188]);
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1601: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 131, eq134_e1600);
        let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);
        let eq134_e1602_d_n3: f64 = ((s.dn[188][3] * ddt_scale) + (p.p355 * ddt_scale));
        let eq134_e1602_d_n13: f64 = ((s.dn[188][13] * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq134_value: f64 = eq134_e1602;
        let eq134_node_derivatives: [f64; 30] = [(s.dn[188][0] * ddt_scale), (s.dn[188][1] * ddt_scale), (s.dn[188][2] * ddt_scale), eq134_e1602_d_n3, (s.dn[188][4] * ddt_scale), (s.dn[188][5] * ddt_scale), (s.dn[188][6] * ddt_scale), (s.dn[188][7] * ddt_scale), (s.dn[188][8] * ddt_scale), (s.dn[188][9] * ddt_scale), (s.dn[188][10] * ddt_scale), (s.dn[188][11] * ddt_scale), (s.dn[188][12] * ddt_scale), eq134_e1602_d_n13, (s.dn[188][14] * ddt_scale), (s.dn[188][15] * ddt_scale), (s.dn[188][16] * ddt_scale), (s.dn[188][17] * ddt_scale), (s.dn[188][18] * ddt_scale), (s.dn[188][19] * ddt_scale), (s.dn[188][20] * ddt_scale), (s.dn[188][21] * ddt_scale), (s.dn[188][22] * ddt_scale), (s.dn[188][23] * ddt_scale), (s.dn[188][24] * ddt_scale), (s.dn[188][25] * ddt_scale), (s.dn[188][26] * ddt_scale), (s.dn[188][27] * ddt_scale), (s.dn[188][28] * ddt_scale), (s.dn[188][29] * ddt_scale)];
        let eq134_branch_derivatives: [f64; 36] = [(s.db[188][0] * ddt_scale), (s.db[188][1] * ddt_scale), (s.db[188][2] * ddt_scale), (s.db[188][3] * ddt_scale), (s.db[188][4] * ddt_scale), (s.db[188][5] * ddt_scale), (s.db[188][6] * ddt_scale), (s.db[188][7] * ddt_scale), (s.db[188][8] * ddt_scale), (s.db[188][9] * ddt_scale), (s.db[188][10] * ddt_scale), (s.db[188][11] * ddt_scale), (s.db[188][12] * ddt_scale), (s.db[188][13] * ddt_scale), (s.db[188][14] * ddt_scale), (s.db[188][15] * ddt_scale), (s.db[188][16] * ddt_scale), (s.db[188][17] * ddt_scale), (s.db[188][18] * ddt_scale), (s.db[188][19] * ddt_scale), (s.db[188][20] * ddt_scale), (s.db[188][21] * ddt_scale), (s.db[188][22] * ddt_scale), (s.db[188][23] * ddt_scale), (s.db[188][24] * ddt_scale), (s.db[188][25] * ddt_scale), (s.db[188][26] * ddt_scale), (s.db[188][27] * ddt_scale), (s.db[188][28] * ddt_scale), (s.db[188][29] * ddt_scale), (s.db[188][30] * ddt_scale), (s.db[188][31] * ddt_scale), (s.db[188][32] * ddt_scale), (s.db[188][33] * ddt_scale), (s.db[188][34] * ddt_scale), (s.db[188][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(13),
            multiplicity * (eq134_value),
            &eq134_node_derivatives,
            &eq134_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1610, eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29, eq135_e1610_d_b0, eq135_e1610_d_b1, eq135_e1610_d_b2, eq135_e1610_d_b3, eq135_e1610_d_b4, eq135_e1610_d_b5, eq135_e1610_d_b6, eq135_e1610_d_b7, eq135_e1610_d_b8, eq135_e1610_d_b9, eq135_e1610_d_b10, eq135_e1610_d_b11, eq135_e1610_d_b12, eq135_e1610_d_b13, eq135_e1610_d_b14, eq135_e1610_d_b15, eq135_e1610_d_b16, eq135_e1610_d_b17, eq135_e1610_d_b18, eq135_e1610_d_b19, eq135_e1610_d_b20, eq135_e1610_d_b21, eq135_e1610_d_b22, eq135_e1610_d_b23, eq135_e1610_d_b24, eq135_e1610_d_b25, eq135_e1610_d_b26, eq135_e1610_d_b27, eq135_e1610_d_b28, eq135_e1610_d_b29, eq135_e1610_d_b30, eq135_e1610_d_b31, eq135_e1610_d_b32, eq135_e1610_d_b33, eq135_e1610_d_b34, eq135_e1610_d_b35,) = {
    if s.b[1496] {
        let eq135_e1607: f64 = (s.v[0] * (nv13 - nv19));
        let eq135_e1608: f64 = (s.v[154] + eq135_e1607);
        let eq135_e1608_d_n13: f64 = (s.dn[154][13] + s.v[0]);
        let eq135_e1608_d_n19: f64 = (s.dn[154][19] + (-s.v[0]));
        (eq135_e1608, s.dn[154][0], s.dn[154][1], s.dn[154][2], s.dn[154][3], s.dn[154][4], s.dn[154][5], s.dn[154][6], s.dn[154][7], s.dn[154][8], s.dn[154][9], s.dn[154][10], s.dn[154][11], s.dn[154][12], eq135_e1608_d_n13, s.dn[154][14], s.dn[154][15], s.dn[154][16], s.dn[154][17], s.dn[154][18], eq135_e1608_d_n19, s.dn[154][20], s.dn[154][21], s.dn[154][22], s.dn[154][23], s.dn[154][24], s.dn[154][25], s.dn[154][26], s.dn[154][27], s.dn[154][28], s.dn[154][29], s.db[154][0], s.db[154][1], s.db[154][2], s.db[154][3], s.db[154][4], s.db[154][5], s.db[154][6], s.db[154][7], s.db[154][8], s.db[154][9], s.db[154][10], s.db[154][11], s.db[154][12], s.db[154][13], s.db[154][14], s.db[154][15], s.db[154][16], s.db[154][17], s.db[154][18], s.db[154][19], s.db[154][20], s.db[154][21], s.db[154][22], s.db[154][23], s.db[154][24], s.db[154][25], s.db[154][26], s.db[154][27], s.db[154][28], s.db[154][29], s.db[154][30], s.db[154][31], s.db[154][32], s.db[154][33], s.db[154][34], s.db[154][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1610;
        let eq135_node_derivatives: [f64; 30] = [eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29];
        let eq135_branch_derivatives: [f64; 36] = [eq135_e1610_d_b0, eq135_e1610_d_b1, eq135_e1610_d_b2, eq135_e1610_d_b3, eq135_e1610_d_b4, eq135_e1610_d_b5, eq135_e1610_d_b6, eq135_e1610_d_b7, eq135_e1610_d_b8, eq135_e1610_d_b9, eq135_e1610_d_b10, eq135_e1610_d_b11, eq135_e1610_d_b12, eq135_e1610_d_b13, eq135_e1610_d_b14, eq135_e1610_d_b15, eq135_e1610_d_b16, eq135_e1610_d_b17, eq135_e1610_d_b18, eq135_e1610_d_b19, eq135_e1610_d_b20, eq135_e1610_d_b21, eq135_e1610_d_b22, eq135_e1610_d_b23, eq135_e1610_d_b24, eq135_e1610_d_b25, eq135_e1610_d_b26, eq135_e1610_d_b27, eq135_e1610_d_b28, eq135_e1610_d_b29, eq135_e1610_d_b30, eq135_e1610_d_b31, eq135_e1610_d_b32, eq135_e1610_d_b33, eq135_e1610_d_b34, eq135_e1610_d_b35];
        stamper.stamp_current_dense_local(
            Some(13),
            Some(19),
            multiplicity * (eq135_value),
            &eq135_node_derivatives,
            &eq135_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1623, eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29, eq137_e1623_d_b0, eq137_e1623_d_b1, eq137_e1623_d_b2, eq137_e1623_d_b3, eq137_e1623_d_b4, eq137_e1623_d_b5, eq137_e1623_d_b6, eq137_e1623_d_b7, eq137_e1623_d_b8, eq137_e1623_d_b9, eq137_e1623_d_b10, eq137_e1623_d_b11, eq137_e1623_d_b12, eq137_e1623_d_b13, eq137_e1623_d_b14, eq137_e1623_d_b15, eq137_e1623_d_b16, eq137_e1623_d_b17, eq137_e1623_d_b18, eq137_e1623_d_b19, eq137_e1623_d_b20, eq137_e1623_d_b21, eq137_e1623_d_b22, eq137_e1623_d_b23, eq137_e1623_d_b24, eq137_e1623_d_b25, eq137_e1623_d_b26, eq137_e1623_d_b27, eq137_e1623_d_b28, eq137_e1623_d_b29, eq137_e1623_d_b30, eq137_e1623_d_b31, eq137_e1623_d_b32, eq137_e1623_d_b33, eq137_e1623_d_b34, eq137_e1623_d_b35,) = {
    if s.b[1642] {
        let eq137_e1620: f64 = (s.v[0] * (nv18 - nv17));
        let eq137_e1621: f64 = (s.v[160] + eq137_e1620);
        let eq137_e1621_d_n17: f64 = (s.dn[160][17] + (-s.v[0]));
        let eq137_e1621_d_n18: f64 = (s.dn[160][18] + s.v[0]);
        (eq137_e1621, s.dn[160][0], s.dn[160][1], s.dn[160][2], s.dn[160][3], s.dn[160][4], s.dn[160][5], s.dn[160][6], s.dn[160][7], s.dn[160][8], s.dn[160][9], s.dn[160][10], s.dn[160][11], s.dn[160][12], s.dn[160][13], s.dn[160][14], s.dn[160][15], s.dn[160][16], eq137_e1621_d_n17, eq137_e1621_d_n18, s.dn[160][19], s.dn[160][20], s.dn[160][21], s.dn[160][22], s.dn[160][23], s.dn[160][24], s.dn[160][25], s.dn[160][26], s.dn[160][27], s.dn[160][28], s.dn[160][29], s.db[160][0], s.db[160][1], s.db[160][2], s.db[160][3], s.db[160][4], s.db[160][5], s.db[160][6], s.db[160][7], s.db[160][8], s.db[160][9], s.db[160][10], s.db[160][11], s.db[160][12], s.db[160][13], s.db[160][14], s.db[160][15], s.db[160][16], s.db[160][17], s.db[160][18], s.db[160][19], s.db[160][20], s.db[160][21], s.db[160][22], s.db[160][23], s.db[160][24], s.db[160][25], s.db[160][26], s.db[160][27], s.db[160][28], s.db[160][29], s.db[160][30], s.db[160][31], s.db[160][32], s.db[160][33], s.db[160][34], s.db[160][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1623;
        let eq137_node_derivatives: [f64; 30] = [eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29];
        let eq137_branch_derivatives: [f64; 36] = [eq137_e1623_d_b0, eq137_e1623_d_b1, eq137_e1623_d_b2, eq137_e1623_d_b3, eq137_e1623_d_b4, eq137_e1623_d_b5, eq137_e1623_d_b6, eq137_e1623_d_b7, eq137_e1623_d_b8, eq137_e1623_d_b9, eq137_e1623_d_b10, eq137_e1623_d_b11, eq137_e1623_d_b12, eq137_e1623_d_b13, eq137_e1623_d_b14, eq137_e1623_d_b15, eq137_e1623_d_b16, eq137_e1623_d_b17, eq137_e1623_d_b18, eq137_e1623_d_b19, eq137_e1623_d_b20, eq137_e1623_d_b21, eq137_e1623_d_b22, eq137_e1623_d_b23, eq137_e1623_d_b24, eq137_e1623_d_b25, eq137_e1623_d_b26, eq137_e1623_d_b27, eq137_e1623_d_b28, eq137_e1623_d_b29, eq137_e1623_d_b30, eq137_e1623_d_b31, eq137_e1623_d_b32, eq137_e1623_d_b33, eq137_e1623_d_b34, eq137_e1623_d_b35];
        stamper.stamp_current_dense_local(
            Some(18),
            Some(17),
            multiplicity * (eq137_value),
            &eq137_node_derivatives,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1644, eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29, eq141_e1644_d_b0, eq141_e1644_d_b1, eq141_e1644_d_b2, eq141_e1644_d_b3, eq141_e1644_d_b4, eq141_e1644_d_b5, eq141_e1644_d_b6, eq141_e1644_d_b7, eq141_e1644_d_b8, eq141_e1644_d_b9, eq141_e1644_d_b10, eq141_e1644_d_b11, eq141_e1644_d_b12, eq141_e1644_d_b13, eq141_e1644_d_b14, eq141_e1644_d_b15, eq141_e1644_d_b16, eq141_e1644_d_b17, eq141_e1644_d_b18, eq141_e1644_d_b19, eq141_e1644_d_b20, eq141_e1644_d_b21, eq141_e1644_d_b22, eq141_e1644_d_b23, eq141_e1644_d_b24, eq141_e1644_d_b25, eq141_e1644_d_b26, eq141_e1644_d_b27, eq141_e1644_d_b28, eq141_e1644_d_b29, eq141_e1644_d_b30, eq141_e1644_d_b31, eq141_e1644_d_b32, eq141_e1644_d_b33, eq141_e1644_d_b34, eq141_e1644_d_b35,) = {
    if s.b[1933] {
        let eq141_e1641: f64 = (s.v[0] * (nv5 - nv9));
        let eq141_e1642: f64 = (s.v[115] + eq141_e1641);
        let eq141_e1642_d_n5: f64 = (s.dn[115][5] + s.v[0]);
        let eq141_e1642_d_n9: f64 = (s.dn[115][9] + (-s.v[0]));
        (eq141_e1642, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], eq141_e1642_d_n5, s.dn[115][6], s.dn[115][7], s.dn[115][8], eq141_e1642_d_n9, s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], s.dn[115][28], s.dn[115][29], s.db[115][0], s.db[115][1], s.db[115][2], s.db[115][3], s.db[115][4], s.db[115][5], s.db[115][6], s.db[115][7], s.db[115][8], s.db[115][9], s.db[115][10], s.db[115][11], s.db[115][12], s.db[115][13], s.db[115][14], s.db[115][15], s.db[115][16], s.db[115][17], s.db[115][18], s.db[115][19], s.db[115][20], s.db[115][21], s.db[115][22], s.db[115][23], s.db[115][24], s.db[115][25], s.db[115][26], s.db[115][27], s.db[115][28], s.db[115][29], s.db[115][30], s.db[115][31], s.db[115][32], s.db[115][33], s.db[115][34], s.db[115][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1644;
        let eq141_node_derivatives: [f64; 30] = [eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29];
        let eq141_branch_derivatives: [f64; 36] = [eq141_e1644_d_b0, eq141_e1644_d_b1, eq141_e1644_d_b2, eq141_e1644_d_b3, eq141_e1644_d_b4, eq141_e1644_d_b5, eq141_e1644_d_b6, eq141_e1644_d_b7, eq141_e1644_d_b8, eq141_e1644_d_b9, eq141_e1644_d_b10, eq141_e1644_d_b11, eq141_e1644_d_b12, eq141_e1644_d_b13, eq141_e1644_d_b14, eq141_e1644_d_b15, eq141_e1644_d_b16, eq141_e1644_d_b17, eq141_e1644_d_b18, eq141_e1644_d_b19, eq141_e1644_d_b20, eq141_e1644_d_b21, eq141_e1644_d_b22, eq141_e1644_d_b23, eq141_e1644_d_b24, eq141_e1644_d_b25, eq141_e1644_d_b26, eq141_e1644_d_b27, eq141_e1644_d_b28, eq141_e1644_d_b29, eq141_e1644_d_b30, eq141_e1644_d_b31, eq141_e1644_d_b32, eq141_e1644_d_b33, eq141_e1644_d_b34, eq141_e1644_d_b35];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(9),
            multiplicity * (eq141_value),
            &eq141_node_derivatives,
            &eq141_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_d_b0, eq142_e1656_d_b1, eq142_e1656_d_b2, eq142_e1656_d_b3, eq142_e1656_d_b4, eq142_e1656_d_b5, eq142_e1656_d_b6, eq142_e1656_d_b7, eq142_e1656_d_b8, eq142_e1656_d_b9, eq142_e1656_d_b10, eq142_e1656_d_b11, eq142_e1656_d_b12, eq142_e1656_d_b13, eq142_e1656_d_b14, eq142_e1656_d_b15, eq142_e1656_d_b16, eq142_e1656_d_b17, eq142_e1656_d_b18, eq142_e1656_d_b19, eq142_e1656_d_b20, eq142_e1656_d_b21, eq142_e1656_d_b22, eq142_e1656_d_b23, eq142_e1656_d_b24, eq142_e1656_d_b25, eq142_e1656_d_b26, eq142_e1656_d_b27, eq142_e1656_d_b28, eq142_e1656_d_b29, eq142_e1656_d_b30, eq142_e1656_d_b31, eq142_e1656_d_b32, eq142_e1656_d_b33, eq142_e1656_d_b34, eq142_e1656_d_b35,) = {
    if (!s.b[1933]) {
        let eq142_e1649: f64 = (s.v[115] - (nv29 - 0.0));
        let eq142_e1649_d_n29: f64 = (s.dn[115][29] - 1.0);
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 132, eq142_e1652);
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);
        let eq142_e1654_d_n28: f64 = (s.dn[115][28] - (p.p323 * ddt_scale));
        (eq142_e1654, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], s.dn[115][5], s.dn[115][6], s.dn[115][7], s.dn[115][8], s.dn[115][9], s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], eq142_e1654_d_n28, eq142_e1649_d_n29, s.db[115][0], s.db[115][1], s.db[115][2], s.db[115][3], s.db[115][4], s.db[115][5], s.db[115][6], s.db[115][7], s.db[115][8], s.db[115][9], s.db[115][10], s.db[115][11], s.db[115][12], s.db[115][13], s.db[115][14], s.db[115][15], s.db[115][16], s.db[115][17], s.db[115][18], s.db[115][19], s.db[115][20], s.db[115][21], s.db[115][22], s.db[115][23], s.db[115][24], s.db[115][25], s.db[115][26], s.db[115][27], s.db[115][28], s.db[115][29], s.db[115][30], s.db[115][31], s.db[115][32], s.db[115][33], s.db[115][34], s.db[115][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        let eq142_node_derivatives: [f64; 30] = [eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29];
        let eq142_branch_derivatives: [f64; 36] = [eq142_e1656_d_b0, eq142_e1656_d_b1, eq142_e1656_d_b2, eq142_e1656_d_b3, eq142_e1656_d_b4, eq142_e1656_d_b5, eq142_e1656_d_b6, eq142_e1656_d_b7, eq142_e1656_d_b8, eq142_e1656_d_b9, eq142_e1656_d_b10, eq142_e1656_d_b11, eq142_e1656_d_b12, eq142_e1656_d_b13, eq142_e1656_d_b14, eq142_e1656_d_b15, eq142_e1656_d_b16, eq142_e1656_d_b17, eq142_e1656_d_b18, eq142_e1656_d_b19, eq142_e1656_d_b20, eq142_e1656_d_b21, eq142_e1656_d_b22, eq142_e1656_d_b23, eq142_e1656_d_b24, eq142_e1656_d_b25, eq142_e1656_d_b26, eq142_e1656_d_b27, eq142_e1656_d_b28, eq142_e1656_d_b29, eq142_e1656_d_b30, eq142_e1656_d_b31, eq142_e1656_d_b32, eq142_e1656_d_b33, eq142_e1656_d_b34, eq142_e1656_d_b35];
        stamper.stamp_current_dense_local(
            Some(28),
            None,
            multiplicity * (eq142_value),
            &eq142_node_derivatives,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (!s.b[1933]) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 133, eq143_e1666);
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
        let eq145_e1681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 134, s.v[117]);
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 135, eq145_e1684);
        let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);
        let eq145_e1686_d_n8: f64 = ((s.dn[117][8] * ddt_scale) + (p.p355 * ddt_scale));
        let eq145_e1686_d_n9: f64 = ((s.dn[117][9] * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq145_value: f64 = eq145_e1686;
        let eq145_node_derivatives: [f64; 30] = [(s.dn[117][0] * ddt_scale), (s.dn[117][1] * ddt_scale), (s.dn[117][2] * ddt_scale), (s.dn[117][3] * ddt_scale), (s.dn[117][4] * ddt_scale), (s.dn[117][5] * ddt_scale), (s.dn[117][6] * ddt_scale), (s.dn[117][7] * ddt_scale), eq145_e1686_d_n8, eq145_e1686_d_n9, (s.dn[117][10] * ddt_scale), (s.dn[117][11] * ddt_scale), (s.dn[117][12] * ddt_scale), (s.dn[117][13] * ddt_scale), (s.dn[117][14] * ddt_scale), (s.dn[117][15] * ddt_scale), (s.dn[117][16] * ddt_scale), (s.dn[117][17] * ddt_scale), (s.dn[117][18] * ddt_scale), (s.dn[117][19] * ddt_scale), (s.dn[117][20] * ddt_scale), (s.dn[117][21] * ddt_scale), (s.dn[117][22] * ddt_scale), (s.dn[117][23] * ddt_scale), (s.dn[117][24] * ddt_scale), (s.dn[117][25] * ddt_scale), (s.dn[117][26] * ddt_scale), (s.dn[117][27] * ddt_scale), (s.dn[117][28] * ddt_scale), (s.dn[117][29] * ddt_scale)];
        let eq145_branch_derivatives: [f64; 36] = [(s.db[117][0] * ddt_scale), (s.db[117][1] * ddt_scale), (s.db[117][2] * ddt_scale), (s.db[117][3] * ddt_scale), (s.db[117][4] * ddt_scale), (s.db[117][5] * ddt_scale), (s.db[117][6] * ddt_scale), (s.db[117][7] * ddt_scale), (s.db[117][8] * ddt_scale), (s.db[117][9] * ddt_scale), (s.db[117][10] * ddt_scale), (s.db[117][11] * ddt_scale), (s.db[117][12] * ddt_scale), (s.db[117][13] * ddt_scale), (s.db[117][14] * ddt_scale), (s.db[117][15] * ddt_scale), (s.db[117][16] * ddt_scale), (s.db[117][17] * ddt_scale), (s.db[117][18] * ddt_scale), (s.db[117][19] * ddt_scale), (s.db[117][20] * ddt_scale), (s.db[117][21] * ddt_scale), (s.db[117][22] * ddt_scale), (s.db[117][23] * ddt_scale), (s.db[117][24] * ddt_scale), (s.db[117][25] * ddt_scale), (s.db[117][26] * ddt_scale), (s.db[117][27] * ddt_scale), (s.db[117][28] * ddt_scale), (s.db[117][29] * ddt_scale), (s.db[117][30] * ddt_scale), (s.db[117][31] * ddt_scale), (s.db[117][32] * ddt_scale), (s.db[117][33] * ddt_scale), (s.db[117][34] * ddt_scale), (s.db[117][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq145_value),
            &eq145_node_derivatives,
            &eq145_branch_derivatives,
            multiplicity,
        );
        let eq146_e1688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 136, s.v[118]);
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1692: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 137, eq146_e1691);
        let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);
        let eq146_e1693_d_n5: f64 = ((s.dn[118][5] * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq146_e1693_d_n8: f64 = ((s.dn[118][8] * ddt_scale) + (p.p355 * ddt_scale));
        let eq146_value: f64 = eq146_e1693;
        let eq146_node_derivatives: [f64; 30] = [(s.dn[118][0] * ddt_scale), (s.dn[118][1] * ddt_scale), (s.dn[118][2] * ddt_scale), (s.dn[118][3] * ddt_scale), (s.dn[118][4] * ddt_scale), eq146_e1693_d_n5, (s.dn[118][6] * ddt_scale), (s.dn[118][7] * ddt_scale), eq146_e1693_d_n8, (s.dn[118][9] * ddt_scale), (s.dn[118][10] * ddt_scale), (s.dn[118][11] * ddt_scale), (s.dn[118][12] * ddt_scale), (s.dn[118][13] * ddt_scale), (s.dn[118][14] * ddt_scale), (s.dn[118][15] * ddt_scale), (s.dn[118][16] * ddt_scale), (s.dn[118][17] * ddt_scale), (s.dn[118][18] * ddt_scale), (s.dn[118][19] * ddt_scale), (s.dn[118][20] * ddt_scale), (s.dn[118][21] * ddt_scale), (s.dn[118][22] * ddt_scale), (s.dn[118][23] * ddt_scale), (s.dn[118][24] * ddt_scale), (s.dn[118][25] * ddt_scale), (s.dn[118][26] * ddt_scale), (s.dn[118][27] * ddt_scale), (s.dn[118][28] * ddt_scale), (s.dn[118][29] * ddt_scale)];
        let eq146_branch_derivatives: [f64; 36] = [(s.db[118][0] * ddt_scale), (s.db[118][1] * ddt_scale), (s.db[118][2] * ddt_scale), (s.db[118][3] * ddt_scale), (s.db[118][4] * ddt_scale), (s.db[118][5] * ddt_scale), (s.db[118][6] * ddt_scale), (s.db[118][7] * ddt_scale), (s.db[118][8] * ddt_scale), (s.db[118][9] * ddt_scale), (s.db[118][10] * ddt_scale), (s.db[118][11] * ddt_scale), (s.db[118][12] * ddt_scale), (s.db[118][13] * ddt_scale), (s.db[118][14] * ddt_scale), (s.db[118][15] * ddt_scale), (s.db[118][16] * ddt_scale), (s.db[118][17] * ddt_scale), (s.db[118][18] * ddt_scale), (s.db[118][19] * ddt_scale), (s.db[118][20] * ddt_scale), (s.db[118][21] * ddt_scale), (s.db[118][22] * ddt_scale), (s.db[118][23] * ddt_scale), (s.db[118][24] * ddt_scale), (s.db[118][25] * ddt_scale), (s.db[118][26] * ddt_scale), (s.db[118][27] * ddt_scale), (s.db[118][28] * ddt_scale), (s.db[118][29] * ddt_scale), (s.db[118][30] * ddt_scale), (s.db[118][31] * ddt_scale), (s.db[118][32] * ddt_scale), (s.db[118][33] * ddt_scale), (s.db[118][34] * ddt_scale), (s.db[118][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq146_value),
            &eq146_node_derivatives,
            &eq146_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1701, eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29, eq147_e1701_d_b0, eq147_e1701_d_b1, eq147_e1701_d_b2, eq147_e1701_d_b3, eq147_e1701_d_b4, eq147_e1701_d_b5, eq147_e1701_d_b6, eq147_e1701_d_b7, eq147_e1701_d_b8, eq147_e1701_d_b9, eq147_e1701_d_b10, eq147_e1701_d_b11, eq147_e1701_d_b12, eq147_e1701_d_b13, eq147_e1701_d_b14, eq147_e1701_d_b15, eq147_e1701_d_b16, eq147_e1701_d_b17, eq147_e1701_d_b18, eq147_e1701_d_b19, eq147_e1701_d_b20, eq147_e1701_d_b21, eq147_e1701_d_b22, eq147_e1701_d_b23, eq147_e1701_d_b24, eq147_e1701_d_b25, eq147_e1701_d_b26, eq147_e1701_d_b27, eq147_e1701_d_b28, eq147_e1701_d_b29, eq147_e1701_d_b30, eq147_e1701_d_b31, eq147_e1701_d_b32, eq147_e1701_d_b33, eq147_e1701_d_b34, eq147_e1701_d_b35,) = {
    if s.b[1934] {
        let eq147_e1698: f64 = (s.v[0] * (nv8 - nv13));
        let eq147_e1699: f64 = (s.v[122] + eq147_e1698);
        let eq147_e1699_d_n8: f64 = (s.dn[122][8] + s.v[0]);
        let eq147_e1699_d_n13: f64 = (s.dn[122][13] + (-s.v[0]));
        (eq147_e1699, s.dn[122][0], s.dn[122][1], s.dn[122][2], s.dn[122][3], s.dn[122][4], s.dn[122][5], s.dn[122][6], s.dn[122][7], eq147_e1699_d_n8, s.dn[122][9], s.dn[122][10], s.dn[122][11], s.dn[122][12], eq147_e1699_d_n13, s.dn[122][14], s.dn[122][15], s.dn[122][16], s.dn[122][17], s.dn[122][18], s.dn[122][19], s.dn[122][20], s.dn[122][21], s.dn[122][22], s.dn[122][23], s.dn[122][24], s.dn[122][25], s.dn[122][26], s.dn[122][27], s.dn[122][28], s.dn[122][29], s.db[122][0], s.db[122][1], s.db[122][2], s.db[122][3], s.db[122][4], s.db[122][5], s.db[122][6], s.db[122][7], s.db[122][8], s.db[122][9], s.db[122][10], s.db[122][11], s.db[122][12], s.db[122][13], s.db[122][14], s.db[122][15], s.db[122][16], s.db[122][17], s.db[122][18], s.db[122][19], s.db[122][20], s.db[122][21], s.db[122][22], s.db[122][23], s.db[122][24], s.db[122][25], s.db[122][26], s.db[122][27], s.db[122][28], s.db[122][29], s.db[122][30], s.db[122][31], s.db[122][32], s.db[122][33], s.db[122][34], s.db[122][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1701;
        let eq147_node_derivatives: [f64; 30] = [eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29];
        let eq147_branch_derivatives: [f64; 36] = [eq147_e1701_d_b0, eq147_e1701_d_b1, eq147_e1701_d_b2, eq147_e1701_d_b3, eq147_e1701_d_b4, eq147_e1701_d_b5, eq147_e1701_d_b6, eq147_e1701_d_b7, eq147_e1701_d_b8, eq147_e1701_d_b9, eq147_e1701_d_b10, eq147_e1701_d_b11, eq147_e1701_d_b12, eq147_e1701_d_b13, eq147_e1701_d_b14, eq147_e1701_d_b15, eq147_e1701_d_b16, eq147_e1701_d_b17, eq147_e1701_d_b18, eq147_e1701_d_b19, eq147_e1701_d_b20, eq147_e1701_d_b21, eq147_e1701_d_b22, eq147_e1701_d_b23, eq147_e1701_d_b24, eq147_e1701_d_b25, eq147_e1701_d_b26, eq147_e1701_d_b27, eq147_e1701_d_b28, eq147_e1701_d_b29, eq147_e1701_d_b30, eq147_e1701_d_b31, eq147_e1701_d_b32, eq147_e1701_d_b33, eq147_e1701_d_b34, eq147_e1701_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(13),
            multiplicity * (eq147_value),
            &eq147_node_derivatives,
            &eq147_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1709, eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29, eq148_e1709_d_b0, eq148_e1709_d_b1, eq148_e1709_d_b2, eq148_e1709_d_b3, eq148_e1709_d_b4, eq148_e1709_d_b5, eq148_e1709_d_b6, eq148_e1709_d_b7, eq148_e1709_d_b8, eq148_e1709_d_b9, eq148_e1709_d_b10, eq148_e1709_d_b11, eq148_e1709_d_b12, eq148_e1709_d_b13, eq148_e1709_d_b14, eq148_e1709_d_b15, eq148_e1709_d_b16, eq148_e1709_d_b17, eq148_e1709_d_b18, eq148_e1709_d_b19, eq148_e1709_d_b20, eq148_e1709_d_b21, eq148_e1709_d_b22, eq148_e1709_d_b23, eq148_e1709_d_b24, eq148_e1709_d_b25, eq148_e1709_d_b26, eq148_e1709_d_b27, eq148_e1709_d_b28, eq148_e1709_d_b29, eq148_e1709_d_b30, eq148_e1709_d_b31, eq148_e1709_d_b32, eq148_e1709_d_b33, eq148_e1709_d_b34, eq148_e1709_d_b35,) = {
    if s.b[1934] {
        let eq148_e1706: f64 = (s.v[0] * (nv8 - nv17));
        let eq148_e1707: f64 = (s.v[123] + eq148_e1706);
        let eq148_e1707_d_n8: f64 = (s.dn[123][8] + s.v[0]);
        let eq148_e1707_d_n17: f64 = (s.dn[123][17] + (-s.v[0]));
        (eq148_e1707, s.dn[123][0], s.dn[123][1], s.dn[123][2], s.dn[123][3], s.dn[123][4], s.dn[123][5], s.dn[123][6], s.dn[123][7], eq148_e1707_d_n8, s.dn[123][9], s.dn[123][10], s.dn[123][11], s.dn[123][12], s.dn[123][13], s.dn[123][14], s.dn[123][15], s.dn[123][16], eq148_e1707_d_n17, s.dn[123][18], s.dn[123][19], s.dn[123][20], s.dn[123][21], s.dn[123][22], s.dn[123][23], s.dn[123][24], s.dn[123][25], s.dn[123][26], s.dn[123][27], s.dn[123][28], s.dn[123][29], s.db[123][0], s.db[123][1], s.db[123][2], s.db[123][3], s.db[123][4], s.db[123][5], s.db[123][6], s.db[123][7], s.db[123][8], s.db[123][9], s.db[123][10], s.db[123][11], s.db[123][12], s.db[123][13], s.db[123][14], s.db[123][15], s.db[123][16], s.db[123][17], s.db[123][18], s.db[123][19], s.db[123][20], s.db[123][21], s.db[123][22], s.db[123][23], s.db[123][24], s.db[123][25], s.db[123][26], s.db[123][27], s.db[123][28], s.db[123][29], s.db[123][30], s.db[123][31], s.db[123][32], s.db[123][33], s.db[123][34], s.db[123][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1709;
        let eq148_node_derivatives: [f64; 30] = [eq148_e1709_d_n0, eq148_e1709_d_n1, eq148_e1709_d_n2, eq148_e1709_d_n3, eq148_e1709_d_n4, eq148_e1709_d_n5, eq148_e1709_d_n6, eq148_e1709_d_n7, eq148_e1709_d_n8, eq148_e1709_d_n9, eq148_e1709_d_n10, eq148_e1709_d_n11, eq148_e1709_d_n12, eq148_e1709_d_n13, eq148_e1709_d_n14, eq148_e1709_d_n15, eq148_e1709_d_n16, eq148_e1709_d_n17, eq148_e1709_d_n18, eq148_e1709_d_n19, eq148_e1709_d_n20, eq148_e1709_d_n21, eq148_e1709_d_n22, eq148_e1709_d_n23, eq148_e1709_d_n24, eq148_e1709_d_n25, eq148_e1709_d_n26, eq148_e1709_d_n27, eq148_e1709_d_n28, eq148_e1709_d_n29];
        let eq148_branch_derivatives: [f64; 36] = [eq148_e1709_d_b0, eq148_e1709_d_b1, eq148_e1709_d_b2, eq148_e1709_d_b3, eq148_e1709_d_b4, eq148_e1709_d_b5, eq148_e1709_d_b6, eq148_e1709_d_b7, eq148_e1709_d_b8, eq148_e1709_d_b9, eq148_e1709_d_b10, eq148_e1709_d_b11, eq148_e1709_d_b12, eq148_e1709_d_b13, eq148_e1709_d_b14, eq148_e1709_d_b15, eq148_e1709_d_b16, eq148_e1709_d_b17, eq148_e1709_d_b18, eq148_e1709_d_b19, eq148_e1709_d_b20, eq148_e1709_d_b21, eq148_e1709_d_b22, eq148_e1709_d_b23, eq148_e1709_d_b24, eq148_e1709_d_b25, eq148_e1709_d_b26, eq148_e1709_d_b27, eq148_e1709_d_b28, eq148_e1709_d_b29, eq148_e1709_d_b30, eq148_e1709_d_b31, eq148_e1709_d_b32, eq148_e1709_d_b33, eq148_e1709_d_b34, eq148_e1709_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(17),
            multiplicity * (eq148_value),
            &eq148_node_derivatives,
            &eq148_branch_derivatives,
            multiplicity,
        );
        let (eq149_e1719, eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29, eq149_e1719_d_b0, eq149_e1719_d_b1, eq149_e1719_d_b2, eq149_e1719_d_b3, eq149_e1719_d_b4, eq149_e1719_d_b5, eq149_e1719_d_b6, eq149_e1719_d_b7, eq149_e1719_d_b8, eq149_e1719_d_b9, eq149_e1719_d_b10, eq149_e1719_d_b11, eq149_e1719_d_b12, eq149_e1719_d_b13, eq149_e1719_d_b14, eq149_e1719_d_b15, eq149_e1719_d_b16, eq149_e1719_d_b17, eq149_e1719_d_b18, eq149_e1719_d_b19, eq149_e1719_d_b20, eq149_e1719_d_b21, eq149_e1719_d_b22, eq149_e1719_d_b23, eq149_e1719_d_b24, eq149_e1719_d_b25, eq149_e1719_d_b26, eq149_e1719_d_b27, eq149_e1719_d_b28, eq149_e1719_d_b29, eq149_e1719_d_b30, eq149_e1719_d_b31, eq149_e1719_d_b32, eq149_e1719_d_b33, eq149_e1719_d_b34, eq149_e1719_d_b35,) = {
    if (s.b[1934] && s.b[2055]) {
        let eq149_e1716: f64 = (s.v[0] * (nv8 - nv13));
        let eq149_e1717: f64 = (s.v[134] + eq149_e1716);
        let eq149_e1717_d_n8: f64 = (s.dn[134][8] + s.v[0]);
        let eq149_e1717_d_n13: f64 = (s.dn[134][13] + (-s.v[0]));
        (eq149_e1717, s.dn[134][0], s.dn[134][1], s.dn[134][2], s.dn[134][3], s.dn[134][4], s.dn[134][5], s.dn[134][6], s.dn[134][7], eq149_e1717_d_n8, s.dn[134][9], s.dn[134][10], s.dn[134][11], s.dn[134][12], eq149_e1717_d_n13, s.dn[134][14], s.dn[134][15], s.dn[134][16], s.dn[134][17], s.dn[134][18], s.dn[134][19], s.dn[134][20], s.dn[134][21], s.dn[134][22], s.dn[134][23], s.dn[134][24], s.dn[134][25], s.dn[134][26], s.dn[134][27], s.dn[134][28], s.dn[134][29], s.db[134][0], s.db[134][1], s.db[134][2], s.db[134][3], s.db[134][4], s.db[134][5], s.db[134][6], s.db[134][7], s.db[134][8], s.db[134][9], s.db[134][10], s.db[134][11], s.db[134][12], s.db[134][13], s.db[134][14], s.db[134][15], s.db[134][16], s.db[134][17], s.db[134][18], s.db[134][19], s.db[134][20], s.db[134][21], s.db[134][22], s.db[134][23], s.db[134][24], s.db[134][25], s.db[134][26], s.db[134][27], s.db[134][28], s.db[134][29], s.db[134][30], s.db[134][31], s.db[134][32], s.db[134][33], s.db[134][34], s.db[134][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1719;
        let eq149_node_derivatives: [f64; 30] = [eq149_e1719_d_n0, eq149_e1719_d_n1, eq149_e1719_d_n2, eq149_e1719_d_n3, eq149_e1719_d_n4, eq149_e1719_d_n5, eq149_e1719_d_n6, eq149_e1719_d_n7, eq149_e1719_d_n8, eq149_e1719_d_n9, eq149_e1719_d_n10, eq149_e1719_d_n11, eq149_e1719_d_n12, eq149_e1719_d_n13, eq149_e1719_d_n14, eq149_e1719_d_n15, eq149_e1719_d_n16, eq149_e1719_d_n17, eq149_e1719_d_n18, eq149_e1719_d_n19, eq149_e1719_d_n20, eq149_e1719_d_n21, eq149_e1719_d_n22, eq149_e1719_d_n23, eq149_e1719_d_n24, eq149_e1719_d_n25, eq149_e1719_d_n26, eq149_e1719_d_n27, eq149_e1719_d_n28, eq149_e1719_d_n29];
        let eq149_branch_derivatives: [f64; 36] = [eq149_e1719_d_b0, eq149_e1719_d_b1, eq149_e1719_d_b2, eq149_e1719_d_b3, eq149_e1719_d_b4, eq149_e1719_d_b5, eq149_e1719_d_b6, eq149_e1719_d_b7, eq149_e1719_d_b8, eq149_e1719_d_b9, eq149_e1719_d_b10, eq149_e1719_d_b11, eq149_e1719_d_b12, eq149_e1719_d_b13, eq149_e1719_d_b14, eq149_e1719_d_b15, eq149_e1719_d_b16, eq149_e1719_d_b17, eq149_e1719_d_b18, eq149_e1719_d_b19, eq149_e1719_d_b20, eq149_e1719_d_b21, eq149_e1719_d_b22, eq149_e1719_d_b23, eq149_e1719_d_b24, eq149_e1719_d_b25, eq149_e1719_d_b26, eq149_e1719_d_b27, eq149_e1719_d_b28, eq149_e1719_d_b29, eq149_e1719_d_b30, eq149_e1719_d_b31, eq149_e1719_d_b32, eq149_e1719_d_b33, eq149_e1719_d_b34, eq149_e1719_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(13),
            multiplicity * (eq149_value),
            &eq149_node_derivatives,
            &eq149_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1729, eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29, eq150_e1729_d_b0, eq150_e1729_d_b1, eq150_e1729_d_b2, eq150_e1729_d_b3, eq150_e1729_d_b4, eq150_e1729_d_b5, eq150_e1729_d_b6, eq150_e1729_d_b7, eq150_e1729_d_b8, eq150_e1729_d_b9, eq150_e1729_d_b10, eq150_e1729_d_b11, eq150_e1729_d_b12, eq150_e1729_d_b13, eq150_e1729_d_b14, eq150_e1729_d_b15, eq150_e1729_d_b16, eq150_e1729_d_b17, eq150_e1729_d_b18, eq150_e1729_d_b19, eq150_e1729_d_b20, eq150_e1729_d_b21, eq150_e1729_d_b22, eq150_e1729_d_b23, eq150_e1729_d_b24, eq150_e1729_d_b25, eq150_e1729_d_b26, eq150_e1729_d_b27, eq150_e1729_d_b28, eq150_e1729_d_b29, eq150_e1729_d_b30, eq150_e1729_d_b31, eq150_e1729_d_b32, eq150_e1729_d_b33, eq150_e1729_d_b34, eq150_e1729_d_b35,) = {
    if (s.b[1934] && s.b[2055]) {
        let eq150_e1726: f64 = (s.v[0] * (nv8 - nv17));
        let eq150_e1727: f64 = (s.v[135] + eq150_e1726);
        let eq150_e1727_d_n8: f64 = (s.dn[135][8] + s.v[0]);
        let eq150_e1727_d_n17: f64 = (s.dn[135][17] + (-s.v[0]));
        (eq150_e1727, s.dn[135][0], s.dn[135][1], s.dn[135][2], s.dn[135][3], s.dn[135][4], s.dn[135][5], s.dn[135][6], s.dn[135][7], eq150_e1727_d_n8, s.dn[135][9], s.dn[135][10], s.dn[135][11], s.dn[135][12], s.dn[135][13], s.dn[135][14], s.dn[135][15], s.dn[135][16], eq150_e1727_d_n17, s.dn[135][18], s.dn[135][19], s.dn[135][20], s.dn[135][21], s.dn[135][22], s.dn[135][23], s.dn[135][24], s.dn[135][25], s.dn[135][26], s.dn[135][27], s.dn[135][28], s.dn[135][29], s.db[135][0], s.db[135][1], s.db[135][2], s.db[135][3], s.db[135][4], s.db[135][5], s.db[135][6], s.db[135][7], s.db[135][8], s.db[135][9], s.db[135][10], s.db[135][11], s.db[135][12], s.db[135][13], s.db[135][14], s.db[135][15], s.db[135][16], s.db[135][17], s.db[135][18], s.db[135][19], s.db[135][20], s.db[135][21], s.db[135][22], s.db[135][23], s.db[135][24], s.db[135][25], s.db[135][26], s.db[135][27], s.db[135][28], s.db[135][29], s.db[135][30], s.db[135][31], s.db[135][32], s.db[135][33], s.db[135][34], s.db[135][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1729;
        let eq150_node_derivatives: [f64; 30] = [eq150_e1729_d_n0, eq150_e1729_d_n1, eq150_e1729_d_n2, eq150_e1729_d_n3, eq150_e1729_d_n4, eq150_e1729_d_n5, eq150_e1729_d_n6, eq150_e1729_d_n7, eq150_e1729_d_n8, eq150_e1729_d_n9, eq150_e1729_d_n10, eq150_e1729_d_n11, eq150_e1729_d_n12, eq150_e1729_d_n13, eq150_e1729_d_n14, eq150_e1729_d_n15, eq150_e1729_d_n16, eq150_e1729_d_n17, eq150_e1729_d_n18, eq150_e1729_d_n19, eq150_e1729_d_n20, eq150_e1729_d_n21, eq150_e1729_d_n22, eq150_e1729_d_n23, eq150_e1729_d_n24, eq150_e1729_d_n25, eq150_e1729_d_n26, eq150_e1729_d_n27, eq150_e1729_d_n28, eq150_e1729_d_n29];
        let eq150_branch_derivatives: [f64; 36] = [eq150_e1729_d_b0, eq150_e1729_d_b1, eq150_e1729_d_b2, eq150_e1729_d_b3, eq150_e1729_d_b4, eq150_e1729_d_b5, eq150_e1729_d_b6, eq150_e1729_d_b7, eq150_e1729_d_b8, eq150_e1729_d_b9, eq150_e1729_d_b10, eq150_e1729_d_b11, eq150_e1729_d_b12, eq150_e1729_d_b13, eq150_e1729_d_b14, eq150_e1729_d_b15, eq150_e1729_d_b16, eq150_e1729_d_b17, eq150_e1729_d_b18, eq150_e1729_d_b19, eq150_e1729_d_b20, eq150_e1729_d_b21, eq150_e1729_d_b22, eq150_e1729_d_b23, eq150_e1729_d_b24, eq150_e1729_d_b25, eq150_e1729_d_b26, eq150_e1729_d_b27, eq150_e1729_d_b28, eq150_e1729_d_b29, eq150_e1729_d_b30, eq150_e1729_d_b31, eq150_e1729_d_b32, eq150_e1729_d_b33, eq150_e1729_d_b34, eq150_e1729_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(17),
            multiplicity * (eq150_value),
            &eq150_node_derivatives,
            &eq150_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1739, eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29, eq151_e1739_d_b0, eq151_e1739_d_b1, eq151_e1739_d_b2, eq151_e1739_d_b3, eq151_e1739_d_b4, eq151_e1739_d_b5, eq151_e1739_d_b6, eq151_e1739_d_b7, eq151_e1739_d_b8, eq151_e1739_d_b9, eq151_e1739_d_b10, eq151_e1739_d_b11, eq151_e1739_d_b12, eq151_e1739_d_b13, eq151_e1739_d_b14, eq151_e1739_d_b15, eq151_e1739_d_b16, eq151_e1739_d_b17, eq151_e1739_d_b18, eq151_e1739_d_b19, eq151_e1739_d_b20, eq151_e1739_d_b21, eq151_e1739_d_b22, eq151_e1739_d_b23, eq151_e1739_d_b24, eq151_e1739_d_b25, eq151_e1739_d_b26, eq151_e1739_d_b27, eq151_e1739_d_b28, eq151_e1739_d_b29, eq151_e1739_d_b30, eq151_e1739_d_b31, eq151_e1739_d_b32, eq151_e1739_d_b33, eq151_e1739_d_b34, eq151_e1739_d_b35,) = {
    if (s.b[1934] && s.b[2176]) {
        let eq151_e1736: f64 = (s.v[0] * (nv8 - nv9));
        let eq151_e1737: f64 = (s.v[128] + eq151_e1736);
        let eq151_e1737_d_n8: f64 = (s.dn[128][8] + s.v[0]);
        let eq151_e1737_d_n9: f64 = (s.dn[128][9] + (-s.v[0]));
        (eq151_e1737, s.dn[128][0], s.dn[128][1], s.dn[128][2], s.dn[128][3], s.dn[128][4], s.dn[128][5], s.dn[128][6], s.dn[128][7], eq151_e1737_d_n8, eq151_e1737_d_n9, s.dn[128][10], s.dn[128][11], s.dn[128][12], s.dn[128][13], s.dn[128][14], s.dn[128][15], s.dn[128][16], s.dn[128][17], s.dn[128][18], s.dn[128][19], s.dn[128][20], s.dn[128][21], s.dn[128][22], s.dn[128][23], s.dn[128][24], s.dn[128][25], s.dn[128][26], s.dn[128][27], s.dn[128][28], s.dn[128][29], s.db[128][0], s.db[128][1], s.db[128][2], s.db[128][3], s.db[128][4], s.db[128][5], s.db[128][6], s.db[128][7], s.db[128][8], s.db[128][9], s.db[128][10], s.db[128][11], s.db[128][12], s.db[128][13], s.db[128][14], s.db[128][15], s.db[128][16], s.db[128][17], s.db[128][18], s.db[128][19], s.db[128][20], s.db[128][21], s.db[128][22], s.db[128][23], s.db[128][24], s.db[128][25], s.db[128][26], s.db[128][27], s.db[128][28], s.db[128][29], s.db[128][30], s.db[128][31], s.db[128][32], s.db[128][33], s.db[128][34], s.db[128][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1739;
        let eq151_node_derivatives: [f64; 30] = [eq151_e1739_d_n0, eq151_e1739_d_n1, eq151_e1739_d_n2, eq151_e1739_d_n3, eq151_e1739_d_n4, eq151_e1739_d_n5, eq151_e1739_d_n6, eq151_e1739_d_n7, eq151_e1739_d_n8, eq151_e1739_d_n9, eq151_e1739_d_n10, eq151_e1739_d_n11, eq151_e1739_d_n12, eq151_e1739_d_n13, eq151_e1739_d_n14, eq151_e1739_d_n15, eq151_e1739_d_n16, eq151_e1739_d_n17, eq151_e1739_d_n18, eq151_e1739_d_n19, eq151_e1739_d_n20, eq151_e1739_d_n21, eq151_e1739_d_n22, eq151_e1739_d_n23, eq151_e1739_d_n24, eq151_e1739_d_n25, eq151_e1739_d_n26, eq151_e1739_d_n27, eq151_e1739_d_n28, eq151_e1739_d_n29];
        let eq151_branch_derivatives: [f64; 36] = [eq151_e1739_d_b0, eq151_e1739_d_b1, eq151_e1739_d_b2, eq151_e1739_d_b3, eq151_e1739_d_b4, eq151_e1739_d_b5, eq151_e1739_d_b6, eq151_e1739_d_b7, eq151_e1739_d_b8, eq151_e1739_d_b9, eq151_e1739_d_b10, eq151_e1739_d_b11, eq151_e1739_d_b12, eq151_e1739_d_b13, eq151_e1739_d_b14, eq151_e1739_d_b15, eq151_e1739_d_b16, eq151_e1739_d_b17, eq151_e1739_d_b18, eq151_e1739_d_b19, eq151_e1739_d_b20, eq151_e1739_d_b21, eq151_e1739_d_b22, eq151_e1739_d_b23, eq151_e1739_d_b24, eq151_e1739_d_b25, eq151_e1739_d_b26, eq151_e1739_d_b27, eq151_e1739_d_b28, eq151_e1739_d_b29, eq151_e1739_d_b30, eq151_e1739_d_b31, eq151_e1739_d_b32, eq151_e1739_d_b33, eq151_e1739_d_b34, eq151_e1739_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq151_value),
            &eq151_node_derivatives,
            &eq151_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1749, eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29, eq152_e1749_d_b0, eq152_e1749_d_b1, eq152_e1749_d_b2, eq152_e1749_d_b3, eq152_e1749_d_b4, eq152_e1749_d_b5, eq152_e1749_d_b6, eq152_e1749_d_b7, eq152_e1749_d_b8, eq152_e1749_d_b9, eq152_e1749_d_b10, eq152_e1749_d_b11, eq152_e1749_d_b12, eq152_e1749_d_b13, eq152_e1749_d_b14, eq152_e1749_d_b15, eq152_e1749_d_b16, eq152_e1749_d_b17, eq152_e1749_d_b18, eq152_e1749_d_b19, eq152_e1749_d_b20, eq152_e1749_d_b21, eq152_e1749_d_b22, eq152_e1749_d_b23, eq152_e1749_d_b24, eq152_e1749_d_b25, eq152_e1749_d_b26, eq152_e1749_d_b27, eq152_e1749_d_b28, eq152_e1749_d_b29, eq152_e1749_d_b30, eq152_e1749_d_b31, eq152_e1749_d_b32, eq152_e1749_d_b33, eq152_e1749_d_b34, eq152_e1749_d_b35,) = {
    if (s.b[1934] && s.b[2176]) {
        let eq152_e1746: f64 = (s.v[0] * (nv8 - nv5));
        let eq152_e1747: f64 = (s.v[129] + eq152_e1746);
        let eq152_e1747_d_n5: f64 = (s.dn[129][5] + (-s.v[0]));
        let eq152_e1747_d_n8: f64 = (s.dn[129][8] + s.v[0]);
        (eq152_e1747, s.dn[129][0], s.dn[129][1], s.dn[129][2], s.dn[129][3], s.dn[129][4], eq152_e1747_d_n5, s.dn[129][6], s.dn[129][7], eq152_e1747_d_n8, s.dn[129][9], s.dn[129][10], s.dn[129][11], s.dn[129][12], s.dn[129][13], s.dn[129][14], s.dn[129][15], s.dn[129][16], s.dn[129][17], s.dn[129][18], s.dn[129][19], s.dn[129][20], s.dn[129][21], s.dn[129][22], s.dn[129][23], s.dn[129][24], s.dn[129][25], s.dn[129][26], s.dn[129][27], s.dn[129][28], s.dn[129][29], s.db[129][0], s.db[129][1], s.db[129][2], s.db[129][3], s.db[129][4], s.db[129][5], s.db[129][6], s.db[129][7], s.db[129][8], s.db[129][9], s.db[129][10], s.db[129][11], s.db[129][12], s.db[129][13], s.db[129][14], s.db[129][15], s.db[129][16], s.db[129][17], s.db[129][18], s.db[129][19], s.db[129][20], s.db[129][21], s.db[129][22], s.db[129][23], s.db[129][24], s.db[129][25], s.db[129][26], s.db[129][27], s.db[129][28], s.db[129][29], s.db[129][30], s.db[129][31], s.db[129][32], s.db[129][33], s.db[129][34], s.db[129][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1749;
        let eq152_node_derivatives: [f64; 30] = [eq152_e1749_d_n0, eq152_e1749_d_n1, eq152_e1749_d_n2, eq152_e1749_d_n3, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n6, eq152_e1749_d_n7, eq152_e1749_d_n8, eq152_e1749_d_n9, eq152_e1749_d_n10, eq152_e1749_d_n11, eq152_e1749_d_n12, eq152_e1749_d_n13, eq152_e1749_d_n14, eq152_e1749_d_n15, eq152_e1749_d_n16, eq152_e1749_d_n17, eq152_e1749_d_n18, eq152_e1749_d_n19, eq152_e1749_d_n20, eq152_e1749_d_n21, eq152_e1749_d_n22, eq152_e1749_d_n23, eq152_e1749_d_n24, eq152_e1749_d_n25, eq152_e1749_d_n26, eq152_e1749_d_n27, eq152_e1749_d_n28, eq152_e1749_d_n29];
        let eq152_branch_derivatives: [f64; 36] = [eq152_e1749_d_b0, eq152_e1749_d_b1, eq152_e1749_d_b2, eq152_e1749_d_b3, eq152_e1749_d_b4, eq152_e1749_d_b5, eq152_e1749_d_b6, eq152_e1749_d_b7, eq152_e1749_d_b8, eq152_e1749_d_b9, eq152_e1749_d_b10, eq152_e1749_d_b11, eq152_e1749_d_b12, eq152_e1749_d_b13, eq152_e1749_d_b14, eq152_e1749_d_b15, eq152_e1749_d_b16, eq152_e1749_d_b17, eq152_e1749_d_b18, eq152_e1749_d_b19, eq152_e1749_d_b20, eq152_e1749_d_b21, eq152_e1749_d_b22, eq152_e1749_d_b23, eq152_e1749_d_b24, eq152_e1749_d_b25, eq152_e1749_d_b26, eq152_e1749_d_b27, eq152_e1749_d_b28, eq152_e1749_d_b29, eq152_e1749_d_b30, eq152_e1749_d_b31, eq152_e1749_d_b32, eq152_e1749_d_b33, eq152_e1749_d_b34, eq152_e1749_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq152_value),
            &eq152_node_derivatives,
            &eq152_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1761, eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29, eq153_e1761_d_b0, eq153_e1761_d_b1, eq153_e1761_d_b2, eq153_e1761_d_b3, eq153_e1761_d_b4, eq153_e1761_d_b5, eq153_e1761_d_b6, eq153_e1761_d_b7, eq153_e1761_d_b8, eq153_e1761_d_b9, eq153_e1761_d_b10, eq153_e1761_d_b11, eq153_e1761_d_b12, eq153_e1761_d_b13, eq153_e1761_d_b14, eq153_e1761_d_b15, eq153_e1761_d_b16, eq153_e1761_d_b17, eq153_e1761_d_b18, eq153_e1761_d_b19, eq153_e1761_d_b20, eq153_e1761_d_b21, eq153_e1761_d_b22, eq153_e1761_d_b23, eq153_e1761_d_b24, eq153_e1761_d_b25, eq153_e1761_d_b26, eq153_e1761_d_b27, eq153_e1761_d_b28, eq153_e1761_d_b29, eq153_e1761_d_b30, eq153_e1761_d_b31, eq153_e1761_d_b32, eq153_e1761_d_b33, eq153_e1761_d_b34, eq153_e1761_d_b35,) = {
    if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
        let eq153_e1758: f64 = (s.v[0] * (nv8 - nv9));
        let eq153_e1759: f64 = (s.v[140] + eq153_e1758);
        let eq153_e1759_d_n8: f64 = (s.dn[140][8] + s.v[0]);
        let eq153_e1759_d_n9: f64 = (s.dn[140][9] + (-s.v[0]));
        (eq153_e1759, s.dn[140][0], s.dn[140][1], s.dn[140][2], s.dn[140][3], s.dn[140][4], s.dn[140][5], s.dn[140][6], s.dn[140][7], eq153_e1759_d_n8, eq153_e1759_d_n9, s.dn[140][10], s.dn[140][11], s.dn[140][12], s.dn[140][13], s.dn[140][14], s.dn[140][15], s.dn[140][16], s.dn[140][17], s.dn[140][18], s.dn[140][19], s.dn[140][20], s.dn[140][21], s.dn[140][22], s.dn[140][23], s.dn[140][24], s.dn[140][25], s.dn[140][26], s.dn[140][27], s.dn[140][28], s.dn[140][29], s.db[140][0], s.db[140][1], s.db[140][2], s.db[140][3], s.db[140][4], s.db[140][5], s.db[140][6], s.db[140][7], s.db[140][8], s.db[140][9], s.db[140][10], s.db[140][11], s.db[140][12], s.db[140][13], s.db[140][14], s.db[140][15], s.db[140][16], s.db[140][17], s.db[140][18], s.db[140][19], s.db[140][20], s.db[140][21], s.db[140][22], s.db[140][23], s.db[140][24], s.db[140][25], s.db[140][26], s.db[140][27], s.db[140][28], s.db[140][29], s.db[140][30], s.db[140][31], s.db[140][32], s.db[140][33], s.db[140][34], s.db[140][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1761;
        let eq153_node_derivatives: [f64; 30] = [eq153_e1761_d_n0, eq153_e1761_d_n1, eq153_e1761_d_n2, eq153_e1761_d_n3, eq153_e1761_d_n4, eq153_e1761_d_n5, eq153_e1761_d_n6, eq153_e1761_d_n7, eq153_e1761_d_n8, eq153_e1761_d_n9, eq153_e1761_d_n10, eq153_e1761_d_n11, eq153_e1761_d_n12, eq153_e1761_d_n13, eq153_e1761_d_n14, eq153_e1761_d_n15, eq153_e1761_d_n16, eq153_e1761_d_n17, eq153_e1761_d_n18, eq153_e1761_d_n19, eq153_e1761_d_n20, eq153_e1761_d_n21, eq153_e1761_d_n22, eq153_e1761_d_n23, eq153_e1761_d_n24, eq153_e1761_d_n25, eq153_e1761_d_n26, eq153_e1761_d_n27, eq153_e1761_d_n28, eq153_e1761_d_n29];
        let eq153_branch_derivatives: [f64; 36] = [eq153_e1761_d_b0, eq153_e1761_d_b1, eq153_e1761_d_b2, eq153_e1761_d_b3, eq153_e1761_d_b4, eq153_e1761_d_b5, eq153_e1761_d_b6, eq153_e1761_d_b7, eq153_e1761_d_b8, eq153_e1761_d_b9, eq153_e1761_d_b10, eq153_e1761_d_b11, eq153_e1761_d_b12, eq153_e1761_d_b13, eq153_e1761_d_b14, eq153_e1761_d_b15, eq153_e1761_d_b16, eq153_e1761_d_b17, eq153_e1761_d_b18, eq153_e1761_d_b19, eq153_e1761_d_b20, eq153_e1761_d_b21, eq153_e1761_d_b22, eq153_e1761_d_b23, eq153_e1761_d_b24, eq153_e1761_d_b25, eq153_e1761_d_b26, eq153_e1761_d_b27, eq153_e1761_d_b28, eq153_e1761_d_b29, eq153_e1761_d_b30, eq153_e1761_d_b31, eq153_e1761_d_b32, eq153_e1761_d_b33, eq153_e1761_d_b34, eq153_e1761_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq153_value),
            &eq153_node_derivatives,
            &eq153_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1773, eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29, eq154_e1773_d_b0, eq154_e1773_d_b1, eq154_e1773_d_b2, eq154_e1773_d_b3, eq154_e1773_d_b4, eq154_e1773_d_b5, eq154_e1773_d_b6, eq154_e1773_d_b7, eq154_e1773_d_b8, eq154_e1773_d_b9, eq154_e1773_d_b10, eq154_e1773_d_b11, eq154_e1773_d_b12, eq154_e1773_d_b13, eq154_e1773_d_b14, eq154_e1773_d_b15, eq154_e1773_d_b16, eq154_e1773_d_b17, eq154_e1773_d_b18, eq154_e1773_d_b19, eq154_e1773_d_b20, eq154_e1773_d_b21, eq154_e1773_d_b22, eq154_e1773_d_b23, eq154_e1773_d_b24, eq154_e1773_d_b25, eq154_e1773_d_b26, eq154_e1773_d_b27, eq154_e1773_d_b28, eq154_e1773_d_b29, eq154_e1773_d_b30, eq154_e1773_d_b31, eq154_e1773_d_b32, eq154_e1773_d_b33, eq154_e1773_d_b34, eq154_e1773_d_b35,) = {
    if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
        let eq154_e1770: f64 = (s.v[0] * (nv8 - nv5));
        let eq154_e1771: f64 = (s.v[141] + eq154_e1770);
        let eq154_e1771_d_n5: f64 = (s.dn[141][5] + (-s.v[0]));
        let eq154_e1771_d_n8: f64 = (s.dn[141][8] + s.v[0]);
        (eq154_e1771, s.dn[141][0], s.dn[141][1], s.dn[141][2], s.dn[141][3], s.dn[141][4], eq154_e1771_d_n5, s.dn[141][6], s.dn[141][7], eq154_e1771_d_n8, s.dn[141][9], s.dn[141][10], s.dn[141][11], s.dn[141][12], s.dn[141][13], s.dn[141][14], s.dn[141][15], s.dn[141][16], s.dn[141][17], s.dn[141][18], s.dn[141][19], s.dn[141][20], s.dn[141][21], s.dn[141][22], s.dn[141][23], s.dn[141][24], s.dn[141][25], s.dn[141][26], s.dn[141][27], s.dn[141][28], s.dn[141][29], s.db[141][0], s.db[141][1], s.db[141][2], s.db[141][3], s.db[141][4], s.db[141][5], s.db[141][6], s.db[141][7], s.db[141][8], s.db[141][9], s.db[141][10], s.db[141][11], s.db[141][12], s.db[141][13], s.db[141][14], s.db[141][15], s.db[141][16], s.db[141][17], s.db[141][18], s.db[141][19], s.db[141][20], s.db[141][21], s.db[141][22], s.db[141][23], s.db[141][24], s.db[141][25], s.db[141][26], s.db[141][27], s.db[141][28], s.db[141][29], s.db[141][30], s.db[141][31], s.db[141][32], s.db[141][33], s.db[141][34], s.db[141][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1773;
        let eq154_node_derivatives: [f64; 30] = [eq154_e1773_d_n0, eq154_e1773_d_n1, eq154_e1773_d_n2, eq154_e1773_d_n3, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n6, eq154_e1773_d_n7, eq154_e1773_d_n8, eq154_e1773_d_n9, eq154_e1773_d_n10, eq154_e1773_d_n11, eq154_e1773_d_n12, eq154_e1773_d_n13, eq154_e1773_d_n14, eq154_e1773_d_n15, eq154_e1773_d_n16, eq154_e1773_d_n17, eq154_e1773_d_n18, eq154_e1773_d_n19, eq154_e1773_d_n20, eq154_e1773_d_n21, eq154_e1773_d_n22, eq154_e1773_d_n23, eq154_e1773_d_n24, eq154_e1773_d_n25, eq154_e1773_d_n26, eq154_e1773_d_n27, eq154_e1773_d_n28, eq154_e1773_d_n29];
        let eq154_branch_derivatives: [f64; 36] = [eq154_e1773_d_b0, eq154_e1773_d_b1, eq154_e1773_d_b2, eq154_e1773_d_b3, eq154_e1773_d_b4, eq154_e1773_d_b5, eq154_e1773_d_b6, eq154_e1773_d_b7, eq154_e1773_d_b8, eq154_e1773_d_b9, eq154_e1773_d_b10, eq154_e1773_d_b11, eq154_e1773_d_b12, eq154_e1773_d_b13, eq154_e1773_d_b14, eq154_e1773_d_b15, eq154_e1773_d_b16, eq154_e1773_d_b17, eq154_e1773_d_b18, eq154_e1773_d_b19, eq154_e1773_d_b20, eq154_e1773_d_b21, eq154_e1773_d_b22, eq154_e1773_d_b23, eq154_e1773_d_b24, eq154_e1773_d_b25, eq154_e1773_d_b26, eq154_e1773_d_b27, eq154_e1773_d_b28, eq154_e1773_d_b29, eq154_e1773_d_b30, eq154_e1773_d_b31, eq154_e1773_d_b32, eq154_e1773_d_b33, eq154_e1773_d_b34, eq154_e1773_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq154_value),
            &eq154_node_derivatives,
            &eq154_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1781, eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29, eq155_e1781_d_b0, eq155_e1781_d_b1, eq155_e1781_d_b2, eq155_e1781_d_b3, eq155_e1781_d_b4, eq155_e1781_d_b5, eq155_e1781_d_b6, eq155_e1781_d_b7, eq155_e1781_d_b8, eq155_e1781_d_b9, eq155_e1781_d_b10, eq155_e1781_d_b11, eq155_e1781_d_b12, eq155_e1781_d_b13, eq155_e1781_d_b14, eq155_e1781_d_b15, eq155_e1781_d_b16, eq155_e1781_d_b17, eq155_e1781_d_b18, eq155_e1781_d_b19, eq155_e1781_d_b20, eq155_e1781_d_b21, eq155_e1781_d_b22, eq155_e1781_d_b23, eq155_e1781_d_b24, eq155_e1781_d_b25, eq155_e1781_d_b26, eq155_e1781_d_b27, eq155_e1781_d_b28, eq155_e1781_d_b29, eq155_e1781_d_b30, eq155_e1781_d_b31, eq155_e1781_d_b32, eq155_e1781_d_b33, eq155_e1781_d_b34, eq155_e1781_d_b35,) = {
    if s.b[2418] {
        let eq155_e1778: f64 = (s.v[0] * (nv8 - nv7));
        let eq155_e1779: f64 = (s.v[235] + eq155_e1778);
        let eq155_e1779_d_n7: f64 = (s.dn[235][7] + (-s.v[0]));
        let eq155_e1779_d_n8: f64 = (s.dn[235][8] + s.v[0]);
        (eq155_e1779, s.dn[235][0], s.dn[235][1], s.dn[235][2], s.dn[235][3], s.dn[235][4], s.dn[235][5], s.dn[235][6], eq155_e1779_d_n7, eq155_e1779_d_n8, s.dn[235][9], s.dn[235][10], s.dn[235][11], s.dn[235][12], s.dn[235][13], s.dn[235][14], s.dn[235][15], s.dn[235][16], s.dn[235][17], s.dn[235][18], s.dn[235][19], s.dn[235][20], s.dn[235][21], s.dn[235][22], s.dn[235][23], s.dn[235][24], s.dn[235][25], s.dn[235][26], s.dn[235][27], s.dn[235][28], s.dn[235][29], s.db[235][0], s.db[235][1], s.db[235][2], s.db[235][3], s.db[235][4], s.db[235][5], s.db[235][6], s.db[235][7], s.db[235][8], s.db[235][9], s.db[235][10], s.db[235][11], s.db[235][12], s.db[235][13], s.db[235][14], s.db[235][15], s.db[235][16], s.db[235][17], s.db[235][18], s.db[235][19], s.db[235][20], s.db[235][21], s.db[235][22], s.db[235][23], s.db[235][24], s.db[235][25], s.db[235][26], s.db[235][27], s.db[235][28], s.db[235][29], s.db[235][30], s.db[235][31], s.db[235][32], s.db[235][33], s.db[235][34], s.db[235][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1781;
        let eq155_node_derivatives: [f64; 30] = [eq155_e1781_d_n0, eq155_e1781_d_n1, eq155_e1781_d_n2, eq155_e1781_d_n3, eq155_e1781_d_n4, eq155_e1781_d_n5, eq155_e1781_d_n6, eq155_e1781_d_n7, eq155_e1781_d_n8, eq155_e1781_d_n9, eq155_e1781_d_n10, eq155_e1781_d_n11, eq155_e1781_d_n12, eq155_e1781_d_n13, eq155_e1781_d_n14, eq155_e1781_d_n15, eq155_e1781_d_n16, eq155_e1781_d_n17, eq155_e1781_d_n18, eq155_e1781_d_n19, eq155_e1781_d_n20, eq155_e1781_d_n21, eq155_e1781_d_n22, eq155_e1781_d_n23, eq155_e1781_d_n24, eq155_e1781_d_n25, eq155_e1781_d_n26, eq155_e1781_d_n27, eq155_e1781_d_n28, eq155_e1781_d_n29];
        let eq155_branch_derivatives: [f64; 36] = [eq155_e1781_d_b0, eq155_e1781_d_b1, eq155_e1781_d_b2, eq155_e1781_d_b3, eq155_e1781_d_b4, eq155_e1781_d_b5, eq155_e1781_d_b6, eq155_e1781_d_b7, eq155_e1781_d_b8, eq155_e1781_d_b9, eq155_e1781_d_b10, eq155_e1781_d_b11, eq155_e1781_d_b12, eq155_e1781_d_b13, eq155_e1781_d_b14, eq155_e1781_d_b15, eq155_e1781_d_b16, eq155_e1781_d_b17, eq155_e1781_d_b18, eq155_e1781_d_b19, eq155_e1781_d_b20, eq155_e1781_d_b21, eq155_e1781_d_b22, eq155_e1781_d_b23, eq155_e1781_d_b24, eq155_e1781_d_b25, eq155_e1781_d_b26, eq155_e1781_d_b27, eq155_e1781_d_b28, eq155_e1781_d_b29, eq155_e1781_d_b30, eq155_e1781_d_b31, eq155_e1781_d_b32, eq155_e1781_d_b33, eq155_e1781_d_b34, eq155_e1781_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq155_value),
            &eq155_node_derivatives,
            &eq155_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1791, eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29, eq156_e1791_d_b0, eq156_e1791_d_b1, eq156_e1791_d_b2, eq156_e1791_d_b3, eq156_e1791_d_b4, eq156_e1791_d_b5, eq156_e1791_d_b6, eq156_e1791_d_b7, eq156_e1791_d_b8, eq156_e1791_d_b9, eq156_e1791_d_b10, eq156_e1791_d_b11, eq156_e1791_d_b12, eq156_e1791_d_b13, eq156_e1791_d_b14, eq156_e1791_d_b15, eq156_e1791_d_b16, eq156_e1791_d_b17, eq156_e1791_d_b18, eq156_e1791_d_b19, eq156_e1791_d_b20, eq156_e1791_d_b21, eq156_e1791_d_b22, eq156_e1791_d_b23, eq156_e1791_d_b24, eq156_e1791_d_b25, eq156_e1791_d_b26, eq156_e1791_d_b27, eq156_e1791_d_b28, eq156_e1791_d_b29, eq156_e1791_d_b30, eq156_e1791_d_b31, eq156_e1791_d_b32, eq156_e1791_d_b33, eq156_e1791_d_b34, eq156_e1791_d_b35,) = {
    if (s.b[2418] && s.b[2479]) {
        let eq156_e1788: f64 = (s.v[0] * (nv8 - nv7));
        let eq156_e1789: f64 = (s.v[238] + eq156_e1788);
        let eq156_e1789_d_n7: f64 = (s.dn[238][7] + (-s.v[0]));
        let eq156_e1789_d_n8: f64 = (s.dn[238][8] + s.v[0]);
        (eq156_e1789, s.dn[238][0], s.dn[238][1], s.dn[238][2], s.dn[238][3], s.dn[238][4], s.dn[238][5], s.dn[238][6], eq156_e1789_d_n7, eq156_e1789_d_n8, s.dn[238][9], s.dn[238][10], s.dn[238][11], s.dn[238][12], s.dn[238][13], s.dn[238][14], s.dn[238][15], s.dn[238][16], s.dn[238][17], s.dn[238][18], s.dn[238][19], s.dn[238][20], s.dn[238][21], s.dn[238][22], s.dn[238][23], s.dn[238][24], s.dn[238][25], s.dn[238][26], s.dn[238][27], s.dn[238][28], s.dn[238][29], s.db[238][0], s.db[238][1], s.db[238][2], s.db[238][3], s.db[238][4], s.db[238][5], s.db[238][6], s.db[238][7], s.db[238][8], s.db[238][9], s.db[238][10], s.db[238][11], s.db[238][12], s.db[238][13], s.db[238][14], s.db[238][15], s.db[238][16], s.db[238][17], s.db[238][18], s.db[238][19], s.db[238][20], s.db[238][21], s.db[238][22], s.db[238][23], s.db[238][24], s.db[238][25], s.db[238][26], s.db[238][27], s.db[238][28], s.db[238][29], s.db[238][30], s.db[238][31], s.db[238][32], s.db[238][33], s.db[238][34], s.db[238][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1791;
        let eq156_node_derivatives: [f64; 30] = [eq156_e1791_d_n0, eq156_e1791_d_n1, eq156_e1791_d_n2, eq156_e1791_d_n3, eq156_e1791_d_n4, eq156_e1791_d_n5, eq156_e1791_d_n6, eq156_e1791_d_n7, eq156_e1791_d_n8, eq156_e1791_d_n9, eq156_e1791_d_n10, eq156_e1791_d_n11, eq156_e1791_d_n12, eq156_e1791_d_n13, eq156_e1791_d_n14, eq156_e1791_d_n15, eq156_e1791_d_n16, eq156_e1791_d_n17, eq156_e1791_d_n18, eq156_e1791_d_n19, eq156_e1791_d_n20, eq156_e1791_d_n21, eq156_e1791_d_n22, eq156_e1791_d_n23, eq156_e1791_d_n24, eq156_e1791_d_n25, eq156_e1791_d_n26, eq156_e1791_d_n27, eq156_e1791_d_n28, eq156_e1791_d_n29];
        let eq156_branch_derivatives: [f64; 36] = [eq156_e1791_d_b0, eq156_e1791_d_b1, eq156_e1791_d_b2, eq156_e1791_d_b3, eq156_e1791_d_b4, eq156_e1791_d_b5, eq156_e1791_d_b6, eq156_e1791_d_b7, eq156_e1791_d_b8, eq156_e1791_d_b9, eq156_e1791_d_b10, eq156_e1791_d_b11, eq156_e1791_d_b12, eq156_e1791_d_b13, eq156_e1791_d_b14, eq156_e1791_d_b15, eq156_e1791_d_b16, eq156_e1791_d_b17, eq156_e1791_d_b18, eq156_e1791_d_b19, eq156_e1791_d_b20, eq156_e1791_d_b21, eq156_e1791_d_b22, eq156_e1791_d_b23, eq156_e1791_d_b24, eq156_e1791_d_b25, eq156_e1791_d_b26, eq156_e1791_d_b27, eq156_e1791_d_b28, eq156_e1791_d_b29, eq156_e1791_d_b30, eq156_e1791_d_b31, eq156_e1791_d_b32, eq156_e1791_d_b33, eq156_e1791_d_b34, eq156_e1791_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq156_value),
            &eq156_node_derivatives,
            &eq156_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29, eq157_e1796_d_b0, eq157_e1796_d_b1, eq157_e1796_d_b2, eq157_e1796_d_b3, eq157_e1796_d_b4, eq157_e1796_d_b5, eq157_e1796_d_b6, eq157_e1796_d_b7, eq157_e1796_d_b8, eq157_e1796_d_b9, eq157_e1796_d_b10, eq157_e1796_d_b11, eq157_e1796_d_b12, eq157_e1796_d_b13, eq157_e1796_d_b14, eq157_e1796_d_b15, eq157_e1796_d_b16, eq157_e1796_d_b17, eq157_e1796_d_b18, eq157_e1796_d_b19, eq157_e1796_d_b20, eq157_e1796_d_b21, eq157_e1796_d_b22, eq157_e1796_d_b23, eq157_e1796_d_b24, eq157_e1796_d_b25, eq157_e1796_d_b26, eq157_e1796_d_b27, eq157_e1796_d_b28, eq157_e1796_d_b29, eq157_e1796_d_b30, eq157_e1796_d_b31, eq157_e1796_d_b32, eq157_e1796_d_b33, eq157_e1796_d_b34, eq157_e1796_d_b35,) = {
    if s.b[2418] {
        let eq157_e1794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 138, s.v[242]);
        (eq157_e1794, (s.dn[242][0] * ddt_scale), (s.dn[242][1] * ddt_scale), (s.dn[242][2] * ddt_scale), (s.dn[242][3] * ddt_scale), (s.dn[242][4] * ddt_scale), (s.dn[242][5] * ddt_scale), (s.dn[242][6] * ddt_scale), (s.dn[242][7] * ddt_scale), (s.dn[242][8] * ddt_scale), (s.dn[242][9] * ddt_scale), (s.dn[242][10] * ddt_scale), (s.dn[242][11] * ddt_scale), (s.dn[242][12] * ddt_scale), (s.dn[242][13] * ddt_scale), (s.dn[242][14] * ddt_scale), (s.dn[242][15] * ddt_scale), (s.dn[242][16] * ddt_scale), (s.dn[242][17] * ddt_scale), (s.dn[242][18] * ddt_scale), (s.dn[242][19] * ddt_scale), (s.dn[242][20] * ddt_scale), (s.dn[242][21] * ddt_scale), (s.dn[242][22] * ddt_scale), (s.dn[242][23] * ddt_scale), (s.dn[242][24] * ddt_scale), (s.dn[242][25] * ddt_scale), (s.dn[242][26] * ddt_scale), (s.dn[242][27] * ddt_scale), (s.dn[242][28] * ddt_scale), (s.dn[242][29] * ddt_scale), (s.db[242][0] * ddt_scale), (s.db[242][1] * ddt_scale), (s.db[242][2] * ddt_scale), (s.db[242][3] * ddt_scale), (s.db[242][4] * ddt_scale), (s.db[242][5] * ddt_scale), (s.db[242][6] * ddt_scale), (s.db[242][7] * ddt_scale), (s.db[242][8] * ddt_scale), (s.db[242][9] * ddt_scale), (s.db[242][10] * ddt_scale), (s.db[242][11] * ddt_scale), (s.db[242][12] * ddt_scale), (s.db[242][13] * ddt_scale), (s.db[242][14] * ddt_scale), (s.db[242][15] * ddt_scale), (s.db[242][16] * ddt_scale), (s.db[242][17] * ddt_scale), (s.db[242][18] * ddt_scale), (s.db[242][19] * ddt_scale), (s.db[242][20] * ddt_scale), (s.db[242][21] * ddt_scale), (s.db[242][22] * ddt_scale), (s.db[242][23] * ddt_scale), (s.db[242][24] * ddt_scale), (s.db[242][25] * ddt_scale), (s.db[242][26] * ddt_scale), (s.db[242][27] * ddt_scale), (s.db[242][28] * ddt_scale), (s.db[242][29] * ddt_scale), (s.db[242][30] * ddt_scale), (s.db[242][31] * ddt_scale), (s.db[242][32] * ddt_scale), (s.db[242][33] * ddt_scale), (s.db[242][34] * ddt_scale), (s.db[242][35] * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        let eq157_node_derivatives: [f64; 30] = [eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29];
        let eq157_branch_derivatives: [f64; 36] = [eq157_e1796_d_b0, eq157_e1796_d_b1, eq157_e1796_d_b2, eq157_e1796_d_b3, eq157_e1796_d_b4, eq157_e1796_d_b5, eq157_e1796_d_b6, eq157_e1796_d_b7, eq157_e1796_d_b8, eq157_e1796_d_b9, eq157_e1796_d_b10, eq157_e1796_d_b11, eq157_e1796_d_b12, eq157_e1796_d_b13, eq157_e1796_d_b14, eq157_e1796_d_b15, eq157_e1796_d_b16, eq157_e1796_d_b17, eq157_e1796_d_b18, eq157_e1796_d_b19, eq157_e1796_d_b20, eq157_e1796_d_b21, eq157_e1796_d_b22, eq157_e1796_d_b23, eq157_e1796_d_b24, eq157_e1796_d_b25, eq157_e1796_d_b26, eq157_e1796_d_b27, eq157_e1796_d_b28, eq157_e1796_d_b29, eq157_e1796_d_b30, eq157_e1796_d_b31, eq157_e1796_d_b32, eq157_e1796_d_b33, eq157_e1796_d_b34, eq157_e1796_d_b35];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq157_value),
            &eq157_node_derivatives,
            &eq157_branch_derivatives,
            multiplicity,
        );
        let (eq160_e1815, eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29, eq160_e1815_d_b0, eq160_e1815_d_b1, eq160_e1815_d_b2, eq160_e1815_d_b3, eq160_e1815_d_b4, eq160_e1815_d_b5, eq160_e1815_d_b6, eq160_e1815_d_b7, eq160_e1815_d_b8, eq160_e1815_d_b9, eq160_e1815_d_b10, eq160_e1815_d_b11, eq160_e1815_d_b12, eq160_e1815_d_b13, eq160_e1815_d_b14, eq160_e1815_d_b15, eq160_e1815_d_b16, eq160_e1815_d_b17, eq160_e1815_d_b18, eq160_e1815_d_b19, eq160_e1815_d_b20, eq160_e1815_d_b21, eq160_e1815_d_b22, eq160_e1815_d_b23, eq160_e1815_d_b24, eq160_e1815_d_b25, eq160_e1815_d_b26, eq160_e1815_d_b27, eq160_e1815_d_b28, eq160_e1815_d_b29, eq160_e1815_d_b30, eq160_e1815_d_b31, eq160_e1815_d_b32, eq160_e1815_d_b33, eq160_e1815_d_b34, eq160_e1815_d_b35,) = {
    if (s.b[2547] && s.b[2669]) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29], s.db[148][0], s.db[148][1], s.db[148][2], s.db[148][3], s.db[148][4], s.db[148][5], s.db[148][6], s.db[148][7], s.db[148][8], s.db[148][9], s.db[148][10], s.db[148][11], s.db[148][12], s.db[148][13], s.db[148][14], s.db[148][15], s.db[148][16], s.db[148][17], s.db[148][18], s.db[148][19], s.db[148][20], s.db[148][21], s.db[148][22], s.db[148][23], s.db[148][24], s.db[148][25], s.db[148][26], s.db[148][27], s.db[148][28], s.db[148][29], s.db[148][30], s.db[148][31], s.db[148][32], s.db[148][33], s.db[148][34], s.db[148][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e1815;
        let eq160_node_derivatives: [f64; 30] = [eq160_e1815_d_n0, eq160_e1815_d_n1, eq160_e1815_d_n2, eq160_e1815_d_n3, eq160_e1815_d_n4, eq160_e1815_d_n5, eq160_e1815_d_n6, eq160_e1815_d_n7, eq160_e1815_d_n8, eq160_e1815_d_n9, eq160_e1815_d_n10, eq160_e1815_d_n11, eq160_e1815_d_n12, eq160_e1815_d_n13, eq160_e1815_d_n14, eq160_e1815_d_n15, eq160_e1815_d_n16, eq160_e1815_d_n17, eq160_e1815_d_n18, eq160_e1815_d_n19, eq160_e1815_d_n20, eq160_e1815_d_n21, eq160_e1815_d_n22, eq160_e1815_d_n23, eq160_e1815_d_n24, eq160_e1815_d_n25, eq160_e1815_d_n26, eq160_e1815_d_n27, eq160_e1815_d_n28, eq160_e1815_d_n29];
        let eq160_branch_derivatives: [f64; 36] = [eq160_e1815_d_b0, eq160_e1815_d_b1, eq160_e1815_d_b2, eq160_e1815_d_b3, eq160_e1815_d_b4, eq160_e1815_d_b5, eq160_e1815_d_b6, eq160_e1815_d_b7, eq160_e1815_d_b8, eq160_e1815_d_b9, eq160_e1815_d_b10, eq160_e1815_d_b11, eq160_e1815_d_b12, eq160_e1815_d_b13, eq160_e1815_d_b14, eq160_e1815_d_b15, eq160_e1815_d_b16, eq160_e1815_d_b17, eq160_e1815_d_b18, eq160_e1815_d_b19, eq160_e1815_d_b20, eq160_e1815_d_b21, eq160_e1815_d_b22, eq160_e1815_d_b23, eq160_e1815_d_b24, eq160_e1815_d_b25, eq160_e1815_d_b26, eq160_e1815_d_b27, eq160_e1815_d_b28, eq160_e1815_d_b29, eq160_e1815_d_b30, eq160_e1815_d_b31, eq160_e1815_d_b32, eq160_e1815_d_b33, eq160_e1815_d_b34, eq160_e1815_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(0),
            multiplicity * (eq160_value),
            &eq160_node_derivatives,
            &eq160_branch_derivatives,
            multiplicity,
        );
        let (eq161_e1821, eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29, eq161_e1821_d_b0, eq161_e1821_d_b1, eq161_e1821_d_b2, eq161_e1821_d_b3, eq161_e1821_d_b4, eq161_e1821_d_b5, eq161_e1821_d_b6, eq161_e1821_d_b7, eq161_e1821_d_b8, eq161_e1821_d_b9, eq161_e1821_d_b10, eq161_e1821_d_b11, eq161_e1821_d_b12, eq161_e1821_d_b13, eq161_e1821_d_b14, eq161_e1821_d_b15, eq161_e1821_d_b16, eq161_e1821_d_b17, eq161_e1821_d_b18, eq161_e1821_d_b19, eq161_e1821_d_b20, eq161_e1821_d_b21, eq161_e1821_d_b22, eq161_e1821_d_b23, eq161_e1821_d_b24, eq161_e1821_d_b25, eq161_e1821_d_b26, eq161_e1821_d_b27, eq161_e1821_d_b28, eq161_e1821_d_b29, eq161_e1821_d_b30, eq161_e1821_d_b31, eq161_e1821_d_b32, eq161_e1821_d_b33, eq161_e1821_d_b34, eq161_e1821_d_b35,) = {
    if (s.b[2547] && s.b[2669]) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29], s.db[149][0], s.db[149][1], s.db[149][2], s.db[149][3], s.db[149][4], s.db[149][5], s.db[149][6], s.db[149][7], s.db[149][8], s.db[149][9], s.db[149][10], s.db[149][11], s.db[149][12], s.db[149][13], s.db[149][14], s.db[149][15], s.db[149][16], s.db[149][17], s.db[149][18], s.db[149][19], s.db[149][20], s.db[149][21], s.db[149][22], s.db[149][23], s.db[149][24], s.db[149][25], s.db[149][26], s.db[149][27], s.db[149][28], s.db[149][29], s.db[149][30], s.db[149][31], s.db[149][32], s.db[149][33], s.db[149][34], s.db[149][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e1821;
        let eq161_node_derivatives: [f64; 30] = [eq161_e1821_d_n0, eq161_e1821_d_n1, eq161_e1821_d_n2, eq161_e1821_d_n3, eq161_e1821_d_n4, eq161_e1821_d_n5, eq161_e1821_d_n6, eq161_e1821_d_n7, eq161_e1821_d_n8, eq161_e1821_d_n9, eq161_e1821_d_n10, eq161_e1821_d_n11, eq161_e1821_d_n12, eq161_e1821_d_n13, eq161_e1821_d_n14, eq161_e1821_d_n15, eq161_e1821_d_n16, eq161_e1821_d_n17, eq161_e1821_d_n18, eq161_e1821_d_n19, eq161_e1821_d_n20, eq161_e1821_d_n21, eq161_e1821_d_n22, eq161_e1821_d_n23, eq161_e1821_d_n24, eq161_e1821_d_n25, eq161_e1821_d_n26, eq161_e1821_d_n27, eq161_e1821_d_n28, eq161_e1821_d_n29];
        let eq161_branch_derivatives: [f64; 36] = [eq161_e1821_d_b0, eq161_e1821_d_b1, eq161_e1821_d_b2, eq161_e1821_d_b3, eq161_e1821_d_b4, eq161_e1821_d_b5, eq161_e1821_d_b6, eq161_e1821_d_b7, eq161_e1821_d_b8, eq161_e1821_d_b9, eq161_e1821_d_b10, eq161_e1821_d_b11, eq161_e1821_d_b12, eq161_e1821_d_b13, eq161_e1821_d_b14, eq161_e1821_d_b15, eq161_e1821_d_b16, eq161_e1821_d_b17, eq161_e1821_d_b18, eq161_e1821_d_b19, eq161_e1821_d_b20, eq161_e1821_d_b21, eq161_e1821_d_b22, eq161_e1821_d_b23, eq161_e1821_d_b24, eq161_e1821_d_b25, eq161_e1821_d_b26, eq161_e1821_d_b27, eq161_e1821_d_b28, eq161_e1821_d_b29, eq161_e1821_d_b30, eq161_e1821_d_b31, eq161_e1821_d_b32, eq161_e1821_d_b33, eq161_e1821_d_b34, eq161_e1821_d_b35];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq161_value),
            &eq161_node_derivatives,
            &eq161_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq162_e1828, eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29, eq162_e1828_d_b0, eq162_e1828_d_b1, eq162_e1828_d_b2, eq162_e1828_d_b3, eq162_e1828_d_b4, eq162_e1828_d_b5, eq162_e1828_d_b6, eq162_e1828_d_b7, eq162_e1828_d_b8, eq162_e1828_d_b9, eq162_e1828_d_b10, eq162_e1828_d_b11, eq162_e1828_d_b12, eq162_e1828_d_b13, eq162_e1828_d_b14, eq162_e1828_d_b15, eq162_e1828_d_b16, eq162_e1828_d_b17, eq162_e1828_d_b18, eq162_e1828_d_b19, eq162_e1828_d_b20, eq162_e1828_d_b21, eq162_e1828_d_b22, eq162_e1828_d_b23, eq162_e1828_d_b24, eq162_e1828_d_b25, eq162_e1828_d_b26, eq162_e1828_d_b27, eq162_e1828_d_b28, eq162_e1828_d_b29, eq162_e1828_d_b30, eq162_e1828_d_b31, eq162_e1828_d_b32, eq162_e1828_d_b33, eq162_e1828_d_b34, eq162_e1828_d_b35,) = {
    if (s.b[2547] && (!s.b[2669])) {
        (s.v[148], s.dn[148][0], s.dn[148][1], s.dn[148][2], s.dn[148][3], s.dn[148][4], s.dn[148][5], s.dn[148][6], s.dn[148][7], s.dn[148][8], s.dn[148][9], s.dn[148][10], s.dn[148][11], s.dn[148][12], s.dn[148][13], s.dn[148][14], s.dn[148][15], s.dn[148][16], s.dn[148][17], s.dn[148][18], s.dn[148][19], s.dn[148][20], s.dn[148][21], s.dn[148][22], s.dn[148][23], s.dn[148][24], s.dn[148][25], s.dn[148][26], s.dn[148][27], s.dn[148][28], s.dn[148][29], s.db[148][0], s.db[148][1], s.db[148][2], s.db[148][3], s.db[148][4], s.db[148][5], s.db[148][6], s.db[148][7], s.db[148][8], s.db[148][9], s.db[148][10], s.db[148][11], s.db[148][12], s.db[148][13], s.db[148][14], s.db[148][15], s.db[148][16], s.db[148][17], s.db[148][18], s.db[148][19], s.db[148][20], s.db[148][21], s.db[148][22], s.db[148][23], s.db[148][24], s.db[148][25], s.db[148][26], s.db[148][27], s.db[148][28], s.db[148][29], s.db[148][30], s.db[148][31], s.db[148][32], s.db[148][33], s.db[148][34], s.db[148][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e1828;
        let eq162_node_derivatives: [f64; 30] = [eq162_e1828_d_n0, eq162_e1828_d_n1, eq162_e1828_d_n2, eq162_e1828_d_n3, eq162_e1828_d_n4, eq162_e1828_d_n5, eq162_e1828_d_n6, eq162_e1828_d_n7, eq162_e1828_d_n8, eq162_e1828_d_n9, eq162_e1828_d_n10, eq162_e1828_d_n11, eq162_e1828_d_n12, eq162_e1828_d_n13, eq162_e1828_d_n14, eq162_e1828_d_n15, eq162_e1828_d_n16, eq162_e1828_d_n17, eq162_e1828_d_n18, eq162_e1828_d_n19, eq162_e1828_d_n20, eq162_e1828_d_n21, eq162_e1828_d_n22, eq162_e1828_d_n23, eq162_e1828_d_n24, eq162_e1828_d_n25, eq162_e1828_d_n26, eq162_e1828_d_n27, eq162_e1828_d_n28, eq162_e1828_d_n29];
        let eq162_branch_derivatives: [f64; 36] = [eq162_e1828_d_b0, eq162_e1828_d_b1, eq162_e1828_d_b2, eq162_e1828_d_b3, eq162_e1828_d_b4, eq162_e1828_d_b5, eq162_e1828_d_b6, eq162_e1828_d_b7, eq162_e1828_d_b8, eq162_e1828_d_b9, eq162_e1828_d_b10, eq162_e1828_d_b11, eq162_e1828_d_b12, eq162_e1828_d_b13, eq162_e1828_d_b14, eq162_e1828_d_b15, eq162_e1828_d_b16, eq162_e1828_d_b17, eq162_e1828_d_b18, eq162_e1828_d_b19, eq162_e1828_d_b20, eq162_e1828_d_b21, eq162_e1828_d_b22, eq162_e1828_d_b23, eq162_e1828_d_b24, eq162_e1828_d_b25, eq162_e1828_d_b26, eq162_e1828_d_b27, eq162_e1828_d_b28, eq162_e1828_d_b29, eq162_e1828_d_b30, eq162_e1828_d_b31, eq162_e1828_d_b32, eq162_e1828_d_b33, eq162_e1828_d_b34, eq162_e1828_d_b35];
        stamper.stamp_current_dense_local(
            Some(19),
            Some(18),
            multiplicity * (eq162_value),
            &eq162_node_derivatives,
            &eq162_branch_derivatives,
            multiplicity,
        );
        let (eq163_e1835, eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29, eq163_e1835_d_b0, eq163_e1835_d_b1, eq163_e1835_d_b2, eq163_e1835_d_b3, eq163_e1835_d_b4, eq163_e1835_d_b5, eq163_e1835_d_b6, eq163_e1835_d_b7, eq163_e1835_d_b8, eq163_e1835_d_b9, eq163_e1835_d_b10, eq163_e1835_d_b11, eq163_e1835_d_b12, eq163_e1835_d_b13, eq163_e1835_d_b14, eq163_e1835_d_b15, eq163_e1835_d_b16, eq163_e1835_d_b17, eq163_e1835_d_b18, eq163_e1835_d_b19, eq163_e1835_d_b20, eq163_e1835_d_b21, eq163_e1835_d_b22, eq163_e1835_d_b23, eq163_e1835_d_b24, eq163_e1835_d_b25, eq163_e1835_d_b26, eq163_e1835_d_b27, eq163_e1835_d_b28, eq163_e1835_d_b29, eq163_e1835_d_b30, eq163_e1835_d_b31, eq163_e1835_d_b32, eq163_e1835_d_b33, eq163_e1835_d_b34, eq163_e1835_d_b35,) = {
    if (s.b[2547] && (!s.b[2669])) {
        (s.v[149], s.dn[149][0], s.dn[149][1], s.dn[149][2], s.dn[149][3], s.dn[149][4], s.dn[149][5], s.dn[149][6], s.dn[149][7], s.dn[149][8], s.dn[149][9], s.dn[149][10], s.dn[149][11], s.dn[149][12], s.dn[149][13], s.dn[149][14], s.dn[149][15], s.dn[149][16], s.dn[149][17], s.dn[149][18], s.dn[149][19], s.dn[149][20], s.dn[149][21], s.dn[149][22], s.dn[149][23], s.dn[149][24], s.dn[149][25], s.dn[149][26], s.dn[149][27], s.dn[149][28], s.dn[149][29], s.db[149][0], s.db[149][1], s.db[149][2], s.db[149][3], s.db[149][4], s.db[149][5], s.db[149][6], s.db[149][7], s.db[149][8], s.db[149][9], s.db[149][10], s.db[149][11], s.db[149][12], s.db[149][13], s.db[149][14], s.db[149][15], s.db[149][16], s.db[149][17], s.db[149][18], s.db[149][19], s.db[149][20], s.db[149][21], s.db[149][22], s.db[149][23], s.db[149][24], s.db[149][25], s.db[149][26], s.db[149][27], s.db[149][28], s.db[149][29], s.db[149][30], s.db[149][31], s.db[149][32], s.db[149][33], s.db[149][34], s.db[149][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e1835;
        let eq163_node_derivatives: [f64; 30] = [eq163_e1835_d_n0, eq163_e1835_d_n1, eq163_e1835_d_n2, eq163_e1835_d_n3, eq163_e1835_d_n4, eq163_e1835_d_n5, eq163_e1835_d_n6, eq163_e1835_d_n7, eq163_e1835_d_n8, eq163_e1835_d_n9, eq163_e1835_d_n10, eq163_e1835_d_n11, eq163_e1835_d_n12, eq163_e1835_d_n13, eq163_e1835_d_n14, eq163_e1835_d_n15, eq163_e1835_d_n16, eq163_e1835_d_n17, eq163_e1835_d_n18, eq163_e1835_d_n19, eq163_e1835_d_n20, eq163_e1835_d_n21, eq163_e1835_d_n22, eq163_e1835_d_n23, eq163_e1835_d_n24, eq163_e1835_d_n25, eq163_e1835_d_n26, eq163_e1835_d_n27, eq163_e1835_d_n28, eq163_e1835_d_n29];
        let eq163_branch_derivatives: [f64; 36] = [eq163_e1835_d_b0, eq163_e1835_d_b1, eq163_e1835_d_b2, eq163_e1835_d_b3, eq163_e1835_d_b4, eq163_e1835_d_b5, eq163_e1835_d_b6, eq163_e1835_d_b7, eq163_e1835_d_b8, eq163_e1835_d_b9, eq163_e1835_d_b10, eq163_e1835_d_b11, eq163_e1835_d_b12, eq163_e1835_d_b13, eq163_e1835_d_b14, eq163_e1835_d_b15, eq163_e1835_d_b16, eq163_e1835_d_b17, eq163_e1835_d_b18, eq163_e1835_d_b19, eq163_e1835_d_b20, eq163_e1835_d_b21, eq163_e1835_d_b22, eq163_e1835_d_b23, eq163_e1835_d_b24, eq163_e1835_d_b25, eq163_e1835_d_b26, eq163_e1835_d_b27, eq163_e1835_d_b28, eq163_e1835_d_b29, eq163_e1835_d_b30, eq163_e1835_d_b31, eq163_e1835_d_b32, eq163_e1835_d_b33, eq163_e1835_d_b34, eq163_e1835_d_b35];
        stamper.stamp_current_dense_local(
            Some(18),
            Some(19),
            multiplicity * (eq163_value),
            &eq163_node_derivatives,
            &eq163_branch_derivatives,
            multiplicity,
        );
        let (eq164_e1841, eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29, eq164_e1841_d_b0, eq164_e1841_d_b1, eq164_e1841_d_b2, eq164_e1841_d_b3, eq164_e1841_d_b4, eq164_e1841_d_b5, eq164_e1841_d_b6, eq164_e1841_d_b7, eq164_e1841_d_b8, eq164_e1841_d_b9, eq164_e1841_d_b10, eq164_e1841_d_b11, eq164_e1841_d_b12, eq164_e1841_d_b13, eq164_e1841_d_b14, eq164_e1841_d_b15, eq164_e1841_d_b16, eq164_e1841_d_b17, eq164_e1841_d_b18, eq164_e1841_d_b19, eq164_e1841_d_b20, eq164_e1841_d_b21, eq164_e1841_d_b22, eq164_e1841_d_b23, eq164_e1841_d_b24, eq164_e1841_d_b25, eq164_e1841_d_b26, eq164_e1841_d_b27, eq164_e1841_d_b28, eq164_e1841_d_b29, eq164_e1841_d_b30, eq164_e1841_d_b31, eq164_e1841_d_b32, eq164_e1841_d_b33, eq164_e1841_d_b34, eq164_e1841_d_b35,) = {
    if s.b[2670] {
        let eq164_e1839: f64 = ((nv0 - nv18) / s.v[1]);
        let eq164_e1839_d_n0: f64 = ((s.v[1] - ((nv0 - nv18) * s.dn[1][0])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n1: f64 = (-(((nv0 - nv18) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n2: f64 = (-(((nv0 - nv18) * s.dn[1][2]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n3: f64 = (-(((nv0 - nv18) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n4: f64 = (-(((nv0 - nv18) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n5: f64 = (-(((nv0 - nv18) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n6: f64 = (-(((nv0 - nv18) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n7: f64 = (-(((nv0 - nv18) * s.dn[1][7]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n8: f64 = (-(((nv0 - nv18) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n9: f64 = (-(((nv0 - nv18) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n10: f64 = (-(((nv0 - nv18) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n11: f64 = (-(((nv0 - nv18) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n12: f64 = (-(((nv0 - nv18) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n13: f64 = (-(((nv0 - nv18) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n14: f64 = (-(((nv0 - nv18) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n15: f64 = (-(((nv0 - nv18) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n16: f64 = (-(((nv0 - nv18) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n17: f64 = (-(((nv0 - nv18) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n18: f64 = (((-s.v[1]) - ((nv0 - nv18) * s.dn[1][18])) / (s.v[1] * s.v[1]));
        let eq164_e1839_d_n19: f64 = (-(((nv0 - nv18) * s.dn[1][19]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n20: f64 = (-(((nv0 - nv18) * s.dn[1][20]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n21: f64 = (-(((nv0 - nv18) * s.dn[1][21]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n22: f64 = (-(((nv0 - nv18) * s.dn[1][22]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n23: f64 = (-(((nv0 - nv18) * s.dn[1][23]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n24: f64 = (-(((nv0 - nv18) * s.dn[1][24]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n25: f64 = (-(((nv0 - nv18) * s.dn[1][25]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n26: f64 = (-(((nv0 - nv18) * s.dn[1][26]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n27: f64 = (-(((nv0 - nv18) * s.dn[1][27]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n28: f64 = (-(((nv0 - nv18) * s.dn[1][28]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_n29: f64 = (-(((nv0 - nv18) * s.dn[1][29]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b0: f64 = (-(((nv0 - nv18) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b1: f64 = (-(((nv0 - nv18) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b2: f64 = (-(((nv0 - nv18) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b3: f64 = (-(((nv0 - nv18) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b4: f64 = (-(((nv0 - nv18) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b5: f64 = (-(((nv0 - nv18) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b6: f64 = (-(((nv0 - nv18) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b7: f64 = (-(((nv0 - nv18) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b8: f64 = (-(((nv0 - nv18) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b9: f64 = (-(((nv0 - nv18) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b10: f64 = (-(((nv0 - nv18) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b11: f64 = (-(((nv0 - nv18) * s.db[1][11]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b12: f64 = (-(((nv0 - nv18) * s.db[1][12]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b13: f64 = (-(((nv0 - nv18) * s.db[1][13]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b14: f64 = (-(((nv0 - nv18) * s.db[1][14]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b15: f64 = (-(((nv0 - nv18) * s.db[1][15]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b16: f64 = (-(((nv0 - nv18) * s.db[1][16]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b17: f64 = (-(((nv0 - nv18) * s.db[1][17]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b18: f64 = (-(((nv0 - nv18) * s.db[1][18]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b19: f64 = (-(((nv0 - nv18) * s.db[1][19]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b20: f64 = (-(((nv0 - nv18) * s.db[1][20]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b21: f64 = (-(((nv0 - nv18) * s.db[1][21]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b22: f64 = (-(((nv0 - nv18) * s.db[1][22]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b23: f64 = (-(((nv0 - nv18) * s.db[1][23]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b24: f64 = (-(((nv0 - nv18) * s.db[1][24]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b25: f64 = (-(((nv0 - nv18) * s.db[1][25]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b26: f64 = (-(((nv0 - nv18) * s.db[1][26]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b27: f64 = (-(((nv0 - nv18) * s.db[1][27]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b28: f64 = (-(((nv0 - nv18) * s.db[1][28]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b29: f64 = (-(((nv0 - nv18) * s.db[1][29]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b30: f64 = (-(((nv0 - nv18) * s.db[1][30]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b31: f64 = (-(((nv0 - nv18) * s.db[1][31]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b32: f64 = (-(((nv0 - nv18) * s.db[1][32]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b33: f64 = (-(((nv0 - nv18) * s.db[1][33]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b34: f64 = (-(((nv0 - nv18) * s.db[1][34]) / (s.v[1] * s.v[1])));
        let eq164_e1839_d_b35: f64 = (-(((nv0 - nv18) * s.db[1][35]) / (s.v[1] * s.v[1])));
        (eq164_e1839, eq164_e1839_d_n0, eq164_e1839_d_n1, eq164_e1839_d_n2, eq164_e1839_d_n3, eq164_e1839_d_n4, eq164_e1839_d_n5, eq164_e1839_d_n6, eq164_e1839_d_n7, eq164_e1839_d_n8, eq164_e1839_d_n9, eq164_e1839_d_n10, eq164_e1839_d_n11, eq164_e1839_d_n12, eq164_e1839_d_n13, eq164_e1839_d_n14, eq164_e1839_d_n15, eq164_e1839_d_n16, eq164_e1839_d_n17, eq164_e1839_d_n18, eq164_e1839_d_n19, eq164_e1839_d_n20, eq164_e1839_d_n21, eq164_e1839_d_n22, eq164_e1839_d_n23, eq164_e1839_d_n24, eq164_e1839_d_n25, eq164_e1839_d_n26, eq164_e1839_d_n27, eq164_e1839_d_n28, eq164_e1839_d_n29, eq164_e1839_d_b0, eq164_e1839_d_b1, eq164_e1839_d_b2, eq164_e1839_d_b3, eq164_e1839_d_b4, eq164_e1839_d_b5, eq164_e1839_d_b6, eq164_e1839_d_b7, eq164_e1839_d_b8, eq164_e1839_d_b9, eq164_e1839_d_b10, eq164_e1839_d_b11, eq164_e1839_d_b12, eq164_e1839_d_b13, eq164_e1839_d_b14, eq164_e1839_d_b15, eq164_e1839_d_b16, eq164_e1839_d_b17, eq164_e1839_d_b18, eq164_e1839_d_b19, eq164_e1839_d_b20, eq164_e1839_d_b21, eq164_e1839_d_b22, eq164_e1839_d_b23, eq164_e1839_d_b24, eq164_e1839_d_b25, eq164_e1839_d_b26, eq164_e1839_d_b27, eq164_e1839_d_b28, eq164_e1839_d_b29, eq164_e1839_d_b30, eq164_e1839_d_b31, eq164_e1839_d_b32, eq164_e1839_d_b33, eq164_e1839_d_b34, eq164_e1839_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e1841;
        let eq164_node_derivatives: [f64; 30] = [eq164_e1841_d_n0, eq164_e1841_d_n1, eq164_e1841_d_n2, eq164_e1841_d_n3, eq164_e1841_d_n4, eq164_e1841_d_n5, eq164_e1841_d_n6, eq164_e1841_d_n7, eq164_e1841_d_n8, eq164_e1841_d_n9, eq164_e1841_d_n10, eq164_e1841_d_n11, eq164_e1841_d_n12, eq164_e1841_d_n13, eq164_e1841_d_n14, eq164_e1841_d_n15, eq164_e1841_d_n16, eq164_e1841_d_n17, eq164_e1841_d_n18, eq164_e1841_d_n19, eq164_e1841_d_n20, eq164_e1841_d_n21, eq164_e1841_d_n22, eq164_e1841_d_n23, eq164_e1841_d_n24, eq164_e1841_d_n25, eq164_e1841_d_n26, eq164_e1841_d_n27, eq164_e1841_d_n28, eq164_e1841_d_n29];
        let eq164_branch_derivatives: [f64; 36] = [eq164_e1841_d_b0, eq164_e1841_d_b1, eq164_e1841_d_b2, eq164_e1841_d_b3, eq164_e1841_d_b4, eq164_e1841_d_b5, eq164_e1841_d_b6, eq164_e1841_d_b7, eq164_e1841_d_b8, eq164_e1841_d_b9, eq164_e1841_d_b10, eq164_e1841_d_b11, eq164_e1841_d_b12, eq164_e1841_d_b13, eq164_e1841_d_b14, eq164_e1841_d_b15, eq164_e1841_d_b16, eq164_e1841_d_b17, eq164_e1841_d_b18, eq164_e1841_d_b19, eq164_e1841_d_b20, eq164_e1841_d_b21, eq164_e1841_d_b22, eq164_e1841_d_b23, eq164_e1841_d_b24, eq164_e1841_d_b25, eq164_e1841_d_b26, eq164_e1841_d_b27, eq164_e1841_d_b28, eq164_e1841_d_b29, eq164_e1841_d_b30, eq164_e1841_d_b31, eq164_e1841_d_b32, eq164_e1841_d_b33, eq164_e1841_d_b34, eq164_e1841_d_b35];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(18),
            multiplicity * (eq164_value),
            &eq164_node_derivatives,
            &eq164_branch_derivatives,
            multiplicity,
        );
        let (eq166_e1852, eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29, eq166_e1852_d_b0, eq166_e1852_d_b1, eq166_e1852_d_b2, eq166_e1852_d_b3, eq166_e1852_d_b4, eq166_e1852_d_b5, eq166_e1852_d_b6, eq166_e1852_d_b7, eq166_e1852_d_b8, eq166_e1852_d_b9, eq166_e1852_d_b10, eq166_e1852_d_b11, eq166_e1852_d_b12, eq166_e1852_d_b13, eq166_e1852_d_b14, eq166_e1852_d_b15, eq166_e1852_d_b16, eq166_e1852_d_b17, eq166_e1852_d_b18, eq166_e1852_d_b19, eq166_e1852_d_b20, eq166_e1852_d_b21, eq166_e1852_d_b22, eq166_e1852_d_b23, eq166_e1852_d_b24, eq166_e1852_d_b25, eq166_e1852_d_b26, eq166_e1852_d_b27, eq166_e1852_d_b28, eq166_e1852_d_b29, eq166_e1852_d_b30, eq166_e1852_d_b31, eq166_e1852_d_b32, eq166_e1852_d_b33, eq166_e1852_d_b34, eq166_e1852_d_b35,) = {
    if s.b[2671] {
        let eq166_e1850: f64 = ((nv19 - nv2) / s.v[2]);
        let eq166_e1850_d_n0: f64 = (-(((nv19 - nv2) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n1: f64 = (-(((nv19 - nv2) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n2: f64 = (((-s.v[2]) - ((nv19 - nv2) * s.dn[2][2])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n3: f64 = (-(((nv19 - nv2) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n4: f64 = (-(((nv19 - nv2) * s.dn[2][4]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n5: f64 = (-(((nv19 - nv2) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n6: f64 = (-(((nv19 - nv2) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n7: f64 = (-(((nv19 - nv2) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n8: f64 = (-(((nv19 - nv2) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n9: f64 = (-(((nv19 - nv2) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n10: f64 = (-(((nv19 - nv2) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n11: f64 = (-(((nv19 - nv2) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n12: f64 = (-(((nv19 - nv2) * s.dn[2][12]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n13: f64 = (-(((nv19 - nv2) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n14: f64 = (-(((nv19 - nv2) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n15: f64 = (-(((nv19 - nv2) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n16: f64 = (-(((nv19 - nv2) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n17: f64 = (-(((nv19 - nv2) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n18: f64 = (-(((nv19 - nv2) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n19: f64 = ((s.v[2] - ((nv19 - nv2) * s.dn[2][19])) / (s.v[2] * s.v[2]));
        let eq166_e1850_d_n20: f64 = (-(((nv19 - nv2) * s.dn[2][20]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n21: f64 = (-(((nv19 - nv2) * s.dn[2][21]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n22: f64 = (-(((nv19 - nv2) * s.dn[2][22]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n23: f64 = (-(((nv19 - nv2) * s.dn[2][23]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n24: f64 = (-(((nv19 - nv2) * s.dn[2][24]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n25: f64 = (-(((nv19 - nv2) * s.dn[2][25]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n26: f64 = (-(((nv19 - nv2) * s.dn[2][26]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n27: f64 = (-(((nv19 - nv2) * s.dn[2][27]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n28: f64 = (-(((nv19 - nv2) * s.dn[2][28]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_n29: f64 = (-(((nv19 - nv2) * s.dn[2][29]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b0: f64 = (-(((nv19 - nv2) * s.db[2][0]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b1: f64 = (-(((nv19 - nv2) * s.db[2][1]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b2: f64 = (-(((nv19 - nv2) * s.db[2][2]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b3: f64 = (-(((nv19 - nv2) * s.db[2][3]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b4: f64 = (-(((nv19 - nv2) * s.db[2][4]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b5: f64 = (-(((nv19 - nv2) * s.db[2][5]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b6: f64 = (-(((nv19 - nv2) * s.db[2][6]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b7: f64 = (-(((nv19 - nv2) * s.db[2][7]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b8: f64 = (-(((nv19 - nv2) * s.db[2][8]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b9: f64 = (-(((nv19 - nv2) * s.db[2][9]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b10: f64 = (-(((nv19 - nv2) * s.db[2][10]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b11: f64 = (-(((nv19 - nv2) * s.db[2][11]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b12: f64 = (-(((nv19 - nv2) * s.db[2][12]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b13: f64 = (-(((nv19 - nv2) * s.db[2][13]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b14: f64 = (-(((nv19 - nv2) * s.db[2][14]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b15: f64 = (-(((nv19 - nv2) * s.db[2][15]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b16: f64 = (-(((nv19 - nv2) * s.db[2][16]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b17: f64 = (-(((nv19 - nv2) * s.db[2][17]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b18: f64 = (-(((nv19 - nv2) * s.db[2][18]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b19: f64 = (-(((nv19 - nv2) * s.db[2][19]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b20: f64 = (-(((nv19 - nv2) * s.db[2][20]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b21: f64 = (-(((nv19 - nv2) * s.db[2][21]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b22: f64 = (-(((nv19 - nv2) * s.db[2][22]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b23: f64 = (-(((nv19 - nv2) * s.db[2][23]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b24: f64 = (-(((nv19 - nv2) * s.db[2][24]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b25: f64 = (-(((nv19 - nv2) * s.db[2][25]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b26: f64 = (-(((nv19 - nv2) * s.db[2][26]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b27: f64 = (-(((nv19 - nv2) * s.db[2][27]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b28: f64 = (-(((nv19 - nv2) * s.db[2][28]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b29: f64 = (-(((nv19 - nv2) * s.db[2][29]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b30: f64 = (-(((nv19 - nv2) * s.db[2][30]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b31: f64 = (-(((nv19 - nv2) * s.db[2][31]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b32: f64 = (-(((nv19 - nv2) * s.db[2][32]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b33: f64 = (-(((nv19 - nv2) * s.db[2][33]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b34: f64 = (-(((nv19 - nv2) * s.db[2][34]) / (s.v[2] * s.v[2])));
        let eq166_e1850_d_b35: f64 = (-(((nv19 - nv2) * s.db[2][35]) / (s.v[2] * s.v[2])));
        (eq166_e1850, eq166_e1850_d_n0, eq166_e1850_d_n1, eq166_e1850_d_n2, eq166_e1850_d_n3, eq166_e1850_d_n4, eq166_e1850_d_n5, eq166_e1850_d_n6, eq166_e1850_d_n7, eq166_e1850_d_n8, eq166_e1850_d_n9, eq166_e1850_d_n10, eq166_e1850_d_n11, eq166_e1850_d_n12, eq166_e1850_d_n13, eq166_e1850_d_n14, eq166_e1850_d_n15, eq166_e1850_d_n16, eq166_e1850_d_n17, eq166_e1850_d_n18, eq166_e1850_d_n19, eq166_e1850_d_n20, eq166_e1850_d_n21, eq166_e1850_d_n22, eq166_e1850_d_n23, eq166_e1850_d_n24, eq166_e1850_d_n25, eq166_e1850_d_n26, eq166_e1850_d_n27, eq166_e1850_d_n28, eq166_e1850_d_n29, eq166_e1850_d_b0, eq166_e1850_d_b1, eq166_e1850_d_b2, eq166_e1850_d_b3, eq166_e1850_d_b4, eq166_e1850_d_b5, eq166_e1850_d_b6, eq166_e1850_d_b7, eq166_e1850_d_b8, eq166_e1850_d_b9, eq166_e1850_d_b10, eq166_e1850_d_b11, eq166_e1850_d_b12, eq166_e1850_d_b13, eq166_e1850_d_b14, eq166_e1850_d_b15, eq166_e1850_d_b16, eq166_e1850_d_b17, eq166_e1850_d_b18, eq166_e1850_d_b19, eq166_e1850_d_b20, eq166_e1850_d_b21, eq166_e1850_d_b22, eq166_e1850_d_b23, eq166_e1850_d_b24, eq166_e1850_d_b25, eq166_e1850_d_b26, eq166_e1850_d_b27, eq166_e1850_d_b28, eq166_e1850_d_b29, eq166_e1850_d_b30, eq166_e1850_d_b31, eq166_e1850_d_b32, eq166_e1850_d_b33, eq166_e1850_d_b34, eq166_e1850_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e1852;
        let eq166_node_derivatives: [f64; 30] = [eq166_e1852_d_n0, eq166_e1852_d_n1, eq166_e1852_d_n2, eq166_e1852_d_n3, eq166_e1852_d_n4, eq166_e1852_d_n5, eq166_e1852_d_n6, eq166_e1852_d_n7, eq166_e1852_d_n8, eq166_e1852_d_n9, eq166_e1852_d_n10, eq166_e1852_d_n11, eq166_e1852_d_n12, eq166_e1852_d_n13, eq166_e1852_d_n14, eq166_e1852_d_n15, eq166_e1852_d_n16, eq166_e1852_d_n17, eq166_e1852_d_n18, eq166_e1852_d_n19, eq166_e1852_d_n20, eq166_e1852_d_n21, eq166_e1852_d_n22, eq166_e1852_d_n23, eq166_e1852_d_n24, eq166_e1852_d_n25, eq166_e1852_d_n26, eq166_e1852_d_n27, eq166_e1852_d_n28, eq166_e1852_d_n29];
        let eq166_branch_derivatives: [f64; 36] = [eq166_e1852_d_b0, eq166_e1852_d_b1, eq166_e1852_d_b2, eq166_e1852_d_b3, eq166_e1852_d_b4, eq166_e1852_d_b5, eq166_e1852_d_b6, eq166_e1852_d_b7, eq166_e1852_d_b8, eq166_e1852_d_b9, eq166_e1852_d_b10, eq166_e1852_d_b11, eq166_e1852_d_b12, eq166_e1852_d_b13, eq166_e1852_d_b14, eq166_e1852_d_b15, eq166_e1852_d_b16, eq166_e1852_d_b17, eq166_e1852_d_b18, eq166_e1852_d_b19, eq166_e1852_d_b20, eq166_e1852_d_b21, eq166_e1852_d_b22, eq166_e1852_d_b23, eq166_e1852_d_b24, eq166_e1852_d_b25, eq166_e1852_d_b26, eq166_e1852_d_b27, eq166_e1852_d_b28, eq166_e1852_d_b29, eq166_e1852_d_b30, eq166_e1852_d_b31, eq166_e1852_d_b32, eq166_e1852_d_b33, eq166_e1852_d_b34, eq166_e1852_d_b35];
        stamper.stamp_current_dense_local(
            Some(19),
            Some(2),
            multiplicity * (eq166_value),
            &eq166_node_derivatives,
            &eq166_branch_derivatives,
            multiplicity,
        );
        let eq172_e1881: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 139, s.v[214]);
        let eq172_value: f64 = eq172_e1881;
        let eq172_node_derivatives: [f64; 30] = [(s.dn[214][0] * ddt_scale), (s.dn[214][1] * ddt_scale), (s.dn[214][2] * ddt_scale), (s.dn[214][3] * ddt_scale), (s.dn[214][4] * ddt_scale), (s.dn[214][5] * ddt_scale), (s.dn[214][6] * ddt_scale), (s.dn[214][7] * ddt_scale), (s.dn[214][8] * ddt_scale), (s.dn[214][9] * ddt_scale), (s.dn[214][10] * ddt_scale), (s.dn[214][11] * ddt_scale), (s.dn[214][12] * ddt_scale), (s.dn[214][13] * ddt_scale), (s.dn[214][14] * ddt_scale), (s.dn[214][15] * ddt_scale), (s.dn[214][16] * ddt_scale), (s.dn[214][17] * ddt_scale), (s.dn[214][18] * ddt_scale), (s.dn[214][19] * ddt_scale), (s.dn[214][20] * ddt_scale), (s.dn[214][21] * ddt_scale), (s.dn[214][22] * ddt_scale), (s.dn[214][23] * ddt_scale), (s.dn[214][24] * ddt_scale), (s.dn[214][25] * ddt_scale), (s.dn[214][26] * ddt_scale), (s.dn[214][27] * ddt_scale), (s.dn[214][28] * ddt_scale), (s.dn[214][29] * ddt_scale)];
        let eq172_branch_derivatives: [f64; 36] = [(s.db[214][0] * ddt_scale), (s.db[214][1] * ddt_scale), (s.db[214][2] * ddt_scale), (s.db[214][3] * ddt_scale), (s.db[214][4] * ddt_scale), (s.db[214][5] * ddt_scale), (s.db[214][6] * ddt_scale), (s.db[214][7] * ddt_scale), (s.db[214][8] * ddt_scale), (s.db[214][9] * ddt_scale), (s.db[214][10] * ddt_scale), (s.db[214][11] * ddt_scale), (s.db[214][12] * ddt_scale), (s.db[214][13] * ddt_scale), (s.db[214][14] * ddt_scale), (s.db[214][15] * ddt_scale), (s.db[214][16] * ddt_scale), (s.db[214][17] * ddt_scale), (s.db[214][18] * ddt_scale), (s.db[214][19] * ddt_scale), (s.db[214][20] * ddt_scale), (s.db[214][21] * ddt_scale), (s.db[214][22] * ddt_scale), (s.db[214][23] * ddt_scale), (s.db[214][24] * ddt_scale), (s.db[214][25] * ddt_scale), (s.db[214][26] * ddt_scale), (s.db[214][27] * ddt_scale), (s.db[214][28] * ddt_scale), (s.db[214][29] * ddt_scale), (s.db[214][30] * ddt_scale), (s.db[214][31] * ddt_scale), (s.db[214][32] * ddt_scale), (s.db[214][33] * ddt_scale), (s.db[214][34] * ddt_scale), (s.db[214][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq172_value),
            &eq172_node_derivatives,
            &eq172_branch_derivatives,
            multiplicity,
        );
        let eq173_e1883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 140, s.v[215]);
        let eq173_value: f64 = eq173_e1883;
        let eq173_node_derivatives: [f64; 30] = [(s.dn[215][0] * ddt_scale), (s.dn[215][1] * ddt_scale), (s.dn[215][2] * ddt_scale), (s.dn[215][3] * ddt_scale), (s.dn[215][4] * ddt_scale), (s.dn[215][5] * ddt_scale), (s.dn[215][6] * ddt_scale), (s.dn[215][7] * ddt_scale), (s.dn[215][8] * ddt_scale), (s.dn[215][9] * ddt_scale), (s.dn[215][10] * ddt_scale), (s.dn[215][11] * ddt_scale), (s.dn[215][12] * ddt_scale), (s.dn[215][13] * ddt_scale), (s.dn[215][14] * ddt_scale), (s.dn[215][15] * ddt_scale), (s.dn[215][16] * ddt_scale), (s.dn[215][17] * ddt_scale), (s.dn[215][18] * ddt_scale), (s.dn[215][19] * ddt_scale), (s.dn[215][20] * ddt_scale), (s.dn[215][21] * ddt_scale), (s.dn[215][22] * ddt_scale), (s.dn[215][23] * ddt_scale), (s.dn[215][24] * ddt_scale), (s.dn[215][25] * ddt_scale), (s.dn[215][26] * ddt_scale), (s.dn[215][27] * ddt_scale), (s.dn[215][28] * ddt_scale), (s.dn[215][29] * ddt_scale)];
        let eq173_branch_derivatives: [f64; 36] = [(s.db[215][0] * ddt_scale), (s.db[215][1] * ddt_scale), (s.db[215][2] * ddt_scale), (s.db[215][3] * ddt_scale), (s.db[215][4] * ddt_scale), (s.db[215][5] * ddt_scale), (s.db[215][6] * ddt_scale), (s.db[215][7] * ddt_scale), (s.db[215][8] * ddt_scale), (s.db[215][9] * ddt_scale), (s.db[215][10] * ddt_scale), (s.db[215][11] * ddt_scale), (s.db[215][12] * ddt_scale), (s.db[215][13] * ddt_scale), (s.db[215][14] * ddt_scale), (s.db[215][15] * ddt_scale), (s.db[215][16] * ddt_scale), (s.db[215][17] * ddt_scale), (s.db[215][18] * ddt_scale), (s.db[215][19] * ddt_scale), (s.db[215][20] * ddt_scale), (s.db[215][21] * ddt_scale), (s.db[215][22] * ddt_scale), (s.db[215][23] * ddt_scale), (s.db[215][24] * ddt_scale), (s.db[215][25] * ddt_scale), (s.db[215][26] * ddt_scale), (s.db[215][27] * ddt_scale), (s.db[215][28] * ddt_scale), (s.db[215][29] * ddt_scale), (s.db[215][30] * ddt_scale), (s.db[215][31] * ddt_scale), (s.db[215][32] * ddt_scale), (s.db[215][33] * ddt_scale), (s.db[215][34] * ddt_scale), (s.db[215][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(0),
            multiplicity * (eq173_value),
            &eq173_node_derivatives,
            &eq173_branch_derivatives,
            multiplicity,
        );
        let eq174_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 141, s.v[216]);
        let eq174_value: f64 = eq174_e1885;
        let eq174_node_derivatives: [f64; 30] = [(s.dn[216][0] * ddt_scale), (s.dn[216][1] * ddt_scale), (s.dn[216][2] * ddt_scale), (s.dn[216][3] * ddt_scale), (s.dn[216][4] * ddt_scale), (s.dn[216][5] * ddt_scale), (s.dn[216][6] * ddt_scale), (s.dn[216][7] * ddt_scale), (s.dn[216][8] * ddt_scale), (s.dn[216][9] * ddt_scale), (s.dn[216][10] * ddt_scale), (s.dn[216][11] * ddt_scale), (s.dn[216][12] * ddt_scale), (s.dn[216][13] * ddt_scale), (s.dn[216][14] * ddt_scale), (s.dn[216][15] * ddt_scale), (s.dn[216][16] * ddt_scale), (s.dn[216][17] * ddt_scale), (s.dn[216][18] * ddt_scale), (s.dn[216][19] * ddt_scale), (s.dn[216][20] * ddt_scale), (s.dn[216][21] * ddt_scale), (s.dn[216][22] * ddt_scale), (s.dn[216][23] * ddt_scale), (s.dn[216][24] * ddt_scale), (s.dn[216][25] * ddt_scale), (s.dn[216][26] * ddt_scale), (s.dn[216][27] * ddt_scale), (s.dn[216][28] * ddt_scale), (s.dn[216][29] * ddt_scale)];
        let eq174_branch_derivatives: [f64; 36] = [(s.db[216][0] * ddt_scale), (s.db[216][1] * ddt_scale), (s.db[216][2] * ddt_scale), (s.db[216][3] * ddt_scale), (s.db[216][4] * ddt_scale), (s.db[216][5] * ddt_scale), (s.db[216][6] * ddt_scale), (s.db[216][7] * ddt_scale), (s.db[216][8] * ddt_scale), (s.db[216][9] * ddt_scale), (s.db[216][10] * ddt_scale), (s.db[216][11] * ddt_scale), (s.db[216][12] * ddt_scale), (s.db[216][13] * ddt_scale), (s.db[216][14] * ddt_scale), (s.db[216][15] * ddt_scale), (s.db[216][16] * ddt_scale), (s.db[216][17] * ddt_scale), (s.db[216][18] * ddt_scale), (s.db[216][19] * ddt_scale), (s.db[216][20] * ddt_scale), (s.db[216][21] * ddt_scale), (s.db[216][22] * ddt_scale), (s.db[216][23] * ddt_scale), (s.db[216][24] * ddt_scale), (s.db[216][25] * ddt_scale), (s.db[216][26] * ddt_scale), (s.db[216][27] * ddt_scale), (s.db[216][28] * ddt_scale), (s.db[216][29] * ddt_scale), (s.db[216][30] * ddt_scale), (s.db[216][31] * ddt_scale), (s.db[216][32] * ddt_scale), (s.db[216][33] * ddt_scale), (s.db[216][34] * ddt_scale), (s.db[216][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(0),
            multiplicity * (eq174_value),
            &eq174_node_derivatives,
            &eq174_branch_derivatives,
            multiplicity,
        );
        let eq175_e1887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 142, s.v[218]);
        let eq175_value: f64 = eq175_e1887;
        let eq175_node_derivatives: [f64; 30] = [(s.dn[218][0] * ddt_scale), (s.dn[218][1] * ddt_scale), (s.dn[218][2] * ddt_scale), (s.dn[218][3] * ddt_scale), (s.dn[218][4] * ddt_scale), (s.dn[218][5] * ddt_scale), (s.dn[218][6] * ddt_scale), (s.dn[218][7] * ddt_scale), (s.dn[218][8] * ddt_scale), (s.dn[218][9] * ddt_scale), (s.dn[218][10] * ddt_scale), (s.dn[218][11] * ddt_scale), (s.dn[218][12] * ddt_scale), (s.dn[218][13] * ddt_scale), (s.dn[218][14] * ddt_scale), (s.dn[218][15] * ddt_scale), (s.dn[218][16] * ddt_scale), (s.dn[218][17] * ddt_scale), (s.dn[218][18] * ddt_scale), (s.dn[218][19] * ddt_scale), (s.dn[218][20] * ddt_scale), (s.dn[218][21] * ddt_scale), (s.dn[218][22] * ddt_scale), (s.dn[218][23] * ddt_scale), (s.dn[218][24] * ddt_scale), (s.dn[218][25] * ddt_scale), (s.dn[218][26] * ddt_scale), (s.dn[218][27] * ddt_scale), (s.dn[218][28] * ddt_scale), (s.dn[218][29] * ddt_scale)];
        let eq175_branch_derivatives: [f64; 36] = [(s.db[218][0] * ddt_scale), (s.db[218][1] * ddt_scale), (s.db[218][2] * ddt_scale), (s.db[218][3] * ddt_scale), (s.db[218][4] * ddt_scale), (s.db[218][5] * ddt_scale), (s.db[218][6] * ddt_scale), (s.db[218][7] * ddt_scale), (s.db[218][8] * ddt_scale), (s.db[218][9] * ddt_scale), (s.db[218][10] * ddt_scale), (s.db[218][11] * ddt_scale), (s.db[218][12] * ddt_scale), (s.db[218][13] * ddt_scale), (s.db[218][14] * ddt_scale), (s.db[218][15] * ddt_scale), (s.db[218][16] * ddt_scale), (s.db[218][17] * ddt_scale), (s.db[218][18] * ddt_scale), (s.db[218][19] * ddt_scale), (s.db[218][20] * ddt_scale), (s.db[218][21] * ddt_scale), (s.db[218][22] * ddt_scale), (s.db[218][23] * ddt_scale), (s.db[218][24] * ddt_scale), (s.db[218][25] * ddt_scale), (s.db[218][26] * ddt_scale), (s.db[218][27] * ddt_scale), (s.db[218][28] * ddt_scale), (s.db[218][29] * ddt_scale), (s.db[218][30] * ddt_scale), (s.db[218][31] * ddt_scale), (s.db[218][32] * ddt_scale), (s.db[218][33] * ddt_scale), (s.db[218][34] * ddt_scale), (s.db[218][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(2),
            multiplicity * (eq175_value),
            &eq175_node_derivatives,
            &eq175_branch_derivatives,
            multiplicity,
        );
        let eq176_e1889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 143, s.v[217]);
        let eq176_value: f64 = eq176_e1889;
        let eq176_node_derivatives: [f64; 30] = [(s.dn[217][0] * ddt_scale), (s.dn[217][1] * ddt_scale), (s.dn[217][2] * ddt_scale), (s.dn[217][3] * ddt_scale), (s.dn[217][4] * ddt_scale), (s.dn[217][5] * ddt_scale), (s.dn[217][6] * ddt_scale), (s.dn[217][7] * ddt_scale), (s.dn[217][8] * ddt_scale), (s.dn[217][9] * ddt_scale), (s.dn[217][10] * ddt_scale), (s.dn[217][11] * ddt_scale), (s.dn[217][12] * ddt_scale), (s.dn[217][13] * ddt_scale), (s.dn[217][14] * ddt_scale), (s.dn[217][15] * ddt_scale), (s.dn[217][16] * ddt_scale), (s.dn[217][17] * ddt_scale), (s.dn[217][18] * ddt_scale), (s.dn[217][19] * ddt_scale), (s.dn[217][20] * ddt_scale), (s.dn[217][21] * ddt_scale), (s.dn[217][22] * ddt_scale), (s.dn[217][23] * ddt_scale), (s.dn[217][24] * ddt_scale), (s.dn[217][25] * ddt_scale), (s.dn[217][26] * ddt_scale), (s.dn[217][27] * ddt_scale), (s.dn[217][28] * ddt_scale), (s.dn[217][29] * ddt_scale)];
        let eq176_branch_derivatives: [f64; 36] = [(s.db[217][0] * ddt_scale), (s.db[217][1] * ddt_scale), (s.db[217][2] * ddt_scale), (s.db[217][3] * ddt_scale), (s.db[217][4] * ddt_scale), (s.db[217][5] * ddt_scale), (s.db[217][6] * ddt_scale), (s.db[217][7] * ddt_scale), (s.db[217][8] * ddt_scale), (s.db[217][9] * ddt_scale), (s.db[217][10] * ddt_scale), (s.db[217][11] * ddt_scale), (s.db[217][12] * ddt_scale), (s.db[217][13] * ddt_scale), (s.db[217][14] * ddt_scale), (s.db[217][15] * ddt_scale), (s.db[217][16] * ddt_scale), (s.db[217][17] * ddt_scale), (s.db[217][18] * ddt_scale), (s.db[217][19] * ddt_scale), (s.db[217][20] * ddt_scale), (s.db[217][21] * ddt_scale), (s.db[217][22] * ddt_scale), (s.db[217][23] * ddt_scale), (s.db[217][24] * ddt_scale), (s.db[217][25] * ddt_scale), (s.db[217][26] * ddt_scale), (s.db[217][27] * ddt_scale), (s.db[217][28] * ddt_scale), (s.db[217][29] * ddt_scale), (s.db[217][30] * ddt_scale), (s.db[217][31] * ddt_scale), (s.db[217][32] * ddt_scale), (s.db[217][33] * ddt_scale), (s.db[217][34] * ddt_scale), (s.db[217][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq176_value),
            &eq176_node_derivatives,
            &eq176_branch_derivatives,
            multiplicity,
        );
        let eq177_e1891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 144, s.v[219]);
        let eq177_value: f64 = eq177_e1891;
        let eq177_node_derivatives: [f64; 30] = [(s.dn[219][0] * ddt_scale), (s.dn[219][1] * ddt_scale), (s.dn[219][2] * ddt_scale), (s.dn[219][3] * ddt_scale), (s.dn[219][4] * ddt_scale), (s.dn[219][5] * ddt_scale), (s.dn[219][6] * ddt_scale), (s.dn[219][7] * ddt_scale), (s.dn[219][8] * ddt_scale), (s.dn[219][9] * ddt_scale), (s.dn[219][10] * ddt_scale), (s.dn[219][11] * ddt_scale), (s.dn[219][12] * ddt_scale), (s.dn[219][13] * ddt_scale), (s.dn[219][14] * ddt_scale), (s.dn[219][15] * ddt_scale), (s.dn[219][16] * ddt_scale), (s.dn[219][17] * ddt_scale), (s.dn[219][18] * ddt_scale), (s.dn[219][19] * ddt_scale), (s.dn[219][20] * ddt_scale), (s.dn[219][21] * ddt_scale), (s.dn[219][22] * ddt_scale), (s.dn[219][23] * ddt_scale), (s.dn[219][24] * ddt_scale), (s.dn[219][25] * ddt_scale), (s.dn[219][26] * ddt_scale), (s.dn[219][27] * ddt_scale), (s.dn[219][28] * ddt_scale), (s.dn[219][29] * ddt_scale)];
        let eq177_branch_derivatives: [f64; 36] = [(s.db[219][0] * ddt_scale), (s.db[219][1] * ddt_scale), (s.db[219][2] * ddt_scale), (s.db[219][3] * ddt_scale), (s.db[219][4] * ddt_scale), (s.db[219][5] * ddt_scale), (s.db[219][6] * ddt_scale), (s.db[219][7] * ddt_scale), (s.db[219][8] * ddt_scale), (s.db[219][9] * ddt_scale), (s.db[219][10] * ddt_scale), (s.db[219][11] * ddt_scale), (s.db[219][12] * ddt_scale), (s.db[219][13] * ddt_scale), (s.db[219][14] * ddt_scale), (s.db[219][15] * ddt_scale), (s.db[219][16] * ddt_scale), (s.db[219][17] * ddt_scale), (s.db[219][18] * ddt_scale), (s.db[219][19] * ddt_scale), (s.db[219][20] * ddt_scale), (s.db[219][21] * ddt_scale), (s.db[219][22] * ddt_scale), (s.db[219][23] * ddt_scale), (s.db[219][24] * ddt_scale), (s.db[219][25] * ddt_scale), (s.db[219][26] * ddt_scale), (s.db[219][27] * ddt_scale), (s.db[219][28] * ddt_scale), (s.db[219][29] * ddt_scale), (s.db[219][30] * ddt_scale), (s.db[219][31] * ddt_scale), (s.db[219][32] * ddt_scale), (s.db[219][33] * ddt_scale), (s.db[219][34] * ddt_scale), (s.db[219][35] * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq177_value),
            &eq177_node_derivatives,
            &eq177_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2167, eq194_e2167_d_n4,) = {
    if s.b[2700] {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 145, eq194_e2164);
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
        let (eq195_e2172, eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29, eq195_e2172_d_b0, eq195_e2172_d_b1, eq195_e2172_d_b2, eq195_e2172_d_b3, eq195_e2172_d_b4, eq195_e2172_d_b5, eq195_e2172_d_b6, eq195_e2172_d_b7, eq195_e2172_d_b8, eq195_e2172_d_b9, eq195_e2172_d_b10, eq195_e2172_d_b11, eq195_e2172_d_b12, eq195_e2172_d_b13, eq195_e2172_d_b14, eq195_e2172_d_b15, eq195_e2172_d_b16, eq195_e2172_d_b17, eq195_e2172_d_b18, eq195_e2172_d_b19, eq195_e2172_d_b20, eq195_e2172_d_b21, eq195_e2172_d_b22, eq195_e2172_d_b23, eq195_e2172_d_b24, eq195_e2172_d_b25, eq195_e2172_d_b26, eq195_e2172_d_b27, eq195_e2172_d_b28, eq195_e2172_d_b29, eq195_e2172_d_b30, eq195_e2172_d_b31, eq195_e2172_d_b32, eq195_e2172_d_b33, eq195_e2172_d_b34, eq195_e2172_d_b35,) = {
    if s.b[2700] {
        let eq195_e2170: f64 = (-s.v[114]);
        (eq195_e2170, (-s.dn[114][0]), (-s.dn[114][1]), (-s.dn[114][2]), (-s.dn[114][3]), (-s.dn[114][4]), (-s.dn[114][5]), (-s.dn[114][6]), (-s.dn[114][7]), (-s.dn[114][8]), (-s.dn[114][9]), (-s.dn[114][10]), (-s.dn[114][11]), (-s.dn[114][12]), (-s.dn[114][13]), (-s.dn[114][14]), (-s.dn[114][15]), (-s.dn[114][16]), (-s.dn[114][17]), (-s.dn[114][18]), (-s.dn[114][19]), (-s.dn[114][20]), (-s.dn[114][21]), (-s.dn[114][22]), (-s.dn[114][23]), (-s.dn[114][24]), (-s.dn[114][25]), (-s.dn[114][26]), (-s.dn[114][27]), (-s.dn[114][28]), (-s.dn[114][29]), (-s.db[114][0]), (-s.db[114][1]), (-s.db[114][2]), (-s.db[114][3]), (-s.db[114][4]), (-s.db[114][5]), (-s.db[114][6]), (-s.db[114][7]), (-s.db[114][8]), (-s.db[114][9]), (-s.db[114][10]), (-s.db[114][11]), (-s.db[114][12]), (-s.db[114][13]), (-s.db[114][14]), (-s.db[114][15]), (-s.db[114][16]), (-s.db[114][17]), (-s.db[114][18]), (-s.db[114][19]), (-s.db[114][20]), (-s.db[114][21]), (-s.db[114][22]), (-s.db[114][23]), (-s.db[114][24]), (-s.db[114][25]), (-s.db[114][26]), (-s.db[114][27]), (-s.db[114][28]), (-s.db[114][29]), (-s.db[114][30]), (-s.db[114][31]), (-s.db[114][32]), (-s.db[114][33]), (-s.db[114][34]), (-s.db[114][35]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2172;
        let eq195_node_derivatives: [f64; 30] = [eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29];
        let eq195_branch_derivatives: [f64; 36] = [eq195_e2172_d_b0, eq195_e2172_d_b1, eq195_e2172_d_b2, eq195_e2172_d_b3, eq195_e2172_d_b4, eq195_e2172_d_b5, eq195_e2172_d_b6, eq195_e2172_d_b7, eq195_e2172_d_b8, eq195_e2172_d_b9, eq195_e2172_d_b10, eq195_e2172_d_b11, eq195_e2172_d_b12, eq195_e2172_d_b13, eq195_e2172_d_b14, eq195_e2172_d_b15, eq195_e2172_d_b16, eq195_e2172_d_b17, eq195_e2172_d_b18, eq195_e2172_d_b19, eq195_e2172_d_b20, eq195_e2172_d_b21, eq195_e2172_d_b22, eq195_e2172_d_b23, eq195_e2172_d_b24, eq195_e2172_d_b25, eq195_e2172_d_b26, eq195_e2172_d_b27, eq195_e2172_d_b28, eq195_e2172_d_b29, eq195_e2172_d_b30, eq195_e2172_d_b31, eq195_e2172_d_b32, eq195_e2172_d_b33, eq195_e2172_d_b34, eq195_e2172_d_b35];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq195_value),
            &eq195_node_derivatives,
            &eq195_branch_derivatives,
            multiplicity,
        );
        let (eq196_e2178, eq196_e2178_d_n4,) = {
    if s.b[2700] {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p320;
        let eq196_e2176: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq196_e2176_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        (eq196_e2176, eq196_e2176_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2178;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq196_value),
            4,
            multiplicity * (eq196_e2178_d_n4),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_q,) = {
    if s.b[308] {
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
    if s.b[308] {
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
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29, eq17_e564_d_b0, eq17_e564_d_b1, eq17_e564_d_b2, eq17_e564_d_b3, eq17_e564_d_b4, eq17_e564_d_b5, eq17_e564_d_b6, eq17_e564_d_b7, eq17_e564_d_b8, eq17_e564_d_b9, eq17_e564_d_b10, eq17_e564_d_b11, eq17_e564_d_b12, eq17_e564_d_b13, eq17_e564_d_b14, eq17_e564_d_b15, eq17_e564_d_b16, eq17_e564_d_b17, eq17_e564_d_b18, eq17_e564_d_b19, eq17_e564_d_b20, eq17_e564_d_b21, eq17_e564_d_b22, eq17_e564_d_b23, eq17_e564_d_b24, eq17_e564_d_b25, eq17_e564_d_b26, eq17_e564_d_b27, eq17_e564_d_b28, eq17_e564_d_b29, eq17_e564_d_b30, eq17_e564_d_b31, eq17_e564_d_b32, eq17_e564_d_b33, eq17_e564_d_b34, eq17_e564_d_b35, eq17_e564_q, eq17_e564_q_d_n0, eq17_e564_q_d_n1, eq17_e564_q_d_n2, eq17_e564_q_d_n3, eq17_e564_q_d_n4, eq17_e564_q_d_n5, eq17_e564_q_d_n6, eq17_e564_q_d_n7, eq17_e564_q_d_n8, eq17_e564_q_d_n9, eq17_e564_q_d_n10, eq17_e564_q_d_n11, eq17_e564_q_d_n12, eq17_e564_q_d_n13, eq17_e564_q_d_n14, eq17_e564_q_d_n15, eq17_e564_q_d_n16, eq17_e564_q_d_n17, eq17_e564_q_d_n18, eq17_e564_q_d_n19, eq17_e564_q_d_n20, eq17_e564_q_d_n21, eq17_e564_q_d_n22, eq17_e564_q_d_n23, eq17_e564_q_d_n24, eq17_e564_q_d_n25, eq17_e564_q_d_n26, eq17_e564_q_d_n27, eq17_e564_q_d_n28, eq17_e564_q_d_n29, eq17_e564_q_d_b0, eq17_e564_q_d_b1, eq17_e564_q_d_b2, eq17_e564_q_d_b3, eq17_e564_q_d_b4, eq17_e564_q_d_b5, eq17_e564_q_d_b6, eq17_e564_q_d_b7, eq17_e564_q_d_b8, eq17_e564_q_d_b9, eq17_e564_q_d_b10, eq17_e564_q_d_b11, eq17_e564_q_d_b12, eq17_e564_q_d_b13, eq17_e564_q_d_b14, eq17_e564_q_d_b15, eq17_e564_q_d_b16, eq17_e564_q_d_b17, eq17_e564_q_d_b18, eq17_e564_q_d_b19, eq17_e564_q_d_b20, eq17_e564_q_d_b21, eq17_e564_q_d_b22, eq17_e564_q_d_b23, eq17_e564_q_d_b24, eq17_e564_q_d_b25, eq17_e564_q_d_b26, eq17_e564_q_d_b27, eq17_e564_q_d_b28, eq17_e564_q_d_b29, eq17_e564_q_d_b30, eq17_e564_q_d_b31, eq17_e564_q_d_b32, eq17_e564_q_d_b33, eq17_e564_q_d_b34, eq17_e564_q_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq17_e543_q: f64 = s.v[225];
        let eq17_e544: f64 = (p.p341 * s.v[225]);
        let eq17_e544_d_n0: f64 = (p.p341 * s.dn[225][0]);
        let eq17_e544_d_n1: f64 = (p.p341 * s.dn[225][1]);
        let eq17_e544_d_n2: f64 = (p.p341 * s.dn[225][2]);
        let eq17_e544_d_n3: f64 = (p.p341 * s.dn[225][3]);
        let eq17_e544_d_n4: f64 = (p.p341 * s.dn[225][4]);
        let eq17_e544_d_n5: f64 = (p.p341 * s.dn[225][5]);
        let eq17_e544_d_n6: f64 = (p.p341 * s.dn[225][6]);
        let eq17_e544_d_n7: f64 = (p.p341 * s.dn[225][7]);
        let eq17_e544_d_n8: f64 = (p.p341 * s.dn[225][8]);
        let eq17_e544_d_n9: f64 = (p.p341 * s.dn[225][9]);
        let eq17_e544_d_n10: f64 = (p.p341 * s.dn[225][10]);
        let eq17_e544_d_n11: f64 = (p.p341 * s.dn[225][11]);
        let eq17_e544_d_n12: f64 = (p.p341 * s.dn[225][12]);
        let eq17_e544_d_n13: f64 = (p.p341 * s.dn[225][13]);
        let eq17_e544_d_n14: f64 = (p.p341 * s.dn[225][14]);
        let eq17_e544_d_n15: f64 = (p.p341 * s.dn[225][15]);
        let eq17_e544_d_n16: f64 = (p.p341 * s.dn[225][16]);
        let eq17_e544_d_n17: f64 = (p.p341 * s.dn[225][17]);
        let eq17_e544_d_n18: f64 = (p.p341 * s.dn[225][18]);
        let eq17_e544_d_n19: f64 = (p.p341 * s.dn[225][19]);
        let eq17_e544_d_n20: f64 = (p.p341 * s.dn[225][20]);
        let eq17_e544_d_n21: f64 = (p.p341 * s.dn[225][21]);
        let eq17_e544_d_n22: f64 = (p.p341 * s.dn[225][22]);
        let eq17_e544_d_n23: f64 = (p.p341 * s.dn[225][23]);
        let eq17_e544_d_n24: f64 = (p.p341 * s.dn[225][24]);
        let eq17_e544_d_n25: f64 = (p.p341 * s.dn[225][25]);
        let eq17_e544_d_n26: f64 = (p.p341 * s.dn[225][26]);
        let eq17_e544_d_n27: f64 = (p.p341 * s.dn[225][27]);
        let eq17_e544_d_n28: f64 = (p.p341 * s.dn[225][28]);
        let eq17_e544_d_n29: f64 = (p.p341 * s.dn[225][29]);
        let eq17_e544_d_b0: f64 = (p.p341 * s.db[225][0]);
        let eq17_e544_d_b1: f64 = (p.p341 * s.db[225][1]);
        let eq17_e544_d_b2: f64 = (p.p341 * s.db[225][2]);
        let eq17_e544_d_b3: f64 = (p.p341 * s.db[225][3]);
        let eq17_e544_d_b4: f64 = (p.p341 * s.db[225][4]);
        let eq17_e544_d_b5: f64 = (p.p341 * s.db[225][5]);
        let eq17_e544_d_b6: f64 = (p.p341 * s.db[225][6]);
        let eq17_e544_d_b7: f64 = (p.p341 * s.db[225][7]);
        let eq17_e544_d_b8: f64 = (p.p341 * s.db[225][8]);
        let eq17_e544_d_b9: f64 = (p.p341 * s.db[225][9]);
        let eq17_e544_d_b10: f64 = (p.p341 * s.db[225][10]);
        let eq17_e544_d_b11: f64 = (p.p341 * s.db[225][11]);
        let eq17_e544_d_b12: f64 = (p.p341 * s.db[225][12]);
        let eq17_e544_d_b13: f64 = (p.p341 * s.db[225][13]);
        let eq17_e544_d_b14: f64 = (p.p341 * s.db[225][14]);
        let eq17_e544_d_b15: f64 = (p.p341 * s.db[225][15]);
        let eq17_e544_d_b16: f64 = (p.p341 * s.db[225][16]);
        let eq17_e544_d_b17: f64 = (p.p341 * s.db[225][17]);
        let eq17_e544_d_b18: f64 = (p.p341 * s.db[225][18]);
        let eq17_e544_d_b19: f64 = (p.p341 * s.db[225][19]);
        let eq17_e544_d_b20: f64 = (p.p341 * s.db[225][20]);
        let eq17_e544_d_b21: f64 = (p.p341 * s.db[225][21]);
        let eq17_e544_d_b22: f64 = (p.p341 * s.db[225][22]);
        let eq17_e544_d_b23: f64 = (p.p341 * s.db[225][23]);
        let eq17_e544_d_b24: f64 = (p.p341 * s.db[225][24]);
        let eq17_e544_d_b25: f64 = (p.p341 * s.db[225][25]);
        let eq17_e544_d_b26: f64 = (p.p341 * s.db[225][26]);
        let eq17_e544_d_b27: f64 = (p.p341 * s.db[225][27]);
        let eq17_e544_d_b28: f64 = (p.p341 * s.db[225][28]);
        let eq17_e544_d_b29: f64 = (p.p341 * s.db[225][29]);
        let eq17_e544_d_b30: f64 = (p.p341 * s.db[225][30]);
        let eq17_e544_d_b31: f64 = (p.p341 * s.db[225][31]);
        let eq17_e544_d_b32: f64 = (p.p341 * s.db[225][32]);
        let eq17_e544_d_b33: f64 = (p.p341 * s.db[225][33]);
        let eq17_e544_d_b34: f64 = (p.p341 * s.db[225][34]);
        let eq17_e544_d_b35: f64 = (p.p341 * s.db[225][35]);
        let eq17_e544_q: f64 = (p.p341 * eq17_e543_q);
        let eq17_e549: f64 = (s.v[111] - s.v[109]);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n0: f64 = (p.p342 * s.dn[111][0]);
        let eq17_e550_d_n1: f64 = (p.p342 * s.dn[111][1]);
        let eq17_e550_d_n2: f64 = (p.p342 * s.dn[111][2]);
        let eq17_e550_d_n3: f64 = (p.p342 * s.dn[111][3]);
        let eq17_e550_d_n4: f64 = (p.p342 * s.dn[111][4]);
        let eq17_e550_d_n5: f64 = (p.p342 * s.dn[111][5]);
        let eq17_e550_d_n6: f64 = (p.p342 * s.dn[111][6]);
        let eq17_e550_d_n7: f64 = (p.p342 * s.dn[111][7]);
        let eq17_e550_d_n8: f64 = (p.p342 * s.dn[111][8]);
        let eq17_e550_d_n9: f64 = (p.p342 * s.dn[111][9]);
        let eq17_e550_d_n10: f64 = (p.p342 * s.dn[111][10]);
        let eq17_e550_d_n11: f64 = (p.p342 * s.dn[111][11]);
        let eq17_e550_d_n12: f64 = (p.p342 * s.dn[111][12]);
        let eq17_e550_d_n13: f64 = (p.p342 * s.dn[111][13]);
        let eq17_e550_d_n14: f64 = (p.p342 * s.dn[111][14]);
        let eq17_e550_d_n15: f64 = (p.p342 * s.dn[111][15]);
        let eq17_e550_d_n16: f64 = (p.p342 * s.dn[111][16]);
        let eq17_e550_d_n17: f64 = (p.p342 * s.dn[111][17]);
        let eq17_e550_d_n18: f64 = (p.p342 * s.dn[111][18]);
        let eq17_e550_d_n19: f64 = (p.p342 * s.dn[111][19]);
        let eq17_e550_d_n20: f64 = (p.p342 * s.dn[111][20]);
        let eq17_e550_d_n21: f64 = (p.p342 * s.dn[111][21]);
        let eq17_e550_d_n22: f64 = (p.p342 * s.dn[111][22]);
        let eq17_e550_d_n23: f64 = (p.p342 * s.dn[111][23]);
        let eq17_e550_d_n24: f64 = (p.p342 * s.dn[111][24]);
        let eq17_e550_d_n25: f64 = (p.p342 * s.dn[111][25]);
        let eq17_e550_d_n26: f64 = (p.p342 * s.dn[111][26]);
        let eq17_e550_d_n27: f64 = (p.p342 * s.dn[111][27]);
        let eq17_e550_d_n28: f64 = (p.p342 * s.dn[111][28]);
        let eq17_e550_d_n29: f64 = (p.p342 * s.dn[111][29]);
        let eq17_e550_d_b0: f64 = (p.p342 * s.db[111][0]);
        let eq17_e550_d_b1: f64 = (p.p342 * s.db[111][1]);
        let eq17_e550_d_b2: f64 = (p.p342 * s.db[111][2]);
        let eq17_e550_d_b3: f64 = (p.p342 * s.db[111][3]);
        let eq17_e550_d_b4: f64 = (p.p342 * s.db[111][4]);
        let eq17_e550_d_b5: f64 = (p.p342 * s.db[111][5]);
        let eq17_e550_d_b6: f64 = (p.p342 * s.db[111][6]);
        let eq17_e550_d_b7: f64 = (p.p342 * s.db[111][7]);
        let eq17_e550_d_b8: f64 = (p.p342 * s.db[111][8]);
        let eq17_e550_d_b9: f64 = (p.p342 * s.db[111][9]);
        let eq17_e550_d_b10: f64 = (p.p342 * s.db[111][10]);
        let eq17_e550_d_b11: f64 = (p.p342 * s.db[111][11]);
        let eq17_e550_d_b12: f64 = (p.p342 * s.db[111][12]);
        let eq17_e550_d_b13: f64 = (p.p342 * s.db[111][13]);
        let eq17_e550_d_b14: f64 = (p.p342 * s.db[111][14]);
        let eq17_e550_d_b15: f64 = (p.p342 * s.db[111][15]);
        let eq17_e550_d_b16: f64 = (p.p342 * s.db[111][16]);
        let eq17_e550_d_b17: f64 = (p.p342 * s.db[111][17]);
        let eq17_e550_d_b18: f64 = (p.p342 * s.db[111][18]);
        let eq17_e550_d_b19: f64 = (p.p342 * s.db[111][19]);
        let eq17_e550_d_b20: f64 = (p.p342 * s.db[111][20]);
        let eq17_e550_d_b21: f64 = (p.p342 * s.db[111][21]);
        let eq17_e550_d_b22: f64 = (p.p342 * s.db[111][22]);
        let eq17_e550_d_b23: f64 = (p.p342 * s.db[111][23]);
        let eq17_e550_d_b24: f64 = (p.p342 * s.db[111][24]);
        let eq17_e550_d_b25: f64 = (p.p342 * s.db[111][25]);
        let eq17_e550_d_b26: f64 = (p.p342 * s.db[111][26]);
        let eq17_e550_d_b27: f64 = (p.p342 * s.db[111][27]);
        let eq17_e550_d_b28: f64 = (p.p342 * s.db[111][28]);
        let eq17_e550_d_b29: f64 = (p.p342 * s.db[111][29]);
        let eq17_e550_d_b30: f64 = (p.p342 * s.db[111][30]);
        let eq17_e550_d_b31: f64 = (p.p342 * s.db[111][31]);
        let eq17_e550_d_b32: f64 = (p.p342 * s.db[111][32]);
        let eq17_e550_d_b33: f64 = (p.p342 * s.db[111][33]);
        let eq17_e550_d_b34: f64 = (p.p342 * s.db[111][34]);
        let eq17_e550_d_b35: f64 = (p.p342 * s.db[111][35]);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (s.v[111] - s.v[109]);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n0: f64 = (p.p344 * s.dn[111][0]);
        let eq17_e556_d_n1: f64 = (p.p344 * s.dn[111][1]);
        let eq17_e556_d_n2: f64 = (p.p344 * s.dn[111][2]);
        let eq17_e556_d_n3: f64 = (p.p344 * s.dn[111][3]);
        let eq17_e556_d_n4: f64 = (p.p344 * s.dn[111][4]);
        let eq17_e556_d_n5: f64 = (p.p344 * s.dn[111][5]);
        let eq17_e556_d_n6: f64 = (p.p344 * s.dn[111][6]);
        let eq17_e556_d_n7: f64 = (p.p344 * s.dn[111][7]);
        let eq17_e556_d_n8: f64 = (p.p344 * s.dn[111][8]);
        let eq17_e556_d_n9: f64 = (p.p344 * s.dn[111][9]);
        let eq17_e556_d_n10: f64 = (p.p344 * s.dn[111][10]);
        let eq17_e556_d_n11: f64 = (p.p344 * s.dn[111][11]);
        let eq17_e556_d_n12: f64 = (p.p344 * s.dn[111][12]);
        let eq17_e556_d_n13: f64 = (p.p344 * s.dn[111][13]);
        let eq17_e556_d_n14: f64 = (p.p344 * s.dn[111][14]);
        let eq17_e556_d_n15: f64 = (p.p344 * s.dn[111][15]);
        let eq17_e556_d_n16: f64 = (p.p344 * s.dn[111][16]);
        let eq17_e556_d_n17: f64 = (p.p344 * s.dn[111][17]);
        let eq17_e556_d_n18: f64 = (p.p344 * s.dn[111][18]);
        let eq17_e556_d_n19: f64 = (p.p344 * s.dn[111][19]);
        let eq17_e556_d_n20: f64 = (p.p344 * s.dn[111][20]);
        let eq17_e556_d_n21: f64 = (p.p344 * s.dn[111][21]);
        let eq17_e556_d_n22: f64 = (p.p344 * s.dn[111][22]);
        let eq17_e556_d_n23: f64 = (p.p344 * s.dn[111][23]);
        let eq17_e556_d_n24: f64 = (p.p344 * s.dn[111][24]);
        let eq17_e556_d_n25: f64 = (p.p344 * s.dn[111][25]);
        let eq17_e556_d_n26: f64 = (p.p344 * s.dn[111][26]);
        let eq17_e556_d_n27: f64 = (p.p344 * s.dn[111][27]);
        let eq17_e556_d_n28: f64 = (p.p344 * s.dn[111][28]);
        let eq17_e556_d_n29: f64 = (p.p344 * s.dn[111][29]);
        let eq17_e556_d_b0: f64 = (p.p344 * s.db[111][0]);
        let eq17_e556_d_b1: f64 = (p.p344 * s.db[111][1]);
        let eq17_e556_d_b2: f64 = (p.p344 * s.db[111][2]);
        let eq17_e556_d_b3: f64 = (p.p344 * s.db[111][3]);
        let eq17_e556_d_b4: f64 = (p.p344 * s.db[111][4]);
        let eq17_e556_d_b5: f64 = (p.p344 * s.db[111][5]);
        let eq17_e556_d_b6: f64 = (p.p344 * s.db[111][6]);
        let eq17_e556_d_b7: f64 = (p.p344 * s.db[111][7]);
        let eq17_e556_d_b8: f64 = (p.p344 * s.db[111][8]);
        let eq17_e556_d_b9: f64 = (p.p344 * s.db[111][9]);
        let eq17_e556_d_b10: f64 = (p.p344 * s.db[111][10]);
        let eq17_e556_d_b11: f64 = (p.p344 * s.db[111][11]);
        let eq17_e556_d_b12: f64 = (p.p344 * s.db[111][12]);
        let eq17_e556_d_b13: f64 = (p.p344 * s.db[111][13]);
        let eq17_e556_d_b14: f64 = (p.p344 * s.db[111][14]);
        let eq17_e556_d_b15: f64 = (p.p344 * s.db[111][15]);
        let eq17_e556_d_b16: f64 = (p.p344 * s.db[111][16]);
        let eq17_e556_d_b17: f64 = (p.p344 * s.db[111][17]);
        let eq17_e556_d_b18: f64 = (p.p344 * s.db[111][18]);
        let eq17_e556_d_b19: f64 = (p.p344 * s.db[111][19]);
        let eq17_e556_d_b20: f64 = (p.p344 * s.db[111][20]);
        let eq17_e556_d_b21: f64 = (p.p344 * s.db[111][21]);
        let eq17_e556_d_b22: f64 = (p.p344 * s.db[111][22]);
        let eq17_e556_d_b23: f64 = (p.p344 * s.db[111][23]);
        let eq17_e556_d_b24: f64 = (p.p344 * s.db[111][24]);
        let eq17_e556_d_b25: f64 = (p.p344 * s.db[111][25]);
        let eq17_e556_d_b26: f64 = (p.p344 * s.db[111][26]);
        let eq17_e556_d_b27: f64 = (p.p344 * s.db[111][27]);
        let eq17_e556_d_b28: f64 = (p.p344 * s.db[111][28]);
        let eq17_e556_d_b29: f64 = (p.p344 * s.db[111][29]);
        let eq17_e556_d_b30: f64 = (p.p344 * s.db[111][30]);
        let eq17_e556_d_b31: f64 = (p.p344 * s.db[111][31]);
        let eq17_e556_d_b32: f64 = (p.p344 * s.db[111][32]);
        let eq17_e556_d_b33: f64 = (p.p344 * s.db[111][33]);
        let eq17_e556_d_b34: f64 = (p.p344 * s.db[111][34]);
        let eq17_e556_d_b35: f64 = (p.p344 * s.db[111][35]);
        let eq17_e559: f64 = (s.v[111] - s.v[109]);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n0: f64 = ((eq17_e556_d_n0 * eq17_e559) + (eq17_e556 * s.dn[111][0]));
        let eq17_e560_d_n1: f64 = ((eq17_e556_d_n1 * eq17_e559) + (eq17_e556 * s.dn[111][1]));
        let eq17_e560_d_n2: f64 = ((eq17_e556_d_n2 * eq17_e559) + (eq17_e556 * s.dn[111][2]));
        let eq17_e560_d_n3: f64 = ((eq17_e556_d_n3 * eq17_e559) + (eq17_e556 * s.dn[111][3]));
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * s.dn[111][4]));
        let eq17_e560_d_n5: f64 = ((eq17_e556_d_n5 * eq17_e559) + (eq17_e556 * s.dn[111][5]));
        let eq17_e560_d_n6: f64 = ((eq17_e556_d_n6 * eq17_e559) + (eq17_e556 * s.dn[111][6]));
        let eq17_e560_d_n7: f64 = ((eq17_e556_d_n7 * eq17_e559) + (eq17_e556 * s.dn[111][7]));
        let eq17_e560_d_n8: f64 = ((eq17_e556_d_n8 * eq17_e559) + (eq17_e556 * s.dn[111][8]));
        let eq17_e560_d_n9: f64 = ((eq17_e556_d_n9 * eq17_e559) + (eq17_e556 * s.dn[111][9]));
        let eq17_e560_d_n10: f64 = ((eq17_e556_d_n10 * eq17_e559) + (eq17_e556 * s.dn[111][10]));
        let eq17_e560_d_n11: f64 = ((eq17_e556_d_n11 * eq17_e559) + (eq17_e556 * s.dn[111][11]));
        let eq17_e560_d_n12: f64 = ((eq17_e556_d_n12 * eq17_e559) + (eq17_e556 * s.dn[111][12]));
        let eq17_e560_d_n13: f64 = ((eq17_e556_d_n13 * eq17_e559) + (eq17_e556 * s.dn[111][13]));
        let eq17_e560_d_n14: f64 = ((eq17_e556_d_n14 * eq17_e559) + (eq17_e556 * s.dn[111][14]));
        let eq17_e560_d_n15: f64 = ((eq17_e556_d_n15 * eq17_e559) + (eq17_e556 * s.dn[111][15]));
        let eq17_e560_d_n16: f64 = ((eq17_e556_d_n16 * eq17_e559) + (eq17_e556 * s.dn[111][16]));
        let eq17_e560_d_n17: f64 = ((eq17_e556_d_n17 * eq17_e559) + (eq17_e556 * s.dn[111][17]));
        let eq17_e560_d_n18: f64 = ((eq17_e556_d_n18 * eq17_e559) + (eq17_e556 * s.dn[111][18]));
        let eq17_e560_d_n19: f64 = ((eq17_e556_d_n19 * eq17_e559) + (eq17_e556 * s.dn[111][19]));
        let eq17_e560_d_n20: f64 = ((eq17_e556_d_n20 * eq17_e559) + (eq17_e556 * s.dn[111][20]));
        let eq17_e560_d_n21: f64 = ((eq17_e556_d_n21 * eq17_e559) + (eq17_e556 * s.dn[111][21]));
        let eq17_e560_d_n22: f64 = ((eq17_e556_d_n22 * eq17_e559) + (eq17_e556 * s.dn[111][22]));
        let eq17_e560_d_n23: f64 = ((eq17_e556_d_n23 * eq17_e559) + (eq17_e556 * s.dn[111][23]));
        let eq17_e560_d_n24: f64 = ((eq17_e556_d_n24 * eq17_e559) + (eq17_e556 * s.dn[111][24]));
        let eq17_e560_d_n25: f64 = ((eq17_e556_d_n25 * eq17_e559) + (eq17_e556 * s.dn[111][25]));
        let eq17_e560_d_n26: f64 = ((eq17_e556_d_n26 * eq17_e559) + (eq17_e556 * s.dn[111][26]));
        let eq17_e560_d_n27: f64 = ((eq17_e556_d_n27 * eq17_e559) + (eq17_e556 * s.dn[111][27]));
        let eq17_e560_d_n28: f64 = ((eq17_e556_d_n28 * eq17_e559) + (eq17_e556 * s.dn[111][28]));
        let eq17_e560_d_n29: f64 = ((eq17_e556_d_n29 * eq17_e559) + (eq17_e556 * s.dn[111][29]));
        let eq17_e560_d_b0: f64 = ((eq17_e556_d_b0 * eq17_e559) + (eq17_e556 * s.db[111][0]));
        let eq17_e560_d_b1: f64 = ((eq17_e556_d_b1 * eq17_e559) + (eq17_e556 * s.db[111][1]));
        let eq17_e560_d_b2: f64 = ((eq17_e556_d_b2 * eq17_e559) + (eq17_e556 * s.db[111][2]));
        let eq17_e560_d_b3: f64 = ((eq17_e556_d_b3 * eq17_e559) + (eq17_e556 * s.db[111][3]));
        let eq17_e560_d_b4: f64 = ((eq17_e556_d_b4 * eq17_e559) + (eq17_e556 * s.db[111][4]));
        let eq17_e560_d_b5: f64 = ((eq17_e556_d_b5 * eq17_e559) + (eq17_e556 * s.db[111][5]));
        let eq17_e560_d_b6: f64 = ((eq17_e556_d_b6 * eq17_e559) + (eq17_e556 * s.db[111][6]));
        let eq17_e560_d_b7: f64 = ((eq17_e556_d_b7 * eq17_e559) + (eq17_e556 * s.db[111][7]));
        let eq17_e560_d_b8: f64 = ((eq17_e556_d_b8 * eq17_e559) + (eq17_e556 * s.db[111][8]));
        let eq17_e560_d_b9: f64 = ((eq17_e556_d_b9 * eq17_e559) + (eq17_e556 * s.db[111][9]));
        let eq17_e560_d_b10: f64 = ((eq17_e556_d_b10 * eq17_e559) + (eq17_e556 * s.db[111][10]));
        let eq17_e560_d_b11: f64 = ((eq17_e556_d_b11 * eq17_e559) + (eq17_e556 * s.db[111][11]));
        let eq17_e560_d_b12: f64 = ((eq17_e556_d_b12 * eq17_e559) + (eq17_e556 * s.db[111][12]));
        let eq17_e560_d_b13: f64 = ((eq17_e556_d_b13 * eq17_e559) + (eq17_e556 * s.db[111][13]));
        let eq17_e560_d_b14: f64 = ((eq17_e556_d_b14 * eq17_e559) + (eq17_e556 * s.db[111][14]));
        let eq17_e560_d_b15: f64 = ((eq17_e556_d_b15 * eq17_e559) + (eq17_e556 * s.db[111][15]));
        let eq17_e560_d_b16: f64 = ((eq17_e556_d_b16 * eq17_e559) + (eq17_e556 * s.db[111][16]));
        let eq17_e560_d_b17: f64 = ((eq17_e556_d_b17 * eq17_e559) + (eq17_e556 * s.db[111][17]));
        let eq17_e560_d_b18: f64 = ((eq17_e556_d_b18 * eq17_e559) + (eq17_e556 * s.db[111][18]));
        let eq17_e560_d_b19: f64 = ((eq17_e556_d_b19 * eq17_e559) + (eq17_e556 * s.db[111][19]));
        let eq17_e560_d_b20: f64 = ((eq17_e556_d_b20 * eq17_e559) + (eq17_e556 * s.db[111][20]));
        let eq17_e560_d_b21: f64 = ((eq17_e556_d_b21 * eq17_e559) + (eq17_e556 * s.db[111][21]));
        let eq17_e560_d_b22: f64 = ((eq17_e556_d_b22 * eq17_e559) + (eq17_e556 * s.db[111][22]));
        let eq17_e560_d_b23: f64 = ((eq17_e556_d_b23 * eq17_e559) + (eq17_e556 * s.db[111][23]));
        let eq17_e560_d_b24: f64 = ((eq17_e556_d_b24 * eq17_e559) + (eq17_e556 * s.db[111][24]));
        let eq17_e560_d_b25: f64 = ((eq17_e556_d_b25 * eq17_e559) + (eq17_e556 * s.db[111][25]));
        let eq17_e560_d_b26: f64 = ((eq17_e556_d_b26 * eq17_e559) + (eq17_e556 * s.db[111][26]));
        let eq17_e560_d_b27: f64 = ((eq17_e556_d_b27 * eq17_e559) + (eq17_e556 * s.db[111][27]));
        let eq17_e560_d_b28: f64 = ((eq17_e556_d_b28 * eq17_e559) + (eq17_e556 * s.db[111][28]));
        let eq17_e560_d_b29: f64 = ((eq17_e556_d_b29 * eq17_e559) + (eq17_e556 * s.db[111][29]));
        let eq17_e560_d_b30: f64 = ((eq17_e556_d_b30 * eq17_e559) + (eq17_e556 * s.db[111][30]));
        let eq17_e560_d_b31: f64 = ((eq17_e556_d_b31 * eq17_e559) + (eq17_e556 * s.db[111][31]));
        let eq17_e560_d_b32: f64 = ((eq17_e556_d_b32 * eq17_e559) + (eq17_e556 * s.db[111][32]));
        let eq17_e560_d_b33: f64 = ((eq17_e556_d_b33 * eq17_e559) + (eq17_e556 * s.db[111][33]));
        let eq17_e560_d_b34: f64 = ((eq17_e556_d_b34 * eq17_e559) + (eq17_e556 * s.db[111][34]));
        let eq17_e560_d_b35: f64 = ((eq17_e556_d_b35 * eq17_e559) + (eq17_e556 * s.db[111][35]));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n0: f64 = (eq17_e550_d_n0 + eq17_e560_d_n0);
        let eq17_e561_d_n1: f64 = (eq17_e550_d_n1 + eq17_e560_d_n1);
        let eq17_e561_d_n2: f64 = (eq17_e550_d_n2 + eq17_e560_d_n2);
        let eq17_e561_d_n3: f64 = (eq17_e550_d_n3 + eq17_e560_d_n3);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e561_d_n5: f64 = (eq17_e550_d_n5 + eq17_e560_d_n5);
        let eq17_e561_d_n6: f64 = (eq17_e550_d_n6 + eq17_e560_d_n6);
        let eq17_e561_d_n7: f64 = (eq17_e550_d_n7 + eq17_e560_d_n7);
        let eq17_e561_d_n8: f64 = (eq17_e550_d_n8 + eq17_e560_d_n8);
        let eq17_e561_d_n9: f64 = (eq17_e550_d_n9 + eq17_e560_d_n9);
        let eq17_e561_d_n10: f64 = (eq17_e550_d_n10 + eq17_e560_d_n10);
        let eq17_e561_d_n11: f64 = (eq17_e550_d_n11 + eq17_e560_d_n11);
        let eq17_e561_d_n12: f64 = (eq17_e550_d_n12 + eq17_e560_d_n12);
        let eq17_e561_d_n13: f64 = (eq17_e550_d_n13 + eq17_e560_d_n13);
        let eq17_e561_d_n14: f64 = (eq17_e550_d_n14 + eq17_e560_d_n14);
        let eq17_e561_d_n15: f64 = (eq17_e550_d_n15 + eq17_e560_d_n15);
        let eq17_e561_d_n16: f64 = (eq17_e550_d_n16 + eq17_e560_d_n16);
        let eq17_e561_d_n17: f64 = (eq17_e550_d_n17 + eq17_e560_d_n17);
        let eq17_e561_d_n18: f64 = (eq17_e550_d_n18 + eq17_e560_d_n18);
        let eq17_e561_d_n19: f64 = (eq17_e550_d_n19 + eq17_e560_d_n19);
        let eq17_e561_d_n20: f64 = (eq17_e550_d_n20 + eq17_e560_d_n20);
        let eq17_e561_d_n21: f64 = (eq17_e550_d_n21 + eq17_e560_d_n21);
        let eq17_e561_d_n22: f64 = (eq17_e550_d_n22 + eq17_e560_d_n22);
        let eq17_e561_d_n23: f64 = (eq17_e550_d_n23 + eq17_e560_d_n23);
        let eq17_e561_d_n24: f64 = (eq17_e550_d_n24 + eq17_e560_d_n24);
        let eq17_e561_d_n25: f64 = (eq17_e550_d_n25 + eq17_e560_d_n25);
        let eq17_e561_d_n26: f64 = (eq17_e550_d_n26 + eq17_e560_d_n26);
        let eq17_e561_d_n27: f64 = (eq17_e550_d_n27 + eq17_e560_d_n27);
        let eq17_e561_d_n28: f64 = (eq17_e550_d_n28 + eq17_e560_d_n28);
        let eq17_e561_d_n29: f64 = (eq17_e550_d_n29 + eq17_e560_d_n29);
        let eq17_e561_d_b0: f64 = (eq17_e550_d_b0 + eq17_e560_d_b0);
        let eq17_e561_d_b1: f64 = (eq17_e550_d_b1 + eq17_e560_d_b1);
        let eq17_e561_d_b2: f64 = (eq17_e550_d_b2 + eq17_e560_d_b2);
        let eq17_e561_d_b3: f64 = (eq17_e550_d_b3 + eq17_e560_d_b3);
        let eq17_e561_d_b4: f64 = (eq17_e550_d_b4 + eq17_e560_d_b4);
        let eq17_e561_d_b5: f64 = (eq17_e550_d_b5 + eq17_e560_d_b5);
        let eq17_e561_d_b6: f64 = (eq17_e550_d_b6 + eq17_e560_d_b6);
        let eq17_e561_d_b7: f64 = (eq17_e550_d_b7 + eq17_e560_d_b7);
        let eq17_e561_d_b8: f64 = (eq17_e550_d_b8 + eq17_e560_d_b8);
        let eq17_e561_d_b9: f64 = (eq17_e550_d_b9 + eq17_e560_d_b9);
        let eq17_e561_d_b10: f64 = (eq17_e550_d_b10 + eq17_e560_d_b10);
        let eq17_e561_d_b11: f64 = (eq17_e550_d_b11 + eq17_e560_d_b11);
        let eq17_e561_d_b12: f64 = (eq17_e550_d_b12 + eq17_e560_d_b12);
        let eq17_e561_d_b13: f64 = (eq17_e550_d_b13 + eq17_e560_d_b13);
        let eq17_e561_d_b14: f64 = (eq17_e550_d_b14 + eq17_e560_d_b14);
        let eq17_e561_d_b15: f64 = (eq17_e550_d_b15 + eq17_e560_d_b15);
        let eq17_e561_d_b16: f64 = (eq17_e550_d_b16 + eq17_e560_d_b16);
        let eq17_e561_d_b17: f64 = (eq17_e550_d_b17 + eq17_e560_d_b17);
        let eq17_e561_d_b18: f64 = (eq17_e550_d_b18 + eq17_e560_d_b18);
        let eq17_e561_d_b19: f64 = (eq17_e550_d_b19 + eq17_e560_d_b19);
        let eq17_e561_d_b20: f64 = (eq17_e550_d_b20 + eq17_e560_d_b20);
        let eq17_e561_d_b21: f64 = (eq17_e550_d_b21 + eq17_e560_d_b21);
        let eq17_e561_d_b22: f64 = (eq17_e550_d_b22 + eq17_e560_d_b22);
        let eq17_e561_d_b23: f64 = (eq17_e550_d_b23 + eq17_e560_d_b23);
        let eq17_e561_d_b24: f64 = (eq17_e550_d_b24 + eq17_e560_d_b24);
        let eq17_e561_d_b25: f64 = (eq17_e550_d_b25 + eq17_e560_d_b25);
        let eq17_e561_d_b26: f64 = (eq17_e550_d_b26 + eq17_e560_d_b26);
        let eq17_e561_d_b27: f64 = (eq17_e550_d_b27 + eq17_e560_d_b27);
        let eq17_e561_d_b28: f64 = (eq17_e550_d_b28 + eq17_e560_d_b28);
        let eq17_e561_d_b29: f64 = (eq17_e550_d_b29 + eq17_e560_d_b29);
        let eq17_e561_d_b30: f64 = (eq17_e550_d_b30 + eq17_e560_d_b30);
        let eq17_e561_d_b31: f64 = (eq17_e550_d_b31 + eq17_e560_d_b31);
        let eq17_e561_d_b32: f64 = (eq17_e550_d_b32 + eq17_e560_d_b32);
        let eq17_e561_d_b33: f64 = (eq17_e550_d_b33 + eq17_e560_d_b33);
        let eq17_e561_d_b34: f64 = (eq17_e550_d_b34 + eq17_e560_d_b34);
        let eq17_e561_d_b35: f64 = (eq17_e550_d_b35 + eq17_e560_d_b35);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n0: f64 = ((eq17_e544_d_n0 * eq17_e561) + (eq17_e544 * eq17_e561_d_n0));
        let eq17_e562_d_n1: f64 = ((eq17_e544_d_n1 * eq17_e561) + (eq17_e544 * eq17_e561_d_n1));
        let eq17_e562_d_n2: f64 = ((eq17_e544_d_n2 * eq17_e561) + (eq17_e544 * eq17_e561_d_n2));
        let eq17_e562_d_n3: f64 = ((eq17_e544_d_n3 * eq17_e561) + (eq17_e544 * eq17_e561_d_n3));
        let eq17_e562_d_n4: f64 = ((eq17_e544_d_n4 * eq17_e561) + (eq17_e544 * eq17_e561_d_n4));
        let eq17_e562_d_n5: f64 = ((eq17_e544_d_n5 * eq17_e561) + (eq17_e544 * eq17_e561_d_n5));
        let eq17_e562_d_n6: f64 = ((eq17_e544_d_n6 * eq17_e561) + (eq17_e544 * eq17_e561_d_n6));
        let eq17_e562_d_n7: f64 = ((eq17_e544_d_n7 * eq17_e561) + (eq17_e544 * eq17_e561_d_n7));
        let eq17_e562_d_n8: f64 = ((eq17_e544_d_n8 * eq17_e561) + (eq17_e544 * eq17_e561_d_n8));
        let eq17_e562_d_n9: f64 = ((eq17_e544_d_n9 * eq17_e561) + (eq17_e544 * eq17_e561_d_n9));
        let eq17_e562_d_n10: f64 = ((eq17_e544_d_n10 * eq17_e561) + (eq17_e544 * eq17_e561_d_n10));
        let eq17_e562_d_n11: f64 = ((eq17_e544_d_n11 * eq17_e561) + (eq17_e544 * eq17_e561_d_n11));
        let eq17_e562_d_n12: f64 = ((eq17_e544_d_n12 * eq17_e561) + (eq17_e544 * eq17_e561_d_n12));
        let eq17_e562_d_n13: f64 = ((eq17_e544_d_n13 * eq17_e561) + (eq17_e544 * eq17_e561_d_n13));
        let eq17_e562_d_n14: f64 = ((eq17_e544_d_n14 * eq17_e561) + (eq17_e544 * eq17_e561_d_n14));
        let eq17_e562_d_n15: f64 = ((eq17_e544_d_n15 * eq17_e561) + (eq17_e544 * eq17_e561_d_n15));
        let eq17_e562_d_n16: f64 = ((eq17_e544_d_n16 * eq17_e561) + (eq17_e544 * eq17_e561_d_n16));
        let eq17_e562_d_n17: f64 = ((eq17_e544_d_n17 * eq17_e561) + (eq17_e544 * eq17_e561_d_n17));
        let eq17_e562_d_n18: f64 = ((eq17_e544_d_n18 * eq17_e561) + (eq17_e544 * eq17_e561_d_n18));
        let eq17_e562_d_n19: f64 = ((eq17_e544_d_n19 * eq17_e561) + (eq17_e544 * eq17_e561_d_n19));
        let eq17_e562_d_n20: f64 = ((eq17_e544_d_n20 * eq17_e561) + (eq17_e544 * eq17_e561_d_n20));
        let eq17_e562_d_n21: f64 = ((eq17_e544_d_n21 * eq17_e561) + (eq17_e544 * eq17_e561_d_n21));
        let eq17_e562_d_n22: f64 = ((eq17_e544_d_n22 * eq17_e561) + (eq17_e544 * eq17_e561_d_n22));
        let eq17_e562_d_n23: f64 = ((eq17_e544_d_n23 * eq17_e561) + (eq17_e544 * eq17_e561_d_n23));
        let eq17_e562_d_n24: f64 = ((eq17_e544_d_n24 * eq17_e561) + (eq17_e544 * eq17_e561_d_n24));
        let eq17_e562_d_n25: f64 = ((eq17_e544_d_n25 * eq17_e561) + (eq17_e544 * eq17_e561_d_n25));
        let eq17_e562_d_n26: f64 = ((eq17_e544_d_n26 * eq17_e561) + (eq17_e544 * eq17_e561_d_n26));
        let eq17_e562_d_n27: f64 = ((eq17_e544_d_n27 * eq17_e561) + (eq17_e544 * eq17_e561_d_n27));
        let eq17_e562_d_n28: f64 = ((eq17_e544_d_n28 * eq17_e561) + (eq17_e544 * eq17_e561_d_n28));
        let eq17_e562_d_n29: f64 = ((eq17_e544_d_n29 * eq17_e561) + (eq17_e544 * eq17_e561_d_n29));
        let eq17_e562_d_b0: f64 = ((eq17_e544_d_b0 * eq17_e561) + (eq17_e544 * eq17_e561_d_b0));
        let eq17_e562_d_b1: f64 = ((eq17_e544_d_b1 * eq17_e561) + (eq17_e544 * eq17_e561_d_b1));
        let eq17_e562_d_b2: f64 = ((eq17_e544_d_b2 * eq17_e561) + (eq17_e544 * eq17_e561_d_b2));
        let eq17_e562_d_b3: f64 = ((eq17_e544_d_b3 * eq17_e561) + (eq17_e544 * eq17_e561_d_b3));
        let eq17_e562_d_b4: f64 = ((eq17_e544_d_b4 * eq17_e561) + (eq17_e544 * eq17_e561_d_b4));
        let eq17_e562_d_b5: f64 = ((eq17_e544_d_b5 * eq17_e561) + (eq17_e544 * eq17_e561_d_b5));
        let eq17_e562_d_b6: f64 = ((eq17_e544_d_b6 * eq17_e561) + (eq17_e544 * eq17_e561_d_b6));
        let eq17_e562_d_b7: f64 = ((eq17_e544_d_b7 * eq17_e561) + (eq17_e544 * eq17_e561_d_b7));
        let eq17_e562_d_b8: f64 = ((eq17_e544_d_b8 * eq17_e561) + (eq17_e544 * eq17_e561_d_b8));
        let eq17_e562_d_b9: f64 = ((eq17_e544_d_b9 * eq17_e561) + (eq17_e544 * eq17_e561_d_b9));
        let eq17_e562_d_b10: f64 = ((eq17_e544_d_b10 * eq17_e561) + (eq17_e544 * eq17_e561_d_b10));
        let eq17_e562_d_b11: f64 = ((eq17_e544_d_b11 * eq17_e561) + (eq17_e544 * eq17_e561_d_b11));
        let eq17_e562_d_b12: f64 = ((eq17_e544_d_b12 * eq17_e561) + (eq17_e544 * eq17_e561_d_b12));
        let eq17_e562_d_b13: f64 = ((eq17_e544_d_b13 * eq17_e561) + (eq17_e544 * eq17_e561_d_b13));
        let eq17_e562_d_b14: f64 = ((eq17_e544_d_b14 * eq17_e561) + (eq17_e544 * eq17_e561_d_b14));
        let eq17_e562_d_b15: f64 = ((eq17_e544_d_b15 * eq17_e561) + (eq17_e544 * eq17_e561_d_b15));
        let eq17_e562_d_b16: f64 = ((eq17_e544_d_b16 * eq17_e561) + (eq17_e544 * eq17_e561_d_b16));
        let eq17_e562_d_b17: f64 = ((eq17_e544_d_b17 * eq17_e561) + (eq17_e544 * eq17_e561_d_b17));
        let eq17_e562_d_b18: f64 = ((eq17_e544_d_b18 * eq17_e561) + (eq17_e544 * eq17_e561_d_b18));
        let eq17_e562_d_b19: f64 = ((eq17_e544_d_b19 * eq17_e561) + (eq17_e544 * eq17_e561_d_b19));
        let eq17_e562_d_b20: f64 = ((eq17_e544_d_b20 * eq17_e561) + (eq17_e544 * eq17_e561_d_b20));
        let eq17_e562_d_b21: f64 = ((eq17_e544_d_b21 * eq17_e561) + (eq17_e544 * eq17_e561_d_b21));
        let eq17_e562_d_b22: f64 = ((eq17_e544_d_b22 * eq17_e561) + (eq17_e544 * eq17_e561_d_b22));
        let eq17_e562_d_b23: f64 = ((eq17_e544_d_b23 * eq17_e561) + (eq17_e544 * eq17_e561_d_b23));
        let eq17_e562_d_b24: f64 = ((eq17_e544_d_b24 * eq17_e561) + (eq17_e544 * eq17_e561_d_b24));
        let eq17_e562_d_b25: f64 = ((eq17_e544_d_b25 * eq17_e561) + (eq17_e544 * eq17_e561_d_b25));
        let eq17_e562_d_b26: f64 = ((eq17_e544_d_b26 * eq17_e561) + (eq17_e544 * eq17_e561_d_b26));
        let eq17_e562_d_b27: f64 = ((eq17_e544_d_b27 * eq17_e561) + (eq17_e544 * eq17_e561_d_b27));
        let eq17_e562_d_b28: f64 = ((eq17_e544_d_b28 * eq17_e561) + (eq17_e544 * eq17_e561_d_b28));
        let eq17_e562_d_b29: f64 = ((eq17_e544_d_b29 * eq17_e561) + (eq17_e544 * eq17_e561_d_b29));
        let eq17_e562_d_b30: f64 = ((eq17_e544_d_b30 * eq17_e561) + (eq17_e544 * eq17_e561_d_b30));
        let eq17_e562_d_b31: f64 = ((eq17_e544_d_b31 * eq17_e561) + (eq17_e544 * eq17_e561_d_b31));
        let eq17_e562_d_b32: f64 = ((eq17_e544_d_b32 * eq17_e561) + (eq17_e544 * eq17_e561_d_b32));
        let eq17_e562_d_b33: f64 = ((eq17_e544_d_b33 * eq17_e561) + (eq17_e544 * eq17_e561_d_b33));
        let eq17_e562_d_b34: f64 = ((eq17_e544_d_b34 * eq17_e561) + (eq17_e544 * eq17_e561_d_b34));
        let eq17_e562_d_b35: f64 = ((eq17_e544_d_b35 * eq17_e561) + (eq17_e544 * eq17_e561_d_b35));
        let eq17_e562_q: f64 = (eq17_e544_q * eq17_e561);
        let eq17_e562_q_d_n0: f64 = ((eq17_e544_d_n0 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n0));
        let eq17_e562_q_d_n1: f64 = ((eq17_e544_d_n1 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n1));
        let eq17_e562_q_d_n2: f64 = ((eq17_e544_d_n2 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n2));
        let eq17_e562_q_d_n3: f64 = ((eq17_e544_d_n3 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n3));
        let eq17_e562_q_d_n4: f64 = ((eq17_e544_d_n4 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n4));
        let eq17_e562_q_d_n5: f64 = ((eq17_e544_d_n5 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n5));
        let eq17_e562_q_d_n6: f64 = ((eq17_e544_d_n6 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n6));
        let eq17_e562_q_d_n7: f64 = ((eq17_e544_d_n7 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n7));
        let eq17_e562_q_d_n8: f64 = ((eq17_e544_d_n8 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n8));
        let eq17_e562_q_d_n9: f64 = ((eq17_e544_d_n9 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n9));
        let eq17_e562_q_d_n10: f64 = ((eq17_e544_d_n10 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n10));
        let eq17_e562_q_d_n11: f64 = ((eq17_e544_d_n11 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n11));
        let eq17_e562_q_d_n12: f64 = ((eq17_e544_d_n12 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n12));
        let eq17_e562_q_d_n13: f64 = ((eq17_e544_d_n13 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n13));
        let eq17_e562_q_d_n14: f64 = ((eq17_e544_d_n14 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n14));
        let eq17_e562_q_d_n15: f64 = ((eq17_e544_d_n15 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n15));
        let eq17_e562_q_d_n16: f64 = ((eq17_e544_d_n16 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n16));
        let eq17_e562_q_d_n17: f64 = ((eq17_e544_d_n17 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n17));
        let eq17_e562_q_d_n18: f64 = ((eq17_e544_d_n18 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n18));
        let eq17_e562_q_d_n19: f64 = ((eq17_e544_d_n19 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n19));
        let eq17_e562_q_d_n20: f64 = ((eq17_e544_d_n20 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n20));
        let eq17_e562_q_d_n21: f64 = ((eq17_e544_d_n21 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n21));
        let eq17_e562_q_d_n22: f64 = ((eq17_e544_d_n22 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n22));
        let eq17_e562_q_d_n23: f64 = ((eq17_e544_d_n23 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n23));
        let eq17_e562_q_d_n24: f64 = ((eq17_e544_d_n24 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n24));
        let eq17_e562_q_d_n25: f64 = ((eq17_e544_d_n25 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n25));
        let eq17_e562_q_d_n26: f64 = ((eq17_e544_d_n26 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n26));
        let eq17_e562_q_d_n27: f64 = ((eq17_e544_d_n27 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n27));
        let eq17_e562_q_d_n28: f64 = ((eq17_e544_d_n28 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n28));
        let eq17_e562_q_d_n29: f64 = ((eq17_e544_d_n29 * eq17_e561) + (eq17_e544_q * eq17_e561_d_n29));
        let eq17_e562_q_d_b0: f64 = ((eq17_e544_d_b0 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b0));
        let eq17_e562_q_d_b1: f64 = ((eq17_e544_d_b1 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b1));
        let eq17_e562_q_d_b2: f64 = ((eq17_e544_d_b2 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b2));
        let eq17_e562_q_d_b3: f64 = ((eq17_e544_d_b3 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b3));
        let eq17_e562_q_d_b4: f64 = ((eq17_e544_d_b4 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b4));
        let eq17_e562_q_d_b5: f64 = ((eq17_e544_d_b5 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b5));
        let eq17_e562_q_d_b6: f64 = ((eq17_e544_d_b6 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b6));
        let eq17_e562_q_d_b7: f64 = ((eq17_e544_d_b7 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b7));
        let eq17_e562_q_d_b8: f64 = ((eq17_e544_d_b8 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b8));
        let eq17_e562_q_d_b9: f64 = ((eq17_e544_d_b9 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b9));
        let eq17_e562_q_d_b10: f64 = ((eq17_e544_d_b10 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b10));
        let eq17_e562_q_d_b11: f64 = ((eq17_e544_d_b11 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b11));
        let eq17_e562_q_d_b12: f64 = ((eq17_e544_d_b12 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b12));
        let eq17_e562_q_d_b13: f64 = ((eq17_e544_d_b13 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b13));
        let eq17_e562_q_d_b14: f64 = ((eq17_e544_d_b14 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b14));
        let eq17_e562_q_d_b15: f64 = ((eq17_e544_d_b15 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b15));
        let eq17_e562_q_d_b16: f64 = ((eq17_e544_d_b16 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b16));
        let eq17_e562_q_d_b17: f64 = ((eq17_e544_d_b17 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b17));
        let eq17_e562_q_d_b18: f64 = ((eq17_e544_d_b18 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b18));
        let eq17_e562_q_d_b19: f64 = ((eq17_e544_d_b19 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b19));
        let eq17_e562_q_d_b20: f64 = ((eq17_e544_d_b20 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b20));
        let eq17_e562_q_d_b21: f64 = ((eq17_e544_d_b21 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b21));
        let eq17_e562_q_d_b22: f64 = ((eq17_e544_d_b22 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b22));
        let eq17_e562_q_d_b23: f64 = ((eq17_e544_d_b23 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b23));
        let eq17_e562_q_d_b24: f64 = ((eq17_e544_d_b24 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b24));
        let eq17_e562_q_d_b25: f64 = ((eq17_e544_d_b25 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b25));
        let eq17_e562_q_d_b26: f64 = ((eq17_e544_d_b26 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b26));
        let eq17_e562_q_d_b27: f64 = ((eq17_e544_d_b27 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b27));
        let eq17_e562_q_d_b28: f64 = ((eq17_e544_d_b28 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b28));
        let eq17_e562_q_d_b29: f64 = ((eq17_e544_d_b29 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b29));
        let eq17_e562_q_d_b30: f64 = ((eq17_e544_d_b30 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b30));
        let eq17_e562_q_d_b31: f64 = ((eq17_e544_d_b31 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b31));
        let eq17_e562_q_d_b32: f64 = ((eq17_e544_d_b32 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b32));
        let eq17_e562_q_d_b33: f64 = ((eq17_e544_d_b33 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b33));
        let eq17_e562_q_d_b34: f64 = ((eq17_e544_d_b34 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b34));
        let eq17_e562_q_d_b35: f64 = ((eq17_e544_d_b35 * eq17_e561) + (eq17_e544_q * eq17_e561_d_b35));
        (eq17_e562, eq17_e562_d_n0, eq17_e562_d_n1, eq17_e562_d_n2, eq17_e562_d_n3, eq17_e562_d_n4, eq17_e562_d_n5, eq17_e562_d_n6, eq17_e562_d_n7, eq17_e562_d_n8, eq17_e562_d_n9, eq17_e562_d_n10, eq17_e562_d_n11, eq17_e562_d_n12, eq17_e562_d_n13, eq17_e562_d_n14, eq17_e562_d_n15, eq17_e562_d_n16, eq17_e562_d_n17, eq17_e562_d_n18, eq17_e562_d_n19, eq17_e562_d_n20, eq17_e562_d_n21, eq17_e562_d_n22, eq17_e562_d_n23, eq17_e562_d_n24, eq17_e562_d_n25, eq17_e562_d_n26, eq17_e562_d_n27, eq17_e562_d_n28, eq17_e562_d_n29, eq17_e562_d_b0, eq17_e562_d_b1, eq17_e562_d_b2, eq17_e562_d_b3, eq17_e562_d_b4, eq17_e562_d_b5, eq17_e562_d_b6, eq17_e562_d_b7, eq17_e562_d_b8, eq17_e562_d_b9, eq17_e562_d_b10, eq17_e562_d_b11, eq17_e562_d_b12, eq17_e562_d_b13, eq17_e562_d_b14, eq17_e562_d_b15, eq17_e562_d_b16, eq17_e562_d_b17, eq17_e562_d_b18, eq17_e562_d_b19, eq17_e562_d_b20, eq17_e562_d_b21, eq17_e562_d_b22, eq17_e562_d_b23, eq17_e562_d_b24, eq17_e562_d_b25, eq17_e562_d_b26, eq17_e562_d_b27, eq17_e562_d_b28, eq17_e562_d_b29, eq17_e562_d_b30, eq17_e562_d_b31, eq17_e562_d_b32, eq17_e562_d_b33, eq17_e562_d_b34, eq17_e562_d_b35, eq17_e562_q, eq17_e562_q_d_n0, eq17_e562_q_d_n1, eq17_e562_q_d_n2, eq17_e562_q_d_n3, eq17_e562_q_d_n4, eq17_e562_q_d_n5, eq17_e562_q_d_n6, eq17_e562_q_d_n7, eq17_e562_q_d_n8, eq17_e562_q_d_n9, eq17_e562_q_d_n10, eq17_e562_q_d_n11, eq17_e562_q_d_n12, eq17_e562_q_d_n13, eq17_e562_q_d_n14, eq17_e562_q_d_n15, eq17_e562_q_d_n16, eq17_e562_q_d_n17, eq17_e562_q_d_n18, eq17_e562_q_d_n19, eq17_e562_q_d_n20, eq17_e562_q_d_n21, eq17_e562_q_d_n22, eq17_e562_q_d_n23, eq17_e562_q_d_n24, eq17_e562_q_d_n25, eq17_e562_q_d_n26, eq17_e562_q_d_n27, eq17_e562_q_d_n28, eq17_e562_q_d_n29, eq17_e562_q_d_b0, eq17_e562_q_d_b1, eq17_e562_q_d_b2, eq17_e562_q_d_b3, eq17_e562_q_d_b4, eq17_e562_q_d_b5, eq17_e562_q_d_b6, eq17_e562_q_d_b7, eq17_e562_q_d_b8, eq17_e562_q_d_b9, eq17_e562_q_d_b10, eq17_e562_q_d_b11, eq17_e562_q_d_b12, eq17_e562_q_d_b13, eq17_e562_q_d_b14, eq17_e562_q_d_b15, eq17_e562_q_d_b16, eq17_e562_q_d_b17, eq17_e562_q_d_b18, eq17_e562_q_d_b19, eq17_e562_q_d_b20, eq17_e562_q_d_b21, eq17_e562_q_d_b22, eq17_e562_q_d_b23, eq17_e562_q_d_b24, eq17_e562_q_d_b25, eq17_e562_q_d_b26, eq17_e562_q_d_b27, eq17_e562_q_d_b28, eq17_e562_q_d_b29, eq17_e562_q_d_b30, eq17_e562_q_d_b31, eq17_e562_q_d_b32, eq17_e562_q_d_b33, eq17_e562_q_d_b34, eq17_e562_q_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_reactive_node_derivatives: [f64; 30] = [eq17_e564_q_d_n0, eq17_e564_q_d_n1, eq17_e564_q_d_n2, eq17_e564_q_d_n3, eq17_e564_q_d_n4, eq17_e564_q_d_n5, eq17_e564_q_d_n6, eq17_e564_q_d_n7, eq17_e564_q_d_n8, eq17_e564_q_d_n9, eq17_e564_q_d_n10, eq17_e564_q_d_n11, eq17_e564_q_d_n12, eq17_e564_q_d_n13, eq17_e564_q_d_n14, eq17_e564_q_d_n15, eq17_e564_q_d_n16, eq17_e564_q_d_n17, eq17_e564_q_d_n18, eq17_e564_q_d_n19, eq17_e564_q_d_n20, eq17_e564_q_d_n21, eq17_e564_q_d_n22, eq17_e564_q_d_n23, eq17_e564_q_d_n24, eq17_e564_q_d_n25, eq17_e564_q_d_n26, eq17_e564_q_d_n27, eq17_e564_q_d_n28, eq17_e564_q_d_n29];
        let eq17_reactive_branch_derivatives: [f64; 36] = [eq17_e564_q_d_b0, eq17_e564_q_d_b1, eq17_e564_q_d_b2, eq17_e564_q_d_b3, eq17_e564_q_d_b4, eq17_e564_q_d_b5, eq17_e564_q_d_b6, eq17_e564_q_d_b7, eq17_e564_q_d_b8, eq17_e564_q_d_b9, eq17_e564_q_d_b10, eq17_e564_q_d_b11, eq17_e564_q_d_b12, eq17_e564_q_d_b13, eq17_e564_q_d_b14, eq17_e564_q_d_b15, eq17_e564_q_d_b16, eq17_e564_q_d_b17, eq17_e564_q_d_b18, eq17_e564_q_d_b19, eq17_e564_q_d_b20, eq17_e564_q_d_b21, eq17_e564_q_d_b22, eq17_e564_q_d_b23, eq17_e564_q_d_b24, eq17_e564_q_d_b25, eq17_e564_q_d_b26, eq17_e564_q_d_b27, eq17_e564_q_d_b28, eq17_e564_q_d_b29, eq17_e564_q_d_b30, eq17_e564_q_d_b31, eq17_e564_q_d_b32, eq17_e564_q_d_b33, eq17_e564_q_d_b34, eq17_e564_q_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[23]),
            None,
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
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
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29, eq22_e682_d_b0, eq22_e682_d_b1, eq22_e682_d_b2, eq22_e682_d_b3, eq22_e682_d_b4, eq22_e682_d_b5, eq22_e682_d_b6, eq22_e682_d_b7, eq22_e682_d_b8, eq22_e682_d_b9, eq22_e682_d_b10, eq22_e682_d_b11, eq22_e682_d_b12, eq22_e682_d_b13, eq22_e682_d_b14, eq22_e682_d_b15, eq22_e682_d_b16, eq22_e682_d_b17, eq22_e682_d_b18, eq22_e682_d_b19, eq22_e682_d_b20, eq22_e682_d_b21, eq22_e682_d_b22, eq22_e682_d_b23, eq22_e682_d_b24, eq22_e682_d_b25, eq22_e682_d_b26, eq22_e682_d_b27, eq22_e682_d_b28, eq22_e682_d_b29, eq22_e682_d_b30, eq22_e682_d_b31, eq22_e682_d_b32, eq22_e682_d_b33, eq22_e682_d_b34, eq22_e682_d_b35, eq22_e682_q, eq22_e682_q_d_n0, eq22_e682_q_d_n1, eq22_e682_q_d_n2, eq22_e682_q_d_n3, eq22_e682_q_d_n4, eq22_e682_q_d_n5, eq22_e682_q_d_n6, eq22_e682_q_d_n7, eq22_e682_q_d_n8, eq22_e682_q_d_n9, eq22_e682_q_d_n10, eq22_e682_q_d_n11, eq22_e682_q_d_n12, eq22_e682_q_d_n13, eq22_e682_q_d_n14, eq22_e682_q_d_n15, eq22_e682_q_d_n16, eq22_e682_q_d_n17, eq22_e682_q_d_n18, eq22_e682_q_d_n19, eq22_e682_q_d_n20, eq22_e682_q_d_n21, eq22_e682_q_d_n22, eq22_e682_q_d_n23, eq22_e682_q_d_n24, eq22_e682_q_d_n25, eq22_e682_q_d_n26, eq22_e682_q_d_n27, eq22_e682_q_d_n28, eq22_e682_q_d_n29, eq22_e682_q_d_b0, eq22_e682_q_d_b1, eq22_e682_q_d_b2, eq22_e682_q_d_b3, eq22_e682_q_d_b4, eq22_e682_q_d_b5, eq22_e682_q_d_b6, eq22_e682_q_d_b7, eq22_e682_q_d_b8, eq22_e682_q_d_b9, eq22_e682_q_d_b10, eq22_e682_q_d_b11, eq22_e682_q_d_b12, eq22_e682_q_d_b13, eq22_e682_q_d_b14, eq22_e682_q_d_b15, eq22_e682_q_d_b16, eq22_e682_q_d_b17, eq22_e682_q_d_b18, eq22_e682_q_d_b19, eq22_e682_q_d_b20, eq22_e682_q_d_b21, eq22_e682_q_d_b22, eq22_e682_q_d_b23, eq22_e682_q_d_b24, eq22_e682_q_d_b25, eq22_e682_q_d_b26, eq22_e682_q_d_b27, eq22_e682_q_d_b28, eq22_e682_q_d_b29, eq22_e682_q_d_b30, eq22_e682_q_d_b31, eq22_e682_q_d_b32, eq22_e682_q_d_b33, eq22_e682_q_d_b34, eq22_e682_q_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq22_e661_q: f64 = s.v[227];
        let eq22_e662: f64 = (p.p341 * s.v[227]);
        let eq22_e662_d_n0: f64 = (p.p341 * s.dn[227][0]);
        let eq22_e662_d_n1: f64 = (p.p341 * s.dn[227][1]);
        let eq22_e662_d_n2: f64 = (p.p341 * s.dn[227][2]);
        let eq22_e662_d_n3: f64 = (p.p341 * s.dn[227][3]);
        let eq22_e662_d_n4: f64 = (p.p341 * s.dn[227][4]);
        let eq22_e662_d_n5: f64 = (p.p341 * s.dn[227][5]);
        let eq22_e662_d_n6: f64 = (p.p341 * s.dn[227][6]);
        let eq22_e662_d_n7: f64 = (p.p341 * s.dn[227][7]);
        let eq22_e662_d_n8: f64 = (p.p341 * s.dn[227][8]);
        let eq22_e662_d_n9: f64 = (p.p341 * s.dn[227][9]);
        let eq22_e662_d_n10: f64 = (p.p341 * s.dn[227][10]);
        let eq22_e662_d_n11: f64 = (p.p341 * s.dn[227][11]);
        let eq22_e662_d_n12: f64 = (p.p341 * s.dn[227][12]);
        let eq22_e662_d_n13: f64 = (p.p341 * s.dn[227][13]);
        let eq22_e662_d_n14: f64 = (p.p341 * s.dn[227][14]);
        let eq22_e662_d_n15: f64 = (p.p341 * s.dn[227][15]);
        let eq22_e662_d_n16: f64 = (p.p341 * s.dn[227][16]);
        let eq22_e662_d_n17: f64 = (p.p341 * s.dn[227][17]);
        let eq22_e662_d_n18: f64 = (p.p341 * s.dn[227][18]);
        let eq22_e662_d_n19: f64 = (p.p341 * s.dn[227][19]);
        let eq22_e662_d_n20: f64 = (p.p341 * s.dn[227][20]);
        let eq22_e662_d_n21: f64 = (p.p341 * s.dn[227][21]);
        let eq22_e662_d_n22: f64 = (p.p341 * s.dn[227][22]);
        let eq22_e662_d_n23: f64 = (p.p341 * s.dn[227][23]);
        let eq22_e662_d_n24: f64 = (p.p341 * s.dn[227][24]);
        let eq22_e662_d_n25: f64 = (p.p341 * s.dn[227][25]);
        let eq22_e662_d_n26: f64 = (p.p341 * s.dn[227][26]);
        let eq22_e662_d_n27: f64 = (p.p341 * s.dn[227][27]);
        let eq22_e662_d_n28: f64 = (p.p341 * s.dn[227][28]);
        let eq22_e662_d_n29: f64 = (p.p341 * s.dn[227][29]);
        let eq22_e662_d_b0: f64 = (p.p341 * s.db[227][0]);
        let eq22_e662_d_b1: f64 = (p.p341 * s.db[227][1]);
        let eq22_e662_d_b2: f64 = (p.p341 * s.db[227][2]);
        let eq22_e662_d_b3: f64 = (p.p341 * s.db[227][3]);
        let eq22_e662_d_b4: f64 = (p.p341 * s.db[227][4]);
        let eq22_e662_d_b5: f64 = (p.p341 * s.db[227][5]);
        let eq22_e662_d_b6: f64 = (p.p341 * s.db[227][6]);
        let eq22_e662_d_b7: f64 = (p.p341 * s.db[227][7]);
        let eq22_e662_d_b8: f64 = (p.p341 * s.db[227][8]);
        let eq22_e662_d_b9: f64 = (p.p341 * s.db[227][9]);
        let eq22_e662_d_b10: f64 = (p.p341 * s.db[227][10]);
        let eq22_e662_d_b11: f64 = (p.p341 * s.db[227][11]);
        let eq22_e662_d_b12: f64 = (p.p341 * s.db[227][12]);
        let eq22_e662_d_b13: f64 = (p.p341 * s.db[227][13]);
        let eq22_e662_d_b14: f64 = (p.p341 * s.db[227][14]);
        let eq22_e662_d_b15: f64 = (p.p341 * s.db[227][15]);
        let eq22_e662_d_b16: f64 = (p.p341 * s.db[227][16]);
        let eq22_e662_d_b17: f64 = (p.p341 * s.db[227][17]);
        let eq22_e662_d_b18: f64 = (p.p341 * s.db[227][18]);
        let eq22_e662_d_b19: f64 = (p.p341 * s.db[227][19]);
        let eq22_e662_d_b20: f64 = (p.p341 * s.db[227][20]);
        let eq22_e662_d_b21: f64 = (p.p341 * s.db[227][21]);
        let eq22_e662_d_b22: f64 = (p.p341 * s.db[227][22]);
        let eq22_e662_d_b23: f64 = (p.p341 * s.db[227][23]);
        let eq22_e662_d_b24: f64 = (p.p341 * s.db[227][24]);
        let eq22_e662_d_b25: f64 = (p.p341 * s.db[227][25]);
        let eq22_e662_d_b26: f64 = (p.p341 * s.db[227][26]);
        let eq22_e662_d_b27: f64 = (p.p341 * s.db[227][27]);
        let eq22_e662_d_b28: f64 = (p.p341 * s.db[227][28]);
        let eq22_e662_d_b29: f64 = (p.p341 * s.db[227][29]);
        let eq22_e662_d_b30: f64 = (p.p341 * s.db[227][30]);
        let eq22_e662_d_b31: f64 = (p.p341 * s.db[227][31]);
        let eq22_e662_d_b32: f64 = (p.p341 * s.db[227][32]);
        let eq22_e662_d_b33: f64 = (p.p341 * s.db[227][33]);
        let eq22_e662_d_b34: f64 = (p.p341 * s.db[227][34]);
        let eq22_e662_d_b35: f64 = (p.p341 * s.db[227][35]);
        let eq22_e662_q: f64 = (p.p341 * eq22_e661_q);
        let eq22_e667: f64 = (s.v[111] - s.v[109]);
        let eq22_e668: f64 = (p.p343 * eq22_e667);
        let eq22_e668_d_n0: f64 = (p.p343 * s.dn[111][0]);
        let eq22_e668_d_n1: f64 = (p.p343 * s.dn[111][1]);
        let eq22_e668_d_n2: f64 = (p.p343 * s.dn[111][2]);
        let eq22_e668_d_n3: f64 = (p.p343 * s.dn[111][3]);
        let eq22_e668_d_n4: f64 = (p.p343 * s.dn[111][4]);
        let eq22_e668_d_n5: f64 = (p.p343 * s.dn[111][5]);
        let eq22_e668_d_n6: f64 = (p.p343 * s.dn[111][6]);
        let eq22_e668_d_n7: f64 = (p.p343 * s.dn[111][7]);
        let eq22_e668_d_n8: f64 = (p.p343 * s.dn[111][8]);
        let eq22_e668_d_n9: f64 = (p.p343 * s.dn[111][9]);
        let eq22_e668_d_n10: f64 = (p.p343 * s.dn[111][10]);
        let eq22_e668_d_n11: f64 = (p.p343 * s.dn[111][11]);
        let eq22_e668_d_n12: f64 = (p.p343 * s.dn[111][12]);
        let eq22_e668_d_n13: f64 = (p.p343 * s.dn[111][13]);
        let eq22_e668_d_n14: f64 = (p.p343 * s.dn[111][14]);
        let eq22_e668_d_n15: f64 = (p.p343 * s.dn[111][15]);
        let eq22_e668_d_n16: f64 = (p.p343 * s.dn[111][16]);
        let eq22_e668_d_n17: f64 = (p.p343 * s.dn[111][17]);
        let eq22_e668_d_n18: f64 = (p.p343 * s.dn[111][18]);
        let eq22_e668_d_n19: f64 = (p.p343 * s.dn[111][19]);
        let eq22_e668_d_n20: f64 = (p.p343 * s.dn[111][20]);
        let eq22_e668_d_n21: f64 = (p.p343 * s.dn[111][21]);
        let eq22_e668_d_n22: f64 = (p.p343 * s.dn[111][22]);
        let eq22_e668_d_n23: f64 = (p.p343 * s.dn[111][23]);
        let eq22_e668_d_n24: f64 = (p.p343 * s.dn[111][24]);
        let eq22_e668_d_n25: f64 = (p.p343 * s.dn[111][25]);
        let eq22_e668_d_n26: f64 = (p.p343 * s.dn[111][26]);
        let eq22_e668_d_n27: f64 = (p.p343 * s.dn[111][27]);
        let eq22_e668_d_n28: f64 = (p.p343 * s.dn[111][28]);
        let eq22_e668_d_n29: f64 = (p.p343 * s.dn[111][29]);
        let eq22_e668_d_b0: f64 = (p.p343 * s.db[111][0]);
        let eq22_e668_d_b1: f64 = (p.p343 * s.db[111][1]);
        let eq22_e668_d_b2: f64 = (p.p343 * s.db[111][2]);
        let eq22_e668_d_b3: f64 = (p.p343 * s.db[111][3]);
        let eq22_e668_d_b4: f64 = (p.p343 * s.db[111][4]);
        let eq22_e668_d_b5: f64 = (p.p343 * s.db[111][5]);
        let eq22_e668_d_b6: f64 = (p.p343 * s.db[111][6]);
        let eq22_e668_d_b7: f64 = (p.p343 * s.db[111][7]);
        let eq22_e668_d_b8: f64 = (p.p343 * s.db[111][8]);
        let eq22_e668_d_b9: f64 = (p.p343 * s.db[111][9]);
        let eq22_e668_d_b10: f64 = (p.p343 * s.db[111][10]);
        let eq22_e668_d_b11: f64 = (p.p343 * s.db[111][11]);
        let eq22_e668_d_b12: f64 = (p.p343 * s.db[111][12]);
        let eq22_e668_d_b13: f64 = (p.p343 * s.db[111][13]);
        let eq22_e668_d_b14: f64 = (p.p343 * s.db[111][14]);
        let eq22_e668_d_b15: f64 = (p.p343 * s.db[111][15]);
        let eq22_e668_d_b16: f64 = (p.p343 * s.db[111][16]);
        let eq22_e668_d_b17: f64 = (p.p343 * s.db[111][17]);
        let eq22_e668_d_b18: f64 = (p.p343 * s.db[111][18]);
        let eq22_e668_d_b19: f64 = (p.p343 * s.db[111][19]);
        let eq22_e668_d_b20: f64 = (p.p343 * s.db[111][20]);
        let eq22_e668_d_b21: f64 = (p.p343 * s.db[111][21]);
        let eq22_e668_d_b22: f64 = (p.p343 * s.db[111][22]);
        let eq22_e668_d_b23: f64 = (p.p343 * s.db[111][23]);
        let eq22_e668_d_b24: f64 = (p.p343 * s.db[111][24]);
        let eq22_e668_d_b25: f64 = (p.p343 * s.db[111][25]);
        let eq22_e668_d_b26: f64 = (p.p343 * s.db[111][26]);
        let eq22_e668_d_b27: f64 = (p.p343 * s.db[111][27]);
        let eq22_e668_d_b28: f64 = (p.p343 * s.db[111][28]);
        let eq22_e668_d_b29: f64 = (p.p343 * s.db[111][29]);
        let eq22_e668_d_b30: f64 = (p.p343 * s.db[111][30]);
        let eq22_e668_d_b31: f64 = (p.p343 * s.db[111][31]);
        let eq22_e668_d_b32: f64 = (p.p343 * s.db[111][32]);
        let eq22_e668_d_b33: f64 = (p.p343 * s.db[111][33]);
        let eq22_e668_d_b34: f64 = (p.p343 * s.db[111][34]);
        let eq22_e668_d_b35: f64 = (p.p343 * s.db[111][35]);
        let eq22_e669: f64 = (1.0 + eq22_e668);
        let eq22_e673: f64 = (s.v[111] - s.v[109]);
        let eq22_e674: f64 = (p.p345 * eq22_e673);
        let eq22_e674_d_n0: f64 = (p.p345 * s.dn[111][0]);
        let eq22_e674_d_n1: f64 = (p.p345 * s.dn[111][1]);
        let eq22_e674_d_n2: f64 = (p.p345 * s.dn[111][2]);
        let eq22_e674_d_n3: f64 = (p.p345 * s.dn[111][3]);
        let eq22_e674_d_n4: f64 = (p.p345 * s.dn[111][4]);
        let eq22_e674_d_n5: f64 = (p.p345 * s.dn[111][5]);
        let eq22_e674_d_n6: f64 = (p.p345 * s.dn[111][6]);
        let eq22_e674_d_n7: f64 = (p.p345 * s.dn[111][7]);
        let eq22_e674_d_n8: f64 = (p.p345 * s.dn[111][8]);
        let eq22_e674_d_n9: f64 = (p.p345 * s.dn[111][9]);
        let eq22_e674_d_n10: f64 = (p.p345 * s.dn[111][10]);
        let eq22_e674_d_n11: f64 = (p.p345 * s.dn[111][11]);
        let eq22_e674_d_n12: f64 = (p.p345 * s.dn[111][12]);
        let eq22_e674_d_n13: f64 = (p.p345 * s.dn[111][13]);
        let eq22_e674_d_n14: f64 = (p.p345 * s.dn[111][14]);
        let eq22_e674_d_n15: f64 = (p.p345 * s.dn[111][15]);
        let eq22_e674_d_n16: f64 = (p.p345 * s.dn[111][16]);
        let eq22_e674_d_n17: f64 = (p.p345 * s.dn[111][17]);
        let eq22_e674_d_n18: f64 = (p.p345 * s.dn[111][18]);
        let eq22_e674_d_n19: f64 = (p.p345 * s.dn[111][19]);
        let eq22_e674_d_n20: f64 = (p.p345 * s.dn[111][20]);
        let eq22_e674_d_n21: f64 = (p.p345 * s.dn[111][21]);
        let eq22_e674_d_n22: f64 = (p.p345 * s.dn[111][22]);
        let eq22_e674_d_n23: f64 = (p.p345 * s.dn[111][23]);
        let eq22_e674_d_n24: f64 = (p.p345 * s.dn[111][24]);
        let eq22_e674_d_n25: f64 = (p.p345 * s.dn[111][25]);
        let eq22_e674_d_n26: f64 = (p.p345 * s.dn[111][26]);
        let eq22_e674_d_n27: f64 = (p.p345 * s.dn[111][27]);
        let eq22_e674_d_n28: f64 = (p.p345 * s.dn[111][28]);
        let eq22_e674_d_n29: f64 = (p.p345 * s.dn[111][29]);
        let eq22_e674_d_b0: f64 = (p.p345 * s.db[111][0]);
        let eq22_e674_d_b1: f64 = (p.p345 * s.db[111][1]);
        let eq22_e674_d_b2: f64 = (p.p345 * s.db[111][2]);
        let eq22_e674_d_b3: f64 = (p.p345 * s.db[111][3]);
        let eq22_e674_d_b4: f64 = (p.p345 * s.db[111][4]);
        let eq22_e674_d_b5: f64 = (p.p345 * s.db[111][5]);
        let eq22_e674_d_b6: f64 = (p.p345 * s.db[111][6]);
        let eq22_e674_d_b7: f64 = (p.p345 * s.db[111][7]);
        let eq22_e674_d_b8: f64 = (p.p345 * s.db[111][8]);
        let eq22_e674_d_b9: f64 = (p.p345 * s.db[111][9]);
        let eq22_e674_d_b10: f64 = (p.p345 * s.db[111][10]);
        let eq22_e674_d_b11: f64 = (p.p345 * s.db[111][11]);
        let eq22_e674_d_b12: f64 = (p.p345 * s.db[111][12]);
        let eq22_e674_d_b13: f64 = (p.p345 * s.db[111][13]);
        let eq22_e674_d_b14: f64 = (p.p345 * s.db[111][14]);
        let eq22_e674_d_b15: f64 = (p.p345 * s.db[111][15]);
        let eq22_e674_d_b16: f64 = (p.p345 * s.db[111][16]);
        let eq22_e674_d_b17: f64 = (p.p345 * s.db[111][17]);
        let eq22_e674_d_b18: f64 = (p.p345 * s.db[111][18]);
        let eq22_e674_d_b19: f64 = (p.p345 * s.db[111][19]);
        let eq22_e674_d_b20: f64 = (p.p345 * s.db[111][20]);
        let eq22_e674_d_b21: f64 = (p.p345 * s.db[111][21]);
        let eq22_e674_d_b22: f64 = (p.p345 * s.db[111][22]);
        let eq22_e674_d_b23: f64 = (p.p345 * s.db[111][23]);
        let eq22_e674_d_b24: f64 = (p.p345 * s.db[111][24]);
        let eq22_e674_d_b25: f64 = (p.p345 * s.db[111][25]);
        let eq22_e674_d_b26: f64 = (p.p345 * s.db[111][26]);
        let eq22_e674_d_b27: f64 = (p.p345 * s.db[111][27]);
        let eq22_e674_d_b28: f64 = (p.p345 * s.db[111][28]);
        let eq22_e674_d_b29: f64 = (p.p345 * s.db[111][29]);
        let eq22_e674_d_b30: f64 = (p.p345 * s.db[111][30]);
        let eq22_e674_d_b31: f64 = (p.p345 * s.db[111][31]);
        let eq22_e674_d_b32: f64 = (p.p345 * s.db[111][32]);
        let eq22_e674_d_b33: f64 = (p.p345 * s.db[111][33]);
        let eq22_e674_d_b34: f64 = (p.p345 * s.db[111][34]);
        let eq22_e674_d_b35: f64 = (p.p345 * s.db[111][35]);
        let eq22_e677: f64 = (s.v[111] - s.v[109]);
        let eq22_e678: f64 = (eq22_e674 * eq22_e677);
        let eq22_e678_d_n0: f64 = ((eq22_e674_d_n0 * eq22_e677) + (eq22_e674 * s.dn[111][0]));
        let eq22_e678_d_n1: f64 = ((eq22_e674_d_n1 * eq22_e677) + (eq22_e674 * s.dn[111][1]));
        let eq22_e678_d_n2: f64 = ((eq22_e674_d_n2 * eq22_e677) + (eq22_e674 * s.dn[111][2]));
        let eq22_e678_d_n3: f64 = ((eq22_e674_d_n3 * eq22_e677) + (eq22_e674 * s.dn[111][3]));
        let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * s.dn[111][4]));
        let eq22_e678_d_n5: f64 = ((eq22_e674_d_n5 * eq22_e677) + (eq22_e674 * s.dn[111][5]));
        let eq22_e678_d_n6: f64 = ((eq22_e674_d_n6 * eq22_e677) + (eq22_e674 * s.dn[111][6]));
        let eq22_e678_d_n7: f64 = ((eq22_e674_d_n7 * eq22_e677) + (eq22_e674 * s.dn[111][7]));
        let eq22_e678_d_n8: f64 = ((eq22_e674_d_n8 * eq22_e677) + (eq22_e674 * s.dn[111][8]));
        let eq22_e678_d_n9: f64 = ((eq22_e674_d_n9 * eq22_e677) + (eq22_e674 * s.dn[111][9]));
        let eq22_e678_d_n10: f64 = ((eq22_e674_d_n10 * eq22_e677) + (eq22_e674 * s.dn[111][10]));
        let eq22_e678_d_n11: f64 = ((eq22_e674_d_n11 * eq22_e677) + (eq22_e674 * s.dn[111][11]));
        let eq22_e678_d_n12: f64 = ((eq22_e674_d_n12 * eq22_e677) + (eq22_e674 * s.dn[111][12]));
        let eq22_e678_d_n13: f64 = ((eq22_e674_d_n13 * eq22_e677) + (eq22_e674 * s.dn[111][13]));
        let eq22_e678_d_n14: f64 = ((eq22_e674_d_n14 * eq22_e677) + (eq22_e674 * s.dn[111][14]));
        let eq22_e678_d_n15: f64 = ((eq22_e674_d_n15 * eq22_e677) + (eq22_e674 * s.dn[111][15]));
        let eq22_e678_d_n16: f64 = ((eq22_e674_d_n16 * eq22_e677) + (eq22_e674 * s.dn[111][16]));
        let eq22_e678_d_n17: f64 = ((eq22_e674_d_n17 * eq22_e677) + (eq22_e674 * s.dn[111][17]));
        let eq22_e678_d_n18: f64 = ((eq22_e674_d_n18 * eq22_e677) + (eq22_e674 * s.dn[111][18]));
        let eq22_e678_d_n19: f64 = ((eq22_e674_d_n19 * eq22_e677) + (eq22_e674 * s.dn[111][19]));
        let eq22_e678_d_n20: f64 = ((eq22_e674_d_n20 * eq22_e677) + (eq22_e674 * s.dn[111][20]));
        let eq22_e678_d_n21: f64 = ((eq22_e674_d_n21 * eq22_e677) + (eq22_e674 * s.dn[111][21]));
        let eq22_e678_d_n22: f64 = ((eq22_e674_d_n22 * eq22_e677) + (eq22_e674 * s.dn[111][22]));
        let eq22_e678_d_n23: f64 = ((eq22_e674_d_n23 * eq22_e677) + (eq22_e674 * s.dn[111][23]));
        let eq22_e678_d_n24: f64 = ((eq22_e674_d_n24 * eq22_e677) + (eq22_e674 * s.dn[111][24]));
        let eq22_e678_d_n25: f64 = ((eq22_e674_d_n25 * eq22_e677) + (eq22_e674 * s.dn[111][25]));
        let eq22_e678_d_n26: f64 = ((eq22_e674_d_n26 * eq22_e677) + (eq22_e674 * s.dn[111][26]));
        let eq22_e678_d_n27: f64 = ((eq22_e674_d_n27 * eq22_e677) + (eq22_e674 * s.dn[111][27]));
        let eq22_e678_d_n28: f64 = ((eq22_e674_d_n28 * eq22_e677) + (eq22_e674 * s.dn[111][28]));
        let eq22_e678_d_n29: f64 = ((eq22_e674_d_n29 * eq22_e677) + (eq22_e674 * s.dn[111][29]));
        let eq22_e678_d_b0: f64 = ((eq22_e674_d_b0 * eq22_e677) + (eq22_e674 * s.db[111][0]));
        let eq22_e678_d_b1: f64 = ((eq22_e674_d_b1 * eq22_e677) + (eq22_e674 * s.db[111][1]));
        let eq22_e678_d_b2: f64 = ((eq22_e674_d_b2 * eq22_e677) + (eq22_e674 * s.db[111][2]));
        let eq22_e678_d_b3: f64 = ((eq22_e674_d_b3 * eq22_e677) + (eq22_e674 * s.db[111][3]));
        let eq22_e678_d_b4: f64 = ((eq22_e674_d_b4 * eq22_e677) + (eq22_e674 * s.db[111][4]));
        let eq22_e678_d_b5: f64 = ((eq22_e674_d_b5 * eq22_e677) + (eq22_e674 * s.db[111][5]));
        let eq22_e678_d_b6: f64 = ((eq22_e674_d_b6 * eq22_e677) + (eq22_e674 * s.db[111][6]));
        let eq22_e678_d_b7: f64 = ((eq22_e674_d_b7 * eq22_e677) + (eq22_e674 * s.db[111][7]));
        let eq22_e678_d_b8: f64 = ((eq22_e674_d_b8 * eq22_e677) + (eq22_e674 * s.db[111][8]));
        let eq22_e678_d_b9: f64 = ((eq22_e674_d_b9 * eq22_e677) + (eq22_e674 * s.db[111][9]));
        let eq22_e678_d_b10: f64 = ((eq22_e674_d_b10 * eq22_e677) + (eq22_e674 * s.db[111][10]));
        let eq22_e678_d_b11: f64 = ((eq22_e674_d_b11 * eq22_e677) + (eq22_e674 * s.db[111][11]));
        let eq22_e678_d_b12: f64 = ((eq22_e674_d_b12 * eq22_e677) + (eq22_e674 * s.db[111][12]));
        let eq22_e678_d_b13: f64 = ((eq22_e674_d_b13 * eq22_e677) + (eq22_e674 * s.db[111][13]));
        let eq22_e678_d_b14: f64 = ((eq22_e674_d_b14 * eq22_e677) + (eq22_e674 * s.db[111][14]));
        let eq22_e678_d_b15: f64 = ((eq22_e674_d_b15 * eq22_e677) + (eq22_e674 * s.db[111][15]));
        let eq22_e678_d_b16: f64 = ((eq22_e674_d_b16 * eq22_e677) + (eq22_e674 * s.db[111][16]));
        let eq22_e678_d_b17: f64 = ((eq22_e674_d_b17 * eq22_e677) + (eq22_e674 * s.db[111][17]));
        let eq22_e678_d_b18: f64 = ((eq22_e674_d_b18 * eq22_e677) + (eq22_e674 * s.db[111][18]));
        let eq22_e678_d_b19: f64 = ((eq22_e674_d_b19 * eq22_e677) + (eq22_e674 * s.db[111][19]));
        let eq22_e678_d_b20: f64 = ((eq22_e674_d_b20 * eq22_e677) + (eq22_e674 * s.db[111][20]));
        let eq22_e678_d_b21: f64 = ((eq22_e674_d_b21 * eq22_e677) + (eq22_e674 * s.db[111][21]));
        let eq22_e678_d_b22: f64 = ((eq22_e674_d_b22 * eq22_e677) + (eq22_e674 * s.db[111][22]));
        let eq22_e678_d_b23: f64 = ((eq22_e674_d_b23 * eq22_e677) + (eq22_e674 * s.db[111][23]));
        let eq22_e678_d_b24: f64 = ((eq22_e674_d_b24 * eq22_e677) + (eq22_e674 * s.db[111][24]));
        let eq22_e678_d_b25: f64 = ((eq22_e674_d_b25 * eq22_e677) + (eq22_e674 * s.db[111][25]));
        let eq22_e678_d_b26: f64 = ((eq22_e674_d_b26 * eq22_e677) + (eq22_e674 * s.db[111][26]));
        let eq22_e678_d_b27: f64 = ((eq22_e674_d_b27 * eq22_e677) + (eq22_e674 * s.db[111][27]));
        let eq22_e678_d_b28: f64 = ((eq22_e674_d_b28 * eq22_e677) + (eq22_e674 * s.db[111][28]));
        let eq22_e678_d_b29: f64 = ((eq22_e674_d_b29 * eq22_e677) + (eq22_e674 * s.db[111][29]));
        let eq22_e678_d_b30: f64 = ((eq22_e674_d_b30 * eq22_e677) + (eq22_e674 * s.db[111][30]));
        let eq22_e678_d_b31: f64 = ((eq22_e674_d_b31 * eq22_e677) + (eq22_e674 * s.db[111][31]));
        let eq22_e678_d_b32: f64 = ((eq22_e674_d_b32 * eq22_e677) + (eq22_e674 * s.db[111][32]));
        let eq22_e678_d_b33: f64 = ((eq22_e674_d_b33 * eq22_e677) + (eq22_e674 * s.db[111][33]));
        let eq22_e678_d_b34: f64 = ((eq22_e674_d_b34 * eq22_e677) + (eq22_e674 * s.db[111][34]));
        let eq22_e678_d_b35: f64 = ((eq22_e674_d_b35 * eq22_e677) + (eq22_e674 * s.db[111][35]));
        let eq22_e679: f64 = (eq22_e669 + eq22_e678);
        let eq22_e679_d_n0: f64 = (eq22_e668_d_n0 + eq22_e678_d_n0);
        let eq22_e679_d_n1: f64 = (eq22_e668_d_n1 + eq22_e678_d_n1);
        let eq22_e679_d_n2: f64 = (eq22_e668_d_n2 + eq22_e678_d_n2);
        let eq22_e679_d_n3: f64 = (eq22_e668_d_n3 + eq22_e678_d_n3);
        let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);
        let eq22_e679_d_n5: f64 = (eq22_e668_d_n5 + eq22_e678_d_n5);
        let eq22_e679_d_n6: f64 = (eq22_e668_d_n6 + eq22_e678_d_n6);
        let eq22_e679_d_n7: f64 = (eq22_e668_d_n7 + eq22_e678_d_n7);
        let eq22_e679_d_n8: f64 = (eq22_e668_d_n8 + eq22_e678_d_n8);
        let eq22_e679_d_n9: f64 = (eq22_e668_d_n9 + eq22_e678_d_n9);
        let eq22_e679_d_n10: f64 = (eq22_e668_d_n10 + eq22_e678_d_n10);
        let eq22_e679_d_n11: f64 = (eq22_e668_d_n11 + eq22_e678_d_n11);
        let eq22_e679_d_n12: f64 = (eq22_e668_d_n12 + eq22_e678_d_n12);
        let eq22_e679_d_n13: f64 = (eq22_e668_d_n13 + eq22_e678_d_n13);
        let eq22_e679_d_n14: f64 = (eq22_e668_d_n14 + eq22_e678_d_n14);
        let eq22_e679_d_n15: f64 = (eq22_e668_d_n15 + eq22_e678_d_n15);
        let eq22_e679_d_n16: f64 = (eq22_e668_d_n16 + eq22_e678_d_n16);
        let eq22_e679_d_n17: f64 = (eq22_e668_d_n17 + eq22_e678_d_n17);
        let eq22_e679_d_n18: f64 = (eq22_e668_d_n18 + eq22_e678_d_n18);
        let eq22_e679_d_n19: f64 = (eq22_e668_d_n19 + eq22_e678_d_n19);
        let eq22_e679_d_n20: f64 = (eq22_e668_d_n20 + eq22_e678_d_n20);
        let eq22_e679_d_n21: f64 = (eq22_e668_d_n21 + eq22_e678_d_n21);
        let eq22_e679_d_n22: f64 = (eq22_e668_d_n22 + eq22_e678_d_n22);
        let eq22_e679_d_n23: f64 = (eq22_e668_d_n23 + eq22_e678_d_n23);
        let eq22_e679_d_n24: f64 = (eq22_e668_d_n24 + eq22_e678_d_n24);
        let eq22_e679_d_n25: f64 = (eq22_e668_d_n25 + eq22_e678_d_n25);
        let eq22_e679_d_n26: f64 = (eq22_e668_d_n26 + eq22_e678_d_n26);
        let eq22_e679_d_n27: f64 = (eq22_e668_d_n27 + eq22_e678_d_n27);
        let eq22_e679_d_n28: f64 = (eq22_e668_d_n28 + eq22_e678_d_n28);
        let eq22_e679_d_n29: f64 = (eq22_e668_d_n29 + eq22_e678_d_n29);
        let eq22_e679_d_b0: f64 = (eq22_e668_d_b0 + eq22_e678_d_b0);
        let eq22_e679_d_b1: f64 = (eq22_e668_d_b1 + eq22_e678_d_b1);
        let eq22_e679_d_b2: f64 = (eq22_e668_d_b2 + eq22_e678_d_b2);
        let eq22_e679_d_b3: f64 = (eq22_e668_d_b3 + eq22_e678_d_b3);
        let eq22_e679_d_b4: f64 = (eq22_e668_d_b4 + eq22_e678_d_b4);
        let eq22_e679_d_b5: f64 = (eq22_e668_d_b5 + eq22_e678_d_b5);
        let eq22_e679_d_b6: f64 = (eq22_e668_d_b6 + eq22_e678_d_b6);
        let eq22_e679_d_b7: f64 = (eq22_e668_d_b7 + eq22_e678_d_b7);
        let eq22_e679_d_b8: f64 = (eq22_e668_d_b8 + eq22_e678_d_b8);
        let eq22_e679_d_b9: f64 = (eq22_e668_d_b9 + eq22_e678_d_b9);
        let eq22_e679_d_b10: f64 = (eq22_e668_d_b10 + eq22_e678_d_b10);
        let eq22_e679_d_b11: f64 = (eq22_e668_d_b11 + eq22_e678_d_b11);
        let eq22_e679_d_b12: f64 = (eq22_e668_d_b12 + eq22_e678_d_b12);
        let eq22_e679_d_b13: f64 = (eq22_e668_d_b13 + eq22_e678_d_b13);
        let eq22_e679_d_b14: f64 = (eq22_e668_d_b14 + eq22_e678_d_b14);
        let eq22_e679_d_b15: f64 = (eq22_e668_d_b15 + eq22_e678_d_b15);
        let eq22_e679_d_b16: f64 = (eq22_e668_d_b16 + eq22_e678_d_b16);
        let eq22_e679_d_b17: f64 = (eq22_e668_d_b17 + eq22_e678_d_b17);
        let eq22_e679_d_b18: f64 = (eq22_e668_d_b18 + eq22_e678_d_b18);
        let eq22_e679_d_b19: f64 = (eq22_e668_d_b19 + eq22_e678_d_b19);
        let eq22_e679_d_b20: f64 = (eq22_e668_d_b20 + eq22_e678_d_b20);
        let eq22_e679_d_b21: f64 = (eq22_e668_d_b21 + eq22_e678_d_b21);
        let eq22_e679_d_b22: f64 = (eq22_e668_d_b22 + eq22_e678_d_b22);
        let eq22_e679_d_b23: f64 = (eq22_e668_d_b23 + eq22_e678_d_b23);
        let eq22_e679_d_b24: f64 = (eq22_e668_d_b24 + eq22_e678_d_b24);
        let eq22_e679_d_b25: f64 = (eq22_e668_d_b25 + eq22_e678_d_b25);
        let eq22_e679_d_b26: f64 = (eq22_e668_d_b26 + eq22_e678_d_b26);
        let eq22_e679_d_b27: f64 = (eq22_e668_d_b27 + eq22_e678_d_b27);
        let eq22_e679_d_b28: f64 = (eq22_e668_d_b28 + eq22_e678_d_b28);
        let eq22_e679_d_b29: f64 = (eq22_e668_d_b29 + eq22_e678_d_b29);
        let eq22_e679_d_b30: f64 = (eq22_e668_d_b30 + eq22_e678_d_b30);
        let eq22_e679_d_b31: f64 = (eq22_e668_d_b31 + eq22_e678_d_b31);
        let eq22_e679_d_b32: f64 = (eq22_e668_d_b32 + eq22_e678_d_b32);
        let eq22_e679_d_b33: f64 = (eq22_e668_d_b33 + eq22_e678_d_b33);
        let eq22_e679_d_b34: f64 = (eq22_e668_d_b34 + eq22_e678_d_b34);
        let eq22_e679_d_b35: f64 = (eq22_e668_d_b35 + eq22_e678_d_b35);
        let eq22_e680: f64 = (eq22_e662 * eq22_e679);
        let eq22_e680_d_n0: f64 = ((eq22_e662_d_n0 * eq22_e679) + (eq22_e662 * eq22_e679_d_n0));
        let eq22_e680_d_n1: f64 = ((eq22_e662_d_n1 * eq22_e679) + (eq22_e662 * eq22_e679_d_n1));
        let eq22_e680_d_n2: f64 = ((eq22_e662_d_n2 * eq22_e679) + (eq22_e662 * eq22_e679_d_n2));
        let eq22_e680_d_n3: f64 = ((eq22_e662_d_n3 * eq22_e679) + (eq22_e662 * eq22_e679_d_n3));
        let eq22_e680_d_n4: f64 = ((eq22_e662_d_n4 * eq22_e679) + (eq22_e662 * eq22_e679_d_n4));
        let eq22_e680_d_n5: f64 = ((eq22_e662_d_n5 * eq22_e679) + (eq22_e662 * eq22_e679_d_n5));
        let eq22_e680_d_n6: f64 = ((eq22_e662_d_n6 * eq22_e679) + (eq22_e662 * eq22_e679_d_n6));
        let eq22_e680_d_n7: f64 = ((eq22_e662_d_n7 * eq22_e679) + (eq22_e662 * eq22_e679_d_n7));
        let eq22_e680_d_n8: f64 = ((eq22_e662_d_n8 * eq22_e679) + (eq22_e662 * eq22_e679_d_n8));
        let eq22_e680_d_n9: f64 = ((eq22_e662_d_n9 * eq22_e679) + (eq22_e662 * eq22_e679_d_n9));
        let eq22_e680_d_n10: f64 = ((eq22_e662_d_n10 * eq22_e679) + (eq22_e662 * eq22_e679_d_n10));
        let eq22_e680_d_n11: f64 = ((eq22_e662_d_n11 * eq22_e679) + (eq22_e662 * eq22_e679_d_n11));
        let eq22_e680_d_n12: f64 = ((eq22_e662_d_n12 * eq22_e679) + (eq22_e662 * eq22_e679_d_n12));
        let eq22_e680_d_n13: f64 = ((eq22_e662_d_n13 * eq22_e679) + (eq22_e662 * eq22_e679_d_n13));
        let eq22_e680_d_n14: f64 = ((eq22_e662_d_n14 * eq22_e679) + (eq22_e662 * eq22_e679_d_n14));
        let eq22_e680_d_n15: f64 = ((eq22_e662_d_n15 * eq22_e679) + (eq22_e662 * eq22_e679_d_n15));
        let eq22_e680_d_n16: f64 = ((eq22_e662_d_n16 * eq22_e679) + (eq22_e662 * eq22_e679_d_n16));
        let eq22_e680_d_n17: f64 = ((eq22_e662_d_n17 * eq22_e679) + (eq22_e662 * eq22_e679_d_n17));
        let eq22_e680_d_n18: f64 = ((eq22_e662_d_n18 * eq22_e679) + (eq22_e662 * eq22_e679_d_n18));
        let eq22_e680_d_n19: f64 = ((eq22_e662_d_n19 * eq22_e679) + (eq22_e662 * eq22_e679_d_n19));
        let eq22_e680_d_n20: f64 = ((eq22_e662_d_n20 * eq22_e679) + (eq22_e662 * eq22_e679_d_n20));
        let eq22_e680_d_n21: f64 = ((eq22_e662_d_n21 * eq22_e679) + (eq22_e662 * eq22_e679_d_n21));
        let eq22_e680_d_n22: f64 = ((eq22_e662_d_n22 * eq22_e679) + (eq22_e662 * eq22_e679_d_n22));
        let eq22_e680_d_n23: f64 = ((eq22_e662_d_n23 * eq22_e679) + (eq22_e662 * eq22_e679_d_n23));
        let eq22_e680_d_n24: f64 = ((eq22_e662_d_n24 * eq22_e679) + (eq22_e662 * eq22_e679_d_n24));
        let eq22_e680_d_n25: f64 = ((eq22_e662_d_n25 * eq22_e679) + (eq22_e662 * eq22_e679_d_n25));
        let eq22_e680_d_n26: f64 = ((eq22_e662_d_n26 * eq22_e679) + (eq22_e662 * eq22_e679_d_n26));
        let eq22_e680_d_n27: f64 = ((eq22_e662_d_n27 * eq22_e679) + (eq22_e662 * eq22_e679_d_n27));
        let eq22_e680_d_n28: f64 = ((eq22_e662_d_n28 * eq22_e679) + (eq22_e662 * eq22_e679_d_n28));
        let eq22_e680_d_n29: f64 = ((eq22_e662_d_n29 * eq22_e679) + (eq22_e662 * eq22_e679_d_n29));
        let eq22_e680_d_b0: f64 = ((eq22_e662_d_b0 * eq22_e679) + (eq22_e662 * eq22_e679_d_b0));
        let eq22_e680_d_b1: f64 = ((eq22_e662_d_b1 * eq22_e679) + (eq22_e662 * eq22_e679_d_b1));
        let eq22_e680_d_b2: f64 = ((eq22_e662_d_b2 * eq22_e679) + (eq22_e662 * eq22_e679_d_b2));
        let eq22_e680_d_b3: f64 = ((eq22_e662_d_b3 * eq22_e679) + (eq22_e662 * eq22_e679_d_b3));
        let eq22_e680_d_b4: f64 = ((eq22_e662_d_b4 * eq22_e679) + (eq22_e662 * eq22_e679_d_b4));
        let eq22_e680_d_b5: f64 = ((eq22_e662_d_b5 * eq22_e679) + (eq22_e662 * eq22_e679_d_b5));
        let eq22_e680_d_b6: f64 = ((eq22_e662_d_b6 * eq22_e679) + (eq22_e662 * eq22_e679_d_b6));
        let eq22_e680_d_b7: f64 = ((eq22_e662_d_b7 * eq22_e679) + (eq22_e662 * eq22_e679_d_b7));
        let eq22_e680_d_b8: f64 = ((eq22_e662_d_b8 * eq22_e679) + (eq22_e662 * eq22_e679_d_b8));
        let eq22_e680_d_b9: f64 = ((eq22_e662_d_b9 * eq22_e679) + (eq22_e662 * eq22_e679_d_b9));
        let eq22_e680_d_b10: f64 = ((eq22_e662_d_b10 * eq22_e679) + (eq22_e662 * eq22_e679_d_b10));
        let eq22_e680_d_b11: f64 = ((eq22_e662_d_b11 * eq22_e679) + (eq22_e662 * eq22_e679_d_b11));
        let eq22_e680_d_b12: f64 = ((eq22_e662_d_b12 * eq22_e679) + (eq22_e662 * eq22_e679_d_b12));
        let eq22_e680_d_b13: f64 = ((eq22_e662_d_b13 * eq22_e679) + (eq22_e662 * eq22_e679_d_b13));
        let eq22_e680_d_b14: f64 = ((eq22_e662_d_b14 * eq22_e679) + (eq22_e662 * eq22_e679_d_b14));
        let eq22_e680_d_b15: f64 = ((eq22_e662_d_b15 * eq22_e679) + (eq22_e662 * eq22_e679_d_b15));
        let eq22_e680_d_b16: f64 = ((eq22_e662_d_b16 * eq22_e679) + (eq22_e662 * eq22_e679_d_b16));
        let eq22_e680_d_b17: f64 = ((eq22_e662_d_b17 * eq22_e679) + (eq22_e662 * eq22_e679_d_b17));
        let eq22_e680_d_b18: f64 = ((eq22_e662_d_b18 * eq22_e679) + (eq22_e662 * eq22_e679_d_b18));
        let eq22_e680_d_b19: f64 = ((eq22_e662_d_b19 * eq22_e679) + (eq22_e662 * eq22_e679_d_b19));
        let eq22_e680_d_b20: f64 = ((eq22_e662_d_b20 * eq22_e679) + (eq22_e662 * eq22_e679_d_b20));
        let eq22_e680_d_b21: f64 = ((eq22_e662_d_b21 * eq22_e679) + (eq22_e662 * eq22_e679_d_b21));
        let eq22_e680_d_b22: f64 = ((eq22_e662_d_b22 * eq22_e679) + (eq22_e662 * eq22_e679_d_b22));
        let eq22_e680_d_b23: f64 = ((eq22_e662_d_b23 * eq22_e679) + (eq22_e662 * eq22_e679_d_b23));
        let eq22_e680_d_b24: f64 = ((eq22_e662_d_b24 * eq22_e679) + (eq22_e662 * eq22_e679_d_b24));
        let eq22_e680_d_b25: f64 = ((eq22_e662_d_b25 * eq22_e679) + (eq22_e662 * eq22_e679_d_b25));
        let eq22_e680_d_b26: f64 = ((eq22_e662_d_b26 * eq22_e679) + (eq22_e662 * eq22_e679_d_b26));
        let eq22_e680_d_b27: f64 = ((eq22_e662_d_b27 * eq22_e679) + (eq22_e662 * eq22_e679_d_b27));
        let eq22_e680_d_b28: f64 = ((eq22_e662_d_b28 * eq22_e679) + (eq22_e662 * eq22_e679_d_b28));
        let eq22_e680_d_b29: f64 = ((eq22_e662_d_b29 * eq22_e679) + (eq22_e662 * eq22_e679_d_b29));
        let eq22_e680_d_b30: f64 = ((eq22_e662_d_b30 * eq22_e679) + (eq22_e662 * eq22_e679_d_b30));
        let eq22_e680_d_b31: f64 = ((eq22_e662_d_b31 * eq22_e679) + (eq22_e662 * eq22_e679_d_b31));
        let eq22_e680_d_b32: f64 = ((eq22_e662_d_b32 * eq22_e679) + (eq22_e662 * eq22_e679_d_b32));
        let eq22_e680_d_b33: f64 = ((eq22_e662_d_b33 * eq22_e679) + (eq22_e662 * eq22_e679_d_b33));
        let eq22_e680_d_b34: f64 = ((eq22_e662_d_b34 * eq22_e679) + (eq22_e662 * eq22_e679_d_b34));
        let eq22_e680_d_b35: f64 = ((eq22_e662_d_b35 * eq22_e679) + (eq22_e662 * eq22_e679_d_b35));
        let eq22_e680_q: f64 = (eq22_e662_q * eq22_e679);
        let eq22_e680_q_d_n0: f64 = ((eq22_e662_d_n0 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n0));
        let eq22_e680_q_d_n1: f64 = ((eq22_e662_d_n1 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n1));
        let eq22_e680_q_d_n2: f64 = ((eq22_e662_d_n2 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n2));
        let eq22_e680_q_d_n3: f64 = ((eq22_e662_d_n3 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n3));
        let eq22_e680_q_d_n4: f64 = ((eq22_e662_d_n4 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n4));
        let eq22_e680_q_d_n5: f64 = ((eq22_e662_d_n5 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n5));
        let eq22_e680_q_d_n6: f64 = ((eq22_e662_d_n6 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n6));
        let eq22_e680_q_d_n7: f64 = ((eq22_e662_d_n7 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n7));
        let eq22_e680_q_d_n8: f64 = ((eq22_e662_d_n8 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n8));
        let eq22_e680_q_d_n9: f64 = ((eq22_e662_d_n9 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n9));
        let eq22_e680_q_d_n10: f64 = ((eq22_e662_d_n10 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n10));
        let eq22_e680_q_d_n11: f64 = ((eq22_e662_d_n11 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n11));
        let eq22_e680_q_d_n12: f64 = ((eq22_e662_d_n12 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n12));
        let eq22_e680_q_d_n13: f64 = ((eq22_e662_d_n13 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n13));
        let eq22_e680_q_d_n14: f64 = ((eq22_e662_d_n14 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n14));
        let eq22_e680_q_d_n15: f64 = ((eq22_e662_d_n15 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n15));
        let eq22_e680_q_d_n16: f64 = ((eq22_e662_d_n16 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n16));
        let eq22_e680_q_d_n17: f64 = ((eq22_e662_d_n17 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n17));
        let eq22_e680_q_d_n18: f64 = ((eq22_e662_d_n18 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n18));
        let eq22_e680_q_d_n19: f64 = ((eq22_e662_d_n19 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n19));
        let eq22_e680_q_d_n20: f64 = ((eq22_e662_d_n20 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n20));
        let eq22_e680_q_d_n21: f64 = ((eq22_e662_d_n21 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n21));
        let eq22_e680_q_d_n22: f64 = ((eq22_e662_d_n22 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n22));
        let eq22_e680_q_d_n23: f64 = ((eq22_e662_d_n23 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n23));
        let eq22_e680_q_d_n24: f64 = ((eq22_e662_d_n24 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n24));
        let eq22_e680_q_d_n25: f64 = ((eq22_e662_d_n25 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n25));
        let eq22_e680_q_d_n26: f64 = ((eq22_e662_d_n26 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n26));
        let eq22_e680_q_d_n27: f64 = ((eq22_e662_d_n27 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n27));
        let eq22_e680_q_d_n28: f64 = ((eq22_e662_d_n28 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n28));
        let eq22_e680_q_d_n29: f64 = ((eq22_e662_d_n29 * eq22_e679) + (eq22_e662_q * eq22_e679_d_n29));
        let eq22_e680_q_d_b0: f64 = ((eq22_e662_d_b0 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b0));
        let eq22_e680_q_d_b1: f64 = ((eq22_e662_d_b1 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b1));
        let eq22_e680_q_d_b2: f64 = ((eq22_e662_d_b2 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b2));
        let eq22_e680_q_d_b3: f64 = ((eq22_e662_d_b3 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b3));
        let eq22_e680_q_d_b4: f64 = ((eq22_e662_d_b4 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b4));
        let eq22_e680_q_d_b5: f64 = ((eq22_e662_d_b5 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b5));
        let eq22_e680_q_d_b6: f64 = ((eq22_e662_d_b6 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b6));
        let eq22_e680_q_d_b7: f64 = ((eq22_e662_d_b7 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b7));
        let eq22_e680_q_d_b8: f64 = ((eq22_e662_d_b8 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b8));
        let eq22_e680_q_d_b9: f64 = ((eq22_e662_d_b9 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b9));
        let eq22_e680_q_d_b10: f64 = ((eq22_e662_d_b10 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b10));
        let eq22_e680_q_d_b11: f64 = ((eq22_e662_d_b11 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b11));
        let eq22_e680_q_d_b12: f64 = ((eq22_e662_d_b12 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b12));
        let eq22_e680_q_d_b13: f64 = ((eq22_e662_d_b13 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b13));
        let eq22_e680_q_d_b14: f64 = ((eq22_e662_d_b14 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b14));
        let eq22_e680_q_d_b15: f64 = ((eq22_e662_d_b15 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b15));
        let eq22_e680_q_d_b16: f64 = ((eq22_e662_d_b16 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b16));
        let eq22_e680_q_d_b17: f64 = ((eq22_e662_d_b17 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b17));
        let eq22_e680_q_d_b18: f64 = ((eq22_e662_d_b18 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b18));
        let eq22_e680_q_d_b19: f64 = ((eq22_e662_d_b19 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b19));
        let eq22_e680_q_d_b20: f64 = ((eq22_e662_d_b20 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b20));
        let eq22_e680_q_d_b21: f64 = ((eq22_e662_d_b21 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b21));
        let eq22_e680_q_d_b22: f64 = ((eq22_e662_d_b22 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b22));
        let eq22_e680_q_d_b23: f64 = ((eq22_e662_d_b23 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b23));
        let eq22_e680_q_d_b24: f64 = ((eq22_e662_d_b24 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b24));
        let eq22_e680_q_d_b25: f64 = ((eq22_e662_d_b25 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b25));
        let eq22_e680_q_d_b26: f64 = ((eq22_e662_d_b26 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b26));
        let eq22_e680_q_d_b27: f64 = ((eq22_e662_d_b27 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b27));
        let eq22_e680_q_d_b28: f64 = ((eq22_e662_d_b28 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b28));
        let eq22_e680_q_d_b29: f64 = ((eq22_e662_d_b29 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b29));
        let eq22_e680_q_d_b30: f64 = ((eq22_e662_d_b30 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b30));
        let eq22_e680_q_d_b31: f64 = ((eq22_e662_d_b31 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b31));
        let eq22_e680_q_d_b32: f64 = ((eq22_e662_d_b32 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b32));
        let eq22_e680_q_d_b33: f64 = ((eq22_e662_d_b33 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b33));
        let eq22_e680_q_d_b34: f64 = ((eq22_e662_d_b34 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b34));
        let eq22_e680_q_d_b35: f64 = ((eq22_e662_d_b35 * eq22_e679) + (eq22_e662_q * eq22_e679_d_b35));
        (eq22_e680, eq22_e680_d_n0, eq22_e680_d_n1, eq22_e680_d_n2, eq22_e680_d_n3, eq22_e680_d_n4, eq22_e680_d_n5, eq22_e680_d_n6, eq22_e680_d_n7, eq22_e680_d_n8, eq22_e680_d_n9, eq22_e680_d_n10, eq22_e680_d_n11, eq22_e680_d_n12, eq22_e680_d_n13, eq22_e680_d_n14, eq22_e680_d_n15, eq22_e680_d_n16, eq22_e680_d_n17, eq22_e680_d_n18, eq22_e680_d_n19, eq22_e680_d_n20, eq22_e680_d_n21, eq22_e680_d_n22, eq22_e680_d_n23, eq22_e680_d_n24, eq22_e680_d_n25, eq22_e680_d_n26, eq22_e680_d_n27, eq22_e680_d_n28, eq22_e680_d_n29, eq22_e680_d_b0, eq22_e680_d_b1, eq22_e680_d_b2, eq22_e680_d_b3, eq22_e680_d_b4, eq22_e680_d_b5, eq22_e680_d_b6, eq22_e680_d_b7, eq22_e680_d_b8, eq22_e680_d_b9, eq22_e680_d_b10, eq22_e680_d_b11, eq22_e680_d_b12, eq22_e680_d_b13, eq22_e680_d_b14, eq22_e680_d_b15, eq22_e680_d_b16, eq22_e680_d_b17, eq22_e680_d_b18, eq22_e680_d_b19, eq22_e680_d_b20, eq22_e680_d_b21, eq22_e680_d_b22, eq22_e680_d_b23, eq22_e680_d_b24, eq22_e680_d_b25, eq22_e680_d_b26, eq22_e680_d_b27, eq22_e680_d_b28, eq22_e680_d_b29, eq22_e680_d_b30, eq22_e680_d_b31, eq22_e680_d_b32, eq22_e680_d_b33, eq22_e680_d_b34, eq22_e680_d_b35, eq22_e680_q, eq22_e680_q_d_n0, eq22_e680_q_d_n1, eq22_e680_q_d_n2, eq22_e680_q_d_n3, eq22_e680_q_d_n4, eq22_e680_q_d_n5, eq22_e680_q_d_n6, eq22_e680_q_d_n7, eq22_e680_q_d_n8, eq22_e680_q_d_n9, eq22_e680_q_d_n10, eq22_e680_q_d_n11, eq22_e680_q_d_n12, eq22_e680_q_d_n13, eq22_e680_q_d_n14, eq22_e680_q_d_n15, eq22_e680_q_d_n16, eq22_e680_q_d_n17, eq22_e680_q_d_n18, eq22_e680_q_d_n19, eq22_e680_q_d_n20, eq22_e680_q_d_n21, eq22_e680_q_d_n22, eq22_e680_q_d_n23, eq22_e680_q_d_n24, eq22_e680_q_d_n25, eq22_e680_q_d_n26, eq22_e680_q_d_n27, eq22_e680_q_d_n28, eq22_e680_q_d_n29, eq22_e680_q_d_b0, eq22_e680_q_d_b1, eq22_e680_q_d_b2, eq22_e680_q_d_b3, eq22_e680_q_d_b4, eq22_e680_q_d_b5, eq22_e680_q_d_b6, eq22_e680_q_d_b7, eq22_e680_q_d_b8, eq22_e680_q_d_b9, eq22_e680_q_d_b10, eq22_e680_q_d_b11, eq22_e680_q_d_b12, eq22_e680_q_d_b13, eq22_e680_q_d_b14, eq22_e680_q_d_b15, eq22_e680_q_d_b16, eq22_e680_q_d_b17, eq22_e680_q_d_b18, eq22_e680_q_d_b19, eq22_e680_q_d_b20, eq22_e680_q_d_b21, eq22_e680_q_d_b22, eq22_e680_q_d_b23, eq22_e680_q_d_b24, eq22_e680_q_d_b25, eq22_e680_q_d_b26, eq22_e680_q_d_b27, eq22_e680_q_d_b28, eq22_e680_q_d_b29, eq22_e680_q_d_b30, eq22_e680_q_d_b31, eq22_e680_q_d_b32, eq22_e680_q_d_b33, eq22_e680_q_d_b34, eq22_e680_q_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_reactive_node_derivatives: [f64; 30] = [eq22_e682_q_d_n0, eq22_e682_q_d_n1, eq22_e682_q_d_n2, eq22_e682_q_d_n3, eq22_e682_q_d_n4, eq22_e682_q_d_n5, eq22_e682_q_d_n6, eq22_e682_q_d_n7, eq22_e682_q_d_n8, eq22_e682_q_d_n9, eq22_e682_q_d_n10, eq22_e682_q_d_n11, eq22_e682_q_d_n12, eq22_e682_q_d_n13, eq22_e682_q_d_n14, eq22_e682_q_d_n15, eq22_e682_q_d_n16, eq22_e682_q_d_n17, eq22_e682_q_d_n18, eq22_e682_q_d_n19, eq22_e682_q_d_n20, eq22_e682_q_d_n21, eq22_e682_q_d_n22, eq22_e682_q_d_n23, eq22_e682_q_d_n24, eq22_e682_q_d_n25, eq22_e682_q_d_n26, eq22_e682_q_d_n27, eq22_e682_q_d_n28, eq22_e682_q_d_n29];
        let eq22_reactive_branch_derivatives: [f64; 36] = [eq22_e682_q_d_b0, eq22_e682_q_d_b1, eq22_e682_q_d_b2, eq22_e682_q_d_b3, eq22_e682_q_d_b4, eq22_e682_q_d_b5, eq22_e682_q_d_b6, eq22_e682_q_d_b7, eq22_e682_q_d_b8, eq22_e682_q_d_b9, eq22_e682_q_d_b10, eq22_e682_q_d_b11, eq22_e682_q_d_b12, eq22_e682_q_d_b13, eq22_e682_q_d_b14, eq22_e682_q_d_b15, eq22_e682_q_d_b16, eq22_e682_q_d_b17, eq22_e682_q_d_b18, eq22_e682_q_d_b19, eq22_e682_q_d_b20, eq22_e682_q_d_b21, eq22_e682_q_d_b22, eq22_e682_q_d_b23, eq22_e682_q_d_b24, eq22_e682_q_d_b25, eq22_e682_q_d_b26, eq22_e682_q_d_b27, eq22_e682_q_d_b28, eq22_e682_q_d_b29, eq22_e682_q_d_b30, eq22_e682_q_d_b31, eq22_e682_q_d_b32, eq22_e682_q_d_b33, eq22_e682_q_d_b34, eq22_e682_q_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[26]),
            None,
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let __rspice_deriv_cse_0: f64 = (s.dn[209][16] + (-p.p355));
        let __rspice_deriv_cse_1: f64 = (s.dn[210][17] + (-p.p355));
        let __rspice_deriv_cse_2: f64 = (s.dn[211][16] + (-p.p355));
        let __rspice_deriv_cse_3: f64 = (s.dn[203][15] + (-p.p355));
        let __rspice_deriv_cse_4: f64 = (s.dn[204][16] + (-p.p355));
        let __rspice_deriv_cse_5: f64 = (s.dn[205][15] + (-p.p355));
        let __rspice_deriv_cse_6: f64 = (s.dn[197][14] + (-p.p355));
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29, eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35, eq33_e769_q,) = {
    if s.b[466] {
        let eq33_e762_q: f64 = s.v[209];
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e766_q: f64 = eq33_e765;
        let eq33_e767: f64 = (s.v[209] + eq33_e765);
        let eq33_e767_d_n7: f64 = (s.dn[209][7] + p.p355);
        let eq33_e767_q: f64 = (eq33_e762_q + eq33_e766_q);
        (eq33_e767, s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], eq33_e767_d_n7, s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], __rspice_deriv_cse_0, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29], s.db[209][0], s.db[209][1], s.db[209][2], s.db[209][3], s.db[209][4], s.db[209][5], s.db[209][6], s.db[209][7], s.db[209][8], s.db[209][9], s.db[209][10], s.db[209][11], s.db[209][12], s.db[209][13], s.db[209][14], s.db[209][15], s.db[209][16], s.db[209][17], s.db[209][18], s.db[209][19], s.db[209][20], s.db[209][21], s.db[209][22], s.db[209][23], s.db[209][24], s.db[209][25], s.db[209][26], s.db[209][27], s.db[209][28], s.db[209][29], s.db[209][30], s.db[209][31], s.db[209][32], s.db[209][33], s.db[209][34], s.db[209][35], eq33_e767_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_reactive_node_derivatives: [f64; 30] = [eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29];
        let eq33_reactive_branch_derivatives: [f64; 36] = [eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29, eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35, eq34_e779_q,) = {
    if s.b[466] {
        let eq34_e772_q: f64 = s.v[210];
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e776_q: f64 = eq34_e775;
        let eq34_e777: f64 = (s.v[210] + eq34_e775);
        let eq34_e777_d_n7: f64 = (s.dn[210][7] + p.p355);
        let eq34_e777_q: f64 = (eq34_e772_q + eq34_e776_q);
        (eq34_e777, s.dn[210][0], s.dn[210][1], s.dn[210][2], s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], eq34_e777_d_n7, s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], __rspice_deriv_cse_1, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29], s.db[210][0], s.db[210][1], s.db[210][2], s.db[210][3], s.db[210][4], s.db[210][5], s.db[210][6], s.db[210][7], s.db[210][8], s.db[210][9], s.db[210][10], s.db[210][11], s.db[210][12], s.db[210][13], s.db[210][14], s.db[210][15], s.db[210][16], s.db[210][17], s.db[210][18], s.db[210][19], s.db[210][20], s.db[210][21], s.db[210][22], s.db[210][23], s.db[210][24], s.db[210][25], s.db[210][26], s.db[210][27], s.db[210][28], s.db[210][29], s.db[210][30], s.db[210][31], s.db[210][32], s.db[210][33], s.db[210][34], s.db[210][35], eq34_e777_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 30] = [eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29];
        let eq34_reactive_branch_derivatives: [f64; 36] = [eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[17]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29, eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35, eq35_e789_q,) = {
    if s.b[466] {
        let eq35_e782_q: f64 = s.v[211];
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e786_q: f64 = eq35_e785;
        let eq35_e787: f64 = (s.v[211] + eq35_e785);
        let eq35_e787_d_n2: f64 = (s.dn[211][2] + p.p355);
        let eq35_e787_q: f64 = (eq35_e782_q + eq35_e786_q);
        (eq35_e787, s.dn[211][0], s.dn[211][1], eq35_e787_d_n2, s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], s.dn[211][7], s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], __rspice_deriv_cse_2, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29], s.db[211][0], s.db[211][1], s.db[211][2], s.db[211][3], s.db[211][4], s.db[211][5], s.db[211][6], s.db[211][7], s.db[211][8], s.db[211][9], s.db[211][10], s.db[211][11], s.db[211][12], s.db[211][13], s.db[211][14], s.db[211][15], s.db[211][16], s.db[211][17], s.db[211][18], s.db[211][19], s.db[211][20], s.db[211][21], s.db[211][22], s.db[211][23], s.db[211][24], s.db[211][25], s.db[211][26], s.db[211][27], s.db[211][28], s.db[211][29], s.db[211][30], s.db[211][31], s.db[211][32], s.db[211][33], s.db[211][34], s.db[211][35], eq35_e787_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 30] = [eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29];
        let eq35_reactive_branch_derivatives: [f64; 36] = [eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29, eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35, eq37_e803_q,) = {
    if s.b[466] {
        let eq37_e796_q: f64 = s.v[213];
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e800_q: f64 = eq37_e799;
        let eq37_e801: f64 = (s.v[213] + eq37_e799);
        let eq37_e801_d_n7: f64 = (s.dn[213][7] + p.p355);
        let eq37_e801_d_n9: f64 = (s.dn[213][9] + (-p.p355));
        let eq37_e801_q: f64 = (eq37_e796_q + eq37_e800_q);
        (eq37_e801, s.dn[213][0], s.dn[213][1], s.dn[213][2], s.dn[213][3], s.dn[213][4], s.dn[213][5], s.dn[213][6], eq37_e801_d_n7, s.dn[213][8], eq37_e801_d_n9, s.dn[213][10], s.dn[213][11], s.dn[213][12], s.dn[213][13], s.dn[213][14], s.dn[213][15], s.dn[213][16], s.dn[213][17], s.dn[213][18], s.dn[213][19], s.dn[213][20], s.dn[213][21], s.dn[213][22], s.dn[213][23], s.dn[213][24], s.dn[213][25], s.dn[213][26], s.dn[213][27], s.dn[213][28], s.dn[213][29], s.db[213][0], s.db[213][1], s.db[213][2], s.db[213][3], s.db[213][4], s.db[213][5], s.db[213][6], s.db[213][7], s.db[213][8], s.db[213][9], s.db[213][10], s.db[213][11], s.db[213][12], s.db[213][13], s.db[213][14], s.db[213][15], s.db[213][16], s.db[213][17], s.db[213][18], s.db[213][19], s.db[213][20], s.db[213][21], s.db[213][22], s.db[213][23], s.db[213][24], s.db[213][25], s.db[213][26], s.db[213][27], s.db[213][28], s.db[213][29], s.db[213][30], s.db[213][31], s.db[213][32], s.db[213][33], s.db[213][34], s.db[213][35], eq37_e801_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 30] = [eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29];
        let eq37_reactive_branch_derivatives: [f64; 36] = [eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29, eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35, eq38_e814_q,) = {
    if (!s.b[466]) {
        let eq38_e807_q: f64 = s.v[209];
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e811_q: f64 = eq38_e810;
        let eq38_e812: f64 = (s.v[209] + eq38_e810);
        let eq38_e812_d_n2: f64 = (s.dn[209][2] + p.p355);
        let eq38_e812_q: f64 = (eq38_e807_q + eq38_e811_q);
        (eq38_e812, s.dn[209][0], s.dn[209][1], eq38_e812_d_n2, s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11], s.dn[209][12], s.dn[209][13], s.dn[209][14], s.dn[209][15], __rspice_deriv_cse_0, s.dn[209][17], s.dn[209][18], s.dn[209][19], s.dn[209][20], s.dn[209][21], s.dn[209][22], s.dn[209][23], s.dn[209][24], s.dn[209][25], s.dn[209][26], s.dn[209][27], s.dn[209][28], s.dn[209][29], s.db[209][0], s.db[209][1], s.db[209][2], s.db[209][3], s.db[209][4], s.db[209][5], s.db[209][6], s.db[209][7], s.db[209][8], s.db[209][9], s.db[209][10], s.db[209][11], s.db[209][12], s.db[209][13], s.db[209][14], s.db[209][15], s.db[209][16], s.db[209][17], s.db[209][18], s.db[209][19], s.db[209][20], s.db[209][21], s.db[209][22], s.db[209][23], s.db[209][24], s.db[209][25], s.db[209][26], s.db[209][27], s.db[209][28], s.db[209][29], s.db[209][30], s.db[209][31], s.db[209][32], s.db[209][33], s.db[209][34], s.db[209][35], eq38_e812_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 30] = [eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29];
        let eq38_reactive_branch_derivatives: [f64; 36] = [eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29, eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35, eq39_e825_q,) = {
    if (!s.b[466]) {
        let eq39_e818_q: f64 = s.v[210];
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e822_q: f64 = eq39_e821;
        let eq39_e823: f64 = (s.v[210] + eq39_e821);
        let eq39_e823_d_n2: f64 = (s.dn[210][2] + p.p355);
        let eq39_e823_q: f64 = (eq39_e818_q + eq39_e822_q);
        (eq39_e823, s.dn[210][0], s.dn[210][1], eq39_e823_d_n2, s.dn[210][3], s.dn[210][4], s.dn[210][5], s.dn[210][6], s.dn[210][7], s.dn[210][8], s.dn[210][9], s.dn[210][10], s.dn[210][11], s.dn[210][12], s.dn[210][13], s.dn[210][14], s.dn[210][15], s.dn[210][16], __rspice_deriv_cse_1, s.dn[210][18], s.dn[210][19], s.dn[210][20], s.dn[210][21], s.dn[210][22], s.dn[210][23], s.dn[210][24], s.dn[210][25], s.dn[210][26], s.dn[210][27], s.dn[210][28], s.dn[210][29], s.db[210][0], s.db[210][1], s.db[210][2], s.db[210][3], s.db[210][4], s.db[210][5], s.db[210][6], s.db[210][7], s.db[210][8], s.db[210][9], s.db[210][10], s.db[210][11], s.db[210][12], s.db[210][13], s.db[210][14], s.db[210][15], s.db[210][16], s.db[210][17], s.db[210][18], s.db[210][19], s.db[210][20], s.db[210][21], s.db[210][22], s.db[210][23], s.db[210][24], s.db[210][25], s.db[210][26], s.db[210][27], s.db[210][28], s.db[210][29], s.db[210][30], s.db[210][31], s.db[210][32], s.db[210][33], s.db[210][34], s.db[210][35], eq39_e823_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 30] = [eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29];
        let eq39_reactive_branch_derivatives: [f64; 36] = [eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29, eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35, eq40_e836_q,) = {
    if (!s.b[466]) {
        let eq40_e829_q: f64 = s.v[211];
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e833_q: f64 = eq40_e832;
        let eq40_e834: f64 = (s.v[211] + eq40_e832);
        let eq40_e834_d_n7: f64 = (s.dn[211][7] + p.p355);
        let eq40_e834_q: f64 = (eq40_e829_q + eq40_e833_q);
        (eq40_e834, s.dn[211][0], s.dn[211][1], s.dn[211][2], s.dn[211][3], s.dn[211][4], s.dn[211][5], s.dn[211][6], eq40_e834_d_n7, s.dn[211][8], s.dn[211][9], s.dn[211][10], s.dn[211][11], s.dn[211][12], s.dn[211][13], s.dn[211][14], s.dn[211][15], __rspice_deriv_cse_2, s.dn[211][17], s.dn[211][18], s.dn[211][19], s.dn[211][20], s.dn[211][21], s.dn[211][22], s.dn[211][23], s.dn[211][24], s.dn[211][25], s.dn[211][26], s.dn[211][27], s.dn[211][28], s.dn[211][29], s.db[211][0], s.db[211][1], s.db[211][2], s.db[211][3], s.db[211][4], s.db[211][5], s.db[211][6], s.db[211][7], s.db[211][8], s.db[211][9], s.db[211][10], s.db[211][11], s.db[211][12], s.db[211][13], s.db[211][14], s.db[211][15], s.db[211][16], s.db[211][17], s.db[211][18], s.db[211][19], s.db[211][20], s.db[211][21], s.db[211][22], s.db[211][23], s.db[211][24], s.db[211][25], s.db[211][26], s.db[211][27], s.db[211][28], s.db[211][29], s.db[211][30], s.db[211][31], s.db[211][32], s.db[211][33], s.db[211][34], s.db[211][35], eq40_e834_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 30] = [eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29];
        let eq40_reactive_branch_derivatives: [f64; 36] = [eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e848_q: f64 = s.v[212];
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e852_q: f64 = eq43_e851;
        let eq43_e853: f64 = (s.v[212] + eq43_e851);
        let eq43_e853_d_n3: f64 = (s.dn[212][3] + p.p355);
        let eq43_e853_d_n16: f64 = (s.dn[212][16] + (-p.p355));
        let eq43_e853_q: f64 = (eq43_e848_q + eq43_e852_q);
        let eq43_reactive_node_derivatives: [f64; 30] = [s.dn[212][0], s.dn[212][1], s.dn[212][2], eq43_e853_d_n3, s.dn[212][4], s.dn[212][5], s.dn[212][6], s.dn[212][7], s.dn[212][8], s.dn[212][9], s.dn[212][10], s.dn[212][11], s.dn[212][12], s.dn[212][13], s.dn[212][14], s.dn[212][15], eq43_e853_d_n16, s.dn[212][17], s.dn[212][18], s.dn[212][19], s.dn[212][20], s.dn[212][21], s.dn[212][22], s.dn[212][23], s.dn[212][24], s.dn[212][25], s.dn[212][26], s.dn[212][27], s.dn[212][28], s.dn[212][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &s.db[212],
            multiplicity,
        );
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29, eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35, eq46_e876_q,) = {
    if s.b[613] {
        let eq46_e869_q: f64 = s.v[203];
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e873_q: f64 = eq46_e872;
        let eq46_e874: f64 = (s.v[203] + eq46_e872);
        let eq46_e874_d_n7: f64 = (s.dn[203][7] + p.p355);
        let eq46_e874_q: f64 = (eq46_e869_q + eq46_e873_q);
        (eq46_e874, s.dn[203][0], s.dn[203][1], s.dn[203][2], s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], eq46_e874_d_n7, s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], __rspice_deriv_cse_3, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29], s.db[203][0], s.db[203][1], s.db[203][2], s.db[203][3], s.db[203][4], s.db[203][5], s.db[203][6], s.db[203][7], s.db[203][8], s.db[203][9], s.db[203][10], s.db[203][11], s.db[203][12], s.db[203][13], s.db[203][14], s.db[203][15], s.db[203][16], s.db[203][17], s.db[203][18], s.db[203][19], s.db[203][20], s.db[203][21], s.db[203][22], s.db[203][23], s.db[203][24], s.db[203][25], s.db[203][26], s.db[203][27], s.db[203][28], s.db[203][29], s.db[203][30], s.db[203][31], s.db[203][32], s.db[203][33], s.db[203][34], s.db[203][35], eq46_e874_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 30] = [eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29];
        let eq46_reactive_branch_derivatives: [f64; 36] = [eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29, eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35, eq47_e886_q,) = {
    if s.b[613] {
        let eq47_e879_q: f64 = s.v[204];
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e883_q: f64 = eq47_e882;
        let eq47_e884: f64 = (s.v[204] + eq47_e882);
        let eq47_e884_d_n7: f64 = (s.dn[204][7] + p.p355);
        let eq47_e884_q: f64 = (eq47_e879_q + eq47_e883_q);
        (eq47_e884, s.dn[204][0], s.dn[204][1], s.dn[204][2], s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], eq47_e884_d_n7, s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], __rspice_deriv_cse_4, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29], s.db[204][0], s.db[204][1], s.db[204][2], s.db[204][3], s.db[204][4], s.db[204][5], s.db[204][6], s.db[204][7], s.db[204][8], s.db[204][9], s.db[204][10], s.db[204][11], s.db[204][12], s.db[204][13], s.db[204][14], s.db[204][15], s.db[204][16], s.db[204][17], s.db[204][18], s.db[204][19], s.db[204][20], s.db[204][21], s.db[204][22], s.db[204][23], s.db[204][24], s.db[204][25], s.db[204][26], s.db[204][27], s.db[204][28], s.db[204][29], s.db[204][30], s.db[204][31], s.db[204][32], s.db[204][33], s.db[204][34], s.db[204][35], eq47_e884_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 30] = [eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29];
        let eq47_reactive_branch_derivatives: [f64; 36] = [eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[16]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29, eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35, eq48_e896_q,) = {
    if s.b[613] {
        let eq48_e889_q: f64 = s.v[205];
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e893_q: f64 = eq48_e892;
        let eq48_e894: f64 = (s.v[205] + eq48_e892);
        let eq48_e894_d_n2: f64 = (s.dn[205][2] + p.p355);
        let eq48_e894_q: f64 = (eq48_e889_q + eq48_e893_q);
        (eq48_e894, s.dn[205][0], s.dn[205][1], eq48_e894_d_n2, s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], s.dn[205][7], s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], __rspice_deriv_cse_5, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], s.db[205][0], s.db[205][1], s.db[205][2], s.db[205][3], s.db[205][4], s.db[205][5], s.db[205][6], s.db[205][7], s.db[205][8], s.db[205][9], s.db[205][10], s.db[205][11], s.db[205][12], s.db[205][13], s.db[205][14], s.db[205][15], s.db[205][16], s.db[205][17], s.db[205][18], s.db[205][19], s.db[205][20], s.db[205][21], s.db[205][22], s.db[205][23], s.db[205][24], s.db[205][25], s.db[205][26], s.db[205][27], s.db[205][28], s.db[205][29], s.db[205][30], s.db[205][31], s.db[205][32], s.db[205][33], s.db[205][34], s.db[205][35], eq48_e894_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 30] = [eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29];
        let eq48_reactive_branch_derivatives: [f64; 36] = [eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29, eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35, eq50_e910_q,) = {
    if s.b[613] {
        let eq50_e903_q: f64 = s.v[207];
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e907_q: f64 = eq50_e906;
        let eq50_e908: f64 = (s.v[207] + eq50_e906);
        let eq50_e908_d_n7: f64 = (s.dn[207][7] + p.p355);
        let eq50_e908_d_n9: f64 = (s.dn[207][9] + (-p.p355));
        let eq50_e908_q: f64 = (eq50_e903_q + eq50_e907_q);
        (eq50_e908, s.dn[207][0], s.dn[207][1], s.dn[207][2], s.dn[207][3], s.dn[207][4], s.dn[207][5], s.dn[207][6], eq50_e908_d_n7, s.dn[207][8], eq50_e908_d_n9, s.dn[207][10], s.dn[207][11], s.dn[207][12], s.dn[207][13], s.dn[207][14], s.dn[207][15], s.dn[207][16], s.dn[207][17], s.dn[207][18], s.dn[207][19], s.dn[207][20], s.dn[207][21], s.dn[207][22], s.dn[207][23], s.dn[207][24], s.dn[207][25], s.dn[207][26], s.dn[207][27], s.dn[207][28], s.dn[207][29], s.db[207][0], s.db[207][1], s.db[207][2], s.db[207][3], s.db[207][4], s.db[207][5], s.db[207][6], s.db[207][7], s.db[207][8], s.db[207][9], s.db[207][10], s.db[207][11], s.db[207][12], s.db[207][13], s.db[207][14], s.db[207][15], s.db[207][16], s.db[207][17], s.db[207][18], s.db[207][19], s.db[207][20], s.db[207][21], s.db[207][22], s.db[207][23], s.db[207][24], s.db[207][25], s.db[207][26], s.db[207][27], s.db[207][28], s.db[207][29], s.db[207][30], s.db[207][31], s.db[207][32], s.db[207][33], s.db[207][34], s.db[207][35], eq50_e908_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 30] = [eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29];
        let eq50_reactive_branch_derivatives: [f64; 36] = [eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29, eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35, eq51_e921_q,) = {
    if (!s.b[613]) {
        let eq51_e914_q: f64 = s.v[203];
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e918_q: f64 = eq51_e917;
        let eq51_e919: f64 = (s.v[203] + eq51_e917);
        let eq51_e919_d_n2: f64 = (s.dn[203][2] + p.p355);
        let eq51_e919_q: f64 = (eq51_e914_q + eq51_e918_q);
        (eq51_e919, s.dn[203][0], s.dn[203][1], eq51_e919_d_n2, s.dn[203][3], s.dn[203][4], s.dn[203][5], s.dn[203][6], s.dn[203][7], s.dn[203][8], s.dn[203][9], s.dn[203][10], s.dn[203][11], s.dn[203][12], s.dn[203][13], s.dn[203][14], __rspice_deriv_cse_3, s.dn[203][16], s.dn[203][17], s.dn[203][18], s.dn[203][19], s.dn[203][20], s.dn[203][21], s.dn[203][22], s.dn[203][23], s.dn[203][24], s.dn[203][25], s.dn[203][26], s.dn[203][27], s.dn[203][28], s.dn[203][29], s.db[203][0], s.db[203][1], s.db[203][2], s.db[203][3], s.db[203][4], s.db[203][5], s.db[203][6], s.db[203][7], s.db[203][8], s.db[203][9], s.db[203][10], s.db[203][11], s.db[203][12], s.db[203][13], s.db[203][14], s.db[203][15], s.db[203][16], s.db[203][17], s.db[203][18], s.db[203][19], s.db[203][20], s.db[203][21], s.db[203][22], s.db[203][23], s.db[203][24], s.db[203][25], s.db[203][26], s.db[203][27], s.db[203][28], s.db[203][29], s.db[203][30], s.db[203][31], s.db[203][32], s.db[203][33], s.db[203][34], s.db[203][35], eq51_e919_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 30] = [eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29];
        let eq51_reactive_branch_derivatives: [f64; 36] = [eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29, eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35, eq52_e932_q,) = {
    if (!s.b[613]) {
        let eq52_e925_q: f64 = s.v[204];
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e929_q: f64 = eq52_e928;
        let eq52_e930: f64 = (s.v[204] + eq52_e928);
        let eq52_e930_d_n2: f64 = (s.dn[204][2] + p.p355);
        let eq52_e930_q: f64 = (eq52_e925_q + eq52_e929_q);
        (eq52_e930, s.dn[204][0], s.dn[204][1], eq52_e930_d_n2, s.dn[204][3], s.dn[204][4], s.dn[204][5], s.dn[204][6], s.dn[204][7], s.dn[204][8], s.dn[204][9], s.dn[204][10], s.dn[204][11], s.dn[204][12], s.dn[204][13], s.dn[204][14], s.dn[204][15], __rspice_deriv_cse_4, s.dn[204][17], s.dn[204][18], s.dn[204][19], s.dn[204][20], s.dn[204][21], s.dn[204][22], s.dn[204][23], s.dn[204][24], s.dn[204][25], s.dn[204][26], s.dn[204][27], s.dn[204][28], s.dn[204][29], s.db[204][0], s.db[204][1], s.db[204][2], s.db[204][3], s.db[204][4], s.db[204][5], s.db[204][6], s.db[204][7], s.db[204][8], s.db[204][9], s.db[204][10], s.db[204][11], s.db[204][12], s.db[204][13], s.db[204][14], s.db[204][15], s.db[204][16], s.db[204][17], s.db[204][18], s.db[204][19], s.db[204][20], s.db[204][21], s.db[204][22], s.db[204][23], s.db[204][24], s.db[204][25], s.db[204][26], s.db[204][27], s.db[204][28], s.db[204][29], s.db[204][30], s.db[204][31], s.db[204][32], s.db[204][33], s.db[204][34], s.db[204][35], eq52_e930_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 30] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29];
        let eq52_reactive_branch_derivatives: [f64; 36] = [eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29, eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35, eq53_e943_q,) = {
    if (!s.b[613]) {
        let eq53_e936_q: f64 = s.v[205];
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e940_q: f64 = eq53_e939;
        let eq53_e941: f64 = (s.v[205] + eq53_e939);
        let eq53_e941_d_n7: f64 = (s.dn[205][7] + p.p355);
        let eq53_e941_q: f64 = (eq53_e936_q + eq53_e940_q);
        (eq53_e941, s.dn[205][0], s.dn[205][1], s.dn[205][2], s.dn[205][3], s.dn[205][4], s.dn[205][5], s.dn[205][6], eq53_e941_d_n7, s.dn[205][8], s.dn[205][9], s.dn[205][10], s.dn[205][11], s.dn[205][12], s.dn[205][13], s.dn[205][14], __rspice_deriv_cse_5, s.dn[205][16], s.dn[205][17], s.dn[205][18], s.dn[205][19], s.dn[205][20], s.dn[205][21], s.dn[205][22], s.dn[205][23], s.dn[205][24], s.dn[205][25], s.dn[205][26], s.dn[205][27], s.dn[205][28], s.dn[205][29], s.db[205][0], s.db[205][1], s.db[205][2], s.db[205][3], s.db[205][4], s.db[205][5], s.db[205][6], s.db[205][7], s.db[205][8], s.db[205][9], s.db[205][10], s.db[205][11], s.db[205][12], s.db[205][13], s.db[205][14], s.db[205][15], s.db[205][16], s.db[205][17], s.db[205][18], s.db[205][19], s.db[205][20], s.db[205][21], s.db[205][22], s.db[205][23], s.db[205][24], s.db[205][25], s.db[205][26], s.db[205][27], s.db[205][28], s.db[205][29], s.db[205][30], s.db[205][31], s.db[205][32], s.db[205][33], s.db[205][34], s.db[205][35], eq53_e941_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 30] = [eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29];
        let eq53_reactive_branch_derivatives: [f64; 36] = [eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e955_q: f64 = s.v[206];
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e959_q: f64 = eq56_e958;
        let eq56_e960: f64 = (s.v[206] + eq56_e958);
        let eq56_e960_d_n3: f64 = (s.dn[206][3] + p.p355);
        let eq56_e960_d_n15: f64 = (s.dn[206][15] + (-p.p355));
        let eq56_e960_q: f64 = (eq56_e955_q + eq56_e959_q);
        let eq56_reactive_node_derivatives: [f64; 30] = [s.dn[206][0], s.dn[206][1], s.dn[206][2], eq56_e960_d_n3, s.dn[206][4], s.dn[206][5], s.dn[206][6], s.dn[206][7], s.dn[206][8], s.dn[206][9], s.dn[206][10], s.dn[206][11], s.dn[206][12], s.dn[206][13], s.dn[206][14], eq56_e960_d_n15, s.dn[206][16], s.dn[206][17], s.dn[206][18], s.dn[206][19], s.dn[206][20], s.dn[206][21], s.dn[206][22], s.dn[206][23], s.dn[206][24], s.dn[206][25], s.dn[206][26], s.dn[206][27], s.dn[206][28], s.dn[206][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &s.db[206],
            multiplicity,
        );
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29, eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35, eq59_e983_q,) = {
    if s.b[760] {
        let eq59_e976_q: f64 = s.v[197];
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e980_q: f64 = eq59_e979;
        let eq59_e981: f64 = (s.v[197] + eq59_e979);
        let eq59_e981_d_n7: f64 = (s.dn[197][7] + p.p355);
        let eq59_e981_q: f64 = (eq59_e976_q + eq59_e980_q);
        (eq59_e981, s.dn[197][0], s.dn[197][1], s.dn[197][2], s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], eq59_e981_d_n7, s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], __rspice_deriv_cse_6, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], s.db[197][0], s.db[197][1], s.db[197][2], s.db[197][3], s.db[197][4], s.db[197][5], s.db[197][6], s.db[197][7], s.db[197][8], s.db[197][9], s.db[197][10], s.db[197][11], s.db[197][12], s.db[197][13], s.db[197][14], s.db[197][15], s.db[197][16], s.db[197][17], s.db[197][18], s.db[197][19], s.db[197][20], s.db[197][21], s.db[197][22], s.db[197][23], s.db[197][24], s.db[197][25], s.db[197][26], s.db[197][27], s.db[197][28], s.db[197][29], s.db[197][30], s.db[197][31], s.db[197][32], s.db[197][33], s.db[197][34], s.db[197][35], eq59_e981_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_reactive_node_derivatives: [f64; 30] = [eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29];
        let eq59_reactive_branch_derivatives: [f64; 36] = [eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq59_reactive_node_derivatives,
            branches,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29, eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35, eq60_e993_q,) = {
    if s.b[760] {
        let eq60_e986_q: f64 = s.v[198];
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e990_q: f64 = eq60_e989;
        let eq60_e991: f64 = (s.v[198] + eq60_e989);
        let eq60_e991_d_n7: f64 = (s.dn[198][7] + p.p355);
        let eq60_e991_d_n15: f64 = (s.dn[198][15] + (-p.p355));
        let eq60_e991_q: f64 = (eq60_e986_q + eq60_e990_q);
        (eq60_e991, s.dn[198][0], s.dn[198][1], s.dn[198][2], s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], eq60_e991_d_n7, s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq60_e991_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], s.db[198][0], s.db[198][1], s.db[198][2], s.db[198][3], s.db[198][4], s.db[198][5], s.db[198][6], s.db[198][7], s.db[198][8], s.db[198][9], s.db[198][10], s.db[198][11], s.db[198][12], s.db[198][13], s.db[198][14], s.db[198][15], s.db[198][16], s.db[198][17], s.db[198][18], s.db[198][19], s.db[198][20], s.db[198][21], s.db[198][22], s.db[198][23], s.db[198][24], s.db[198][25], s.db[198][26], s.db[198][27], s.db[198][28], s.db[198][29], s.db[198][30], s.db[198][31], s.db[198][32], s.db[198][33], s.db[198][34], s.db[198][35], eq60_e991_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_reactive_node_derivatives: [f64; 30] = [eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29];
        let eq60_reactive_branch_derivatives: [f64; 36] = [eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[15]),
            nodes,
            &eq60_reactive_node_derivatives,
            branches,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29, eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35, eq61_e1003_q,) = {
    if s.b[760] {
        let eq61_e996_q: f64 = s.v[199];
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e1000_q: f64 = eq61_e999;
        let eq61_e1001: f64 = (s.v[199] + eq61_e999);
        let eq61_e1001_d_n2: f64 = (s.dn[199][2] + p.p355);
        let eq61_e1001_d_n14: f64 = (s.dn[199][14] + (-p.p355));
        let eq61_e1001_q: f64 = (eq61_e996_q + eq61_e1000_q);
        (eq61_e1001, s.dn[199][0], s.dn[199][1], eq61_e1001_d_n2, s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], s.dn[199][7], s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq61_e1001_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], s.db[199][0], s.db[199][1], s.db[199][2], s.db[199][3], s.db[199][4], s.db[199][5], s.db[199][6], s.db[199][7], s.db[199][8], s.db[199][9], s.db[199][10], s.db[199][11], s.db[199][12], s.db[199][13], s.db[199][14], s.db[199][15], s.db[199][16], s.db[199][17], s.db[199][18], s.db[199][19], s.db[199][20], s.db[199][21], s.db[199][22], s.db[199][23], s.db[199][24], s.db[199][25], s.db[199][26], s.db[199][27], s.db[199][28], s.db[199][29], s.db[199][30], s.db[199][31], s.db[199][32], s.db[199][33], s.db[199][34], s.db[199][35], eq61_e1001_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 30] = [eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29];
        let eq61_reactive_branch_derivatives: [f64; 36] = [eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29, eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35, eq63_e1017_q,) = {
    if s.b[760] {
        let eq63_e1010_q: f64 = s.v[201];
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1014_q: f64 = eq63_e1013;
        let eq63_e1015: f64 = (s.v[201] + eq63_e1013);
        let eq63_e1015_d_n7: f64 = (s.dn[201][7] + p.p355);
        let eq63_e1015_d_n9: f64 = (s.dn[201][9] + (-p.p355));
        let eq63_e1015_q: f64 = (eq63_e1010_q + eq63_e1014_q);
        (eq63_e1015, s.dn[201][0], s.dn[201][1], s.dn[201][2], s.dn[201][3], s.dn[201][4], s.dn[201][5], s.dn[201][6], eq63_e1015_d_n7, s.dn[201][8], eq63_e1015_d_n9, s.dn[201][10], s.dn[201][11], s.dn[201][12], s.dn[201][13], s.dn[201][14], s.dn[201][15], s.dn[201][16], s.dn[201][17], s.dn[201][18], s.dn[201][19], s.dn[201][20], s.dn[201][21], s.dn[201][22], s.dn[201][23], s.dn[201][24], s.dn[201][25], s.dn[201][26], s.dn[201][27], s.dn[201][28], s.dn[201][29], s.db[201][0], s.db[201][1], s.db[201][2], s.db[201][3], s.db[201][4], s.db[201][5], s.db[201][6], s.db[201][7], s.db[201][8], s.db[201][9], s.db[201][10], s.db[201][11], s.db[201][12], s.db[201][13], s.db[201][14], s.db[201][15], s.db[201][16], s.db[201][17], s.db[201][18], s.db[201][19], s.db[201][20], s.db[201][21], s.db[201][22], s.db[201][23], s.db[201][24], s.db[201][25], s.db[201][26], s.db[201][27], s.db[201][28], s.db[201][29], s.db[201][30], s.db[201][31], s.db[201][32], s.db[201][33], s.db[201][34], s.db[201][35], eq63_e1015_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_reactive_node_derivatives: [f64; 30] = [eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29];
        let eq63_reactive_branch_derivatives: [f64; 36] = [eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29, eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35, eq64_e1028_q,) = {
    if (!s.b[760]) {
        let eq64_e1021_q: f64 = s.v[197];
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1025_q: f64 = eq64_e1024;
        let eq64_e1026: f64 = (s.v[197] + eq64_e1024);
        let eq64_e1026_d_n2: f64 = (s.dn[197][2] + p.p355);
        let eq64_e1026_q: f64 = (eq64_e1021_q + eq64_e1025_q);
        (eq64_e1026, s.dn[197][0], s.dn[197][1], eq64_e1026_d_n2, s.dn[197][3], s.dn[197][4], s.dn[197][5], s.dn[197][6], s.dn[197][7], s.dn[197][8], s.dn[197][9], s.dn[197][10], s.dn[197][11], s.dn[197][12], s.dn[197][13], __rspice_deriv_cse_6, s.dn[197][15], s.dn[197][16], s.dn[197][17], s.dn[197][18], s.dn[197][19], s.dn[197][20], s.dn[197][21], s.dn[197][22], s.dn[197][23], s.dn[197][24], s.dn[197][25], s.dn[197][26], s.dn[197][27], s.dn[197][28], s.dn[197][29], s.db[197][0], s.db[197][1], s.db[197][2], s.db[197][3], s.db[197][4], s.db[197][5], s.db[197][6], s.db[197][7], s.db[197][8], s.db[197][9], s.db[197][10], s.db[197][11], s.db[197][12], s.db[197][13], s.db[197][14], s.db[197][15], s.db[197][16], s.db[197][17], s.db[197][18], s.db[197][19], s.db[197][20], s.db[197][21], s.db[197][22], s.db[197][23], s.db[197][24], s.db[197][25], s.db[197][26], s.db[197][27], s.db[197][28], s.db[197][29], s.db[197][30], s.db[197][31], s.db[197][32], s.db[197][33], s.db[197][34], s.db[197][35], eq64_e1026_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_reactive_node_derivatives: [f64; 30] = [eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29];
        let eq64_reactive_branch_derivatives: [f64; 36] = [eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq64_reactive_node_derivatives,
            branches,
            &eq64_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let __rspice_deriv_cse_0: f64 = (s.dn[191][5] + (-p.p355));
        let __rspice_deriv_cse_1: f64 = (s.dn[192][14] + (-p.p355));
        let __rspice_deriv_cse_2: f64 = (s.dn[193][5] + (-p.p355));
        let __rspice_deriv_cse_3: f64 = (s.dn[167][10] + (-p.p355));
        let __rspice_deriv_cse_4: f64 = (s.dn[168][9] + (-p.p355));
        let __rspice_deriv_cse_5: f64 = (s.dn[169][10] + (-p.p355));
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29, eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35, eq65_e1039_q,) = {
    if (!s.b[760]) {
        let eq65_e1032_q: f64 = s.v[198];
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1036_q: f64 = eq65_e1035;
        let eq65_e1037: f64 = (s.v[198] + eq65_e1035);
        let eq65_e1037_d_n2: f64 = (s.dn[198][2] + p.p355);
        let eq65_e1037_d_n15: f64 = (s.dn[198][15] + (-p.p355));
        let eq65_e1037_q: f64 = (eq65_e1032_q + eq65_e1036_q);
        (eq65_e1037, s.dn[198][0], s.dn[198][1], eq65_e1037_d_n2, s.dn[198][3], s.dn[198][4], s.dn[198][5], s.dn[198][6], s.dn[198][7], s.dn[198][8], s.dn[198][9], s.dn[198][10], s.dn[198][11], s.dn[198][12], s.dn[198][13], s.dn[198][14], eq65_e1037_d_n15, s.dn[198][16], s.dn[198][17], s.dn[198][18], s.dn[198][19], s.dn[198][20], s.dn[198][21], s.dn[198][22], s.dn[198][23], s.dn[198][24], s.dn[198][25], s.dn[198][26], s.dn[198][27], s.dn[198][28], s.dn[198][29], s.db[198][0], s.db[198][1], s.db[198][2], s.db[198][3], s.db[198][4], s.db[198][5], s.db[198][6], s.db[198][7], s.db[198][8], s.db[198][9], s.db[198][10], s.db[198][11], s.db[198][12], s.db[198][13], s.db[198][14], s.db[198][15], s.db[198][16], s.db[198][17], s.db[198][18], s.db[198][19], s.db[198][20], s.db[198][21], s.db[198][22], s.db[198][23], s.db[198][24], s.db[198][25], s.db[198][26], s.db[198][27], s.db[198][28], s.db[198][29], s.db[198][30], s.db[198][31], s.db[198][32], s.db[198][33], s.db[198][34], s.db[198][35], eq65_e1037_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 30] = [eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29];
        let eq65_reactive_branch_derivatives: [f64; 36] = [eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq65_reactive_node_derivatives,
            branches,
            &eq65_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29, eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35, eq66_e1050_q,) = {
    if (!s.b[760]) {
        let eq66_e1043_q: f64 = s.v[199];
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1047_q: f64 = eq66_e1046;
        let eq66_e1048: f64 = (s.v[199] + eq66_e1046);
        let eq66_e1048_d_n7: f64 = (s.dn[199][7] + p.p355);
        let eq66_e1048_d_n14: f64 = (s.dn[199][14] + (-p.p355));
        let eq66_e1048_q: f64 = (eq66_e1043_q + eq66_e1047_q);
        (eq66_e1048, s.dn[199][0], s.dn[199][1], s.dn[199][2], s.dn[199][3], s.dn[199][4], s.dn[199][5], s.dn[199][6], eq66_e1048_d_n7, s.dn[199][8], s.dn[199][9], s.dn[199][10], s.dn[199][11], s.dn[199][12], s.dn[199][13], eq66_e1048_d_n14, s.dn[199][15], s.dn[199][16], s.dn[199][17], s.dn[199][18], s.dn[199][19], s.dn[199][20], s.dn[199][21], s.dn[199][22], s.dn[199][23], s.dn[199][24], s.dn[199][25], s.dn[199][26], s.dn[199][27], s.dn[199][28], s.dn[199][29], s.db[199][0], s.db[199][1], s.db[199][2], s.db[199][3], s.db[199][4], s.db[199][5], s.db[199][6], s.db[199][7], s.db[199][8], s.db[199][9], s.db[199][10], s.db[199][11], s.db[199][12], s.db[199][13], s.db[199][14], s.db[199][15], s.db[199][16], s.db[199][17], s.db[199][18], s.db[199][19], s.db[199][20], s.db[199][21], s.db[199][22], s.db[199][23], s.db[199][24], s.db[199][25], s.db[199][26], s.db[199][27], s.db[199][28], s.db[199][29], s.db[199][30], s.db[199][31], s.db[199][32], s.db[199][33], s.db[199][34], s.db[199][35], eq66_e1048_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 30] = [eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29];
        let eq66_reactive_branch_derivatives: [f64; 36] = [eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
        let eq69_e1062_q: f64 = s.v[200];
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1066_q: f64 = eq69_e1065;
        let eq69_e1067: f64 = (s.v[200] + eq69_e1065);
        let eq69_e1067_d_n3: f64 = (s.dn[200][3] + p.p355);
        let eq69_e1067_d_n14: f64 = (s.dn[200][14] + (-p.p355));
        let eq69_e1067_q: f64 = (eq69_e1062_q + eq69_e1066_q);
        let eq69_reactive_node_derivatives: [f64; 30] = [s.dn[200][0], s.dn[200][1], s.dn[200][2], eq69_e1067_d_n3, s.dn[200][4], s.dn[200][5], s.dn[200][6], s.dn[200][7], s.dn[200][8], s.dn[200][9], s.dn[200][10], s.dn[200][11], s.dn[200][12], s.dn[200][13], eq69_e1067_d_n14, s.dn[200][15], s.dn[200][16], s.dn[200][17], s.dn[200][18], s.dn[200][19], s.dn[200][20], s.dn[200][21], s.dn[200][22], s.dn[200][23], s.dn[200][24], s.dn[200][25], s.dn[200][26], s.dn[200][27], s.dn[200][28], s.dn[200][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[14]),
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &s.db[200],
            multiplicity,
        );
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29, eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35, eq72_e1090_q,) = {
    if s.b[907] {
        let eq72_e1083_q: f64 = s.v[191];
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1087_q: f64 = eq72_e1086;
        let eq72_e1088: f64 = (s.v[191] + eq72_e1086);
        let eq72_e1088_d_n7: f64 = (s.dn[191][7] + p.p355);
        let eq72_e1088_q: f64 = (eq72_e1083_q + eq72_e1087_q);
        (eq72_e1088, s.dn[191][0], s.dn[191][1], s.dn[191][2], s.dn[191][3], s.dn[191][4], __rspice_deriv_cse_0, s.dn[191][6], eq72_e1088_d_n7, s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29], s.db[191][0], s.db[191][1], s.db[191][2], s.db[191][3], s.db[191][4], s.db[191][5], s.db[191][6], s.db[191][7], s.db[191][8], s.db[191][9], s.db[191][10], s.db[191][11], s.db[191][12], s.db[191][13], s.db[191][14], s.db[191][15], s.db[191][16], s.db[191][17], s.db[191][18], s.db[191][19], s.db[191][20], s.db[191][21], s.db[191][22], s.db[191][23], s.db[191][24], s.db[191][25], s.db[191][26], s.db[191][27], s.db[191][28], s.db[191][29], s.db[191][30], s.db[191][31], s.db[191][32], s.db[191][33], s.db[191][34], s.db[191][35], eq72_e1088_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 30] = [eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29];
        let eq72_reactive_branch_derivatives: [f64; 36] = [eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29, eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35, eq73_e1100_q,) = {
    if s.b[907] {
        let eq73_e1093_q: f64 = s.v[192];
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1097_q: f64 = eq73_e1096;
        let eq73_e1098: f64 = (s.v[192] + eq73_e1096);
        let eq73_e1098_d_n7: f64 = (s.dn[192][7] + p.p355);
        let eq73_e1098_q: f64 = (eq73_e1093_q + eq73_e1097_q);
        (eq73_e1098, s.dn[192][0], s.dn[192][1], s.dn[192][2], s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], eq73_e1098_d_n7, s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], __rspice_deriv_cse_1, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29], s.db[192][0], s.db[192][1], s.db[192][2], s.db[192][3], s.db[192][4], s.db[192][5], s.db[192][6], s.db[192][7], s.db[192][8], s.db[192][9], s.db[192][10], s.db[192][11], s.db[192][12], s.db[192][13], s.db[192][14], s.db[192][15], s.db[192][16], s.db[192][17], s.db[192][18], s.db[192][19], s.db[192][20], s.db[192][21], s.db[192][22], s.db[192][23], s.db[192][24], s.db[192][25], s.db[192][26], s.db[192][27], s.db[192][28], s.db[192][29], s.db[192][30], s.db[192][31], s.db[192][32], s.db[192][33], s.db[192][34], s.db[192][35], eq73_e1098_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 30] = [eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29];
        let eq73_reactive_branch_derivatives: [f64; 36] = [eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[14]),
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29, eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35, eq74_e1110_q,) = {
    if s.b[907] {
        let eq74_e1103_q: f64 = s.v[193];
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1107_q: f64 = eq74_e1106;
        let eq74_e1108: f64 = (s.v[193] + eq74_e1106);
        let eq74_e1108_d_n2: f64 = (s.dn[193][2] + p.p355);
        let eq74_e1108_q: f64 = (eq74_e1103_q + eq74_e1107_q);
        (eq74_e1108, s.dn[193][0], s.dn[193][1], eq74_e1108_d_n2, s.dn[193][3], s.dn[193][4], __rspice_deriv_cse_2, s.dn[193][6], s.dn[193][7], s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29], s.db[193][0], s.db[193][1], s.db[193][2], s.db[193][3], s.db[193][4], s.db[193][5], s.db[193][6], s.db[193][7], s.db[193][8], s.db[193][9], s.db[193][10], s.db[193][11], s.db[193][12], s.db[193][13], s.db[193][14], s.db[193][15], s.db[193][16], s.db[193][17], s.db[193][18], s.db[193][19], s.db[193][20], s.db[193][21], s.db[193][22], s.db[193][23], s.db[193][24], s.db[193][25], s.db[193][26], s.db[193][27], s.db[193][28], s.db[193][29], s.db[193][30], s.db[193][31], s.db[193][32], s.db[193][33], s.db[193][34], s.db[193][35], eq74_e1108_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 30] = [eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29];
        let eq74_reactive_branch_derivatives: [f64; 36] = [eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29, eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35, eq76_e1124_q,) = {
    if s.b[907] {
        let eq76_e1117_q: f64 = s.v[195];
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1121_q: f64 = eq76_e1120;
        let eq76_e1122: f64 = (s.v[195] + eq76_e1120);
        let eq76_e1122_d_n7: f64 = (s.dn[195][7] + p.p355);
        let eq76_e1122_d_n9: f64 = (s.dn[195][9] + (-p.p355));
        let eq76_e1122_q: f64 = (eq76_e1117_q + eq76_e1121_q);
        (eq76_e1122, s.dn[195][0], s.dn[195][1], s.dn[195][2], s.dn[195][3], s.dn[195][4], s.dn[195][5], s.dn[195][6], eq76_e1122_d_n7, s.dn[195][8], eq76_e1122_d_n9, s.dn[195][10], s.dn[195][11], s.dn[195][12], s.dn[195][13], s.dn[195][14], s.dn[195][15], s.dn[195][16], s.dn[195][17], s.dn[195][18], s.dn[195][19], s.dn[195][20], s.dn[195][21], s.dn[195][22], s.dn[195][23], s.dn[195][24], s.dn[195][25], s.dn[195][26], s.dn[195][27], s.dn[195][28], s.dn[195][29], s.db[195][0], s.db[195][1], s.db[195][2], s.db[195][3], s.db[195][4], s.db[195][5], s.db[195][6], s.db[195][7], s.db[195][8], s.db[195][9], s.db[195][10], s.db[195][11], s.db[195][12], s.db[195][13], s.db[195][14], s.db[195][15], s.db[195][16], s.db[195][17], s.db[195][18], s.db[195][19], s.db[195][20], s.db[195][21], s.db[195][22], s.db[195][23], s.db[195][24], s.db[195][25], s.db[195][26], s.db[195][27], s.db[195][28], s.db[195][29], s.db[195][30], s.db[195][31], s.db[195][32], s.db[195][33], s.db[195][34], s.db[195][35], eq76_e1122_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 30] = [eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29];
        let eq76_reactive_branch_derivatives: [f64; 36] = [eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29, eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35, eq77_e1135_q,) = {
    if (!s.b[907]) {
        let eq77_e1128_q: f64 = s.v[191];
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1132_q: f64 = eq77_e1131;
        let eq77_e1133: f64 = (s.v[191] + eq77_e1131);
        let eq77_e1133_d_n2: f64 = (s.dn[191][2] + p.p355);
        let eq77_e1133_q: f64 = (eq77_e1128_q + eq77_e1132_q);
        (eq77_e1133, s.dn[191][0], s.dn[191][1], eq77_e1133_d_n2, s.dn[191][3], s.dn[191][4], __rspice_deriv_cse_0, s.dn[191][6], s.dn[191][7], s.dn[191][8], s.dn[191][9], s.dn[191][10], s.dn[191][11], s.dn[191][12], s.dn[191][13], s.dn[191][14], s.dn[191][15], s.dn[191][16], s.dn[191][17], s.dn[191][18], s.dn[191][19], s.dn[191][20], s.dn[191][21], s.dn[191][22], s.dn[191][23], s.dn[191][24], s.dn[191][25], s.dn[191][26], s.dn[191][27], s.dn[191][28], s.dn[191][29], s.db[191][0], s.db[191][1], s.db[191][2], s.db[191][3], s.db[191][4], s.db[191][5], s.db[191][6], s.db[191][7], s.db[191][8], s.db[191][9], s.db[191][10], s.db[191][11], s.db[191][12], s.db[191][13], s.db[191][14], s.db[191][15], s.db[191][16], s.db[191][17], s.db[191][18], s.db[191][19], s.db[191][20], s.db[191][21], s.db[191][22], s.db[191][23], s.db[191][24], s.db[191][25], s.db[191][26], s.db[191][27], s.db[191][28], s.db[191][29], s.db[191][30], s.db[191][31], s.db[191][32], s.db[191][33], s.db[191][34], s.db[191][35], eq77_e1133_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 30] = [eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29];
        let eq77_reactive_branch_derivatives: [f64; 36] = [eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[5]),
            nodes,
            &eq77_reactive_node_derivatives,
            branches,
            &eq77_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29, eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35, eq78_e1146_q,) = {
    if (!s.b[907]) {
        let eq78_e1139_q: f64 = s.v[192];
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1143_q: f64 = eq78_e1142;
        let eq78_e1144: f64 = (s.v[192] + eq78_e1142);
        let eq78_e1144_d_n2: f64 = (s.dn[192][2] + p.p355);
        let eq78_e1144_q: f64 = (eq78_e1139_q + eq78_e1143_q);
        (eq78_e1144, s.dn[192][0], s.dn[192][1], eq78_e1144_d_n2, s.dn[192][3], s.dn[192][4], s.dn[192][5], s.dn[192][6], s.dn[192][7], s.dn[192][8], s.dn[192][9], s.dn[192][10], s.dn[192][11], s.dn[192][12], s.dn[192][13], __rspice_deriv_cse_1, s.dn[192][15], s.dn[192][16], s.dn[192][17], s.dn[192][18], s.dn[192][19], s.dn[192][20], s.dn[192][21], s.dn[192][22], s.dn[192][23], s.dn[192][24], s.dn[192][25], s.dn[192][26], s.dn[192][27], s.dn[192][28], s.dn[192][29], s.db[192][0], s.db[192][1], s.db[192][2], s.db[192][3], s.db[192][4], s.db[192][5], s.db[192][6], s.db[192][7], s.db[192][8], s.db[192][9], s.db[192][10], s.db[192][11], s.db[192][12], s.db[192][13], s.db[192][14], s.db[192][15], s.db[192][16], s.db[192][17], s.db[192][18], s.db[192][19], s.db[192][20], s.db[192][21], s.db[192][22], s.db[192][23], s.db[192][24], s.db[192][25], s.db[192][26], s.db[192][27], s.db[192][28], s.db[192][29], s.db[192][30], s.db[192][31], s.db[192][32], s.db[192][33], s.db[192][34], s.db[192][35], eq78_e1144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_reactive_node_derivatives: [f64; 30] = [eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29];
        let eq78_reactive_branch_derivatives: [f64; 36] = [eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[14]),
            nodes,
            &eq78_reactive_node_derivatives,
            branches,
            &eq78_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29, eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35, eq79_e1157_q,) = {
    if (!s.b[907]) {
        let eq79_e1150_q: f64 = s.v[193];
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1154_q: f64 = eq79_e1153;
        let eq79_e1155: f64 = (s.v[193] + eq79_e1153);
        let eq79_e1155_d_n7: f64 = (s.dn[193][7] + p.p355);
        let eq79_e1155_q: f64 = (eq79_e1150_q + eq79_e1154_q);
        (eq79_e1155, s.dn[193][0], s.dn[193][1], s.dn[193][2], s.dn[193][3], s.dn[193][4], __rspice_deriv_cse_2, s.dn[193][6], eq79_e1155_d_n7, s.dn[193][8], s.dn[193][9], s.dn[193][10], s.dn[193][11], s.dn[193][12], s.dn[193][13], s.dn[193][14], s.dn[193][15], s.dn[193][16], s.dn[193][17], s.dn[193][18], s.dn[193][19], s.dn[193][20], s.dn[193][21], s.dn[193][22], s.dn[193][23], s.dn[193][24], s.dn[193][25], s.dn[193][26], s.dn[193][27], s.dn[193][28], s.dn[193][29], s.db[193][0], s.db[193][1], s.db[193][2], s.db[193][3], s.db[193][4], s.db[193][5], s.db[193][6], s.db[193][7], s.db[193][8], s.db[193][9], s.db[193][10], s.db[193][11], s.db[193][12], s.db[193][13], s.db[193][14], s.db[193][15], s.db[193][16], s.db[193][17], s.db[193][18], s.db[193][19], s.db[193][20], s.db[193][21], s.db[193][22], s.db[193][23], s.db[193][24], s.db[193][25], s.db[193][26], s.db[193][27], s.db[193][28], s.db[193][29], s.db[193][30], s.db[193][31], s.db[193][32], s.db[193][33], s.db[193][34], s.db[193][35], eq79_e1155_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_reactive_node_derivatives: [f64; 30] = [eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29];
        let eq79_reactive_branch_derivatives: [f64; 36] = [eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq79_reactive_node_derivatives,
            branches,
            &eq79_reactive_branch_derivatives,
            multiplicity,
        );
        let eq82_e1169_q: f64 = s.v[194];
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1173_q: f64 = eq82_e1172;
        let eq82_e1174: f64 = (s.v[194] + eq82_e1172);
        let eq82_e1174_d_n3: f64 = (s.dn[194][3] + p.p355);
        let eq82_e1174_d_n5: f64 = (s.dn[194][5] + (-p.p355));
        let eq82_e1174_q: f64 = (eq82_e1169_q + eq82_e1173_q);
        let eq82_reactive_node_derivatives: [f64; 30] = [s.dn[194][0], s.dn[194][1], s.dn[194][2], eq82_e1174_d_n3, s.dn[194][4], eq82_e1174_d_n5, s.dn[194][6], s.dn[194][7], s.dn[194][8], s.dn[194][9], s.dn[194][10], s.dn[194][11], s.dn[194][12], s.dn[194][13], s.dn[194][14], s.dn[194][15], s.dn[194][16], s.dn[194][17], s.dn[194][18], s.dn[194][19], s.dn[194][20], s.dn[194][21], s.dn[194][22], s.dn[194][23], s.dn[194][24], s.dn[194][25], s.dn[194][26], s.dn[194][27], s.dn[194][28], s.dn[194][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq82_reactive_node_derivatives,
            branches,
            &s.db[194],
            multiplicity,
        );
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29, eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35, eq85_e1197_q,) = {
    if s.b[1054] {
        let eq85_e1190_q: f64 = s.v[167];
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1194_q: f64 = eq85_e1193;
        let eq85_e1195: f64 = (s.v[167] + eq85_e1193);
        let eq85_e1195_d_n7: f64 = (s.dn[167][7] + p.p355);
        let eq85_e1195_q: f64 = (eq85_e1190_q + eq85_e1194_q);
        (eq85_e1195, s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], eq85_e1195_d_n7, s.dn[167][8], s.dn[167][9], __rspice_deriv_cse_3, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], s.db[167][4], s.db[167][5], s.db[167][6], s.db[167][7], s.db[167][8], s.db[167][9], s.db[167][10], s.db[167][11], s.db[167][12], s.db[167][13], s.db[167][14], s.db[167][15], s.db[167][16], s.db[167][17], s.db[167][18], s.db[167][19], s.db[167][20], s.db[167][21], s.db[167][22], s.db[167][23], s.db[167][24], s.db[167][25], s.db[167][26], s.db[167][27], s.db[167][28], s.db[167][29], s.db[167][30], s.db[167][31], s.db[167][32], s.db[167][33], s.db[167][34], s.db[167][35], eq85_e1195_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_reactive_node_derivatives: [f64; 30] = [eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29];
        let eq85_reactive_branch_derivatives: [f64; 36] = [eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq85_reactive_node_derivatives,
            branches,
            &eq85_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29, eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35, eq86_e1207_q,) = {
    if s.b[1054] {
        let eq86_e1200_q: f64 = s.v[168];
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1204_q: f64 = eq86_e1203;
        let eq86_e1205: f64 = (s.v[168] + eq86_e1203);
        let eq86_e1205_d_n7: f64 = (s.dn[168][7] + p.p355);
        let eq86_e1205_q: f64 = (eq86_e1200_q + eq86_e1204_q);
        (eq86_e1205, s.dn[168][0], s.dn[168][1], s.dn[168][2], s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], eq86_e1205_d_n7, s.dn[168][8], __rspice_deriv_cse_4, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29], s.db[168][0], s.db[168][1], s.db[168][2], s.db[168][3], s.db[168][4], s.db[168][5], s.db[168][6], s.db[168][7], s.db[168][8], s.db[168][9], s.db[168][10], s.db[168][11], s.db[168][12], s.db[168][13], s.db[168][14], s.db[168][15], s.db[168][16], s.db[168][17], s.db[168][18], s.db[168][19], s.db[168][20], s.db[168][21], s.db[168][22], s.db[168][23], s.db[168][24], s.db[168][25], s.db[168][26], s.db[168][27], s.db[168][28], s.db[168][29], s.db[168][30], s.db[168][31], s.db[168][32], s.db[168][33], s.db[168][34], s.db[168][35], eq86_e1205_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_reactive_node_derivatives: [f64; 30] = [eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29];
        let eq86_reactive_branch_derivatives: [f64; 36] = [eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq86_reactive_node_derivatives,
            branches,
            &eq86_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29, eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35, eq87_e1217_q,) = {
    if s.b[1054] {
        let eq87_e1210_q: f64 = s.v[169];
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1214_q: f64 = eq87_e1213;
        let eq87_e1215: f64 = (s.v[169] + eq87_e1213);
        let eq87_e1215_d_n2: f64 = (s.dn[169][2] + p.p355);
        let eq87_e1215_q: f64 = (eq87_e1210_q + eq87_e1214_q);
        (eq87_e1215, s.dn[169][0], s.dn[169][1], eq87_e1215_d_n2, s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], s.dn[169][7], s.dn[169][8], s.dn[169][9], __rspice_deriv_cse_5, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29], s.db[169][0], s.db[169][1], s.db[169][2], s.db[169][3], s.db[169][4], s.db[169][5], s.db[169][6], s.db[169][7], s.db[169][8], s.db[169][9], s.db[169][10], s.db[169][11], s.db[169][12], s.db[169][13], s.db[169][14], s.db[169][15], s.db[169][16], s.db[169][17], s.db[169][18], s.db[169][19], s.db[169][20], s.db[169][21], s.db[169][22], s.db[169][23], s.db[169][24], s.db[169][25], s.db[169][26], s.db[169][27], s.db[169][28], s.db[169][29], s.db[169][30], s.db[169][31], s.db[169][32], s.db[169][33], s.db[169][34], s.db[169][35], eq87_e1215_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_reactive_node_derivatives: [f64; 30] = [eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29];
        let eq87_reactive_branch_derivatives: [f64; 36] = [eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq87_reactive_node_derivatives,
            branches,
            &eq87_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29, eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35, eq89_e1231_q,) = {
    if s.b[1054] {
        let eq89_e1224_q: f64 = s.v[171];
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1228_q: f64 = eq89_e1227;
        let eq89_e1229: f64 = (s.v[171] + eq89_e1227);
        let eq89_e1229_d_n7: f64 = (s.dn[171][7] + p.p355);
        let eq89_e1229_d_n9: f64 = (s.dn[171][9] + (-p.p355));
        let eq89_e1229_q: f64 = (eq89_e1224_q + eq89_e1228_q);
        (eq89_e1229, s.dn[171][0], s.dn[171][1], s.dn[171][2], s.dn[171][3], s.dn[171][4], s.dn[171][5], s.dn[171][6], eq89_e1229_d_n7, s.dn[171][8], eq89_e1229_d_n9, s.dn[171][10], s.dn[171][11], s.dn[171][12], s.dn[171][13], s.dn[171][14], s.dn[171][15], s.dn[171][16], s.dn[171][17], s.dn[171][18], s.dn[171][19], s.dn[171][20], s.dn[171][21], s.dn[171][22], s.dn[171][23], s.dn[171][24], s.dn[171][25], s.dn[171][26], s.dn[171][27], s.dn[171][28], s.dn[171][29], s.db[171][0], s.db[171][1], s.db[171][2], s.db[171][3], s.db[171][4], s.db[171][5], s.db[171][6], s.db[171][7], s.db[171][8], s.db[171][9], s.db[171][10], s.db[171][11], s.db[171][12], s.db[171][13], s.db[171][14], s.db[171][15], s.db[171][16], s.db[171][17], s.db[171][18], s.db[171][19], s.db[171][20], s.db[171][21], s.db[171][22], s.db[171][23], s.db[171][24], s.db[171][25], s.db[171][26], s.db[171][27], s.db[171][28], s.db[171][29], s.db[171][30], s.db[171][31], s.db[171][32], s.db[171][33], s.db[171][34], s.db[171][35], eq89_e1229_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_reactive_node_derivatives: [f64; 30] = [eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29];
        let eq89_reactive_branch_derivatives: [f64; 36] = [eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq89_reactive_node_derivatives,
            branches,
            &eq89_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29, eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35, eq90_e1242_q,) = {
    if (!s.b[1054]) {
        let eq90_e1235_q: f64 = s.v[167];
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1239_q: f64 = eq90_e1238;
        let eq90_e1240: f64 = (s.v[167] + eq90_e1238);
        let eq90_e1240_d_n2: f64 = (s.dn[167][2] + p.p355);
        let eq90_e1240_q: f64 = (eq90_e1235_q + eq90_e1239_q);
        (eq90_e1240, s.dn[167][0], s.dn[167][1], eq90_e1240_d_n2, s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], __rspice_deriv_cse_3, s.dn[167][11], s.dn[167][12], s.dn[167][13], s.dn[167][14], s.dn[167][15], s.dn[167][16], s.dn[167][17], s.dn[167][18], s.dn[167][19], s.dn[167][20], s.dn[167][21], s.dn[167][22], s.dn[167][23], s.dn[167][24], s.dn[167][25], s.dn[167][26], s.dn[167][27], s.dn[167][28], s.dn[167][29], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], s.db[167][4], s.db[167][5], s.db[167][6], s.db[167][7], s.db[167][8], s.db[167][9], s.db[167][10], s.db[167][11], s.db[167][12], s.db[167][13], s.db[167][14], s.db[167][15], s.db[167][16], s.db[167][17], s.db[167][18], s.db[167][19], s.db[167][20], s.db[167][21], s.db[167][22], s.db[167][23], s.db[167][24], s.db[167][25], s.db[167][26], s.db[167][27], s.db[167][28], s.db[167][29], s.db[167][30], s.db[167][31], s.db[167][32], s.db[167][33], s.db[167][34], s.db[167][35], eq90_e1240_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_reactive_node_derivatives: [f64; 30] = [eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29];
        let eq90_reactive_branch_derivatives: [f64; 36] = [eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq90_reactive_node_derivatives,
            branches,
            &eq90_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29, eq91_e1253_d_b0, eq91_e1253_d_b1, eq91_e1253_d_b2, eq91_e1253_d_b3, eq91_e1253_d_b4, eq91_e1253_d_b5, eq91_e1253_d_b6, eq91_e1253_d_b7, eq91_e1253_d_b8, eq91_e1253_d_b9, eq91_e1253_d_b10, eq91_e1253_d_b11, eq91_e1253_d_b12, eq91_e1253_d_b13, eq91_e1253_d_b14, eq91_e1253_d_b15, eq91_e1253_d_b16, eq91_e1253_d_b17, eq91_e1253_d_b18, eq91_e1253_d_b19, eq91_e1253_d_b20, eq91_e1253_d_b21, eq91_e1253_d_b22, eq91_e1253_d_b23, eq91_e1253_d_b24, eq91_e1253_d_b25, eq91_e1253_d_b26, eq91_e1253_d_b27, eq91_e1253_d_b28, eq91_e1253_d_b29, eq91_e1253_d_b30, eq91_e1253_d_b31, eq91_e1253_d_b32, eq91_e1253_d_b33, eq91_e1253_d_b34, eq91_e1253_d_b35, eq91_e1253_q,) = {
    if (!s.b[1054]) {
        let eq91_e1246_q: f64 = s.v[168];
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1250_q: f64 = eq91_e1249;
        let eq91_e1251: f64 = (s.v[168] + eq91_e1249);
        let eq91_e1251_d_n2: f64 = (s.dn[168][2] + p.p355);
        let eq91_e1251_q: f64 = (eq91_e1246_q + eq91_e1250_q);
        (eq91_e1251, s.dn[168][0], s.dn[168][1], eq91_e1251_d_n2, s.dn[168][3], s.dn[168][4], s.dn[168][5], s.dn[168][6], s.dn[168][7], s.dn[168][8], __rspice_deriv_cse_4, s.dn[168][10], s.dn[168][11], s.dn[168][12], s.dn[168][13], s.dn[168][14], s.dn[168][15], s.dn[168][16], s.dn[168][17], s.dn[168][18], s.dn[168][19], s.dn[168][20], s.dn[168][21], s.dn[168][22], s.dn[168][23], s.dn[168][24], s.dn[168][25], s.dn[168][26], s.dn[168][27], s.dn[168][28], s.dn[168][29], s.db[168][0], s.db[168][1], s.db[168][2], s.db[168][3], s.db[168][4], s.db[168][5], s.db[168][6], s.db[168][7], s.db[168][8], s.db[168][9], s.db[168][10], s.db[168][11], s.db[168][12], s.db[168][13], s.db[168][14], s.db[168][15], s.db[168][16], s.db[168][17], s.db[168][18], s.db[168][19], s.db[168][20], s.db[168][21], s.db[168][22], s.db[168][23], s.db[168][24], s.db[168][25], s.db[168][26], s.db[168][27], s.db[168][28], s.db[168][29], s.db[168][30], s.db[168][31], s.db[168][32], s.db[168][33], s.db[168][34], s.db[168][35], eq91_e1251_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_reactive_node_derivatives: [f64; 30] = [eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29];
        let eq91_reactive_branch_derivatives: [f64; 36] = [eq91_e1253_d_b0, eq91_e1253_d_b1, eq91_e1253_d_b2, eq91_e1253_d_b3, eq91_e1253_d_b4, eq91_e1253_d_b5, eq91_e1253_d_b6, eq91_e1253_d_b7, eq91_e1253_d_b8, eq91_e1253_d_b9, eq91_e1253_d_b10, eq91_e1253_d_b11, eq91_e1253_d_b12, eq91_e1253_d_b13, eq91_e1253_d_b14, eq91_e1253_d_b15, eq91_e1253_d_b16, eq91_e1253_d_b17, eq91_e1253_d_b18, eq91_e1253_d_b19, eq91_e1253_d_b20, eq91_e1253_d_b21, eq91_e1253_d_b22, eq91_e1253_d_b23, eq91_e1253_d_b24, eq91_e1253_d_b25, eq91_e1253_d_b26, eq91_e1253_d_b27, eq91_e1253_d_b28, eq91_e1253_d_b29, eq91_e1253_d_b30, eq91_e1253_d_b31, eq91_e1253_d_b32, eq91_e1253_d_b33, eq91_e1253_d_b34, eq91_e1253_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[9]),
            nodes,
            &eq91_reactive_node_derivatives,
            branches,
            &eq91_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29, eq92_e1264_d_b0, eq92_e1264_d_b1, eq92_e1264_d_b2, eq92_e1264_d_b3, eq92_e1264_d_b4, eq92_e1264_d_b5, eq92_e1264_d_b6, eq92_e1264_d_b7, eq92_e1264_d_b8, eq92_e1264_d_b9, eq92_e1264_d_b10, eq92_e1264_d_b11, eq92_e1264_d_b12, eq92_e1264_d_b13, eq92_e1264_d_b14, eq92_e1264_d_b15, eq92_e1264_d_b16, eq92_e1264_d_b17, eq92_e1264_d_b18, eq92_e1264_d_b19, eq92_e1264_d_b20, eq92_e1264_d_b21, eq92_e1264_d_b22, eq92_e1264_d_b23, eq92_e1264_d_b24, eq92_e1264_d_b25, eq92_e1264_d_b26, eq92_e1264_d_b27, eq92_e1264_d_b28, eq92_e1264_d_b29, eq92_e1264_d_b30, eq92_e1264_d_b31, eq92_e1264_d_b32, eq92_e1264_d_b33, eq92_e1264_d_b34, eq92_e1264_d_b35, eq92_e1264_q,) = {
    if (!s.b[1054]) {
        let eq92_e1257_q: f64 = s.v[169];
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1261_q: f64 = eq92_e1260;
        let eq92_e1262: f64 = (s.v[169] + eq92_e1260);
        let eq92_e1262_d_n7: f64 = (s.dn[169][7] + p.p355);
        let eq92_e1262_q: f64 = (eq92_e1257_q + eq92_e1261_q);
        (eq92_e1262, s.dn[169][0], s.dn[169][1], s.dn[169][2], s.dn[169][3], s.dn[169][4], s.dn[169][5], s.dn[169][6], eq92_e1262_d_n7, s.dn[169][8], s.dn[169][9], __rspice_deriv_cse_5, s.dn[169][11], s.dn[169][12], s.dn[169][13], s.dn[169][14], s.dn[169][15], s.dn[169][16], s.dn[169][17], s.dn[169][18], s.dn[169][19], s.dn[169][20], s.dn[169][21], s.dn[169][22], s.dn[169][23], s.dn[169][24], s.dn[169][25], s.dn[169][26], s.dn[169][27], s.dn[169][28], s.dn[169][29], s.db[169][0], s.db[169][1], s.db[169][2], s.db[169][3], s.db[169][4], s.db[169][5], s.db[169][6], s.db[169][7], s.db[169][8], s.db[169][9], s.db[169][10], s.db[169][11], s.db[169][12], s.db[169][13], s.db[169][14], s.db[169][15], s.db[169][16], s.db[169][17], s.db[169][18], s.db[169][19], s.db[169][20], s.db[169][21], s.db[169][22], s.db[169][23], s.db[169][24], s.db[169][25], s.db[169][26], s.db[169][27], s.db[169][28], s.db[169][29], s.db[169][30], s.db[169][31], s.db[169][32], s.db[169][33], s.db[169][34], s.db[169][35], eq92_e1262_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_reactive_node_derivatives: [f64; 30] = [eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29];
        let eq92_reactive_branch_derivatives: [f64; 36] = [eq92_e1264_d_b0, eq92_e1264_d_b1, eq92_e1264_d_b2, eq92_e1264_d_b3, eq92_e1264_d_b4, eq92_e1264_d_b5, eq92_e1264_d_b6, eq92_e1264_d_b7, eq92_e1264_d_b8, eq92_e1264_d_b9, eq92_e1264_d_b10, eq92_e1264_d_b11, eq92_e1264_d_b12, eq92_e1264_d_b13, eq92_e1264_d_b14, eq92_e1264_d_b15, eq92_e1264_d_b16, eq92_e1264_d_b17, eq92_e1264_d_b18, eq92_e1264_d_b19, eq92_e1264_d_b20, eq92_e1264_d_b21, eq92_e1264_d_b22, eq92_e1264_d_b23, eq92_e1264_d_b24, eq92_e1264_d_b25, eq92_e1264_d_b26, eq92_e1264_d_b27, eq92_e1264_d_b28, eq92_e1264_d_b29, eq92_e1264_d_b30, eq92_e1264_d_b31, eq92_e1264_d_b32, eq92_e1264_d_b33, eq92_e1264_d_b34, eq92_e1264_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq92_reactive_node_derivatives,
            branches,
            &eq92_reactive_branch_derivatives,
            multiplicity,
        );
        let eq95_e1276_q: f64 = s.v[170];
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1280_q: f64 = eq95_e1279;
        let eq95_e1281: f64 = (s.v[170] + eq95_e1279);
        let eq95_e1281_d_n3: f64 = (s.dn[170][3] + p.p355);
        let eq95_e1281_d_n10: f64 = (s.dn[170][10] + (-p.p355));
        let eq95_e1281_q: f64 = (eq95_e1276_q + eq95_e1280_q);
        let eq95_reactive_node_derivatives: [f64; 30] = [s.dn[170][0], s.dn[170][1], s.dn[170][2], eq95_e1281_d_n3, s.dn[170][4], s.dn[170][5], s.dn[170][6], s.dn[170][7], s.dn[170][8], s.dn[170][9], eq95_e1281_d_n10, s.dn[170][11], s.dn[170][12], s.dn[170][13], s.dn[170][14], s.dn[170][15], s.dn[170][16], s.dn[170][17], s.dn[170][18], s.dn[170][19], s.dn[170][20], s.dn[170][21], s.dn[170][22], s.dn[170][23], s.dn[170][24], s.dn[170][25], s.dn[170][26], s.dn[170][27], s.dn[170][28], s.dn[170][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq95_reactive_node_derivatives,
            branches,
            &s.db[170],
            multiplicity,
        );
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29, eq98_e1304_d_b0, eq98_e1304_d_b1, eq98_e1304_d_b2, eq98_e1304_d_b3, eq98_e1304_d_b4, eq98_e1304_d_b5, eq98_e1304_d_b6, eq98_e1304_d_b7, eq98_e1304_d_b8, eq98_e1304_d_b9, eq98_e1304_d_b10, eq98_e1304_d_b11, eq98_e1304_d_b12, eq98_e1304_d_b13, eq98_e1304_d_b14, eq98_e1304_d_b15, eq98_e1304_d_b16, eq98_e1304_d_b17, eq98_e1304_d_b18, eq98_e1304_d_b19, eq98_e1304_d_b20, eq98_e1304_d_b21, eq98_e1304_d_b22, eq98_e1304_d_b23, eq98_e1304_d_b24, eq98_e1304_d_b25, eq98_e1304_d_b26, eq98_e1304_d_b27, eq98_e1304_d_b28, eq98_e1304_d_b29, eq98_e1304_d_b30, eq98_e1304_d_b31, eq98_e1304_d_b32, eq98_e1304_d_b33, eq98_e1304_d_b34, eq98_e1304_d_b35, eq98_e1304_q,) = {
    if s.b[1201] {
        let eq98_e1297_q: f64 = s.v[173];
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1301_q: f64 = eq98_e1300;
        let eq98_e1302: f64 = (s.v[173] + eq98_e1300);
        let eq98_e1302_d_n7: f64 = (s.dn[173][7] + p.p355);
        let eq98_e1302_d_n11: f64 = (s.dn[173][11] + (-p.p355));
        let eq98_e1302_q: f64 = (eq98_e1297_q + eq98_e1301_q);
        (eq98_e1302, s.dn[173][0], s.dn[173][1], s.dn[173][2], s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], eq98_e1302_d_n7, s.dn[173][8], s.dn[173][9], s.dn[173][10], eq98_e1302_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29], s.db[173][0], s.db[173][1], s.db[173][2], s.db[173][3], s.db[173][4], s.db[173][5], s.db[173][6], s.db[173][7], s.db[173][8], s.db[173][9], s.db[173][10], s.db[173][11], s.db[173][12], s.db[173][13], s.db[173][14], s.db[173][15], s.db[173][16], s.db[173][17], s.db[173][18], s.db[173][19], s.db[173][20], s.db[173][21], s.db[173][22], s.db[173][23], s.db[173][24], s.db[173][25], s.db[173][26], s.db[173][27], s.db[173][28], s.db[173][29], s.db[173][30], s.db[173][31], s.db[173][32], s.db[173][33], s.db[173][34], s.db[173][35], eq98_e1302_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_reactive_node_derivatives: [f64; 30] = [eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29];
        let eq98_reactive_branch_derivatives: [f64; 36] = [eq98_e1304_d_b0, eq98_e1304_d_b1, eq98_e1304_d_b2, eq98_e1304_d_b3, eq98_e1304_d_b4, eq98_e1304_d_b5, eq98_e1304_d_b6, eq98_e1304_d_b7, eq98_e1304_d_b8, eq98_e1304_d_b9, eq98_e1304_d_b10, eq98_e1304_d_b11, eq98_e1304_d_b12, eq98_e1304_d_b13, eq98_e1304_d_b14, eq98_e1304_d_b15, eq98_e1304_d_b16, eq98_e1304_d_b17, eq98_e1304_d_b18, eq98_e1304_d_b19, eq98_e1304_d_b20, eq98_e1304_d_b21, eq98_e1304_d_b22, eq98_e1304_d_b23, eq98_e1304_d_b24, eq98_e1304_d_b25, eq98_e1304_d_b26, eq98_e1304_d_b27, eq98_e1304_d_b28, eq98_e1304_d_b29, eq98_e1304_d_b30, eq98_e1304_d_b31, eq98_e1304_d_b32, eq98_e1304_d_b33, eq98_e1304_d_b34, eq98_e1304_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq98_reactive_node_derivatives,
            branches,
            &eq98_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq99_e1314, eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29, eq99_e1314_d_b0, eq99_e1314_d_b1, eq99_e1314_d_b2, eq99_e1314_d_b3, eq99_e1314_d_b4, eq99_e1314_d_b5, eq99_e1314_d_b6, eq99_e1314_d_b7, eq99_e1314_d_b8, eq99_e1314_d_b9, eq99_e1314_d_b10, eq99_e1314_d_b11, eq99_e1314_d_b12, eq99_e1314_d_b13, eq99_e1314_d_b14, eq99_e1314_d_b15, eq99_e1314_d_b16, eq99_e1314_d_b17, eq99_e1314_d_b18, eq99_e1314_d_b19, eq99_e1314_d_b20, eq99_e1314_d_b21, eq99_e1314_d_b22, eq99_e1314_d_b23, eq99_e1314_d_b24, eq99_e1314_d_b25, eq99_e1314_d_b26, eq99_e1314_d_b27, eq99_e1314_d_b28, eq99_e1314_d_b29, eq99_e1314_d_b30, eq99_e1314_d_b31, eq99_e1314_d_b32, eq99_e1314_d_b33, eq99_e1314_d_b34, eq99_e1314_d_b35, eq99_e1314_q,) = {
    if s.b[1201] {
        let eq99_e1307_q: f64 = s.v[174];
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1311_q: f64 = eq99_e1310;
        let eq99_e1312: f64 = (s.v[174] + eq99_e1310);
        let eq99_e1312_d_n7: f64 = (s.dn[174][7] + p.p355);
        let eq99_e1312_d_n10: f64 = (s.dn[174][10] + (-p.p355));
        let eq99_e1312_q: f64 = (eq99_e1307_q + eq99_e1311_q);
        (eq99_e1312, s.dn[174][0], s.dn[174][1], s.dn[174][2], s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], eq99_e1312_d_n7, s.dn[174][8], s.dn[174][9], eq99_e1312_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29], s.db[174][0], s.db[174][1], s.db[174][2], s.db[174][3], s.db[174][4], s.db[174][5], s.db[174][6], s.db[174][7], s.db[174][8], s.db[174][9], s.db[174][10], s.db[174][11], s.db[174][12], s.db[174][13], s.db[174][14], s.db[174][15], s.db[174][16], s.db[174][17], s.db[174][18], s.db[174][19], s.db[174][20], s.db[174][21], s.db[174][22], s.db[174][23], s.db[174][24], s.db[174][25], s.db[174][26], s.db[174][27], s.db[174][28], s.db[174][29], s.db[174][30], s.db[174][31], s.db[174][32], s.db[174][33], s.db[174][34], s.db[174][35], eq99_e1312_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_reactive_node_derivatives: [f64; 30] = [eq99_e1314_d_n0, eq99_e1314_d_n1, eq99_e1314_d_n2, eq99_e1314_d_n3, eq99_e1314_d_n4, eq99_e1314_d_n5, eq99_e1314_d_n6, eq99_e1314_d_n7, eq99_e1314_d_n8, eq99_e1314_d_n9, eq99_e1314_d_n10, eq99_e1314_d_n11, eq99_e1314_d_n12, eq99_e1314_d_n13, eq99_e1314_d_n14, eq99_e1314_d_n15, eq99_e1314_d_n16, eq99_e1314_d_n17, eq99_e1314_d_n18, eq99_e1314_d_n19, eq99_e1314_d_n20, eq99_e1314_d_n21, eq99_e1314_d_n22, eq99_e1314_d_n23, eq99_e1314_d_n24, eq99_e1314_d_n25, eq99_e1314_d_n26, eq99_e1314_d_n27, eq99_e1314_d_n28, eq99_e1314_d_n29];
        let eq99_reactive_branch_derivatives: [f64; 36] = [eq99_e1314_d_b0, eq99_e1314_d_b1, eq99_e1314_d_b2, eq99_e1314_d_b3, eq99_e1314_d_b4, eq99_e1314_d_b5, eq99_e1314_d_b6, eq99_e1314_d_b7, eq99_e1314_d_b8, eq99_e1314_d_b9, eq99_e1314_d_b10, eq99_e1314_d_b11, eq99_e1314_d_b12, eq99_e1314_d_b13, eq99_e1314_d_b14, eq99_e1314_d_b15, eq99_e1314_d_b16, eq99_e1314_d_b17, eq99_e1314_d_b18, eq99_e1314_d_b19, eq99_e1314_d_b20, eq99_e1314_d_b21, eq99_e1314_d_b22, eq99_e1314_d_b23, eq99_e1314_d_b24, eq99_e1314_d_b25, eq99_e1314_d_b26, eq99_e1314_d_b27, eq99_e1314_d_b28, eq99_e1314_d_b29, eq99_e1314_d_b30, eq99_e1314_d_b31, eq99_e1314_d_b32, eq99_e1314_d_b33, eq99_e1314_d_b34, eq99_e1314_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq99_reactive_node_derivatives,
            branches,
            &eq99_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let __rspice_deriv_cse_0: f64 = (s.dn[175][11] + (-p.p355));
        let __rspice_deriv_cse_1: f64 = (s.dn[179][12] + (-p.p355));
        let __rspice_deriv_cse_2: f64 = (s.dn[180][11] + (-p.p355));
        let __rspice_deriv_cse_3: f64 = (s.dn[181][12] + (-p.p355));
        let __rspice_deriv_cse_4: f64 = (s.dn[185][13] + (-p.p355));
        let __rspice_deriv_cse_5: f64 = (s.dn[186][12] + (-p.p355));
        let __rspice_deriv_cse_6: f64 = (s.dn[187][13] + (-p.p355));
        let (eq100_e1324, eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29, eq100_e1324_d_b0, eq100_e1324_d_b1, eq100_e1324_d_b2, eq100_e1324_d_b3, eq100_e1324_d_b4, eq100_e1324_d_b5, eq100_e1324_d_b6, eq100_e1324_d_b7, eq100_e1324_d_b8, eq100_e1324_d_b9, eq100_e1324_d_b10, eq100_e1324_d_b11, eq100_e1324_d_b12, eq100_e1324_d_b13, eq100_e1324_d_b14, eq100_e1324_d_b15, eq100_e1324_d_b16, eq100_e1324_d_b17, eq100_e1324_d_b18, eq100_e1324_d_b19, eq100_e1324_d_b20, eq100_e1324_d_b21, eq100_e1324_d_b22, eq100_e1324_d_b23, eq100_e1324_d_b24, eq100_e1324_d_b25, eq100_e1324_d_b26, eq100_e1324_d_b27, eq100_e1324_d_b28, eq100_e1324_d_b29, eq100_e1324_d_b30, eq100_e1324_d_b31, eq100_e1324_d_b32, eq100_e1324_d_b33, eq100_e1324_d_b34, eq100_e1324_d_b35, eq100_e1324_q,) = {
    if s.b[1201] {
        let eq100_e1317_q: f64 = s.v[175];
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1321_q: f64 = eq100_e1320;
        let eq100_e1322: f64 = (s.v[175] + eq100_e1320);
        let eq100_e1322_d_n2: f64 = (s.dn[175][2] + p.p355);
        let eq100_e1322_q: f64 = (eq100_e1317_q + eq100_e1321_q);
        (eq100_e1322, s.dn[175][0], s.dn[175][1], eq100_e1322_d_n2, s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], s.dn[175][7], s.dn[175][8], s.dn[175][9], s.dn[175][10], __rspice_deriv_cse_0, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29], s.db[175][0], s.db[175][1], s.db[175][2], s.db[175][3], s.db[175][4], s.db[175][5], s.db[175][6], s.db[175][7], s.db[175][8], s.db[175][9], s.db[175][10], s.db[175][11], s.db[175][12], s.db[175][13], s.db[175][14], s.db[175][15], s.db[175][16], s.db[175][17], s.db[175][18], s.db[175][19], s.db[175][20], s.db[175][21], s.db[175][22], s.db[175][23], s.db[175][24], s.db[175][25], s.db[175][26], s.db[175][27], s.db[175][28], s.db[175][29], s.db[175][30], s.db[175][31], s.db[175][32], s.db[175][33], s.db[175][34], s.db[175][35], eq100_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_reactive_node_derivatives: [f64; 30] = [eq100_e1324_d_n0, eq100_e1324_d_n1, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n5, eq100_e1324_d_n6, eq100_e1324_d_n7, eq100_e1324_d_n8, eq100_e1324_d_n9, eq100_e1324_d_n10, eq100_e1324_d_n11, eq100_e1324_d_n12, eq100_e1324_d_n13, eq100_e1324_d_n14, eq100_e1324_d_n15, eq100_e1324_d_n16, eq100_e1324_d_n17, eq100_e1324_d_n18, eq100_e1324_d_n19, eq100_e1324_d_n20, eq100_e1324_d_n21, eq100_e1324_d_n22, eq100_e1324_d_n23, eq100_e1324_d_n24, eq100_e1324_d_n25, eq100_e1324_d_n26, eq100_e1324_d_n27, eq100_e1324_d_n28, eq100_e1324_d_n29];
        let eq100_reactive_branch_derivatives: [f64; 36] = [eq100_e1324_d_b0, eq100_e1324_d_b1, eq100_e1324_d_b2, eq100_e1324_d_b3, eq100_e1324_d_b4, eq100_e1324_d_b5, eq100_e1324_d_b6, eq100_e1324_d_b7, eq100_e1324_d_b8, eq100_e1324_d_b9, eq100_e1324_d_b10, eq100_e1324_d_b11, eq100_e1324_d_b12, eq100_e1324_d_b13, eq100_e1324_d_b14, eq100_e1324_d_b15, eq100_e1324_d_b16, eq100_e1324_d_b17, eq100_e1324_d_b18, eq100_e1324_d_b19, eq100_e1324_d_b20, eq100_e1324_d_b21, eq100_e1324_d_b22, eq100_e1324_d_b23, eq100_e1324_d_b24, eq100_e1324_d_b25, eq100_e1324_d_b26, eq100_e1324_d_b27, eq100_e1324_d_b28, eq100_e1324_d_b29, eq100_e1324_d_b30, eq100_e1324_d_b31, eq100_e1324_d_b32, eq100_e1324_d_b33, eq100_e1324_d_b34, eq100_e1324_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq100_reactive_node_derivatives,
            branches,
            &eq100_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29, eq102_e1338_d_b0, eq102_e1338_d_b1, eq102_e1338_d_b2, eq102_e1338_d_b3, eq102_e1338_d_b4, eq102_e1338_d_b5, eq102_e1338_d_b6, eq102_e1338_d_b7, eq102_e1338_d_b8, eq102_e1338_d_b9, eq102_e1338_d_b10, eq102_e1338_d_b11, eq102_e1338_d_b12, eq102_e1338_d_b13, eq102_e1338_d_b14, eq102_e1338_d_b15, eq102_e1338_d_b16, eq102_e1338_d_b17, eq102_e1338_d_b18, eq102_e1338_d_b19, eq102_e1338_d_b20, eq102_e1338_d_b21, eq102_e1338_d_b22, eq102_e1338_d_b23, eq102_e1338_d_b24, eq102_e1338_d_b25, eq102_e1338_d_b26, eq102_e1338_d_b27, eq102_e1338_d_b28, eq102_e1338_d_b29, eq102_e1338_d_b30, eq102_e1338_d_b31, eq102_e1338_d_b32, eq102_e1338_d_b33, eq102_e1338_d_b34, eq102_e1338_d_b35, eq102_e1338_q,) = {
    if s.b[1201] {
        let eq102_e1331_q: f64 = s.v[177];
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1335_q: f64 = eq102_e1334;
        let eq102_e1336: f64 = (s.v[177] + eq102_e1334);
        let eq102_e1336_d_n7: f64 = (s.dn[177][7] + p.p355);
        let eq102_e1336_d_n9: f64 = (s.dn[177][9] + (-p.p355));
        let eq102_e1336_q: f64 = (eq102_e1331_q + eq102_e1335_q);
        (eq102_e1336, s.dn[177][0], s.dn[177][1], s.dn[177][2], s.dn[177][3], s.dn[177][4], s.dn[177][5], s.dn[177][6], eq102_e1336_d_n7, s.dn[177][8], eq102_e1336_d_n9, s.dn[177][10], s.dn[177][11], s.dn[177][12], s.dn[177][13], s.dn[177][14], s.dn[177][15], s.dn[177][16], s.dn[177][17], s.dn[177][18], s.dn[177][19], s.dn[177][20], s.dn[177][21], s.dn[177][22], s.dn[177][23], s.dn[177][24], s.dn[177][25], s.dn[177][26], s.dn[177][27], s.dn[177][28], s.dn[177][29], s.db[177][0], s.db[177][1], s.db[177][2], s.db[177][3], s.db[177][4], s.db[177][5], s.db[177][6], s.db[177][7], s.db[177][8], s.db[177][9], s.db[177][10], s.db[177][11], s.db[177][12], s.db[177][13], s.db[177][14], s.db[177][15], s.db[177][16], s.db[177][17], s.db[177][18], s.db[177][19], s.db[177][20], s.db[177][21], s.db[177][22], s.db[177][23], s.db[177][24], s.db[177][25], s.db[177][26], s.db[177][27], s.db[177][28], s.db[177][29], s.db[177][30], s.db[177][31], s.db[177][32], s.db[177][33], s.db[177][34], s.db[177][35], eq102_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_reactive_node_derivatives: [f64; 30] = [eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29];
        let eq102_reactive_branch_derivatives: [f64; 36] = [eq102_e1338_d_b0, eq102_e1338_d_b1, eq102_e1338_d_b2, eq102_e1338_d_b3, eq102_e1338_d_b4, eq102_e1338_d_b5, eq102_e1338_d_b6, eq102_e1338_d_b7, eq102_e1338_d_b8, eq102_e1338_d_b9, eq102_e1338_d_b10, eq102_e1338_d_b11, eq102_e1338_d_b12, eq102_e1338_d_b13, eq102_e1338_d_b14, eq102_e1338_d_b15, eq102_e1338_d_b16, eq102_e1338_d_b17, eq102_e1338_d_b18, eq102_e1338_d_b19, eq102_e1338_d_b20, eq102_e1338_d_b21, eq102_e1338_d_b22, eq102_e1338_d_b23, eq102_e1338_d_b24, eq102_e1338_d_b25, eq102_e1338_d_b26, eq102_e1338_d_b27, eq102_e1338_d_b28, eq102_e1338_d_b29, eq102_e1338_d_b30, eq102_e1338_d_b31, eq102_e1338_d_b32, eq102_e1338_d_b33, eq102_e1338_d_b34, eq102_e1338_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq102_reactive_node_derivatives,
            branches,
            &eq102_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1349, eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29, eq103_e1349_d_b0, eq103_e1349_d_b1, eq103_e1349_d_b2, eq103_e1349_d_b3, eq103_e1349_d_b4, eq103_e1349_d_b5, eq103_e1349_d_b6, eq103_e1349_d_b7, eq103_e1349_d_b8, eq103_e1349_d_b9, eq103_e1349_d_b10, eq103_e1349_d_b11, eq103_e1349_d_b12, eq103_e1349_d_b13, eq103_e1349_d_b14, eq103_e1349_d_b15, eq103_e1349_d_b16, eq103_e1349_d_b17, eq103_e1349_d_b18, eq103_e1349_d_b19, eq103_e1349_d_b20, eq103_e1349_d_b21, eq103_e1349_d_b22, eq103_e1349_d_b23, eq103_e1349_d_b24, eq103_e1349_d_b25, eq103_e1349_d_b26, eq103_e1349_d_b27, eq103_e1349_d_b28, eq103_e1349_d_b29, eq103_e1349_d_b30, eq103_e1349_d_b31, eq103_e1349_d_b32, eq103_e1349_d_b33, eq103_e1349_d_b34, eq103_e1349_d_b35, eq103_e1349_q,) = {
    if (!s.b[1201]) {
        let eq103_e1342_q: f64 = s.v[173];
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1346_q: f64 = eq103_e1345;
        let eq103_e1347: f64 = (s.v[173] + eq103_e1345);
        let eq103_e1347_d_n2: f64 = (s.dn[173][2] + p.p355);
        let eq103_e1347_d_n11: f64 = (s.dn[173][11] + (-p.p355));
        let eq103_e1347_q: f64 = (eq103_e1342_q + eq103_e1346_q);
        (eq103_e1347, s.dn[173][0], s.dn[173][1], eq103_e1347_d_n2, s.dn[173][3], s.dn[173][4], s.dn[173][5], s.dn[173][6], s.dn[173][7], s.dn[173][8], s.dn[173][9], s.dn[173][10], eq103_e1347_d_n11, s.dn[173][12], s.dn[173][13], s.dn[173][14], s.dn[173][15], s.dn[173][16], s.dn[173][17], s.dn[173][18], s.dn[173][19], s.dn[173][20], s.dn[173][21], s.dn[173][22], s.dn[173][23], s.dn[173][24], s.dn[173][25], s.dn[173][26], s.dn[173][27], s.dn[173][28], s.dn[173][29], s.db[173][0], s.db[173][1], s.db[173][2], s.db[173][3], s.db[173][4], s.db[173][5], s.db[173][6], s.db[173][7], s.db[173][8], s.db[173][9], s.db[173][10], s.db[173][11], s.db[173][12], s.db[173][13], s.db[173][14], s.db[173][15], s.db[173][16], s.db[173][17], s.db[173][18], s.db[173][19], s.db[173][20], s.db[173][21], s.db[173][22], s.db[173][23], s.db[173][24], s.db[173][25], s.db[173][26], s.db[173][27], s.db[173][28], s.db[173][29], s.db[173][30], s.db[173][31], s.db[173][32], s.db[173][33], s.db[173][34], s.db[173][35], eq103_e1347_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_reactive_node_derivatives: [f64; 30] = [eq103_e1349_d_n0, eq103_e1349_d_n1, eq103_e1349_d_n2, eq103_e1349_d_n3, eq103_e1349_d_n4, eq103_e1349_d_n5, eq103_e1349_d_n6, eq103_e1349_d_n7, eq103_e1349_d_n8, eq103_e1349_d_n9, eq103_e1349_d_n10, eq103_e1349_d_n11, eq103_e1349_d_n12, eq103_e1349_d_n13, eq103_e1349_d_n14, eq103_e1349_d_n15, eq103_e1349_d_n16, eq103_e1349_d_n17, eq103_e1349_d_n18, eq103_e1349_d_n19, eq103_e1349_d_n20, eq103_e1349_d_n21, eq103_e1349_d_n22, eq103_e1349_d_n23, eq103_e1349_d_n24, eq103_e1349_d_n25, eq103_e1349_d_n26, eq103_e1349_d_n27, eq103_e1349_d_n28, eq103_e1349_d_n29];
        let eq103_reactive_branch_derivatives: [f64; 36] = [eq103_e1349_d_b0, eq103_e1349_d_b1, eq103_e1349_d_b2, eq103_e1349_d_b3, eq103_e1349_d_b4, eq103_e1349_d_b5, eq103_e1349_d_b6, eq103_e1349_d_b7, eq103_e1349_d_b8, eq103_e1349_d_b9, eq103_e1349_d_b10, eq103_e1349_d_b11, eq103_e1349_d_b12, eq103_e1349_d_b13, eq103_e1349_d_b14, eq103_e1349_d_b15, eq103_e1349_d_b16, eq103_e1349_d_b17, eq103_e1349_d_b18, eq103_e1349_d_b19, eq103_e1349_d_b20, eq103_e1349_d_b21, eq103_e1349_d_b22, eq103_e1349_d_b23, eq103_e1349_d_b24, eq103_e1349_d_b25, eq103_e1349_d_b26, eq103_e1349_d_b27, eq103_e1349_d_b28, eq103_e1349_d_b29, eq103_e1349_d_b30, eq103_e1349_d_b31, eq103_e1349_d_b32, eq103_e1349_d_b33, eq103_e1349_d_b34, eq103_e1349_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq103_reactive_node_derivatives,
            branches,
            &eq103_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29, eq104_e1360_d_b0, eq104_e1360_d_b1, eq104_e1360_d_b2, eq104_e1360_d_b3, eq104_e1360_d_b4, eq104_e1360_d_b5, eq104_e1360_d_b6, eq104_e1360_d_b7, eq104_e1360_d_b8, eq104_e1360_d_b9, eq104_e1360_d_b10, eq104_e1360_d_b11, eq104_e1360_d_b12, eq104_e1360_d_b13, eq104_e1360_d_b14, eq104_e1360_d_b15, eq104_e1360_d_b16, eq104_e1360_d_b17, eq104_e1360_d_b18, eq104_e1360_d_b19, eq104_e1360_d_b20, eq104_e1360_d_b21, eq104_e1360_d_b22, eq104_e1360_d_b23, eq104_e1360_d_b24, eq104_e1360_d_b25, eq104_e1360_d_b26, eq104_e1360_d_b27, eq104_e1360_d_b28, eq104_e1360_d_b29, eq104_e1360_d_b30, eq104_e1360_d_b31, eq104_e1360_d_b32, eq104_e1360_d_b33, eq104_e1360_d_b34, eq104_e1360_d_b35, eq104_e1360_q,) = {
    if (!s.b[1201]) {
        let eq104_e1353_q: f64 = s.v[174];
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1357_q: f64 = eq104_e1356;
        let eq104_e1358: f64 = (s.v[174] + eq104_e1356);
        let eq104_e1358_d_n2: f64 = (s.dn[174][2] + p.p355);
        let eq104_e1358_d_n10: f64 = (s.dn[174][10] + (-p.p355));
        let eq104_e1358_q: f64 = (eq104_e1353_q + eq104_e1357_q);
        (eq104_e1358, s.dn[174][0], s.dn[174][1], eq104_e1358_d_n2, s.dn[174][3], s.dn[174][4], s.dn[174][5], s.dn[174][6], s.dn[174][7], s.dn[174][8], s.dn[174][9], eq104_e1358_d_n10, s.dn[174][11], s.dn[174][12], s.dn[174][13], s.dn[174][14], s.dn[174][15], s.dn[174][16], s.dn[174][17], s.dn[174][18], s.dn[174][19], s.dn[174][20], s.dn[174][21], s.dn[174][22], s.dn[174][23], s.dn[174][24], s.dn[174][25], s.dn[174][26], s.dn[174][27], s.dn[174][28], s.dn[174][29], s.db[174][0], s.db[174][1], s.db[174][2], s.db[174][3], s.db[174][4], s.db[174][5], s.db[174][6], s.db[174][7], s.db[174][8], s.db[174][9], s.db[174][10], s.db[174][11], s.db[174][12], s.db[174][13], s.db[174][14], s.db[174][15], s.db[174][16], s.db[174][17], s.db[174][18], s.db[174][19], s.db[174][20], s.db[174][21], s.db[174][22], s.db[174][23], s.db[174][24], s.db[174][25], s.db[174][26], s.db[174][27], s.db[174][28], s.db[174][29], s.db[174][30], s.db[174][31], s.db[174][32], s.db[174][33], s.db[174][34], s.db[174][35], eq104_e1358_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_reactive_node_derivatives: [f64; 30] = [eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29];
        let eq104_reactive_branch_derivatives: [f64; 36] = [eq104_e1360_d_b0, eq104_e1360_d_b1, eq104_e1360_d_b2, eq104_e1360_d_b3, eq104_e1360_d_b4, eq104_e1360_d_b5, eq104_e1360_d_b6, eq104_e1360_d_b7, eq104_e1360_d_b8, eq104_e1360_d_b9, eq104_e1360_d_b10, eq104_e1360_d_b11, eq104_e1360_d_b12, eq104_e1360_d_b13, eq104_e1360_d_b14, eq104_e1360_d_b15, eq104_e1360_d_b16, eq104_e1360_d_b17, eq104_e1360_d_b18, eq104_e1360_d_b19, eq104_e1360_d_b20, eq104_e1360_d_b21, eq104_e1360_d_b22, eq104_e1360_d_b23, eq104_e1360_d_b24, eq104_e1360_d_b25, eq104_e1360_d_b26, eq104_e1360_d_b27, eq104_e1360_d_b28, eq104_e1360_d_b29, eq104_e1360_d_b30, eq104_e1360_d_b31, eq104_e1360_d_b32, eq104_e1360_d_b33, eq104_e1360_d_b34, eq104_e1360_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[10]),
            nodes,
            &eq104_reactive_node_derivatives,
            branches,
            &eq104_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq105_e1371, eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29, eq105_e1371_d_b0, eq105_e1371_d_b1, eq105_e1371_d_b2, eq105_e1371_d_b3, eq105_e1371_d_b4, eq105_e1371_d_b5, eq105_e1371_d_b6, eq105_e1371_d_b7, eq105_e1371_d_b8, eq105_e1371_d_b9, eq105_e1371_d_b10, eq105_e1371_d_b11, eq105_e1371_d_b12, eq105_e1371_d_b13, eq105_e1371_d_b14, eq105_e1371_d_b15, eq105_e1371_d_b16, eq105_e1371_d_b17, eq105_e1371_d_b18, eq105_e1371_d_b19, eq105_e1371_d_b20, eq105_e1371_d_b21, eq105_e1371_d_b22, eq105_e1371_d_b23, eq105_e1371_d_b24, eq105_e1371_d_b25, eq105_e1371_d_b26, eq105_e1371_d_b27, eq105_e1371_d_b28, eq105_e1371_d_b29, eq105_e1371_d_b30, eq105_e1371_d_b31, eq105_e1371_d_b32, eq105_e1371_d_b33, eq105_e1371_d_b34, eq105_e1371_d_b35, eq105_e1371_q,) = {
    if (!s.b[1201]) {
        let eq105_e1364_q: f64 = s.v[175];
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1368_q: f64 = eq105_e1367;
        let eq105_e1369: f64 = (s.v[175] + eq105_e1367);
        let eq105_e1369_d_n7: f64 = (s.dn[175][7] + p.p355);
        let eq105_e1369_q: f64 = (eq105_e1364_q + eq105_e1368_q);
        (eq105_e1369, s.dn[175][0], s.dn[175][1], s.dn[175][2], s.dn[175][3], s.dn[175][4], s.dn[175][5], s.dn[175][6], eq105_e1369_d_n7, s.dn[175][8], s.dn[175][9], s.dn[175][10], __rspice_deriv_cse_0, s.dn[175][12], s.dn[175][13], s.dn[175][14], s.dn[175][15], s.dn[175][16], s.dn[175][17], s.dn[175][18], s.dn[175][19], s.dn[175][20], s.dn[175][21], s.dn[175][22], s.dn[175][23], s.dn[175][24], s.dn[175][25], s.dn[175][26], s.dn[175][27], s.dn[175][28], s.dn[175][29], s.db[175][0], s.db[175][1], s.db[175][2], s.db[175][3], s.db[175][4], s.db[175][5], s.db[175][6], s.db[175][7], s.db[175][8], s.db[175][9], s.db[175][10], s.db[175][11], s.db[175][12], s.db[175][13], s.db[175][14], s.db[175][15], s.db[175][16], s.db[175][17], s.db[175][18], s.db[175][19], s.db[175][20], s.db[175][21], s.db[175][22], s.db[175][23], s.db[175][24], s.db[175][25], s.db[175][26], s.db[175][27], s.db[175][28], s.db[175][29], s.db[175][30], s.db[175][31], s.db[175][32], s.db[175][33], s.db[175][34], s.db[175][35], eq105_e1369_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_reactive_node_derivatives: [f64; 30] = [eq105_e1371_d_n0, eq105_e1371_d_n1, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n5, eq105_e1371_d_n6, eq105_e1371_d_n7, eq105_e1371_d_n8, eq105_e1371_d_n9, eq105_e1371_d_n10, eq105_e1371_d_n11, eq105_e1371_d_n12, eq105_e1371_d_n13, eq105_e1371_d_n14, eq105_e1371_d_n15, eq105_e1371_d_n16, eq105_e1371_d_n17, eq105_e1371_d_n18, eq105_e1371_d_n19, eq105_e1371_d_n20, eq105_e1371_d_n21, eq105_e1371_d_n22, eq105_e1371_d_n23, eq105_e1371_d_n24, eq105_e1371_d_n25, eq105_e1371_d_n26, eq105_e1371_d_n27, eq105_e1371_d_n28, eq105_e1371_d_n29];
        let eq105_reactive_branch_derivatives: [f64; 36] = [eq105_e1371_d_b0, eq105_e1371_d_b1, eq105_e1371_d_b2, eq105_e1371_d_b3, eq105_e1371_d_b4, eq105_e1371_d_b5, eq105_e1371_d_b6, eq105_e1371_d_b7, eq105_e1371_d_b8, eq105_e1371_d_b9, eq105_e1371_d_b10, eq105_e1371_d_b11, eq105_e1371_d_b12, eq105_e1371_d_b13, eq105_e1371_d_b14, eq105_e1371_d_b15, eq105_e1371_d_b16, eq105_e1371_d_b17, eq105_e1371_d_b18, eq105_e1371_d_b19, eq105_e1371_d_b20, eq105_e1371_d_b21, eq105_e1371_d_b22, eq105_e1371_d_b23, eq105_e1371_d_b24, eq105_e1371_d_b25, eq105_e1371_d_b26, eq105_e1371_d_b27, eq105_e1371_d_b28, eq105_e1371_d_b29, eq105_e1371_d_b30, eq105_e1371_d_b31, eq105_e1371_d_b32, eq105_e1371_d_b33, eq105_e1371_d_b34, eq105_e1371_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq105_reactive_node_derivatives,
            branches,
            &eq105_reactive_branch_derivatives,
            multiplicity,
        );
        let eq108_e1383_q: f64 = s.v[176];
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1387_q: f64 = eq108_e1386;
        let eq108_e1388: f64 = (s.v[176] + eq108_e1386);
        let eq108_e1388_d_n3: f64 = (s.dn[176][3] + p.p355);
        let eq108_e1388_d_n11: f64 = (s.dn[176][11] + (-p.p355));
        let eq108_e1388_q: f64 = (eq108_e1383_q + eq108_e1387_q);
        let eq108_reactive_node_derivatives: [f64; 30] = [s.dn[176][0], s.dn[176][1], s.dn[176][2], eq108_e1388_d_n3, s.dn[176][4], s.dn[176][5], s.dn[176][6], s.dn[176][7], s.dn[176][8], s.dn[176][9], s.dn[176][10], eq108_e1388_d_n11, s.dn[176][12], s.dn[176][13], s.dn[176][14], s.dn[176][15], s.dn[176][16], s.dn[176][17], s.dn[176][18], s.dn[176][19], s.dn[176][20], s.dn[176][21], s.dn[176][22], s.dn[176][23], s.dn[176][24], s.dn[176][25], s.dn[176][26], s.dn[176][27], s.dn[176][28], s.dn[176][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[11]),
            nodes,
            &eq108_reactive_node_derivatives,
            branches,
            &s.db[176],
            multiplicity,
        );
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29, eq111_e1411_d_b0, eq111_e1411_d_b1, eq111_e1411_d_b2, eq111_e1411_d_b3, eq111_e1411_d_b4, eq111_e1411_d_b5, eq111_e1411_d_b6, eq111_e1411_d_b7, eq111_e1411_d_b8, eq111_e1411_d_b9, eq111_e1411_d_b10, eq111_e1411_d_b11, eq111_e1411_d_b12, eq111_e1411_d_b13, eq111_e1411_d_b14, eq111_e1411_d_b15, eq111_e1411_d_b16, eq111_e1411_d_b17, eq111_e1411_d_b18, eq111_e1411_d_b19, eq111_e1411_d_b20, eq111_e1411_d_b21, eq111_e1411_d_b22, eq111_e1411_d_b23, eq111_e1411_d_b24, eq111_e1411_d_b25, eq111_e1411_d_b26, eq111_e1411_d_b27, eq111_e1411_d_b28, eq111_e1411_d_b29, eq111_e1411_d_b30, eq111_e1411_d_b31, eq111_e1411_d_b32, eq111_e1411_d_b33, eq111_e1411_d_b34, eq111_e1411_d_b35, eq111_e1411_q,) = {
    if s.b[1348] {
        let eq111_e1404_q: f64 = s.v[179];
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1408_q: f64 = eq111_e1407;
        let eq111_e1409: f64 = (s.v[179] + eq111_e1407);
        let eq111_e1409_d_n7: f64 = (s.dn[179][7] + p.p355);
        let eq111_e1409_q: f64 = (eq111_e1404_q + eq111_e1408_q);
        (eq111_e1409, s.dn[179][0], s.dn[179][1], s.dn[179][2], s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], eq111_e1409_d_n7, s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], __rspice_deriv_cse_1, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29], s.db[179][0], s.db[179][1], s.db[179][2], s.db[179][3], s.db[179][4], s.db[179][5], s.db[179][6], s.db[179][7], s.db[179][8], s.db[179][9], s.db[179][10], s.db[179][11], s.db[179][12], s.db[179][13], s.db[179][14], s.db[179][15], s.db[179][16], s.db[179][17], s.db[179][18], s.db[179][19], s.db[179][20], s.db[179][21], s.db[179][22], s.db[179][23], s.db[179][24], s.db[179][25], s.db[179][26], s.db[179][27], s.db[179][28], s.db[179][29], s.db[179][30], s.db[179][31], s.db[179][32], s.db[179][33], s.db[179][34], s.db[179][35], eq111_e1409_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 30] = [eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29];
        let eq111_reactive_branch_derivatives: [f64; 36] = [eq111_e1411_d_b0, eq111_e1411_d_b1, eq111_e1411_d_b2, eq111_e1411_d_b3, eq111_e1411_d_b4, eq111_e1411_d_b5, eq111_e1411_d_b6, eq111_e1411_d_b7, eq111_e1411_d_b8, eq111_e1411_d_b9, eq111_e1411_d_b10, eq111_e1411_d_b11, eq111_e1411_d_b12, eq111_e1411_d_b13, eq111_e1411_d_b14, eq111_e1411_d_b15, eq111_e1411_d_b16, eq111_e1411_d_b17, eq111_e1411_d_b18, eq111_e1411_d_b19, eq111_e1411_d_b20, eq111_e1411_d_b21, eq111_e1411_d_b22, eq111_e1411_d_b23, eq111_e1411_d_b24, eq111_e1411_d_b25, eq111_e1411_d_b26, eq111_e1411_d_b27, eq111_e1411_d_b28, eq111_e1411_d_b29, eq111_e1411_d_b30, eq111_e1411_d_b31, eq111_e1411_d_b32, eq111_e1411_d_b33, eq111_e1411_d_b34, eq111_e1411_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29, eq112_e1421_d_b0, eq112_e1421_d_b1, eq112_e1421_d_b2, eq112_e1421_d_b3, eq112_e1421_d_b4, eq112_e1421_d_b5, eq112_e1421_d_b6, eq112_e1421_d_b7, eq112_e1421_d_b8, eq112_e1421_d_b9, eq112_e1421_d_b10, eq112_e1421_d_b11, eq112_e1421_d_b12, eq112_e1421_d_b13, eq112_e1421_d_b14, eq112_e1421_d_b15, eq112_e1421_d_b16, eq112_e1421_d_b17, eq112_e1421_d_b18, eq112_e1421_d_b19, eq112_e1421_d_b20, eq112_e1421_d_b21, eq112_e1421_d_b22, eq112_e1421_d_b23, eq112_e1421_d_b24, eq112_e1421_d_b25, eq112_e1421_d_b26, eq112_e1421_d_b27, eq112_e1421_d_b28, eq112_e1421_d_b29, eq112_e1421_d_b30, eq112_e1421_d_b31, eq112_e1421_d_b32, eq112_e1421_d_b33, eq112_e1421_d_b34, eq112_e1421_d_b35, eq112_e1421_q,) = {
    if s.b[1348] {
        let eq112_e1414_q: f64 = s.v[180];
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1418_q: f64 = eq112_e1417;
        let eq112_e1419: f64 = (s.v[180] + eq112_e1417);
        let eq112_e1419_d_n7: f64 = (s.dn[180][7] + p.p355);
        let eq112_e1419_q: f64 = (eq112_e1414_q + eq112_e1418_q);
        (eq112_e1419, s.dn[180][0], s.dn[180][1], s.dn[180][2], s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], eq112_e1419_d_n7, s.dn[180][8], s.dn[180][9], s.dn[180][10], __rspice_deriv_cse_2, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29], s.db[180][0], s.db[180][1], s.db[180][2], s.db[180][3], s.db[180][4], s.db[180][5], s.db[180][6], s.db[180][7], s.db[180][8], s.db[180][9], s.db[180][10], s.db[180][11], s.db[180][12], s.db[180][13], s.db[180][14], s.db[180][15], s.db[180][16], s.db[180][17], s.db[180][18], s.db[180][19], s.db[180][20], s.db[180][21], s.db[180][22], s.db[180][23], s.db[180][24], s.db[180][25], s.db[180][26], s.db[180][27], s.db[180][28], s.db[180][29], s.db[180][30], s.db[180][31], s.db[180][32], s.db[180][33], s.db[180][34], s.db[180][35], eq112_e1419_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 30] = [eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29];
        let eq112_reactive_branch_derivatives: [f64; 36] = [eq112_e1421_d_b0, eq112_e1421_d_b1, eq112_e1421_d_b2, eq112_e1421_d_b3, eq112_e1421_d_b4, eq112_e1421_d_b5, eq112_e1421_d_b6, eq112_e1421_d_b7, eq112_e1421_d_b8, eq112_e1421_d_b9, eq112_e1421_d_b10, eq112_e1421_d_b11, eq112_e1421_d_b12, eq112_e1421_d_b13, eq112_e1421_d_b14, eq112_e1421_d_b15, eq112_e1421_d_b16, eq112_e1421_d_b17, eq112_e1421_d_b18, eq112_e1421_d_b19, eq112_e1421_d_b20, eq112_e1421_d_b21, eq112_e1421_d_b22, eq112_e1421_d_b23, eq112_e1421_d_b24, eq112_e1421_d_b25, eq112_e1421_d_b26, eq112_e1421_d_b27, eq112_e1421_d_b28, eq112_e1421_d_b29, eq112_e1421_d_b30, eq112_e1421_d_b31, eq112_e1421_d_b32, eq112_e1421_d_b33, eq112_e1421_d_b34, eq112_e1421_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq112_reactive_node_derivatives,
            branches,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1431, eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29, eq113_e1431_d_b0, eq113_e1431_d_b1, eq113_e1431_d_b2, eq113_e1431_d_b3, eq113_e1431_d_b4, eq113_e1431_d_b5, eq113_e1431_d_b6, eq113_e1431_d_b7, eq113_e1431_d_b8, eq113_e1431_d_b9, eq113_e1431_d_b10, eq113_e1431_d_b11, eq113_e1431_d_b12, eq113_e1431_d_b13, eq113_e1431_d_b14, eq113_e1431_d_b15, eq113_e1431_d_b16, eq113_e1431_d_b17, eq113_e1431_d_b18, eq113_e1431_d_b19, eq113_e1431_d_b20, eq113_e1431_d_b21, eq113_e1431_d_b22, eq113_e1431_d_b23, eq113_e1431_d_b24, eq113_e1431_d_b25, eq113_e1431_d_b26, eq113_e1431_d_b27, eq113_e1431_d_b28, eq113_e1431_d_b29, eq113_e1431_d_b30, eq113_e1431_d_b31, eq113_e1431_d_b32, eq113_e1431_d_b33, eq113_e1431_d_b34, eq113_e1431_d_b35, eq113_e1431_q,) = {
    if s.b[1348] {
        let eq113_e1424_q: f64 = s.v[181];
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1428_q: f64 = eq113_e1427;
        let eq113_e1429: f64 = (s.v[181] + eq113_e1427);
        let eq113_e1429_d_n2: f64 = (s.dn[181][2] + p.p355);
        let eq113_e1429_q: f64 = (eq113_e1424_q + eq113_e1428_q);
        (eq113_e1429, s.dn[181][0], s.dn[181][1], eq113_e1429_d_n2, s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], s.dn[181][7], s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], __rspice_deriv_cse_3, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29], s.db[181][0], s.db[181][1], s.db[181][2], s.db[181][3], s.db[181][4], s.db[181][5], s.db[181][6], s.db[181][7], s.db[181][8], s.db[181][9], s.db[181][10], s.db[181][11], s.db[181][12], s.db[181][13], s.db[181][14], s.db[181][15], s.db[181][16], s.db[181][17], s.db[181][18], s.db[181][19], s.db[181][20], s.db[181][21], s.db[181][22], s.db[181][23], s.db[181][24], s.db[181][25], s.db[181][26], s.db[181][27], s.db[181][28], s.db[181][29], s.db[181][30], s.db[181][31], s.db[181][32], s.db[181][33], s.db[181][34], s.db[181][35], eq113_e1429_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 30] = [eq113_e1431_d_n0, eq113_e1431_d_n1, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n5, eq113_e1431_d_n6, eq113_e1431_d_n7, eq113_e1431_d_n8, eq113_e1431_d_n9, eq113_e1431_d_n10, eq113_e1431_d_n11, eq113_e1431_d_n12, eq113_e1431_d_n13, eq113_e1431_d_n14, eq113_e1431_d_n15, eq113_e1431_d_n16, eq113_e1431_d_n17, eq113_e1431_d_n18, eq113_e1431_d_n19, eq113_e1431_d_n20, eq113_e1431_d_n21, eq113_e1431_d_n22, eq113_e1431_d_n23, eq113_e1431_d_n24, eq113_e1431_d_n25, eq113_e1431_d_n26, eq113_e1431_d_n27, eq113_e1431_d_n28, eq113_e1431_d_n29];
        let eq113_reactive_branch_derivatives: [f64; 36] = [eq113_e1431_d_b0, eq113_e1431_d_b1, eq113_e1431_d_b2, eq113_e1431_d_b3, eq113_e1431_d_b4, eq113_e1431_d_b5, eq113_e1431_d_b6, eq113_e1431_d_b7, eq113_e1431_d_b8, eq113_e1431_d_b9, eq113_e1431_d_b10, eq113_e1431_d_b11, eq113_e1431_d_b12, eq113_e1431_d_b13, eq113_e1431_d_b14, eq113_e1431_d_b15, eq113_e1431_d_b16, eq113_e1431_d_b17, eq113_e1431_d_b18, eq113_e1431_d_b19, eq113_e1431_d_b20, eq113_e1431_d_b21, eq113_e1431_d_b22, eq113_e1431_d_b23, eq113_e1431_d_b24, eq113_e1431_d_b25, eq113_e1431_d_b26, eq113_e1431_d_b27, eq113_e1431_d_b28, eq113_e1431_d_b29, eq113_e1431_d_b30, eq113_e1431_d_b31, eq113_e1431_d_b32, eq113_e1431_d_b33, eq113_e1431_d_b34, eq113_e1431_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq113_reactive_node_derivatives,
            branches,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29, eq115_e1445_d_b0, eq115_e1445_d_b1, eq115_e1445_d_b2, eq115_e1445_d_b3, eq115_e1445_d_b4, eq115_e1445_d_b5, eq115_e1445_d_b6, eq115_e1445_d_b7, eq115_e1445_d_b8, eq115_e1445_d_b9, eq115_e1445_d_b10, eq115_e1445_d_b11, eq115_e1445_d_b12, eq115_e1445_d_b13, eq115_e1445_d_b14, eq115_e1445_d_b15, eq115_e1445_d_b16, eq115_e1445_d_b17, eq115_e1445_d_b18, eq115_e1445_d_b19, eq115_e1445_d_b20, eq115_e1445_d_b21, eq115_e1445_d_b22, eq115_e1445_d_b23, eq115_e1445_d_b24, eq115_e1445_d_b25, eq115_e1445_d_b26, eq115_e1445_d_b27, eq115_e1445_d_b28, eq115_e1445_d_b29, eq115_e1445_d_b30, eq115_e1445_d_b31, eq115_e1445_d_b32, eq115_e1445_d_b33, eq115_e1445_d_b34, eq115_e1445_d_b35, eq115_e1445_q,) = {
    if s.b[1348] {
        let eq115_e1438_q: f64 = s.v[183];
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1442_q: f64 = eq115_e1441;
        let eq115_e1443: f64 = (s.v[183] + eq115_e1441);
        let eq115_e1443_d_n7: f64 = (s.dn[183][7] + p.p355);
        let eq115_e1443_d_n9: f64 = (s.dn[183][9] + (-p.p355));
        let eq115_e1443_q: f64 = (eq115_e1438_q + eq115_e1442_q);
        (eq115_e1443, s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], eq115_e1443_d_n7, s.dn[183][8], eq115_e1443_d_n9, s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.dn[183][15], s.dn[183][16], s.dn[183][17], s.dn[183][18], s.dn[183][19], s.dn[183][20], s.dn[183][21], s.dn[183][22], s.dn[183][23], s.dn[183][24], s.dn[183][25], s.dn[183][26], s.dn[183][27], s.dn[183][28], s.dn[183][29], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5], s.db[183][6], s.db[183][7], s.db[183][8], s.db[183][9], s.db[183][10], s.db[183][11], s.db[183][12], s.db[183][13], s.db[183][14], s.db[183][15], s.db[183][16], s.db[183][17], s.db[183][18], s.db[183][19], s.db[183][20], s.db[183][21], s.db[183][22], s.db[183][23], s.db[183][24], s.db[183][25], s.db[183][26], s.db[183][27], s.db[183][28], s.db[183][29], s.db[183][30], s.db[183][31], s.db[183][32], s.db[183][33], s.db[183][34], s.db[183][35], eq115_e1443_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_reactive_node_derivatives: [f64; 30] = [eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29];
        let eq115_reactive_branch_derivatives: [f64; 36] = [eq115_e1445_d_b0, eq115_e1445_d_b1, eq115_e1445_d_b2, eq115_e1445_d_b3, eq115_e1445_d_b4, eq115_e1445_d_b5, eq115_e1445_d_b6, eq115_e1445_d_b7, eq115_e1445_d_b8, eq115_e1445_d_b9, eq115_e1445_d_b10, eq115_e1445_d_b11, eq115_e1445_d_b12, eq115_e1445_d_b13, eq115_e1445_d_b14, eq115_e1445_d_b15, eq115_e1445_d_b16, eq115_e1445_d_b17, eq115_e1445_d_b18, eq115_e1445_d_b19, eq115_e1445_d_b20, eq115_e1445_d_b21, eq115_e1445_d_b22, eq115_e1445_d_b23, eq115_e1445_d_b24, eq115_e1445_d_b25, eq115_e1445_d_b26, eq115_e1445_d_b27, eq115_e1445_d_b28, eq115_e1445_d_b29, eq115_e1445_d_b30, eq115_e1445_d_b31, eq115_e1445_d_b32, eq115_e1445_d_b33, eq115_e1445_d_b34, eq115_e1445_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq115_reactive_node_derivatives,
            branches,
            &eq115_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29, eq116_e1456_d_b0, eq116_e1456_d_b1, eq116_e1456_d_b2, eq116_e1456_d_b3, eq116_e1456_d_b4, eq116_e1456_d_b5, eq116_e1456_d_b6, eq116_e1456_d_b7, eq116_e1456_d_b8, eq116_e1456_d_b9, eq116_e1456_d_b10, eq116_e1456_d_b11, eq116_e1456_d_b12, eq116_e1456_d_b13, eq116_e1456_d_b14, eq116_e1456_d_b15, eq116_e1456_d_b16, eq116_e1456_d_b17, eq116_e1456_d_b18, eq116_e1456_d_b19, eq116_e1456_d_b20, eq116_e1456_d_b21, eq116_e1456_d_b22, eq116_e1456_d_b23, eq116_e1456_d_b24, eq116_e1456_d_b25, eq116_e1456_d_b26, eq116_e1456_d_b27, eq116_e1456_d_b28, eq116_e1456_d_b29, eq116_e1456_d_b30, eq116_e1456_d_b31, eq116_e1456_d_b32, eq116_e1456_d_b33, eq116_e1456_d_b34, eq116_e1456_d_b35, eq116_e1456_q,) = {
    if (!s.b[1348]) {
        let eq116_e1449_q: f64 = s.v[179];
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1453_q: f64 = eq116_e1452;
        let eq116_e1454: f64 = (s.v[179] + eq116_e1452);
        let eq116_e1454_d_n2: f64 = (s.dn[179][2] + p.p355);
        let eq116_e1454_q: f64 = (eq116_e1449_q + eq116_e1453_q);
        (eq116_e1454, s.dn[179][0], s.dn[179][1], eq116_e1454_d_n2, s.dn[179][3], s.dn[179][4], s.dn[179][5], s.dn[179][6], s.dn[179][7], s.dn[179][8], s.dn[179][9], s.dn[179][10], s.dn[179][11], __rspice_deriv_cse_1, s.dn[179][13], s.dn[179][14], s.dn[179][15], s.dn[179][16], s.dn[179][17], s.dn[179][18], s.dn[179][19], s.dn[179][20], s.dn[179][21], s.dn[179][22], s.dn[179][23], s.dn[179][24], s.dn[179][25], s.dn[179][26], s.dn[179][27], s.dn[179][28], s.dn[179][29], s.db[179][0], s.db[179][1], s.db[179][2], s.db[179][3], s.db[179][4], s.db[179][5], s.db[179][6], s.db[179][7], s.db[179][8], s.db[179][9], s.db[179][10], s.db[179][11], s.db[179][12], s.db[179][13], s.db[179][14], s.db[179][15], s.db[179][16], s.db[179][17], s.db[179][18], s.db[179][19], s.db[179][20], s.db[179][21], s.db[179][22], s.db[179][23], s.db[179][24], s.db[179][25], s.db[179][26], s.db[179][27], s.db[179][28], s.db[179][29], s.db[179][30], s.db[179][31], s.db[179][32], s.db[179][33], s.db[179][34], s.db[179][35], eq116_e1454_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_reactive_node_derivatives: [f64; 30] = [eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29];
        let eq116_reactive_branch_derivatives: [f64; 36] = [eq116_e1456_d_b0, eq116_e1456_d_b1, eq116_e1456_d_b2, eq116_e1456_d_b3, eq116_e1456_d_b4, eq116_e1456_d_b5, eq116_e1456_d_b6, eq116_e1456_d_b7, eq116_e1456_d_b8, eq116_e1456_d_b9, eq116_e1456_d_b10, eq116_e1456_d_b11, eq116_e1456_d_b12, eq116_e1456_d_b13, eq116_e1456_d_b14, eq116_e1456_d_b15, eq116_e1456_d_b16, eq116_e1456_d_b17, eq116_e1456_d_b18, eq116_e1456_d_b19, eq116_e1456_d_b20, eq116_e1456_d_b21, eq116_e1456_d_b22, eq116_e1456_d_b23, eq116_e1456_d_b24, eq116_e1456_d_b25, eq116_e1456_d_b26, eq116_e1456_d_b27, eq116_e1456_d_b28, eq116_e1456_d_b29, eq116_e1456_d_b30, eq116_e1456_d_b31, eq116_e1456_d_b32, eq116_e1456_d_b33, eq116_e1456_d_b34, eq116_e1456_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq116_reactive_node_derivatives,
            branches,
            &eq116_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29, eq117_e1467_d_b0, eq117_e1467_d_b1, eq117_e1467_d_b2, eq117_e1467_d_b3, eq117_e1467_d_b4, eq117_e1467_d_b5, eq117_e1467_d_b6, eq117_e1467_d_b7, eq117_e1467_d_b8, eq117_e1467_d_b9, eq117_e1467_d_b10, eq117_e1467_d_b11, eq117_e1467_d_b12, eq117_e1467_d_b13, eq117_e1467_d_b14, eq117_e1467_d_b15, eq117_e1467_d_b16, eq117_e1467_d_b17, eq117_e1467_d_b18, eq117_e1467_d_b19, eq117_e1467_d_b20, eq117_e1467_d_b21, eq117_e1467_d_b22, eq117_e1467_d_b23, eq117_e1467_d_b24, eq117_e1467_d_b25, eq117_e1467_d_b26, eq117_e1467_d_b27, eq117_e1467_d_b28, eq117_e1467_d_b29, eq117_e1467_d_b30, eq117_e1467_d_b31, eq117_e1467_d_b32, eq117_e1467_d_b33, eq117_e1467_d_b34, eq117_e1467_d_b35, eq117_e1467_q,) = {
    if (!s.b[1348]) {
        let eq117_e1460_q: f64 = s.v[180];
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1464_q: f64 = eq117_e1463;
        let eq117_e1465: f64 = (s.v[180] + eq117_e1463);
        let eq117_e1465_d_n2: f64 = (s.dn[180][2] + p.p355);
        let eq117_e1465_q: f64 = (eq117_e1460_q + eq117_e1464_q);
        (eq117_e1465, s.dn[180][0], s.dn[180][1], eq117_e1465_d_n2, s.dn[180][3], s.dn[180][4], s.dn[180][5], s.dn[180][6], s.dn[180][7], s.dn[180][8], s.dn[180][9], s.dn[180][10], __rspice_deriv_cse_2, s.dn[180][12], s.dn[180][13], s.dn[180][14], s.dn[180][15], s.dn[180][16], s.dn[180][17], s.dn[180][18], s.dn[180][19], s.dn[180][20], s.dn[180][21], s.dn[180][22], s.dn[180][23], s.dn[180][24], s.dn[180][25], s.dn[180][26], s.dn[180][27], s.dn[180][28], s.dn[180][29], s.db[180][0], s.db[180][1], s.db[180][2], s.db[180][3], s.db[180][4], s.db[180][5], s.db[180][6], s.db[180][7], s.db[180][8], s.db[180][9], s.db[180][10], s.db[180][11], s.db[180][12], s.db[180][13], s.db[180][14], s.db[180][15], s.db[180][16], s.db[180][17], s.db[180][18], s.db[180][19], s.db[180][20], s.db[180][21], s.db[180][22], s.db[180][23], s.db[180][24], s.db[180][25], s.db[180][26], s.db[180][27], s.db[180][28], s.db[180][29], s.db[180][30], s.db[180][31], s.db[180][32], s.db[180][33], s.db[180][34], s.db[180][35], eq117_e1465_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_reactive_node_derivatives: [f64; 30] = [eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29];
        let eq117_reactive_branch_derivatives: [f64; 36] = [eq117_e1467_d_b0, eq117_e1467_d_b1, eq117_e1467_d_b2, eq117_e1467_d_b3, eq117_e1467_d_b4, eq117_e1467_d_b5, eq117_e1467_d_b6, eq117_e1467_d_b7, eq117_e1467_d_b8, eq117_e1467_d_b9, eq117_e1467_d_b10, eq117_e1467_d_b11, eq117_e1467_d_b12, eq117_e1467_d_b13, eq117_e1467_d_b14, eq117_e1467_d_b15, eq117_e1467_d_b16, eq117_e1467_d_b17, eq117_e1467_d_b18, eq117_e1467_d_b19, eq117_e1467_d_b20, eq117_e1467_d_b21, eq117_e1467_d_b22, eq117_e1467_d_b23, eq117_e1467_d_b24, eq117_e1467_d_b25, eq117_e1467_d_b26, eq117_e1467_d_b27, eq117_e1467_d_b28, eq117_e1467_d_b29, eq117_e1467_d_b30, eq117_e1467_d_b31, eq117_e1467_d_b32, eq117_e1467_d_b33, eq117_e1467_d_b34, eq117_e1467_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            nodes,
            &eq117_reactive_node_derivatives,
            branches,
            &eq117_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29, eq118_e1478_d_b0, eq118_e1478_d_b1, eq118_e1478_d_b2, eq118_e1478_d_b3, eq118_e1478_d_b4, eq118_e1478_d_b5, eq118_e1478_d_b6, eq118_e1478_d_b7, eq118_e1478_d_b8, eq118_e1478_d_b9, eq118_e1478_d_b10, eq118_e1478_d_b11, eq118_e1478_d_b12, eq118_e1478_d_b13, eq118_e1478_d_b14, eq118_e1478_d_b15, eq118_e1478_d_b16, eq118_e1478_d_b17, eq118_e1478_d_b18, eq118_e1478_d_b19, eq118_e1478_d_b20, eq118_e1478_d_b21, eq118_e1478_d_b22, eq118_e1478_d_b23, eq118_e1478_d_b24, eq118_e1478_d_b25, eq118_e1478_d_b26, eq118_e1478_d_b27, eq118_e1478_d_b28, eq118_e1478_d_b29, eq118_e1478_d_b30, eq118_e1478_d_b31, eq118_e1478_d_b32, eq118_e1478_d_b33, eq118_e1478_d_b34, eq118_e1478_d_b35, eq118_e1478_q,) = {
    if (!s.b[1348]) {
        let eq118_e1471_q: f64 = s.v[181];
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1475_q: f64 = eq118_e1474;
        let eq118_e1476: f64 = (s.v[181] + eq118_e1474);
        let eq118_e1476_d_n7: f64 = (s.dn[181][7] + p.p355);
        let eq118_e1476_q: f64 = (eq118_e1471_q + eq118_e1475_q);
        (eq118_e1476, s.dn[181][0], s.dn[181][1], s.dn[181][2], s.dn[181][3], s.dn[181][4], s.dn[181][5], s.dn[181][6], eq118_e1476_d_n7, s.dn[181][8], s.dn[181][9], s.dn[181][10], s.dn[181][11], __rspice_deriv_cse_3, s.dn[181][13], s.dn[181][14], s.dn[181][15], s.dn[181][16], s.dn[181][17], s.dn[181][18], s.dn[181][19], s.dn[181][20], s.dn[181][21], s.dn[181][22], s.dn[181][23], s.dn[181][24], s.dn[181][25], s.dn[181][26], s.dn[181][27], s.dn[181][28], s.dn[181][29], s.db[181][0], s.db[181][1], s.db[181][2], s.db[181][3], s.db[181][4], s.db[181][5], s.db[181][6], s.db[181][7], s.db[181][8], s.db[181][9], s.db[181][10], s.db[181][11], s.db[181][12], s.db[181][13], s.db[181][14], s.db[181][15], s.db[181][16], s.db[181][17], s.db[181][18], s.db[181][19], s.db[181][20], s.db[181][21], s.db[181][22], s.db[181][23], s.db[181][24], s.db[181][25], s.db[181][26], s.db[181][27], s.db[181][28], s.db[181][29], s.db[181][30], s.db[181][31], s.db[181][32], s.db[181][33], s.db[181][34], s.db[181][35], eq118_e1476_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_reactive_node_derivatives: [f64; 30] = [eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29];
        let eq118_reactive_branch_derivatives: [f64; 36] = [eq118_e1478_d_b0, eq118_e1478_d_b1, eq118_e1478_d_b2, eq118_e1478_d_b3, eq118_e1478_d_b4, eq118_e1478_d_b5, eq118_e1478_d_b6, eq118_e1478_d_b7, eq118_e1478_d_b8, eq118_e1478_d_b9, eq118_e1478_d_b10, eq118_e1478_d_b11, eq118_e1478_d_b12, eq118_e1478_d_b13, eq118_e1478_d_b14, eq118_e1478_d_b15, eq118_e1478_d_b16, eq118_e1478_d_b17, eq118_e1478_d_b18, eq118_e1478_d_b19, eq118_e1478_d_b20, eq118_e1478_d_b21, eq118_e1478_d_b22, eq118_e1478_d_b23, eq118_e1478_d_b24, eq118_e1478_d_b25, eq118_e1478_d_b26, eq118_e1478_d_b27, eq118_e1478_d_b28, eq118_e1478_d_b29, eq118_e1478_d_b30, eq118_e1478_d_b31, eq118_e1478_d_b32, eq118_e1478_d_b33, eq118_e1478_d_b34, eq118_e1478_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq118_reactive_node_derivatives,
            branches,
            &eq118_reactive_branch_derivatives,
            multiplicity,
        );
        let eq121_e1490_q: f64 = s.v[182];
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1494_q: f64 = eq121_e1493;
        let eq121_e1495: f64 = (s.v[182] + eq121_e1493);
        let eq121_e1495_d_n3: f64 = (s.dn[182][3] + p.p355);
        let eq121_e1495_d_n12: f64 = (s.dn[182][12] + (-p.p355));
        let eq121_e1495_q: f64 = (eq121_e1490_q + eq121_e1494_q);
        let eq121_reactive_node_derivatives: [f64; 30] = [s.dn[182][0], s.dn[182][1], s.dn[182][2], eq121_e1495_d_n3, s.dn[182][4], s.dn[182][5], s.dn[182][6], s.dn[182][7], s.dn[182][8], s.dn[182][9], s.dn[182][10], s.dn[182][11], eq121_e1495_d_n12, s.dn[182][13], s.dn[182][14], s.dn[182][15], s.dn[182][16], s.dn[182][17], s.dn[182][18], s.dn[182][19], s.dn[182][20], s.dn[182][21], s.dn[182][22], s.dn[182][23], s.dn[182][24], s.dn[182][25], s.dn[182][26], s.dn[182][27], s.dn[182][28], s.dn[182][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &s.db[182],
            multiplicity,
        );
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29, eq124_e1518_d_b0, eq124_e1518_d_b1, eq124_e1518_d_b2, eq124_e1518_d_b3, eq124_e1518_d_b4, eq124_e1518_d_b5, eq124_e1518_d_b6, eq124_e1518_d_b7, eq124_e1518_d_b8, eq124_e1518_d_b9, eq124_e1518_d_b10, eq124_e1518_d_b11, eq124_e1518_d_b12, eq124_e1518_d_b13, eq124_e1518_d_b14, eq124_e1518_d_b15, eq124_e1518_d_b16, eq124_e1518_d_b17, eq124_e1518_d_b18, eq124_e1518_d_b19, eq124_e1518_d_b20, eq124_e1518_d_b21, eq124_e1518_d_b22, eq124_e1518_d_b23, eq124_e1518_d_b24, eq124_e1518_d_b25, eq124_e1518_d_b26, eq124_e1518_d_b27, eq124_e1518_d_b28, eq124_e1518_d_b29, eq124_e1518_d_b30, eq124_e1518_d_b31, eq124_e1518_d_b32, eq124_e1518_d_b33, eq124_e1518_d_b34, eq124_e1518_d_b35, eq124_e1518_q,) = {
    if s.b[1495] {
        let eq124_e1511_q: f64 = s.v[185];
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1515_q: f64 = eq124_e1514;
        let eq124_e1516: f64 = (s.v[185] + eq124_e1514);
        let eq124_e1516_d_n7: f64 = (s.dn[185][7] + p.p355);
        let eq124_e1516_q: f64 = (eq124_e1511_q + eq124_e1515_q);
        (eq124_e1516, s.dn[185][0], s.dn[185][1], s.dn[185][2], s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], eq124_e1516_d_n7, s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], __rspice_deriv_cse_4, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29], s.db[185][0], s.db[185][1], s.db[185][2], s.db[185][3], s.db[185][4], s.db[185][5], s.db[185][6], s.db[185][7], s.db[185][8], s.db[185][9], s.db[185][10], s.db[185][11], s.db[185][12], s.db[185][13], s.db[185][14], s.db[185][15], s.db[185][16], s.db[185][17], s.db[185][18], s.db[185][19], s.db[185][20], s.db[185][21], s.db[185][22], s.db[185][23], s.db[185][24], s.db[185][25], s.db[185][26], s.db[185][27], s.db[185][28], s.db[185][29], s.db[185][30], s.db[185][31], s.db[185][32], s.db[185][33], s.db[185][34], s.db[185][35], eq124_e1516_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 30] = [eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29];
        let eq124_reactive_branch_derivatives: [f64; 36] = [eq124_e1518_d_b0, eq124_e1518_d_b1, eq124_e1518_d_b2, eq124_e1518_d_b3, eq124_e1518_d_b4, eq124_e1518_d_b5, eq124_e1518_d_b6, eq124_e1518_d_b7, eq124_e1518_d_b8, eq124_e1518_d_b9, eq124_e1518_d_b10, eq124_e1518_d_b11, eq124_e1518_d_b12, eq124_e1518_d_b13, eq124_e1518_d_b14, eq124_e1518_d_b15, eq124_e1518_d_b16, eq124_e1518_d_b17, eq124_e1518_d_b18, eq124_e1518_d_b19, eq124_e1518_d_b20, eq124_e1518_d_b21, eq124_e1518_d_b22, eq124_e1518_d_b23, eq124_e1518_d_b24, eq124_e1518_d_b25, eq124_e1518_d_b26, eq124_e1518_d_b27, eq124_e1518_d_b28, eq124_e1518_d_b29, eq124_e1518_d_b30, eq124_e1518_d_b31, eq124_e1518_d_b32, eq124_e1518_d_b33, eq124_e1518_d_b34, eq124_e1518_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29, eq125_e1528_d_b0, eq125_e1528_d_b1, eq125_e1528_d_b2, eq125_e1528_d_b3, eq125_e1528_d_b4, eq125_e1528_d_b5, eq125_e1528_d_b6, eq125_e1528_d_b7, eq125_e1528_d_b8, eq125_e1528_d_b9, eq125_e1528_d_b10, eq125_e1528_d_b11, eq125_e1528_d_b12, eq125_e1528_d_b13, eq125_e1528_d_b14, eq125_e1528_d_b15, eq125_e1528_d_b16, eq125_e1528_d_b17, eq125_e1528_d_b18, eq125_e1528_d_b19, eq125_e1528_d_b20, eq125_e1528_d_b21, eq125_e1528_d_b22, eq125_e1528_d_b23, eq125_e1528_d_b24, eq125_e1528_d_b25, eq125_e1528_d_b26, eq125_e1528_d_b27, eq125_e1528_d_b28, eq125_e1528_d_b29, eq125_e1528_d_b30, eq125_e1528_d_b31, eq125_e1528_d_b32, eq125_e1528_d_b33, eq125_e1528_d_b34, eq125_e1528_d_b35, eq125_e1528_q,) = {
    if s.b[1495] {
        let eq125_e1521_q: f64 = s.v[186];
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1525_q: f64 = eq125_e1524;
        let eq125_e1526: f64 = (s.v[186] + eq125_e1524);
        let eq125_e1526_d_n7: f64 = (s.dn[186][7] + p.p355);
        let eq125_e1526_q: f64 = (eq125_e1521_q + eq125_e1525_q);
        (eq125_e1526, s.dn[186][0], s.dn[186][1], s.dn[186][2], s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], eq125_e1526_d_n7, s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], __rspice_deriv_cse_5, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29], s.db[186][0], s.db[186][1], s.db[186][2], s.db[186][3], s.db[186][4], s.db[186][5], s.db[186][6], s.db[186][7], s.db[186][8], s.db[186][9], s.db[186][10], s.db[186][11], s.db[186][12], s.db[186][13], s.db[186][14], s.db[186][15], s.db[186][16], s.db[186][17], s.db[186][18], s.db[186][19], s.db[186][20], s.db[186][21], s.db[186][22], s.db[186][23], s.db[186][24], s.db[186][25], s.db[186][26], s.db[186][27], s.db[186][28], s.db[186][29], s.db[186][30], s.db[186][31], s.db[186][32], s.db[186][33], s.db[186][34], s.db[186][35], eq125_e1526_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 30] = [eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29];
        let eq125_reactive_branch_derivatives: [f64; 36] = [eq125_e1528_d_b0, eq125_e1528_d_b1, eq125_e1528_d_b2, eq125_e1528_d_b3, eq125_e1528_d_b4, eq125_e1528_d_b5, eq125_e1528_d_b6, eq125_e1528_d_b7, eq125_e1528_d_b8, eq125_e1528_d_b9, eq125_e1528_d_b10, eq125_e1528_d_b11, eq125_e1528_d_b12, eq125_e1528_d_b13, eq125_e1528_d_b14, eq125_e1528_d_b15, eq125_e1528_d_b16, eq125_e1528_d_b17, eq125_e1528_d_b18, eq125_e1528_d_b19, eq125_e1528_d_b20, eq125_e1528_d_b21, eq125_e1528_d_b22, eq125_e1528_d_b23, eq125_e1528_d_b24, eq125_e1528_d_b25, eq125_e1528_d_b26, eq125_e1528_d_b27, eq125_e1528_d_b28, eq125_e1528_d_b29, eq125_e1528_d_b30, eq125_e1528_d_b31, eq125_e1528_d_b32, eq125_e1528_d_b33, eq125_e1528_d_b34, eq125_e1528_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29, eq126_e1538_d_b0, eq126_e1538_d_b1, eq126_e1538_d_b2, eq126_e1538_d_b3, eq126_e1538_d_b4, eq126_e1538_d_b5, eq126_e1538_d_b6, eq126_e1538_d_b7, eq126_e1538_d_b8, eq126_e1538_d_b9, eq126_e1538_d_b10, eq126_e1538_d_b11, eq126_e1538_d_b12, eq126_e1538_d_b13, eq126_e1538_d_b14, eq126_e1538_d_b15, eq126_e1538_d_b16, eq126_e1538_d_b17, eq126_e1538_d_b18, eq126_e1538_d_b19, eq126_e1538_d_b20, eq126_e1538_d_b21, eq126_e1538_d_b22, eq126_e1538_d_b23, eq126_e1538_d_b24, eq126_e1538_d_b25, eq126_e1538_d_b26, eq126_e1538_d_b27, eq126_e1538_d_b28, eq126_e1538_d_b29, eq126_e1538_d_b30, eq126_e1538_d_b31, eq126_e1538_d_b32, eq126_e1538_d_b33, eq126_e1538_d_b34, eq126_e1538_d_b35, eq126_e1538_q,) = {
    if s.b[1495] {
        let eq126_e1531_q: f64 = s.v[187];
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1535_q: f64 = eq126_e1534;
        let eq126_e1536: f64 = (s.v[187] + eq126_e1534);
        let eq126_e1536_d_n2: f64 = (s.dn[187][2] + p.p355);
        let eq126_e1536_q: f64 = (eq126_e1531_q + eq126_e1535_q);
        (eq126_e1536, s.dn[187][0], s.dn[187][1], eq126_e1536_d_n2, s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], s.dn[187][7], s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], __rspice_deriv_cse_6, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29], s.db[187][0], s.db[187][1], s.db[187][2], s.db[187][3], s.db[187][4], s.db[187][5], s.db[187][6], s.db[187][7], s.db[187][8], s.db[187][9], s.db[187][10], s.db[187][11], s.db[187][12], s.db[187][13], s.db[187][14], s.db[187][15], s.db[187][16], s.db[187][17], s.db[187][18], s.db[187][19], s.db[187][20], s.db[187][21], s.db[187][22], s.db[187][23], s.db[187][24], s.db[187][25], s.db[187][26], s.db[187][27], s.db[187][28], s.db[187][29], s.db[187][30], s.db[187][31], s.db[187][32], s.db[187][33], s.db[187][34], s.db[187][35], eq126_e1536_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 30] = [eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29];
        let eq126_reactive_branch_derivatives: [f64; 36] = [eq126_e1538_d_b0, eq126_e1538_d_b1, eq126_e1538_d_b2, eq126_e1538_d_b3, eq126_e1538_d_b4, eq126_e1538_d_b5, eq126_e1538_d_b6, eq126_e1538_d_b7, eq126_e1538_d_b8, eq126_e1538_d_b9, eq126_e1538_d_b10, eq126_e1538_d_b11, eq126_e1538_d_b12, eq126_e1538_d_b13, eq126_e1538_d_b14, eq126_e1538_d_b15, eq126_e1538_d_b16, eq126_e1538_d_b17, eq126_e1538_d_b18, eq126_e1538_d_b19, eq126_e1538_d_b20, eq126_e1538_d_b21, eq126_e1538_d_b22, eq126_e1538_d_b23, eq126_e1538_d_b24, eq126_e1538_d_b25, eq126_e1538_d_b26, eq126_e1538_d_b27, eq126_e1538_d_b28, eq126_e1538_d_b29, eq126_e1538_d_b30, eq126_e1538_d_b31, eq126_e1538_d_b32, eq126_e1538_d_b33, eq126_e1538_d_b34, eq126_e1538_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29, eq128_e1552_d_b0, eq128_e1552_d_b1, eq128_e1552_d_b2, eq128_e1552_d_b3, eq128_e1552_d_b4, eq128_e1552_d_b5, eq128_e1552_d_b6, eq128_e1552_d_b7, eq128_e1552_d_b8, eq128_e1552_d_b9, eq128_e1552_d_b10, eq128_e1552_d_b11, eq128_e1552_d_b12, eq128_e1552_d_b13, eq128_e1552_d_b14, eq128_e1552_d_b15, eq128_e1552_d_b16, eq128_e1552_d_b17, eq128_e1552_d_b18, eq128_e1552_d_b19, eq128_e1552_d_b20, eq128_e1552_d_b21, eq128_e1552_d_b22, eq128_e1552_d_b23, eq128_e1552_d_b24, eq128_e1552_d_b25, eq128_e1552_d_b26, eq128_e1552_d_b27, eq128_e1552_d_b28, eq128_e1552_d_b29, eq128_e1552_d_b30, eq128_e1552_d_b31, eq128_e1552_d_b32, eq128_e1552_d_b33, eq128_e1552_d_b34, eq128_e1552_d_b35, eq128_e1552_q,) = {
    if s.b[1495] {
        let eq128_e1545_q: f64 = s.v[189];
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1549_q: f64 = eq128_e1548;
        let eq128_e1550: f64 = (s.v[189] + eq128_e1548);
        let eq128_e1550_d_n7: f64 = (s.dn[189][7] + p.p355);
        let eq128_e1550_d_n9: f64 = (s.dn[189][9] + (-p.p355));
        let eq128_e1550_q: f64 = (eq128_e1545_q + eq128_e1549_q);
        (eq128_e1550, s.dn[189][0], s.dn[189][1], s.dn[189][2], s.dn[189][3], s.dn[189][4], s.dn[189][5], s.dn[189][6], eq128_e1550_d_n7, s.dn[189][8], eq128_e1550_d_n9, s.dn[189][10], s.dn[189][11], s.dn[189][12], s.dn[189][13], s.dn[189][14], s.dn[189][15], s.dn[189][16], s.dn[189][17], s.dn[189][18], s.dn[189][19], s.dn[189][20], s.dn[189][21], s.dn[189][22], s.dn[189][23], s.dn[189][24], s.dn[189][25], s.dn[189][26], s.dn[189][27], s.dn[189][28], s.dn[189][29], s.db[189][0], s.db[189][1], s.db[189][2], s.db[189][3], s.db[189][4], s.db[189][5], s.db[189][6], s.db[189][7], s.db[189][8], s.db[189][9], s.db[189][10], s.db[189][11], s.db[189][12], s.db[189][13], s.db[189][14], s.db[189][15], s.db[189][16], s.db[189][17], s.db[189][18], s.db[189][19], s.db[189][20], s.db[189][21], s.db[189][22], s.db[189][23], s.db[189][24], s.db[189][25], s.db[189][26], s.db[189][27], s.db[189][28], s.db[189][29], s.db[189][30], s.db[189][31], s.db[189][32], s.db[189][33], s.db[189][34], s.db[189][35], eq128_e1550_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 30] = [eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29];
        let eq128_reactive_branch_derivatives: [f64; 36] = [eq128_e1552_d_b0, eq128_e1552_d_b1, eq128_e1552_d_b2, eq128_e1552_d_b3, eq128_e1552_d_b4, eq128_e1552_d_b5, eq128_e1552_d_b6, eq128_e1552_d_b7, eq128_e1552_d_b8, eq128_e1552_d_b9, eq128_e1552_d_b10, eq128_e1552_d_b11, eq128_e1552_d_b12, eq128_e1552_d_b13, eq128_e1552_d_b14, eq128_e1552_d_b15, eq128_e1552_d_b16, eq128_e1552_d_b17, eq128_e1552_d_b18, eq128_e1552_d_b19, eq128_e1552_d_b20, eq128_e1552_d_b21, eq128_e1552_d_b22, eq128_e1552_d_b23, eq128_e1552_d_b24, eq128_e1552_d_b25, eq128_e1552_d_b26, eq128_e1552_d_b27, eq128_e1552_d_b28, eq128_e1552_d_b29, eq128_e1552_d_b30, eq128_e1552_d_b31, eq128_e1552_d_b32, eq128_e1552_d_b33, eq128_e1552_d_b34, eq128_e1552_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29, eq129_e1563_d_b0, eq129_e1563_d_b1, eq129_e1563_d_b2, eq129_e1563_d_b3, eq129_e1563_d_b4, eq129_e1563_d_b5, eq129_e1563_d_b6, eq129_e1563_d_b7, eq129_e1563_d_b8, eq129_e1563_d_b9, eq129_e1563_d_b10, eq129_e1563_d_b11, eq129_e1563_d_b12, eq129_e1563_d_b13, eq129_e1563_d_b14, eq129_e1563_d_b15, eq129_e1563_d_b16, eq129_e1563_d_b17, eq129_e1563_d_b18, eq129_e1563_d_b19, eq129_e1563_d_b20, eq129_e1563_d_b21, eq129_e1563_d_b22, eq129_e1563_d_b23, eq129_e1563_d_b24, eq129_e1563_d_b25, eq129_e1563_d_b26, eq129_e1563_d_b27, eq129_e1563_d_b28, eq129_e1563_d_b29, eq129_e1563_d_b30, eq129_e1563_d_b31, eq129_e1563_d_b32, eq129_e1563_d_b33, eq129_e1563_d_b34, eq129_e1563_d_b35, eq129_e1563_q,) = {
    if (!s.b[1495]) {
        let eq129_e1556_q: f64 = s.v[185];
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1560_q: f64 = eq129_e1559;
        let eq129_e1561: f64 = (s.v[185] + eq129_e1559);
        let eq129_e1561_d_n2: f64 = (s.dn[185][2] + p.p355);
        let eq129_e1561_q: f64 = (eq129_e1556_q + eq129_e1560_q);
        (eq129_e1561, s.dn[185][0], s.dn[185][1], eq129_e1561_d_n2, s.dn[185][3], s.dn[185][4], s.dn[185][5], s.dn[185][6], s.dn[185][7], s.dn[185][8], s.dn[185][9], s.dn[185][10], s.dn[185][11], s.dn[185][12], __rspice_deriv_cse_4, s.dn[185][14], s.dn[185][15], s.dn[185][16], s.dn[185][17], s.dn[185][18], s.dn[185][19], s.dn[185][20], s.dn[185][21], s.dn[185][22], s.dn[185][23], s.dn[185][24], s.dn[185][25], s.dn[185][26], s.dn[185][27], s.dn[185][28], s.dn[185][29], s.db[185][0], s.db[185][1], s.db[185][2], s.db[185][3], s.db[185][4], s.db[185][5], s.db[185][6], s.db[185][7], s.db[185][8], s.db[185][9], s.db[185][10], s.db[185][11], s.db[185][12], s.db[185][13], s.db[185][14], s.db[185][15], s.db[185][16], s.db[185][17], s.db[185][18], s.db[185][19], s.db[185][20], s.db[185][21], s.db[185][22], s.db[185][23], s.db[185][24], s.db[185][25], s.db[185][26], s.db[185][27], s.db[185][28], s.db[185][29], s.db[185][30], s.db[185][31], s.db[185][32], s.db[185][33], s.db[185][34], s.db[185][35], eq129_e1561_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 30] = [eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29];
        let eq129_reactive_branch_derivatives: [f64; 36] = [eq129_e1563_d_b0, eq129_e1563_d_b1, eq129_e1563_d_b2, eq129_e1563_d_b3, eq129_e1563_d_b4, eq129_e1563_d_b5, eq129_e1563_d_b6, eq129_e1563_d_b7, eq129_e1563_d_b8, eq129_e1563_d_b9, eq129_e1563_d_b10, eq129_e1563_d_b11, eq129_e1563_d_b12, eq129_e1563_d_b13, eq129_e1563_d_b14, eq129_e1563_d_b15, eq129_e1563_d_b16, eq129_e1563_d_b17, eq129_e1563_d_b18, eq129_e1563_d_b19, eq129_e1563_d_b20, eq129_e1563_d_b21, eq129_e1563_d_b22, eq129_e1563_d_b23, eq129_e1563_d_b24, eq129_e1563_d_b25, eq129_e1563_d_b26, eq129_e1563_d_b27, eq129_e1563_d_b28, eq129_e1563_d_b29, eq129_e1563_d_b30, eq129_e1563_d_b31, eq129_e1563_d_b32, eq129_e1563_d_b33, eq129_e1563_d_b34, eq129_e1563_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29, eq130_e1574_d_b0, eq130_e1574_d_b1, eq130_e1574_d_b2, eq130_e1574_d_b3, eq130_e1574_d_b4, eq130_e1574_d_b5, eq130_e1574_d_b6, eq130_e1574_d_b7, eq130_e1574_d_b8, eq130_e1574_d_b9, eq130_e1574_d_b10, eq130_e1574_d_b11, eq130_e1574_d_b12, eq130_e1574_d_b13, eq130_e1574_d_b14, eq130_e1574_d_b15, eq130_e1574_d_b16, eq130_e1574_d_b17, eq130_e1574_d_b18, eq130_e1574_d_b19, eq130_e1574_d_b20, eq130_e1574_d_b21, eq130_e1574_d_b22, eq130_e1574_d_b23, eq130_e1574_d_b24, eq130_e1574_d_b25, eq130_e1574_d_b26, eq130_e1574_d_b27, eq130_e1574_d_b28, eq130_e1574_d_b29, eq130_e1574_d_b30, eq130_e1574_d_b31, eq130_e1574_d_b32, eq130_e1574_d_b33, eq130_e1574_d_b34, eq130_e1574_d_b35, eq130_e1574_q,) = {
    if (!s.b[1495]) {
        let eq130_e1567_q: f64 = s.v[186];
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1571_q: f64 = eq130_e1570;
        let eq130_e1572: f64 = (s.v[186] + eq130_e1570);
        let eq130_e1572_d_n2: f64 = (s.dn[186][2] + p.p355);
        let eq130_e1572_q: f64 = (eq130_e1567_q + eq130_e1571_q);
        (eq130_e1572, s.dn[186][0], s.dn[186][1], eq130_e1572_d_n2, s.dn[186][3], s.dn[186][4], s.dn[186][5], s.dn[186][6], s.dn[186][7], s.dn[186][8], s.dn[186][9], s.dn[186][10], s.dn[186][11], __rspice_deriv_cse_5, s.dn[186][13], s.dn[186][14], s.dn[186][15], s.dn[186][16], s.dn[186][17], s.dn[186][18], s.dn[186][19], s.dn[186][20], s.dn[186][21], s.dn[186][22], s.dn[186][23], s.dn[186][24], s.dn[186][25], s.dn[186][26], s.dn[186][27], s.dn[186][28], s.dn[186][29], s.db[186][0], s.db[186][1], s.db[186][2], s.db[186][3], s.db[186][4], s.db[186][5], s.db[186][6], s.db[186][7], s.db[186][8], s.db[186][9], s.db[186][10], s.db[186][11], s.db[186][12], s.db[186][13], s.db[186][14], s.db[186][15], s.db[186][16], s.db[186][17], s.db[186][18], s.db[186][19], s.db[186][20], s.db[186][21], s.db[186][22], s.db[186][23], s.db[186][24], s.db[186][25], s.db[186][26], s.db[186][27], s.db[186][28], s.db[186][29], s.db[186][30], s.db[186][31], s.db[186][32], s.db[186][33], s.db[186][34], s.db[186][35], eq130_e1572_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 30] = [eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29];
        let eq130_reactive_branch_derivatives: [f64; 36] = [eq130_e1574_d_b0, eq130_e1574_d_b1, eq130_e1574_d_b2, eq130_e1574_d_b3, eq130_e1574_d_b4, eq130_e1574_d_b5, eq130_e1574_d_b6, eq130_e1574_d_b7, eq130_e1574_d_b8, eq130_e1574_d_b9, eq130_e1574_d_b10, eq130_e1574_d_b11, eq130_e1574_d_b12, eq130_e1574_d_b13, eq130_e1574_d_b14, eq130_e1574_d_b15, eq130_e1574_d_b16, eq130_e1574_d_b17, eq130_e1574_d_b18, eq130_e1574_d_b19, eq130_e1574_d_b20, eq130_e1574_d_b21, eq130_e1574_d_b22, eq130_e1574_d_b23, eq130_e1574_d_b24, eq130_e1574_d_b25, eq130_e1574_d_b26, eq130_e1574_d_b27, eq130_e1574_d_b28, eq130_e1574_d_b29, eq130_e1574_d_b30, eq130_e1574_d_b31, eq130_e1574_d_b32, eq130_e1574_d_b33, eq130_e1574_d_b34, eq130_e1574_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29, eq131_e1585_d_b0, eq131_e1585_d_b1, eq131_e1585_d_b2, eq131_e1585_d_b3, eq131_e1585_d_b4, eq131_e1585_d_b5, eq131_e1585_d_b6, eq131_e1585_d_b7, eq131_e1585_d_b8, eq131_e1585_d_b9, eq131_e1585_d_b10, eq131_e1585_d_b11, eq131_e1585_d_b12, eq131_e1585_d_b13, eq131_e1585_d_b14, eq131_e1585_d_b15, eq131_e1585_d_b16, eq131_e1585_d_b17, eq131_e1585_d_b18, eq131_e1585_d_b19, eq131_e1585_d_b20, eq131_e1585_d_b21, eq131_e1585_d_b22, eq131_e1585_d_b23, eq131_e1585_d_b24, eq131_e1585_d_b25, eq131_e1585_d_b26, eq131_e1585_d_b27, eq131_e1585_d_b28, eq131_e1585_d_b29, eq131_e1585_d_b30, eq131_e1585_d_b31, eq131_e1585_d_b32, eq131_e1585_d_b33, eq131_e1585_d_b34, eq131_e1585_d_b35, eq131_e1585_q,) = {
    if (!s.b[1495]) {
        let eq131_e1578_q: f64 = s.v[187];
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1582_q: f64 = eq131_e1581;
        let eq131_e1583: f64 = (s.v[187] + eq131_e1581);
        let eq131_e1583_d_n7: f64 = (s.dn[187][7] + p.p355);
        let eq131_e1583_q: f64 = (eq131_e1578_q + eq131_e1582_q);
        (eq131_e1583, s.dn[187][0], s.dn[187][1], s.dn[187][2], s.dn[187][3], s.dn[187][4], s.dn[187][5], s.dn[187][6], eq131_e1583_d_n7, s.dn[187][8], s.dn[187][9], s.dn[187][10], s.dn[187][11], s.dn[187][12], __rspice_deriv_cse_6, s.dn[187][14], s.dn[187][15], s.dn[187][16], s.dn[187][17], s.dn[187][18], s.dn[187][19], s.dn[187][20], s.dn[187][21], s.dn[187][22], s.dn[187][23], s.dn[187][24], s.dn[187][25], s.dn[187][26], s.dn[187][27], s.dn[187][28], s.dn[187][29], s.db[187][0], s.db[187][1], s.db[187][2], s.db[187][3], s.db[187][4], s.db[187][5], s.db[187][6], s.db[187][7], s.db[187][8], s.db[187][9], s.db[187][10], s.db[187][11], s.db[187][12], s.db[187][13], s.db[187][14], s.db[187][15], s.db[187][16], s.db[187][17], s.db[187][18], s.db[187][19], s.db[187][20], s.db[187][21], s.db[187][22], s.db[187][23], s.db[187][24], s.db[187][25], s.db[187][26], s.db[187][27], s.db[187][28], s.db[187][29], s.db[187][30], s.db[187][31], s.db[187][32], s.db[187][33], s.db[187][34], s.db[187][35], eq131_e1583_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 30] = [eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29];
        let eq131_reactive_branch_derivatives: [f64; 36] = [eq131_e1585_d_b0, eq131_e1585_d_b1, eq131_e1585_d_b2, eq131_e1585_d_b3, eq131_e1585_d_b4, eq131_e1585_d_b5, eq131_e1585_d_b6, eq131_e1585_d_b7, eq131_e1585_d_b8, eq131_e1585_d_b9, eq131_e1585_d_b10, eq131_e1585_d_b11, eq131_e1585_d_b12, eq131_e1585_d_b13, eq131_e1585_d_b14, eq131_e1585_d_b15, eq131_e1585_d_b16, eq131_e1585_d_b17, eq131_e1585_d_b18, eq131_e1585_d_b19, eq131_e1585_d_b20, eq131_e1585_d_b21, eq131_e1585_d_b22, eq131_e1585_d_b23, eq131_e1585_d_b24, eq131_e1585_d_b25, eq131_e1585_d_b26, eq131_e1585_d_b27, eq131_e1585_d_b28, eq131_e1585_d_b29, eq131_e1585_d_b30, eq131_e1585_d_b31, eq131_e1585_d_b32, eq131_e1585_d_b33, eq131_e1585_d_b34, eq131_e1585_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let eq134_e1597_q: f64 = s.v[188];
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1601_q: f64 = eq134_e1600;
        let eq134_e1602: f64 = (s.v[188] + eq134_e1600);
        let eq134_e1602_d_n3: f64 = (s.dn[188][3] + p.p355);
        let eq134_e1602_d_n13: f64 = (s.dn[188][13] + (-p.p355));
        let eq134_e1602_q: f64 = (eq134_e1597_q + eq134_e1601_q);
        let eq134_reactive_node_derivatives: [f64; 30] = [s.dn[188][0], s.dn[188][1], s.dn[188][2], eq134_e1602_d_n3, s.dn[188][4], s.dn[188][5], s.dn[188][6], s.dn[188][7], s.dn[188][8], s.dn[188][9], s.dn[188][10], s.dn[188][11], s.dn[188][12], eq134_e1602_d_n13, s.dn[188][14], s.dn[188][15], s.dn[188][16], s.dn[188][17], s.dn[188][18], s.dn[188][19], s.dn[188][20], s.dn[188][21], s.dn[188][22], s.dn[188][23], s.dn[188][24], s.dn[188][25], s.dn[188][26], s.dn[188][27], s.dn[188][28], s.dn[188][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &s.db[188],
            multiplicity,
        );
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29, eq142_e1656_d_b0, eq142_e1656_d_b1, eq142_e1656_d_b2, eq142_e1656_d_b3, eq142_e1656_d_b4, eq142_e1656_d_b5, eq142_e1656_d_b6, eq142_e1656_d_b7, eq142_e1656_d_b8, eq142_e1656_d_b9, eq142_e1656_d_b10, eq142_e1656_d_b11, eq142_e1656_d_b12, eq142_e1656_d_b13, eq142_e1656_d_b14, eq142_e1656_d_b15, eq142_e1656_d_b16, eq142_e1656_d_b17, eq142_e1656_d_b18, eq142_e1656_d_b19, eq142_e1656_d_b20, eq142_e1656_d_b21, eq142_e1656_d_b22, eq142_e1656_d_b23, eq142_e1656_d_b24, eq142_e1656_d_b25, eq142_e1656_d_b26, eq142_e1656_d_b27, eq142_e1656_d_b28, eq142_e1656_d_b29, eq142_e1656_d_b30, eq142_e1656_d_b31, eq142_e1656_d_b32, eq142_e1656_d_b33, eq142_e1656_d_b34, eq142_e1656_d_b35, eq142_e1656_q, eq142_e1656_q_d_n28,) = {
    if (!s.b[1933]) {
        let eq142_e1649: f64 = (s.v[115] - (nv29 - 0.0));
        let eq142_e1649_d_n29: f64 = (s.dn[115][29] - 1.0);
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1653_q: f64 = eq142_e1652;
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1652);
        let eq142_e1654_d_n28: f64 = (s.dn[115][28] - p.p323);
        let eq142_e1654_q: f64 = (-eq142_e1653_q);
        (eq142_e1654, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], s.dn[115][5], s.dn[115][6], s.dn[115][7], s.dn[115][8], s.dn[115][9], s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], eq142_e1654_d_n28, eq142_e1649_d_n29, s.db[115][0], s.db[115][1], s.db[115][2], s.db[115][3], s.db[115][4], s.db[115][5], s.db[115][6], s.db[115][7], s.db[115][8], s.db[115][9], s.db[115][10], s.db[115][11], s.db[115][12], s.db[115][13], s.db[115][14], s.db[115][15], s.db[115][16], s.db[115][17], s.db[115][18], s.db[115][19], s.db[115][20], s.db[115][21], s.db[115][22], s.db[115][23], s.db[115][24], s.db[115][25], s.db[115][26], s.db[115][27], s.db[115][28], s.db[115][29], s.db[115][30], s.db[115][31], s.db[115][32], s.db[115][33], s.db[115][34], s.db[115][35], eq142_e1654_q, (-p.p323),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[28]),
            None,
            nodes[28],
            multiplicity * (eq142_e1656_q_d_n28),
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29, eq143_e1670_q, eq143_e1670_q_d_n29,) = {
    if (!s.b[1933]) {
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
        let eq145_e1681_q: f64 = s.v[117];
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1685_q: f64 = eq145_e1684;
        let eq145_e1686: f64 = (s.v[117] + eq145_e1684);
        let eq145_e1686_d_n8: f64 = (s.dn[117][8] + p.p355);
        let eq145_e1686_d_n9: f64 = (s.dn[117][9] + (-p.p355));
        let eq145_e1686_q: f64 = (eq145_e1681_q + eq145_e1685_q);
        let eq145_reactive_node_derivatives: [f64; 30] = [s.dn[117][0], s.dn[117][1], s.dn[117][2], s.dn[117][3], s.dn[117][4], s.dn[117][5], s.dn[117][6], s.dn[117][7], eq145_e1686_d_n8, eq145_e1686_d_n9, s.dn[117][10], s.dn[117][11], s.dn[117][12], s.dn[117][13], s.dn[117][14], s.dn[117][15], s.dn[117][16], s.dn[117][17], s.dn[117][18], s.dn[117][19], s.dn[117][20], s.dn[117][21], s.dn[117][22], s.dn[117][23], s.dn[117][24], s.dn[117][25], s.dn[117][26], s.dn[117][27], s.dn[117][28], s.dn[117][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &s.db[117],
            multiplicity,
        );
        let eq146_e1688_q: f64 = s.v[118];
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1692_q: f64 = eq146_e1691;
        let eq146_e1693: f64 = (s.v[118] + eq146_e1691);
        let eq146_e1693_d_n5: f64 = (s.dn[118][5] + (-p.p355));
        let eq146_e1693_d_n8: f64 = (s.dn[118][8] + p.p355);
        let eq146_e1693_q: f64 = (eq146_e1688_q + eq146_e1692_q);
        let eq146_reactive_node_derivatives: [f64; 30] = [s.dn[118][0], s.dn[118][1], s.dn[118][2], s.dn[118][3], s.dn[118][4], eq146_e1693_d_n5, s.dn[118][6], s.dn[118][7], eq146_e1693_d_n8, s.dn[118][9], s.dn[118][10], s.dn[118][11], s.dn[118][12], s.dn[118][13], s.dn[118][14], s.dn[118][15], s.dn[118][16], s.dn[118][17], s.dn[118][18], s.dn[118][19], s.dn[118][20], s.dn[118][21], s.dn[118][22], s.dn[118][23], s.dn[118][24], s.dn[118][25], s.dn[118][26], s.dn[118][27], s.dn[118][28], s.dn[118][29]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &s.db[118],
            multiplicity,
        );
        let (eq157_e1796, eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29, eq157_e1796_d_b0, eq157_e1796_d_b1, eq157_e1796_d_b2, eq157_e1796_d_b3, eq157_e1796_d_b4, eq157_e1796_d_b5, eq157_e1796_d_b6, eq157_e1796_d_b7, eq157_e1796_d_b8, eq157_e1796_d_b9, eq157_e1796_d_b10, eq157_e1796_d_b11, eq157_e1796_d_b12, eq157_e1796_d_b13, eq157_e1796_d_b14, eq157_e1796_d_b15, eq157_e1796_d_b16, eq157_e1796_d_b17, eq157_e1796_d_b18, eq157_e1796_d_b19, eq157_e1796_d_b20, eq157_e1796_d_b21, eq157_e1796_d_b22, eq157_e1796_d_b23, eq157_e1796_d_b24, eq157_e1796_d_b25, eq157_e1796_d_b26, eq157_e1796_d_b27, eq157_e1796_d_b28, eq157_e1796_d_b29, eq157_e1796_d_b30, eq157_e1796_d_b31, eq157_e1796_d_b32, eq157_e1796_d_b33, eq157_e1796_d_b34, eq157_e1796_d_b35, eq157_e1796_q,) = {
    if s.b[2418] {
        let eq157_e1794_q: f64 = s.v[242];
        (s.v[242], s.dn[242][0], s.dn[242][1], s.dn[242][2], s.dn[242][3], s.dn[242][4], s.dn[242][5], s.dn[242][6], s.dn[242][7], s.dn[242][8], s.dn[242][9], s.dn[242][10], s.dn[242][11], s.dn[242][12], s.dn[242][13], s.dn[242][14], s.dn[242][15], s.dn[242][16], s.dn[242][17], s.dn[242][18], s.dn[242][19], s.dn[242][20], s.dn[242][21], s.dn[242][22], s.dn[242][23], s.dn[242][24], s.dn[242][25], s.dn[242][26], s.dn[242][27], s.dn[242][28], s.dn[242][29], s.db[242][0], s.db[242][1], s.db[242][2], s.db[242][3], s.db[242][4], s.db[242][5], s.db[242][6], s.db[242][7], s.db[242][8], s.db[242][9], s.db[242][10], s.db[242][11], s.db[242][12], s.db[242][13], s.db[242][14], s.db[242][15], s.db[242][16], s.db[242][17], s.db[242][18], s.db[242][19], s.db[242][20], s.db[242][21], s.db[242][22], s.db[242][23], s.db[242][24], s.db[242][25], s.db[242][26], s.db[242][27], s.db[242][28], s.db[242][29], s.db[242][30], s.db[242][31], s.db[242][32], s.db[242][33], s.db[242][34], s.db[242][35], eq157_e1794_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 30] = [eq157_e1796_d_n0, eq157_e1796_d_n1, eq157_e1796_d_n2, eq157_e1796_d_n3, eq157_e1796_d_n4, eq157_e1796_d_n5, eq157_e1796_d_n6, eq157_e1796_d_n7, eq157_e1796_d_n8, eq157_e1796_d_n9, eq157_e1796_d_n10, eq157_e1796_d_n11, eq157_e1796_d_n12, eq157_e1796_d_n13, eq157_e1796_d_n14, eq157_e1796_d_n15, eq157_e1796_d_n16, eq157_e1796_d_n17, eq157_e1796_d_n18, eq157_e1796_d_n19, eq157_e1796_d_n20, eq157_e1796_d_n21, eq157_e1796_d_n22, eq157_e1796_d_n23, eq157_e1796_d_n24, eq157_e1796_d_n25, eq157_e1796_d_n26, eq157_e1796_d_n27, eq157_e1796_d_n28, eq157_e1796_d_n29];
        let eq157_reactive_branch_derivatives: [f64; 36] = [eq157_e1796_d_b0, eq157_e1796_d_b1, eq157_e1796_d_b2, eq157_e1796_d_b3, eq157_e1796_d_b4, eq157_e1796_d_b5, eq157_e1796_d_b6, eq157_e1796_d_b7, eq157_e1796_d_b8, eq157_e1796_d_b9, eq157_e1796_d_b10, eq157_e1796_d_b11, eq157_e1796_d_b12, eq157_e1796_d_b13, eq157_e1796_d_b14, eq157_e1796_d_b15, eq157_e1796_d_b16, eq157_e1796_d_b17, eq157_e1796_d_b18, eq157_e1796_d_b19, eq157_e1796_d_b20, eq157_e1796_d_b21, eq157_e1796_d_b22, eq157_e1796_d_b23, eq157_e1796_d_b24, eq157_e1796_d_b25, eq157_e1796_d_b26, eq157_e1796_d_b27, eq157_e1796_d_b28, eq157_e1796_d_b29, eq157_e1796_d_b30, eq157_e1796_d_b31, eq157_e1796_d_b32, eq157_e1796_d_b33, eq157_e1796_d_b34, eq157_e1796_d_b35];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq157_reactive_node_derivatives,
            branches,
            &eq157_reactive_branch_derivatives,
            multiplicity,
        );
        let eq172_e1881_q: f64 = s.v[214];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &s.dn[214],
            branches,
            &s.db[214],
            multiplicity,
        );
        let eq173_e1883_q: f64 = s.v[215];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes,
            &s.dn[215],
            branches,
            &s.db[215],
            multiplicity,
        );
        let eq174_e1885_q: f64 = s.v[216];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &s.dn[216],
            branches,
            &s.db[216],
            multiplicity,
        );
        let eq175_e1887_q: f64 = s.v[218];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            nodes,
            &s.dn[218],
            branches,
            &s.db[218],
            multiplicity,
        );
        let eq176_e1889_q: f64 = s.v[217];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &s.dn[217],
            branches,
            &s.db[217],
            multiplicity,
        );
        let eq177_e1891_q: f64 = s.v[219];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &s.dn[219],
            branches,
            &s.db[219],
            multiplicity,
        );
        let (eq194_e2167, eq194_e2167_d_n4, eq194_e2167_q,) = {
    if s.b[2700] {
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
