#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq11_e1050: f64 = (s.v[0] * s.v[15]);let eq11_e1050_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq11_e1050_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq11_e1050_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq11_e1050_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq11_e1050_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq11_e1050_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq11_e1050_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq11_e1050_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq11_e1050_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq11_e1050_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq11_e1050_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq11_e1050_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq11_e1050_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq11_e1050_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq11_e1050_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq11_e1050_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq11_e1050_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq11_e1050_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq11_e1050_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq11_e1052: f64 = (eq11_e1050 * p.p32);let eq11_e1052_d_n0: f64 = (eq11_e1050_d_n0 * p.p32);let eq11_e1052_d_n1: f64 = (eq11_e1050_d_n1 * p.p32);let eq11_e1052_d_n2: f64 = (eq11_e1050_d_n2 * p.p32);let eq11_e1052_d_n3: f64 = (eq11_e1050_d_n3 * p.p32);let eq11_e1052_d_n4: f64 = (eq11_e1050_d_n4 * p.p32);let eq11_e1052_d_n5: f64 = (eq11_e1050_d_n5 * p.p32);let eq11_e1052_d_n6: f64 = (eq11_e1050_d_n6 * p.p32);let eq11_e1052_d_n7: f64 = (eq11_e1050_d_n7 * p.p32);let eq11_e1052_d_n8: f64 = (eq11_e1050_d_n8 * p.p32);let eq11_e1052_d_n9: f64 = (eq11_e1050_d_n9 * p.p32);let eq11_e1052_d_n10: f64 = (eq11_e1050_d_n10 * p.p32);let eq11_e1052_d_n11: f64 = (eq11_e1050_d_n11 * p.p32);let eq11_e1052_d_b0: f64 = (eq11_e1050_d_b0 * p.p32);let eq11_e1052_d_b1: f64 = (eq11_e1050_d_b1 * p.p32);let eq11_e1052_d_b2: f64 = (eq11_e1050_d_b2 * p.p32);let eq11_e1052_d_b3: f64 = (eq11_e1050_d_b3 * p.p32);let eq11_e1052_d_b4: f64 = (eq11_e1050_d_b4 * p.p32);let eq11_e1052_d_b5: f64 = (eq11_e1050_d_b5 * p.p32);let eq11_e1052_d_b6: f64 = (eq11_e1050_d_b6 * p.p32);let eq11_e1054: f64 = (eq11_e1052 * s.v[838]);let eq11_e1054_d_n0: f64 = ((eq11_e1052_d_n0 * s.v[838]) + (eq11_e1052 * s.dn[838][0]));let eq11_e1054_d_n1: f64 = ((eq11_e1052_d_n1 * s.v[838]) + (eq11_e1052 * s.dn[838][1]));let eq11_e1054_d_n2: f64 = ((eq11_e1052_d_n2 * s.v[838]) + (eq11_e1052 * s.dn[838][2]));let eq11_e1054_d_n3: f64 = ((eq11_e1052_d_n3 * s.v[838]) + (eq11_e1052 * s.dn[838][3]));let eq11_e1054_d_n4: f64 = ((eq11_e1052_d_n4 * s.v[838]) + (eq11_e1052 * s.dn[838][4]));let eq11_e1054_d_n5: f64 = ((eq11_e1052_d_n5 * s.v[838]) + (eq11_e1052 * s.dn[838][5]));let eq11_e1054_d_n6: f64 = ((eq11_e1052_d_n6 * s.v[838]) + (eq11_e1052 * s.dn[838][6]));let eq11_e1054_d_n7: f64 = ((eq11_e1052_d_n7 * s.v[838]) + (eq11_e1052 * s.dn[838][7]));let eq11_e1054_d_n8: f64 = ((eq11_e1052_d_n8 * s.v[838]) + (eq11_e1052 * s.dn[838][8]));let eq11_e1054_d_n9: f64 = ((eq11_e1052_d_n9 * s.v[838]) + (eq11_e1052 * s.dn[838][9]));let eq11_e1054_d_n10: f64 = ((eq11_e1052_d_n10 * s.v[838]) + (eq11_e1052 * s.dn[838][10]));let eq11_e1054_d_n11: f64 = ((eq11_e1052_d_n11 * s.v[838]) + (eq11_e1052 * s.dn[838][11]));let eq11_e1054_d_b0: f64 = ((eq11_e1052_d_b0 * s.v[838]) + (eq11_e1052 * s.db[838][0]));let eq11_e1054_d_b1: f64 = ((eq11_e1052_d_b1 * s.v[838]) + (eq11_e1052 * s.db[838][1]));let eq11_e1054_d_b2: f64 = ((eq11_e1052_d_b2 * s.v[838]) + (eq11_e1052 * s.db[838][2]));let eq11_e1054_d_b3: f64 = ((eq11_e1052_d_b3 * s.v[838]) + (eq11_e1052 * s.db[838][3]));
        let eq11_e1054_d_b4: f64 = ((eq11_e1052_d_b4 * s.v[838]) + (eq11_e1052 * s.db[838][4]));let eq11_e1054_d_b5: f64 = ((eq11_e1052_d_b5 * s.v[838]) + (eq11_e1052 * s.db[838][5]));let eq11_e1054_d_b6: f64 = ((eq11_e1052_d_b6 * s.v[838]) + (eq11_e1052 * s.db[838][6]));let eq11_value: f64 = eq11_e1054;let eq11_node_derivatives: [f64; 12] = [eq11_e1054_d_n0, eq11_e1054_d_n1, eq11_e1054_d_n2, eq11_e1054_d_n3, eq11_e1054_d_n4, eq11_e1054_d_n5, eq11_e1054_d_n6, eq11_e1054_d_n7, eq11_e1054_d_n8, eq11_e1054_d_n9, eq11_e1054_d_n10, eq11_e1054_d_n11];let eq11_branch_derivatives: [f64; 7] = [eq11_e1054_d_b0, eq11_e1054_d_b1, eq11_e1054_d_b2, eq11_e1054_d_b3, eq11_e1054_d_b4, eq11_e1054_d_b5, eq11_e1054_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq12_e1057: f64 = (s.v[0] * s.v[15]);let eq12_e1057_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq12_e1057_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq12_e1057_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq12_e1057_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq12_e1057_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq12_e1057_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq12_e1057_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq12_e1057_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq12_e1057_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq12_e1057_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq12_e1057_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq12_e1057_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq12_e1057_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq12_e1057_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq12_e1057_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq12_e1057_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq12_e1057_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq12_e1057_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq12_e1057_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq12_e1059: f64 = (eq12_e1057 * p.p32);let eq12_e1059_d_n0: f64 = (eq12_e1057_d_n0 * p.p32);let eq12_e1059_d_n1: f64 = (eq12_e1057_d_n1 * p.p32);let eq12_e1059_d_n2: f64 = (eq12_e1057_d_n2 * p.p32);let eq12_e1059_d_n3: f64 = (eq12_e1057_d_n3 * p.p32);let eq12_e1059_d_n4: f64 = (eq12_e1057_d_n4 * p.p32);let eq12_e1059_d_n5: f64 = (eq12_e1057_d_n5 * p.p32);let eq12_e1059_d_n6: f64 = (eq12_e1057_d_n6 * p.p32);let eq12_e1059_d_n7: f64 = (eq12_e1057_d_n7 * p.p32);let eq12_e1059_d_n8: f64 = (eq12_e1057_d_n8 * p.p32);let eq12_e1059_d_n9: f64 = (eq12_e1057_d_n9 * p.p32);let eq12_e1059_d_n10: f64 = (eq12_e1057_d_n10 * p.p32);let eq12_e1059_d_n11: f64 = (eq12_e1057_d_n11 * p.p32);let eq12_e1059_d_b0: f64 = (eq12_e1057_d_b0 * p.p32);let eq12_e1059_d_b1: f64 = (eq12_e1057_d_b1 * p.p32);let eq12_e1059_d_b2: f64 = (eq12_e1057_d_b2 * p.p32);let eq12_e1059_d_b3: f64 = (eq12_e1057_d_b3 * p.p32);let eq12_e1059_d_b4: f64 = (eq12_e1057_d_b4 * p.p32);let eq12_e1059_d_b5: f64 = (eq12_e1057_d_b5 * p.p32);let eq12_e1059_d_b6: f64 = (eq12_e1057_d_b6 * p.p32);let eq12_e1061: f64 = (eq12_e1059 * s.v[839]);let eq12_e1061_d_n0: f64 = ((eq12_e1059_d_n0 * s.v[839]) + (eq12_e1059 * s.dn[839][0]));let eq12_e1061_d_n1: f64 = ((eq12_e1059_d_n1 * s.v[839]) + (eq12_e1059 * s.dn[839][1]));let eq12_e1061_d_n2: f64 = ((eq12_e1059_d_n2 * s.v[839]) + (eq12_e1059 * s.dn[839][2]));let eq12_e1061_d_n3: f64 = ((eq12_e1059_d_n3 * s.v[839]) + (eq12_e1059 * s.dn[839][3]));let eq12_e1061_d_n4: f64 = ((eq12_e1059_d_n4 * s.v[839]) + (eq12_e1059 * s.dn[839][4]));let eq12_e1061_d_n5: f64 = ((eq12_e1059_d_n5 * s.v[839]) + (eq12_e1059 * s.dn[839][5]));let eq12_e1061_d_n6: f64 = ((eq12_e1059_d_n6 * s.v[839]) + (eq12_e1059 * s.dn[839][6]));let eq12_e1061_d_n7: f64 = ((eq12_e1059_d_n7 * s.v[839]) + (eq12_e1059 * s.dn[839][7]));let eq12_e1061_d_n8: f64 = ((eq12_e1059_d_n8 * s.v[839]) + (eq12_e1059 * s.dn[839][8]));let eq12_e1061_d_n9: f64 = ((eq12_e1059_d_n9 * s.v[839]) + (eq12_e1059 * s.dn[839][9]));let eq12_e1061_d_n10: f64 = ((eq12_e1059_d_n10 * s.v[839]) + (eq12_e1059 * s.dn[839][10]));let eq12_e1061_d_n11: f64 = ((eq12_e1059_d_n11 * s.v[839]) + (eq12_e1059 * s.dn[839][11]));let eq12_e1061_d_b0: f64 = ((eq12_e1059_d_b0 * s.v[839]) + (eq12_e1059 * s.db[839][0]));let eq12_e1061_d_b1: f64 = ((eq12_e1059_d_b1 * s.v[839]) + (eq12_e1059 * s.db[839][1]));let eq12_e1061_d_b2: f64 = ((eq12_e1059_d_b2 * s.v[839]) + (eq12_e1059 * s.db[839][2]));let eq12_e1061_d_b3: f64 = ((eq12_e1059_d_b3 * s.v[839]) + (eq12_e1059 * s.db[839][3]));
        let eq12_e1061_d_b4: f64 = ((eq12_e1059_d_b4 * s.v[839]) + (eq12_e1059 * s.db[839][4]));let eq12_e1061_d_b5: f64 = ((eq12_e1059_d_b5 * s.v[839]) + (eq12_e1059 * s.db[839][5]));let eq12_e1061_d_b6: f64 = ((eq12_e1059_d_b6 * s.v[839]) + (eq12_e1059 * s.db[839][6]));let eq12_value: f64 = eq12_e1061;let eq12_node_derivatives: [f64; 12] = [eq12_e1061_d_n0, eq12_e1061_d_n1, eq12_e1061_d_n2, eq12_e1061_d_n3, eq12_e1061_d_n4, eq12_e1061_d_n5, eq12_e1061_d_n6, eq12_e1061_d_n7, eq12_e1061_d_n8, eq12_e1061_d_n9, eq12_e1061_d_n10, eq12_e1061_d_n11];let eq12_branch_derivatives: [f64; 7] = [eq12_e1061_d_b0, eq12_e1061_d_b1, eq12_e1061_d_b2, eq12_e1061_d_b3, eq12_e1061_d_b4, eq12_e1061_d_b5, eq12_e1061_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq13_e1064: f64 = (s.v[0] * s.v[15]);let eq13_e1064_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq13_e1064_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq13_e1064_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq13_e1064_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq13_e1064_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq13_e1064_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq13_e1064_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq13_e1064_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq13_e1064_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq13_e1064_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq13_e1064_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq13_e1064_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq13_e1064_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq13_e1064_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq13_e1064_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq13_e1064_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq13_e1064_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq13_e1064_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq13_e1064_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq13_e1066: f64 = (eq13_e1064 * p.p32);let eq13_e1066_d_n0: f64 = (eq13_e1064_d_n0 * p.p32);let eq13_e1066_d_n1: f64 = (eq13_e1064_d_n1 * p.p32);let eq13_e1066_d_n2: f64 = (eq13_e1064_d_n2 * p.p32);let eq13_e1066_d_n3: f64 = (eq13_e1064_d_n3 * p.p32);let eq13_e1066_d_n4: f64 = (eq13_e1064_d_n4 * p.p32);let eq13_e1066_d_n5: f64 = (eq13_e1064_d_n5 * p.p32);let eq13_e1066_d_n6: f64 = (eq13_e1064_d_n6 * p.p32);let eq13_e1066_d_n7: f64 = (eq13_e1064_d_n7 * p.p32);let eq13_e1066_d_n8: f64 = (eq13_e1064_d_n8 * p.p32);let eq13_e1066_d_n9: f64 = (eq13_e1064_d_n9 * p.p32);let eq13_e1066_d_n10: f64 = (eq13_e1064_d_n10 * p.p32);let eq13_e1066_d_n11: f64 = (eq13_e1064_d_n11 * p.p32);let eq13_e1066_d_b0: f64 = (eq13_e1064_d_b0 * p.p32);let eq13_e1066_d_b1: f64 = (eq13_e1064_d_b1 * p.p32);let eq13_e1066_d_b2: f64 = (eq13_e1064_d_b2 * p.p32);let eq13_e1066_d_b3: f64 = (eq13_e1064_d_b3 * p.p32);let eq13_e1066_d_b4: f64 = (eq13_e1064_d_b4 * p.p32);let eq13_e1066_d_b5: f64 = (eq13_e1064_d_b5 * p.p32);let eq13_e1066_d_b6: f64 = (eq13_e1064_d_b6 * p.p32);let eq13_e1068: f64 = (eq13_e1066 * s.v[842]);let eq13_e1068_d_n0: f64 = ((eq13_e1066_d_n0 * s.v[842]) + (eq13_e1066 * s.dn[842][0]));let eq13_e1068_d_n1: f64 = ((eq13_e1066_d_n1 * s.v[842]) + (eq13_e1066 * s.dn[842][1]));let eq13_e1068_d_n2: f64 = ((eq13_e1066_d_n2 * s.v[842]) + (eq13_e1066 * s.dn[842][2]));let eq13_e1068_d_n3: f64 = ((eq13_e1066_d_n3 * s.v[842]) + (eq13_e1066 * s.dn[842][3]));let eq13_e1068_d_n4: f64 = ((eq13_e1066_d_n4 * s.v[842]) + (eq13_e1066 * s.dn[842][4]));let eq13_e1068_d_n5: f64 = ((eq13_e1066_d_n5 * s.v[842]) + (eq13_e1066 * s.dn[842][5]));let eq13_e1068_d_n6: f64 = ((eq13_e1066_d_n6 * s.v[842]) + (eq13_e1066 * s.dn[842][6]));let eq13_e1068_d_n7: f64 = ((eq13_e1066_d_n7 * s.v[842]) + (eq13_e1066 * s.dn[842][7]));let eq13_e1068_d_n8: f64 = ((eq13_e1066_d_n8 * s.v[842]) + (eq13_e1066 * s.dn[842][8]));let eq13_e1068_d_n9: f64 = ((eq13_e1066_d_n9 * s.v[842]) + (eq13_e1066 * s.dn[842][9]));let eq13_e1068_d_n10: f64 = ((eq13_e1066_d_n10 * s.v[842]) + (eq13_e1066 * s.dn[842][10]));let eq13_e1068_d_n11: f64 = ((eq13_e1066_d_n11 * s.v[842]) + (eq13_e1066 * s.dn[842][11]));let eq13_e1068_d_b0: f64 = ((eq13_e1066_d_b0 * s.v[842]) + (eq13_e1066 * s.db[842][0]));let eq13_e1068_d_b1: f64 = ((eq13_e1066_d_b1 * s.v[842]) + (eq13_e1066 * s.db[842][1]));let eq13_e1068_d_b2: f64 = ((eq13_e1066_d_b2 * s.v[842]) + (eq13_e1066 * s.db[842][2]));let eq13_e1068_d_b3: f64 = ((eq13_e1066_d_b3 * s.v[842]) + (eq13_e1066 * s.db[842][3]));
        let eq13_e1068_d_b4: f64 = ((eq13_e1066_d_b4 * s.v[842]) + (eq13_e1066 * s.db[842][4]));let eq13_e1068_d_b5: f64 = ((eq13_e1066_d_b5 * s.v[842]) + (eq13_e1066 * s.db[842][5]));let eq13_e1068_d_b6: f64 = ((eq13_e1066_d_b6 * s.v[842]) + (eq13_e1066 * s.db[842][6]));let eq13_value: f64 = eq13_e1068;let eq13_node_derivatives: [f64; 12] = [eq13_e1068_d_n0, eq13_e1068_d_n1, eq13_e1068_d_n2, eq13_e1068_d_n3, eq13_e1068_d_n4, eq13_e1068_d_n5, eq13_e1068_d_n6, eq13_e1068_d_n7, eq13_e1068_d_n8, eq13_e1068_d_n9, eq13_e1068_d_n10, eq13_e1068_d_n11];let eq13_branch_derivatives: [f64; 7] = [eq13_e1068_d_b0, eq13_e1068_d_b1, eq13_e1068_d_b2, eq13_e1068_d_b3, eq13_e1068_d_b4, eq13_e1068_d_b5, eq13_e1068_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq14_e1071: f64 = (s.v[0] * s.v[15]);let eq14_e1071_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq14_e1071_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq14_e1071_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq14_e1071_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq14_e1071_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq14_e1071_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq14_e1071_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq14_e1071_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq14_e1071_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq14_e1071_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq14_e1071_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq14_e1071_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq14_e1071_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq14_e1071_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq14_e1071_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq14_e1071_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq14_e1071_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq14_e1071_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq14_e1071_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq14_e1073: f64 = (eq14_e1071 * p.p32);let eq14_e1073_d_n0: f64 = (eq14_e1071_d_n0 * p.p32);let eq14_e1073_d_n1: f64 = (eq14_e1071_d_n1 * p.p32);let eq14_e1073_d_n2: f64 = (eq14_e1071_d_n2 * p.p32);let eq14_e1073_d_n3: f64 = (eq14_e1071_d_n3 * p.p32);let eq14_e1073_d_n4: f64 = (eq14_e1071_d_n4 * p.p32);let eq14_e1073_d_n5: f64 = (eq14_e1071_d_n5 * p.p32);let eq14_e1073_d_n6: f64 = (eq14_e1071_d_n6 * p.p32);let eq14_e1073_d_n7: f64 = (eq14_e1071_d_n7 * p.p32);let eq14_e1073_d_n8: f64 = (eq14_e1071_d_n8 * p.p32);let eq14_e1073_d_n9: f64 = (eq14_e1071_d_n9 * p.p32);let eq14_e1073_d_n10: f64 = (eq14_e1071_d_n10 * p.p32);let eq14_e1073_d_n11: f64 = (eq14_e1071_d_n11 * p.p32);let eq14_e1073_d_b0: f64 = (eq14_e1071_d_b0 * p.p32);let eq14_e1073_d_b1: f64 = (eq14_e1071_d_b1 * p.p32);let eq14_e1073_d_b2: f64 = (eq14_e1071_d_b2 * p.p32);let eq14_e1073_d_b3: f64 = (eq14_e1071_d_b3 * p.p32);let eq14_e1073_d_b4: f64 = (eq14_e1071_d_b4 * p.p32);let eq14_e1073_d_b5: f64 = (eq14_e1071_d_b5 * p.p32);let eq14_e1073_d_b6: f64 = (eq14_e1071_d_b6 * p.p32);let eq14_e1075: f64 = (eq14_e1073 * s.v[843]);let eq14_e1075_d_n0: f64 = ((eq14_e1073_d_n0 * s.v[843]) + (eq14_e1073 * s.dn[843][0]));let eq14_e1075_d_n1: f64 = ((eq14_e1073_d_n1 * s.v[843]) + (eq14_e1073 * s.dn[843][1]));let eq14_e1075_d_n2: f64 = ((eq14_e1073_d_n2 * s.v[843]) + (eq14_e1073 * s.dn[843][2]));let eq14_e1075_d_n3: f64 = ((eq14_e1073_d_n3 * s.v[843]) + (eq14_e1073 * s.dn[843][3]));let eq14_e1075_d_n4: f64 = ((eq14_e1073_d_n4 * s.v[843]) + (eq14_e1073 * s.dn[843][4]));let eq14_e1075_d_n5: f64 = ((eq14_e1073_d_n5 * s.v[843]) + (eq14_e1073 * s.dn[843][5]));let eq14_e1075_d_n6: f64 = ((eq14_e1073_d_n6 * s.v[843]) + (eq14_e1073 * s.dn[843][6]));let eq14_e1075_d_n7: f64 = ((eq14_e1073_d_n7 * s.v[843]) + (eq14_e1073 * s.dn[843][7]));let eq14_e1075_d_n8: f64 = ((eq14_e1073_d_n8 * s.v[843]) + (eq14_e1073 * s.dn[843][8]));let eq14_e1075_d_n9: f64 = ((eq14_e1073_d_n9 * s.v[843]) + (eq14_e1073 * s.dn[843][9]));let eq14_e1075_d_n10: f64 = ((eq14_e1073_d_n10 * s.v[843]) + (eq14_e1073 * s.dn[843][10]));let eq14_e1075_d_n11: f64 = ((eq14_e1073_d_n11 * s.v[843]) + (eq14_e1073 * s.dn[843][11]));let eq14_e1075_d_b0: f64 = ((eq14_e1073_d_b0 * s.v[843]) + (eq14_e1073 * s.db[843][0]));let eq14_e1075_d_b1: f64 = ((eq14_e1073_d_b1 * s.v[843]) + (eq14_e1073 * s.db[843][1]));let eq14_e1075_d_b2: f64 = ((eq14_e1073_d_b2 * s.v[843]) + (eq14_e1073 * s.db[843][2]));let eq14_e1075_d_b3: f64 = ((eq14_e1073_d_b3 * s.v[843]) + (eq14_e1073 * s.db[843][3]));
        let eq14_e1075_d_b4: f64 = ((eq14_e1073_d_b4 * s.v[843]) + (eq14_e1073 * s.db[843][4]));let eq14_e1075_d_b5: f64 = ((eq14_e1073_d_b5 * s.v[843]) + (eq14_e1073 * s.db[843][5]));let eq14_e1075_d_b6: f64 = ((eq14_e1073_d_b6 * s.v[843]) + (eq14_e1073 * s.db[843][6]));let eq14_value: f64 = eq14_e1075;let eq14_node_derivatives: [f64; 12] = [eq14_e1075_d_n0, eq14_e1075_d_n1, eq14_e1075_d_n2, eq14_e1075_d_n3, eq14_e1075_d_n4, eq14_e1075_d_n5, eq14_e1075_d_n6, eq14_e1075_d_n7, eq14_e1075_d_n8, eq14_e1075_d_n9, eq14_e1075_d_n10, eq14_e1075_d_n11];let eq14_branch_derivatives: [f64; 7] = [eq14_e1075_d_b0, eq14_e1075_d_b1, eq14_e1075_d_b2, eq14_e1075_d_b3, eq14_e1075_d_b4, eq14_e1075_d_b5, eq14_e1075_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_15(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv5 = ctx.node_voltage(nodes[5]);
        let (eq15_e1085, eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11, eq15_e1085_d_b0, eq15_e1085_d_b1, eq15_e1085_d_b2, eq15_e1085_d_b3, eq15_e1085_d_b4, eq15_e1085_d_b5, eq15_e1085_d_b6,) = {
    if s.b[2702] {
        let eq15_e1079: f64 = (s.v[15] * p.p32);let eq15_e1081: f64 = (eq15_e1079 * s.v[805]);let eq15_e1081_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][0]));let eq15_e1081_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][1]));let eq15_e1081_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][2]));let eq15_e1081_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][3]));let eq15_e1081_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][4]));let eq15_e1081_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][5]));let eq15_e1081_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][6]));let eq15_e1081_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][7]));let eq15_e1081_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][8]));let eq15_e1081_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][9]));let eq15_e1081_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][10]));let eq15_e1081_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[805]) + (eq15_e1079 * s.dn[805][11]));let eq15_e1081_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][0]));let eq15_e1081_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][1]));let eq15_e1081_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][2]));let eq15_e1081_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][3]));let eq15_e1081_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][4]));let eq15_e1081_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][5]));let eq15_e1081_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[805]) + (eq15_e1079 * s.db[805][6]));let eq15_e1083: f64 = (eq15_e1081 * (nv1 - nv5));let eq15_e1083_d_n0: f64 = (eq15_e1081_d_n0 * (nv1 - nv5));let eq15_e1083_d_n1: f64 = ((eq15_e1081_d_n1 * (nv1 - nv5)) + eq15_e1081);let eq15_e1083_d_n2: f64 = (eq15_e1081_d_n2 * (nv1 - nv5));let eq15_e1083_d_n3: f64 = (eq15_e1081_d_n3 * (nv1 - nv5));let eq15_e1083_d_n4: f64 = (eq15_e1081_d_n4 * (nv1 - nv5));let eq15_e1083_d_n5: f64 = ((eq15_e1081_d_n5 * (nv1 - nv5)) + (-eq15_e1081));let eq15_e1083_d_n6: f64 = (eq15_e1081_d_n6 * (nv1 - nv5));let eq15_e1083_d_n7: f64 = (eq15_e1081_d_n7 * (nv1 - nv5));let eq15_e1083_d_n8: f64 = (eq15_e1081_d_n8 * (nv1 - nv5));let eq15_e1083_d_n9: f64 = (eq15_e1081_d_n9 * (nv1 - nv5));let eq15_e1083_d_n10: f64 = (eq15_e1081_d_n10 * (nv1 - nv5));let eq15_e1083_d_n11: f64 = (eq15_e1081_d_n11 * (nv1 - nv5));let eq15_e1083_d_b0: f64 = (eq15_e1081_d_b0 * (nv1 - nv5));let eq15_e1083_d_b1: f64 = (eq15_e1081_d_b1 * (nv1 - nv5));let eq15_e1083_d_b2: f64 = (eq15_e1081_d_b2 * (nv1 - nv5));let eq15_e1083_d_b3: f64 = (eq15_e1081_d_b3 * (nv1 - nv5));let eq15_e1083_d_b4: f64 = (eq15_e1081_d_b4 * (nv1 - nv5));let eq15_e1083_d_b5: f64 = (eq15_e1081_d_b5 * (nv1 - nv5));let eq15_e1083_d_b6: f64 = (eq15_e1081_d_b6 * (nv1 - nv5));
        (eq15_e1083, eq15_e1083_d_n0, eq15_e1083_d_n1, eq15_e1083_d_n2, eq15_e1083_d_n3, eq15_e1083_d_n4, eq15_e1083_d_n5, eq15_e1083_d_n6, eq15_e1083_d_n7, eq15_e1083_d_n8, eq15_e1083_d_n9, eq15_e1083_d_n10, eq15_e1083_d_n11, eq15_e1083_d_b0, eq15_e1083_d_b1, eq15_e1083_d_b2, eq15_e1083_d_b3, eq15_e1083_d_b4, eq15_e1083_d_b5, eq15_e1083_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1085;let eq15_node_derivatives: [f64; 12] = [eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11];let eq15_branch_derivatives: [f64; 7] = [eq15_e1085_d_b0, eq15_e1085_d_b1, eq15_e1085_d_b2, eq15_e1085_d_b3, eq15_e1085_d_b4, eq15_e1085_d_b5, eq15_e1085_d_b6];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1100,) = {
    if (!s.b[2702]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1100;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_16(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6,) = {
    if s.b[2703] {
        let eq18_e1104: f64 = (s.v[15] * p.p32);let eq18_e1106: f64 = (eq18_e1104 * s.v[806]);let eq18_e1106_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][0]));let eq18_e1106_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][1]));let eq18_e1106_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][2]));let eq18_e1106_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][3]));let eq18_e1106_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][4]));let eq18_e1106_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][5]));let eq18_e1106_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][6]));let eq18_e1106_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][7]));let eq18_e1106_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][8]));let eq18_e1106_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][9]));let eq18_e1106_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][10]));let eq18_e1106_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[806]) + (eq18_e1104 * s.dn[806][11]));let eq18_e1106_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][0]));let eq18_e1106_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][1]));let eq18_e1106_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][2]));let eq18_e1106_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][3]));let eq18_e1106_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][4]));let eq18_e1106_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][5]));let eq18_e1106_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[806]) + (eq18_e1104 * s.db[806][6]));let eq18_e1108: f64 = (eq18_e1106 * (nv2 - nv6));let eq18_e1108_d_n0: f64 = (eq18_e1106_d_n0 * (nv2 - nv6));let eq18_e1108_d_n1: f64 = (eq18_e1106_d_n1 * (nv2 - nv6));let eq18_e1108_d_n2: f64 = ((eq18_e1106_d_n2 * (nv2 - nv6)) + eq18_e1106);let eq18_e1108_d_n3: f64 = (eq18_e1106_d_n3 * (nv2 - nv6));let eq18_e1108_d_n4: f64 = (eq18_e1106_d_n4 * (nv2 - nv6));let eq18_e1108_d_n5: f64 = (eq18_e1106_d_n5 * (nv2 - nv6));let eq18_e1108_d_n6: f64 = ((eq18_e1106_d_n6 * (nv2 - nv6)) + (-eq18_e1106));let eq18_e1108_d_n7: f64 = (eq18_e1106_d_n7 * (nv2 - nv6));let eq18_e1108_d_n8: f64 = (eq18_e1106_d_n8 * (nv2 - nv6));let eq18_e1108_d_n9: f64 = (eq18_e1106_d_n9 * (nv2 - nv6));let eq18_e1108_d_n10: f64 = (eq18_e1106_d_n10 * (nv2 - nv6));let eq18_e1108_d_n11: f64 = (eq18_e1106_d_n11 * (nv2 - nv6));let eq18_e1108_d_b0: f64 = (eq18_e1106_d_b0 * (nv2 - nv6));let eq18_e1108_d_b1: f64 = (eq18_e1106_d_b1 * (nv2 - nv6));let eq18_e1108_d_b2: f64 = (eq18_e1106_d_b2 * (nv2 - nv6));let eq18_e1108_d_b3: f64 = (eq18_e1106_d_b3 * (nv2 - nv6));let eq18_e1108_d_b4: f64 = (eq18_e1106_d_b4 * (nv2 - nv6));let eq18_e1108_d_b5: f64 = (eq18_e1106_d_b5 * (nv2 - nv6));let eq18_e1108_d_b6: f64 = (eq18_e1106_d_b6 * (nv2 - nv6));
        (eq18_e1108, eq18_e1108_d_n0, eq18_e1108_d_n1, eq18_e1108_d_n2, eq18_e1108_d_n3, eq18_e1108_d_n4, eq18_e1108_d_n5, eq18_e1108_d_n6, eq18_e1108_d_n7, eq18_e1108_d_n8, eq18_e1108_d_n9, eq18_e1108_d_n10, eq18_e1108_d_n11, eq18_e1108_d_b0, eq18_e1108_d_b1, eq18_e1108_d_b2, eq18_e1108_d_b3, eq18_e1108_d_b4, eq18_e1108_d_b5, eq18_e1108_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1110;let eq18_node_derivatives: [f64; 12] = [eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11];let eq18_branch_derivatives: [f64; 7] = [eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1125,) = {
    if (!s.b[2703]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq21_e1135, eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11, eq21_e1135_d_b0, eq21_e1135_d_b1, eq21_e1135_d_b2, eq21_e1135_d_b3, eq21_e1135_d_b4, eq21_e1135_d_b5, eq21_e1135_d_b6,) = {
    if s.b[2704] {
        let eq21_e1129: f64 = (s.v[15] * p.p32);let eq21_e1131: f64 = (eq21_e1129 * s.v[807]);let eq21_e1131_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][0]));let eq21_e1131_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][1]));let eq21_e1131_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][2]));let eq21_e1131_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][3]));let eq21_e1131_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][4]));let eq21_e1131_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][5]));let eq21_e1131_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][6]));let eq21_e1131_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][7]));let eq21_e1131_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][8]));let eq21_e1131_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][9]));let eq21_e1131_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][10]));let eq21_e1131_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[807]) + (eq21_e1129 * s.dn[807][11]));let eq21_e1131_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][0]));let eq21_e1131_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][1]));let eq21_e1131_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][2]));let eq21_e1131_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][3]));let eq21_e1131_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][4]));let eq21_e1131_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][5]));let eq21_e1131_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[807]) + (eq21_e1129 * s.db[807][6]));let eq21_e1133: f64 = (eq21_e1131 * (nv0 - nv7));let eq21_e1133_d_n0: f64 = ((eq21_e1131_d_n0 * (nv0 - nv7)) + eq21_e1131);let eq21_e1133_d_n1: f64 = (eq21_e1131_d_n1 * (nv0 - nv7));let eq21_e1133_d_n2: f64 = (eq21_e1131_d_n2 * (nv0 - nv7));let eq21_e1133_d_n3: f64 = (eq21_e1131_d_n3 * (nv0 - nv7));let eq21_e1133_d_n4: f64 = (eq21_e1131_d_n4 * (nv0 - nv7));let eq21_e1133_d_n5: f64 = (eq21_e1131_d_n5 * (nv0 - nv7));let eq21_e1133_d_n6: f64 = (eq21_e1131_d_n6 * (nv0 - nv7));let eq21_e1133_d_n7: f64 = ((eq21_e1131_d_n7 * (nv0 - nv7)) + (-eq21_e1131));let eq21_e1133_d_n8: f64 = (eq21_e1131_d_n8 * (nv0 - nv7));let eq21_e1133_d_n9: f64 = (eq21_e1131_d_n9 * (nv0 - nv7));let eq21_e1133_d_n10: f64 = (eq21_e1131_d_n10 * (nv0 - nv7));let eq21_e1133_d_n11: f64 = (eq21_e1131_d_n11 * (nv0 - nv7));let eq21_e1133_d_b0: f64 = (eq21_e1131_d_b0 * (nv0 - nv7));let eq21_e1133_d_b1: f64 = (eq21_e1131_d_b1 * (nv0 - nv7));let eq21_e1133_d_b2: f64 = (eq21_e1131_d_b2 * (nv0 - nv7));let eq21_e1133_d_b3: f64 = (eq21_e1131_d_b3 * (nv0 - nv7));let eq21_e1133_d_b4: f64 = (eq21_e1131_d_b4 * (nv0 - nv7));let eq21_e1133_d_b5: f64 = (eq21_e1131_d_b5 * (nv0 - nv7));let eq21_e1133_d_b6: f64 = (eq21_e1131_d_b6 * (nv0 - nv7));
        (eq21_e1133, eq21_e1133_d_n0, eq21_e1133_d_n1, eq21_e1133_d_n2, eq21_e1133_d_n3, eq21_e1133_d_n4, eq21_e1133_d_n5, eq21_e1133_d_n6, eq21_e1133_d_n7, eq21_e1133_d_n8, eq21_e1133_d_n9, eq21_e1133_d_n10, eq21_e1133_d_n11, eq21_e1133_d_b0, eq21_e1133_d_b1, eq21_e1133_d_b2, eq21_e1133_d_b3, eq21_e1133_d_b4, eq21_e1133_d_b5, eq21_e1133_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1135;let eq21_node_derivatives: [f64; 12] = [eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11];let eq21_branch_derivatives: [f64; 7] = [eq21_e1135_d_b0, eq21_e1135_d_b1, eq21_e1135_d_b2, eq21_e1135_d_b3, eq21_e1135_d_b4, eq21_e1135_d_b5, eq21_e1135_d_b6];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1150,) = {
    if (!s.b[2704]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1150;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_18(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq24_e1160, eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11, eq24_e1160_d_b0, eq24_e1160_d_b1, eq24_e1160_d_b2, eq24_e1160_d_b3, eq24_e1160_d_b4, eq24_e1160_d_b5, eq24_e1160_d_b6,) = {
    if s.b[2705] {
        let eq24_e1154: f64 = (s.v[15] * p.p32);let eq24_e1156: f64 = (eq24_e1154 * s.v[808]);let eq24_e1156_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][0]));let eq24_e1156_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][1]));let eq24_e1156_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][2]));let eq24_e1156_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][3]));let eq24_e1156_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][4]));let eq24_e1156_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][5]));let eq24_e1156_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][6]));let eq24_e1156_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][7]));let eq24_e1156_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][8]));let eq24_e1156_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][9]));let eq24_e1156_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][10]));let eq24_e1156_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[808]) + (eq24_e1154 * s.dn[808][11]));let eq24_e1156_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][0]));let eq24_e1156_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][1]));let eq24_e1156_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][2]));let eq24_e1156_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][3]));let eq24_e1156_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][4]));let eq24_e1156_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][5]));let eq24_e1156_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[808]) + (eq24_e1154 * s.db[808][6]));let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));let eq24_e1158_d_n0: f64 = (eq24_e1156_d_n0 * (nv8 - nv9));let eq24_e1158_d_n1: f64 = (eq24_e1156_d_n1 * (nv8 - nv9));let eq24_e1158_d_n2: f64 = (eq24_e1156_d_n2 * (nv8 - nv9));let eq24_e1158_d_n3: f64 = (eq24_e1156_d_n3 * (nv8 - nv9));let eq24_e1158_d_n4: f64 = (eq24_e1156_d_n4 * (nv8 - nv9));let eq24_e1158_d_n5: f64 = (eq24_e1156_d_n5 * (nv8 - nv9));let eq24_e1158_d_n6: f64 = (eq24_e1156_d_n6 * (nv8 - nv9));let eq24_e1158_d_n7: f64 = (eq24_e1156_d_n7 * (nv8 - nv9));let eq24_e1158_d_n8: f64 = ((eq24_e1156_d_n8 * (nv8 - nv9)) + eq24_e1156);let eq24_e1158_d_n9: f64 = ((eq24_e1156_d_n9 * (nv8 - nv9)) + (-eq24_e1156));let eq24_e1158_d_n10: f64 = (eq24_e1156_d_n10 * (nv8 - nv9));let eq24_e1158_d_n11: f64 = (eq24_e1156_d_n11 * (nv8 - nv9));let eq24_e1158_d_b0: f64 = (eq24_e1156_d_b0 * (nv8 - nv9));let eq24_e1158_d_b1: f64 = (eq24_e1156_d_b1 * (nv8 - nv9));let eq24_e1158_d_b2: f64 = (eq24_e1156_d_b2 * (nv8 - nv9));let eq24_e1158_d_b3: f64 = (eq24_e1156_d_b3 * (nv8 - nv9));let eq24_e1158_d_b4: f64 = (eq24_e1156_d_b4 * (nv8 - nv9));let eq24_e1158_d_b5: f64 = (eq24_e1156_d_b5 * (nv8 - nv9));let eq24_e1158_d_b6: f64 = (eq24_e1156_d_b6 * (nv8 - nv9));
        (eq24_e1158, eq24_e1158_d_n0, eq24_e1158_d_n1, eq24_e1158_d_n2, eq24_e1158_d_n3, eq24_e1158_d_n4, eq24_e1158_d_n5, eq24_e1158_d_n6, eq24_e1158_d_n7, eq24_e1158_d_n8, eq24_e1158_d_n9, eq24_e1158_d_n10, eq24_e1158_d_n11, eq24_e1158_d_b0, eq24_e1158_d_b1, eq24_e1158_d_b2, eq24_e1158_d_b3, eq24_e1158_d_b4, eq24_e1158_d_b5, eq24_e1158_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;let eq24_node_derivatives: [f64; 12] = [eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11];let eq24_branch_derivatives: [f64; 7] = [eq24_e1160_d_b0, eq24_e1160_d_b1, eq24_e1160_d_b2, eq24_e1160_d_b3, eq24_e1160_d_b4, eq24_e1160_d_b5, eq24_e1160_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1175,) = {
    if (!s.b[2705]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_19(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq27_e1185, eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11, eq27_e1185_d_b0, eq27_e1185_d_b1, eq27_e1185_d_b2, eq27_e1185_d_b3, eq27_e1185_d_b4, eq27_e1185_d_b5, eq27_e1185_d_b6,) = {
    if s.b[2706] {
        let eq27_e1179: f64 = (s.v[15] * p.p32);let eq27_e1181: f64 = (eq27_e1179 * s.v[809]);let eq27_e1181_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][0]));let eq27_e1181_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][1]));let eq27_e1181_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][2]));let eq27_e1181_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][3]));let eq27_e1181_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][4]));let eq27_e1181_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][5]));let eq27_e1181_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][6]));let eq27_e1181_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][7]));let eq27_e1181_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][8]));let eq27_e1181_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][9]));let eq27_e1181_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][10]));let eq27_e1181_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[809]) + (eq27_e1179 * s.dn[809][11]));let eq27_e1181_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][0]));let eq27_e1181_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][1]));let eq27_e1181_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][2]));let eq27_e1181_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][3]));let eq27_e1181_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][4]));let eq27_e1181_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][5]));let eq27_e1181_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[809]) + (eq27_e1179 * s.db[809][6]));let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));let eq27_e1183_d_n0: f64 = (eq27_e1181_d_n0 * (nv10 - nv9));let eq27_e1183_d_n1: f64 = (eq27_e1181_d_n1 * (nv10 - nv9));let eq27_e1183_d_n2: f64 = (eq27_e1181_d_n2 * (nv10 - nv9));let eq27_e1183_d_n3: f64 = (eq27_e1181_d_n3 * (nv10 - nv9));let eq27_e1183_d_n4: f64 = (eq27_e1181_d_n4 * (nv10 - nv9));let eq27_e1183_d_n5: f64 = (eq27_e1181_d_n5 * (nv10 - nv9));let eq27_e1183_d_n6: f64 = (eq27_e1181_d_n6 * (nv10 - nv9));let eq27_e1183_d_n7: f64 = (eq27_e1181_d_n7 * (nv10 - nv9));let eq27_e1183_d_n8: f64 = (eq27_e1181_d_n8 * (nv10 - nv9));let eq27_e1183_d_n9: f64 = ((eq27_e1181_d_n9 * (nv10 - nv9)) + (-eq27_e1181));let eq27_e1183_d_n10: f64 = ((eq27_e1181_d_n10 * (nv10 - nv9)) + eq27_e1181);let eq27_e1183_d_n11: f64 = (eq27_e1181_d_n11 * (nv10 - nv9));let eq27_e1183_d_b0: f64 = (eq27_e1181_d_b0 * (nv10 - nv9));let eq27_e1183_d_b1: f64 = (eq27_e1181_d_b1 * (nv10 - nv9));let eq27_e1183_d_b2: f64 = (eq27_e1181_d_b2 * (nv10 - nv9));let eq27_e1183_d_b3: f64 = (eq27_e1181_d_b3 * (nv10 - nv9));let eq27_e1183_d_b4: f64 = (eq27_e1181_d_b4 * (nv10 - nv9));let eq27_e1183_d_b5: f64 = (eq27_e1181_d_b5 * (nv10 - nv9));let eq27_e1183_d_b6: f64 = (eq27_e1181_d_b6 * (nv10 - nv9));
        (eq27_e1183, eq27_e1183_d_n0, eq27_e1183_d_n1, eq27_e1183_d_n2, eq27_e1183_d_n3, eq27_e1183_d_n4, eq27_e1183_d_n5, eq27_e1183_d_n6, eq27_e1183_d_n7, eq27_e1183_d_n8, eq27_e1183_d_n9, eq27_e1183_d_n10, eq27_e1183_d_n11, eq27_e1183_d_b0, eq27_e1183_d_b1, eq27_e1183_d_b2, eq27_e1183_d_b3, eq27_e1183_d_b4, eq27_e1183_d_b5, eq27_e1183_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;let eq27_node_derivatives: [f64; 12] = [eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11];let eq27_branch_derivatives: [f64; 7] = [eq27_e1185_d_b0, eq27_e1185_d_b1, eq27_e1185_d_b2, eq27_e1185_d_b3, eq27_e1185_d_b4, eq27_e1185_d_b5, eq27_e1185_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1200,) = {
    if (!s.b[2706]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_20(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq30_e1210, eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11, eq30_e1210_d_b0, eq30_e1210_d_b1, eq30_e1210_d_b2, eq30_e1210_d_b3, eq30_e1210_d_b4, eq30_e1210_d_b5, eq30_e1210_d_b6,) = {
    if s.b[2707] {
        let eq30_e1204: f64 = (s.v[15] * p.p32);let eq30_e1206: f64 = (eq30_e1204 * s.v[810]);let eq30_e1206_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][0]));let eq30_e1206_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][1]));let eq30_e1206_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][2]));let eq30_e1206_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][3]));let eq30_e1206_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][4]));let eq30_e1206_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][5]));let eq30_e1206_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][6]));let eq30_e1206_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][7]));let eq30_e1206_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][8]));let eq30_e1206_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][9]));let eq30_e1206_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][10]));let eq30_e1206_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[810]) + (eq30_e1204 * s.dn[810][11]));let eq30_e1206_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][0]));let eq30_e1206_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][1]));let eq30_e1206_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][2]));let eq30_e1206_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][3]));let eq30_e1206_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][4]));let eq30_e1206_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][5]));let eq30_e1206_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[810]) + (eq30_e1204 * s.db[810][6]));let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));let eq30_e1208_d_n0: f64 = (eq30_e1206_d_n0 * (nv11 - nv9));let eq30_e1208_d_n1: f64 = (eq30_e1206_d_n1 * (nv11 - nv9));let eq30_e1208_d_n2: f64 = (eq30_e1206_d_n2 * (nv11 - nv9));let eq30_e1208_d_n3: f64 = (eq30_e1206_d_n3 * (nv11 - nv9));let eq30_e1208_d_n4: f64 = (eq30_e1206_d_n4 * (nv11 - nv9));let eq30_e1208_d_n5: f64 = (eq30_e1206_d_n5 * (nv11 - nv9));let eq30_e1208_d_n6: f64 = (eq30_e1206_d_n6 * (nv11 - nv9));let eq30_e1208_d_n7: f64 = (eq30_e1206_d_n7 * (nv11 - nv9));let eq30_e1208_d_n8: f64 = (eq30_e1206_d_n8 * (nv11 - nv9));let eq30_e1208_d_n9: f64 = ((eq30_e1206_d_n9 * (nv11 - nv9)) + (-eq30_e1206));let eq30_e1208_d_n10: f64 = (eq30_e1206_d_n10 * (nv11 - nv9));let eq30_e1208_d_n11: f64 = ((eq30_e1206_d_n11 * (nv11 - nv9)) + eq30_e1206);let eq30_e1208_d_b0: f64 = (eq30_e1206_d_b0 * (nv11 - nv9));let eq30_e1208_d_b1: f64 = (eq30_e1206_d_b1 * (nv11 - nv9));let eq30_e1208_d_b2: f64 = (eq30_e1206_d_b2 * (nv11 - nv9));let eq30_e1208_d_b3: f64 = (eq30_e1206_d_b3 * (nv11 - nv9));let eq30_e1208_d_b4: f64 = (eq30_e1206_d_b4 * (nv11 - nv9));let eq30_e1208_d_b5: f64 = (eq30_e1206_d_b5 * (nv11 - nv9));let eq30_e1208_d_b6: f64 = (eq30_e1206_d_b6 * (nv11 - nv9));
        (eq30_e1208, eq30_e1208_d_n0, eq30_e1208_d_n1, eq30_e1208_d_n2, eq30_e1208_d_n3, eq30_e1208_d_n4, eq30_e1208_d_n5, eq30_e1208_d_n6, eq30_e1208_d_n7, eq30_e1208_d_n8, eq30_e1208_d_n9, eq30_e1208_d_n10, eq30_e1208_d_n11, eq30_e1208_d_b0, eq30_e1208_d_b1, eq30_e1208_d_b2, eq30_e1208_d_b3, eq30_e1208_d_b4, eq30_e1208_d_b5, eq30_e1208_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;let eq30_node_derivatives: [f64; 12] = [eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11];let eq30_branch_derivatives: [f64; 7] = [eq30_e1210_d_b0, eq30_e1210_d_b1, eq30_e1210_d_b2, eq30_e1210_d_b3, eq30_e1210_d_b4, eq30_e1210_d_b5, eq30_e1210_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1225,) = {
    if (!s.b[2707]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_21(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e1235, eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11, eq33_e1235_d_b0, eq33_e1235_d_b1, eq33_e1235_d_b2, eq33_e1235_d_b3, eq33_e1235_d_b4, eq33_e1235_d_b5, eq33_e1235_d_b6,) = {
    if s.b[2708] {
        let eq33_e1229: f64 = (s.v[15] * p.p32);let eq33_e1231: f64 = (eq33_e1229 * s.v[811]);let eq33_e1231_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][0]));let eq33_e1231_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][1]));let eq33_e1231_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][2]));let eq33_e1231_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][3]));let eq33_e1231_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][4]));let eq33_e1231_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][5]));let eq33_e1231_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][6]));let eq33_e1231_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][7]));let eq33_e1231_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][8]));let eq33_e1231_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][9]));let eq33_e1231_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][10]));let eq33_e1231_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[811]) + (eq33_e1229 * s.dn[811][11]));let eq33_e1231_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][0]));let eq33_e1231_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][1]));let eq33_e1231_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][2]));let eq33_e1231_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][3]));let eq33_e1231_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][4]));let eq33_e1231_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][5]));let eq33_e1231_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[811]) + (eq33_e1229 * s.db[811][6]));let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));let eq33_e1233_d_n0: f64 = (eq33_e1231_d_n0 * (nv3 - nv9));let eq33_e1233_d_n1: f64 = (eq33_e1231_d_n1 * (nv3 - nv9));let eq33_e1233_d_n2: f64 = (eq33_e1231_d_n2 * (nv3 - nv9));let eq33_e1233_d_n3: f64 = ((eq33_e1231_d_n3 * (nv3 - nv9)) + eq33_e1231);let eq33_e1233_d_n4: f64 = (eq33_e1231_d_n4 * (nv3 - nv9));let eq33_e1233_d_n5: f64 = (eq33_e1231_d_n5 * (nv3 - nv9));let eq33_e1233_d_n6: f64 = (eq33_e1231_d_n6 * (nv3 - nv9));let eq33_e1233_d_n7: f64 = (eq33_e1231_d_n7 * (nv3 - nv9));let eq33_e1233_d_n8: f64 = (eq33_e1231_d_n8 * (nv3 - nv9));let eq33_e1233_d_n9: f64 = ((eq33_e1231_d_n9 * (nv3 - nv9)) + (-eq33_e1231));let eq33_e1233_d_n10: f64 = (eq33_e1231_d_n10 * (nv3 - nv9));let eq33_e1233_d_n11: f64 = (eq33_e1231_d_n11 * (nv3 - nv9));let eq33_e1233_d_b0: f64 = (eq33_e1231_d_b0 * (nv3 - nv9));let eq33_e1233_d_b1: f64 = (eq33_e1231_d_b1 * (nv3 - nv9));let eq33_e1233_d_b2: f64 = (eq33_e1231_d_b2 * (nv3 - nv9));let eq33_e1233_d_b3: f64 = (eq33_e1231_d_b3 * (nv3 - nv9));let eq33_e1233_d_b4: f64 = (eq33_e1231_d_b4 * (nv3 - nv9));let eq33_e1233_d_b5: f64 = (eq33_e1231_d_b5 * (nv3 - nv9));let eq33_e1233_d_b6: f64 = (eq33_e1231_d_b6 * (nv3 - nv9));
        (eq33_e1233, eq33_e1233_d_n0, eq33_e1233_d_n1, eq33_e1233_d_n2, eq33_e1233_d_n3, eq33_e1233_d_n4, eq33_e1233_d_n5, eq33_e1233_d_n6, eq33_e1233_d_n7, eq33_e1233_d_n8, eq33_e1233_d_n9, eq33_e1233_d_n10, eq33_e1233_d_n11, eq33_e1233_d_b0, eq33_e1233_d_b1, eq33_e1233_d_b2, eq33_e1233_d_b3, eq33_e1233_d_b4, eq33_e1233_d_b5, eq33_e1233_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;let eq33_node_derivatives: [f64; 12] = [eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11];let eq33_branch_derivatives: [f64; 7] = [eq33_e1235_d_b0, eq33_e1235_d_b1, eq33_e1235_d_b2, eq33_e1235_d_b3, eq33_e1235_d_b4, eq33_e1235_d_b5, eq33_e1235_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq35_e1250,) = {
    if (!s.b[2708]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );let eq36_e1253: f64 = (p.p32 * s.v[872]);let eq36_e1255: f64 = (eq36_e1253 * (nv7 - nv8));let eq36_value: f64 = eq36_e1255;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(8),
            multiplicity * (eq36_value),
            7,
            multiplicity * (eq36_e1253),
            8,
            multiplicity * ((-eq36_e1253)),
        );let eq37_e1258: f64 = (p.p32 * s.v[872]);let eq37_e1260: f64 = (eq37_e1258 * (nv6 - nv8));let eq37_value: f64 = eq37_e1260;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(8),
            multiplicity * (eq37_value),
            6,
            multiplicity * (eq37_e1258),
            8,
            multiplicity * ((-eq37_e1258)),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_22(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq38_e1263: f64 = (s.v[0] * s.v[15]);let eq38_e1263_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq38_e1263_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq38_e1263_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq38_e1263_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq38_e1263_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq38_e1263_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq38_e1263_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq38_e1263_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq38_e1263_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq38_e1263_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq38_e1263_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq38_e1263_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq38_e1263_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq38_e1263_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq38_e1263_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq38_e1263_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq38_e1263_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq38_e1263_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq38_e1263_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq38_e1265: f64 = (eq38_e1263 * p.p33);let eq38_e1265_d_n0: f64 = (eq38_e1263_d_n0 * p.p33);let eq38_e1265_d_n1: f64 = (eq38_e1263_d_n1 * p.p33);let eq38_e1265_d_n2: f64 = (eq38_e1263_d_n2 * p.p33);let eq38_e1265_d_n3: f64 = (eq38_e1263_d_n3 * p.p33);let eq38_e1265_d_n4: f64 = (eq38_e1263_d_n4 * p.p33);let eq38_e1265_d_n5: f64 = (eq38_e1263_d_n5 * p.p33);let eq38_e1265_d_n6: f64 = (eq38_e1263_d_n6 * p.p33);let eq38_e1265_d_n7: f64 = (eq38_e1263_d_n7 * p.p33);let eq38_e1265_d_n8: f64 = (eq38_e1263_d_n8 * p.p33);let eq38_e1265_d_n9: f64 = (eq38_e1263_d_n9 * p.p33);let eq38_e1265_d_n10: f64 = (eq38_e1263_d_n10 * p.p33);let eq38_e1265_d_n11: f64 = (eq38_e1263_d_n11 * p.p33);let eq38_e1265_d_b0: f64 = (eq38_e1263_d_b0 * p.p33);let eq38_e1265_d_b1: f64 = (eq38_e1263_d_b1 * p.p33);let eq38_e1265_d_b2: f64 = (eq38_e1263_d_b2 * p.p33);let eq38_e1265_d_b3: f64 = (eq38_e1263_d_b3 * p.p33);let eq38_e1265_d_b4: f64 = (eq38_e1263_d_b4 * p.p33);let eq38_e1265_d_b5: f64 = (eq38_e1263_d_b5 * p.p33);let eq38_e1265_d_b6: f64 = (eq38_e1263_d_b6 * p.p33);let eq38_e1267: f64 = (eq38_e1265 * s.v[845]);let eq38_e1267_d_n0: f64 = ((eq38_e1265_d_n0 * s.v[845]) + (eq38_e1265 * s.dn[845][0]));let eq38_e1267_d_n1: f64 = ((eq38_e1265_d_n1 * s.v[845]) + (eq38_e1265 * s.dn[845][1]));let eq38_e1267_d_n2: f64 = ((eq38_e1265_d_n2 * s.v[845]) + (eq38_e1265 * s.dn[845][2]));let eq38_e1267_d_n3: f64 = ((eq38_e1265_d_n3 * s.v[845]) + (eq38_e1265 * s.dn[845][3]));let eq38_e1267_d_n4: f64 = ((eq38_e1265_d_n4 * s.v[845]) + (eq38_e1265 * s.dn[845][4]));let eq38_e1267_d_n5: f64 = ((eq38_e1265_d_n5 * s.v[845]) + (eq38_e1265 * s.dn[845][5]));let eq38_e1267_d_n6: f64 = ((eq38_e1265_d_n6 * s.v[845]) + (eq38_e1265 * s.dn[845][6]));let eq38_e1267_d_n7: f64 = ((eq38_e1265_d_n7 * s.v[845]) + (eq38_e1265 * s.dn[845][7]));let eq38_e1267_d_n8: f64 = ((eq38_e1265_d_n8 * s.v[845]) + (eq38_e1265 * s.dn[845][8]));let eq38_e1267_d_n9: f64 = ((eq38_e1265_d_n9 * s.v[845]) + (eq38_e1265 * s.dn[845][9]));let eq38_e1267_d_n10: f64 = ((eq38_e1265_d_n10 * s.v[845]) + (eq38_e1265 * s.dn[845][10]));let eq38_e1267_d_n11: f64 = ((eq38_e1265_d_n11 * s.v[845]) + (eq38_e1265 * s.dn[845][11]));let eq38_e1267_d_b0: f64 = ((eq38_e1265_d_b0 * s.v[845]) + (eq38_e1265 * s.db[845][0]));let eq38_e1267_d_b1: f64 = ((eq38_e1265_d_b1 * s.v[845]) + (eq38_e1265 * s.db[845][1]));let eq38_e1267_d_b2: f64 = ((eq38_e1265_d_b2 * s.v[845]) + (eq38_e1265 * s.db[845][2]));let eq38_e1267_d_b3: f64 = ((eq38_e1265_d_b3 * s.v[845]) + (eq38_e1265 * s.db[845][3]));
        let eq38_e1267_d_b4: f64 = ((eq38_e1265_d_b4 * s.v[845]) + (eq38_e1265 * s.db[845][4]));let eq38_e1267_d_b5: f64 = ((eq38_e1265_d_b5 * s.v[845]) + (eq38_e1265 * s.db[845][5]));let eq38_e1267_d_b6: f64 = ((eq38_e1265_d_b6 * s.v[845]) + (eq38_e1265 * s.db[845][6]));let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq38_e1267);let eq38_value: f64 = eq38_e1268;let eq38_node_derivatives: [f64; 12] = [(eq38_e1267_d_n0 * ddt_scale), (eq38_e1267_d_n1 * ddt_scale), (eq38_e1267_d_n2 * ddt_scale), (eq38_e1267_d_n3 * ddt_scale), (eq38_e1267_d_n4 * ddt_scale), (eq38_e1267_d_n5 * ddt_scale), (eq38_e1267_d_n6 * ddt_scale), (eq38_e1267_d_n7 * ddt_scale), (eq38_e1267_d_n8 * ddt_scale), (eq38_e1267_d_n9 * ddt_scale), (eq38_e1267_d_n10 * ddt_scale), (eq38_e1267_d_n11 * ddt_scale)];let eq38_branch_derivatives: [f64; 7] = [(eq38_e1267_d_b0 * ddt_scale), (eq38_e1267_d_b1 * ddt_scale), (eq38_e1267_d_b2 * ddt_scale), (eq38_e1267_d_b3 * ddt_scale), (eq38_e1267_d_b4 * ddt_scale), (eq38_e1267_d_b5 * ddt_scale), (eq38_e1267_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_23(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq39_e1271: f64 = (s.v[0] * s.v[15]);let eq39_e1271_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq39_e1271_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq39_e1271_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq39_e1271_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq39_e1271_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq39_e1271_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq39_e1271_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq39_e1271_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq39_e1271_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq39_e1271_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq39_e1271_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq39_e1271_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq39_e1271_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq39_e1271_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq39_e1271_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq39_e1271_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq39_e1271_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq39_e1271_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq39_e1271_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq39_e1273: f64 = (eq39_e1271 * p.p33);let eq39_e1273_d_n0: f64 = (eq39_e1271_d_n0 * p.p33);let eq39_e1273_d_n1: f64 = (eq39_e1271_d_n1 * p.p33);let eq39_e1273_d_n2: f64 = (eq39_e1271_d_n2 * p.p33);let eq39_e1273_d_n3: f64 = (eq39_e1271_d_n3 * p.p33);let eq39_e1273_d_n4: f64 = (eq39_e1271_d_n4 * p.p33);let eq39_e1273_d_n5: f64 = (eq39_e1271_d_n5 * p.p33);let eq39_e1273_d_n6: f64 = (eq39_e1271_d_n6 * p.p33);let eq39_e1273_d_n7: f64 = (eq39_e1271_d_n7 * p.p33);let eq39_e1273_d_n8: f64 = (eq39_e1271_d_n8 * p.p33);let eq39_e1273_d_n9: f64 = (eq39_e1271_d_n9 * p.p33);let eq39_e1273_d_n10: f64 = (eq39_e1271_d_n10 * p.p33);let eq39_e1273_d_n11: f64 = (eq39_e1271_d_n11 * p.p33);let eq39_e1273_d_b0: f64 = (eq39_e1271_d_b0 * p.p33);let eq39_e1273_d_b1: f64 = (eq39_e1271_d_b1 * p.p33);let eq39_e1273_d_b2: f64 = (eq39_e1271_d_b2 * p.p33);let eq39_e1273_d_b3: f64 = (eq39_e1271_d_b3 * p.p33);let eq39_e1273_d_b4: f64 = (eq39_e1271_d_b4 * p.p33);let eq39_e1273_d_b5: f64 = (eq39_e1271_d_b5 * p.p33);let eq39_e1273_d_b6: f64 = (eq39_e1271_d_b6 * p.p33);let eq39_e1275: f64 = (eq39_e1273 * s.v[846]);let eq39_e1275_d_n0: f64 = ((eq39_e1273_d_n0 * s.v[846]) + (eq39_e1273 * s.dn[846][0]));let eq39_e1275_d_n1: f64 = ((eq39_e1273_d_n1 * s.v[846]) + (eq39_e1273 * s.dn[846][1]));let eq39_e1275_d_n2: f64 = ((eq39_e1273_d_n2 * s.v[846]) + (eq39_e1273 * s.dn[846][2]));let eq39_e1275_d_n3: f64 = ((eq39_e1273_d_n3 * s.v[846]) + (eq39_e1273 * s.dn[846][3]));let eq39_e1275_d_n4: f64 = ((eq39_e1273_d_n4 * s.v[846]) + (eq39_e1273 * s.dn[846][4]));let eq39_e1275_d_n5: f64 = ((eq39_e1273_d_n5 * s.v[846]) + (eq39_e1273 * s.dn[846][5]));let eq39_e1275_d_n6: f64 = ((eq39_e1273_d_n6 * s.v[846]) + (eq39_e1273 * s.dn[846][6]));let eq39_e1275_d_n7: f64 = ((eq39_e1273_d_n7 * s.v[846]) + (eq39_e1273 * s.dn[846][7]));let eq39_e1275_d_n8: f64 = ((eq39_e1273_d_n8 * s.v[846]) + (eq39_e1273 * s.dn[846][8]));let eq39_e1275_d_n9: f64 = ((eq39_e1273_d_n9 * s.v[846]) + (eq39_e1273 * s.dn[846][9]));let eq39_e1275_d_n10: f64 = ((eq39_e1273_d_n10 * s.v[846]) + (eq39_e1273 * s.dn[846][10]));let eq39_e1275_d_n11: f64 = ((eq39_e1273_d_n11 * s.v[846]) + (eq39_e1273 * s.dn[846][11]));let eq39_e1275_d_b0: f64 = ((eq39_e1273_d_b0 * s.v[846]) + (eq39_e1273 * s.db[846][0]));let eq39_e1275_d_b1: f64 = ((eq39_e1273_d_b1 * s.v[846]) + (eq39_e1273 * s.db[846][1]));let eq39_e1275_d_b2: f64 = ((eq39_e1273_d_b2 * s.v[846]) + (eq39_e1273 * s.db[846][2]));let eq39_e1275_d_b3: f64 = ((eq39_e1273_d_b3 * s.v[846]) + (eq39_e1273 * s.db[846][3]));
        let eq39_e1275_d_b4: f64 = ((eq39_e1273_d_b4 * s.v[846]) + (eq39_e1273 * s.db[846][4]));let eq39_e1275_d_b5: f64 = ((eq39_e1273_d_b5 * s.v[846]) + (eq39_e1273 * s.db[846][5]));let eq39_e1275_d_b6: f64 = ((eq39_e1273_d_b6 * s.v[846]) + (eq39_e1273 * s.db[846][6]));let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq39_e1275);let eq39_value: f64 = eq39_e1276;let eq39_node_derivatives: [f64; 12] = [(eq39_e1275_d_n0 * ddt_scale), (eq39_e1275_d_n1 * ddt_scale), (eq39_e1275_d_n2 * ddt_scale), (eq39_e1275_d_n3 * ddt_scale), (eq39_e1275_d_n4 * ddt_scale), (eq39_e1275_d_n5 * ddt_scale), (eq39_e1275_d_n6 * ddt_scale), (eq39_e1275_d_n7 * ddt_scale), (eq39_e1275_d_n8 * ddt_scale), (eq39_e1275_d_n9 * ddt_scale), (eq39_e1275_d_n10 * ddt_scale), (eq39_e1275_d_n11 * ddt_scale)];let eq39_branch_derivatives: [f64; 7] = [(eq39_e1275_d_b0 * ddt_scale), (eq39_e1275_d_b1 * ddt_scale), (eq39_e1275_d_b2 * ddt_scale), (eq39_e1275_d_b3 * ddt_scale), (eq39_e1275_d_b4 * ddt_scale), (eq39_e1275_d_b5 * ddt_scale), (eq39_e1275_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_24(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq40_e1279: f64 = (s.v[0] * s.v[15]);let eq40_e1279_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq40_e1279_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq40_e1279_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq40_e1279_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq40_e1279_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq40_e1279_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq40_e1279_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq40_e1279_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq40_e1279_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq40_e1279_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq40_e1279_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq40_e1279_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq40_e1279_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq40_e1279_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq40_e1279_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq40_e1279_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq40_e1279_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq40_e1279_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq40_e1279_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq40_e1281: f64 = (eq40_e1279 * p.p33);let eq40_e1281_d_n0: f64 = (eq40_e1279_d_n0 * p.p33);let eq40_e1281_d_n1: f64 = (eq40_e1279_d_n1 * p.p33);let eq40_e1281_d_n2: f64 = (eq40_e1279_d_n2 * p.p33);let eq40_e1281_d_n3: f64 = (eq40_e1279_d_n3 * p.p33);let eq40_e1281_d_n4: f64 = (eq40_e1279_d_n4 * p.p33);let eq40_e1281_d_n5: f64 = (eq40_e1279_d_n5 * p.p33);let eq40_e1281_d_n6: f64 = (eq40_e1279_d_n6 * p.p33);let eq40_e1281_d_n7: f64 = (eq40_e1279_d_n7 * p.p33);let eq40_e1281_d_n8: f64 = (eq40_e1279_d_n8 * p.p33);let eq40_e1281_d_n9: f64 = (eq40_e1279_d_n9 * p.p33);let eq40_e1281_d_n10: f64 = (eq40_e1279_d_n10 * p.p33);let eq40_e1281_d_n11: f64 = (eq40_e1279_d_n11 * p.p33);let eq40_e1281_d_b0: f64 = (eq40_e1279_d_b0 * p.p33);let eq40_e1281_d_b1: f64 = (eq40_e1279_d_b1 * p.p33);let eq40_e1281_d_b2: f64 = (eq40_e1279_d_b2 * p.p33);let eq40_e1281_d_b3: f64 = (eq40_e1279_d_b3 * p.p33);let eq40_e1281_d_b4: f64 = (eq40_e1279_d_b4 * p.p33);let eq40_e1281_d_b5: f64 = (eq40_e1279_d_b5 * p.p33);let eq40_e1281_d_b6: f64 = (eq40_e1279_d_b6 * p.p33);let eq40_e1283: f64 = (eq40_e1281 * s.v[847]);let eq40_e1283_d_n0: f64 = ((eq40_e1281_d_n0 * s.v[847]) + (eq40_e1281 * s.dn[847][0]));let eq40_e1283_d_n1: f64 = ((eq40_e1281_d_n1 * s.v[847]) + (eq40_e1281 * s.dn[847][1]));let eq40_e1283_d_n2: f64 = ((eq40_e1281_d_n2 * s.v[847]) + (eq40_e1281 * s.dn[847][2]));let eq40_e1283_d_n3: f64 = ((eq40_e1281_d_n3 * s.v[847]) + (eq40_e1281 * s.dn[847][3]));let eq40_e1283_d_n4: f64 = ((eq40_e1281_d_n4 * s.v[847]) + (eq40_e1281 * s.dn[847][4]));let eq40_e1283_d_n5: f64 = ((eq40_e1281_d_n5 * s.v[847]) + (eq40_e1281 * s.dn[847][5]));let eq40_e1283_d_n6: f64 = ((eq40_e1281_d_n6 * s.v[847]) + (eq40_e1281 * s.dn[847][6]));let eq40_e1283_d_n7: f64 = ((eq40_e1281_d_n7 * s.v[847]) + (eq40_e1281 * s.dn[847][7]));let eq40_e1283_d_n8: f64 = ((eq40_e1281_d_n8 * s.v[847]) + (eq40_e1281 * s.dn[847][8]));let eq40_e1283_d_n9: f64 = ((eq40_e1281_d_n9 * s.v[847]) + (eq40_e1281 * s.dn[847][9]));let eq40_e1283_d_n10: f64 = ((eq40_e1281_d_n10 * s.v[847]) + (eq40_e1281 * s.dn[847][10]));let eq40_e1283_d_n11: f64 = ((eq40_e1281_d_n11 * s.v[847]) + (eq40_e1281 * s.dn[847][11]));let eq40_e1283_d_b0: f64 = ((eq40_e1281_d_b0 * s.v[847]) + (eq40_e1281 * s.db[847][0]));let eq40_e1283_d_b1: f64 = ((eq40_e1281_d_b1 * s.v[847]) + (eq40_e1281 * s.db[847][1]));let eq40_e1283_d_b2: f64 = ((eq40_e1281_d_b2 * s.v[847]) + (eq40_e1281 * s.db[847][2]));let eq40_e1283_d_b3: f64 = ((eq40_e1281_d_b3 * s.v[847]) + (eq40_e1281 * s.db[847][3]));
        let eq40_e1283_d_b4: f64 = ((eq40_e1281_d_b4 * s.v[847]) + (eq40_e1281 * s.db[847][4]));let eq40_e1283_d_b5: f64 = ((eq40_e1281_d_b5 * s.v[847]) + (eq40_e1281 * s.db[847][5]));let eq40_e1283_d_b6: f64 = ((eq40_e1281_d_b6 * s.v[847]) + (eq40_e1281 * s.db[847][6]));let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq40_e1283);let eq40_value: f64 = eq40_e1284;let eq40_node_derivatives: [f64; 12] = [(eq40_e1283_d_n0 * ddt_scale), (eq40_e1283_d_n1 * ddt_scale), (eq40_e1283_d_n2 * ddt_scale), (eq40_e1283_d_n3 * ddt_scale), (eq40_e1283_d_n4 * ddt_scale), (eq40_e1283_d_n5 * ddt_scale), (eq40_e1283_d_n6 * ddt_scale), (eq40_e1283_d_n7 * ddt_scale), (eq40_e1283_d_n8 * ddt_scale), (eq40_e1283_d_n9 * ddt_scale), (eq40_e1283_d_n10 * ddt_scale), (eq40_e1283_d_n11 * ddt_scale)];let eq40_branch_derivatives: [f64; 7] = [(eq40_e1283_d_b0 * ddt_scale), (eq40_e1283_d_b1 * ddt_scale), (eq40_e1283_d_b2 * ddt_scale), (eq40_e1283_d_b3 * ddt_scale), (eq40_e1283_d_b4 * ddt_scale), (eq40_e1283_d_b5 * ddt_scale), (eq40_e1283_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_25(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq41_e1287: f64 = (s.v[0] * s.v[15]);let eq41_e1287_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq41_e1287_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq41_e1287_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq41_e1287_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq41_e1287_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq41_e1287_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq41_e1287_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq41_e1287_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq41_e1287_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq41_e1287_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq41_e1287_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq41_e1287_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq41_e1287_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq41_e1287_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq41_e1287_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq41_e1287_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq41_e1287_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq41_e1287_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq41_e1287_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq41_e1289: f64 = (eq41_e1287 * p.p33);let eq41_e1289_d_n0: f64 = (eq41_e1287_d_n0 * p.p33);let eq41_e1289_d_n1: f64 = (eq41_e1287_d_n1 * p.p33);let eq41_e1289_d_n2: f64 = (eq41_e1287_d_n2 * p.p33);let eq41_e1289_d_n3: f64 = (eq41_e1287_d_n3 * p.p33);let eq41_e1289_d_n4: f64 = (eq41_e1287_d_n4 * p.p33);let eq41_e1289_d_n5: f64 = (eq41_e1287_d_n5 * p.p33);let eq41_e1289_d_n6: f64 = (eq41_e1287_d_n6 * p.p33);let eq41_e1289_d_n7: f64 = (eq41_e1287_d_n7 * p.p33);let eq41_e1289_d_n8: f64 = (eq41_e1287_d_n8 * p.p33);let eq41_e1289_d_n9: f64 = (eq41_e1287_d_n9 * p.p33);let eq41_e1289_d_n10: f64 = (eq41_e1287_d_n10 * p.p33);let eq41_e1289_d_n11: f64 = (eq41_e1287_d_n11 * p.p33);let eq41_e1289_d_b0: f64 = (eq41_e1287_d_b0 * p.p33);let eq41_e1289_d_b1: f64 = (eq41_e1287_d_b1 * p.p33);let eq41_e1289_d_b2: f64 = (eq41_e1287_d_b2 * p.p33);let eq41_e1289_d_b3: f64 = (eq41_e1287_d_b3 * p.p33);let eq41_e1289_d_b4: f64 = (eq41_e1287_d_b4 * p.p33);let eq41_e1289_d_b5: f64 = (eq41_e1287_d_b5 * p.p33);let eq41_e1289_d_b6: f64 = (eq41_e1287_d_b6 * p.p33);let eq41_e1291: f64 = (eq41_e1289 * s.v[848]);let eq41_e1291_d_n0: f64 = ((eq41_e1289_d_n0 * s.v[848]) + (eq41_e1289 * s.dn[848][0]));let eq41_e1291_d_n1: f64 = ((eq41_e1289_d_n1 * s.v[848]) + (eq41_e1289 * s.dn[848][1]));let eq41_e1291_d_n2: f64 = ((eq41_e1289_d_n2 * s.v[848]) + (eq41_e1289 * s.dn[848][2]));let eq41_e1291_d_n3: f64 = ((eq41_e1289_d_n3 * s.v[848]) + (eq41_e1289 * s.dn[848][3]));let eq41_e1291_d_n4: f64 = ((eq41_e1289_d_n4 * s.v[848]) + (eq41_e1289 * s.dn[848][4]));let eq41_e1291_d_n5: f64 = ((eq41_e1289_d_n5 * s.v[848]) + (eq41_e1289 * s.dn[848][5]));let eq41_e1291_d_n6: f64 = ((eq41_e1289_d_n6 * s.v[848]) + (eq41_e1289 * s.dn[848][6]));let eq41_e1291_d_n7: f64 = ((eq41_e1289_d_n7 * s.v[848]) + (eq41_e1289 * s.dn[848][7]));let eq41_e1291_d_n8: f64 = ((eq41_e1289_d_n8 * s.v[848]) + (eq41_e1289 * s.dn[848][8]));let eq41_e1291_d_n9: f64 = ((eq41_e1289_d_n9 * s.v[848]) + (eq41_e1289 * s.dn[848][9]));let eq41_e1291_d_n10: f64 = ((eq41_e1289_d_n10 * s.v[848]) + (eq41_e1289 * s.dn[848][10]));let eq41_e1291_d_n11: f64 = ((eq41_e1289_d_n11 * s.v[848]) + (eq41_e1289 * s.dn[848][11]));let eq41_e1291_d_b0: f64 = ((eq41_e1289_d_b0 * s.v[848]) + (eq41_e1289 * s.db[848][0]));let eq41_e1291_d_b1: f64 = ((eq41_e1289_d_b1 * s.v[848]) + (eq41_e1289 * s.db[848][1]));let eq41_e1291_d_b2: f64 = ((eq41_e1289_d_b2 * s.v[848]) + (eq41_e1289 * s.db[848][2]));let eq41_e1291_d_b3: f64 = ((eq41_e1289_d_b3 * s.v[848]) + (eq41_e1289 * s.db[848][3]));
        let eq41_e1291_d_b4: f64 = ((eq41_e1289_d_b4 * s.v[848]) + (eq41_e1289 * s.db[848][4]));let eq41_e1291_d_b5: f64 = ((eq41_e1289_d_b5 * s.v[848]) + (eq41_e1289 * s.db[848][5]));let eq41_e1291_d_b6: f64 = ((eq41_e1289_d_b6 * s.v[848]) + (eq41_e1289 * s.db[848][6]));let eq41_e1292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq41_e1291);let eq41_value: f64 = eq41_e1292;let eq41_node_derivatives: [f64; 12] = [(eq41_e1291_d_n0 * ddt_scale), (eq41_e1291_d_n1 * ddt_scale), (eq41_e1291_d_n2 * ddt_scale), (eq41_e1291_d_n3 * ddt_scale), (eq41_e1291_d_n4 * ddt_scale), (eq41_e1291_d_n5 * ddt_scale), (eq41_e1291_d_n6 * ddt_scale), (eq41_e1291_d_n7 * ddt_scale), (eq41_e1291_d_n8 * ddt_scale), (eq41_e1291_d_n9 * ddt_scale), (eq41_e1291_d_n10 * ddt_scale), (eq41_e1291_d_n11 * ddt_scale)];let eq41_branch_derivatives: [f64; 7] = [(eq41_e1291_d_b0 * ddt_scale), (eq41_e1291_d_b1 * ddt_scale), (eq41_e1291_d_b2 * ddt_scale), (eq41_e1291_d_b3 * ddt_scale), (eq41_e1291_d_b4 * ddt_scale), (eq41_e1291_d_b5 * ddt_scale), (eq41_e1291_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_26(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        let eq42_e1295: f64 = (s.v[0] * s.v[15]);let eq42_e1295_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq42_e1295_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq42_e1295_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq42_e1295_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq42_e1295_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq42_e1295_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq42_e1295_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq42_e1295_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq42_e1295_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq42_e1295_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq42_e1295_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq42_e1295_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq42_e1295_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq42_e1295_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq42_e1295_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq42_e1295_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq42_e1295_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq42_e1295_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq42_e1295_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq42_e1297: f64 = (eq42_e1295 * p.p33);let eq42_e1297_d_n0: f64 = (eq42_e1295_d_n0 * p.p33);let eq42_e1297_d_n1: f64 = (eq42_e1295_d_n1 * p.p33);let eq42_e1297_d_n2: f64 = (eq42_e1295_d_n2 * p.p33);let eq42_e1297_d_n3: f64 = (eq42_e1295_d_n3 * p.p33);let eq42_e1297_d_n4: f64 = (eq42_e1295_d_n4 * p.p33);let eq42_e1297_d_n5: f64 = (eq42_e1295_d_n5 * p.p33);let eq42_e1297_d_n6: f64 = (eq42_e1295_d_n6 * p.p33);let eq42_e1297_d_n7: f64 = (eq42_e1295_d_n7 * p.p33);let eq42_e1297_d_n8: f64 = (eq42_e1295_d_n8 * p.p33);let eq42_e1297_d_n9: f64 = (eq42_e1295_d_n9 * p.p33);let eq42_e1297_d_n10: f64 = (eq42_e1295_d_n10 * p.p33);let eq42_e1297_d_n11: f64 = (eq42_e1295_d_n11 * p.p33);let eq42_e1297_d_b0: f64 = (eq42_e1295_d_b0 * p.p33);let eq42_e1297_d_b1: f64 = (eq42_e1295_d_b1 * p.p33);let eq42_e1297_d_b2: f64 = (eq42_e1295_d_b2 * p.p33);let eq42_e1297_d_b3: f64 = (eq42_e1295_d_b3 * p.p33);let eq42_e1297_d_b4: f64 = (eq42_e1295_d_b4 * p.p33);let eq42_e1297_d_b5: f64 = (eq42_e1295_d_b5 * p.p33);let eq42_e1297_d_b6: f64 = (eq42_e1295_d_b6 * p.p33);let eq42_e1299: f64 = (eq42_e1297 * s.v[849]);let eq42_e1299_d_n0: f64 = ((eq42_e1297_d_n0 * s.v[849]) + (eq42_e1297 * s.dn[849][0]));let eq42_e1299_d_n1: f64 = ((eq42_e1297_d_n1 * s.v[849]) + (eq42_e1297 * s.dn[849][1]));let eq42_e1299_d_n2: f64 = ((eq42_e1297_d_n2 * s.v[849]) + (eq42_e1297 * s.dn[849][2]));let eq42_e1299_d_n3: f64 = ((eq42_e1297_d_n3 * s.v[849]) + (eq42_e1297 * s.dn[849][3]));let eq42_e1299_d_n4: f64 = ((eq42_e1297_d_n4 * s.v[849]) + (eq42_e1297 * s.dn[849][4]));let eq42_e1299_d_n5: f64 = ((eq42_e1297_d_n5 * s.v[849]) + (eq42_e1297 * s.dn[849][5]));let eq42_e1299_d_n6: f64 = ((eq42_e1297_d_n6 * s.v[849]) + (eq42_e1297 * s.dn[849][6]));let eq42_e1299_d_n7: f64 = ((eq42_e1297_d_n7 * s.v[849]) + (eq42_e1297 * s.dn[849][7]));let eq42_e1299_d_n8: f64 = ((eq42_e1297_d_n8 * s.v[849]) + (eq42_e1297 * s.dn[849][8]));let eq42_e1299_d_n9: f64 = ((eq42_e1297_d_n9 * s.v[849]) + (eq42_e1297 * s.dn[849][9]));let eq42_e1299_d_n10: f64 = ((eq42_e1297_d_n10 * s.v[849]) + (eq42_e1297 * s.dn[849][10]));let eq42_e1299_d_n11: f64 = ((eq42_e1297_d_n11 * s.v[849]) + (eq42_e1297 * s.dn[849][11]));let eq42_e1299_d_b0: f64 = ((eq42_e1297_d_b0 * s.v[849]) + (eq42_e1297 * s.db[849][0]));let eq42_e1299_d_b1: f64 = ((eq42_e1297_d_b1 * s.v[849]) + (eq42_e1297 * s.db[849][1]));let eq42_e1299_d_b2: f64 = ((eq42_e1297_d_b2 * s.v[849]) + (eq42_e1297 * s.db[849][2]));let eq42_e1299_d_b3: f64 = ((eq42_e1297_d_b3 * s.v[849]) + (eq42_e1297 * s.db[849][3]));
        let eq42_e1299_d_b4: f64 = ((eq42_e1297_d_b4 * s.v[849]) + (eq42_e1297 * s.db[849][4]));let eq42_e1299_d_b5: f64 = ((eq42_e1297_d_b5 * s.v[849]) + (eq42_e1297 * s.db[849][5]));let eq42_e1299_d_b6: f64 = ((eq42_e1297_d_b6 * s.v[849]) + (eq42_e1297 * s.db[849][6]));let eq42_e1300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq42_e1299);let eq42_value: f64 = eq42_e1300;let eq42_node_derivatives: [f64; 12] = [(eq42_e1299_d_n0 * ddt_scale), (eq42_e1299_d_n1 * ddt_scale), (eq42_e1299_d_n2 * ddt_scale), (eq42_e1299_d_n3 * ddt_scale), (eq42_e1299_d_n4 * ddt_scale), (eq42_e1299_d_n5 * ddt_scale), (eq42_e1299_d_n6 * ddt_scale), (eq42_e1299_d_n7 * ddt_scale), (eq42_e1299_d_n8 * ddt_scale), (eq42_e1299_d_n9 * ddt_scale), (eq42_e1299_d_n10 * ddt_scale), (eq42_e1299_d_n11 * ddt_scale)];let eq42_branch_derivatives: [f64; 7] = [(eq42_e1299_d_b0 * ddt_scale), (eq42_e1299_d_b1 * ddt_scale), (eq42_e1299_d_b2 * ddt_scale), (eq42_e1299_d_b3 * ddt_scale), (eq42_e1299_d_b4 * ddt_scale), (eq42_e1299_d_b5 * ddt_scale), (eq42_e1299_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
    }
}
