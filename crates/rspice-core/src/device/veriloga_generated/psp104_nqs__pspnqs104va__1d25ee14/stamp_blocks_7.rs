#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_14(
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
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq68_e1475: f64 = (s.v[19] * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq68_e1476);
        let eq68_e1476_d_n0: f64 = ((s.dn[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n1: f64 = ((s.dn[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n2: f64 = ((s.dn[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n3: f64 = ((s.dn[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n4: f64 = ((s.dn[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n5: f64 = ((s.dn[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n6: f64 = ((s.dn[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n7: f64 = ((s.dn[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n8: f64 = ((s.dn[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n9: f64 = ((s.dn[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n10: f64 = ((s.dn[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n11: f64 = ((s.dn[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n12: f64 = ((s.dn[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n13: f64 = ((s.dn[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n14: f64 = ((s.dn[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n15: f64 = ((s.dn[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n16: f64 = ((s.dn[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n17: f64 = ((s.dn[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n18: f64 = ((s.dn[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n19: f64 = ((s.dn[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_n20: f64 = ((s.dn[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b0: f64 = ((s.db[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b1: f64 = ((s.db[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b2: f64 = ((s.db[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b3: f64 = ((s.db[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b4: f64 = ((s.db[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b5: f64 = ((s.db[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b6: f64 = ((s.db[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b7: f64 = ((s.db[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b8: f64 = ((s.db[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b9: f64 = ((s.db[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b10: f64 = ((s.db[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b11: f64 = ((s.db[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b12: f64 = ((s.db[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b13: f64 = ((s.db[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b14: f64 = ((s.db[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b15: f64 = ((s.db[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b16: f64 = ((s.db[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b17: f64 = ((s.db[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b18: f64 = ((s.db[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b19: f64 = ((s.db[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b20: f64 = ((s.db[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b21: f64 = ((s.db[19][21] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b22: f64 = ((s.db[19][22] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b23: f64 = ((s.db[19][23] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1476_d_b24: f64 = ((s.db[19][24] * p.p32) * __rspice_inv_cse_0);
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1478_d_n0: f64 = (eq68_e1476_d_n0 * 0.5);
        let eq68_e1478_d_n1: f64 = (eq68_e1476_d_n1 * 0.5);
        let eq68_e1478_d_n2: f64 = (eq68_e1476_d_n2 * 0.5);
        let eq68_e1478_d_n3: f64 = (eq68_e1476_d_n3 * 0.5);
        let eq68_e1478_d_n4: f64 = (eq68_e1476_d_n4 * 0.5);
        let eq68_e1478_d_n5: f64 = (eq68_e1476_d_n5 * 0.5);
        let eq68_e1478_d_n6: f64 = (eq68_e1476_d_n6 * 0.5);
        let eq68_e1478_d_n7: f64 = (eq68_e1476_d_n7 * 0.5);
        let eq68_e1478_d_n8: f64 = (eq68_e1476_d_n8 * 0.5);
        let eq68_e1478_d_n9: f64 = (eq68_e1476_d_n9 * 0.5);
        let eq68_e1478_d_n10: f64 = (eq68_e1476_d_n10 * 0.5);
        let eq68_e1478_d_n11: f64 = (eq68_e1476_d_n11 * 0.5);
        let eq68_e1478_d_n12: f64 = (eq68_e1476_d_n12 * 0.5);
        let eq68_e1478_d_n13: f64 = (eq68_e1476_d_n13 * 0.5);
        let eq68_e1478_d_n14: f64 = (eq68_e1476_d_n14 * 0.5);
        let eq68_e1478_d_n15: f64 = (eq68_e1476_d_n15 * 0.5);
        let eq68_e1478_d_n16: f64 = (eq68_e1476_d_n16 * 0.5);
        let eq68_e1478_d_n17: f64 = (eq68_e1476_d_n17 * 0.5);
        let eq68_e1478_d_n18: f64 = (eq68_e1476_d_n18 * 0.5);
        let eq68_e1478_d_n19: f64 = (eq68_e1476_d_n19 * 0.5);
        let eq68_e1478_d_n20: f64 = (eq68_e1476_d_n20 * 0.5);
        let eq68_e1478_d_b0: f64 = (eq68_e1476_d_b0 * 0.5);
        let eq68_e1478_d_b1: f64 = (eq68_e1476_d_b1 * 0.5);
        let eq68_e1478_d_b2: f64 = (eq68_e1476_d_b2 * 0.5);
        let eq68_e1478_d_b3: f64 = (eq68_e1476_d_b3 * 0.5);
        let eq68_e1478_d_b4: f64 = (eq68_e1476_d_b4 * 0.5);
        let eq68_e1478_d_b5: f64 = (eq68_e1476_d_b5 * 0.5);
        let eq68_e1478_d_b6: f64 = (eq68_e1476_d_b6 * 0.5);
        let eq68_e1478_d_b7: f64 = (eq68_e1476_d_b7 * 0.5);
        let eq68_e1478_d_b8: f64 = (eq68_e1476_d_b8 * 0.5);
        let eq68_e1478_d_b9: f64 = (eq68_e1476_d_b9 * 0.5);
        let eq68_e1478_d_b10: f64 = (eq68_e1476_d_b10 * 0.5);
        let eq68_e1478_d_b11: f64 = (eq68_e1476_d_b11 * 0.5);
        let eq68_e1478_d_b12: f64 = (eq68_e1476_d_b12 * 0.5);
        let eq68_e1478_d_b13: f64 = (eq68_e1476_d_b13 * 0.5);
        let eq68_e1478_d_b14: f64 = (eq68_e1476_d_b14 * 0.5);
        let eq68_e1478_d_b15: f64 = (eq68_e1476_d_b15 * 0.5);
        let eq68_e1478_d_b16: f64 = (eq68_e1476_d_b16 * 0.5);
        let eq68_e1478_d_b17: f64 = (eq68_e1476_d_b17 * 0.5);
        let eq68_e1478_d_b18: f64 = (eq68_e1476_d_b18 * 0.5);
        let eq68_e1478_d_b19: f64 = (eq68_e1476_d_b19 * 0.5);
        let eq68_e1478_d_b20: f64 = (eq68_e1476_d_b20 * 0.5);
        let eq68_e1478_d_b21: f64 = (eq68_e1476_d_b21 * 0.5);
        let eq68_e1478_d_b22: f64 = (eq68_e1476_d_b22 * 0.5);
        let eq68_e1478_d_b23: f64 = (eq68_e1476_d_b23 * 0.5);
        let eq68_e1478_d_b24: f64 = (eq68_e1476_d_b24 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * s.v[860]);
        let eq68_e1480_d_n0: f64 = ((eq68_e1478_d_n0 * s.v[860]) + (eq68_e1478 * s.dn[860][0]));
        let eq68_e1480_d_n1: f64 = ((eq68_e1478_d_n1 * s.v[860]) + (eq68_e1478 * s.dn[860][1]));
        let eq68_e1480_d_n2: f64 = ((eq68_e1478_d_n2 * s.v[860]) + (eq68_e1478 * s.dn[860][2]));
        let eq68_e1480_d_n3: f64 = ((eq68_e1478_d_n3 * s.v[860]) + (eq68_e1478 * s.dn[860][3]));
        let eq68_e1480_d_n4: f64 = ((eq68_e1478_d_n4 * s.v[860]) + (eq68_e1478 * s.dn[860][4]));
        let eq68_e1480_d_n5: f64 = ((eq68_e1478_d_n5 * s.v[860]) + (eq68_e1478 * s.dn[860][5]));
        let eq68_e1480_d_n6: f64 = ((eq68_e1478_d_n6 * s.v[860]) + (eq68_e1478 * s.dn[860][6]));
        let eq68_e1480_d_n7: f64 = ((eq68_e1478_d_n7 * s.v[860]) + (eq68_e1478 * s.dn[860][7]));
        let eq68_e1480_d_n8: f64 = ((eq68_e1478_d_n8 * s.v[860]) + (eq68_e1478 * s.dn[860][8]));
        let eq68_e1480_d_n9: f64 = ((eq68_e1478_d_n9 * s.v[860]) + (eq68_e1478 * s.dn[860][9]));
        let eq68_e1480_d_n10: f64 = ((eq68_e1478_d_n10 * s.v[860]) + (eq68_e1478 * s.dn[860][10]));
        let eq68_e1480_d_n11: f64 = ((eq68_e1478_d_n11 * s.v[860]) + (eq68_e1478 * s.dn[860][11]));
        let eq68_e1480_d_n12: f64 = ((eq68_e1478_d_n12 * s.v[860]) + (eq68_e1478 * s.dn[860][12]));
        let eq68_e1480_d_n13: f64 = ((eq68_e1478_d_n13 * s.v[860]) + (eq68_e1478 * s.dn[860][13]));
        let eq68_e1480_d_n14: f64 = ((eq68_e1478_d_n14 * s.v[860]) + (eq68_e1478 * s.dn[860][14]));
        let eq68_e1480_d_n15: f64 = ((eq68_e1478_d_n15 * s.v[860]) + (eq68_e1478 * s.dn[860][15]));
        let eq68_e1480_d_n16: f64 = ((eq68_e1478_d_n16 * s.v[860]) + (eq68_e1478 * s.dn[860][16]));
        let eq68_e1480_d_n17: f64 = ((eq68_e1478_d_n17 * s.v[860]) + (eq68_e1478 * s.dn[860][17]));
        let eq68_e1480_d_n18: f64 = ((eq68_e1478_d_n18 * s.v[860]) + (eq68_e1478 * s.dn[860][18]));
        let eq68_e1480_d_n19: f64 = ((eq68_e1478_d_n19 * s.v[860]) + (eq68_e1478 * s.dn[860][19]));
        let eq68_e1480_d_n20: f64 = ((eq68_e1478_d_n20 * s.v[860]) + (eq68_e1478 * s.dn[860][20]));
        let eq68_e1480_d_b0: f64 = ((eq68_e1478_d_b0 * s.v[860]) + (eq68_e1478 * s.db[860][0]));
        let eq68_e1480_d_b1: f64 = ((eq68_e1478_d_b1 * s.v[860]) + (eq68_e1478 * s.db[860][1]));
        let eq68_e1480_d_b2: f64 = ((eq68_e1478_d_b2 * s.v[860]) + (eq68_e1478 * s.db[860][2]));
        let eq68_e1480_d_b3: f64 = ((eq68_e1478_d_b3 * s.v[860]) + (eq68_e1478 * s.db[860][3]));
        let eq68_e1480_d_b4: f64 = ((eq68_e1478_d_b4 * s.v[860]) + (eq68_e1478 * s.db[860][4]));
        let eq68_e1480_d_b5: f64 = ((eq68_e1478_d_b5 * s.v[860]) + (eq68_e1478 * s.db[860][5]));
        let eq68_e1480_d_b6: f64 = ((eq68_e1478_d_b6 * s.v[860]) + (eq68_e1478 * s.db[860][6]));
        let eq68_e1480_d_b7: f64 = ((eq68_e1478_d_b7 * s.v[860]) + (eq68_e1478 * s.db[860][7]));
        let eq68_e1480_d_b8: f64 = ((eq68_e1478_d_b8 * s.v[860]) + (eq68_e1478 * s.db[860][8]));
        let eq68_e1480_d_b9: f64 = ((eq68_e1478_d_b9 * s.v[860]) + (eq68_e1478 * s.db[860][9]));
        let eq68_e1480_d_b10: f64 = ((eq68_e1478_d_b10 * s.v[860]) + (eq68_e1478 * s.db[860][10]));
        let eq68_e1480_d_b11: f64 = ((eq68_e1478_d_b11 * s.v[860]) + (eq68_e1478 * s.db[860][11]));
        let eq68_e1480_d_b12: f64 = ((eq68_e1478_d_b12 * s.v[860]) + (eq68_e1478 * s.db[860][12]));
        let eq68_e1480_d_b13: f64 = ((eq68_e1478_d_b13 * s.v[860]) + (eq68_e1478 * s.db[860][13]));
        let eq68_e1480_d_b14: f64 = ((eq68_e1478_d_b14 * s.v[860]) + (eq68_e1478 * s.db[860][14]));
        let eq68_e1480_d_b15: f64 = ((eq68_e1478_d_b15 * s.v[860]) + (eq68_e1478 * s.db[860][15]));
        let eq68_e1480_d_b16: f64 = ((eq68_e1478_d_b16 * s.v[860]) + (eq68_e1478 * s.db[860][16]));
        let eq68_e1480_d_b17: f64 = ((eq68_e1478_d_b17 * s.v[860]) + (eq68_e1478 * s.db[860][17]));
        let eq68_e1480_d_b18: f64 = ((eq68_e1478_d_b18 * s.v[860]) + (eq68_e1478 * s.db[860][18]));
        let eq68_e1480_d_b19: f64 = ((eq68_e1478_d_b19 * s.v[860]) + (eq68_e1478 * s.db[860][19]));
        let eq68_e1480_d_b20: f64 = ((eq68_e1478_d_b20 * s.v[860]) + (eq68_e1478 * s.db[860][20]));
        let eq68_e1480_d_b21: f64 = ((eq68_e1478_d_b21 * s.v[860]) + (eq68_e1478 * s.db[860][21]));
        let eq68_e1480_d_b22: f64 = ((eq68_e1478_d_b22 * s.v[860]) + (eq68_e1478 * s.db[860][22]));
        let eq68_e1480_d_b23: f64 = ((eq68_e1478_d_b23 * s.v[860]) + (eq68_e1478 * s.db[860][23]));
        let eq68_e1480_d_b24: f64 = ((eq68_e1478_d_b24 * s.v[860]) + (eq68_e1478 * s.db[860][24]));
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n0: f64 = (eq68_e1480_d_n0 * (nv4 - 0.0));
        let eq68_e1482_d_n1: f64 = (eq68_e1480_d_n1 * (nv4 - 0.0));
        let eq68_e1482_d_n2: f64 = (eq68_e1480_d_n2 * (nv4 - 0.0));
        let eq68_e1482_d_n3: f64 = (eq68_e1480_d_n3 * (nv4 - 0.0));
        let eq68_e1482_d_n4: f64 = ((eq68_e1480_d_n4 * (nv4 - 0.0)) + eq68_e1480);
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n9: f64 = (eq68_e1480_d_n9 * (nv4 - 0.0));
        let eq68_e1482_d_n10: f64 = (eq68_e1480_d_n10 * (nv4 - 0.0));
        let eq68_e1482_d_n11: f64 = (eq68_e1480_d_n11 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1482_d_b0: f64 = (eq68_e1480_d_b0 * (nv4 - 0.0));
        let eq68_e1482_d_b1: f64 = (eq68_e1480_d_b1 * (nv4 - 0.0));
        let eq68_e1482_d_b2: f64 = (eq68_e1480_d_b2 * (nv4 - 0.0));
        let eq68_e1482_d_b3: f64 = (eq68_e1480_d_b3 * (nv4 - 0.0));
        let eq68_e1482_d_b4: f64 = (eq68_e1480_d_b4 * (nv4 - 0.0));
        let eq68_e1482_d_b5: f64 = (eq68_e1480_d_b5 * (nv4 - 0.0));
        let eq68_e1482_d_b6: f64 = (eq68_e1480_d_b6 * (nv4 - 0.0));
        let eq68_e1482_d_b7: f64 = (eq68_e1480_d_b7 * (nv4 - 0.0));
        let eq68_e1482_d_b8: f64 = (eq68_e1480_d_b8 * (nv4 - 0.0));
        let eq68_e1482_d_b9: f64 = (eq68_e1480_d_b9 * (nv4 - 0.0));
        let eq68_e1482_d_b10: f64 = (eq68_e1480_d_b10 * (nv4 - 0.0));
        let eq68_e1482_d_b11: f64 = (eq68_e1480_d_b11 * (nv4 - 0.0));
        let eq68_e1482_d_b12: f64 = (eq68_e1480_d_b12 * (nv4 - 0.0));
        let eq68_e1482_d_b13: f64 = (eq68_e1480_d_b13 * (nv4 - 0.0));
        let eq68_e1482_d_b14: f64 = (eq68_e1480_d_b14 * (nv4 - 0.0));
        let eq68_e1482_d_b15: f64 = (eq68_e1480_d_b15 * (nv4 - 0.0));
        let eq68_e1482_d_b16: f64 = (eq68_e1480_d_b16 * (nv4 - 0.0));
        let eq68_e1482_d_b17: f64 = (eq68_e1480_d_b17 * (nv4 - 0.0));
        let eq68_e1482_d_b18: f64 = (eq68_e1480_d_b18 * (nv4 - 0.0));
        let eq68_e1482_d_b19: f64 = (eq68_e1480_d_b19 * (nv4 - 0.0));
        let eq68_e1482_d_b20: f64 = (eq68_e1480_d_b20 * (nv4 - 0.0));
        let eq68_e1482_d_b21: f64 = (eq68_e1480_d_b21 * (nv4 - 0.0));
        let eq68_e1482_d_b22: f64 = (eq68_e1480_d_b22 * (nv4 - 0.0));
        let eq68_e1482_d_b23: f64 = (eq68_e1480_d_b23 * (nv4 - 0.0));
        let eq68_e1482_d_b24: f64 = (eq68_e1480_d_b24 * (nv4 - 0.0));
        let eq68_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq68_e1482);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n0: f64 = (-(eq68_e1482_d_n0 * ddt_scale));
        let eq68_e1484_d_n1: f64 = (-(eq68_e1482_d_n1 * ddt_scale));
        let eq68_e1484_d_n2: f64 = (-(eq68_e1482_d_n2 * ddt_scale));
        let eq68_e1484_d_n3: f64 = (-(eq68_e1482_d_n3 * ddt_scale));
        let eq68_e1484_d_n4: f64 = (-(eq68_e1482_d_n4 * ddt_scale));
        let eq68_e1484_d_n5: f64 = (-(eq68_e1482_d_n5 * ddt_scale));
        let eq68_e1484_d_n6: f64 = (-(eq68_e1482_d_n6 * ddt_scale));
        let eq68_e1484_d_n7: f64 = (-(eq68_e1482_d_n7 * ddt_scale));
        let eq68_e1484_d_n8: f64 = (-(eq68_e1482_d_n8 * ddt_scale));
        let eq68_e1484_d_n9: f64 = (-(eq68_e1482_d_n9 * ddt_scale));
        let eq68_e1484_d_n10: f64 = (-(eq68_e1482_d_n10 * ddt_scale));
        let eq68_e1484_d_n11: f64 = (-(eq68_e1482_d_n11 * ddt_scale));
        let eq68_e1484_d_n12: f64 = (-(eq68_e1482_d_n12 * ddt_scale));
        let eq68_e1484_d_n13: f64 = (-(eq68_e1482_d_n13 * ddt_scale));
        let eq68_e1484_d_n14: f64 = (-(eq68_e1482_d_n14 * ddt_scale));
        let eq68_e1484_d_n15: f64 = (-(eq68_e1482_d_n15 * ddt_scale));
        let eq68_e1484_d_n16: f64 = (-(eq68_e1482_d_n16 * ddt_scale));
        let eq68_e1484_d_n17: f64 = (-(eq68_e1482_d_n17 * ddt_scale));
        let eq68_e1484_d_n18: f64 = (-(eq68_e1482_d_n18 * ddt_scale));
        let eq68_e1484_d_n19: f64 = (-(eq68_e1482_d_n19 * ddt_scale));
        let eq68_e1484_d_n20: f64 = (-(eq68_e1482_d_n20 * ddt_scale));
        let eq68_e1484_d_b0: f64 = (-(eq68_e1482_d_b0 * ddt_scale));
        let eq68_e1484_d_b1: f64 = (-(eq68_e1482_d_b1 * ddt_scale));
        let eq68_e1484_d_b2: f64 = (-(eq68_e1482_d_b2 * ddt_scale));
        let eq68_e1484_d_b3: f64 = (-(eq68_e1482_d_b3 * ddt_scale));
        let eq68_e1484_d_b4: f64 = (-(eq68_e1482_d_b4 * ddt_scale));
        let eq68_e1484_d_b5: f64 = (-(eq68_e1482_d_b5 * ddt_scale));
        let eq68_e1484_d_b6: f64 = (-(eq68_e1482_d_b6 * ddt_scale));
        let eq68_e1484_d_b7: f64 = (-(eq68_e1482_d_b7 * ddt_scale));
        let eq68_e1484_d_b8: f64 = (-(eq68_e1482_d_b8 * ddt_scale));
        let eq68_e1484_d_b9: f64 = (-(eq68_e1482_d_b9 * ddt_scale));
        let eq68_e1484_d_b10: f64 = (-(eq68_e1482_d_b10 * ddt_scale));
        let eq68_e1484_d_b11: f64 = (-(eq68_e1482_d_b11 * ddt_scale));
        let eq68_e1484_d_b12: f64 = (-(eq68_e1482_d_b12 * ddt_scale));
        let eq68_e1484_d_b13: f64 = (-(eq68_e1482_d_b13 * ddt_scale));
        let eq68_e1484_d_b14: f64 = (-(eq68_e1482_d_b14 * ddt_scale));
        let eq68_e1484_d_b15: f64 = (-(eq68_e1482_d_b15 * ddt_scale));
        let eq68_e1484_d_b16: f64 = (-(eq68_e1482_d_b16 * ddt_scale));
        let eq68_e1484_d_b17: f64 = (-(eq68_e1482_d_b17 * ddt_scale));
        let eq68_e1484_d_b18: f64 = (-(eq68_e1482_d_b18 * ddt_scale));
        let eq68_e1484_d_b19: f64 = (-(eq68_e1482_d_b19 * ddt_scale));
        let eq68_e1484_d_b20: f64 = (-(eq68_e1482_d_b20 * ddt_scale));
        let eq68_e1484_d_b21: f64 = (-(eq68_e1482_d_b21 * ddt_scale));
        let eq68_e1484_d_b22: f64 = (-(eq68_e1482_d_b22 * ddt_scale));
        let eq68_e1484_d_b23: f64 = (-(eq68_e1482_d_b23 * ddt_scale));
        let eq68_e1484_d_b24: f64 = (-(eq68_e1482_d_b24 * ddt_scale));
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivatives: [f64; 21] = [eq68_e1484_d_n0, eq68_e1484_d_n1, eq68_e1484_d_n2, eq68_e1484_d_n3, eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n9, eq68_e1484_d_n10, eq68_e1484_d_n11, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivatives: [f64; 25] = [eq68_e1484_d_b0, eq68_e1484_d_b1, eq68_e1484_d_b2, eq68_e1484_d_b3, eq68_e1484_d_b4, eq68_e1484_d_b5, eq68_e1484_d_b6, eq68_e1484_d_b7, eq68_e1484_d_b8, eq68_e1484_d_b9, eq68_e1484_d_b10, eq68_e1484_d_b11, eq68_e1484_d_b12, eq68_e1484_d_b13, eq68_e1484_d_b14, eq68_e1484_d_b15, eq68_e1484_d_b16, eq68_e1484_d_b17, eq68_e1484_d_b18, eq68_e1484_d_b19, eq68_e1484_d_b20, eq68_e1484_d_b21, eq68_e1484_d_b22, eq68_e1484_d_b23, eq68_e1484_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq68_value),
            &eq68_node_derivatives,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq56_e1387: f64 = (s.v[0] * s.v[19]);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1389_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq56_e1389_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq56_e1389_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq56_e1389_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq56_e1389_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq56_e1389_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq56_e1389_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq56_e1389_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq56_e1389_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq56_e1389_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq56_e1389_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq56_e1389_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq56_e1389_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq56_e1389_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq56_e1389_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq56_e1389_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq56_e1389_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq56_e1389_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq56_e1389_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq56_e1389_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq56_e1389_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq56_e1389_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq56_e1389_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq56_e1389_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq56_e1389_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq56_e1389_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq56_e1389_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq56_e1389_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq56_e1389_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq56_e1389_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq56_e1389_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq56_e1389_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq56_e1389_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq56_e1389_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq56_e1389_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq56_e1389_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq56_e1389_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq56_e1389_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq56_e1389_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq56_e1389_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq56_e1389_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq56_e1389_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq56_e1389_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq56_e1389_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq56_e1389_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq56_e1389_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * s.v[851]);
        let eq56_e1391_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[851]) + (eq56_e1389 * s.dn[851][0]));
        let eq56_e1391_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[851]) + (eq56_e1389 * s.dn[851][1]));
        let eq56_e1391_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[851]) + (eq56_e1389 * s.dn[851][2]));
        let eq56_e1391_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[851]) + (eq56_e1389 * s.dn[851][3]));
        let eq56_e1391_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[851]) + (eq56_e1389 * s.dn[851][4]));
        let eq56_e1391_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[851]) + (eq56_e1389 * s.dn[851][5]));
        let eq56_e1391_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[851]) + (eq56_e1389 * s.dn[851][6]));
        let eq56_e1391_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[851]) + (eq56_e1389 * s.dn[851][7]));
        let eq56_e1391_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[851]) + (eq56_e1389 * s.dn[851][8]));
        let eq56_e1391_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[851]) + (eq56_e1389 * s.dn[851][9]));
        let eq56_e1391_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[851]) + (eq56_e1389 * s.dn[851][10]));
        let eq56_e1391_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[851]) + (eq56_e1389 * s.dn[851][11]));
        let eq56_e1391_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[851]) + (eq56_e1389 * s.dn[851][12]));
        let eq56_e1391_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[851]) + (eq56_e1389 * s.dn[851][13]));
        let eq56_e1391_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[851]) + (eq56_e1389 * s.dn[851][14]));
        let eq56_e1391_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[851]) + (eq56_e1389 * s.dn[851][15]));
        let eq56_e1391_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[851]) + (eq56_e1389 * s.dn[851][16]));
        let eq56_e1391_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[851]) + (eq56_e1389 * s.dn[851][17]));
        let eq56_e1391_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[851]) + (eq56_e1389 * s.dn[851][18]));
        let eq56_e1391_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[851]) + (eq56_e1389 * s.dn[851][19]));
        let eq56_e1391_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[851]) + (eq56_e1389 * s.dn[851][20]));
        let eq56_e1391_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[851]) + (eq56_e1389 * s.db[851][0]));
        let eq56_e1391_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[851]) + (eq56_e1389 * s.db[851][1]));
        let eq56_e1391_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[851]) + (eq56_e1389 * s.db[851][2]));
        let eq56_e1391_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[851]) + (eq56_e1389 * s.db[851][3]));
        let eq56_e1391_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[851]) + (eq56_e1389 * s.db[851][4]));
        let eq56_e1391_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[851]) + (eq56_e1389 * s.db[851][5]));
        let eq56_e1391_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[851]) + (eq56_e1389 * s.db[851][6]));
        let eq56_e1391_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[851]) + (eq56_e1389 * s.db[851][7]));
        let eq56_e1391_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[851]) + (eq56_e1389 * s.db[851][8]));
        let eq56_e1391_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[851]) + (eq56_e1389 * s.db[851][9]));
        let eq56_e1391_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[851]) + (eq56_e1389 * s.db[851][10]));
        let eq56_e1391_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[851]) + (eq56_e1389 * s.db[851][11]));
        let eq56_e1391_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[851]) + (eq56_e1389 * s.db[851][12]));
        let eq56_e1391_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[851]) + (eq56_e1389 * s.db[851][13]));
        let eq56_e1391_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[851]) + (eq56_e1389 * s.db[851][14]));
        let eq56_e1391_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[851]) + (eq56_e1389 * s.db[851][15]));
        let eq56_e1391_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[851]) + (eq56_e1389 * s.db[851][16]));
        let eq56_e1391_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[851]) + (eq56_e1389 * s.db[851][17]));
        let eq56_e1391_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[851]) + (eq56_e1389 * s.db[851][18]));
        let eq56_e1391_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[851]) + (eq56_e1389 * s.db[851][19]));
        let eq56_e1391_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[851]) + (eq56_e1389 * s.db[851][20]));
        let eq56_e1391_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[851]) + (eq56_e1389 * s.db[851][21]));
        let eq56_e1391_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[851]) + (eq56_e1389 * s.db[851][22]));
        let eq56_e1391_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[851]) + (eq56_e1389 * s.db[851][23]));
        let eq56_e1391_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[851]) + (eq56_e1389 * s.db[851][24]));
        let eq56_e1392_q: f64 = eq56_e1391;
        let eq56_reactive_node_derivatives: [f64; 21] = [eq56_e1391_d_n0, eq56_e1391_d_n1, eq56_e1391_d_n2, eq56_e1391_d_n3, eq56_e1391_d_n4, eq56_e1391_d_n5, eq56_e1391_d_n6, eq56_e1391_d_n7, eq56_e1391_d_n8, eq56_e1391_d_n9, eq56_e1391_d_n10, eq56_e1391_d_n11, eq56_e1391_d_n12, eq56_e1391_d_n13, eq56_e1391_d_n14, eq56_e1391_d_n15, eq56_e1391_d_n16, eq56_e1391_d_n17, eq56_e1391_d_n18, eq56_e1391_d_n19, eq56_e1391_d_n20];
        let eq56_reactive_branch_derivatives: [f64; 25] = [eq56_e1391_d_b0, eq56_e1391_d_b1, eq56_e1391_d_b2, eq56_e1391_d_b3, eq56_e1391_d_b4, eq56_e1391_d_b5, eq56_e1391_d_b6, eq56_e1391_d_b7, eq56_e1391_d_b8, eq56_e1391_d_b9, eq56_e1391_d_b10, eq56_e1391_d_b11, eq56_e1391_d_b12, eq56_e1391_d_b13, eq56_e1391_d_b14, eq56_e1391_d_b15, eq56_e1391_d_b16, eq56_e1391_d_b17, eq56_e1391_d_b18, eq56_e1391_d_b19, eq56_e1391_d_b20, eq56_e1391_d_b21, eq56_e1391_d_b22, eq56_e1391_d_b23, eq56_e1391_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (s.v[0] * s.v[19]);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * s.v[852]);
        let eq57_e1399_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[852]) + (eq57_e1397 * s.dn[852][0]));
        let eq57_e1399_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[852]) + (eq57_e1397 * s.dn[852][1]));
        let eq57_e1399_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[852]) + (eq57_e1397 * s.dn[852][2]));
        let eq57_e1399_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[852]) + (eq57_e1397 * s.dn[852][3]));
        let eq57_e1399_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[852]) + (eq57_e1397 * s.dn[852][4]));
        let eq57_e1399_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[852]) + (eq57_e1397 * s.dn[852][5]));
        let eq57_e1399_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[852]) + (eq57_e1397 * s.dn[852][6]));
        let eq57_e1399_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[852]) + (eq57_e1397 * s.dn[852][7]));
        let eq57_e1399_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[852]) + (eq57_e1397 * s.dn[852][8]));
        let eq57_e1399_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[852]) + (eq57_e1397 * s.dn[852][9]));
        let eq57_e1399_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[852]) + (eq57_e1397 * s.dn[852][10]));
        let eq57_e1399_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[852]) + (eq57_e1397 * s.dn[852][11]));
        let eq57_e1399_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[852]) + (eq57_e1397 * s.dn[852][12]));
        let eq57_e1399_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[852]) + (eq57_e1397 * s.dn[852][13]));
        let eq57_e1399_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[852]) + (eq57_e1397 * s.dn[852][14]));
        let eq57_e1399_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[852]) + (eq57_e1397 * s.dn[852][15]));
        let eq57_e1399_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[852]) + (eq57_e1397 * s.dn[852][16]));
        let eq57_e1399_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[852]) + (eq57_e1397 * s.dn[852][17]));
        let eq57_e1399_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[852]) + (eq57_e1397 * s.dn[852][18]));
        let eq57_e1399_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[852]) + (eq57_e1397 * s.dn[852][19]));
        let eq57_e1399_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[852]) + (eq57_e1397 * s.dn[852][20]));
        let eq57_e1399_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[852]) + (eq57_e1397 * s.db[852][0]));
        let eq57_e1399_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[852]) + (eq57_e1397 * s.db[852][1]));
        let eq57_e1399_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[852]) + (eq57_e1397 * s.db[852][2]));
        let eq57_e1399_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[852]) + (eq57_e1397 * s.db[852][3]));
        let eq57_e1399_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[852]) + (eq57_e1397 * s.db[852][4]));
        let eq57_e1399_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[852]) + (eq57_e1397 * s.db[852][5]));
        let eq57_e1399_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[852]) + (eq57_e1397 * s.db[852][6]));
        let eq57_e1399_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[852]) + (eq57_e1397 * s.db[852][7]));
        let eq57_e1399_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[852]) + (eq57_e1397 * s.db[852][8]));
        let eq57_e1399_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[852]) + (eq57_e1397 * s.db[852][9]));
        let eq57_e1399_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[852]) + (eq57_e1397 * s.db[852][10]));
        let eq57_e1399_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[852]) + (eq57_e1397 * s.db[852][11]));
        let eq57_e1399_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[852]) + (eq57_e1397 * s.db[852][12]));
        let eq57_e1399_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[852]) + (eq57_e1397 * s.db[852][13]));
        let eq57_e1399_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[852]) + (eq57_e1397 * s.db[852][14]));
        let eq57_e1399_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[852]) + (eq57_e1397 * s.db[852][15]));
        let eq57_e1399_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[852]) + (eq57_e1397 * s.db[852][16]));
        let eq57_e1399_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[852]) + (eq57_e1397 * s.db[852][17]));
        let eq57_e1399_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[852]) + (eq57_e1397 * s.db[852][18]));
        let eq57_e1399_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[852]) + (eq57_e1397 * s.db[852][19]));
        let eq57_e1399_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[852]) + (eq57_e1397 * s.db[852][20]));
        let eq57_e1399_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[852]) + (eq57_e1397 * s.db[852][21]));
        let eq57_e1399_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[852]) + (eq57_e1397 * s.db[852][22]));
        let eq57_e1399_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[852]) + (eq57_e1397 * s.db[852][23]));
        let eq57_e1399_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[852]) + (eq57_e1397 * s.db[852][24]));
        let eq57_e1400_q: f64 = eq57_e1399;
        let eq57_reactive_node_derivatives: [f64; 21] = [eq57_e1399_d_n0, eq57_e1399_d_n1, eq57_e1399_d_n2, eq57_e1399_d_n3, eq57_e1399_d_n4, eq57_e1399_d_n5, eq57_e1399_d_n6, eq57_e1399_d_n7, eq57_e1399_d_n8, eq57_e1399_d_n9, eq57_e1399_d_n10, eq57_e1399_d_n11, eq57_e1399_d_n12, eq57_e1399_d_n13, eq57_e1399_d_n14, eq57_e1399_d_n15, eq57_e1399_d_n16, eq57_e1399_d_n17, eq57_e1399_d_n18, eq57_e1399_d_n19, eq57_e1399_d_n20];
        let eq57_reactive_branch_derivatives: [f64; 25] = [eq57_e1399_d_b0, eq57_e1399_d_b1, eq57_e1399_d_b2, eq57_e1399_d_b3, eq57_e1399_d_b4, eq57_e1399_d_b5, eq57_e1399_d_b6, eq57_e1399_d_b7, eq57_e1399_d_b8, eq57_e1399_d_b9, eq57_e1399_d_b10, eq57_e1399_d_b11, eq57_e1399_d_b12, eq57_e1399_d_b13, eq57_e1399_d_b14, eq57_e1399_d_b15, eq57_e1399_d_b16, eq57_e1399_d_b17, eq57_e1399_d_b18, eq57_e1399_d_b19, eq57_e1399_d_b20, eq57_e1399_d_b21, eq57_e1399_d_b22, eq57_e1399_d_b23, eq57_e1399_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq57_reactive_node_derivatives,
            branches,
            &eq57_reactive_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (s.v[0] * s.v[19]);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * s.v[853]);
        let eq58_e1407_d_n0: f64 = ((eq56_e1389_d_n0 * s.v[853]) + (eq58_e1405 * s.dn[853][0]));
        let eq58_e1407_d_n1: f64 = ((eq56_e1389_d_n1 * s.v[853]) + (eq58_e1405 * s.dn[853][1]));
        let eq58_e1407_d_n2: f64 = ((eq56_e1389_d_n2 * s.v[853]) + (eq58_e1405 * s.dn[853][2]));
        let eq58_e1407_d_n3: f64 = ((eq56_e1389_d_n3 * s.v[853]) + (eq58_e1405 * s.dn[853][3]));
        let eq58_e1407_d_n4: f64 = ((eq56_e1389_d_n4 * s.v[853]) + (eq58_e1405 * s.dn[853][4]));
        let eq58_e1407_d_n5: f64 = ((eq56_e1389_d_n5 * s.v[853]) + (eq58_e1405 * s.dn[853][5]));
        let eq58_e1407_d_n6: f64 = ((eq56_e1389_d_n6 * s.v[853]) + (eq58_e1405 * s.dn[853][6]));
        let eq58_e1407_d_n7: f64 = ((eq56_e1389_d_n7 * s.v[853]) + (eq58_e1405 * s.dn[853][7]));
        let eq58_e1407_d_n8: f64 = ((eq56_e1389_d_n8 * s.v[853]) + (eq58_e1405 * s.dn[853][8]));
        let eq58_e1407_d_n9: f64 = ((eq56_e1389_d_n9 * s.v[853]) + (eq58_e1405 * s.dn[853][9]));
        let eq58_e1407_d_n10: f64 = ((eq56_e1389_d_n10 * s.v[853]) + (eq58_e1405 * s.dn[853][10]));
        let eq58_e1407_d_n11: f64 = ((eq56_e1389_d_n11 * s.v[853]) + (eq58_e1405 * s.dn[853][11]));
        let eq58_e1407_d_n12: f64 = ((eq56_e1389_d_n12 * s.v[853]) + (eq58_e1405 * s.dn[853][12]));
        let eq58_e1407_d_n13: f64 = ((eq56_e1389_d_n13 * s.v[853]) + (eq58_e1405 * s.dn[853][13]));
        let eq58_e1407_d_n14: f64 = ((eq56_e1389_d_n14 * s.v[853]) + (eq58_e1405 * s.dn[853][14]));
        let eq58_e1407_d_n15: f64 = ((eq56_e1389_d_n15 * s.v[853]) + (eq58_e1405 * s.dn[853][15]));
        let eq58_e1407_d_n16: f64 = ((eq56_e1389_d_n16 * s.v[853]) + (eq58_e1405 * s.dn[853][16]));
        let eq58_e1407_d_n17: f64 = ((eq56_e1389_d_n17 * s.v[853]) + (eq58_e1405 * s.dn[853][17]));
        let eq58_e1407_d_n18: f64 = ((eq56_e1389_d_n18 * s.v[853]) + (eq58_e1405 * s.dn[853][18]));
        let eq58_e1407_d_n19: f64 = ((eq56_e1389_d_n19 * s.v[853]) + (eq58_e1405 * s.dn[853][19]));
        let eq58_e1407_d_n20: f64 = ((eq56_e1389_d_n20 * s.v[853]) + (eq58_e1405 * s.dn[853][20]));
        let eq58_e1407_d_b0: f64 = ((eq56_e1389_d_b0 * s.v[853]) + (eq58_e1405 * s.db[853][0]));
        let eq58_e1407_d_b1: f64 = ((eq56_e1389_d_b1 * s.v[853]) + (eq58_e1405 * s.db[853][1]));
        let eq58_e1407_d_b2: f64 = ((eq56_e1389_d_b2 * s.v[853]) + (eq58_e1405 * s.db[853][2]));
        let eq58_e1407_d_b3: f64 = ((eq56_e1389_d_b3 * s.v[853]) + (eq58_e1405 * s.db[853][3]));
        let eq58_e1407_d_b4: f64 = ((eq56_e1389_d_b4 * s.v[853]) + (eq58_e1405 * s.db[853][4]));
        let eq58_e1407_d_b5: f64 = ((eq56_e1389_d_b5 * s.v[853]) + (eq58_e1405 * s.db[853][5]));
        let eq58_e1407_d_b6: f64 = ((eq56_e1389_d_b6 * s.v[853]) + (eq58_e1405 * s.db[853][6]));
        let eq58_e1407_d_b7: f64 = ((eq56_e1389_d_b7 * s.v[853]) + (eq58_e1405 * s.db[853][7]));
        let eq58_e1407_d_b8: f64 = ((eq56_e1389_d_b8 * s.v[853]) + (eq58_e1405 * s.db[853][8]));
        let eq58_e1407_d_b9: f64 = ((eq56_e1389_d_b9 * s.v[853]) + (eq58_e1405 * s.db[853][9]));
        let eq58_e1407_d_b10: f64 = ((eq56_e1389_d_b10 * s.v[853]) + (eq58_e1405 * s.db[853][10]));
        let eq58_e1407_d_b11: f64 = ((eq56_e1389_d_b11 * s.v[853]) + (eq58_e1405 * s.db[853][11]));
        let eq58_e1407_d_b12: f64 = ((eq56_e1389_d_b12 * s.v[853]) + (eq58_e1405 * s.db[853][12]));
        let eq58_e1407_d_b13: f64 = ((eq56_e1389_d_b13 * s.v[853]) + (eq58_e1405 * s.db[853][13]));
        let eq58_e1407_d_b14: f64 = ((eq56_e1389_d_b14 * s.v[853]) + (eq58_e1405 * s.db[853][14]));
        let eq58_e1407_d_b15: f64 = ((eq56_e1389_d_b15 * s.v[853]) + (eq58_e1405 * s.db[853][15]));
        let eq58_e1407_d_b16: f64 = ((eq56_e1389_d_b16 * s.v[853]) + (eq58_e1405 * s.db[853][16]));
        let eq58_e1407_d_b17: f64 = ((eq56_e1389_d_b17 * s.v[853]) + (eq58_e1405 * s.db[853][17]));
        let eq58_e1407_d_b18: f64 = ((eq56_e1389_d_b18 * s.v[853]) + (eq58_e1405 * s.db[853][18]));
        let eq58_e1407_d_b19: f64 = ((eq56_e1389_d_b19 * s.v[853]) + (eq58_e1405 * s.db[853][19]));
        let eq58_e1407_d_b20: f64 = ((eq56_e1389_d_b20 * s.v[853]) + (eq58_e1405 * s.db[853][20]));
        let eq58_e1407_d_b21: f64 = ((eq56_e1389_d_b21 * s.v[853]) + (eq58_e1405 * s.db[853][21]));
        let eq58_e1407_d_b22: f64 = ((eq56_e1389_d_b22 * s.v[853]) + (eq58_e1405 * s.db[853][22]));
        let eq58_e1407_d_b23: f64 = ((eq56_e1389_d_b23 * s.v[853]) + (eq58_e1405 * s.db[853][23]));
        let eq58_e1407_d_b24: f64 = ((eq56_e1389_d_b24 * s.v[853]) + (eq58_e1405 * s.db[853][24]));
        let eq58_e1408_q: f64 = eq58_e1407;
        let eq58_reactive_node_derivatives: [f64; 21] = [eq58_e1407_d_n0, eq58_e1407_d_n1, eq58_e1407_d_n2, eq58_e1407_d_n3, eq58_e1407_d_n4, eq58_e1407_d_n5, eq58_e1407_d_n6, eq58_e1407_d_n7, eq58_e1407_d_n8, eq58_e1407_d_n9, eq58_e1407_d_n10, eq58_e1407_d_n11, eq58_e1407_d_n12, eq58_e1407_d_n13, eq58_e1407_d_n14, eq58_e1407_d_n15, eq58_e1407_d_n16, eq58_e1407_d_n17, eq58_e1407_d_n18, eq58_e1407_d_n19, eq58_e1407_d_n20];
        let eq58_reactive_branch_derivatives: [f64; 25] = [eq58_e1407_d_b0, eq58_e1407_d_b1, eq58_e1407_d_b2, eq58_e1407_d_b3, eq58_e1407_d_b4, eq58_e1407_d_b5, eq58_e1407_d_b6, eq58_e1407_d_b7, eq58_e1407_d_b8, eq58_e1407_d_b9, eq58_e1407_d_b10, eq58_e1407_d_b11, eq58_e1407_d_b12, eq58_e1407_d_b13, eq58_e1407_d_b14, eq58_e1407_d_b15, eq58_e1407_d_b16, eq58_e1407_d_b17, eq58_e1407_d_b18, eq58_e1407_d_b19, eq58_e1407_d_b20, eq58_e1407_d_b21, eq58_e1407_d_b22, eq58_e1407_d_b23, eq58_e1407_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
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
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq59_e1411: f64 = (s.v[0] * s.v[19]);
        let eq59_e1413: f64 = (eq59_e1411 * p.p33);
        let eq59_e1413_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq59_e1413_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq59_e1413_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq59_e1413_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq59_e1413_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq59_e1413_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq59_e1413_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq59_e1413_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq59_e1413_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq59_e1413_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq59_e1413_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq59_e1413_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq59_e1413_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq59_e1413_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq59_e1413_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq59_e1413_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq59_e1413_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq59_e1413_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq59_e1413_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq59_e1413_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq59_e1413_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq59_e1413_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq59_e1413_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq59_e1413_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq59_e1413_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq59_e1413_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq59_e1413_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq59_e1413_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq59_e1413_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq59_e1413_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq59_e1413_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq59_e1413_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq59_e1413_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq59_e1413_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq59_e1413_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq59_e1413_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq59_e1413_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq59_e1413_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq59_e1413_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq59_e1413_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq59_e1413_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq59_e1413_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq59_e1413_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq59_e1413_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq59_e1413_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq59_e1413_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq59_e1415: f64 = (eq59_e1413 * s.v[854]);
        let eq59_e1415_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[854]) + (eq59_e1413 * s.dn[854][0]));
        let eq59_e1415_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[854]) + (eq59_e1413 * s.dn[854][1]));
        let eq59_e1415_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[854]) + (eq59_e1413 * s.dn[854][2]));
        let eq59_e1415_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[854]) + (eq59_e1413 * s.dn[854][3]));
        let eq59_e1415_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[854]) + (eq59_e1413 * s.dn[854][4]));
        let eq59_e1415_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[854]) + (eq59_e1413 * s.dn[854][5]));
        let eq59_e1415_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[854]) + (eq59_e1413 * s.dn[854][6]));
        let eq59_e1415_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[854]) + (eq59_e1413 * s.dn[854][7]));
        let eq59_e1415_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[854]) + (eq59_e1413 * s.dn[854][8]));
        let eq59_e1415_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[854]) + (eq59_e1413 * s.dn[854][9]));
        let eq59_e1415_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[854]) + (eq59_e1413 * s.dn[854][10]));
        let eq59_e1415_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[854]) + (eq59_e1413 * s.dn[854][11]));
        let eq59_e1415_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[854]) + (eq59_e1413 * s.dn[854][12]));
        let eq59_e1415_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[854]) + (eq59_e1413 * s.dn[854][13]));
        let eq59_e1415_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[854]) + (eq59_e1413 * s.dn[854][14]));
        let eq59_e1415_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[854]) + (eq59_e1413 * s.dn[854][15]));
        let eq59_e1415_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[854]) + (eq59_e1413 * s.dn[854][16]));
        let eq59_e1415_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[854]) + (eq59_e1413 * s.dn[854][17]));
        let eq59_e1415_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[854]) + (eq59_e1413 * s.dn[854][18]));
        let eq59_e1415_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[854]) + (eq59_e1413 * s.dn[854][19]));
        let eq59_e1415_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[854]) + (eq59_e1413 * s.dn[854][20]));
        let eq59_e1415_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[854]) + (eq59_e1413 * s.db[854][0]));
        let eq59_e1415_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[854]) + (eq59_e1413 * s.db[854][1]));
        let eq59_e1415_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[854]) + (eq59_e1413 * s.db[854][2]));
        let eq59_e1415_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[854]) + (eq59_e1413 * s.db[854][3]));
        let eq59_e1415_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[854]) + (eq59_e1413 * s.db[854][4]));
        let eq59_e1415_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[854]) + (eq59_e1413 * s.db[854][5]));
        let eq59_e1415_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[854]) + (eq59_e1413 * s.db[854][6]));
        let eq59_e1415_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[854]) + (eq59_e1413 * s.db[854][7]));
        let eq59_e1415_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[854]) + (eq59_e1413 * s.db[854][8]));
        let eq59_e1415_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[854]) + (eq59_e1413 * s.db[854][9]));
        let eq59_e1415_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[854]) + (eq59_e1413 * s.db[854][10]));
        let eq59_e1415_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[854]) + (eq59_e1413 * s.db[854][11]));
        let eq59_e1415_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[854]) + (eq59_e1413 * s.db[854][12]));
        let eq59_e1415_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[854]) + (eq59_e1413 * s.db[854][13]));
        let eq59_e1415_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[854]) + (eq59_e1413 * s.db[854][14]));
        let eq59_e1415_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[854]) + (eq59_e1413 * s.db[854][15]));
        let eq59_e1415_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[854]) + (eq59_e1413 * s.db[854][16]));
        let eq59_e1415_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[854]) + (eq59_e1413 * s.db[854][17]));
        let eq59_e1415_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[854]) + (eq59_e1413 * s.db[854][18]));
        let eq59_e1415_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[854]) + (eq59_e1413 * s.db[854][19]));
        let eq59_e1415_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[854]) + (eq59_e1413 * s.db[854][20]));
        let eq59_e1415_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[854]) + (eq59_e1413 * s.db[854][21]));
        let eq59_e1415_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[854]) + (eq59_e1413 * s.db[854][22]));
        let eq59_e1415_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[854]) + (eq59_e1413 * s.db[854][23]));
        let eq59_e1415_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[854]) + (eq59_e1413 * s.db[854][24]));
        let eq59_e1416_q: f64 = eq59_e1415;
        let eq59_reactive_node_derivatives: [f64; 21] = [eq59_e1415_d_n0, eq59_e1415_d_n1, eq59_e1415_d_n2, eq59_e1415_d_n3, eq59_e1415_d_n4, eq59_e1415_d_n5, eq59_e1415_d_n6, eq59_e1415_d_n7, eq59_e1415_d_n8, eq59_e1415_d_n9, eq59_e1415_d_n10, eq59_e1415_d_n11, eq59_e1415_d_n12, eq59_e1415_d_n13, eq59_e1415_d_n14, eq59_e1415_d_n15, eq59_e1415_d_n16, eq59_e1415_d_n17, eq59_e1415_d_n18, eq59_e1415_d_n19, eq59_e1415_d_n20];
        let eq59_reactive_branch_derivatives: [f64; 25] = [eq59_e1415_d_b0, eq59_e1415_d_b1, eq59_e1415_d_b2, eq59_e1415_d_b3, eq59_e1415_d_b4, eq59_e1415_d_b5, eq59_e1415_d_b6, eq59_e1415_d_b7, eq59_e1415_d_b8, eq59_e1415_d_b9, eq59_e1415_d_b10, eq59_e1415_d_b11, eq59_e1415_d_b12, eq59_e1415_d_b13, eq59_e1415_d_b14, eq59_e1415_d_b15, eq59_e1415_d_b16, eq59_e1415_d_b17, eq59_e1415_d_b18, eq59_e1415_d_b19, eq59_e1415_d_b20, eq59_e1415_d_b21, eq59_e1415_d_b22, eq59_e1415_d_b23, eq59_e1415_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq59_reactive_node_derivatives,
            branches,
            &eq59_reactive_branch_derivatives,
            multiplicity,
        );
        let eq60_e1419: f64 = (s.v[0] * s.v[19]);
        let eq60_e1421: f64 = (eq60_e1419 * p.p33);
        let eq60_e1423: f64 = (eq60_e1421 * s.v[855]);
        let eq60_e1423_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[855]) + (eq60_e1421 * s.dn[855][0]));
        let eq60_e1423_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[855]) + (eq60_e1421 * s.dn[855][1]));
        let eq60_e1423_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[855]) + (eq60_e1421 * s.dn[855][2]));
        let eq60_e1423_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[855]) + (eq60_e1421 * s.dn[855][3]));
        let eq60_e1423_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[855]) + (eq60_e1421 * s.dn[855][4]));
        let eq60_e1423_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[855]) + (eq60_e1421 * s.dn[855][5]));
        let eq60_e1423_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[855]) + (eq60_e1421 * s.dn[855][6]));
        let eq60_e1423_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[855]) + (eq60_e1421 * s.dn[855][7]));
        let eq60_e1423_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[855]) + (eq60_e1421 * s.dn[855][8]));
        let eq60_e1423_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[855]) + (eq60_e1421 * s.dn[855][9]));
        let eq60_e1423_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[855]) + (eq60_e1421 * s.dn[855][10]));
        let eq60_e1423_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[855]) + (eq60_e1421 * s.dn[855][11]));
        let eq60_e1423_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[855]) + (eq60_e1421 * s.dn[855][12]));
        let eq60_e1423_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[855]) + (eq60_e1421 * s.dn[855][13]));
        let eq60_e1423_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[855]) + (eq60_e1421 * s.dn[855][14]));
        let eq60_e1423_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[855]) + (eq60_e1421 * s.dn[855][15]));
        let eq60_e1423_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[855]) + (eq60_e1421 * s.dn[855][16]));
        let eq60_e1423_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[855]) + (eq60_e1421 * s.dn[855][17]));
        let eq60_e1423_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[855]) + (eq60_e1421 * s.dn[855][18]));
        let eq60_e1423_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[855]) + (eq60_e1421 * s.dn[855][19]));
        let eq60_e1423_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[855]) + (eq60_e1421 * s.dn[855][20]));
        let eq60_e1423_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[855]) + (eq60_e1421 * s.db[855][0]));
        let eq60_e1423_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[855]) + (eq60_e1421 * s.db[855][1]));
        let eq60_e1423_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[855]) + (eq60_e1421 * s.db[855][2]));
        let eq60_e1423_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[855]) + (eq60_e1421 * s.db[855][3]));
        let eq60_e1423_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[855]) + (eq60_e1421 * s.db[855][4]));
        let eq60_e1423_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[855]) + (eq60_e1421 * s.db[855][5]));
        let eq60_e1423_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[855]) + (eq60_e1421 * s.db[855][6]));
        let eq60_e1423_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[855]) + (eq60_e1421 * s.db[855][7]));
        let eq60_e1423_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[855]) + (eq60_e1421 * s.db[855][8]));
        let eq60_e1423_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[855]) + (eq60_e1421 * s.db[855][9]));
        let eq60_e1423_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[855]) + (eq60_e1421 * s.db[855][10]));
        let eq60_e1423_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[855]) + (eq60_e1421 * s.db[855][11]));
        let eq60_e1423_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[855]) + (eq60_e1421 * s.db[855][12]));
        let eq60_e1423_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[855]) + (eq60_e1421 * s.db[855][13]));
        let eq60_e1423_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[855]) + (eq60_e1421 * s.db[855][14]));
        let eq60_e1423_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[855]) + (eq60_e1421 * s.db[855][15]));
        let eq60_e1423_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[855]) + (eq60_e1421 * s.db[855][16]));
        let eq60_e1423_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[855]) + (eq60_e1421 * s.db[855][17]));
        let eq60_e1423_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[855]) + (eq60_e1421 * s.db[855][18]));
        let eq60_e1423_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[855]) + (eq60_e1421 * s.db[855][19]));
        let eq60_e1423_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[855]) + (eq60_e1421 * s.db[855][20]));
        let eq60_e1423_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[855]) + (eq60_e1421 * s.db[855][21]));
        let eq60_e1423_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[855]) + (eq60_e1421 * s.db[855][22]));
        let eq60_e1423_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[855]) + (eq60_e1421 * s.db[855][23]));
        let eq60_e1423_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[855]) + (eq60_e1421 * s.db[855][24]));
        let eq60_e1424_q: f64 = eq60_e1423;
        let eq60_reactive_node_derivatives: [f64; 21] = [eq60_e1423_d_n0, eq60_e1423_d_n1, eq60_e1423_d_n2, eq60_e1423_d_n3, eq60_e1423_d_n4, eq60_e1423_d_n5, eq60_e1423_d_n6, eq60_e1423_d_n7, eq60_e1423_d_n8, eq60_e1423_d_n9, eq60_e1423_d_n10, eq60_e1423_d_n11, eq60_e1423_d_n12, eq60_e1423_d_n13, eq60_e1423_d_n14, eq60_e1423_d_n15, eq60_e1423_d_n16, eq60_e1423_d_n17, eq60_e1423_d_n18, eq60_e1423_d_n19, eq60_e1423_d_n20];
        let eq60_reactive_branch_derivatives: [f64; 25] = [eq60_e1423_d_b0, eq60_e1423_d_b1, eq60_e1423_d_b2, eq60_e1423_d_b3, eq60_e1423_d_b4, eq60_e1423_d_b5, eq60_e1423_d_b6, eq60_e1423_d_b7, eq60_e1423_d_b8, eq60_e1423_d_b9, eq60_e1423_d_b10, eq60_e1423_d_b11, eq60_e1423_d_b12, eq60_e1423_d_b13, eq60_e1423_d_b14, eq60_e1423_d_b15, eq60_e1423_d_b16, eq60_e1423_d_b17, eq60_e1423_d_b18, eq60_e1423_d_b19, eq60_e1423_d_b20, eq60_e1423_d_b21, eq60_e1423_d_b22, eq60_e1423_d_b23, eq60_e1423_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq60_reactive_node_derivatives,
            branches,
            &eq60_reactive_branch_derivatives,
            multiplicity,
        );
        let eq61_e1427: f64 = (s.v[0] * s.v[19]);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * s.v[856]);
        let eq61_e1431_d_n0: f64 = ((eq59_e1413_d_n0 * s.v[856]) + (eq61_e1429 * s.dn[856][0]));
        let eq61_e1431_d_n1: f64 = ((eq59_e1413_d_n1 * s.v[856]) + (eq61_e1429 * s.dn[856][1]));
        let eq61_e1431_d_n2: f64 = ((eq59_e1413_d_n2 * s.v[856]) + (eq61_e1429 * s.dn[856][2]));
        let eq61_e1431_d_n3: f64 = ((eq59_e1413_d_n3 * s.v[856]) + (eq61_e1429 * s.dn[856][3]));
        let eq61_e1431_d_n4: f64 = ((eq59_e1413_d_n4 * s.v[856]) + (eq61_e1429 * s.dn[856][4]));
        let eq61_e1431_d_n5: f64 = ((eq59_e1413_d_n5 * s.v[856]) + (eq61_e1429 * s.dn[856][5]));
        let eq61_e1431_d_n6: f64 = ((eq59_e1413_d_n6 * s.v[856]) + (eq61_e1429 * s.dn[856][6]));
        let eq61_e1431_d_n7: f64 = ((eq59_e1413_d_n7 * s.v[856]) + (eq61_e1429 * s.dn[856][7]));
        let eq61_e1431_d_n8: f64 = ((eq59_e1413_d_n8 * s.v[856]) + (eq61_e1429 * s.dn[856][8]));
        let eq61_e1431_d_n9: f64 = ((eq59_e1413_d_n9 * s.v[856]) + (eq61_e1429 * s.dn[856][9]));
        let eq61_e1431_d_n10: f64 = ((eq59_e1413_d_n10 * s.v[856]) + (eq61_e1429 * s.dn[856][10]));
        let eq61_e1431_d_n11: f64 = ((eq59_e1413_d_n11 * s.v[856]) + (eq61_e1429 * s.dn[856][11]));
        let eq61_e1431_d_n12: f64 = ((eq59_e1413_d_n12 * s.v[856]) + (eq61_e1429 * s.dn[856][12]));
        let eq61_e1431_d_n13: f64 = ((eq59_e1413_d_n13 * s.v[856]) + (eq61_e1429 * s.dn[856][13]));
        let eq61_e1431_d_n14: f64 = ((eq59_e1413_d_n14 * s.v[856]) + (eq61_e1429 * s.dn[856][14]));
        let eq61_e1431_d_n15: f64 = ((eq59_e1413_d_n15 * s.v[856]) + (eq61_e1429 * s.dn[856][15]));
        let eq61_e1431_d_n16: f64 = ((eq59_e1413_d_n16 * s.v[856]) + (eq61_e1429 * s.dn[856][16]));
        let eq61_e1431_d_n17: f64 = ((eq59_e1413_d_n17 * s.v[856]) + (eq61_e1429 * s.dn[856][17]));
        let eq61_e1431_d_n18: f64 = ((eq59_e1413_d_n18 * s.v[856]) + (eq61_e1429 * s.dn[856][18]));
        let eq61_e1431_d_n19: f64 = ((eq59_e1413_d_n19 * s.v[856]) + (eq61_e1429 * s.dn[856][19]));
        let eq61_e1431_d_n20: f64 = ((eq59_e1413_d_n20 * s.v[856]) + (eq61_e1429 * s.dn[856][20]));
        let eq61_e1431_d_b0: f64 = ((eq59_e1413_d_b0 * s.v[856]) + (eq61_e1429 * s.db[856][0]));
        let eq61_e1431_d_b1: f64 = ((eq59_e1413_d_b1 * s.v[856]) + (eq61_e1429 * s.db[856][1]));
        let eq61_e1431_d_b2: f64 = ((eq59_e1413_d_b2 * s.v[856]) + (eq61_e1429 * s.db[856][2]));
        let eq61_e1431_d_b3: f64 = ((eq59_e1413_d_b3 * s.v[856]) + (eq61_e1429 * s.db[856][3]));
        let eq61_e1431_d_b4: f64 = ((eq59_e1413_d_b4 * s.v[856]) + (eq61_e1429 * s.db[856][4]));
        let eq61_e1431_d_b5: f64 = ((eq59_e1413_d_b5 * s.v[856]) + (eq61_e1429 * s.db[856][5]));
        let eq61_e1431_d_b6: f64 = ((eq59_e1413_d_b6 * s.v[856]) + (eq61_e1429 * s.db[856][6]));
        let eq61_e1431_d_b7: f64 = ((eq59_e1413_d_b7 * s.v[856]) + (eq61_e1429 * s.db[856][7]));
        let eq61_e1431_d_b8: f64 = ((eq59_e1413_d_b8 * s.v[856]) + (eq61_e1429 * s.db[856][8]));
        let eq61_e1431_d_b9: f64 = ((eq59_e1413_d_b9 * s.v[856]) + (eq61_e1429 * s.db[856][9]));
        let eq61_e1431_d_b10: f64 = ((eq59_e1413_d_b10 * s.v[856]) + (eq61_e1429 * s.db[856][10]));
        let eq61_e1431_d_b11: f64 = ((eq59_e1413_d_b11 * s.v[856]) + (eq61_e1429 * s.db[856][11]));
        let eq61_e1431_d_b12: f64 = ((eq59_e1413_d_b12 * s.v[856]) + (eq61_e1429 * s.db[856][12]));
        let eq61_e1431_d_b13: f64 = ((eq59_e1413_d_b13 * s.v[856]) + (eq61_e1429 * s.db[856][13]));
        let eq61_e1431_d_b14: f64 = ((eq59_e1413_d_b14 * s.v[856]) + (eq61_e1429 * s.db[856][14]));
        let eq61_e1431_d_b15: f64 = ((eq59_e1413_d_b15 * s.v[856]) + (eq61_e1429 * s.db[856][15]));
        let eq61_e1431_d_b16: f64 = ((eq59_e1413_d_b16 * s.v[856]) + (eq61_e1429 * s.db[856][16]));
        let eq61_e1431_d_b17: f64 = ((eq59_e1413_d_b17 * s.v[856]) + (eq61_e1429 * s.db[856][17]));
        let eq61_e1431_d_b18: f64 = ((eq59_e1413_d_b18 * s.v[856]) + (eq61_e1429 * s.db[856][18]));
        let eq61_e1431_d_b19: f64 = ((eq59_e1413_d_b19 * s.v[856]) + (eq61_e1429 * s.db[856][19]));
        let eq61_e1431_d_b20: f64 = ((eq59_e1413_d_b20 * s.v[856]) + (eq61_e1429 * s.db[856][20]));
        let eq61_e1431_d_b21: f64 = ((eq59_e1413_d_b21 * s.v[856]) + (eq61_e1429 * s.db[856][21]));
        let eq61_e1431_d_b22: f64 = ((eq59_e1413_d_b22 * s.v[856]) + (eq61_e1429 * s.db[856][22]));
        let eq61_e1431_d_b23: f64 = ((eq59_e1413_d_b23 * s.v[856]) + (eq61_e1429 * s.db[856][23]));
        let eq61_e1431_d_b24: f64 = ((eq59_e1413_d_b24 * s.v[856]) + (eq61_e1429 * s.db[856][24]));
        let eq61_e1432_q: f64 = eq61_e1431;
        let eq61_reactive_node_derivatives: [f64; 21] = [eq61_e1431_d_n0, eq61_e1431_d_n1, eq61_e1431_d_n2, eq61_e1431_d_n3, eq61_e1431_d_n4, eq61_e1431_d_n5, eq61_e1431_d_n6, eq61_e1431_d_n7, eq61_e1431_d_n8, eq61_e1431_d_n9, eq61_e1431_d_n10, eq61_e1431_d_n11, eq61_e1431_d_n12, eq61_e1431_d_n13, eq61_e1431_d_n14, eq61_e1431_d_n15, eq61_e1431_d_n16, eq61_e1431_d_n17, eq61_e1431_d_n18, eq61_e1431_d_n19, eq61_e1431_d_n20];
        let eq61_reactive_branch_derivatives: [f64; 25] = [eq61_e1431_d_b0, eq61_e1431_d_b1, eq61_e1431_d_b2, eq61_e1431_d_b3, eq61_e1431_d_b4, eq61_e1431_d_b5, eq61_e1431_d_b6, eq61_e1431_d_b7, eq61_e1431_d_b8, eq61_e1431_d_b9, eq61_e1431_d_b10, eq61_e1431_d_b11, eq61_e1431_d_b12, eq61_e1431_d_b13, eq61_e1431_d_b14, eq61_e1431_d_b15, eq61_e1431_d_b16, eq61_e1431_d_b17, eq61_e1431_d_b18, eq61_e1431_d_b19, eq61_e1431_d_b20, eq61_e1431_d_b21, eq61_e1431_d_b22, eq61_e1431_d_b23, eq61_e1431_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let __rspice_deriv_cse_13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let __rspice_deriv_cse_14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let __rspice_deriv_cse_15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let __rspice_deriv_cse_16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let __rspice_deriv_cse_17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let __rspice_deriv_cse_18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let __rspice_deriv_cse_19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let __rspice_deriv_cse_20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let __rspice_deriv_cse_21: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let __rspice_deriv_cse_22: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let __rspice_deriv_cse_23: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let __rspice_deriv_cse_24: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let __rspice_deriv_cse_25: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let __rspice_deriv_cse_26: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let __rspice_deriv_cse_27: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let __rspice_deriv_cse_28: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let __rspice_deriv_cse_29: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let __rspice_deriv_cse_30: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let __rspice_deriv_cse_31: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let __rspice_deriv_cse_32: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let __rspice_deriv_cse_33: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let __rspice_deriv_cse_34: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let __rspice_deriv_cse_35: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let __rspice_deriv_cse_36: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let __rspice_deriv_cse_37: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let __rspice_deriv_cse_38: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let __rspice_deriv_cse_39: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let __rspice_deriv_cse_40: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let __rspice_deriv_cse_41: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let __rspice_deriv_cse_42: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let __rspice_deriv_cse_43: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let __rspice_deriv_cse_44: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let __rspice_deriv_cse_45: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq62_e1435: f64 = (s.v[0] * s.v[19]);
        let eq62_e1437: f64 = (eq62_e1435 * p.p33);
        let eq62_e1437_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq62_e1437_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq62_e1437_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq62_e1437_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq62_e1437_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq62_e1437_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq62_e1437_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq62_e1437_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq62_e1437_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq62_e1437_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq62_e1437_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq62_e1437_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq62_e1437_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq62_e1437_d_n13: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq62_e1437_d_n14: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq62_e1437_d_n15: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq62_e1437_d_n16: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq62_e1437_d_n17: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq62_e1437_d_n18: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq62_e1437_d_n19: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq62_e1437_d_n20: f64 = (__rspice_deriv_cse_20 * p.p33);
        let eq62_e1437_d_b0: f64 = (__rspice_deriv_cse_21 * p.p33);
        let eq62_e1437_d_b1: f64 = (__rspice_deriv_cse_22 * p.p33);
        let eq62_e1437_d_b2: f64 = (__rspice_deriv_cse_23 * p.p33);
        let eq62_e1437_d_b3: f64 = (__rspice_deriv_cse_24 * p.p33);
        let eq62_e1437_d_b4: f64 = (__rspice_deriv_cse_25 * p.p33);
        let eq62_e1437_d_b5: f64 = (__rspice_deriv_cse_26 * p.p33);
        let eq62_e1437_d_b6: f64 = (__rspice_deriv_cse_27 * p.p33);
        let eq62_e1437_d_b7: f64 = (__rspice_deriv_cse_28 * p.p33);
        let eq62_e1437_d_b8: f64 = (__rspice_deriv_cse_29 * p.p33);
        let eq62_e1437_d_b9: f64 = (__rspice_deriv_cse_30 * p.p33);
        let eq62_e1437_d_b10: f64 = (__rspice_deriv_cse_31 * p.p33);
        let eq62_e1437_d_b11: f64 = (__rspice_deriv_cse_32 * p.p33);
        let eq62_e1437_d_b12: f64 = (__rspice_deriv_cse_33 * p.p33);
        let eq62_e1437_d_b13: f64 = (__rspice_deriv_cse_34 * p.p33);
        let eq62_e1437_d_b14: f64 = (__rspice_deriv_cse_35 * p.p33);
        let eq62_e1437_d_b15: f64 = (__rspice_deriv_cse_36 * p.p33);
        let eq62_e1437_d_b16: f64 = (__rspice_deriv_cse_37 * p.p33);
        let eq62_e1437_d_b17: f64 = (__rspice_deriv_cse_38 * p.p33);
        let eq62_e1437_d_b18: f64 = (__rspice_deriv_cse_39 * p.p33);
        let eq62_e1437_d_b19: f64 = (__rspice_deriv_cse_40 * p.p33);
        let eq62_e1437_d_b20: f64 = (__rspice_deriv_cse_41 * p.p33);
        let eq62_e1437_d_b21: f64 = (__rspice_deriv_cse_42 * p.p33);
        let eq62_e1437_d_b22: f64 = (__rspice_deriv_cse_43 * p.p33);
        let eq62_e1437_d_b23: f64 = (__rspice_deriv_cse_44 * p.p33);
        let eq62_e1437_d_b24: f64 = (__rspice_deriv_cse_45 * p.p33);
        let eq62_e1439: f64 = (eq62_e1437 * s.v[857]);
        let eq62_e1439_d_n0: f64 = ((eq62_e1437_d_n0 * s.v[857]) + (eq62_e1437 * s.dn[857][0]));
        let eq62_e1439_d_n1: f64 = ((eq62_e1437_d_n1 * s.v[857]) + (eq62_e1437 * s.dn[857][1]));
        let eq62_e1439_d_n2: f64 = ((eq62_e1437_d_n2 * s.v[857]) + (eq62_e1437 * s.dn[857][2]));
        let eq62_e1439_d_n3: f64 = ((eq62_e1437_d_n3 * s.v[857]) + (eq62_e1437 * s.dn[857][3]));
        let eq62_e1439_d_n4: f64 = ((eq62_e1437_d_n4 * s.v[857]) + (eq62_e1437 * s.dn[857][4]));
        let eq62_e1439_d_n5: f64 = ((eq62_e1437_d_n5 * s.v[857]) + (eq62_e1437 * s.dn[857][5]));
        let eq62_e1439_d_n6: f64 = ((eq62_e1437_d_n6 * s.v[857]) + (eq62_e1437 * s.dn[857][6]));
        let eq62_e1439_d_n7: f64 = ((eq62_e1437_d_n7 * s.v[857]) + (eq62_e1437 * s.dn[857][7]));
        let eq62_e1439_d_n8: f64 = ((eq62_e1437_d_n8 * s.v[857]) + (eq62_e1437 * s.dn[857][8]));
        let eq62_e1439_d_n9: f64 = ((eq62_e1437_d_n9 * s.v[857]) + (eq62_e1437 * s.dn[857][9]));
        let eq62_e1439_d_n10: f64 = ((eq62_e1437_d_n10 * s.v[857]) + (eq62_e1437 * s.dn[857][10]));
        let eq62_e1439_d_n11: f64 = ((eq62_e1437_d_n11 * s.v[857]) + (eq62_e1437 * s.dn[857][11]));
        let eq62_e1439_d_n12: f64 = ((eq62_e1437_d_n12 * s.v[857]) + (eq62_e1437 * s.dn[857][12]));
        let eq62_e1439_d_n13: f64 = ((eq62_e1437_d_n13 * s.v[857]) + (eq62_e1437 * s.dn[857][13]));
        let eq62_e1439_d_n14: f64 = ((eq62_e1437_d_n14 * s.v[857]) + (eq62_e1437 * s.dn[857][14]));
        let eq62_e1439_d_n15: f64 = ((eq62_e1437_d_n15 * s.v[857]) + (eq62_e1437 * s.dn[857][15]));
        let eq62_e1439_d_n16: f64 = ((eq62_e1437_d_n16 * s.v[857]) + (eq62_e1437 * s.dn[857][16]));
        let eq62_e1439_d_n17: f64 = ((eq62_e1437_d_n17 * s.v[857]) + (eq62_e1437 * s.dn[857][17]));
        let eq62_e1439_d_n18: f64 = ((eq62_e1437_d_n18 * s.v[857]) + (eq62_e1437 * s.dn[857][18]));
        let eq62_e1439_d_n19: f64 = ((eq62_e1437_d_n19 * s.v[857]) + (eq62_e1437 * s.dn[857][19]));
        let eq62_e1439_d_n20: f64 = ((eq62_e1437_d_n20 * s.v[857]) + (eq62_e1437 * s.dn[857][20]));
        let eq62_e1439_d_b0: f64 = ((eq62_e1437_d_b0 * s.v[857]) + (eq62_e1437 * s.db[857][0]));
        let eq62_e1439_d_b1: f64 = ((eq62_e1437_d_b1 * s.v[857]) + (eq62_e1437 * s.db[857][1]));
        let eq62_e1439_d_b2: f64 = ((eq62_e1437_d_b2 * s.v[857]) + (eq62_e1437 * s.db[857][2]));
        let eq62_e1439_d_b3: f64 = ((eq62_e1437_d_b3 * s.v[857]) + (eq62_e1437 * s.db[857][3]));
        let eq62_e1439_d_b4: f64 = ((eq62_e1437_d_b4 * s.v[857]) + (eq62_e1437 * s.db[857][4]));
        let eq62_e1439_d_b5: f64 = ((eq62_e1437_d_b5 * s.v[857]) + (eq62_e1437 * s.db[857][5]));
        let eq62_e1439_d_b6: f64 = ((eq62_e1437_d_b6 * s.v[857]) + (eq62_e1437 * s.db[857][6]));
        let eq62_e1439_d_b7: f64 = ((eq62_e1437_d_b7 * s.v[857]) + (eq62_e1437 * s.db[857][7]));
        let eq62_e1439_d_b8: f64 = ((eq62_e1437_d_b8 * s.v[857]) + (eq62_e1437 * s.db[857][8]));
        let eq62_e1439_d_b9: f64 = ((eq62_e1437_d_b9 * s.v[857]) + (eq62_e1437 * s.db[857][9]));
        let eq62_e1439_d_b10: f64 = ((eq62_e1437_d_b10 * s.v[857]) + (eq62_e1437 * s.db[857][10]));
        let eq62_e1439_d_b11: f64 = ((eq62_e1437_d_b11 * s.v[857]) + (eq62_e1437 * s.db[857][11]));
        let eq62_e1439_d_b12: f64 = ((eq62_e1437_d_b12 * s.v[857]) + (eq62_e1437 * s.db[857][12]));
        let eq62_e1439_d_b13: f64 = ((eq62_e1437_d_b13 * s.v[857]) + (eq62_e1437 * s.db[857][13]));
        let eq62_e1439_d_b14: f64 = ((eq62_e1437_d_b14 * s.v[857]) + (eq62_e1437 * s.db[857][14]));
        let eq62_e1439_d_b15: f64 = ((eq62_e1437_d_b15 * s.v[857]) + (eq62_e1437 * s.db[857][15]));
        let eq62_e1439_d_b16: f64 = ((eq62_e1437_d_b16 * s.v[857]) + (eq62_e1437 * s.db[857][16]));
        let eq62_e1439_d_b17: f64 = ((eq62_e1437_d_b17 * s.v[857]) + (eq62_e1437 * s.db[857][17]));
        let eq62_e1439_d_b18: f64 = ((eq62_e1437_d_b18 * s.v[857]) + (eq62_e1437 * s.db[857][18]));
        let eq62_e1439_d_b19: f64 = ((eq62_e1437_d_b19 * s.v[857]) + (eq62_e1437 * s.db[857][19]));
        let eq62_e1439_d_b20: f64 = ((eq62_e1437_d_b20 * s.v[857]) + (eq62_e1437 * s.db[857][20]));
        let eq62_e1439_d_b21: f64 = ((eq62_e1437_d_b21 * s.v[857]) + (eq62_e1437 * s.db[857][21]));
        let eq62_e1439_d_b22: f64 = ((eq62_e1437_d_b22 * s.v[857]) + (eq62_e1437 * s.db[857][22]));
        let eq62_e1439_d_b23: f64 = ((eq62_e1437_d_b23 * s.v[857]) + (eq62_e1437 * s.db[857][23]));
        let eq62_e1439_d_b24: f64 = ((eq62_e1437_d_b24 * s.v[857]) + (eq62_e1437 * s.db[857][24]));
        let eq62_e1440_q: f64 = eq62_e1439;
        let eq62_reactive_node_derivatives: [f64; 21] = [eq62_e1439_d_n0, eq62_e1439_d_n1, eq62_e1439_d_n2, eq62_e1439_d_n3, eq62_e1439_d_n4, eq62_e1439_d_n5, eq62_e1439_d_n6, eq62_e1439_d_n7, eq62_e1439_d_n8, eq62_e1439_d_n9, eq62_e1439_d_n10, eq62_e1439_d_n11, eq62_e1439_d_n12, eq62_e1439_d_n13, eq62_e1439_d_n14, eq62_e1439_d_n15, eq62_e1439_d_n16, eq62_e1439_d_n17, eq62_e1439_d_n18, eq62_e1439_d_n19, eq62_e1439_d_n20];
        let eq62_reactive_branch_derivatives: [f64; 25] = [eq62_e1439_d_b0, eq62_e1439_d_b1, eq62_e1439_d_b2, eq62_e1439_d_b3, eq62_e1439_d_b4, eq62_e1439_d_b5, eq62_e1439_d_b6, eq62_e1439_d_b7, eq62_e1439_d_b8, eq62_e1439_d_b9, eq62_e1439_d_b10, eq62_e1439_d_b11, eq62_e1439_d_b12, eq62_e1439_d_b13, eq62_e1439_d_b14, eq62_e1439_d_b15, eq62_e1439_d_b16, eq62_e1439_d_b17, eq62_e1439_d_b18, eq62_e1439_d_b19, eq62_e1439_d_b20, eq62_e1439_d_b21, eq62_e1439_d_b22, eq62_e1439_d_b23, eq62_e1439_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let eq63_e1443: f64 = (s.v[0] * s.v[19]);
        let eq63_e1445: f64 = (eq63_e1443 * p.p33);
        let eq63_e1447: f64 = (eq63_e1445 * s.v[858]);
        let eq63_e1447_d_n0: f64 = ((eq62_e1437_d_n0 * s.v[858]) + (eq63_e1445 * s.dn[858][0]));
        let eq63_e1447_d_n1: f64 = ((eq62_e1437_d_n1 * s.v[858]) + (eq63_e1445 * s.dn[858][1]));
        let eq63_e1447_d_n2: f64 = ((eq62_e1437_d_n2 * s.v[858]) + (eq63_e1445 * s.dn[858][2]));
        let eq63_e1447_d_n3: f64 = ((eq62_e1437_d_n3 * s.v[858]) + (eq63_e1445 * s.dn[858][3]));
        let eq63_e1447_d_n4: f64 = ((eq62_e1437_d_n4 * s.v[858]) + (eq63_e1445 * s.dn[858][4]));
        let eq63_e1447_d_n5: f64 = ((eq62_e1437_d_n5 * s.v[858]) + (eq63_e1445 * s.dn[858][5]));
        let eq63_e1447_d_n6: f64 = ((eq62_e1437_d_n6 * s.v[858]) + (eq63_e1445 * s.dn[858][6]));
        let eq63_e1447_d_n7: f64 = ((eq62_e1437_d_n7 * s.v[858]) + (eq63_e1445 * s.dn[858][7]));
        let eq63_e1447_d_n8: f64 = ((eq62_e1437_d_n8 * s.v[858]) + (eq63_e1445 * s.dn[858][8]));
        let eq63_e1447_d_n9: f64 = ((eq62_e1437_d_n9 * s.v[858]) + (eq63_e1445 * s.dn[858][9]));
        let eq63_e1447_d_n10: f64 = ((eq62_e1437_d_n10 * s.v[858]) + (eq63_e1445 * s.dn[858][10]));
        let eq63_e1447_d_n11: f64 = ((eq62_e1437_d_n11 * s.v[858]) + (eq63_e1445 * s.dn[858][11]));
        let eq63_e1447_d_n12: f64 = ((eq62_e1437_d_n12 * s.v[858]) + (eq63_e1445 * s.dn[858][12]));
        let eq63_e1447_d_n13: f64 = ((eq62_e1437_d_n13 * s.v[858]) + (eq63_e1445 * s.dn[858][13]));
        let eq63_e1447_d_n14: f64 = ((eq62_e1437_d_n14 * s.v[858]) + (eq63_e1445 * s.dn[858][14]));
        let eq63_e1447_d_n15: f64 = ((eq62_e1437_d_n15 * s.v[858]) + (eq63_e1445 * s.dn[858][15]));
        let eq63_e1447_d_n16: f64 = ((eq62_e1437_d_n16 * s.v[858]) + (eq63_e1445 * s.dn[858][16]));
        let eq63_e1447_d_n17: f64 = ((eq62_e1437_d_n17 * s.v[858]) + (eq63_e1445 * s.dn[858][17]));
        let eq63_e1447_d_n18: f64 = ((eq62_e1437_d_n18 * s.v[858]) + (eq63_e1445 * s.dn[858][18]));
        let eq63_e1447_d_n19: f64 = ((eq62_e1437_d_n19 * s.v[858]) + (eq63_e1445 * s.dn[858][19]));
        let eq63_e1447_d_n20: f64 = ((eq62_e1437_d_n20 * s.v[858]) + (eq63_e1445 * s.dn[858][20]));
        let eq63_e1447_d_b0: f64 = ((eq62_e1437_d_b0 * s.v[858]) + (eq63_e1445 * s.db[858][0]));
        let eq63_e1447_d_b1: f64 = ((eq62_e1437_d_b1 * s.v[858]) + (eq63_e1445 * s.db[858][1]));
        let eq63_e1447_d_b2: f64 = ((eq62_e1437_d_b2 * s.v[858]) + (eq63_e1445 * s.db[858][2]));
        let eq63_e1447_d_b3: f64 = ((eq62_e1437_d_b3 * s.v[858]) + (eq63_e1445 * s.db[858][3]));
        let eq63_e1447_d_b4: f64 = ((eq62_e1437_d_b4 * s.v[858]) + (eq63_e1445 * s.db[858][4]));
        let eq63_e1447_d_b5: f64 = ((eq62_e1437_d_b5 * s.v[858]) + (eq63_e1445 * s.db[858][5]));
        let eq63_e1447_d_b6: f64 = ((eq62_e1437_d_b6 * s.v[858]) + (eq63_e1445 * s.db[858][6]));
        let eq63_e1447_d_b7: f64 = ((eq62_e1437_d_b7 * s.v[858]) + (eq63_e1445 * s.db[858][7]));
        let eq63_e1447_d_b8: f64 = ((eq62_e1437_d_b8 * s.v[858]) + (eq63_e1445 * s.db[858][8]));
        let eq63_e1447_d_b9: f64 = ((eq62_e1437_d_b9 * s.v[858]) + (eq63_e1445 * s.db[858][9]));
        let eq63_e1447_d_b10: f64 = ((eq62_e1437_d_b10 * s.v[858]) + (eq63_e1445 * s.db[858][10]));
        let eq63_e1447_d_b11: f64 = ((eq62_e1437_d_b11 * s.v[858]) + (eq63_e1445 * s.db[858][11]));
        let eq63_e1447_d_b12: f64 = ((eq62_e1437_d_b12 * s.v[858]) + (eq63_e1445 * s.db[858][12]));
        let eq63_e1447_d_b13: f64 = ((eq62_e1437_d_b13 * s.v[858]) + (eq63_e1445 * s.db[858][13]));
        let eq63_e1447_d_b14: f64 = ((eq62_e1437_d_b14 * s.v[858]) + (eq63_e1445 * s.db[858][14]));
        let eq63_e1447_d_b15: f64 = ((eq62_e1437_d_b15 * s.v[858]) + (eq63_e1445 * s.db[858][15]));
        let eq63_e1447_d_b16: f64 = ((eq62_e1437_d_b16 * s.v[858]) + (eq63_e1445 * s.db[858][16]));
        let eq63_e1447_d_b17: f64 = ((eq62_e1437_d_b17 * s.v[858]) + (eq63_e1445 * s.db[858][17]));
        let eq63_e1447_d_b18: f64 = ((eq62_e1437_d_b18 * s.v[858]) + (eq63_e1445 * s.db[858][18]));
        let eq63_e1447_d_b19: f64 = ((eq62_e1437_d_b19 * s.v[858]) + (eq63_e1445 * s.db[858][19]));
        let eq63_e1447_d_b20: f64 = ((eq62_e1437_d_b20 * s.v[858]) + (eq63_e1445 * s.db[858][20]));
        let eq63_e1447_d_b21: f64 = ((eq62_e1437_d_b21 * s.v[858]) + (eq63_e1445 * s.db[858][21]));
        let eq63_e1447_d_b22: f64 = ((eq62_e1437_d_b22 * s.v[858]) + (eq63_e1445 * s.db[858][22]));
        let eq63_e1447_d_b23: f64 = ((eq62_e1437_d_b23 * s.v[858]) + (eq63_e1445 * s.db[858][23]));
        let eq63_e1447_d_b24: f64 = ((eq62_e1437_d_b24 * s.v[858]) + (eq63_e1445 * s.db[858][24]));
        let eq63_e1448_q: f64 = eq63_e1447;
        let eq63_reactive_node_derivatives: [f64; 21] = [eq63_e1447_d_n0, eq63_e1447_d_n1, eq63_e1447_d_n2, eq63_e1447_d_n3, eq63_e1447_d_n4, eq63_e1447_d_n5, eq63_e1447_d_n6, eq63_e1447_d_n7, eq63_e1447_d_n8, eq63_e1447_d_n9, eq63_e1447_d_n10, eq63_e1447_d_n11, eq63_e1447_d_n12, eq63_e1447_d_n13, eq63_e1447_d_n14, eq63_e1447_d_n15, eq63_e1447_d_n16, eq63_e1447_d_n17, eq63_e1447_d_n18, eq63_e1447_d_n19, eq63_e1447_d_n20];
        let eq63_reactive_branch_derivatives: [f64; 25] = [eq63_e1447_d_b0, eq63_e1447_d_b1, eq63_e1447_d_b2, eq63_e1447_d_b3, eq63_e1447_d_b4, eq63_e1447_d_b5, eq63_e1447_d_b6, eq63_e1447_d_b7, eq63_e1447_d_b8, eq63_e1447_d_b9, eq63_e1447_d_b10, eq63_e1447_d_b11, eq63_e1447_d_b12, eq63_e1447_d_b13, eq63_e1447_d_b14, eq63_e1447_d_b15, eq63_e1447_d_b16, eq63_e1447_d_b17, eq63_e1447_d_b18, eq63_e1447_d_b19, eq63_e1447_d_b20, eq63_e1447_d_b21, eq63_e1447_d_b22, eq63_e1447_d_b23, eq63_e1447_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq63_reactive_node_derivatives,
            branches,
            &eq63_reactive_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (s.v[860] * (nv4 - 0.0));
        let eq66_e1459_d_n0: f64 = (s.dn[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_n1: f64 = (s.dn[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_n2: f64 = (s.dn[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_n3: f64 = (s.dn[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_n4: f64 = ((s.dn[860][4] * (nv4 - 0.0)) + s.v[860]);
        let eq66_e1459_d_n5: f64 = (s.dn[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (s.dn[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (s.dn[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (s.dn[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_n9: f64 = (s.dn[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_n10: f64 = (s.dn[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_n11: f64 = (s.dn[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (s.dn[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (s.dn[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (s.dn[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (s.dn[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (s.dn[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (s.dn[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (s.dn[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (s.dn[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (s.dn[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b0: f64 = (s.db[860][0] * (nv4 - 0.0));
        let eq66_e1459_d_b1: f64 = (s.db[860][1] * (nv4 - 0.0));
        let eq66_e1459_d_b2: f64 = (s.db[860][2] * (nv4 - 0.0));
        let eq66_e1459_d_b3: f64 = (s.db[860][3] * (nv4 - 0.0));
        let eq66_e1459_d_b4: f64 = (s.db[860][4] * (nv4 - 0.0));
        let eq66_e1459_d_b5: f64 = (s.db[860][5] * (nv4 - 0.0));
        let eq66_e1459_d_b6: f64 = (s.db[860][6] * (nv4 - 0.0));
        let eq66_e1459_d_b7: f64 = (s.db[860][7] * (nv4 - 0.0));
        let eq66_e1459_d_b8: f64 = (s.db[860][8] * (nv4 - 0.0));
        let eq66_e1459_d_b9: f64 = (s.db[860][9] * (nv4 - 0.0));
        let eq66_e1459_d_b10: f64 = (s.db[860][10] * (nv4 - 0.0));
        let eq66_e1459_d_b11: f64 = (s.db[860][11] * (nv4 - 0.0));
        let eq66_e1459_d_b12: f64 = (s.db[860][12] * (nv4 - 0.0));
        let eq66_e1459_d_b13: f64 = (s.db[860][13] * (nv4 - 0.0));
        let eq66_e1459_d_b14: f64 = (s.db[860][14] * (nv4 - 0.0));
        let eq66_e1459_d_b15: f64 = (s.db[860][15] * (nv4 - 0.0));
        let eq66_e1459_d_b16: f64 = (s.db[860][16] * (nv4 - 0.0));
        let eq66_e1459_d_b17: f64 = (s.db[860][17] * (nv4 - 0.0));
        let eq66_e1459_d_b18: f64 = (s.db[860][18] * (nv4 - 0.0));
        let eq66_e1459_d_b19: f64 = (s.db[860][19] * (nv4 - 0.0));
        let eq66_e1459_d_b20: f64 = (s.db[860][20] * (nv4 - 0.0));
        let eq66_e1459_d_b21: f64 = (s.db[860][21] * (nv4 - 0.0));
        let eq66_e1459_d_b22: f64 = (s.db[860][22] * (nv4 - 0.0));
        let eq66_e1459_d_b23: f64 = (s.db[860][23] * (nv4 - 0.0));
        let eq66_e1459_d_b24: f64 = (s.db[860][24] * (nv4 - 0.0));
        let eq66_e1460_q: f64 = eq66_e1459;
        let eq66_reactive_node_derivatives: [f64; 21] = [eq66_e1459_d_n0, eq66_e1459_d_n1, eq66_e1459_d_n2, eq66_e1459_d_n3, eq66_e1459_d_n4, eq66_e1459_d_n5, eq66_e1459_d_n6, eq66_e1459_d_n7, eq66_e1459_d_n8, eq66_e1459_d_n9, eq66_e1459_d_n10, eq66_e1459_d_n11, eq66_e1459_d_n12, eq66_e1459_d_n13, eq66_e1459_d_n14, eq66_e1459_d_n15, eq66_e1459_d_n16, eq66_e1459_d_n17, eq66_e1459_d_n18, eq66_e1459_d_n19, eq66_e1459_d_n20];
        let eq66_reactive_branch_derivatives: [f64; 25] = [eq66_e1459_d_b0, eq66_e1459_d_b1, eq66_e1459_d_b2, eq66_e1459_d_b3, eq66_e1459_d_b4, eq66_e1459_d_b5, eq66_e1459_d_b6, eq66_e1459_d_b7, eq66_e1459_d_b8, eq66_e1459_d_b9, eq66_e1459_d_b10, eq66_e1459_d_b11, eq66_e1459_d_b12, eq66_e1459_d_b13, eq66_e1459_d_b14, eq66_e1459_d_b15, eq66_e1459_d_b16, eq66_e1459_d_b17, eq66_e1459_d_b18, eq66_e1459_d_b19, eq66_e1459_d_b20, eq66_e1459_d_b21, eq66_e1459_d_b22, eq66_e1459_d_b23, eq66_e1459_d_b24];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq67_e1463: f64 = (s.v[19] * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq67_e1464);
        let eq67_e1464_d_n0: f64 = ((s.dn[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n1: f64 = ((s.dn[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n2: f64 = ((s.dn[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n3: f64 = ((s.dn[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n4: f64 = ((s.dn[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n5: f64 = ((s.dn[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n6: f64 = ((s.dn[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n7: f64 = ((s.dn[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n8: f64 = ((s.dn[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n9: f64 = ((s.dn[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n10: f64 = ((s.dn[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n11: f64 = ((s.dn[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n12: f64 = ((s.dn[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n13: f64 = ((s.dn[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n14: f64 = ((s.dn[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n15: f64 = ((s.dn[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n16: f64 = ((s.dn[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n17: f64 = ((s.dn[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n18: f64 = ((s.dn[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n19: f64 = ((s.dn[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_n20: f64 = ((s.dn[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b0: f64 = ((s.db[19][0] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b1: f64 = ((s.db[19][1] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b2: f64 = ((s.db[19][2] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b3: f64 = ((s.db[19][3] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b4: f64 = ((s.db[19][4] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b5: f64 = ((s.db[19][5] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b6: f64 = ((s.db[19][6] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b7: f64 = ((s.db[19][7] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b8: f64 = ((s.db[19][8] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b9: f64 = ((s.db[19][9] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b10: f64 = ((s.db[19][10] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b11: f64 = ((s.db[19][11] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b12: f64 = ((s.db[19][12] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b13: f64 = ((s.db[19][13] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b14: f64 = ((s.db[19][14] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b15: f64 = ((s.db[19][15] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b16: f64 = ((s.db[19][16] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b17: f64 = ((s.db[19][17] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b18: f64 = ((s.db[19][18] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b19: f64 = ((s.db[19][19] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b20: f64 = ((s.db[19][20] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b21: f64 = ((s.db[19][21] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b22: f64 = ((s.db[19][22] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b23: f64 = ((s.db[19][23] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1464_d_b24: f64 = ((s.db[19][24] * p.p32) * __rspice_inv_cse_0);
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1466_d_n0: f64 = (eq67_e1464_d_n0 * 0.5);
        let eq67_e1466_d_n1: f64 = (eq67_e1464_d_n1 * 0.5);
        let eq67_e1466_d_n2: f64 = (eq67_e1464_d_n2 * 0.5);
        let eq67_e1466_d_n3: f64 = (eq67_e1464_d_n3 * 0.5);
        let eq67_e1466_d_n4: f64 = (eq67_e1464_d_n4 * 0.5);
        let eq67_e1466_d_n5: f64 = (eq67_e1464_d_n5 * 0.5);
        let eq67_e1466_d_n6: f64 = (eq67_e1464_d_n6 * 0.5);
        let eq67_e1466_d_n7: f64 = (eq67_e1464_d_n7 * 0.5);
        let eq67_e1466_d_n8: f64 = (eq67_e1464_d_n8 * 0.5);
        let eq67_e1466_d_n9: f64 = (eq67_e1464_d_n9 * 0.5);
        let eq67_e1466_d_n10: f64 = (eq67_e1464_d_n10 * 0.5);
        let eq67_e1466_d_n11: f64 = (eq67_e1464_d_n11 * 0.5);
        let eq67_e1466_d_n12: f64 = (eq67_e1464_d_n12 * 0.5);
        let eq67_e1466_d_n13: f64 = (eq67_e1464_d_n13 * 0.5);
        let eq67_e1466_d_n14: f64 = (eq67_e1464_d_n14 * 0.5);
        let eq67_e1466_d_n15: f64 = (eq67_e1464_d_n15 * 0.5);
        let eq67_e1466_d_n16: f64 = (eq67_e1464_d_n16 * 0.5);
        let eq67_e1466_d_n17: f64 = (eq67_e1464_d_n17 * 0.5);
        let eq67_e1466_d_n18: f64 = (eq67_e1464_d_n18 * 0.5);
        let eq67_e1466_d_n19: f64 = (eq67_e1464_d_n19 * 0.5);
        let eq67_e1466_d_n20: f64 = (eq67_e1464_d_n20 * 0.5);
        let eq67_e1466_d_b0: f64 = (eq67_e1464_d_b0 * 0.5);
        let eq67_e1466_d_b1: f64 = (eq67_e1464_d_b1 * 0.5);
        let eq67_e1466_d_b2: f64 = (eq67_e1464_d_b2 * 0.5);
        let eq67_e1466_d_b3: f64 = (eq67_e1464_d_b3 * 0.5);
        let eq67_e1466_d_b4: f64 = (eq67_e1464_d_b4 * 0.5);
        let eq67_e1466_d_b5: f64 = (eq67_e1464_d_b5 * 0.5);
        let eq67_e1466_d_b6: f64 = (eq67_e1464_d_b6 * 0.5);
        let eq67_e1466_d_b7: f64 = (eq67_e1464_d_b7 * 0.5);
        let eq67_e1466_d_b8: f64 = (eq67_e1464_d_b8 * 0.5);
        let eq67_e1466_d_b9: f64 = (eq67_e1464_d_b9 * 0.5);
        let eq67_e1466_d_b10: f64 = (eq67_e1464_d_b10 * 0.5);
        let eq67_e1466_d_b11: f64 = (eq67_e1464_d_b11 * 0.5);
        let eq67_e1466_d_b12: f64 = (eq67_e1464_d_b12 * 0.5);
        let eq67_e1466_d_b13: f64 = (eq67_e1464_d_b13 * 0.5);
        let eq67_e1466_d_b14: f64 = (eq67_e1464_d_b14 * 0.5);
        let eq67_e1466_d_b15: f64 = (eq67_e1464_d_b15 * 0.5);
        let eq67_e1466_d_b16: f64 = (eq67_e1464_d_b16 * 0.5);
        let eq67_e1466_d_b17: f64 = (eq67_e1464_d_b17 * 0.5);
        let eq67_e1466_d_b18: f64 = (eq67_e1464_d_b18 * 0.5);
        let eq67_e1466_d_b19: f64 = (eq67_e1464_d_b19 * 0.5);
        let eq67_e1466_d_b20: f64 = (eq67_e1464_d_b20 * 0.5);
        let eq67_e1466_d_b21: f64 = (eq67_e1464_d_b21 * 0.5);
        let eq67_e1466_d_b22: f64 = (eq67_e1464_d_b22 * 0.5);
        let eq67_e1466_d_b23: f64 = (eq67_e1464_d_b23 * 0.5);
        let eq67_e1466_d_b24: f64 = (eq67_e1464_d_b24 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * s.v[860]);
        let eq67_e1468_d_n0: f64 = ((eq67_e1466_d_n0 * s.v[860]) + (eq67_e1466 * s.dn[860][0]));
        let eq67_e1468_d_n1: f64 = ((eq67_e1466_d_n1 * s.v[860]) + (eq67_e1466 * s.dn[860][1]));
        let eq67_e1468_d_n2: f64 = ((eq67_e1466_d_n2 * s.v[860]) + (eq67_e1466 * s.dn[860][2]));
        let eq67_e1468_d_n3: f64 = ((eq67_e1466_d_n3 * s.v[860]) + (eq67_e1466 * s.dn[860][3]));
        let eq67_e1468_d_n4: f64 = ((eq67_e1466_d_n4 * s.v[860]) + (eq67_e1466 * s.dn[860][4]));
        let eq67_e1468_d_n5: f64 = ((eq67_e1466_d_n5 * s.v[860]) + (eq67_e1466 * s.dn[860][5]));
        let eq67_e1468_d_n6: f64 = ((eq67_e1466_d_n6 * s.v[860]) + (eq67_e1466 * s.dn[860][6]));
        let eq67_e1468_d_n7: f64 = ((eq67_e1466_d_n7 * s.v[860]) + (eq67_e1466 * s.dn[860][7]));
        let eq67_e1468_d_n8: f64 = ((eq67_e1466_d_n8 * s.v[860]) + (eq67_e1466 * s.dn[860][8]));
        let eq67_e1468_d_n9: f64 = ((eq67_e1466_d_n9 * s.v[860]) + (eq67_e1466 * s.dn[860][9]));
        let eq67_e1468_d_n10: f64 = ((eq67_e1466_d_n10 * s.v[860]) + (eq67_e1466 * s.dn[860][10]));
        let eq67_e1468_d_n11: f64 = ((eq67_e1466_d_n11 * s.v[860]) + (eq67_e1466 * s.dn[860][11]));
        let eq67_e1468_d_n12: f64 = ((eq67_e1466_d_n12 * s.v[860]) + (eq67_e1466 * s.dn[860][12]));
        let eq67_e1468_d_n13: f64 = ((eq67_e1466_d_n13 * s.v[860]) + (eq67_e1466 * s.dn[860][13]));
        let eq67_e1468_d_n14: f64 = ((eq67_e1466_d_n14 * s.v[860]) + (eq67_e1466 * s.dn[860][14]));
        let eq67_e1468_d_n15: f64 = ((eq67_e1466_d_n15 * s.v[860]) + (eq67_e1466 * s.dn[860][15]));
        let eq67_e1468_d_n16: f64 = ((eq67_e1466_d_n16 * s.v[860]) + (eq67_e1466 * s.dn[860][16]));
        let eq67_e1468_d_n17: f64 = ((eq67_e1466_d_n17 * s.v[860]) + (eq67_e1466 * s.dn[860][17]));
        let eq67_e1468_d_n18: f64 = ((eq67_e1466_d_n18 * s.v[860]) + (eq67_e1466 * s.dn[860][18]));
        let eq67_e1468_d_n19: f64 = ((eq67_e1466_d_n19 * s.v[860]) + (eq67_e1466 * s.dn[860][19]));
        let eq67_e1468_d_n20: f64 = ((eq67_e1466_d_n20 * s.v[860]) + (eq67_e1466 * s.dn[860][20]));
        let eq67_e1468_d_b0: f64 = ((eq67_e1466_d_b0 * s.v[860]) + (eq67_e1466 * s.db[860][0]));
        let eq67_e1468_d_b1: f64 = ((eq67_e1466_d_b1 * s.v[860]) + (eq67_e1466 * s.db[860][1]));
        let eq67_e1468_d_b2: f64 = ((eq67_e1466_d_b2 * s.v[860]) + (eq67_e1466 * s.db[860][2]));
        let eq67_e1468_d_b3: f64 = ((eq67_e1466_d_b3 * s.v[860]) + (eq67_e1466 * s.db[860][3]));
        let eq67_e1468_d_b4: f64 = ((eq67_e1466_d_b4 * s.v[860]) + (eq67_e1466 * s.db[860][4]));
        let eq67_e1468_d_b5: f64 = ((eq67_e1466_d_b5 * s.v[860]) + (eq67_e1466 * s.db[860][5]));
        let eq67_e1468_d_b6: f64 = ((eq67_e1466_d_b6 * s.v[860]) + (eq67_e1466 * s.db[860][6]));
        let eq67_e1468_d_b7: f64 = ((eq67_e1466_d_b7 * s.v[860]) + (eq67_e1466 * s.db[860][7]));
        let eq67_e1468_d_b8: f64 = ((eq67_e1466_d_b8 * s.v[860]) + (eq67_e1466 * s.db[860][8]));
        let eq67_e1468_d_b9: f64 = ((eq67_e1466_d_b9 * s.v[860]) + (eq67_e1466 * s.db[860][9]));
        let eq67_e1468_d_b10: f64 = ((eq67_e1466_d_b10 * s.v[860]) + (eq67_e1466 * s.db[860][10]));
        let eq67_e1468_d_b11: f64 = ((eq67_e1466_d_b11 * s.v[860]) + (eq67_e1466 * s.db[860][11]));
        let eq67_e1468_d_b12: f64 = ((eq67_e1466_d_b12 * s.v[860]) + (eq67_e1466 * s.db[860][12]));
        let eq67_e1468_d_b13: f64 = ((eq67_e1466_d_b13 * s.v[860]) + (eq67_e1466 * s.db[860][13]));
        let eq67_e1468_d_b14: f64 = ((eq67_e1466_d_b14 * s.v[860]) + (eq67_e1466 * s.db[860][14]));
        let eq67_e1468_d_b15: f64 = ((eq67_e1466_d_b15 * s.v[860]) + (eq67_e1466 * s.db[860][15]));
        let eq67_e1468_d_b16: f64 = ((eq67_e1466_d_b16 * s.v[860]) + (eq67_e1466 * s.db[860][16]));
        let eq67_e1468_d_b17: f64 = ((eq67_e1466_d_b17 * s.v[860]) + (eq67_e1466 * s.db[860][17]));
        let eq67_e1468_d_b18: f64 = ((eq67_e1466_d_b18 * s.v[860]) + (eq67_e1466 * s.db[860][18]));
        let eq67_e1468_d_b19: f64 = ((eq67_e1466_d_b19 * s.v[860]) + (eq67_e1466 * s.db[860][19]));
        let eq67_e1468_d_b20: f64 = ((eq67_e1466_d_b20 * s.v[860]) + (eq67_e1466 * s.db[860][20]));
        let eq67_e1468_d_b21: f64 = ((eq67_e1466_d_b21 * s.v[860]) + (eq67_e1466 * s.db[860][21]));
        let eq67_e1468_d_b22: f64 = ((eq67_e1466_d_b22 * s.v[860]) + (eq67_e1466 * s.db[860][22]));
        let eq67_e1468_d_b23: f64 = ((eq67_e1466_d_b23 * s.v[860]) + (eq67_e1466 * s.db[860][23]));
        let eq67_e1468_d_b24: f64 = ((eq67_e1466_d_b24 * s.v[860]) + (eq67_e1466 * s.db[860][24]));
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n0: f64 = (eq67_e1468_d_n0 * (nv4 - 0.0));
        let eq67_e1470_d_n1: f64 = (eq67_e1468_d_n1 * (nv4 - 0.0));
        let eq67_e1470_d_n2: f64 = (eq67_e1468_d_n2 * (nv4 - 0.0));
        let eq67_e1470_d_n3: f64 = (eq67_e1468_d_n3 * (nv4 - 0.0));
        let eq67_e1470_d_n4: f64 = ((eq67_e1468_d_n4 * (nv4 - 0.0)) + eq67_e1468);
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n9: f64 = (eq67_e1468_d_n9 * (nv4 - 0.0));
        let eq67_e1470_d_n10: f64 = (eq67_e1468_d_n10 * (nv4 - 0.0));
        let eq67_e1470_d_n11: f64 = (eq67_e1468_d_n11 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1470_d_b0: f64 = (eq67_e1468_d_b0 * (nv4 - 0.0));
        let eq67_e1470_d_b1: f64 = (eq67_e1468_d_b1 * (nv4 - 0.0));
        let eq67_e1470_d_b2: f64 = (eq67_e1468_d_b2 * (nv4 - 0.0));
        let eq67_e1470_d_b3: f64 = (eq67_e1468_d_b3 * (nv4 - 0.0));
        let eq67_e1470_d_b4: f64 = (eq67_e1468_d_b4 * (nv4 - 0.0));
        let eq67_e1470_d_b5: f64 = (eq67_e1468_d_b5 * (nv4 - 0.0));
        let eq67_e1470_d_b6: f64 = (eq67_e1468_d_b6 * (nv4 - 0.0));
        let eq67_e1470_d_b7: f64 = (eq67_e1468_d_b7 * (nv4 - 0.0));
        let eq67_e1470_d_b8: f64 = (eq67_e1468_d_b8 * (nv4 - 0.0));
        let eq67_e1470_d_b9: f64 = (eq67_e1468_d_b9 * (nv4 - 0.0));
        let eq67_e1470_d_b10: f64 = (eq67_e1468_d_b10 * (nv4 - 0.0));
        let eq67_e1470_d_b11: f64 = (eq67_e1468_d_b11 * (nv4 - 0.0));
        let eq67_e1470_d_b12: f64 = (eq67_e1468_d_b12 * (nv4 - 0.0));
        let eq67_e1470_d_b13: f64 = (eq67_e1468_d_b13 * (nv4 - 0.0));
        let eq67_e1470_d_b14: f64 = (eq67_e1468_d_b14 * (nv4 - 0.0));
        let eq67_e1470_d_b15: f64 = (eq67_e1468_d_b15 * (nv4 - 0.0));
        let eq67_e1470_d_b16: f64 = (eq67_e1468_d_b16 * (nv4 - 0.0));
        let eq67_e1470_d_b17: f64 = (eq67_e1468_d_b17 * (nv4 - 0.0));
        let eq67_e1470_d_b18: f64 = (eq67_e1468_d_b18 * (nv4 - 0.0));
        let eq67_e1470_d_b19: f64 = (eq67_e1468_d_b19 * (nv4 - 0.0));
        let eq67_e1470_d_b20: f64 = (eq67_e1468_d_b20 * (nv4 - 0.0));
        let eq67_e1470_d_b21: f64 = (eq67_e1468_d_b21 * (nv4 - 0.0));
        let eq67_e1470_d_b22: f64 = (eq67_e1468_d_b22 * (nv4 - 0.0));
        let eq67_e1470_d_b23: f64 = (eq67_e1468_d_b23 * (nv4 - 0.0));
        let eq67_e1470_d_b24: f64 = (eq67_e1468_d_b24 * (nv4 - 0.0));
        let eq67_e1471_q: f64 = eq67_e1470;
        let eq67_e1472: f64 = (-eq67_e1470);
        let eq67_e1472_q: f64 = (-eq67_e1471_q);
        let eq67_reactive_node_derivatives: [f64; 21] = [(-eq67_e1470_d_n0), (-eq67_e1470_d_n1), (-eq67_e1470_d_n2), (-eq67_e1470_d_n3), (-eq67_e1470_d_n4), (-eq67_e1470_d_n5), (-eq67_e1470_d_n6), (-eq67_e1470_d_n7), (-eq67_e1470_d_n8), (-eq67_e1470_d_n9), (-eq67_e1470_d_n10), (-eq67_e1470_d_n11), (-eq67_e1470_d_n12), (-eq67_e1470_d_n13), (-eq67_e1470_d_n14), (-eq67_e1470_d_n15), (-eq67_e1470_d_n16), (-eq67_e1470_d_n17), (-eq67_e1470_d_n18), (-eq67_e1470_d_n19), (-eq67_e1470_d_n20)];
        let eq67_reactive_branch_derivatives: [f64; 25] = [(-eq67_e1470_d_b0), (-eq67_e1470_d_b1), (-eq67_e1470_d_b2), (-eq67_e1470_d_b3), (-eq67_e1470_d_b4), (-eq67_e1470_d_b5), (-eq67_e1470_d_b6), (-eq67_e1470_d_b7), (-eq67_e1470_d_b8), (-eq67_e1470_d_b9), (-eq67_e1470_d_b10), (-eq67_e1470_d_b11), (-eq67_e1470_d_b12), (-eq67_e1470_d_b13), (-eq67_e1470_d_b14), (-eq67_e1470_d_b15), (-eq67_e1470_d_b16), (-eq67_e1470_d_b17), (-eq67_e1470_d_b18), (-eq67_e1470_d_b19), (-eq67_e1470_d_b20), (-eq67_e1470_d_b21), (-eq67_e1470_d_b22), (-eq67_e1470_d_b23), (-eq67_e1470_d_b24)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (s.v[19] * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let __rspice_inv_cse_1: f64 = 1.0 / (2.0 * eq68_e1476);
        let eq68_e1476_d_n0: f64 = ((s.dn[19][0] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n1: f64 = ((s.dn[19][1] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n2: f64 = ((s.dn[19][2] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n3: f64 = ((s.dn[19][3] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n4: f64 = ((s.dn[19][4] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n5: f64 = ((s.dn[19][5] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n6: f64 = ((s.dn[19][6] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n7: f64 = ((s.dn[19][7] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n8: f64 = ((s.dn[19][8] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n9: f64 = ((s.dn[19][9] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n10: f64 = ((s.dn[19][10] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n11: f64 = ((s.dn[19][11] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n12: f64 = ((s.dn[19][12] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n13: f64 = ((s.dn[19][13] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n14: f64 = ((s.dn[19][14] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n15: f64 = ((s.dn[19][15] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n16: f64 = ((s.dn[19][16] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n17: f64 = ((s.dn[19][17] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n18: f64 = ((s.dn[19][18] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n19: f64 = ((s.dn[19][19] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_n20: f64 = ((s.dn[19][20] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b0: f64 = ((s.db[19][0] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b1: f64 = ((s.db[19][1] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b2: f64 = ((s.db[19][2] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b3: f64 = ((s.db[19][3] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b4: f64 = ((s.db[19][4] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b5: f64 = ((s.db[19][5] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b6: f64 = ((s.db[19][6] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b7: f64 = ((s.db[19][7] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b8: f64 = ((s.db[19][8] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b9: f64 = ((s.db[19][9] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b10: f64 = ((s.db[19][10] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b11: f64 = ((s.db[19][11] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b12: f64 = ((s.db[19][12] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b13: f64 = ((s.db[19][13] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b14: f64 = ((s.db[19][14] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b15: f64 = ((s.db[19][15] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b16: f64 = ((s.db[19][16] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b17: f64 = ((s.db[19][17] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b18: f64 = ((s.db[19][18] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b19: f64 = ((s.db[19][19] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b20: f64 = ((s.db[19][20] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b21: f64 = ((s.db[19][21] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b22: f64 = ((s.db[19][22] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b23: f64 = ((s.db[19][23] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1476_d_b24: f64 = ((s.db[19][24] * p.p32) * __rspice_inv_cse_1);
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1478_d_n0: f64 = (eq68_e1476_d_n0 * 0.5);
        let eq68_e1478_d_n1: f64 = (eq68_e1476_d_n1 * 0.5);
        let eq68_e1478_d_n2: f64 = (eq68_e1476_d_n2 * 0.5);
        let eq68_e1478_d_n3: f64 = (eq68_e1476_d_n3 * 0.5);
        let eq68_e1478_d_n4: f64 = (eq68_e1476_d_n4 * 0.5);
        let eq68_e1478_d_n5: f64 = (eq68_e1476_d_n5 * 0.5);
        let eq68_e1478_d_n6: f64 = (eq68_e1476_d_n6 * 0.5);
        let eq68_e1478_d_n7: f64 = (eq68_e1476_d_n7 * 0.5);
        let eq68_e1478_d_n8: f64 = (eq68_e1476_d_n8 * 0.5);
        let eq68_e1478_d_n9: f64 = (eq68_e1476_d_n9 * 0.5);
        let eq68_e1478_d_n10: f64 = (eq68_e1476_d_n10 * 0.5);
        let eq68_e1478_d_n11: f64 = (eq68_e1476_d_n11 * 0.5);
        let eq68_e1478_d_n12: f64 = (eq68_e1476_d_n12 * 0.5);
        let eq68_e1478_d_n13: f64 = (eq68_e1476_d_n13 * 0.5);
        let eq68_e1478_d_n14: f64 = (eq68_e1476_d_n14 * 0.5);
        let eq68_e1478_d_n15: f64 = (eq68_e1476_d_n15 * 0.5);
        let eq68_e1478_d_n16: f64 = (eq68_e1476_d_n16 * 0.5);
        let eq68_e1478_d_n17: f64 = (eq68_e1476_d_n17 * 0.5);
        let eq68_e1478_d_n18: f64 = (eq68_e1476_d_n18 * 0.5);
        let eq68_e1478_d_n19: f64 = (eq68_e1476_d_n19 * 0.5);
        let eq68_e1478_d_n20: f64 = (eq68_e1476_d_n20 * 0.5);
        let eq68_e1478_d_b0: f64 = (eq68_e1476_d_b0 * 0.5);
        let eq68_e1478_d_b1: f64 = (eq68_e1476_d_b1 * 0.5);
        let eq68_e1478_d_b2: f64 = (eq68_e1476_d_b2 * 0.5);
        let eq68_e1478_d_b3: f64 = (eq68_e1476_d_b3 * 0.5);
        let eq68_e1478_d_b4: f64 = (eq68_e1476_d_b4 * 0.5);
        let eq68_e1478_d_b5: f64 = (eq68_e1476_d_b5 * 0.5);
        let eq68_e1478_d_b6: f64 = (eq68_e1476_d_b6 * 0.5);
        let eq68_e1478_d_b7: f64 = (eq68_e1476_d_b7 * 0.5);
        let eq68_e1478_d_b8: f64 = (eq68_e1476_d_b8 * 0.5);
        let eq68_e1478_d_b9: f64 = (eq68_e1476_d_b9 * 0.5);
        let eq68_e1478_d_b10: f64 = (eq68_e1476_d_b10 * 0.5);
        let eq68_e1478_d_b11: f64 = (eq68_e1476_d_b11 * 0.5);
        let eq68_e1478_d_b12: f64 = (eq68_e1476_d_b12 * 0.5);
        let eq68_e1478_d_b13: f64 = (eq68_e1476_d_b13 * 0.5);
        let eq68_e1478_d_b14: f64 = (eq68_e1476_d_b14 * 0.5);
        let eq68_e1478_d_b15: f64 = (eq68_e1476_d_b15 * 0.5);
        let eq68_e1478_d_b16: f64 = (eq68_e1476_d_b16 * 0.5);
        let eq68_e1478_d_b17: f64 = (eq68_e1476_d_b17 * 0.5);
        let eq68_e1478_d_b18: f64 = (eq68_e1476_d_b18 * 0.5);
        let eq68_e1478_d_b19: f64 = (eq68_e1476_d_b19 * 0.5);
        let eq68_e1478_d_b20: f64 = (eq68_e1476_d_b20 * 0.5);
        let eq68_e1478_d_b21: f64 = (eq68_e1476_d_b21 * 0.5);
        let eq68_e1478_d_b22: f64 = (eq68_e1476_d_b22 * 0.5);
        let eq68_e1478_d_b23: f64 = (eq68_e1476_d_b23 * 0.5);
        let eq68_e1478_d_b24: f64 = (eq68_e1476_d_b24 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * s.v[860]);
        let eq68_e1480_d_n0: f64 = ((eq68_e1478_d_n0 * s.v[860]) + (eq68_e1478 * s.dn[860][0]));
        let eq68_e1480_d_n1: f64 = ((eq68_e1478_d_n1 * s.v[860]) + (eq68_e1478 * s.dn[860][1]));
        let eq68_e1480_d_n2: f64 = ((eq68_e1478_d_n2 * s.v[860]) + (eq68_e1478 * s.dn[860][2]));
        let eq68_e1480_d_n3: f64 = ((eq68_e1478_d_n3 * s.v[860]) + (eq68_e1478 * s.dn[860][3]));
        let eq68_e1480_d_n4: f64 = ((eq68_e1478_d_n4 * s.v[860]) + (eq68_e1478 * s.dn[860][4]));
        let eq68_e1480_d_n5: f64 = ((eq68_e1478_d_n5 * s.v[860]) + (eq68_e1478 * s.dn[860][5]));
        let eq68_e1480_d_n6: f64 = ((eq68_e1478_d_n6 * s.v[860]) + (eq68_e1478 * s.dn[860][6]));
        let eq68_e1480_d_n7: f64 = ((eq68_e1478_d_n7 * s.v[860]) + (eq68_e1478 * s.dn[860][7]));
        let eq68_e1480_d_n8: f64 = ((eq68_e1478_d_n8 * s.v[860]) + (eq68_e1478 * s.dn[860][8]));
        let eq68_e1480_d_n9: f64 = ((eq68_e1478_d_n9 * s.v[860]) + (eq68_e1478 * s.dn[860][9]));
        let eq68_e1480_d_n10: f64 = ((eq68_e1478_d_n10 * s.v[860]) + (eq68_e1478 * s.dn[860][10]));
        let eq68_e1480_d_n11: f64 = ((eq68_e1478_d_n11 * s.v[860]) + (eq68_e1478 * s.dn[860][11]));
        let eq68_e1480_d_n12: f64 = ((eq68_e1478_d_n12 * s.v[860]) + (eq68_e1478 * s.dn[860][12]));
        let eq68_e1480_d_n13: f64 = ((eq68_e1478_d_n13 * s.v[860]) + (eq68_e1478 * s.dn[860][13]));
        let eq68_e1480_d_n14: f64 = ((eq68_e1478_d_n14 * s.v[860]) + (eq68_e1478 * s.dn[860][14]));
        let eq68_e1480_d_n15: f64 = ((eq68_e1478_d_n15 * s.v[860]) + (eq68_e1478 * s.dn[860][15]));
        let eq68_e1480_d_n16: f64 = ((eq68_e1478_d_n16 * s.v[860]) + (eq68_e1478 * s.dn[860][16]));
        let eq68_e1480_d_n17: f64 = ((eq68_e1478_d_n17 * s.v[860]) + (eq68_e1478 * s.dn[860][17]));
        let eq68_e1480_d_n18: f64 = ((eq68_e1478_d_n18 * s.v[860]) + (eq68_e1478 * s.dn[860][18]));
        let eq68_e1480_d_n19: f64 = ((eq68_e1478_d_n19 * s.v[860]) + (eq68_e1478 * s.dn[860][19]));
        let eq68_e1480_d_n20: f64 = ((eq68_e1478_d_n20 * s.v[860]) + (eq68_e1478 * s.dn[860][20]));
        let eq68_e1480_d_b0: f64 = ((eq68_e1478_d_b0 * s.v[860]) + (eq68_e1478 * s.db[860][0]));
        let eq68_e1480_d_b1: f64 = ((eq68_e1478_d_b1 * s.v[860]) + (eq68_e1478 * s.db[860][1]));
        let eq68_e1480_d_b2: f64 = ((eq68_e1478_d_b2 * s.v[860]) + (eq68_e1478 * s.db[860][2]));
        let eq68_e1480_d_b3: f64 = ((eq68_e1478_d_b3 * s.v[860]) + (eq68_e1478 * s.db[860][3]));
        let eq68_e1480_d_b4: f64 = ((eq68_e1478_d_b4 * s.v[860]) + (eq68_e1478 * s.db[860][4]));
        let eq68_e1480_d_b5: f64 = ((eq68_e1478_d_b5 * s.v[860]) + (eq68_e1478 * s.db[860][5]));
        let eq68_e1480_d_b6: f64 = ((eq68_e1478_d_b6 * s.v[860]) + (eq68_e1478 * s.db[860][6]));
        let eq68_e1480_d_b7: f64 = ((eq68_e1478_d_b7 * s.v[860]) + (eq68_e1478 * s.db[860][7]));
        let eq68_e1480_d_b8: f64 = ((eq68_e1478_d_b8 * s.v[860]) + (eq68_e1478 * s.db[860][8]));
        let eq68_e1480_d_b9: f64 = ((eq68_e1478_d_b9 * s.v[860]) + (eq68_e1478 * s.db[860][9]));
        let eq68_e1480_d_b10: f64 = ((eq68_e1478_d_b10 * s.v[860]) + (eq68_e1478 * s.db[860][10]));
        let eq68_e1480_d_b11: f64 = ((eq68_e1478_d_b11 * s.v[860]) + (eq68_e1478 * s.db[860][11]));
        let eq68_e1480_d_b12: f64 = ((eq68_e1478_d_b12 * s.v[860]) + (eq68_e1478 * s.db[860][12]));
        let eq68_e1480_d_b13: f64 = ((eq68_e1478_d_b13 * s.v[860]) + (eq68_e1478 * s.db[860][13]));
        let eq68_e1480_d_b14: f64 = ((eq68_e1478_d_b14 * s.v[860]) + (eq68_e1478 * s.db[860][14]));
        let eq68_e1480_d_b15: f64 = ((eq68_e1478_d_b15 * s.v[860]) + (eq68_e1478 * s.db[860][15]));
        let eq68_e1480_d_b16: f64 = ((eq68_e1478_d_b16 * s.v[860]) + (eq68_e1478 * s.db[860][16]));
        let eq68_e1480_d_b17: f64 = ((eq68_e1478_d_b17 * s.v[860]) + (eq68_e1478 * s.db[860][17]));
        let eq68_e1480_d_b18: f64 = ((eq68_e1478_d_b18 * s.v[860]) + (eq68_e1478 * s.db[860][18]));
        let eq68_e1480_d_b19: f64 = ((eq68_e1478_d_b19 * s.v[860]) + (eq68_e1478 * s.db[860][19]));
        let eq68_e1480_d_b20: f64 = ((eq68_e1478_d_b20 * s.v[860]) + (eq68_e1478 * s.db[860][20]));
        let eq68_e1480_d_b21: f64 = ((eq68_e1478_d_b21 * s.v[860]) + (eq68_e1478 * s.db[860][21]));
        let eq68_e1480_d_b22: f64 = ((eq68_e1478_d_b22 * s.v[860]) + (eq68_e1478 * s.db[860][22]));
        let eq68_e1480_d_b23: f64 = ((eq68_e1478_d_b23 * s.v[860]) + (eq68_e1478 * s.db[860][23]));
        let eq68_e1480_d_b24: f64 = ((eq68_e1478_d_b24 * s.v[860]) + (eq68_e1478 * s.db[860][24]));
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n0: f64 = (eq68_e1480_d_n0 * (nv4 - 0.0));
        let eq68_e1482_d_n1: f64 = (eq68_e1480_d_n1 * (nv4 - 0.0));
        let eq68_e1482_d_n2: f64 = (eq68_e1480_d_n2 * (nv4 - 0.0));
        let eq68_e1482_d_n3: f64 = (eq68_e1480_d_n3 * (nv4 - 0.0));
        let eq68_e1482_d_n4: f64 = ((eq68_e1480_d_n4 * (nv4 - 0.0)) + eq68_e1480);
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n9: f64 = (eq68_e1480_d_n9 * (nv4 - 0.0));
        let eq68_e1482_d_n10: f64 = (eq68_e1480_d_n10 * (nv4 - 0.0));
        let eq68_e1482_d_n11: f64 = (eq68_e1480_d_n11 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1482_d_b0: f64 = (eq68_e1480_d_b0 * (nv4 - 0.0));
        let eq68_e1482_d_b1: f64 = (eq68_e1480_d_b1 * (nv4 - 0.0));
        let eq68_e1482_d_b2: f64 = (eq68_e1480_d_b2 * (nv4 - 0.0));
        let eq68_e1482_d_b3: f64 = (eq68_e1480_d_b3 * (nv4 - 0.0));
        let eq68_e1482_d_b4: f64 = (eq68_e1480_d_b4 * (nv4 - 0.0));
        let eq68_e1482_d_b5: f64 = (eq68_e1480_d_b5 * (nv4 - 0.0));
        let eq68_e1482_d_b6: f64 = (eq68_e1480_d_b6 * (nv4 - 0.0));
        let eq68_e1482_d_b7: f64 = (eq68_e1480_d_b7 * (nv4 - 0.0));
        let eq68_e1482_d_b8: f64 = (eq68_e1480_d_b8 * (nv4 - 0.0));
        let eq68_e1482_d_b9: f64 = (eq68_e1480_d_b9 * (nv4 - 0.0));
        let eq68_e1482_d_b10: f64 = (eq68_e1480_d_b10 * (nv4 - 0.0));
        let eq68_e1482_d_b11: f64 = (eq68_e1480_d_b11 * (nv4 - 0.0));
        let eq68_e1482_d_b12: f64 = (eq68_e1480_d_b12 * (nv4 - 0.0));
        let eq68_e1482_d_b13: f64 = (eq68_e1480_d_b13 * (nv4 - 0.0));
        let eq68_e1482_d_b14: f64 = (eq68_e1480_d_b14 * (nv4 - 0.0));
        let eq68_e1482_d_b15: f64 = (eq68_e1480_d_b15 * (nv4 - 0.0));
        let eq68_e1482_d_b16: f64 = (eq68_e1480_d_b16 * (nv4 - 0.0));
        let eq68_e1482_d_b17: f64 = (eq68_e1480_d_b17 * (nv4 - 0.0));
        let eq68_e1482_d_b18: f64 = (eq68_e1480_d_b18 * (nv4 - 0.0));
        let eq68_e1482_d_b19: f64 = (eq68_e1480_d_b19 * (nv4 - 0.0));
        let eq68_e1482_d_b20: f64 = (eq68_e1480_d_b20 * (nv4 - 0.0));
        let eq68_e1482_d_b21: f64 = (eq68_e1480_d_b21 * (nv4 - 0.0));
        let eq68_e1482_d_b22: f64 = (eq68_e1480_d_b22 * (nv4 - 0.0));
        let eq68_e1482_d_b23: f64 = (eq68_e1480_d_b23 * (nv4 - 0.0));
        let eq68_e1482_d_b24: f64 = (eq68_e1480_d_b24 * (nv4 - 0.0));
        let eq68_e1483_q: f64 = eq68_e1482;
        let eq68_e1484: f64 = (-eq68_e1482);
        let eq68_e1484_q: f64 = (-eq68_e1483_q);
        let eq68_reactive_node_derivatives: [f64; 21] = [(-eq68_e1482_d_n0), (-eq68_e1482_d_n1), (-eq68_e1482_d_n2), (-eq68_e1482_d_n3), (-eq68_e1482_d_n4), (-eq68_e1482_d_n5), (-eq68_e1482_d_n6), (-eq68_e1482_d_n7), (-eq68_e1482_d_n8), (-eq68_e1482_d_n9), (-eq68_e1482_d_n10), (-eq68_e1482_d_n11), (-eq68_e1482_d_n12), (-eq68_e1482_d_n13), (-eq68_e1482_d_n14), (-eq68_e1482_d_n15), (-eq68_e1482_d_n16), (-eq68_e1482_d_n17), (-eq68_e1482_d_n18), (-eq68_e1482_d_n19), (-eq68_e1482_d_n20)];
        let eq68_reactive_branch_derivatives: [f64; 25] = [(-eq68_e1482_d_b0), (-eq68_e1482_d_b1), (-eq68_e1482_d_b2), (-eq68_e1482_d_b3), (-eq68_e1482_d_b4), (-eq68_e1482_d_b5), (-eq68_e1482_d_b6), (-eq68_e1482_d_b7), (-eq68_e1482_d_b8), (-eq68_e1482_d_b9), (-eq68_e1482_d_b10), (-eq68_e1482_d_b11), (-eq68_e1482_d_b12), (-eq68_e1482_d_b13), (-eq68_e1482_d_b14), (-eq68_e1482_d_b15), (-eq68_e1482_d_b16), (-eq68_e1482_d_b17), (-eq68_e1482_d_b18), (-eq68_e1482_d_b19), (-eq68_e1482_d_b20), (-eq68_e1482_d_b21), (-eq68_e1482_d_b22), (-eq68_e1482_d_b23), (-eq68_e1482_d_b24)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
