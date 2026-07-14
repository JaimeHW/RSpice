#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq9_e1060: f64 = (s.v[0] * s.v[15]);let eq9_e1060_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq9_e1060_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq9_e1060_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq9_e1060_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq9_e1060_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq9_e1060_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq9_e1060_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq9_e1060_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq9_e1060_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq9_e1060_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq9_e1060_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq9_e1060_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq9_e1060_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq9_e1060_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq9_e1060_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq9_e1060_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq9_e1060_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq9_e1060_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq9_e1060_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq9_e1060_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq9_e1062: f64 = (eq9_e1060 * p.p32);let eq9_e1062_d_n0: f64 = (eq9_e1060_d_n0 * p.p32);let eq9_e1062_d_n1: f64 = (eq9_e1060_d_n1 * p.p32);let eq9_e1062_d_n2: f64 = (eq9_e1060_d_n2 * p.p32);let eq9_e1062_d_n3: f64 = (eq9_e1060_d_n3 * p.p32);let eq9_e1062_d_n4: f64 = (eq9_e1060_d_n4 * p.p32);let eq9_e1062_d_n5: f64 = (eq9_e1060_d_n5 * p.p32);let eq9_e1062_d_n6: f64 = (eq9_e1060_d_n6 * p.p32);let eq9_e1062_d_n7: f64 = (eq9_e1060_d_n7 * p.p32);let eq9_e1062_d_n8: f64 = (eq9_e1060_d_n8 * p.p32);let eq9_e1062_d_n9: f64 = (eq9_e1060_d_n9 * p.p32);let eq9_e1062_d_n10: f64 = (eq9_e1060_d_n10 * p.p32);let eq9_e1062_d_n11: f64 = (eq9_e1060_d_n11 * p.p32);let eq9_e1062_d_n12: f64 = (eq9_e1060_d_n12 * p.p32);let eq9_e1062_d_b0: f64 = (eq9_e1060_d_b0 * p.p32);let eq9_e1062_d_b1: f64 = (eq9_e1060_d_b1 * p.p32);let eq9_e1062_d_b2: f64 = (eq9_e1060_d_b2 * p.p32);let eq9_e1062_d_b3: f64 = (eq9_e1060_d_b3 * p.p32);let eq9_e1062_d_b4: f64 = (eq9_e1060_d_b4 * p.p32);let eq9_e1062_d_b5: f64 = (eq9_e1060_d_b5 * p.p32);let eq9_e1062_d_b6: f64 = (eq9_e1060_d_b6 * p.p32);let eq9_e1064: f64 = (eq9_e1062 * s.v[828]);let eq9_e1064_d_n0: f64 = ((eq9_e1062_d_n0 * s.v[828]) + (eq9_e1062 * s.dn[828][0]));let eq9_e1064_d_n1: f64 = ((eq9_e1062_d_n1 * s.v[828]) + (eq9_e1062 * s.dn[828][1]));let eq9_e1064_d_n2: f64 = ((eq9_e1062_d_n2 * s.v[828]) + (eq9_e1062 * s.dn[828][2]));let eq9_e1064_d_n3: f64 = ((eq9_e1062_d_n3 * s.v[828]) + (eq9_e1062 * s.dn[828][3]));let eq9_e1064_d_n4: f64 = ((eq9_e1062_d_n4 * s.v[828]) + (eq9_e1062 * s.dn[828][4]));let eq9_e1064_d_n5: f64 = ((eq9_e1062_d_n5 * s.v[828]) + (eq9_e1062 * s.dn[828][5]));let eq9_e1064_d_n6: f64 = ((eq9_e1062_d_n6 * s.v[828]) + (eq9_e1062 * s.dn[828][6]));let eq9_e1064_d_n7: f64 = ((eq9_e1062_d_n7 * s.v[828]) + (eq9_e1062 * s.dn[828][7]));let eq9_e1064_d_n8: f64 = ((eq9_e1062_d_n8 * s.v[828]) + (eq9_e1062 * s.dn[828][8]));let eq9_e1064_d_n9: f64 = ((eq9_e1062_d_n9 * s.v[828]) + (eq9_e1062 * s.dn[828][9]));let eq9_e1064_d_n10: f64 = ((eq9_e1062_d_n10 * s.v[828]) + (eq9_e1062 * s.dn[828][10]));let eq9_e1064_d_n11: f64 = ((eq9_e1062_d_n11 * s.v[828]) + (eq9_e1062 * s.dn[828][11]));let eq9_e1064_d_n12: f64 = ((eq9_e1062_d_n12 * s.v[828]) + (eq9_e1062 * s.dn[828][12]));let eq9_e1064_d_b0: f64 = ((eq9_e1062_d_b0 * s.v[828]) + (eq9_e1062 * s.db[828][0]));let eq9_e1064_d_b1: f64 = ((eq9_e1062_d_b1 * s.v[828]) + (eq9_e1062 * s.db[828][1]));let eq9_e1064_d_b2: f64 = ((eq9_e1062_d_b2 * s.v[828]) + (eq9_e1062 * s.db[828][2]));
        let eq9_e1064_d_b3: f64 = ((eq9_e1062_d_b3 * s.v[828]) + (eq9_e1062 * s.db[828][3]));let eq9_e1064_d_b4: f64 = ((eq9_e1062_d_b4 * s.v[828]) + (eq9_e1062 * s.db[828][4]));let eq9_e1064_d_b5: f64 = ((eq9_e1062_d_b5 * s.v[828]) + (eq9_e1062 * s.db[828][5]));let eq9_e1064_d_b6: f64 = ((eq9_e1062_d_b6 * s.v[828]) + (eq9_e1062 * s.db[828][6]));let eq9_value: f64 = eq9_e1064;let eq9_node_derivatives: [f64; 13] = [eq9_e1064_d_n0, eq9_e1064_d_n1, eq9_e1064_d_n2, eq9_e1064_d_n3, eq9_e1064_d_n4, eq9_e1064_d_n5, eq9_e1064_d_n6, eq9_e1064_d_n7, eq9_e1064_d_n8, eq9_e1064_d_n9, eq9_e1064_d_n10, eq9_e1064_d_n11, eq9_e1064_d_n12];let eq9_branch_derivatives: [f64; 7] = [eq9_e1064_d_b0, eq9_e1064_d_b1, eq9_e1064_d_b2, eq9_e1064_d_b3, eq9_e1064_d_b4, eq9_e1064_d_b5, eq9_e1064_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq10_e1067: f64 = (s.v[0] * s.v[15]);let eq10_e1067_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq10_e1067_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq10_e1067_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq10_e1067_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq10_e1067_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq10_e1067_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq10_e1067_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq10_e1067_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq10_e1067_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq10_e1067_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq10_e1067_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq10_e1067_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq10_e1067_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq10_e1067_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq10_e1067_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq10_e1067_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq10_e1067_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq10_e1067_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq10_e1067_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq10_e1067_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq10_e1069: f64 = (eq10_e1067 * p.p32);let eq10_e1069_d_n0: f64 = (eq10_e1067_d_n0 * p.p32);let eq10_e1069_d_n1: f64 = (eq10_e1067_d_n1 * p.p32);let eq10_e1069_d_n2: f64 = (eq10_e1067_d_n2 * p.p32);let eq10_e1069_d_n3: f64 = (eq10_e1067_d_n3 * p.p32);let eq10_e1069_d_n4: f64 = (eq10_e1067_d_n4 * p.p32);let eq10_e1069_d_n5: f64 = (eq10_e1067_d_n5 * p.p32);let eq10_e1069_d_n6: f64 = (eq10_e1067_d_n6 * p.p32);let eq10_e1069_d_n7: f64 = (eq10_e1067_d_n7 * p.p32);let eq10_e1069_d_n8: f64 = (eq10_e1067_d_n8 * p.p32);let eq10_e1069_d_n9: f64 = (eq10_e1067_d_n9 * p.p32);let eq10_e1069_d_n10: f64 = (eq10_e1067_d_n10 * p.p32);let eq10_e1069_d_n11: f64 = (eq10_e1067_d_n11 * p.p32);let eq10_e1069_d_n12: f64 = (eq10_e1067_d_n12 * p.p32);let eq10_e1069_d_b0: f64 = (eq10_e1067_d_b0 * p.p32);let eq10_e1069_d_b1: f64 = (eq10_e1067_d_b1 * p.p32);let eq10_e1069_d_b2: f64 = (eq10_e1067_d_b2 * p.p32);let eq10_e1069_d_b3: f64 = (eq10_e1067_d_b3 * p.p32);let eq10_e1069_d_b4: f64 = (eq10_e1067_d_b4 * p.p32);let eq10_e1069_d_b5: f64 = (eq10_e1067_d_b5 * p.p32);let eq10_e1069_d_b6: f64 = (eq10_e1067_d_b6 * p.p32);let eq10_e1071: f64 = (eq10_e1069 * s.v[829]);let eq10_e1071_d_n0: f64 = ((eq10_e1069_d_n0 * s.v[829]) + (eq10_e1069 * s.dn[829][0]));let eq10_e1071_d_n1: f64 = ((eq10_e1069_d_n1 * s.v[829]) + (eq10_e1069 * s.dn[829][1]));let eq10_e1071_d_n2: f64 = ((eq10_e1069_d_n2 * s.v[829]) + (eq10_e1069 * s.dn[829][2]));let eq10_e1071_d_n3: f64 = ((eq10_e1069_d_n3 * s.v[829]) + (eq10_e1069 * s.dn[829][3]));let eq10_e1071_d_n4: f64 = ((eq10_e1069_d_n4 * s.v[829]) + (eq10_e1069 * s.dn[829][4]));let eq10_e1071_d_n5: f64 = ((eq10_e1069_d_n5 * s.v[829]) + (eq10_e1069 * s.dn[829][5]));let eq10_e1071_d_n6: f64 = ((eq10_e1069_d_n6 * s.v[829]) + (eq10_e1069 * s.dn[829][6]));let eq10_e1071_d_n7: f64 = ((eq10_e1069_d_n7 * s.v[829]) + (eq10_e1069 * s.dn[829][7]));let eq10_e1071_d_n8: f64 = ((eq10_e1069_d_n8 * s.v[829]) + (eq10_e1069 * s.dn[829][8]));let eq10_e1071_d_n9: f64 = ((eq10_e1069_d_n9 * s.v[829]) + (eq10_e1069 * s.dn[829][9]));let eq10_e1071_d_n10: f64 = ((eq10_e1069_d_n10 * s.v[829]) + (eq10_e1069 * s.dn[829][10]));let eq10_e1071_d_n11: f64 = ((eq10_e1069_d_n11 * s.v[829]) + (eq10_e1069 * s.dn[829][11]));let eq10_e1071_d_n12: f64 = ((eq10_e1069_d_n12 * s.v[829]) + (eq10_e1069 * s.dn[829][12]));let eq10_e1071_d_b0: f64 = ((eq10_e1069_d_b0 * s.v[829]) + (eq10_e1069 * s.db[829][0]));let eq10_e1071_d_b1: f64 = ((eq10_e1069_d_b1 * s.v[829]) + (eq10_e1069 * s.db[829][1]));
        let eq10_e1071_d_b2: f64 = ((eq10_e1069_d_b2 * s.v[829]) + (eq10_e1069 * s.db[829][2]));let eq10_e1071_d_b3: f64 = ((eq10_e1069_d_b3 * s.v[829]) + (eq10_e1069 * s.db[829][3]));let eq10_e1071_d_b4: f64 = ((eq10_e1069_d_b4 * s.v[829]) + (eq10_e1069 * s.db[829][4]));let eq10_e1071_d_b5: f64 = ((eq10_e1069_d_b5 * s.v[829]) + (eq10_e1069 * s.db[829][5]));let eq10_e1071_d_b6: f64 = ((eq10_e1069_d_b6 * s.v[829]) + (eq10_e1069 * s.db[829][6]));let eq10_value: f64 = eq10_e1071;let eq10_node_derivatives: [f64; 13] = [eq10_e1071_d_n0, eq10_e1071_d_n1, eq10_e1071_d_n2, eq10_e1071_d_n3, eq10_e1071_d_n4, eq10_e1071_d_n5, eq10_e1071_d_n6, eq10_e1071_d_n7, eq10_e1071_d_n8, eq10_e1071_d_n9, eq10_e1071_d_n10, eq10_e1071_d_n11, eq10_e1071_d_n12];let eq10_branch_derivatives: [f64; 7] = [eq10_e1071_d_b0, eq10_e1071_d_b1, eq10_e1071_d_b2, eq10_e1071_d_b3, eq10_e1071_d_b4, eq10_e1071_d_b5, eq10_e1071_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq11_e1074: f64 = (s.v[0] * s.v[15]);let eq11_e1074_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq11_e1074_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq11_e1074_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq11_e1074_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq11_e1074_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq11_e1074_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq11_e1074_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq11_e1074_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq11_e1074_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq11_e1074_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq11_e1074_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq11_e1074_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq11_e1074_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq11_e1074_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq11_e1074_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq11_e1074_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq11_e1074_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq11_e1074_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq11_e1074_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq11_e1074_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq11_e1076: f64 = (eq11_e1074 * p.p32);let eq11_e1076_d_n0: f64 = (eq11_e1074_d_n0 * p.p32);let eq11_e1076_d_n1: f64 = (eq11_e1074_d_n1 * p.p32);let eq11_e1076_d_n2: f64 = (eq11_e1074_d_n2 * p.p32);let eq11_e1076_d_n3: f64 = (eq11_e1074_d_n3 * p.p32);let eq11_e1076_d_n4: f64 = (eq11_e1074_d_n4 * p.p32);let eq11_e1076_d_n5: f64 = (eq11_e1074_d_n5 * p.p32);let eq11_e1076_d_n6: f64 = (eq11_e1074_d_n6 * p.p32);let eq11_e1076_d_n7: f64 = (eq11_e1074_d_n7 * p.p32);let eq11_e1076_d_n8: f64 = (eq11_e1074_d_n8 * p.p32);let eq11_e1076_d_n9: f64 = (eq11_e1074_d_n9 * p.p32);let eq11_e1076_d_n10: f64 = (eq11_e1074_d_n10 * p.p32);let eq11_e1076_d_n11: f64 = (eq11_e1074_d_n11 * p.p32);let eq11_e1076_d_n12: f64 = (eq11_e1074_d_n12 * p.p32);let eq11_e1076_d_b0: f64 = (eq11_e1074_d_b0 * p.p32);let eq11_e1076_d_b1: f64 = (eq11_e1074_d_b1 * p.p32);let eq11_e1076_d_b2: f64 = (eq11_e1074_d_b2 * p.p32);let eq11_e1076_d_b3: f64 = (eq11_e1074_d_b3 * p.p32);let eq11_e1076_d_b4: f64 = (eq11_e1074_d_b4 * p.p32);let eq11_e1076_d_b5: f64 = (eq11_e1074_d_b5 * p.p32);let eq11_e1076_d_b6: f64 = (eq11_e1074_d_b6 * p.p32);let eq11_e1078: f64 = (eq11_e1076 * s.v[833]);let eq11_e1078_d_n0: f64 = ((eq11_e1076_d_n0 * s.v[833]) + (eq11_e1076 * s.dn[833][0]));let eq11_e1078_d_n1: f64 = ((eq11_e1076_d_n1 * s.v[833]) + (eq11_e1076 * s.dn[833][1]));let eq11_e1078_d_n2: f64 = ((eq11_e1076_d_n2 * s.v[833]) + (eq11_e1076 * s.dn[833][2]));let eq11_e1078_d_n3: f64 = ((eq11_e1076_d_n3 * s.v[833]) + (eq11_e1076 * s.dn[833][3]));let eq11_e1078_d_n4: f64 = ((eq11_e1076_d_n4 * s.v[833]) + (eq11_e1076 * s.dn[833][4]));let eq11_e1078_d_n5: f64 = ((eq11_e1076_d_n5 * s.v[833]) + (eq11_e1076 * s.dn[833][5]));let eq11_e1078_d_n6: f64 = ((eq11_e1076_d_n6 * s.v[833]) + (eq11_e1076 * s.dn[833][6]));let eq11_e1078_d_n7: f64 = ((eq11_e1076_d_n7 * s.v[833]) + (eq11_e1076 * s.dn[833][7]));let eq11_e1078_d_n8: f64 = ((eq11_e1076_d_n8 * s.v[833]) + (eq11_e1076 * s.dn[833][8]));let eq11_e1078_d_n9: f64 = ((eq11_e1076_d_n9 * s.v[833]) + (eq11_e1076 * s.dn[833][9]));let eq11_e1078_d_n10: f64 = ((eq11_e1076_d_n10 * s.v[833]) + (eq11_e1076 * s.dn[833][10]));let eq11_e1078_d_n11: f64 = ((eq11_e1076_d_n11 * s.v[833]) + (eq11_e1076 * s.dn[833][11]));let eq11_e1078_d_n12: f64 = ((eq11_e1076_d_n12 * s.v[833]) + (eq11_e1076 * s.dn[833][12]));let eq11_e1078_d_b0: f64 = ((eq11_e1076_d_b0 * s.v[833]) + (eq11_e1076 * s.db[833][0]));let eq11_e1078_d_b1: f64 = ((eq11_e1076_d_b1 * s.v[833]) + (eq11_e1076 * s.db[833][1]));
        let eq11_e1078_d_b2: f64 = ((eq11_e1076_d_b2 * s.v[833]) + (eq11_e1076 * s.db[833][2]));let eq11_e1078_d_b3: f64 = ((eq11_e1076_d_b3 * s.v[833]) + (eq11_e1076 * s.db[833][3]));let eq11_e1078_d_b4: f64 = ((eq11_e1076_d_b4 * s.v[833]) + (eq11_e1076 * s.db[833][4]));let eq11_e1078_d_b5: f64 = ((eq11_e1076_d_b5 * s.v[833]) + (eq11_e1076 * s.db[833][5]));let eq11_e1078_d_b6: f64 = ((eq11_e1076_d_b6 * s.v[833]) + (eq11_e1076 * s.db[833][6]));let eq11_value: f64 = eq11_e1078;let eq11_node_derivatives: [f64; 13] = [eq11_e1078_d_n0, eq11_e1078_d_n1, eq11_e1078_d_n2, eq11_e1078_d_n3, eq11_e1078_d_n4, eq11_e1078_d_n5, eq11_e1078_d_n6, eq11_e1078_d_n7, eq11_e1078_d_n8, eq11_e1078_d_n9, eq11_e1078_d_n10, eq11_e1078_d_n11, eq11_e1078_d_n12];let eq11_branch_derivatives: [f64; 7] = [eq11_e1078_d_b0, eq11_e1078_d_b1, eq11_e1078_d_b2, eq11_e1078_d_b3, eq11_e1078_d_b4, eq11_e1078_d_b5, eq11_e1078_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
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
        let eq12_e1081: f64 = (s.v[0] * s.v[15]);let eq12_e1081_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq12_e1081_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq12_e1081_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq12_e1081_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq12_e1081_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq12_e1081_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq12_e1081_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq12_e1081_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq12_e1081_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq12_e1081_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq12_e1081_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq12_e1081_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq12_e1081_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq12_e1081_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq12_e1081_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq12_e1081_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq12_e1081_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq12_e1081_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq12_e1081_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq12_e1081_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq12_e1083: f64 = (eq12_e1081 * p.p32);let eq12_e1083_d_n0: f64 = (eq12_e1081_d_n0 * p.p32);let eq12_e1083_d_n1: f64 = (eq12_e1081_d_n1 * p.p32);let eq12_e1083_d_n2: f64 = (eq12_e1081_d_n2 * p.p32);let eq12_e1083_d_n3: f64 = (eq12_e1081_d_n3 * p.p32);let eq12_e1083_d_n4: f64 = (eq12_e1081_d_n4 * p.p32);let eq12_e1083_d_n5: f64 = (eq12_e1081_d_n5 * p.p32);let eq12_e1083_d_n6: f64 = (eq12_e1081_d_n6 * p.p32);let eq12_e1083_d_n7: f64 = (eq12_e1081_d_n7 * p.p32);let eq12_e1083_d_n8: f64 = (eq12_e1081_d_n8 * p.p32);let eq12_e1083_d_n9: f64 = (eq12_e1081_d_n9 * p.p32);let eq12_e1083_d_n10: f64 = (eq12_e1081_d_n10 * p.p32);let eq12_e1083_d_n11: f64 = (eq12_e1081_d_n11 * p.p32);let eq12_e1083_d_n12: f64 = (eq12_e1081_d_n12 * p.p32);let eq12_e1083_d_b0: f64 = (eq12_e1081_d_b0 * p.p32);let eq12_e1083_d_b1: f64 = (eq12_e1081_d_b1 * p.p32);let eq12_e1083_d_b2: f64 = (eq12_e1081_d_b2 * p.p32);let eq12_e1083_d_b3: f64 = (eq12_e1081_d_b3 * p.p32);let eq12_e1083_d_b4: f64 = (eq12_e1081_d_b4 * p.p32);let eq12_e1083_d_b5: f64 = (eq12_e1081_d_b5 * p.p32);let eq12_e1083_d_b6: f64 = (eq12_e1081_d_b6 * p.p32);let eq12_e1085: f64 = (eq12_e1083 * s.v[834]);let eq12_e1085_d_n0: f64 = ((eq12_e1083_d_n0 * s.v[834]) + (eq12_e1083 * s.dn[834][0]));let eq12_e1085_d_n1: f64 = ((eq12_e1083_d_n1 * s.v[834]) + (eq12_e1083 * s.dn[834][1]));let eq12_e1085_d_n2: f64 = ((eq12_e1083_d_n2 * s.v[834]) + (eq12_e1083 * s.dn[834][2]));let eq12_e1085_d_n3: f64 = ((eq12_e1083_d_n3 * s.v[834]) + (eq12_e1083 * s.dn[834][3]));let eq12_e1085_d_n4: f64 = ((eq12_e1083_d_n4 * s.v[834]) + (eq12_e1083 * s.dn[834][4]));let eq12_e1085_d_n5: f64 = ((eq12_e1083_d_n5 * s.v[834]) + (eq12_e1083 * s.dn[834][5]));let eq12_e1085_d_n6: f64 = ((eq12_e1083_d_n6 * s.v[834]) + (eq12_e1083 * s.dn[834][6]));let eq12_e1085_d_n7: f64 = ((eq12_e1083_d_n7 * s.v[834]) + (eq12_e1083 * s.dn[834][7]));let eq12_e1085_d_n8: f64 = ((eq12_e1083_d_n8 * s.v[834]) + (eq12_e1083 * s.dn[834][8]));let eq12_e1085_d_n9: f64 = ((eq12_e1083_d_n9 * s.v[834]) + (eq12_e1083 * s.dn[834][9]));let eq12_e1085_d_n10: f64 = ((eq12_e1083_d_n10 * s.v[834]) + (eq12_e1083 * s.dn[834][10]));let eq12_e1085_d_n11: f64 = ((eq12_e1083_d_n11 * s.v[834]) + (eq12_e1083 * s.dn[834][11]));let eq12_e1085_d_n12: f64 = ((eq12_e1083_d_n12 * s.v[834]) + (eq12_e1083 * s.dn[834][12]));let eq12_e1085_d_b0: f64 = ((eq12_e1083_d_b0 * s.v[834]) + (eq12_e1083 * s.db[834][0]));let eq12_e1085_d_b1: f64 = ((eq12_e1083_d_b1 * s.v[834]) + (eq12_e1083 * s.db[834][1]));
        let eq12_e1085_d_b2: f64 = ((eq12_e1083_d_b2 * s.v[834]) + (eq12_e1083 * s.db[834][2]));let eq12_e1085_d_b3: f64 = ((eq12_e1083_d_b3 * s.v[834]) + (eq12_e1083 * s.db[834][3]));let eq12_e1085_d_b4: f64 = ((eq12_e1083_d_b4 * s.v[834]) + (eq12_e1083 * s.db[834][4]));let eq12_e1085_d_b5: f64 = ((eq12_e1083_d_b5 * s.v[834]) + (eq12_e1083 * s.db[834][5]));let eq12_e1085_d_b6: f64 = ((eq12_e1083_d_b6 * s.v[834]) + (eq12_e1083 * s.db[834][6]));let eq12_value: f64 = eq12_e1085;let eq12_node_derivatives: [f64; 13] = [eq12_e1085_d_n0, eq12_e1085_d_n1, eq12_e1085_d_n2, eq12_e1085_d_n3, eq12_e1085_d_n4, eq12_e1085_d_n5, eq12_e1085_d_n6, eq12_e1085_d_n7, eq12_e1085_d_n8, eq12_e1085_d_n9, eq12_e1085_d_n10, eq12_e1085_d_n11, eq12_e1085_d_n12];let eq12_branch_derivatives: [f64; 7] = [eq12_e1085_d_b0, eq12_e1085_d_b1, eq12_e1085_d_b2, eq12_e1085_d_b3, eq12_e1085_d_b4, eq12_e1085_d_b5, eq12_e1085_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
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
        let eq13_e1088: f64 = (s.v[0] * s.v[15]);let eq13_e1088_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq13_e1088_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq13_e1088_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq13_e1088_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq13_e1088_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq13_e1088_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq13_e1088_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq13_e1088_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq13_e1088_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq13_e1088_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq13_e1088_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq13_e1088_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq13_e1088_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq13_e1088_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq13_e1088_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq13_e1088_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq13_e1088_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq13_e1088_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq13_e1088_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq13_e1088_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq13_e1090: f64 = (eq13_e1088 * p.p32);let eq13_e1090_d_n0: f64 = (eq13_e1088_d_n0 * p.p32);let eq13_e1090_d_n1: f64 = (eq13_e1088_d_n1 * p.p32);let eq13_e1090_d_n2: f64 = (eq13_e1088_d_n2 * p.p32);let eq13_e1090_d_n3: f64 = (eq13_e1088_d_n3 * p.p32);let eq13_e1090_d_n4: f64 = (eq13_e1088_d_n4 * p.p32);let eq13_e1090_d_n5: f64 = (eq13_e1088_d_n5 * p.p32);let eq13_e1090_d_n6: f64 = (eq13_e1088_d_n6 * p.p32);let eq13_e1090_d_n7: f64 = (eq13_e1088_d_n7 * p.p32);let eq13_e1090_d_n8: f64 = (eq13_e1088_d_n8 * p.p32);let eq13_e1090_d_n9: f64 = (eq13_e1088_d_n9 * p.p32);let eq13_e1090_d_n10: f64 = (eq13_e1088_d_n10 * p.p32);let eq13_e1090_d_n11: f64 = (eq13_e1088_d_n11 * p.p32);let eq13_e1090_d_n12: f64 = (eq13_e1088_d_n12 * p.p32);let eq13_e1090_d_b0: f64 = (eq13_e1088_d_b0 * p.p32);let eq13_e1090_d_b1: f64 = (eq13_e1088_d_b1 * p.p32);let eq13_e1090_d_b2: f64 = (eq13_e1088_d_b2 * p.p32);let eq13_e1090_d_b3: f64 = (eq13_e1088_d_b3 * p.p32);let eq13_e1090_d_b4: f64 = (eq13_e1088_d_b4 * p.p32);let eq13_e1090_d_b5: f64 = (eq13_e1088_d_b5 * p.p32);let eq13_e1090_d_b6: f64 = (eq13_e1088_d_b6 * p.p32);let eq13_e1092: f64 = (eq13_e1090 * s.v[837]);let eq13_e1092_d_n0: f64 = ((eq13_e1090_d_n0 * s.v[837]) + (eq13_e1090 * s.dn[837][0]));let eq13_e1092_d_n1: f64 = ((eq13_e1090_d_n1 * s.v[837]) + (eq13_e1090 * s.dn[837][1]));let eq13_e1092_d_n2: f64 = ((eq13_e1090_d_n2 * s.v[837]) + (eq13_e1090 * s.dn[837][2]));let eq13_e1092_d_n3: f64 = ((eq13_e1090_d_n3 * s.v[837]) + (eq13_e1090 * s.dn[837][3]));let eq13_e1092_d_n4: f64 = ((eq13_e1090_d_n4 * s.v[837]) + (eq13_e1090 * s.dn[837][4]));let eq13_e1092_d_n5: f64 = ((eq13_e1090_d_n5 * s.v[837]) + (eq13_e1090 * s.dn[837][5]));let eq13_e1092_d_n6: f64 = ((eq13_e1090_d_n6 * s.v[837]) + (eq13_e1090 * s.dn[837][6]));let eq13_e1092_d_n7: f64 = ((eq13_e1090_d_n7 * s.v[837]) + (eq13_e1090 * s.dn[837][7]));let eq13_e1092_d_n8: f64 = ((eq13_e1090_d_n8 * s.v[837]) + (eq13_e1090 * s.dn[837][8]));let eq13_e1092_d_n9: f64 = ((eq13_e1090_d_n9 * s.v[837]) + (eq13_e1090 * s.dn[837][9]));let eq13_e1092_d_n10: f64 = ((eq13_e1090_d_n10 * s.v[837]) + (eq13_e1090 * s.dn[837][10]));let eq13_e1092_d_n11: f64 = ((eq13_e1090_d_n11 * s.v[837]) + (eq13_e1090 * s.dn[837][11]));let eq13_e1092_d_n12: f64 = ((eq13_e1090_d_n12 * s.v[837]) + (eq13_e1090 * s.dn[837][12]));let eq13_e1092_d_b0: f64 = ((eq13_e1090_d_b0 * s.v[837]) + (eq13_e1090 * s.db[837][0]));let eq13_e1092_d_b1: f64 = ((eq13_e1090_d_b1 * s.v[837]) + (eq13_e1090 * s.db[837][1]));
        let eq13_e1092_d_b2: f64 = ((eq13_e1090_d_b2 * s.v[837]) + (eq13_e1090 * s.db[837][2]));let eq13_e1092_d_b3: f64 = ((eq13_e1090_d_b3 * s.v[837]) + (eq13_e1090 * s.db[837][3]));let eq13_e1092_d_b4: f64 = ((eq13_e1090_d_b4 * s.v[837]) + (eq13_e1090 * s.db[837][4]));let eq13_e1092_d_b5: f64 = ((eq13_e1090_d_b5 * s.v[837]) + (eq13_e1090 * s.db[837][5]));let eq13_e1092_d_b6: f64 = ((eq13_e1090_d_b6 * s.v[837]) + (eq13_e1090 * s.db[837][6]));let eq13_value: f64 = eq13_e1092;let eq13_node_derivatives: [f64; 13] = [eq13_e1092_d_n0, eq13_e1092_d_n1, eq13_e1092_d_n2, eq13_e1092_d_n3, eq13_e1092_d_n4, eq13_e1092_d_n5, eq13_e1092_d_n6, eq13_e1092_d_n7, eq13_e1092_d_n8, eq13_e1092_d_n9, eq13_e1092_d_n10, eq13_e1092_d_n11, eq13_e1092_d_n12];let eq13_branch_derivatives: [f64; 7] = [eq13_e1092_d_b0, eq13_e1092_d_b1, eq13_e1092_d_b2, eq13_e1092_d_b3, eq13_e1092_d_b4, eq13_e1092_d_b5, eq13_e1092_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
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
        let eq14_e1095: f64 = (s.v[0] * s.v[15]);let eq14_e1095_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq14_e1095_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq14_e1095_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq14_e1095_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq14_e1095_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq14_e1095_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq14_e1095_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq14_e1095_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq14_e1095_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq14_e1095_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq14_e1095_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq14_e1095_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq14_e1095_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq14_e1095_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq14_e1095_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq14_e1095_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq14_e1095_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq14_e1095_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq14_e1095_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq14_e1095_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq14_e1097: f64 = (eq14_e1095 * p.p32);let eq14_e1097_d_n0: f64 = (eq14_e1095_d_n0 * p.p32);let eq14_e1097_d_n1: f64 = (eq14_e1095_d_n1 * p.p32);let eq14_e1097_d_n2: f64 = (eq14_e1095_d_n2 * p.p32);let eq14_e1097_d_n3: f64 = (eq14_e1095_d_n3 * p.p32);let eq14_e1097_d_n4: f64 = (eq14_e1095_d_n4 * p.p32);let eq14_e1097_d_n5: f64 = (eq14_e1095_d_n5 * p.p32);let eq14_e1097_d_n6: f64 = (eq14_e1095_d_n6 * p.p32);let eq14_e1097_d_n7: f64 = (eq14_e1095_d_n7 * p.p32);let eq14_e1097_d_n8: f64 = (eq14_e1095_d_n8 * p.p32);let eq14_e1097_d_n9: f64 = (eq14_e1095_d_n9 * p.p32);let eq14_e1097_d_n10: f64 = (eq14_e1095_d_n10 * p.p32);let eq14_e1097_d_n11: f64 = (eq14_e1095_d_n11 * p.p32);let eq14_e1097_d_n12: f64 = (eq14_e1095_d_n12 * p.p32);let eq14_e1097_d_b0: f64 = (eq14_e1095_d_b0 * p.p32);let eq14_e1097_d_b1: f64 = (eq14_e1095_d_b1 * p.p32);let eq14_e1097_d_b2: f64 = (eq14_e1095_d_b2 * p.p32);let eq14_e1097_d_b3: f64 = (eq14_e1095_d_b3 * p.p32);let eq14_e1097_d_b4: f64 = (eq14_e1095_d_b4 * p.p32);let eq14_e1097_d_b5: f64 = (eq14_e1095_d_b5 * p.p32);let eq14_e1097_d_b6: f64 = (eq14_e1095_d_b6 * p.p32);let eq14_e1099: f64 = (eq14_e1097 * s.v[838]);let eq14_e1099_d_n0: f64 = ((eq14_e1097_d_n0 * s.v[838]) + (eq14_e1097 * s.dn[838][0]));let eq14_e1099_d_n1: f64 = ((eq14_e1097_d_n1 * s.v[838]) + (eq14_e1097 * s.dn[838][1]));let eq14_e1099_d_n2: f64 = ((eq14_e1097_d_n2 * s.v[838]) + (eq14_e1097 * s.dn[838][2]));let eq14_e1099_d_n3: f64 = ((eq14_e1097_d_n3 * s.v[838]) + (eq14_e1097 * s.dn[838][3]));let eq14_e1099_d_n4: f64 = ((eq14_e1097_d_n4 * s.v[838]) + (eq14_e1097 * s.dn[838][4]));let eq14_e1099_d_n5: f64 = ((eq14_e1097_d_n5 * s.v[838]) + (eq14_e1097 * s.dn[838][5]));let eq14_e1099_d_n6: f64 = ((eq14_e1097_d_n6 * s.v[838]) + (eq14_e1097 * s.dn[838][6]));let eq14_e1099_d_n7: f64 = ((eq14_e1097_d_n7 * s.v[838]) + (eq14_e1097 * s.dn[838][7]));let eq14_e1099_d_n8: f64 = ((eq14_e1097_d_n8 * s.v[838]) + (eq14_e1097 * s.dn[838][8]));let eq14_e1099_d_n9: f64 = ((eq14_e1097_d_n9 * s.v[838]) + (eq14_e1097 * s.dn[838][9]));let eq14_e1099_d_n10: f64 = ((eq14_e1097_d_n10 * s.v[838]) + (eq14_e1097 * s.dn[838][10]));let eq14_e1099_d_n11: f64 = ((eq14_e1097_d_n11 * s.v[838]) + (eq14_e1097 * s.dn[838][11]));let eq14_e1099_d_n12: f64 = ((eq14_e1097_d_n12 * s.v[838]) + (eq14_e1097 * s.dn[838][12]));let eq14_e1099_d_b0: f64 = ((eq14_e1097_d_b0 * s.v[838]) + (eq14_e1097 * s.db[838][0]));let eq14_e1099_d_b1: f64 = ((eq14_e1097_d_b1 * s.v[838]) + (eq14_e1097 * s.db[838][1]));
        let eq14_e1099_d_b2: f64 = ((eq14_e1097_d_b2 * s.v[838]) + (eq14_e1097 * s.db[838][2]));let eq14_e1099_d_b3: f64 = ((eq14_e1097_d_b3 * s.v[838]) + (eq14_e1097 * s.db[838][3]));let eq14_e1099_d_b4: f64 = ((eq14_e1097_d_b4 * s.v[838]) + (eq14_e1097 * s.db[838][4]));let eq14_e1099_d_b5: f64 = ((eq14_e1097_d_b5 * s.v[838]) + (eq14_e1097 * s.db[838][5]));let eq14_e1099_d_b6: f64 = ((eq14_e1097_d_b6 * s.v[838]) + (eq14_e1097 * s.db[838][6]));let eq14_value: f64 = eq14_e1099;let eq14_node_derivatives: [f64; 13] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12];let eq14_branch_derivatives: [f64; 7] = [eq14_e1099_d_b0, eq14_e1099_d_b1, eq14_e1099_d_b2, eq14_e1099_d_b3, eq14_e1099_d_b4, eq14_e1099_d_b5, eq14_e1099_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
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
        let nv1 = ctx.node_voltage(nodes[1]);let nv6 = ctx.node_voltage(nodes[6]);
        let (eq15_e1109, eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12, eq15_e1109_d_b0, eq15_e1109_d_b1, eq15_e1109_d_b2, eq15_e1109_d_b3, eq15_e1109_d_b4, eq15_e1109_d_b5, eq15_e1109_d_b6,) = {
    if s.b[2716] {
        let eq15_e1103: f64 = (s.v[15] * p.p32);let eq15_e1105: f64 = (eq15_e1103 * s.v[800]);let eq15_e1105_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][0]));let eq15_e1105_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][1]));let eq15_e1105_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][2]));let eq15_e1105_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][3]));let eq15_e1105_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][4]));let eq15_e1105_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][5]));let eq15_e1105_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][6]));let eq15_e1105_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][7]));let eq15_e1105_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][8]));let eq15_e1105_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][9]));let eq15_e1105_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][10]));let eq15_e1105_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][11]));let eq15_e1105_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[800]) + (eq15_e1103 * s.dn[800][12]));let eq15_e1105_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][0]));let eq15_e1105_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][1]));let eq15_e1105_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][2]));let eq15_e1105_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][3]));let eq15_e1105_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][4]));let eq15_e1105_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][5]));let eq15_e1105_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[800]) + (eq15_e1103 * s.db[800][6]));let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));let eq15_e1107_d_n0: f64 = (eq15_e1105_d_n0 * (nv1 - nv6));let eq15_e1107_d_n1: f64 = ((eq15_e1105_d_n1 * (nv1 - nv6)) + eq15_e1105);let eq15_e1107_d_n2: f64 = (eq15_e1105_d_n2 * (nv1 - nv6));let eq15_e1107_d_n3: f64 = (eq15_e1105_d_n3 * (nv1 - nv6));let eq15_e1107_d_n4: f64 = (eq15_e1105_d_n4 * (nv1 - nv6));let eq15_e1107_d_n5: f64 = (eq15_e1105_d_n5 * (nv1 - nv6));let eq15_e1107_d_n6: f64 = ((eq15_e1105_d_n6 * (nv1 - nv6)) + (-eq15_e1105));let eq15_e1107_d_n7: f64 = (eq15_e1105_d_n7 * (nv1 - nv6));let eq15_e1107_d_n8: f64 = (eq15_e1105_d_n8 * (nv1 - nv6));let eq15_e1107_d_n9: f64 = (eq15_e1105_d_n9 * (nv1 - nv6));let eq15_e1107_d_n10: f64 = (eq15_e1105_d_n10 * (nv1 - nv6));let eq15_e1107_d_n11: f64 = (eq15_e1105_d_n11 * (nv1 - nv6));let eq15_e1107_d_n12: f64 = (eq15_e1105_d_n12 * (nv1 - nv6));let eq15_e1107_d_b0: f64 = (eq15_e1105_d_b0 * (nv1 - nv6));let eq15_e1107_d_b1: f64 = (eq15_e1105_d_b1 * (nv1 - nv6));let eq15_e1107_d_b2: f64 = (eq15_e1105_d_b2 * (nv1 - nv6));let eq15_e1107_d_b3: f64 = (eq15_e1105_d_b3 * (nv1 - nv6));let eq15_e1107_d_b4: f64 = (eq15_e1105_d_b4 * (nv1 - nv6));let eq15_e1107_d_b5: f64 = (eq15_e1105_d_b5 * (nv1 - nv6));let eq15_e1107_d_b6: f64 = (eq15_e1105_d_b6 * (nv1 - nv6));
        (eq15_e1107, eq15_e1107_d_n0, eq15_e1107_d_n1, eq15_e1107_d_n2, eq15_e1107_d_n3, eq15_e1107_d_n4, eq15_e1107_d_n5, eq15_e1107_d_n6, eq15_e1107_d_n7, eq15_e1107_d_n8, eq15_e1107_d_n9, eq15_e1107_d_n10, eq15_e1107_d_n11, eq15_e1107_d_n12, eq15_e1107_d_b0, eq15_e1107_d_b1, eq15_e1107_d_b2, eq15_e1107_d_b3, eq15_e1107_d_b4, eq15_e1107_d_b5, eq15_e1107_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;let eq15_node_derivatives: [f64; 13] = [eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12];let eq15_branch_derivatives: [f64; 7] = [eq15_e1109_d_b0, eq15_e1109_d_b1, eq15_e1109_d_b2, eq15_e1109_d_b3, eq15_e1109_d_b4, eq15_e1109_d_b5, eq15_e1109_d_b6];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1124,) = {
    if (!s.b[2716]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
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
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);
        let (eq18_e1134, eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12, eq18_e1134_d_b0, eq18_e1134_d_b1, eq18_e1134_d_b2, eq18_e1134_d_b3, eq18_e1134_d_b4, eq18_e1134_d_b5, eq18_e1134_d_b6,) = {
    if s.b[2717] {
        let eq18_e1128: f64 = (s.v[15] * p.p32);let eq18_e1130: f64 = (eq18_e1128 * s.v[801]);let eq18_e1130_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][0]));let eq18_e1130_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][1]));let eq18_e1130_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][2]));let eq18_e1130_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][3]));let eq18_e1130_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][4]));let eq18_e1130_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][5]));let eq18_e1130_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][6]));let eq18_e1130_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][7]));let eq18_e1130_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][8]));let eq18_e1130_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][9]));let eq18_e1130_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][10]));let eq18_e1130_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][11]));let eq18_e1130_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[801]) + (eq18_e1128 * s.dn[801][12]));let eq18_e1130_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][0]));let eq18_e1130_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][1]));let eq18_e1130_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][2]));let eq18_e1130_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][3]));let eq18_e1130_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][4]));let eq18_e1130_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][5]));let eq18_e1130_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[801]) + (eq18_e1128 * s.db[801][6]));let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));let eq18_e1132_d_n0: f64 = (eq18_e1130_d_n0 * (nv2 - nv7));let eq18_e1132_d_n1: f64 = (eq18_e1130_d_n1 * (nv2 - nv7));let eq18_e1132_d_n2: f64 = ((eq18_e1130_d_n2 * (nv2 - nv7)) + eq18_e1130);let eq18_e1132_d_n3: f64 = (eq18_e1130_d_n3 * (nv2 - nv7));let eq18_e1132_d_n4: f64 = (eq18_e1130_d_n4 * (nv2 - nv7));let eq18_e1132_d_n5: f64 = (eq18_e1130_d_n5 * (nv2 - nv7));let eq18_e1132_d_n6: f64 = (eq18_e1130_d_n6 * (nv2 - nv7));let eq18_e1132_d_n7: f64 = ((eq18_e1130_d_n7 * (nv2 - nv7)) + (-eq18_e1130));let eq18_e1132_d_n8: f64 = (eq18_e1130_d_n8 * (nv2 - nv7));let eq18_e1132_d_n9: f64 = (eq18_e1130_d_n9 * (nv2 - nv7));let eq18_e1132_d_n10: f64 = (eq18_e1130_d_n10 * (nv2 - nv7));let eq18_e1132_d_n11: f64 = (eq18_e1130_d_n11 * (nv2 - nv7));let eq18_e1132_d_n12: f64 = (eq18_e1130_d_n12 * (nv2 - nv7));let eq18_e1132_d_b0: f64 = (eq18_e1130_d_b0 * (nv2 - nv7));let eq18_e1132_d_b1: f64 = (eq18_e1130_d_b1 * (nv2 - nv7));let eq18_e1132_d_b2: f64 = (eq18_e1130_d_b2 * (nv2 - nv7));let eq18_e1132_d_b3: f64 = (eq18_e1130_d_b3 * (nv2 - nv7));let eq18_e1132_d_b4: f64 = (eq18_e1130_d_b4 * (nv2 - nv7));let eq18_e1132_d_b5: f64 = (eq18_e1130_d_b5 * (nv2 - nv7));let eq18_e1132_d_b6: f64 = (eq18_e1130_d_b6 * (nv2 - nv7));
        (eq18_e1132, eq18_e1132_d_n0, eq18_e1132_d_n1, eq18_e1132_d_n2, eq18_e1132_d_n3, eq18_e1132_d_n4, eq18_e1132_d_n5, eq18_e1132_d_n6, eq18_e1132_d_n7, eq18_e1132_d_n8, eq18_e1132_d_n9, eq18_e1132_d_n10, eq18_e1132_d_n11, eq18_e1132_d_n12, eq18_e1132_d_b0, eq18_e1132_d_b1, eq18_e1132_d_b2, eq18_e1132_d_b3, eq18_e1132_d_b4, eq18_e1132_d_b5, eq18_e1132_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;let eq18_node_derivatives: [f64; 13] = [eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12];let eq18_branch_derivatives: [f64; 7] = [eq18_e1134_d_b0, eq18_e1134_d_b1, eq18_e1134_d_b2, eq18_e1134_d_b3, eq18_e1134_d_b4, eq18_e1134_d_b5, eq18_e1134_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1149,) = {
    if (!s.b[2717]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
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
        let nv0 = ctx.node_voltage(nodes[0]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12, eq21_e1159_d_b0, eq21_e1159_d_b1, eq21_e1159_d_b2, eq21_e1159_d_b3, eq21_e1159_d_b4, eq21_e1159_d_b5, eq21_e1159_d_b6,) = {
    if s.b[2718] {
        let eq21_e1153: f64 = (s.v[15] * p.p32);let eq21_e1155: f64 = (eq21_e1153 * s.v[802]);let eq21_e1155_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][0]));let eq21_e1155_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][1]));let eq21_e1155_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][2]));let eq21_e1155_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][3]));let eq21_e1155_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][4]));let eq21_e1155_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][5]));let eq21_e1155_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][6]));let eq21_e1155_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][7]));let eq21_e1155_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][8]));let eq21_e1155_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][9]));let eq21_e1155_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][10]));let eq21_e1155_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][11]));let eq21_e1155_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[802]) + (eq21_e1153 * s.dn[802][12]));let eq21_e1155_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][0]));let eq21_e1155_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][1]));let eq21_e1155_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][2]));let eq21_e1155_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][3]));let eq21_e1155_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][4]));let eq21_e1155_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][5]));let eq21_e1155_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[802]) + (eq21_e1153 * s.db[802][6]));let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));let eq21_e1157_d_n0: f64 = ((eq21_e1155_d_n0 * (nv0 - nv8)) + eq21_e1155);let eq21_e1157_d_n1: f64 = (eq21_e1155_d_n1 * (nv0 - nv8));let eq21_e1157_d_n2: f64 = (eq21_e1155_d_n2 * (nv0 - nv8));let eq21_e1157_d_n3: f64 = (eq21_e1155_d_n3 * (nv0 - nv8));let eq21_e1157_d_n4: f64 = (eq21_e1155_d_n4 * (nv0 - nv8));let eq21_e1157_d_n5: f64 = (eq21_e1155_d_n5 * (nv0 - nv8));let eq21_e1157_d_n6: f64 = (eq21_e1155_d_n6 * (nv0 - nv8));let eq21_e1157_d_n7: f64 = (eq21_e1155_d_n7 * (nv0 - nv8));let eq21_e1157_d_n8: f64 = ((eq21_e1155_d_n8 * (nv0 - nv8)) + (-eq21_e1155));let eq21_e1157_d_n9: f64 = (eq21_e1155_d_n9 * (nv0 - nv8));let eq21_e1157_d_n10: f64 = (eq21_e1155_d_n10 * (nv0 - nv8));let eq21_e1157_d_n11: f64 = (eq21_e1155_d_n11 * (nv0 - nv8));let eq21_e1157_d_n12: f64 = (eq21_e1155_d_n12 * (nv0 - nv8));let eq21_e1157_d_b0: f64 = (eq21_e1155_d_b0 * (nv0 - nv8));let eq21_e1157_d_b1: f64 = (eq21_e1155_d_b1 * (nv0 - nv8));let eq21_e1157_d_b2: f64 = (eq21_e1155_d_b2 * (nv0 - nv8));let eq21_e1157_d_b3: f64 = (eq21_e1155_d_b3 * (nv0 - nv8));let eq21_e1157_d_b4: f64 = (eq21_e1155_d_b4 * (nv0 - nv8));let eq21_e1157_d_b5: f64 = (eq21_e1155_d_b5 * (nv0 - nv8));let eq21_e1157_d_b6: f64 = (eq21_e1155_d_b6 * (nv0 - nv8));
        (eq21_e1157, eq21_e1157_d_n0, eq21_e1157_d_n1, eq21_e1157_d_n2, eq21_e1157_d_n3, eq21_e1157_d_n4, eq21_e1157_d_n5, eq21_e1157_d_n6, eq21_e1157_d_n7, eq21_e1157_d_n8, eq21_e1157_d_n9, eq21_e1157_d_n10, eq21_e1157_d_n11, eq21_e1157_d_n12, eq21_e1157_d_b0, eq21_e1157_d_b1, eq21_e1157_d_b2, eq21_e1157_d_b3, eq21_e1157_d_b4, eq21_e1157_d_b5, eq21_e1157_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;let eq21_node_derivatives: [f64; 13] = [eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12];let eq21_branch_derivatives: [f64; 7] = [eq21_e1159_d_b0, eq21_e1159_d_b1, eq21_e1159_d_b2, eq21_e1159_d_b3, eq21_e1159_d_b4, eq21_e1159_d_b5, eq21_e1159_d_b6];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1174,) = {
    if (!s.b[2718]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
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
        let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq24_e1184, eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12, eq24_e1184_d_b0, eq24_e1184_d_b1, eq24_e1184_d_b2, eq24_e1184_d_b3, eq24_e1184_d_b4, eq24_e1184_d_b5, eq24_e1184_d_b6,) = {
    if s.b[2719] {
        let eq24_e1178: f64 = (s.v[15] * p.p32);let eq24_e1180: f64 = (eq24_e1178 * s.v[803]);let eq24_e1180_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][0]));let eq24_e1180_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][1]));let eq24_e1180_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][2]));let eq24_e1180_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][3]));let eq24_e1180_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][4]));let eq24_e1180_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][5]));let eq24_e1180_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][6]));let eq24_e1180_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][7]));let eq24_e1180_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][8]));let eq24_e1180_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][9]));let eq24_e1180_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][10]));let eq24_e1180_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][11]));let eq24_e1180_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[803]) + (eq24_e1178 * s.dn[803][12]));let eq24_e1180_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][0]));let eq24_e1180_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][1]));let eq24_e1180_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][2]));let eq24_e1180_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][3]));let eq24_e1180_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][4]));let eq24_e1180_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][5]));let eq24_e1180_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[803]) + (eq24_e1178 * s.db[803][6]));let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));let eq24_e1182_d_n0: f64 = (eq24_e1180_d_n0 * (nv9 - nv10));let eq24_e1182_d_n1: f64 = (eq24_e1180_d_n1 * (nv9 - nv10));let eq24_e1182_d_n2: f64 = (eq24_e1180_d_n2 * (nv9 - nv10));let eq24_e1182_d_n3: f64 = (eq24_e1180_d_n3 * (nv9 - nv10));let eq24_e1182_d_n4: f64 = (eq24_e1180_d_n4 * (nv9 - nv10));let eq24_e1182_d_n5: f64 = (eq24_e1180_d_n5 * (nv9 - nv10));let eq24_e1182_d_n6: f64 = (eq24_e1180_d_n6 * (nv9 - nv10));let eq24_e1182_d_n7: f64 = (eq24_e1180_d_n7 * (nv9 - nv10));let eq24_e1182_d_n8: f64 = (eq24_e1180_d_n8 * (nv9 - nv10));let eq24_e1182_d_n9: f64 = ((eq24_e1180_d_n9 * (nv9 - nv10)) + eq24_e1180);let eq24_e1182_d_n10: f64 = ((eq24_e1180_d_n10 * (nv9 - nv10)) + (-eq24_e1180));let eq24_e1182_d_n11: f64 = (eq24_e1180_d_n11 * (nv9 - nv10));let eq24_e1182_d_n12: f64 = (eq24_e1180_d_n12 * (nv9 - nv10));let eq24_e1182_d_b0: f64 = (eq24_e1180_d_b0 * (nv9 - nv10));let eq24_e1182_d_b1: f64 = (eq24_e1180_d_b1 * (nv9 - nv10));let eq24_e1182_d_b2: f64 = (eq24_e1180_d_b2 * (nv9 - nv10));let eq24_e1182_d_b3: f64 = (eq24_e1180_d_b3 * (nv9 - nv10));let eq24_e1182_d_b4: f64 = (eq24_e1180_d_b4 * (nv9 - nv10));let eq24_e1182_d_b5: f64 = (eq24_e1180_d_b5 * (nv9 - nv10));let eq24_e1182_d_b6: f64 = (eq24_e1180_d_b6 * (nv9 - nv10));
        (eq24_e1182, eq24_e1182_d_n0, eq24_e1182_d_n1, eq24_e1182_d_n2, eq24_e1182_d_n3, eq24_e1182_d_n4, eq24_e1182_d_n5, eq24_e1182_d_n6, eq24_e1182_d_n7, eq24_e1182_d_n8, eq24_e1182_d_n9, eq24_e1182_d_n10, eq24_e1182_d_n11, eq24_e1182_d_n12, eq24_e1182_d_b0, eq24_e1182_d_b1, eq24_e1182_d_b2, eq24_e1182_d_b3, eq24_e1182_d_b4, eq24_e1182_d_b5, eq24_e1182_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;let eq24_node_derivatives: [f64; 13] = [eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12];let eq24_branch_derivatives: [f64; 7] = [eq24_e1184_d_b0, eq24_e1184_d_b1, eq24_e1184_d_b2, eq24_e1184_d_b3, eq24_e1184_d_b4, eq24_e1184_d_b5, eq24_e1184_d_b6];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1199,) = {
    if (!s.b[2719]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
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
        let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq27_e1209, eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12, eq27_e1209_d_b0, eq27_e1209_d_b1, eq27_e1209_d_b2, eq27_e1209_d_b3, eq27_e1209_d_b4, eq27_e1209_d_b5, eq27_e1209_d_b6,) = {
    if s.b[2720] {
        let eq27_e1203: f64 = (s.v[15] * p.p32);let eq27_e1205: f64 = (eq27_e1203 * s.v[804]);let eq27_e1205_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][0]));let eq27_e1205_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][1]));let eq27_e1205_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][2]));let eq27_e1205_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][3]));let eq27_e1205_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][4]));let eq27_e1205_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][5]));let eq27_e1205_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][6]));let eq27_e1205_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][7]));let eq27_e1205_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][8]));let eq27_e1205_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][9]));let eq27_e1205_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][10]));let eq27_e1205_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][11]));let eq27_e1205_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[804]) + (eq27_e1203 * s.dn[804][12]));let eq27_e1205_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][0]));let eq27_e1205_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][1]));let eq27_e1205_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][2]));let eq27_e1205_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][3]));let eq27_e1205_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][4]));let eq27_e1205_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][5]));let eq27_e1205_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[804]) + (eq27_e1203 * s.db[804][6]));let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));let eq27_e1207_d_n0: f64 = (eq27_e1205_d_n0 * (nv11 - nv10));let eq27_e1207_d_n1: f64 = (eq27_e1205_d_n1 * (nv11 - nv10));let eq27_e1207_d_n2: f64 = (eq27_e1205_d_n2 * (nv11 - nv10));let eq27_e1207_d_n3: f64 = (eq27_e1205_d_n3 * (nv11 - nv10));let eq27_e1207_d_n4: f64 = (eq27_e1205_d_n4 * (nv11 - nv10));let eq27_e1207_d_n5: f64 = (eq27_e1205_d_n5 * (nv11 - nv10));let eq27_e1207_d_n6: f64 = (eq27_e1205_d_n6 * (nv11 - nv10));let eq27_e1207_d_n7: f64 = (eq27_e1205_d_n7 * (nv11 - nv10));let eq27_e1207_d_n8: f64 = (eq27_e1205_d_n8 * (nv11 - nv10));let eq27_e1207_d_n9: f64 = (eq27_e1205_d_n9 * (nv11 - nv10));let eq27_e1207_d_n10: f64 = ((eq27_e1205_d_n10 * (nv11 - nv10)) + (-eq27_e1205));let eq27_e1207_d_n11: f64 = ((eq27_e1205_d_n11 * (nv11 - nv10)) + eq27_e1205);let eq27_e1207_d_n12: f64 = (eq27_e1205_d_n12 * (nv11 - nv10));let eq27_e1207_d_b0: f64 = (eq27_e1205_d_b0 * (nv11 - nv10));let eq27_e1207_d_b1: f64 = (eq27_e1205_d_b1 * (nv11 - nv10));let eq27_e1207_d_b2: f64 = (eq27_e1205_d_b2 * (nv11 - nv10));let eq27_e1207_d_b3: f64 = (eq27_e1205_d_b3 * (nv11 - nv10));let eq27_e1207_d_b4: f64 = (eq27_e1205_d_b4 * (nv11 - nv10));let eq27_e1207_d_b5: f64 = (eq27_e1205_d_b5 * (nv11 - nv10));let eq27_e1207_d_b6: f64 = (eq27_e1205_d_b6 * (nv11 - nv10));
        (eq27_e1207, eq27_e1207_d_n0, eq27_e1207_d_n1, eq27_e1207_d_n2, eq27_e1207_d_n3, eq27_e1207_d_n4, eq27_e1207_d_n5, eq27_e1207_d_n6, eq27_e1207_d_n7, eq27_e1207_d_n8, eq27_e1207_d_n9, eq27_e1207_d_n10, eq27_e1207_d_n11, eq27_e1207_d_n12, eq27_e1207_d_b0, eq27_e1207_d_b1, eq27_e1207_d_b2, eq27_e1207_d_b3, eq27_e1207_d_b4, eq27_e1207_d_b5, eq27_e1207_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;let eq27_node_derivatives: [f64; 13] = [eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12];let eq27_branch_derivatives: [f64; 7] = [eq27_e1209_d_b0, eq27_e1209_d_b1, eq27_e1209_d_b2, eq27_e1209_d_b3, eq27_e1209_d_b4, eq27_e1209_d_b5, eq27_e1209_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1224,) = {
    if (!s.b[2720]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
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
        let nv10 = ctx.node_voltage(nodes[10]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq30_e1234, eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12, eq30_e1234_d_b0, eq30_e1234_d_b1, eq30_e1234_d_b2, eq30_e1234_d_b3, eq30_e1234_d_b4, eq30_e1234_d_b5, eq30_e1234_d_b6,) = {
    if s.b[2721] {
        let eq30_e1228: f64 = (s.v[15] * p.p32);let eq30_e1230: f64 = (eq30_e1228 * s.v[805]);let eq30_e1230_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][0]));let eq30_e1230_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][1]));let eq30_e1230_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][2]));let eq30_e1230_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][3]));let eq30_e1230_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][4]));let eq30_e1230_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][5]));let eq30_e1230_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][6]));let eq30_e1230_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][7]));let eq30_e1230_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][8]));let eq30_e1230_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][9]));let eq30_e1230_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][10]));let eq30_e1230_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][11]));let eq30_e1230_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[805]) + (eq30_e1228 * s.dn[805][12]));let eq30_e1230_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][0]));let eq30_e1230_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][1]));let eq30_e1230_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][2]));let eq30_e1230_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][3]));let eq30_e1230_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][4]));let eq30_e1230_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][5]));let eq30_e1230_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[805]) + (eq30_e1228 * s.db[805][6]));let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));let eq30_e1232_d_n0: f64 = (eq30_e1230_d_n0 * (nv12 - nv10));let eq30_e1232_d_n1: f64 = (eq30_e1230_d_n1 * (nv12 - nv10));let eq30_e1232_d_n2: f64 = (eq30_e1230_d_n2 * (nv12 - nv10));let eq30_e1232_d_n3: f64 = (eq30_e1230_d_n3 * (nv12 - nv10));let eq30_e1232_d_n4: f64 = (eq30_e1230_d_n4 * (nv12 - nv10));let eq30_e1232_d_n5: f64 = (eq30_e1230_d_n5 * (nv12 - nv10));let eq30_e1232_d_n6: f64 = (eq30_e1230_d_n6 * (nv12 - nv10));let eq30_e1232_d_n7: f64 = (eq30_e1230_d_n7 * (nv12 - nv10));let eq30_e1232_d_n8: f64 = (eq30_e1230_d_n8 * (nv12 - nv10));let eq30_e1232_d_n9: f64 = (eq30_e1230_d_n9 * (nv12 - nv10));let eq30_e1232_d_n10: f64 = ((eq30_e1230_d_n10 * (nv12 - nv10)) + (-eq30_e1230));let eq30_e1232_d_n11: f64 = (eq30_e1230_d_n11 * (nv12 - nv10));let eq30_e1232_d_n12: f64 = ((eq30_e1230_d_n12 * (nv12 - nv10)) + eq30_e1230);let eq30_e1232_d_b0: f64 = (eq30_e1230_d_b0 * (nv12 - nv10));let eq30_e1232_d_b1: f64 = (eq30_e1230_d_b1 * (nv12 - nv10));let eq30_e1232_d_b2: f64 = (eq30_e1230_d_b2 * (nv12 - nv10));let eq30_e1232_d_b3: f64 = (eq30_e1230_d_b3 * (nv12 - nv10));let eq30_e1232_d_b4: f64 = (eq30_e1230_d_b4 * (nv12 - nv10));let eq30_e1232_d_b5: f64 = (eq30_e1230_d_b5 * (nv12 - nv10));let eq30_e1232_d_b6: f64 = (eq30_e1230_d_b6 * (nv12 - nv10));
        (eq30_e1232, eq30_e1232_d_n0, eq30_e1232_d_n1, eq30_e1232_d_n2, eq30_e1232_d_n3, eq30_e1232_d_n4, eq30_e1232_d_n5, eq30_e1232_d_n6, eq30_e1232_d_n7, eq30_e1232_d_n8, eq30_e1232_d_n9, eq30_e1232_d_n10, eq30_e1232_d_n11, eq30_e1232_d_n12, eq30_e1232_d_b0, eq30_e1232_d_b1, eq30_e1232_d_b2, eq30_e1232_d_b3, eq30_e1232_d_b4, eq30_e1232_d_b5, eq30_e1232_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;let eq30_node_derivatives: [f64; 13] = [eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12];let eq30_branch_derivatives: [f64; 7] = [eq30_e1234_d_b0, eq30_e1234_d_b1, eq30_e1234_d_b2, eq30_e1234_d_b3, eq30_e1234_d_b4, eq30_e1234_d_b5, eq30_e1234_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1249,) = {
    if (!s.b[2721]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq33_e1259, eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12, eq33_e1259_d_b0, eq33_e1259_d_b1, eq33_e1259_d_b2, eq33_e1259_d_b3, eq33_e1259_d_b4, eq33_e1259_d_b5, eq33_e1259_d_b6,) = {
    if s.b[2722] {
        let eq33_e1253: f64 = (s.v[15] * p.p32);let eq33_e1255: f64 = (eq33_e1253 * s.v[806]);let eq33_e1255_d_n0: f64 = (((s.dn[15][0] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][0]));let eq33_e1255_d_n1: f64 = (((s.dn[15][1] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][1]));let eq33_e1255_d_n2: f64 = (((s.dn[15][2] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][2]));let eq33_e1255_d_n3: f64 = (((s.dn[15][3] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][3]));let eq33_e1255_d_n4: f64 = (((s.dn[15][4] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][4]));let eq33_e1255_d_n5: f64 = (((s.dn[15][5] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][5]));let eq33_e1255_d_n6: f64 = (((s.dn[15][6] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][6]));let eq33_e1255_d_n7: f64 = (((s.dn[15][7] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][7]));let eq33_e1255_d_n8: f64 = (((s.dn[15][8] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][8]));let eq33_e1255_d_n9: f64 = (((s.dn[15][9] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][9]));let eq33_e1255_d_n10: f64 = (((s.dn[15][10] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][10]));let eq33_e1255_d_n11: f64 = (((s.dn[15][11] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][11]));let eq33_e1255_d_n12: f64 = (((s.dn[15][12] * p.p32) * s.v[806]) + (eq33_e1253 * s.dn[806][12]));let eq33_e1255_d_b0: f64 = (((s.db[15][0] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][0]));let eq33_e1255_d_b1: f64 = (((s.db[15][1] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][1]));let eq33_e1255_d_b2: f64 = (((s.db[15][2] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][2]));let eq33_e1255_d_b3: f64 = (((s.db[15][3] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][3]));let eq33_e1255_d_b4: f64 = (((s.db[15][4] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][4]));let eq33_e1255_d_b5: f64 = (((s.db[15][5] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][5]));let eq33_e1255_d_b6: f64 = (((s.db[15][6] * p.p32) * s.v[806]) + (eq33_e1253 * s.db[806][6]));let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));let eq33_e1257_d_n0: f64 = (eq33_e1255_d_n0 * (nv3 - nv10));let eq33_e1257_d_n1: f64 = (eq33_e1255_d_n1 * (nv3 - nv10));let eq33_e1257_d_n2: f64 = (eq33_e1255_d_n2 * (nv3 - nv10));let eq33_e1257_d_n3: f64 = ((eq33_e1255_d_n3 * (nv3 - nv10)) + eq33_e1255);let eq33_e1257_d_n4: f64 = (eq33_e1255_d_n4 * (nv3 - nv10));let eq33_e1257_d_n5: f64 = (eq33_e1255_d_n5 * (nv3 - nv10));let eq33_e1257_d_n6: f64 = (eq33_e1255_d_n6 * (nv3 - nv10));let eq33_e1257_d_n7: f64 = (eq33_e1255_d_n7 * (nv3 - nv10));let eq33_e1257_d_n8: f64 = (eq33_e1255_d_n8 * (nv3 - nv10));let eq33_e1257_d_n9: f64 = (eq33_e1255_d_n9 * (nv3 - nv10));let eq33_e1257_d_n10: f64 = ((eq33_e1255_d_n10 * (nv3 - nv10)) + (-eq33_e1255));let eq33_e1257_d_n11: f64 = (eq33_e1255_d_n11 * (nv3 - nv10));let eq33_e1257_d_n12: f64 = (eq33_e1255_d_n12 * (nv3 - nv10));let eq33_e1257_d_b0: f64 = (eq33_e1255_d_b0 * (nv3 - nv10));let eq33_e1257_d_b1: f64 = (eq33_e1255_d_b1 * (nv3 - nv10));let eq33_e1257_d_b2: f64 = (eq33_e1255_d_b2 * (nv3 - nv10));let eq33_e1257_d_b3: f64 = (eq33_e1255_d_b3 * (nv3 - nv10));let eq33_e1257_d_b4: f64 = (eq33_e1255_d_b4 * (nv3 - nv10));let eq33_e1257_d_b5: f64 = (eq33_e1255_d_b5 * (nv3 - nv10));let eq33_e1257_d_b6: f64 = (eq33_e1255_d_b6 * (nv3 - nv10));
        (eq33_e1257, eq33_e1257_d_n0, eq33_e1257_d_n1, eq33_e1257_d_n2, eq33_e1257_d_n3, eq33_e1257_d_n4, eq33_e1257_d_n5, eq33_e1257_d_n6, eq33_e1257_d_n7, eq33_e1257_d_n8, eq33_e1257_d_n9, eq33_e1257_d_n10, eq33_e1257_d_n11, eq33_e1257_d_n12, eq33_e1257_d_b0, eq33_e1257_d_b1, eq33_e1257_d_b2, eq33_e1257_d_b3, eq33_e1257_d_b4, eq33_e1257_d_b5, eq33_e1257_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;let eq33_node_derivatives: [f64; 13] = [eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12];let eq33_branch_derivatives: [f64; 7] = [eq33_e1259_d_b0, eq33_e1259_d_b1, eq33_e1259_d_b2, eq33_e1259_d_b3, eq33_e1259_d_b4, eq33_e1259_d_b5, eq33_e1259_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq35_e1274,) = {
    if (!s.b[2722]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );let eq36_e1277: f64 = (p.p32 * s.v[867]);let eq36_e1279: f64 = (eq36_e1277 * (nv8 - nv9));let eq36_value: f64 = eq36_e1279;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq36_value),
            8,
            multiplicity * (eq36_e1277),
            9,
            multiplicity * ((-eq36_e1277)),
        );let eq37_e1282: f64 = (p.p32 * s.v[867]);let eq37_e1284: f64 = (eq37_e1282 * (nv7 - nv9));let eq37_value: f64 = eq37_e1284;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            7,
            multiplicity * (eq37_e1282),
            9,
            multiplicity * ((-eq37_e1282)),
        );let eq38_e1286: f64 = (-s.v[15]);let eq38_e1288: f64 = (eq38_e1286 * s.v[1915]);let eq38_e1288_d_n0: f64 = (((-s.dn[15][0]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][0]));let eq38_e1288_d_n1: f64 = (((-s.dn[15][1]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][1]));let eq38_e1288_d_n2: f64 = (((-s.dn[15][2]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][2]));let eq38_e1288_d_n3: f64 = (((-s.dn[15][3]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][3]));let eq38_e1288_d_n4: f64 = (((-s.dn[15][4]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][4]));let eq38_e1288_d_n5: f64 = (((-s.dn[15][5]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][5]));let eq38_e1288_d_n6: f64 = (((-s.dn[15][6]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][6]));let eq38_e1288_d_n7: f64 = (((-s.dn[15][7]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][7]));let eq38_e1288_d_n8: f64 = (((-s.dn[15][8]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][8]));let eq38_e1288_d_n9: f64 = (((-s.dn[15][9]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][9]));let eq38_e1288_d_n10: f64 = (((-s.dn[15][10]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][10]));let eq38_e1288_d_n11: f64 = (((-s.dn[15][11]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][11]));let eq38_e1288_d_n12: f64 = (((-s.dn[15][12]) * s.v[1915]) + (eq38_e1286 * s.dn[1915][12]));let eq38_e1288_d_b0: f64 = (((-s.db[15][0]) * s.v[1915]) + (eq38_e1286 * s.db[1915][0]));let eq38_e1288_d_b1: f64 = (((-s.db[15][1]) * s.v[1915]) + (eq38_e1286 * s.db[1915][1]));let eq38_e1288_d_b2: f64 = (((-s.db[15][2]) * s.v[1915]) + (eq38_e1286 * s.db[1915][2]));let eq38_e1288_d_b3: f64 = (((-s.db[15][3]) * s.v[1915]) + (eq38_e1286 * s.db[1915][3]));let eq38_e1288_d_b4: f64 = (((-s.db[15][4]) * s.v[1915]) + (eq38_e1286 * s.db[1915][4]));let eq38_e1288_d_b5: f64 = (((-s.db[15][5]) * s.v[1915]) + (eq38_e1286 * s.db[1915][5]));let eq38_e1288_d_b6: f64 = (((-s.db[15][6]) * s.v[1915]) + (eq38_e1286 * s.db[1915][6]));let eq38_value: f64 = eq38_e1288;let eq38_node_derivatives: [f64; 13] = [eq38_e1288_d_n0, eq38_e1288_d_n1, eq38_e1288_d_n2, eq38_e1288_d_n3, eq38_e1288_d_n4, eq38_e1288_d_n5, eq38_e1288_d_n6, eq38_e1288_d_n7, eq38_e1288_d_n8, eq38_e1288_d_n9, eq38_e1288_d_n10, eq38_e1288_d_n11, eq38_e1288_d_n12];let eq38_branch_derivatives: [f64; 7] = [eq38_e1288_d_b0, eq38_e1288_d_b1, eq38_e1288_d_b2, eq38_e1288_d_b3, eq38_e1288_d_b4, eq38_e1288_d_b5, eq38_e1288_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_22(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let nv4 = ctx.node_voltage(nodes[4]);let eq39_e1291: f64 = (s.v[15] * s.v[306]);let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));let eq39_e1291_d_b0: f64 = ((s.db[15][0] * s.v[306]) + (s.v[15] * s.db[306][0]));let eq39_e1291_d_b1: f64 = ((s.db[15][1] * s.v[306]) + (s.v[15] * s.db[306][1]));let eq39_e1291_d_b2: f64 = ((s.db[15][2] * s.v[306]) + (s.v[15] * s.db[306][2]));let eq39_e1291_d_b3: f64 = ((s.db[15][3] * s.v[306]) + (s.v[15] * s.db[306][3]));let eq39_e1291_d_b4: f64 = ((s.db[15][4] * s.v[306]) + (s.v[15] * s.db[306][4]));let eq39_e1291_d_b5: f64 = ((s.db[15][5] * s.v[306]) + (s.v[15] * s.db[306][5]));let eq39_e1291_d_b6: f64 = ((s.db[15][6] * s.v[306]) + (s.v[15] * s.db[306][6]));let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));let eq39_e1293_d_b0: f64 = (eq39_e1291_d_b0 * (nv4 - 0.0));let eq39_e1293_d_b1: f64 = (eq39_e1291_d_b1 * (nv4 - 0.0));let eq39_e1293_d_b2: f64 = (eq39_e1291_d_b2 * (nv4 - 0.0));let eq39_e1293_d_b3: f64 = (eq39_e1291_d_b3 * (nv4 - 0.0));let eq39_e1293_d_b4: f64 = (eq39_e1291_d_b4 * (nv4 - 0.0));let eq39_e1293_d_b5: f64 = (eq39_e1291_d_b5 * (nv4 - 0.0));let eq39_e1293_d_b6: f64 = (eq39_e1291_d_b6 * (nv4 - 0.0));let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);let eq39_value: f64 = eq39_e1294;let eq39_node_derivatives: [f64; 13] = [(eq39_e1293_d_n0 * ddt_scale), (eq39_e1293_d_n1 * ddt_scale), (eq39_e1293_d_n2 * ddt_scale), (eq39_e1293_d_n3 * ddt_scale), (eq39_e1293_d_n4 * ddt_scale), (eq39_e1293_d_n5 * ddt_scale), (eq39_e1293_d_n6 * ddt_scale), (eq39_e1293_d_n7 * ddt_scale), (eq39_e1293_d_n8 * ddt_scale), (eq39_e1293_d_n9 * ddt_scale), (eq39_e1293_d_n10 * ddt_scale), (eq39_e1293_d_n11 * ddt_scale), (eq39_e1293_d_n12 * ddt_scale)];let eq39_branch_derivatives: [f64; 7] = [(eq39_e1293_d_b0 * ddt_scale), (eq39_e1293_d_b1 * ddt_scale), (eq39_e1293_d_b2 * ddt_scale), (eq39_e1293_d_b3 * ddt_scale), (eq39_e1293_d_b4 * ddt_scale), (eq39_e1293_d_b5 * ddt_scale), (eq39_e1293_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_23(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let eq40_e1297: f64 = (s.v[15] * (nv4 - 0.0));let eq40_e1297_d_n0: f64 = (s.dn[15][0] * (nv4 - 0.0));let eq40_e1297_d_n1: f64 = (s.dn[15][1] * (nv4 - 0.0));let eq40_e1297_d_n2: f64 = (s.dn[15][2] * (nv4 - 0.0));let eq40_e1297_d_n3: f64 = (s.dn[15][3] * (nv4 - 0.0));let eq40_e1297_d_n4: f64 = ((s.dn[15][4] * (nv4 - 0.0)) + s.v[15]);let eq40_e1297_d_n5: f64 = (s.dn[15][5] * (nv4 - 0.0));let eq40_e1297_d_n6: f64 = (s.dn[15][6] * (nv4 - 0.0));let eq40_e1297_d_n7: f64 = (s.dn[15][7] * (nv4 - 0.0));let eq40_e1297_d_n8: f64 = (s.dn[15][8] * (nv4 - 0.0));let eq40_e1297_d_n9: f64 = (s.dn[15][9] * (nv4 - 0.0));let eq40_e1297_d_n10: f64 = (s.dn[15][10] * (nv4 - 0.0));let eq40_e1297_d_n11: f64 = (s.dn[15][11] * (nv4 - 0.0));let eq40_e1297_d_n12: f64 = (s.dn[15][12] * (nv4 - 0.0));let eq40_e1297_d_b0: f64 = (s.db[15][0] * (nv4 - 0.0));let eq40_e1297_d_b1: f64 = (s.db[15][1] * (nv4 - 0.0));let eq40_e1297_d_b2: f64 = (s.db[15][2] * (nv4 - 0.0));let eq40_e1297_d_b3: f64 = (s.db[15][3] * (nv4 - 0.0));let eq40_e1297_d_b4: f64 = (s.db[15][4] * (nv4 - 0.0));let eq40_e1297_d_b5: f64 = (s.db[15][5] * (nv4 - 0.0));let eq40_e1297_d_b6: f64 = (s.db[15][6] * (nv4 - 0.0));let eq40_e1299: f64 = (eq40_e1297 / s.v[716]);let __rspice_inv_cse_0: f64 = 1.0 / (s.v[716] * s.v[716]);let eq40_e1299_d_n0: f64 = (((eq40_e1297_d_n0 * s.v[716]) - (eq40_e1297 * s.dn[716][0])) * __rspice_inv_cse_0);let eq40_e1299_d_n1: f64 = (((eq40_e1297_d_n1 * s.v[716]) - (eq40_e1297 * s.dn[716][1])) * __rspice_inv_cse_0);let eq40_e1299_d_n2: f64 = (((eq40_e1297_d_n2 * s.v[716]) - (eq40_e1297 * s.dn[716][2])) * __rspice_inv_cse_0);let eq40_e1299_d_n3: f64 = (((eq40_e1297_d_n3 * s.v[716]) - (eq40_e1297 * s.dn[716][3])) * __rspice_inv_cse_0);let eq40_e1299_d_n4: f64 = (((eq40_e1297_d_n4 * s.v[716]) - (eq40_e1297 * s.dn[716][4])) * __rspice_inv_cse_0);let eq40_e1299_d_n5: f64 = (((eq40_e1297_d_n5 * s.v[716]) - (eq40_e1297 * s.dn[716][5])) * __rspice_inv_cse_0);let eq40_e1299_d_n6: f64 = (((eq40_e1297_d_n6 * s.v[716]) - (eq40_e1297 * s.dn[716][6])) * __rspice_inv_cse_0);let eq40_e1299_d_n7: f64 = (((eq40_e1297_d_n7 * s.v[716]) - (eq40_e1297 * s.dn[716][7])) * __rspice_inv_cse_0);let eq40_e1299_d_n8: f64 = (((eq40_e1297_d_n8 * s.v[716]) - (eq40_e1297 * s.dn[716][8])) * __rspice_inv_cse_0);let eq40_e1299_d_n9: f64 = (((eq40_e1297_d_n9 * s.v[716]) - (eq40_e1297 * s.dn[716][9])) * __rspice_inv_cse_0);let eq40_e1299_d_n10: f64 = (((eq40_e1297_d_n10 * s.v[716]) - (eq40_e1297 * s.dn[716][10])) * __rspice_inv_cse_0);let eq40_e1299_d_n11: f64 = (((eq40_e1297_d_n11 * s.v[716]) - (eq40_e1297 * s.dn[716][11])) * __rspice_inv_cse_0);let eq40_e1299_d_n12: f64 = (((eq40_e1297_d_n12 * s.v[716]) - (eq40_e1297 * s.dn[716][12])) * __rspice_inv_cse_0);let eq40_e1299_d_b0: f64 = (((eq40_e1297_d_b0 * s.v[716]) - (eq40_e1297 * s.db[716][0])) * __rspice_inv_cse_0);let eq40_e1299_d_b1: f64 = (((eq40_e1297_d_b1 * s.v[716]) - (eq40_e1297 * s.db[716][1])) * __rspice_inv_cse_0);let eq40_e1299_d_b2: f64 = (((eq40_e1297_d_b2 * s.v[716]) - (eq40_e1297 * s.db[716][2])) * __rspice_inv_cse_0);let eq40_e1299_d_b3: f64 = (((eq40_e1297_d_b3 * s.v[716]) - (eq40_e1297 * s.db[716][3])) * __rspice_inv_cse_0);let eq40_e1299_d_b4: f64 = (((eq40_e1297_d_b4 * s.v[716]) - (eq40_e1297 * s.db[716][4])) * __rspice_inv_cse_0);let eq40_e1299_d_b5: f64 = (((eq40_e1297_d_b5 * s.v[716]) - (eq40_e1297 * s.db[716][5])) * __rspice_inv_cse_0);let eq40_e1299_d_b6: f64 = (((eq40_e1297_d_b6 * s.v[716]) - (eq40_e1297 * s.db[716][6])) * __rspice_inv_cse_0);let eq40_value: f64 = eq40_e1299;let eq40_node_derivatives: [f64; 13] = [eq40_e1299_d_n0, eq40_e1299_d_n1, eq40_e1299_d_n2, eq40_e1299_d_n3, eq40_e1299_d_n4, eq40_e1299_d_n5, eq40_e1299_d_n6, eq40_e1299_d_n7, eq40_e1299_d_n8, eq40_e1299_d_n9, eq40_e1299_d_n10, eq40_e1299_d_n11, eq40_e1299_d_n12];let eq40_branch_derivatives: [f64; 7] = [eq40_e1299_d_b0, eq40_e1299_d_b1, eq40_e1299_d_b2, eq40_e1299_d_b3, eq40_e1299_d_b4, eq40_e1299_d_b5, eq40_e1299_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
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
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);let eq41_e1302_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq41_e1302_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq41_e1302_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq41_e1302_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq41_e1302_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq41_e1302_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq41_e1302_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq41_e1302_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq41_e1302_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq41_e1302_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq41_e1302_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq41_e1302_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq41_e1302_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq41_e1302_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq41_e1302_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq41_e1302_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq41_e1302_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq41_e1302_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq41_e1302_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq41_e1302_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq41_e1304: f64 = (eq41_e1302 * p.p33);let eq41_e1304_d_n0: f64 = (eq41_e1302_d_n0 * p.p33);let eq41_e1304_d_n1: f64 = (eq41_e1302_d_n1 * p.p33);let eq41_e1304_d_n2: f64 = (eq41_e1302_d_n2 * p.p33);let eq41_e1304_d_n3: f64 = (eq41_e1302_d_n3 * p.p33);let eq41_e1304_d_n4: f64 = (eq41_e1302_d_n4 * p.p33);let eq41_e1304_d_n5: f64 = (eq41_e1302_d_n5 * p.p33);let eq41_e1304_d_n6: f64 = (eq41_e1302_d_n6 * p.p33);let eq41_e1304_d_n7: f64 = (eq41_e1302_d_n7 * p.p33);let eq41_e1304_d_n8: f64 = (eq41_e1302_d_n8 * p.p33);let eq41_e1304_d_n9: f64 = (eq41_e1302_d_n9 * p.p33);let eq41_e1304_d_n10: f64 = (eq41_e1302_d_n10 * p.p33);let eq41_e1304_d_n11: f64 = (eq41_e1302_d_n11 * p.p33);let eq41_e1304_d_n12: f64 = (eq41_e1302_d_n12 * p.p33);let eq41_e1304_d_b0: f64 = (eq41_e1302_d_b0 * p.p33);let eq41_e1304_d_b1: f64 = (eq41_e1302_d_b1 * p.p33);let eq41_e1304_d_b2: f64 = (eq41_e1302_d_b2 * p.p33);let eq41_e1304_d_b3: f64 = (eq41_e1302_d_b3 * p.p33);let eq41_e1304_d_b4: f64 = (eq41_e1302_d_b4 * p.p33);let eq41_e1304_d_b5: f64 = (eq41_e1302_d_b5 * p.p33);let eq41_e1304_d_b6: f64 = (eq41_e1302_d_b6 * p.p33);let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));let eq41_e1306_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[840]) + (eq41_e1304 * s.db[840][0]));let eq41_e1306_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[840]) + (eq41_e1304 * s.db[840][1]));
        let eq41_e1306_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[840]) + (eq41_e1304 * s.db[840][2]));let eq41_e1306_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[840]) + (eq41_e1304 * s.db[840][3]));let eq41_e1306_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[840]) + (eq41_e1304 * s.db[840][4]));let eq41_e1306_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[840]) + (eq41_e1304 * s.db[840][5]));let eq41_e1306_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[840]) + (eq41_e1304 * s.db[840][6]));let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);let eq41_value: f64 = eq41_e1307;let eq41_node_derivatives: [f64; 13] = [(eq41_e1306_d_n0 * ddt_scale), (eq41_e1306_d_n1 * ddt_scale), (eq41_e1306_d_n2 * ddt_scale), (eq41_e1306_d_n3 * ddt_scale), (eq41_e1306_d_n4 * ddt_scale), (eq41_e1306_d_n5 * ddt_scale), (eq41_e1306_d_n6 * ddt_scale), (eq41_e1306_d_n7 * ddt_scale), (eq41_e1306_d_n8 * ddt_scale), (eq41_e1306_d_n9 * ddt_scale), (eq41_e1306_d_n10 * ddt_scale), (eq41_e1306_d_n11 * ddt_scale), (eq41_e1306_d_n12 * ddt_scale)];let eq41_branch_derivatives: [f64; 7] = [(eq41_e1306_d_b0 * ddt_scale), (eq41_e1306_d_b1 * ddt_scale), (eq41_e1306_d_b2 * ddt_scale), (eq41_e1306_d_b3 * ddt_scale), (eq41_e1306_d_b4 * ddt_scale), (eq41_e1306_d_b5 * ddt_scale), (eq41_e1306_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
    }
}
