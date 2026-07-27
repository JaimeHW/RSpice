#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq43_e1965: f64 = (-s.v[379]);let eq43_e1967: f64 = (eq43_e1965 * s.v[421]);let eq43_e1967_d_n0: f64 = (((-s.dn[379][0]) * s.v[421]) + (eq43_e1965 * s.dn[421][0]));let eq43_e1967_d_n1: f64 = (((-s.dn[379][1]) * s.v[421]) + (eq43_e1965 * s.dn[421][1]));let eq43_e1967_d_n2: f64 = (((-s.dn[379][2]) * s.v[421]) + (eq43_e1965 * s.dn[421][2]));let eq43_e1967_d_n3: f64 = (((-s.dn[379][3]) * s.v[421]) + (eq43_e1965 * s.dn[421][3]));let eq43_e1967_d_n4: f64 = (((-s.dn[379][4]) * s.v[421]) + (eq43_e1965 * s.dn[421][4]));let eq43_e1967_d_n5: f64 = (((-s.dn[379][5]) * s.v[421]) + (eq43_e1965 * s.dn[421][5]));let eq43_e1967_d_n6: f64 = (((-s.dn[379][6]) * s.v[421]) + (eq43_e1965 * s.dn[421][6]));let eq43_e1967_d_n7: f64 = (((-s.dn[379][7]) * s.v[421]) + (eq43_e1965 * s.dn[421][7]));let eq43_e1967_d_n8: f64 = (((-s.dn[379][8]) * s.v[421]) + (eq43_e1965 * s.dn[421][8]));let eq43_e1967_d_n9: f64 = (((-s.dn[379][9]) * s.v[421]) + (eq43_e1965 * s.dn[421][9]));let eq43_e1967_d_n10: f64 = (((-s.dn[379][10]) * s.v[421]) + (eq43_e1965 * s.dn[421][10]));let eq43_e1967_d_n11: f64 = (((-s.dn[379][11]) * s.v[421]) + (eq43_e1965 * s.dn[421][11]));let eq43_e1967_d_n12: f64 = (((-s.dn[379][12]) * s.v[421]) + (eq43_e1965 * s.dn[421][12]));let eq43_e1967_d_n13: f64 = (((-s.dn[379][13]) * s.v[421]) + (eq43_e1965 * s.dn[421][13]));let eq43_e1967_d_b0: f64 = (((-s.db[379][0]) * s.v[421]) + (eq43_e1965 * s.db[421][0]));let eq43_e1967_d_b1: f64 = (((-s.db[379][1]) * s.v[421]) + (eq43_e1965 * s.db[421][1]));let eq43_e1967_d_b2: f64 = (((-s.db[379][2]) * s.v[421]) + (eq43_e1965 * s.db[421][2]));let eq43_e1967_d_b3: f64 = (((-s.db[379][3]) * s.v[421]) + (eq43_e1965 * s.db[421][3]));let eq43_e1967_d_b4: f64 = (((-s.db[379][4]) * s.v[421]) + (eq43_e1965 * s.db[421][4]));let eq43_e1967_d_b5: f64 = (((-s.db[379][5]) * s.v[421]) + (eq43_e1965 * s.db[421][5]));let eq43_e1967_d_b6: f64 = (((-s.db[379][6]) * s.v[421]) + (eq43_e1965 * s.db[421][6]));let eq43_e1967_d_b7: f64 = (((-s.db[379][7]) * s.v[421]) + (eq43_e1965 * s.db[421][7]));let eq43_e1967_d_b8: f64 = (((-s.db[379][8]) * s.v[421]) + (eq43_e1965 * s.db[421][8]));let eq43_e1967_d_b9: f64 = (((-s.db[379][9]) * s.v[421]) + (eq43_e1965 * s.db[421][9]));let eq43_e1967_d_b10: f64 = (((-s.db[379][10]) * s.v[421]) + (eq43_e1965 * s.db[421][10]));let eq43_e1967_d_b11: f64 = (((-s.db[379][11]) * s.v[421]) + (eq43_e1965 * s.db[421][11]));let eq43_e1968: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq43_e1967);let eq43_value: f64 = eq43_e1968;let eq43_node_derivatives: [f64; 14] = [(eq43_e1967_d_n0 * ddt_scale), (eq43_e1967_d_n1 * ddt_scale), (eq43_e1967_d_n2 * ddt_scale), (eq43_e1967_d_n3 * ddt_scale), (eq43_e1967_d_n4 * ddt_scale), (eq43_e1967_d_n5 * ddt_scale), (eq43_e1967_d_n6 * ddt_scale), (eq43_e1967_d_n7 * ddt_scale), (eq43_e1967_d_n8 * ddt_scale), (eq43_e1967_d_n9 * ddt_scale), (eq43_e1967_d_n10 * ddt_scale), (eq43_e1967_d_n11 * ddt_scale), (eq43_e1967_d_n12 * ddt_scale), (eq43_e1967_d_n13 * ddt_scale)];let eq43_branch_derivatives: [f64; 12] = [(eq43_e1967_d_b0 * ddt_scale), (eq43_e1967_d_b1 * ddt_scale), (eq43_e1967_d_b2 * ddt_scale), (eq43_e1967_d_b3 * ddt_scale), (eq43_e1967_d_b4 * ddt_scale), (eq43_e1967_d_b5 * ddt_scale), (eq43_e1967_d_b6 * ddt_scale), (eq43_e1967_d_b7 * ddt_scale), (eq43_e1967_d_b8 * ddt_scale), (eq43_e1967_d_b9 * ddt_scale), (eq43_e1967_d_b10 * ddt_scale), (eq43_e1967_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );let eq44_e1971: f64 = (s.v[379] * s.v[210]);let eq44_e1971_d_n0: f64 = ((s.dn[379][0] * s.v[210]) + (s.v[379] * s.dn[210][0]));let eq44_e1971_d_n1: f64 = ((s.dn[379][1] * s.v[210]) + (s.v[379] * s.dn[210][1]));let eq44_e1971_d_n2: f64 = ((s.dn[379][2] * s.v[210]) + (s.v[379] * s.dn[210][2]));let eq44_e1971_d_n3: f64 = ((s.dn[379][3] * s.v[210]) + (s.v[379] * s.dn[210][3]));let eq44_e1971_d_n4: f64 = ((s.dn[379][4] * s.v[210]) + (s.v[379] * s.dn[210][4]));let eq44_e1971_d_n5: f64 = ((s.dn[379][5] * s.v[210]) + (s.v[379] * s.dn[210][5]));let eq44_e1971_d_n6: f64 = ((s.dn[379][6] * s.v[210]) + (s.v[379] * s.dn[210][6]));let eq44_e1971_d_n7: f64 = ((s.dn[379][7] * s.v[210]) + (s.v[379] * s.dn[210][7]));let eq44_e1971_d_n8: f64 = ((s.dn[379][8] * s.v[210]) + (s.v[379] * s.dn[210][8]));let eq44_e1971_d_n9: f64 = ((s.dn[379][9] * s.v[210]) + (s.v[379] * s.dn[210][9]));let eq44_e1971_d_n10: f64 = ((s.dn[379][10] * s.v[210]) + (s.v[379] * s.dn[210][10]));let eq44_e1971_d_n11: f64 = ((s.dn[379][11] * s.v[210]) + (s.v[379] * s.dn[210][11]));let eq44_e1971_d_n12: f64 = ((s.dn[379][12] * s.v[210]) + (s.v[379] * s.dn[210][12]));let eq44_e1971_d_n13: f64 = ((s.dn[379][13] * s.v[210]) + (s.v[379] * s.dn[210][13]));let eq44_e1971_d_b0: f64 = ((s.db[379][0] * s.v[210]) + (s.v[379] * s.db[210][0]));let eq44_e1971_d_b1: f64 = ((s.db[379][1] * s.v[210]) + (s.v[379] * s.db[210][1]));let eq44_e1971_d_b2: f64 = ((s.db[379][2] * s.v[210]) + (s.v[379] * s.db[210][2]));let eq44_e1971_d_b3: f64 = ((s.db[379][3] * s.v[210]) + (s.v[379] * s.db[210][3]));let eq44_e1971_d_b4: f64 = ((s.db[379][4] * s.v[210]) + (s.v[379] * s.db[210][4]));let eq44_e1971_d_b5: f64 = ((s.db[379][5] * s.v[210]) + (s.v[379] * s.db[210][5]));let eq44_e1971_d_b6: f64 = ((s.db[379][6] * s.v[210]) + (s.v[379] * s.db[210][6]));let eq44_e1971_d_b7: f64 = ((s.db[379][7] * s.v[210]) + (s.v[379] * s.db[210][7]));let eq44_e1971_d_b8: f64 = ((s.db[379][8] * s.v[210]) + (s.v[379] * s.db[210][8]));let eq44_e1971_d_b9: f64 = ((s.db[379][9] * s.v[210]) + (s.v[379] * s.db[210][9]));let eq44_e1971_d_b10: f64 = ((s.db[379][10] * s.v[210]) + (s.v[379] * s.db[210][10]));let eq44_e1971_d_b11: f64 = ((s.db[379][11] * s.v[210]) + (s.v[379] * s.db[210][11]));let eq44_value: f64 = eq44_e1971;let eq44_node_derivatives: [f64; 14] = [eq44_e1971_d_n0, eq44_e1971_d_n1, eq44_e1971_d_n2, eq44_e1971_d_n3, eq44_e1971_d_n4, eq44_e1971_d_n5, eq44_e1971_d_n6, eq44_e1971_d_n7, eq44_e1971_d_n8, eq44_e1971_d_n9, eq44_e1971_d_n10, eq44_e1971_d_n11, eq44_e1971_d_n12, eq44_e1971_d_n13];let eq44_branch_derivatives: [f64; 12] = [eq44_e1971_d_b0, eq44_e1971_d_b1, eq44_e1971_d_b2, eq44_e1971_d_b3, eq44_e1971_d_b4, eq44_e1971_d_b5, eq44_e1971_d_b6, eq44_e1971_d_b7, eq44_e1971_d_b8, eq44_e1971_d_b9, eq44_e1971_d_b10, eq44_e1971_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(11),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq45_e1974: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[1039]);let eq45_e1975: f64 = (s.v[379] * eq45_e1974);let eq45_e1975_d_n0: f64 = ((s.dn[379][0] * eq45_e1974) + (s.v[379] * (s.dn[1039][0] * ddt_scale)));let eq45_e1975_d_n1: f64 = ((s.dn[379][1] * eq45_e1974) + (s.v[379] * (s.dn[1039][1] * ddt_scale)));let eq45_e1975_d_n2: f64 = ((s.dn[379][2] * eq45_e1974) + (s.v[379] * (s.dn[1039][2] * ddt_scale)));let eq45_e1975_d_n3: f64 = ((s.dn[379][3] * eq45_e1974) + (s.v[379] * (s.dn[1039][3] * ddt_scale)));let eq45_e1975_d_n4: f64 = ((s.dn[379][4] * eq45_e1974) + (s.v[379] * (s.dn[1039][4] * ddt_scale)));let eq45_e1975_d_n5: f64 = ((s.dn[379][5] * eq45_e1974) + (s.v[379] * (s.dn[1039][5] * ddt_scale)));let eq45_e1975_d_n6: f64 = ((s.dn[379][6] * eq45_e1974) + (s.v[379] * (s.dn[1039][6] * ddt_scale)));let eq45_e1975_d_n7: f64 = ((s.dn[379][7] * eq45_e1974) + (s.v[379] * (s.dn[1039][7] * ddt_scale)));let eq45_e1975_d_n8: f64 = ((s.dn[379][8] * eq45_e1974) + (s.v[379] * (s.dn[1039][8] * ddt_scale)));let eq45_e1975_d_n9: f64 = ((s.dn[379][9] * eq45_e1974) + (s.v[379] * (s.dn[1039][9] * ddt_scale)));let eq45_e1975_d_n10: f64 = ((s.dn[379][10] * eq45_e1974) + (s.v[379] * (s.dn[1039][10] * ddt_scale)));let eq45_e1975_d_n11: f64 = ((s.dn[379][11] * eq45_e1974) + (s.v[379] * (s.dn[1039][11] * ddt_scale)));let eq45_e1975_d_n12: f64 = ((s.dn[379][12] * eq45_e1974) + (s.v[379] * (s.dn[1039][12] * ddt_scale)));let eq45_e1975_d_n13: f64 = ((s.dn[379][13] * eq45_e1974) + (s.v[379] * (s.dn[1039][13] * ddt_scale)));let eq45_e1975_d_b0: f64 = ((s.db[379][0] * eq45_e1974) + (s.v[379] * (s.db[1039][0] * ddt_scale)));let eq45_e1975_d_b1: f64 = ((s.db[379][1] * eq45_e1974) + (s.v[379] * (s.db[1039][1] * ddt_scale)));let eq45_e1975_d_b2: f64 = ((s.db[379][2] * eq45_e1974) + (s.v[379] * (s.db[1039][2] * ddt_scale)));let eq45_e1975_d_b3: f64 = ((s.db[379][3] * eq45_e1974) + (s.v[379] * (s.db[1039][3] * ddt_scale)));let eq45_e1975_d_b4: f64 = ((s.db[379][4] * eq45_e1974) + (s.v[379] * (s.db[1039][4] * ddt_scale)));let eq45_e1975_d_b5: f64 = ((s.db[379][5] * eq45_e1974) + (s.v[379] * (s.db[1039][5] * ddt_scale)));let eq45_e1975_d_b6: f64 = ((s.db[379][6] * eq45_e1974) + (s.v[379] * (s.db[1039][6] * ddt_scale)));let eq45_e1975_d_b7: f64 = ((s.db[379][7] * eq45_e1974) + (s.v[379] * (s.db[1039][7] * ddt_scale)));let eq45_e1975_d_b8: f64 = ((s.db[379][8] * eq45_e1974) + (s.v[379] * (s.db[1039][8] * ddt_scale)));let eq45_e1975_d_b9: f64 = ((s.db[379][9] * eq45_e1974) + (s.v[379] * (s.db[1039][9] * ddt_scale)));let eq45_e1975_d_b10: f64 = ((s.db[379][10] * eq45_e1974) + (s.v[379] * (s.db[1039][10] * ddt_scale)));let eq45_e1975_d_b11: f64 = ((s.db[379][11] * eq45_e1974) + (s.v[379] * (s.db[1039][11] * ddt_scale)));let eq45_value: f64 = eq45_e1975;let eq45_node_derivatives: [f64; 14] = [eq45_e1975_d_n0, eq45_e1975_d_n1, eq45_e1975_d_n2, eq45_e1975_d_n3, eq45_e1975_d_n4, eq45_e1975_d_n5, eq45_e1975_d_n6, eq45_e1975_d_n7, eq45_e1975_d_n8, eq45_e1975_d_n9, eq45_e1975_d_n10, eq45_e1975_d_n11, eq45_e1975_d_n12, eq45_e1975_d_n13];let eq45_branch_derivatives: [f64; 12] = [eq45_e1975_d_b0, eq45_e1975_d_b1, eq45_e1975_d_b2, eq45_e1975_d_b3, eq45_e1975_d_b4, eq45_e1975_d_b5, eq45_e1975_d_b6, eq45_e1975_d_b7, eq45_e1975_d_b8, eq45_e1975_d_b9, eq45_e1975_d_b10, eq45_e1975_d_b11];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );let eq46_e1977: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, s.v[1047]);let eq46_value: f64 = eq46_e1977;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq46_value),
            &s.dn[1047],
            &s.db[1047],
            (multiplicity) * (ddt_scale),
        );let eq47_e1979: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, s.v[1046]);let eq47_value: f64 = eq47_e1979;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq47_value),
            &s.dn[1046],
            &s.db[1046],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let eq48_e1982: f64 = (s.v[379] * s.v[211]);let eq48_e1982_d_n0: f64 = ((s.dn[379][0] * s.v[211]) + (s.v[379] * s.dn[211][0]));let eq48_e1982_d_n1: f64 = ((s.dn[379][1] * s.v[211]) + (s.v[379] * s.dn[211][1]));let eq48_e1982_d_n2: f64 = ((s.dn[379][2] * s.v[211]) + (s.v[379] * s.dn[211][2]));let eq48_e1982_d_n3: f64 = ((s.dn[379][3] * s.v[211]) + (s.v[379] * s.dn[211][3]));let eq48_e1982_d_n4: f64 = ((s.dn[379][4] * s.v[211]) + (s.v[379] * s.dn[211][4]));let eq48_e1982_d_n5: f64 = ((s.dn[379][5] * s.v[211]) + (s.v[379] * s.dn[211][5]));let eq48_e1982_d_n6: f64 = ((s.dn[379][6] * s.v[211]) + (s.v[379] * s.dn[211][6]));let eq48_e1982_d_n7: f64 = ((s.dn[379][7] * s.v[211]) + (s.v[379] * s.dn[211][7]));let eq48_e1982_d_n8: f64 = ((s.dn[379][8] * s.v[211]) + (s.v[379] * s.dn[211][8]));let eq48_e1982_d_n9: f64 = ((s.dn[379][9] * s.v[211]) + (s.v[379] * s.dn[211][9]));let eq48_e1982_d_n10: f64 = ((s.dn[379][10] * s.v[211]) + (s.v[379] * s.dn[211][10]));let eq48_e1982_d_n11: f64 = ((s.dn[379][11] * s.v[211]) + (s.v[379] * s.dn[211][11]));let eq48_e1982_d_n12: f64 = ((s.dn[379][12] * s.v[211]) + (s.v[379] * s.dn[211][12]));let eq48_e1982_d_n13: f64 = ((s.dn[379][13] * s.v[211]) + (s.v[379] * s.dn[211][13]));let eq48_e1982_d_b0: f64 = ((s.db[379][0] * s.v[211]) + (s.v[379] * s.db[211][0]));let eq48_e1982_d_b1: f64 = ((s.db[379][1] * s.v[211]) + (s.v[379] * s.db[211][1]));let eq48_e1982_d_b2: f64 = ((s.db[379][2] * s.v[211]) + (s.v[379] * s.db[211][2]));let eq48_e1982_d_b3: f64 = ((s.db[379][3] * s.v[211]) + (s.v[379] * s.db[211][3]));let eq48_e1982_d_b4: f64 = ((s.db[379][4] * s.v[211]) + (s.v[379] * s.db[211][4]));let eq48_e1982_d_b5: f64 = ((s.db[379][5] * s.v[211]) + (s.v[379] * s.db[211][5]));let eq48_e1982_d_b6: f64 = ((s.db[379][6] * s.v[211]) + (s.v[379] * s.db[211][6]));let eq48_e1982_d_b7: f64 = ((s.db[379][7] * s.v[211]) + (s.v[379] * s.db[211][7]));let eq48_e1982_d_b8: f64 = ((s.db[379][8] * s.v[211]) + (s.v[379] * s.db[211][8]));let eq48_e1982_d_b9: f64 = ((s.db[379][9] * s.v[211]) + (s.v[379] * s.db[211][9]));let eq48_e1982_d_b10: f64 = ((s.db[379][10] * s.v[211]) + (s.v[379] * s.db[211][10]));let eq48_e1982_d_b11: f64 = ((s.db[379][11] * s.v[211]) + (s.v[379] * s.db[211][11]));let eq48_e1984: f64 = (eq48_e1982 * s.v[380]);let eq48_e1984_d_n0: f64 = ((eq48_e1982_d_n0 * s.v[380]) + (eq48_e1982 * s.dn[380][0]));let eq48_e1984_d_n1: f64 = ((eq48_e1982_d_n1 * s.v[380]) + (eq48_e1982 * s.dn[380][1]));let eq48_e1984_d_n2: f64 = ((eq48_e1982_d_n2 * s.v[380]) + (eq48_e1982 * s.dn[380][2]));let eq48_e1984_d_n3: f64 = ((eq48_e1982_d_n3 * s.v[380]) + (eq48_e1982 * s.dn[380][3]));let eq48_e1984_d_n4: f64 = ((eq48_e1982_d_n4 * s.v[380]) + (eq48_e1982 * s.dn[380][4]));let eq48_e1984_d_n5: f64 = ((eq48_e1982_d_n5 * s.v[380]) + (eq48_e1982 * s.dn[380][5]));let eq48_e1984_d_n6: f64 = ((eq48_e1982_d_n6 * s.v[380]) + (eq48_e1982 * s.dn[380][6]));let eq48_e1984_d_n7: f64 = ((eq48_e1982_d_n7 * s.v[380]) + (eq48_e1982 * s.dn[380][7]));let eq48_e1984_d_n8: f64 = ((eq48_e1982_d_n8 * s.v[380]) + (eq48_e1982 * s.dn[380][8]));let eq48_e1984_d_n9: f64 = ((eq48_e1982_d_n9 * s.v[380]) + (eq48_e1982 * s.dn[380][9]));let eq48_e1984_d_n10: f64 = ((eq48_e1982_d_n10 * s.v[380]) + (eq48_e1982 * s.dn[380][10]));let eq48_e1984_d_n11: f64 = ((eq48_e1982_d_n11 * s.v[380]) + (eq48_e1982 * s.dn[380][11]));let eq48_e1984_d_n12: f64 = ((eq48_e1982_d_n12 * s.v[380]) + (eq48_e1982 * s.dn[380][12]));let eq48_e1984_d_n13: f64 = ((eq48_e1982_d_n13 * s.v[380]) + (eq48_e1982 * s.dn[380][13]));let eq48_e1984_d_b0: f64 = ((eq48_e1982_d_b0 * s.v[380]) + (eq48_e1982 * s.db[380][0]));let eq48_e1984_d_b1: f64 = ((eq48_e1982_d_b1 * s.v[380]) + (eq48_e1982 * s.db[380][1]));let eq48_e1984_d_b2: f64 = ((eq48_e1982_d_b2 * s.v[380]) + (eq48_e1982 * s.db[380][2]));let eq48_e1984_d_b3: f64 = ((eq48_e1982_d_b3 * s.v[380]) + (eq48_e1982 * s.db[380][3]));let eq48_e1984_d_b4: f64 = ((eq48_e1982_d_b4 * s.v[380]) + (eq48_e1982 * s.db[380][4]));let eq48_e1984_d_b5: f64 = ((eq48_e1982_d_b5 * s.v[380]) + (eq48_e1982 * s.db[380][5]));
        let eq48_e1984_d_b6: f64 = ((eq48_e1982_d_b6 * s.v[380]) + (eq48_e1982 * s.db[380][6]));let eq48_e1984_d_b7: f64 = ((eq48_e1982_d_b7 * s.v[380]) + (eq48_e1982 * s.db[380][7]));let eq48_e1984_d_b8: f64 = ((eq48_e1982_d_b8 * s.v[380]) + (eq48_e1982 * s.db[380][8]));let eq48_e1984_d_b9: f64 = ((eq48_e1982_d_b9 * s.v[380]) + (eq48_e1982 * s.db[380][9]));let eq48_e1984_d_b10: f64 = ((eq48_e1982_d_b10 * s.v[380]) + (eq48_e1982 * s.db[380][10]));let eq48_e1984_d_b11: f64 = ((eq48_e1982_d_b11 * s.v[380]) + (eq48_e1982 * s.db[380][11]));let eq48_value: f64 = eq48_e1984;let eq48_node_derivatives: [f64; 14] = [eq48_e1984_d_n0, eq48_e1984_d_n1, eq48_e1984_d_n2, eq48_e1984_d_n3, eq48_e1984_d_n4, eq48_e1984_d_n5, eq48_e1984_d_n6, eq48_e1984_d_n7, eq48_e1984_d_n8, eq48_e1984_d_n9, eq48_e1984_d_n10, eq48_e1984_d_n11, eq48_e1984_d_n12, eq48_e1984_d_n13];let eq48_branch_derivatives: [f64; 12] = [eq48_e1984_d_b0, eq48_e1984_d_b1, eq48_e1984_d_b2, eq48_e1984_d_b3, eq48_e1984_d_b4, eq48_e1984_d_b5, eq48_e1984_d_b6, eq48_e1984_d_b7, eq48_e1984_d_b8, eq48_e1984_d_b9, eq48_e1984_d_b10, eq48_e1984_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e1988, eq49_e1988_d_n0, eq49_e1988_d_n1, eq49_e1988_d_n2, eq49_e1988_d_n3, eq49_e1988_d_n4, eq49_e1988_d_n5, eq49_e1988_d_n6, eq49_e1988_d_n7, eq49_e1988_d_n8, eq49_e1988_d_n9, eq49_e1988_d_n10, eq49_e1988_d_n11, eq49_e1988_d_n12, eq49_e1988_d_n13, eq49_e1988_d_b0, eq49_e1988_d_b1, eq49_e1988_d_b2, eq49_e1988_d_b3, eq49_e1988_d_b4, eq49_e1988_d_b5, eq49_e1988_d_b6, eq49_e1988_d_b7, eq49_e1988_d_b8, eq49_e1988_d_b9, eq49_e1988_d_b10, eq49_e1988_d_b11,) = {
    if s.b[2009] {
        (s.v[1102], s.dn[1102][0], s.dn[1102][1], s.dn[1102][2], s.dn[1102][3], s.dn[1102][4], s.dn[1102][5], s.dn[1102][6], s.dn[1102][7], s.dn[1102][8], s.dn[1102][9], s.dn[1102][10], s.dn[1102][11], s.dn[1102][12], s.dn[1102][13], s.db[1102][0], s.db[1102][1], s.db[1102][2], s.db[1102][3], s.db[1102][4], s.db[1102][5], s.db[1102][6], s.db[1102][7], s.db[1102][8], s.db[1102][9], s.db[1102][10], s.db[1102][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1988;let eq49_node_derivatives: [f64; 14] = [eq49_e1988_d_n0, eq49_e1988_d_n1, eq49_e1988_d_n2, eq49_e1988_d_n3, eq49_e1988_d_n4, eq49_e1988_d_n5, eq49_e1988_d_n6, eq49_e1988_d_n7, eq49_e1988_d_n8, eq49_e1988_d_n9, eq49_e1988_d_n10, eq49_e1988_d_n11, eq49_e1988_d_n12, eq49_e1988_d_n13];let eq49_branch_derivatives: [f64; 12] = [eq49_e1988_d_b0, eq49_e1988_d_b1, eq49_e1988_d_b2, eq49_e1988_d_b3, eq49_e1988_d_b4, eq49_e1988_d_b5, eq49_e1988_d_b6, eq49_e1988_d_b7, eq49_e1988_d_b8, eq49_e1988_d_b9, eq49_e1988_d_b10, eq49_e1988_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(10),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq50_e1994, eq50_e1994_d_n0, eq50_e1994_d_n1, eq50_e1994_d_n2, eq50_e1994_d_n3, eq50_e1994_d_n4, eq50_e1994_d_n5, eq50_e1994_d_n6, eq50_e1994_d_n7, eq50_e1994_d_n8, eq50_e1994_d_n9, eq50_e1994_d_n10, eq50_e1994_d_n11, eq50_e1994_d_n12, eq50_e1994_d_n13, eq50_e1994_d_b0, eq50_e1994_d_b1, eq50_e1994_d_b2, eq50_e1994_d_b3, eq50_e1994_d_b4, eq50_e1994_d_b5, eq50_e1994_d_b6, eq50_e1994_d_b7, eq50_e1994_d_b8, eq50_e1994_d_b9, eq50_e1994_d_b10, eq50_e1994_d_b11,) = {
    if s.b[2010] {
        let eq50_e1992: f64 = (s.v[1098] + s.v[1100]);let eq50_e1992_d_n0: f64 = (s.dn[1098][0] + s.dn[1100][0]);let eq50_e1992_d_n1: f64 = (s.dn[1098][1] + s.dn[1100][1]);let eq50_e1992_d_n2: f64 = (s.dn[1098][2] + s.dn[1100][2]);let eq50_e1992_d_n3: f64 = (s.dn[1098][3] + s.dn[1100][3]);let eq50_e1992_d_n4: f64 = (s.dn[1098][4] + s.dn[1100][4]);let eq50_e1992_d_n5: f64 = (s.dn[1098][5] + s.dn[1100][5]);let eq50_e1992_d_n6: f64 = (s.dn[1098][6] + s.dn[1100][6]);let eq50_e1992_d_n7: f64 = (s.dn[1098][7] + s.dn[1100][7]);let eq50_e1992_d_n8: f64 = (s.dn[1098][8] + s.dn[1100][8]);let eq50_e1992_d_n9: f64 = (s.dn[1098][9] + s.dn[1100][9]);let eq50_e1992_d_n10: f64 = (s.dn[1098][10] + s.dn[1100][10]);let eq50_e1992_d_n11: f64 = (s.dn[1098][11] + s.dn[1100][11]);let eq50_e1992_d_n12: f64 = (s.dn[1098][12] + s.dn[1100][12]);let eq50_e1992_d_n13: f64 = (s.dn[1098][13] + s.dn[1100][13]);let eq50_e1992_d_b0: f64 = (s.db[1098][0] + s.db[1100][0]);let eq50_e1992_d_b1: f64 = (s.db[1098][1] + s.db[1100][1]);let eq50_e1992_d_b2: f64 = (s.db[1098][2] + s.db[1100][2]);let eq50_e1992_d_b3: f64 = (s.db[1098][3] + s.db[1100][3]);let eq50_e1992_d_b4: f64 = (s.db[1098][4] + s.db[1100][4]);let eq50_e1992_d_b5: f64 = (s.db[1098][5] + s.db[1100][5]);let eq50_e1992_d_b6: f64 = (s.db[1098][6] + s.db[1100][6]);let eq50_e1992_d_b7: f64 = (s.db[1098][7] + s.db[1100][7]);let eq50_e1992_d_b8: f64 = (s.db[1098][8] + s.db[1100][8]);let eq50_e1992_d_b9: f64 = (s.db[1098][9] + s.db[1100][9]);let eq50_e1992_d_b10: f64 = (s.db[1098][10] + s.db[1100][10]);let eq50_e1992_d_b11: f64 = (s.db[1098][11] + s.db[1100][11]);
        (eq50_e1992, eq50_e1992_d_n0, eq50_e1992_d_n1, eq50_e1992_d_n2, eq50_e1992_d_n3, eq50_e1992_d_n4, eq50_e1992_d_n5, eq50_e1992_d_n6, eq50_e1992_d_n7, eq50_e1992_d_n8, eq50_e1992_d_n9, eq50_e1992_d_n10, eq50_e1992_d_n11, eq50_e1992_d_n12, eq50_e1992_d_n13, eq50_e1992_d_b0, eq50_e1992_d_b1, eq50_e1992_d_b2, eq50_e1992_d_b3, eq50_e1992_d_b4, eq50_e1992_d_b5, eq50_e1992_d_b6, eq50_e1992_d_b7, eq50_e1992_d_b8, eq50_e1992_d_b9, eq50_e1992_d_b10, eq50_e1992_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1994;let eq50_node_derivatives: [f64; 14] = [eq50_e1994_d_n0, eq50_e1994_d_n1, eq50_e1994_d_n2, eq50_e1994_d_n3, eq50_e1994_d_n4, eq50_e1994_d_n5, eq50_e1994_d_n6, eq50_e1994_d_n7, eq50_e1994_d_n8, eq50_e1994_d_n9, eq50_e1994_d_n10, eq50_e1994_d_n11, eq50_e1994_d_n12, eq50_e1994_d_n13];let eq50_branch_derivatives: [f64; 12] = [eq50_e1994_d_b0, eq50_e1994_d_b1, eq50_e1994_d_b2, eq50_e1994_d_b3, eq50_e1994_d_b4, eq50_e1994_d_b5, eq50_e1994_d_b6, eq50_e1994_d_b7, eq50_e1994_d_b8, eq50_e1994_d_b9, eq50_e1994_d_b10, eq50_e1994_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2000, eq51_e2000_d_n0, eq51_e2000_d_n1, eq51_e2000_d_n2, eq51_e2000_d_n3, eq51_e2000_d_n4, eq51_e2000_d_n5, eq51_e2000_d_n6, eq51_e2000_d_n7, eq51_e2000_d_n8, eq51_e2000_d_n9, eq51_e2000_d_n10, eq51_e2000_d_n11, eq51_e2000_d_n12, eq51_e2000_d_n13, eq51_e2000_d_b0, eq51_e2000_d_b1, eq51_e2000_d_b2, eq51_e2000_d_b3, eq51_e2000_d_b4, eq51_e2000_d_b5, eq51_e2000_d_b6, eq51_e2000_d_b7, eq51_e2000_d_b8, eq51_e2000_d_b9, eq51_e2000_d_b10, eq51_e2000_d_b11,) = {
    if s.b[2010] {
        let eq51_e1998: f64 = (s.v[1099] + s.v[1101]);let eq51_e1998_d_n0: f64 = (s.dn[1099][0] + s.dn[1101][0]);let eq51_e1998_d_n1: f64 = (s.dn[1099][1] + s.dn[1101][1]);let eq51_e1998_d_n2: f64 = (s.dn[1099][2] + s.dn[1101][2]);let eq51_e1998_d_n3: f64 = (s.dn[1099][3] + s.dn[1101][3]);let eq51_e1998_d_n4: f64 = (s.dn[1099][4] + s.dn[1101][4]);let eq51_e1998_d_n5: f64 = (s.dn[1099][5] + s.dn[1101][5]);let eq51_e1998_d_n6: f64 = (s.dn[1099][6] + s.dn[1101][6]);let eq51_e1998_d_n7: f64 = (s.dn[1099][7] + s.dn[1101][7]);let eq51_e1998_d_n8: f64 = (s.dn[1099][8] + s.dn[1101][8]);let eq51_e1998_d_n9: f64 = (s.dn[1099][9] + s.dn[1101][9]);let eq51_e1998_d_n10: f64 = (s.dn[1099][10] + s.dn[1101][10]);let eq51_e1998_d_n11: f64 = (s.dn[1099][11] + s.dn[1101][11]);let eq51_e1998_d_n12: f64 = (s.dn[1099][12] + s.dn[1101][12]);let eq51_e1998_d_n13: f64 = (s.dn[1099][13] + s.dn[1101][13]);let eq51_e1998_d_b0: f64 = (s.db[1099][0] + s.db[1101][0]);let eq51_e1998_d_b1: f64 = (s.db[1099][1] + s.db[1101][1]);let eq51_e1998_d_b2: f64 = (s.db[1099][2] + s.db[1101][2]);let eq51_e1998_d_b3: f64 = (s.db[1099][3] + s.db[1101][3]);let eq51_e1998_d_b4: f64 = (s.db[1099][4] + s.db[1101][4]);let eq51_e1998_d_b5: f64 = (s.db[1099][5] + s.db[1101][5]);let eq51_e1998_d_b6: f64 = (s.db[1099][6] + s.db[1101][6]);let eq51_e1998_d_b7: f64 = (s.db[1099][7] + s.db[1101][7]);let eq51_e1998_d_b8: f64 = (s.db[1099][8] + s.db[1101][8]);let eq51_e1998_d_b9: f64 = (s.db[1099][9] + s.db[1101][9]);let eq51_e1998_d_b10: f64 = (s.db[1099][10] + s.db[1101][10]);let eq51_e1998_d_b11: f64 = (s.db[1099][11] + s.db[1101][11]);
        (eq51_e1998, eq51_e1998_d_n0, eq51_e1998_d_n1, eq51_e1998_d_n2, eq51_e1998_d_n3, eq51_e1998_d_n4, eq51_e1998_d_n5, eq51_e1998_d_n6, eq51_e1998_d_n7, eq51_e1998_d_n8, eq51_e1998_d_n9, eq51_e1998_d_n10, eq51_e1998_d_n11, eq51_e1998_d_n12, eq51_e1998_d_n13, eq51_e1998_d_b0, eq51_e1998_d_b1, eq51_e1998_d_b2, eq51_e1998_d_b3, eq51_e1998_d_b4, eq51_e1998_d_b5, eq51_e1998_d_b6, eq51_e1998_d_b7, eq51_e1998_d_b8, eq51_e1998_d_b9, eq51_e1998_d_b10, eq51_e1998_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2000;let eq51_node_derivatives: [f64; 14] = [eq51_e2000_d_n0, eq51_e2000_d_n1, eq51_e2000_d_n2, eq51_e2000_d_n3, eq51_e2000_d_n4, eq51_e2000_d_n5, eq51_e2000_d_n6, eq51_e2000_d_n7, eq51_e2000_d_n8, eq51_e2000_d_n9, eq51_e2000_d_n10, eq51_e2000_d_n11, eq51_e2000_d_n12, eq51_e2000_d_n13];let eq51_branch_derivatives: [f64; 12] = [eq51_e2000_d_b0, eq51_e2000_d_b1, eq51_e2000_d_b2, eq51_e2000_d_b3, eq51_e2000_d_b4, eq51_e2000_d_b5, eq51_e2000_d_b6, eq51_e2000_d_b7, eq51_e2000_d_b8, eq51_e2000_d_b9, eq51_e2000_d_b10, eq51_e2000_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_15(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq52_e2006, eq52_e2006_d_n0, eq52_e2006_d_n1, eq52_e2006_d_n2, eq52_e2006_d_n3, eq52_e2006_d_n4, eq52_e2006_d_n5, eq52_e2006_d_n6, eq52_e2006_d_n7, eq52_e2006_d_n8, eq52_e2006_d_n9, eq52_e2006_d_n10, eq52_e2006_d_n11, eq52_e2006_d_n12, eq52_e2006_d_n13, eq52_e2006_d_b0, eq52_e2006_d_b1, eq52_e2006_d_b2, eq52_e2006_d_b3, eq52_e2006_d_b4, eq52_e2006_d_b5, eq52_e2006_d_b6, eq52_e2006_d_b7, eq52_e2006_d_b8, eq52_e2006_d_b9, eq52_e2006_d_b10, eq52_e2006_d_b11,) = {
    if s.b[2011] {
        let eq52_e2004: f64 = (s.v[1095] + s.v[1096]);let eq52_e2004_d_n0: f64 = (s.dn[1095][0] + s.dn[1096][0]);let eq52_e2004_d_n1: f64 = (s.dn[1095][1] + s.dn[1096][1]);let eq52_e2004_d_n2: f64 = (s.dn[1095][2] + s.dn[1096][2]);let eq52_e2004_d_n3: f64 = (s.dn[1095][3] + s.dn[1096][3]);let eq52_e2004_d_n4: f64 = (s.dn[1095][4] + s.dn[1096][4]);let eq52_e2004_d_n5: f64 = (s.dn[1095][5] + s.dn[1096][5]);let eq52_e2004_d_n6: f64 = (s.dn[1095][6] + s.dn[1096][6]);let eq52_e2004_d_n7: f64 = (s.dn[1095][7] + s.dn[1096][7]);let eq52_e2004_d_n8: f64 = (s.dn[1095][8] + s.dn[1096][8]);let eq52_e2004_d_n9: f64 = (s.dn[1095][9] + s.dn[1096][9]);let eq52_e2004_d_n10: f64 = (s.dn[1095][10] + s.dn[1096][10]);let eq52_e2004_d_n11: f64 = (s.dn[1095][11] + s.dn[1096][11]);let eq52_e2004_d_n12: f64 = (s.dn[1095][12] + s.dn[1096][12]);let eq52_e2004_d_n13: f64 = (s.dn[1095][13] + s.dn[1096][13]);let eq52_e2004_d_b0: f64 = (s.db[1095][0] + s.db[1096][0]);let eq52_e2004_d_b1: f64 = (s.db[1095][1] + s.db[1096][1]);let eq52_e2004_d_b2: f64 = (s.db[1095][2] + s.db[1096][2]);let eq52_e2004_d_b3: f64 = (s.db[1095][3] + s.db[1096][3]);let eq52_e2004_d_b4: f64 = (s.db[1095][4] + s.db[1096][4]);let eq52_e2004_d_b5: f64 = (s.db[1095][5] + s.db[1096][5]);let eq52_e2004_d_b6: f64 = (s.db[1095][6] + s.db[1096][6]);let eq52_e2004_d_b7: f64 = (s.db[1095][7] + s.db[1096][7]);let eq52_e2004_d_b8: f64 = (s.db[1095][8] + s.db[1096][8]);let eq52_e2004_d_b9: f64 = (s.db[1095][9] + s.db[1096][9]);let eq52_e2004_d_b10: f64 = (s.db[1095][10] + s.db[1096][10]);let eq52_e2004_d_b11: f64 = (s.db[1095][11] + s.db[1096][11]);
        (eq52_e2004, eq52_e2004_d_n0, eq52_e2004_d_n1, eq52_e2004_d_n2, eq52_e2004_d_n3, eq52_e2004_d_n4, eq52_e2004_d_n5, eq52_e2004_d_n6, eq52_e2004_d_n7, eq52_e2004_d_n8, eq52_e2004_d_n9, eq52_e2004_d_n10, eq52_e2004_d_n11, eq52_e2004_d_n12, eq52_e2004_d_n13, eq52_e2004_d_b0, eq52_e2004_d_b1, eq52_e2004_d_b2, eq52_e2004_d_b3, eq52_e2004_d_b4, eq52_e2004_d_b5, eq52_e2004_d_b6, eq52_e2004_d_b7, eq52_e2004_d_b8, eq52_e2004_d_b9, eq52_e2004_d_b10, eq52_e2004_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2006;let eq52_node_derivatives: [f64; 14] = [eq52_e2006_d_n0, eq52_e2006_d_n1, eq52_e2006_d_n2, eq52_e2006_d_n3, eq52_e2006_d_n4, eq52_e2006_d_n5, eq52_e2006_d_n6, eq52_e2006_d_n7, eq52_e2006_d_n8, eq52_e2006_d_n9, eq52_e2006_d_n10, eq52_e2006_d_n11, eq52_e2006_d_n12, eq52_e2006_d_n13];let eq52_branch_derivatives: [f64; 12] = [eq52_e2006_d_b0, eq52_e2006_d_b1, eq52_e2006_d_b2, eq52_e2006_d_b3, eq52_e2006_d_b4, eq52_e2006_d_b5, eq52_e2006_d_b6, eq52_e2006_d_b7, eq52_e2006_d_b8, eq52_e2006_d_b9, eq52_e2006_d_b10, eq52_e2006_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2010, eq53_e2010_d_n0, eq53_e2010_d_n1, eq53_e2010_d_n2, eq53_e2010_d_n3, eq53_e2010_d_n4, eq53_e2010_d_n5, eq53_e2010_d_n6, eq53_e2010_d_n7, eq53_e2010_d_n8, eq53_e2010_d_n9, eq53_e2010_d_n10, eq53_e2010_d_n11, eq53_e2010_d_n12, eq53_e2010_d_n13, eq53_e2010_d_b0, eq53_e2010_d_b1, eq53_e2010_d_b2, eq53_e2010_d_b3, eq53_e2010_d_b4, eq53_e2010_d_b5, eq53_e2010_d_b6, eq53_e2010_d_b7, eq53_e2010_d_b8, eq53_e2010_d_b9, eq53_e2010_d_b10, eq53_e2010_d_b11,) = {
    if s.b[2011] {
        (s.v[1097], s.dn[1097][0], s.dn[1097][1], s.dn[1097][2], s.dn[1097][3], s.dn[1097][4], s.dn[1097][5], s.dn[1097][6], s.dn[1097][7], s.dn[1097][8], s.dn[1097][9], s.dn[1097][10], s.dn[1097][11], s.dn[1097][12], s.dn[1097][13], s.db[1097][0], s.db[1097][1], s.db[1097][2], s.db[1097][3], s.db[1097][4], s.db[1097][5], s.db[1097][6], s.db[1097][7], s.db[1097][8], s.db[1097][9], s.db[1097][10], s.db[1097][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2010;let eq53_node_derivatives: [f64; 14] = [eq53_e2010_d_n0, eq53_e2010_d_n1, eq53_e2010_d_n2, eq53_e2010_d_n3, eq53_e2010_d_n4, eq53_e2010_d_n5, eq53_e2010_d_n6, eq53_e2010_d_n7, eq53_e2010_d_n8, eq53_e2010_d_n9, eq53_e2010_d_n10, eq53_e2010_d_n11, eq53_e2010_d_n12, eq53_e2010_d_n13];let eq53_branch_derivatives: [f64; 12] = [eq53_e2010_d_b0, eq53_e2010_d_b1, eq53_e2010_d_b2, eq53_e2010_d_b3, eq53_e2010_d_b4, eq53_e2010_d_b5, eq53_e2010_d_b6, eq53_e2010_d_b7, eq53_e2010_d_b8, eq53_e2010_d_b9, eq53_e2010_d_b10, eq53_e2010_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2015, eq54_e2015_d_n0, eq54_e2015_d_n1, eq54_e2015_d_n2, eq54_e2015_d_n3, eq54_e2015_d_n4, eq54_e2015_d_n5, eq54_e2015_d_n6, eq54_e2015_d_n7, eq54_e2015_d_n8, eq54_e2015_d_n9, eq54_e2015_d_n10, eq54_e2015_d_n11, eq54_e2015_d_n12, eq54_e2015_d_n13, eq54_e2015_d_b0, eq54_e2015_d_b1, eq54_e2015_d_b2, eq54_e2015_d_b3, eq54_e2015_d_b4, eq54_e2015_d_b5, eq54_e2015_d_b6, eq54_e2015_d_b7, eq54_e2015_d_b8, eq54_e2015_d_b9, eq54_e2015_d_b10, eq54_e2015_d_b11,) = {
    if (!s.b[2011]) {
        (s.v[1096], s.dn[1096][0], s.dn[1096][1], s.dn[1096][2], s.dn[1096][3], s.dn[1096][4], s.dn[1096][5], s.dn[1096][6], s.dn[1096][7], s.dn[1096][8], s.dn[1096][9], s.dn[1096][10], s.dn[1096][11], s.dn[1096][12], s.dn[1096][13], s.db[1096][0], s.db[1096][1], s.db[1096][2], s.db[1096][3], s.db[1096][4], s.db[1096][5], s.db[1096][6], s.db[1096][7], s.db[1096][8], s.db[1096][9], s.db[1096][10], s.db[1096][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2015;let eq54_node_derivatives: [f64; 14] = [eq54_e2015_d_n0, eq54_e2015_d_n1, eq54_e2015_d_n2, eq54_e2015_d_n3, eq54_e2015_d_n4, eq54_e2015_d_n5, eq54_e2015_d_n6, eq54_e2015_d_n7, eq54_e2015_d_n8, eq54_e2015_d_n9, eq54_e2015_d_n10, eq54_e2015_d_n11, eq54_e2015_d_n12, eq54_e2015_d_n13];let eq54_branch_derivatives: [f64; 12] = [eq54_e2015_d_b0, eq54_e2015_d_b1, eq54_e2015_d_b2, eq54_e2015_d_b3, eq54_e2015_d_b4, eq54_e2015_d_b5, eq54_e2015_d_b6, eq54_e2015_d_b7, eq54_e2015_d_b8, eq54_e2015_d_b9, eq54_e2015_d_b10, eq54_e2015_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2022, eq55_e2022_d_n0, eq55_e2022_d_n1, eq55_e2022_d_n2, eq55_e2022_d_n3, eq55_e2022_d_n4, eq55_e2022_d_n5, eq55_e2022_d_n6, eq55_e2022_d_n7, eq55_e2022_d_n8, eq55_e2022_d_n9, eq55_e2022_d_n10, eq55_e2022_d_n11, eq55_e2022_d_n12, eq55_e2022_d_n13, eq55_e2022_d_b0, eq55_e2022_d_b1, eq55_e2022_d_b2, eq55_e2022_d_b3, eq55_e2022_d_b4, eq55_e2022_d_b5, eq55_e2022_d_b6, eq55_e2022_d_b7, eq55_e2022_d_b8, eq55_e2022_d_b9, eq55_e2022_d_b10, eq55_e2022_d_b11,) = {
    if (!s.b[2011]) {
        let eq55_e2020: f64 = (s.v[1095] + s.v[1097]);let eq55_e2020_d_n0: f64 = (s.dn[1095][0] + s.dn[1097][0]);let eq55_e2020_d_n1: f64 = (s.dn[1095][1] + s.dn[1097][1]);let eq55_e2020_d_n2: f64 = (s.dn[1095][2] + s.dn[1097][2]);let eq55_e2020_d_n3: f64 = (s.dn[1095][3] + s.dn[1097][3]);let eq55_e2020_d_n4: f64 = (s.dn[1095][4] + s.dn[1097][4]);let eq55_e2020_d_n5: f64 = (s.dn[1095][5] + s.dn[1097][5]);let eq55_e2020_d_n6: f64 = (s.dn[1095][6] + s.dn[1097][6]);let eq55_e2020_d_n7: f64 = (s.dn[1095][7] + s.dn[1097][7]);let eq55_e2020_d_n8: f64 = (s.dn[1095][8] + s.dn[1097][8]);let eq55_e2020_d_n9: f64 = (s.dn[1095][9] + s.dn[1097][9]);let eq55_e2020_d_n10: f64 = (s.dn[1095][10] + s.dn[1097][10]);let eq55_e2020_d_n11: f64 = (s.dn[1095][11] + s.dn[1097][11]);let eq55_e2020_d_n12: f64 = (s.dn[1095][12] + s.dn[1097][12]);let eq55_e2020_d_n13: f64 = (s.dn[1095][13] + s.dn[1097][13]);let eq55_e2020_d_b0: f64 = (s.db[1095][0] + s.db[1097][0]);let eq55_e2020_d_b1: f64 = (s.db[1095][1] + s.db[1097][1]);let eq55_e2020_d_b2: f64 = (s.db[1095][2] + s.db[1097][2]);let eq55_e2020_d_b3: f64 = (s.db[1095][3] + s.db[1097][3]);let eq55_e2020_d_b4: f64 = (s.db[1095][4] + s.db[1097][4]);let eq55_e2020_d_b5: f64 = (s.db[1095][5] + s.db[1097][5]);let eq55_e2020_d_b6: f64 = (s.db[1095][6] + s.db[1097][6]);let eq55_e2020_d_b7: f64 = (s.db[1095][7] + s.db[1097][7]);let eq55_e2020_d_b8: f64 = (s.db[1095][8] + s.db[1097][8]);let eq55_e2020_d_b9: f64 = (s.db[1095][9] + s.db[1097][9]);let eq55_e2020_d_b10: f64 = (s.db[1095][10] + s.db[1097][10]);let eq55_e2020_d_b11: f64 = (s.db[1095][11] + s.db[1097][11]);
        (eq55_e2020, eq55_e2020_d_n0, eq55_e2020_d_n1, eq55_e2020_d_n2, eq55_e2020_d_n3, eq55_e2020_d_n4, eq55_e2020_d_n5, eq55_e2020_d_n6, eq55_e2020_d_n7, eq55_e2020_d_n8, eq55_e2020_d_n9, eq55_e2020_d_n10, eq55_e2020_d_n11, eq55_e2020_d_n12, eq55_e2020_d_n13, eq55_e2020_d_b0, eq55_e2020_d_b1, eq55_e2020_d_b2, eq55_e2020_d_b3, eq55_e2020_d_b4, eq55_e2020_d_b5, eq55_e2020_d_b6, eq55_e2020_d_b7, eq55_e2020_d_b8, eq55_e2020_d_b9, eq55_e2020_d_b10, eq55_e2020_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2022;let eq55_node_derivatives: [f64; 14] = [eq55_e2022_d_n0, eq55_e2022_d_n1, eq55_e2022_d_n2, eq55_e2022_d_n3, eq55_e2022_d_n4, eq55_e2022_d_n5, eq55_e2022_d_n6, eq55_e2022_d_n7, eq55_e2022_d_n8, eq55_e2022_d_n9, eq55_e2022_d_n10, eq55_e2022_d_n11, eq55_e2022_d_n12, eq55_e2022_d_n13];let eq55_branch_derivatives: [f64; 12] = [eq55_e2022_d_b0, eq55_e2022_d_b1, eq55_e2022_d_b2, eq55_e2022_d_b3, eq55_e2022_d_b4, eq55_e2022_d_b5, eq55_e2022_d_b6, eq55_e2022_d_b7, eq55_e2022_d_b8, eq55_e2022_d_b9, eq55_e2022_d_b10, eq55_e2022_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2026,) = {
    if s.b[2012] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e2026;
        stamper.stamp_potential_const_local(
            2,
            eq56_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_16(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv6 = ctx.node_voltage(nodes[6]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq57_e2033, eq57_e2033_d_n0, eq57_e2033_d_n1, eq57_e2033_d_n2, eq57_e2033_d_n3, eq57_e2033_d_n4, eq57_e2033_d_n5, eq57_e2033_d_n6, eq57_e2033_d_n7, eq57_e2033_d_n8, eq57_e2033_d_n9, eq57_e2033_d_n10, eq57_e2033_d_n11, eq57_e2033_d_n12, eq57_e2033_d_n13, eq57_e2033_d_b0, eq57_e2033_d_b1, eq57_e2033_d_b2, eq57_e2033_d_b3, eq57_e2033_d_b4, eq57_e2033_d_b5, eq57_e2033_d_b6, eq57_e2033_d_b7, eq57_e2033_d_b8, eq57_e2033_d_b9, eq57_e2033_d_b10, eq57_e2033_d_b11,) = {
    if (!s.b[2012]) {
        let eq57_e2031: f64 = ((nv1 - nv9) * s.v[2013]);let eq57_e2031_d_n0: f64 = ((nv1 - nv9) * s.dn[2013][0]);let eq57_e2031_d_n1: f64 = (s.v[2013] + ((nv1 - nv9) * s.dn[2013][1]));let eq57_e2031_d_n2: f64 = ((nv1 - nv9) * s.dn[2013][2]);let eq57_e2031_d_n3: f64 = ((nv1 - nv9) * s.dn[2013][3]);let eq57_e2031_d_n4: f64 = ((nv1 - nv9) * s.dn[2013][4]);let eq57_e2031_d_n5: f64 = ((nv1 - nv9) * s.dn[2013][5]);let eq57_e2031_d_n6: f64 = ((nv1 - nv9) * s.dn[2013][6]);let eq57_e2031_d_n7: f64 = ((nv1 - nv9) * s.dn[2013][7]);let eq57_e2031_d_n8: f64 = ((nv1 - nv9) * s.dn[2013][8]);let eq57_e2031_d_n9: f64 = ((-s.v[2013]) + ((nv1 - nv9) * s.dn[2013][9]));let eq57_e2031_d_n10: f64 = ((nv1 - nv9) * s.dn[2013][10]);let eq57_e2031_d_n11: f64 = ((nv1 - nv9) * s.dn[2013][11]);let eq57_e2031_d_n12: f64 = ((nv1 - nv9) * s.dn[2013][12]);let eq57_e2031_d_n13: f64 = ((nv1 - nv9) * s.dn[2013][13]);let eq57_e2031_d_b0: f64 = ((nv1 - nv9) * s.db[2013][0]);let eq57_e2031_d_b1: f64 = ((nv1 - nv9) * s.db[2013][1]);let eq57_e2031_d_b2: f64 = ((nv1 - nv9) * s.db[2013][2]);let eq57_e2031_d_b3: f64 = ((nv1 - nv9) * s.db[2013][3]);let eq57_e2031_d_b4: f64 = ((nv1 - nv9) * s.db[2013][4]);let eq57_e2031_d_b5: f64 = ((nv1 - nv9) * s.db[2013][5]);let eq57_e2031_d_b6: f64 = ((nv1 - nv9) * s.db[2013][6]);let eq57_e2031_d_b7: f64 = ((nv1 - nv9) * s.db[2013][7]);let eq57_e2031_d_b8: f64 = ((nv1 - nv9) * s.db[2013][8]);let eq57_e2031_d_b9: f64 = ((nv1 - nv9) * s.db[2013][9]);let eq57_e2031_d_b10: f64 = ((nv1 - nv9) * s.db[2013][10]);let eq57_e2031_d_b11: f64 = ((nv1 - nv9) * s.db[2013][11]);
        (eq57_e2031, eq57_e2031_d_n0, eq57_e2031_d_n1, eq57_e2031_d_n2, eq57_e2031_d_n3, eq57_e2031_d_n4, eq57_e2031_d_n5, eq57_e2031_d_n6, eq57_e2031_d_n7, eq57_e2031_d_n8, eq57_e2031_d_n9, eq57_e2031_d_n10, eq57_e2031_d_n11, eq57_e2031_d_n12, eq57_e2031_d_n13, eq57_e2031_d_b0, eq57_e2031_d_b1, eq57_e2031_d_b2, eq57_e2031_d_b3, eq57_e2031_d_b4, eq57_e2031_d_b5, eq57_e2031_d_b6, eq57_e2031_d_b7, eq57_e2031_d_b8, eq57_e2031_d_b9, eq57_e2031_d_b10, eq57_e2031_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2033;let eq57_node_derivatives: [f64; 14] = [eq57_e2033_d_n0, eq57_e2033_d_n1, eq57_e2033_d_n2, eq57_e2033_d_n3, eq57_e2033_d_n4, eq57_e2033_d_n5, eq57_e2033_d_n6, eq57_e2033_d_n7, eq57_e2033_d_n8, eq57_e2033_d_n9, eq57_e2033_d_n10, eq57_e2033_d_n11, eq57_e2033_d_n12, eq57_e2033_d_n13];let eq57_branch_derivatives: [f64; 12] = [eq57_e2033_d_b0, eq57_e2033_d_b1, eq57_e2033_d_b2, eq57_e2033_d_b3, eq57_e2033_d_b4, eq57_e2033_d_b5, eq57_e2033_d_b6, eq57_e2033_d_b7, eq57_e2033_d_b8, eq57_e2033_d_b9, eq57_e2033_d_b10, eq57_e2033_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq59_e2048, eq59_e2048_d_n0, eq59_e2048_d_n1, eq59_e2048_d_n2, eq59_e2048_d_n3, eq59_e2048_d_n4, eq59_e2048_d_n5, eq59_e2048_d_n6, eq59_e2048_d_n7, eq59_e2048_d_n8, eq59_e2048_d_n9, eq59_e2048_d_n10, eq59_e2048_d_n11, eq59_e2048_d_n12, eq59_e2048_d_n13, eq59_e2048_d_b0, eq59_e2048_d_b1, eq59_e2048_d_b2, eq59_e2048_d_b3, eq59_e2048_d_b4, eq59_e2048_d_b5, eq59_e2048_d_b6, eq59_e2048_d_b7, eq59_e2048_d_b8, eq59_e2048_d_b9, eq59_e2048_d_b10, eq59_e2048_d_b11,) = {
    if s.b[2016] {
        let eq59_e2046: f64 = ((nv0 - nv6) * s.v[618]);let eq59_e2046_d_n0: f64 = (s.v[618] + ((nv0 - nv6) * s.dn[618][0]));let eq59_e2046_d_n1: f64 = ((nv0 - nv6) * s.dn[618][1]);let eq59_e2046_d_n2: f64 = ((nv0 - nv6) * s.dn[618][2]);let eq59_e2046_d_n3: f64 = ((nv0 - nv6) * s.dn[618][3]);let eq59_e2046_d_n4: f64 = ((nv0 - nv6) * s.dn[618][4]);let eq59_e2046_d_n5: f64 = ((nv0 - nv6) * s.dn[618][5]);let eq59_e2046_d_n6: f64 = ((-s.v[618]) + ((nv0 - nv6) * s.dn[618][6]));let eq59_e2046_d_n7: f64 = ((nv0 - nv6) * s.dn[618][7]);let eq59_e2046_d_n8: f64 = ((nv0 - nv6) * s.dn[618][8]);let eq59_e2046_d_n9: f64 = ((nv0 - nv6) * s.dn[618][9]);let eq59_e2046_d_n10: f64 = ((nv0 - nv6) * s.dn[618][10]);let eq59_e2046_d_n11: f64 = ((nv0 - nv6) * s.dn[618][11]);let eq59_e2046_d_n12: f64 = ((nv0 - nv6) * s.dn[618][12]);let eq59_e2046_d_n13: f64 = ((nv0 - nv6) * s.dn[618][13]);let eq59_e2046_d_b0: f64 = ((nv0 - nv6) * s.db[618][0]);let eq59_e2046_d_b1: f64 = ((nv0 - nv6) * s.db[618][1]);let eq59_e2046_d_b2: f64 = ((nv0 - nv6) * s.db[618][2]);let eq59_e2046_d_b3: f64 = ((nv0 - nv6) * s.db[618][3]);let eq59_e2046_d_b4: f64 = ((nv0 - nv6) * s.db[618][4]);let eq59_e2046_d_b5: f64 = ((nv0 - nv6) * s.db[618][5]);let eq59_e2046_d_b6: f64 = ((nv0 - nv6) * s.db[618][6]);let eq59_e2046_d_b7: f64 = ((nv0 - nv6) * s.db[618][7]);let eq59_e2046_d_b8: f64 = ((nv0 - nv6) * s.db[618][8]);let eq59_e2046_d_b9: f64 = ((nv0 - nv6) * s.db[618][9]);let eq59_e2046_d_b10: f64 = ((nv0 - nv6) * s.db[618][10]);let eq59_e2046_d_b11: f64 = ((nv0 - nv6) * s.db[618][11]);
        (eq59_e2046, eq59_e2046_d_n0, eq59_e2046_d_n1, eq59_e2046_d_n2, eq59_e2046_d_n3, eq59_e2046_d_n4, eq59_e2046_d_n5, eq59_e2046_d_n6, eq59_e2046_d_n7, eq59_e2046_d_n8, eq59_e2046_d_n9, eq59_e2046_d_n10, eq59_e2046_d_n11, eq59_e2046_d_n12, eq59_e2046_d_n13, eq59_e2046_d_b0, eq59_e2046_d_b1, eq59_e2046_d_b2, eq59_e2046_d_b3, eq59_e2046_d_b4, eq59_e2046_d_b5, eq59_e2046_d_b6, eq59_e2046_d_b7, eq59_e2046_d_b8, eq59_e2046_d_b9, eq59_e2046_d_b10, eq59_e2046_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2048;let eq59_node_derivatives: [f64; 14] = [eq59_e2048_d_n0, eq59_e2048_d_n1, eq59_e2048_d_n2, eq59_e2048_d_n3, eq59_e2048_d_n4, eq59_e2048_d_n5, eq59_e2048_d_n6, eq59_e2048_d_n7, eq59_e2048_d_n8, eq59_e2048_d_n9, eq59_e2048_d_n10, eq59_e2048_d_n11, eq59_e2048_d_n12, eq59_e2048_d_n13];let eq59_branch_derivatives: [f64; 12] = [eq59_e2048_d_b0, eq59_e2048_d_b1, eq59_e2048_d_b2, eq59_e2048_d_b3, eq59_e2048_d_b4, eq59_e2048_d_b5, eq59_e2048_d_b6, eq59_e2048_d_b7, eq59_e2048_d_b8, eq59_e2048_d_b9, eq59_e2048_d_b10, eq59_e2048_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e2053,) = {
    if (!s.b[2016]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2053;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq62_e2067, eq62_e2067_d_n0, eq62_e2067_d_n1, eq62_e2067_d_n2, eq62_e2067_d_n3, eq62_e2067_d_n4, eq62_e2067_d_n5, eq62_e2067_d_n6, eq62_e2067_d_n7, eq62_e2067_d_n8, eq62_e2067_d_n9, eq62_e2067_d_n10, eq62_e2067_d_n11, eq62_e2067_d_n12, eq62_e2067_d_n13, eq62_e2067_d_b0, eq62_e2067_d_b1, eq62_e2067_d_b2, eq62_e2067_d_b3, eq62_e2067_d_b4, eq62_e2067_d_b5, eq62_e2067_d_b6, eq62_e2067_d_b7, eq62_e2067_d_b8, eq62_e2067_d_b9, eq62_e2067_d_b10, eq62_e2067_d_b11,) = {
    if s.b[2018] {
        let eq62_e2065: f64 = ((nv2 - nv7) * s.v[617]);let eq62_e2065_d_n0: f64 = ((nv2 - nv7) * s.dn[617][0]);let eq62_e2065_d_n1: f64 = ((nv2 - nv7) * s.dn[617][1]);let eq62_e2065_d_n2: f64 = (s.v[617] + ((nv2 - nv7) * s.dn[617][2]));let eq62_e2065_d_n3: f64 = ((nv2 - nv7) * s.dn[617][3]);let eq62_e2065_d_n4: f64 = ((nv2 - nv7) * s.dn[617][4]);let eq62_e2065_d_n5: f64 = ((nv2 - nv7) * s.dn[617][5]);let eq62_e2065_d_n6: f64 = ((nv2 - nv7) * s.dn[617][6]);let eq62_e2065_d_n7: f64 = ((-s.v[617]) + ((nv2 - nv7) * s.dn[617][7]));let eq62_e2065_d_n8: f64 = ((nv2 - nv7) * s.dn[617][8]);let eq62_e2065_d_n9: f64 = ((nv2 - nv7) * s.dn[617][9]);let eq62_e2065_d_n10: f64 = ((nv2 - nv7) * s.dn[617][10]);let eq62_e2065_d_n11: f64 = ((nv2 - nv7) * s.dn[617][11]);let eq62_e2065_d_n12: f64 = ((nv2 - nv7) * s.dn[617][12]);let eq62_e2065_d_n13: f64 = ((nv2 - nv7) * s.dn[617][13]);let eq62_e2065_d_b0: f64 = ((nv2 - nv7) * s.db[617][0]);let eq62_e2065_d_b1: f64 = ((nv2 - nv7) * s.db[617][1]);let eq62_e2065_d_b2: f64 = ((nv2 - nv7) * s.db[617][2]);let eq62_e2065_d_b3: f64 = ((nv2 - nv7) * s.db[617][3]);let eq62_e2065_d_b4: f64 = ((nv2 - nv7) * s.db[617][4]);let eq62_e2065_d_b5: f64 = ((nv2 - nv7) * s.db[617][5]);let eq62_e2065_d_b6: f64 = ((nv2 - nv7) * s.db[617][6]);let eq62_e2065_d_b7: f64 = ((nv2 - nv7) * s.db[617][7]);let eq62_e2065_d_b8: f64 = ((nv2 - nv7) * s.db[617][8]);let eq62_e2065_d_b9: f64 = ((nv2 - nv7) * s.db[617][9]);let eq62_e2065_d_b10: f64 = ((nv2 - nv7) * s.db[617][10]);let eq62_e2065_d_b11: f64 = ((nv2 - nv7) * s.db[617][11]);
        (eq62_e2065, eq62_e2065_d_n0, eq62_e2065_d_n1, eq62_e2065_d_n2, eq62_e2065_d_n3, eq62_e2065_d_n4, eq62_e2065_d_n5, eq62_e2065_d_n6, eq62_e2065_d_n7, eq62_e2065_d_n8, eq62_e2065_d_n9, eq62_e2065_d_n10, eq62_e2065_d_n11, eq62_e2065_d_n12, eq62_e2065_d_n13, eq62_e2065_d_b0, eq62_e2065_d_b1, eq62_e2065_d_b2, eq62_e2065_d_b3, eq62_e2065_d_b4, eq62_e2065_d_b5, eq62_e2065_d_b6, eq62_e2065_d_b7, eq62_e2065_d_b8, eq62_e2065_d_b9, eq62_e2065_d_b10, eq62_e2065_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2067;let eq62_node_derivatives: [f64; 14] = [eq62_e2067_d_n0, eq62_e2067_d_n1, eq62_e2067_d_n2, eq62_e2067_d_n3, eq62_e2067_d_n4, eq62_e2067_d_n5, eq62_e2067_d_n6, eq62_e2067_d_n7, eq62_e2067_d_n8, eq62_e2067_d_n9, eq62_e2067_d_n10, eq62_e2067_d_n11, eq62_e2067_d_n12, eq62_e2067_d_n13];let eq62_branch_derivatives: [f64; 12] = [eq62_e2067_d_b0, eq62_e2067_d_b1, eq62_e2067_d_b2, eq62_e2067_d_b3, eq62_e2067_d_b4, eq62_e2067_d_b5, eq62_e2067_d_b6, eq62_e2067_d_b7, eq62_e2067_d_b8, eq62_e2067_d_b9, eq62_e2067_d_b10, eq62_e2067_d_b11];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2072,) = {
    if (!s.b[2018]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2072;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq65_e2086, eq65_e2086_d_n0, eq65_e2086_d_n1, eq65_e2086_d_n2, eq65_e2086_d_n3, eq65_e2086_d_n4, eq65_e2086_d_n5, eq65_e2086_d_n6, eq65_e2086_d_n7, eq65_e2086_d_n8, eq65_e2086_d_n9, eq65_e2086_d_n10, eq65_e2086_d_n11, eq65_e2086_d_n12, eq65_e2086_d_n13, eq65_e2086_d_b0, eq65_e2086_d_b1, eq65_e2086_d_b2, eq65_e2086_d_b3, eq65_e2086_d_b4, eq65_e2086_d_b5, eq65_e2086_d_b6, eq65_e2086_d_b7, eq65_e2086_d_b8, eq65_e2086_d_b9, eq65_e2086_d_b10, eq65_e2086_d_b11,) = {
    if s.b[2020] {
        let eq65_e2084: f64 = ((nv9 - nv8) * s.v[467]);let eq65_e2084_d_n0: f64 = ((nv9 - nv8) * s.dn[467][0]);let eq65_e2084_d_n1: f64 = ((nv9 - nv8) * s.dn[467][1]);let eq65_e2084_d_n2: f64 = ((nv9 - nv8) * s.dn[467][2]);let eq65_e2084_d_n3: f64 = ((nv9 - nv8) * s.dn[467][3]);let eq65_e2084_d_n4: f64 = ((nv9 - nv8) * s.dn[467][4]);let eq65_e2084_d_n5: f64 = ((nv9 - nv8) * s.dn[467][5]);let eq65_e2084_d_n6: f64 = ((nv9 - nv8) * s.dn[467][6]);let eq65_e2084_d_n7: f64 = ((nv9 - nv8) * s.dn[467][7]);let eq65_e2084_d_n8: f64 = ((-s.v[467]) + ((nv9 - nv8) * s.dn[467][8]));let eq65_e2084_d_n9: f64 = (s.v[467] + ((nv9 - nv8) * s.dn[467][9]));let eq65_e2084_d_n10: f64 = ((nv9 - nv8) * s.dn[467][10]);let eq65_e2084_d_n11: f64 = ((nv9 - nv8) * s.dn[467][11]);let eq65_e2084_d_n12: f64 = ((nv9 - nv8) * s.dn[467][12]);let eq65_e2084_d_n13: f64 = ((nv9 - nv8) * s.dn[467][13]);let eq65_e2084_d_b0: f64 = ((nv9 - nv8) * s.db[467][0]);let eq65_e2084_d_b1: f64 = ((nv9 - nv8) * s.db[467][1]);let eq65_e2084_d_b2: f64 = ((nv9 - nv8) * s.db[467][2]);let eq65_e2084_d_b3: f64 = ((nv9 - nv8) * s.db[467][3]);let eq65_e2084_d_b4: f64 = ((nv9 - nv8) * s.db[467][4]);let eq65_e2084_d_b5: f64 = ((nv9 - nv8) * s.db[467][5]);let eq65_e2084_d_b6: f64 = ((nv9 - nv8) * s.db[467][6]);let eq65_e2084_d_b7: f64 = ((nv9 - nv8) * s.db[467][7]);let eq65_e2084_d_b8: f64 = ((nv9 - nv8) * s.db[467][8]);let eq65_e2084_d_b9: f64 = ((nv9 - nv8) * s.db[467][9]);let eq65_e2084_d_b10: f64 = ((nv9 - nv8) * s.db[467][10]);let eq65_e2084_d_b11: f64 = ((nv9 - nv8) * s.db[467][11]);
        (eq65_e2084, eq65_e2084_d_n0, eq65_e2084_d_n1, eq65_e2084_d_n2, eq65_e2084_d_n3, eq65_e2084_d_n4, eq65_e2084_d_n5, eq65_e2084_d_n6, eq65_e2084_d_n7, eq65_e2084_d_n8, eq65_e2084_d_n9, eq65_e2084_d_n10, eq65_e2084_d_n11, eq65_e2084_d_n12, eq65_e2084_d_n13, eq65_e2084_d_b0, eq65_e2084_d_b1, eq65_e2084_d_b2, eq65_e2084_d_b3, eq65_e2084_d_b4, eq65_e2084_d_b5, eq65_e2084_d_b6, eq65_e2084_d_b7, eq65_e2084_d_b8, eq65_e2084_d_b9, eq65_e2084_d_b10, eq65_e2084_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2086;let eq65_node_derivatives: [f64; 14] = [eq65_e2086_d_n0, eq65_e2086_d_n1, eq65_e2086_d_n2, eq65_e2086_d_n3, eq65_e2086_d_n4, eq65_e2086_d_n5, eq65_e2086_d_n6, eq65_e2086_d_n7, eq65_e2086_d_n8, eq65_e2086_d_n9, eq65_e2086_d_n10, eq65_e2086_d_n11, eq65_e2086_d_n12, eq65_e2086_d_n13];let eq65_branch_derivatives: [f64; 12] = [eq65_e2086_d_b0, eq65_e2086_d_b1, eq65_e2086_d_b2, eq65_e2086_d_b3, eq65_e2086_d_b4, eq65_e2086_d_b5, eq65_e2086_d_b6, eq65_e2086_d_b7, eq65_e2086_d_b8, eq65_e2086_d_b9, eq65_e2086_d_b10, eq65_e2086_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e2091,) = {
    if (!s.b[2020]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2091;
        stamper.stamp_potential_const_local(
            5,
            eq66_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_18(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let (eq67_e2108, eq67_e2108_d_n0, eq67_e2108_d_n1, eq67_e2108_d_n2, eq67_e2108_d_n3, eq67_e2108_d_n4, eq67_e2108_d_n5, eq67_e2108_d_n6, eq67_e2108_d_n7, eq67_e2108_d_n8, eq67_e2108_d_n9, eq67_e2108_d_n10, eq67_e2108_d_n11, eq67_e2108_d_n12, eq67_e2108_d_n13, eq67_e2108_d_b0, eq67_e2108_d_b1, eq67_e2108_d_b2, eq67_e2108_d_b3, eq67_e2108_d_b4, eq67_e2108_d_b5, eq67_e2108_d_b6, eq67_e2108_d_b7, eq67_e2108_d_b8, eq67_e2108_d_b9, eq67_e2108_d_b10, eq67_e2108_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2099: f64 = (s.v[634] * s.v[1015]);let eq67_e2099_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq67_e2099_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq67_e2099_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq67_e2099_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq67_e2099_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq67_e2099_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq67_e2099_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq67_e2099_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq67_e2099_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq67_e2099_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq67_e2099_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq67_e2099_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq67_e2099_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq67_e2099_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq67_e2099_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq67_e2099_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq67_e2099_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq67_e2099_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq67_e2099_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq67_e2099_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq67_e2099_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq67_e2099_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq67_e2099_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq67_e2099_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq67_e2099_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq67_e2099_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq67_e2102: f64 = (s.v[634] * s.v[1016]);let eq67_e2102_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq67_e2102_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq67_e2102_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq67_e2102_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq67_e2102_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq67_e2102_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq67_e2102_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq67_e2102_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq67_e2102_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq67_e2102_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq67_e2102_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq67_e2102_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq67_e2102_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq67_e2102_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq67_e2102_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq67_e2102_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq67_e2102_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq67_e2102_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq67_e2102_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq67_e2102_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq67_e2102_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq67_e2102_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq67_e2102_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq67_e2102_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq67_e2102_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq67_e2102_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq67_e2103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq67_e2102);let eq67_e2104: f64 = (eq67_e2099 + eq67_e2103);let eq67_e2104_d_n0: f64 = (eq67_e2099_d_n0 + (eq67_e2102_d_n0 * ddt_scale));let eq67_e2104_d_n1: f64 = (eq67_e2099_d_n1 + (eq67_e2102_d_n1 * ddt_scale));let eq67_e2104_d_n2: f64 = (eq67_e2099_d_n2 + (eq67_e2102_d_n2 * ddt_scale));let eq67_e2104_d_n3: f64 = (eq67_e2099_d_n3 + (eq67_e2102_d_n3 * ddt_scale));let eq67_e2104_d_n4: f64 = (eq67_e2099_d_n4 + (eq67_e2102_d_n4 * ddt_scale));let eq67_e2104_d_n5: f64 = (eq67_e2099_d_n5 + (eq67_e2102_d_n5 * ddt_scale));let eq67_e2104_d_n6: f64 = (eq67_e2099_d_n6 + (eq67_e2102_d_n6 * ddt_scale));let eq67_e2104_d_n7: f64 = (eq67_e2099_d_n7 + (eq67_e2102_d_n7 * ddt_scale));let eq67_e2104_d_n8: f64 = (eq67_e2099_d_n8 + (eq67_e2102_d_n8 * ddt_scale));let eq67_e2104_d_n9: f64 = (eq67_e2099_d_n9 + (eq67_e2102_d_n9 * ddt_scale));let eq67_e2104_d_n10: f64 = (eq67_e2099_d_n10 + (eq67_e2102_d_n10 * ddt_scale));let eq67_e2104_d_n11: f64 = (eq67_e2099_d_n11 + (eq67_e2102_d_n11 * ddt_scale));let eq67_e2104_d_n12: f64 = (eq67_e2099_d_n12 + (eq67_e2102_d_n12 * ddt_scale));let eq67_e2104_d_n13: f64 = (eq67_e2099_d_n13 + (eq67_e2102_d_n13 * ddt_scale));let eq67_e2104_d_b0: f64 = (eq67_e2099_d_b0 + (eq67_e2102_d_b0 * ddt_scale));let eq67_e2104_d_b1: f64 = (eq67_e2099_d_b1 + (eq67_e2102_d_b1 * ddt_scale));let eq67_e2104_d_b2: f64 = (eq67_e2099_d_b2 + (eq67_e2102_d_b2 * ddt_scale));let eq67_e2104_d_b3: f64 = (eq67_e2099_d_b3 + (eq67_e2102_d_b3 * ddt_scale));let eq67_e2104_d_b4: f64 = (eq67_e2099_d_b4 + (eq67_e2102_d_b4 * ddt_scale));let eq67_e2104_d_b5: f64 = (eq67_e2099_d_b5 + (eq67_e2102_d_b5 * ddt_scale));let eq67_e2104_d_b6: f64 = (eq67_e2099_d_b6 + (eq67_e2102_d_b6 * ddt_scale));let eq67_e2104_d_b7: f64 = (eq67_e2099_d_b7 + (eq67_e2102_d_b7 * ddt_scale));let eq67_e2104_d_b8: f64 = (eq67_e2099_d_b8 + (eq67_e2102_d_b8 * ddt_scale));let eq67_e2104_d_b9: f64 = (eq67_e2099_d_b9 + (eq67_e2102_d_b9 * ddt_scale));let eq67_e2104_d_b10: f64 = (eq67_e2099_d_b10 + (eq67_e2102_d_b10 * ddt_scale));let eq67_e2104_d_b11: f64 = (eq67_e2099_d_b11 + (eq67_e2102_d_b11 * ddt_scale));let eq67_e2106: f64 = (eq67_e2104 - s.v[1017]);let eq67_e2106_d_n0: f64 = (eq67_e2104_d_n0 - s.dn[1017][0]);let eq67_e2106_d_n1: f64 = (eq67_e2104_d_n1 - s.dn[1017][1]);let eq67_e2106_d_n2: f64 = (eq67_e2104_d_n2 - s.dn[1017][2]);let eq67_e2106_d_n3: f64 = (eq67_e2104_d_n3 - s.dn[1017][3]);let eq67_e2106_d_n4: f64 = (eq67_e2104_d_n4 - s.dn[1017][4]);let eq67_e2106_d_n5: f64 = (eq67_e2104_d_n5 - s.dn[1017][5]);let eq67_e2106_d_n6: f64 = (eq67_e2104_d_n6 - s.dn[1017][6]);let eq67_e2106_d_n7: f64 = (eq67_e2104_d_n7 - s.dn[1017][7]);let eq67_e2106_d_n8: f64 = (eq67_e2104_d_n8 - s.dn[1017][8]);let eq67_e2106_d_n9: f64 = (eq67_e2104_d_n9 - s.dn[1017][9]);let eq67_e2106_d_n10: f64 = (eq67_e2104_d_n10 - s.dn[1017][10]);let eq67_e2106_d_n11: f64 = (eq67_e2104_d_n11 - s.dn[1017][11]);let eq67_e2106_d_n12: f64 = (eq67_e2104_d_n12 - s.dn[1017][12]);let eq67_e2106_d_n13: f64 = (eq67_e2104_d_n13 - s.dn[1017][13]);let eq67_e2106_d_b0: f64 = (eq67_e2104_d_b0 - s.db[1017][0]);let eq67_e2106_d_b1: f64 = (eq67_e2104_d_b1 - s.db[1017][1]);let eq67_e2106_d_b2: f64 = (eq67_e2104_d_b2 - s.db[1017][2]);let eq67_e2106_d_b3: f64 = (eq67_e2104_d_b3 - s.db[1017][3]);let eq67_e2106_d_b4: f64 = (eq67_e2104_d_b4 - s.db[1017][4]);
        let eq67_e2106_d_b5: f64 = (eq67_e2104_d_b5 - s.db[1017][5]);let eq67_e2106_d_b6: f64 = (eq67_e2104_d_b6 - s.db[1017][6]);let eq67_e2106_d_b7: f64 = (eq67_e2104_d_b7 - s.db[1017][7]);let eq67_e2106_d_b8: f64 = (eq67_e2104_d_b8 - s.db[1017][8]);let eq67_e2106_d_b9: f64 = (eq67_e2104_d_b9 - s.db[1017][9]);let eq67_e2106_d_b10: f64 = (eq67_e2104_d_b10 - s.db[1017][10]);let eq67_e2106_d_b11: f64 = (eq67_e2104_d_b11 - s.db[1017][11]);
        (eq67_e2106, eq67_e2106_d_n0, eq67_e2106_d_n1, eq67_e2106_d_n2, eq67_e2106_d_n3, eq67_e2106_d_n4, eq67_e2106_d_n5, eq67_e2106_d_n6, eq67_e2106_d_n7, eq67_e2106_d_n8, eq67_e2106_d_n9, eq67_e2106_d_n10, eq67_e2106_d_n11, eq67_e2106_d_n12, eq67_e2106_d_n13, eq67_e2106_d_b0, eq67_e2106_d_b1, eq67_e2106_d_b2, eq67_e2106_d_b3, eq67_e2106_d_b4, eq67_e2106_d_b5, eq67_e2106_d_b6, eq67_e2106_d_b7, eq67_e2106_d_b8, eq67_e2106_d_b9, eq67_e2106_d_b10, eq67_e2106_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2108;let eq67_node_derivatives: [f64; 14] = [eq67_e2108_d_n0, eq67_e2108_d_n1, eq67_e2108_d_n2, eq67_e2108_d_n3, eq67_e2108_d_n4, eq67_e2108_d_n5, eq67_e2108_d_n6, eq67_e2108_d_n7, eq67_e2108_d_n8, eq67_e2108_d_n9, eq67_e2108_d_n10, eq67_e2108_d_n11, eq67_e2108_d_n12, eq67_e2108_d_n13];let eq67_branch_derivatives: [f64; 12] = [eq67_e2108_d_b0, eq67_e2108_d_b1, eq67_e2108_d_b2, eq67_e2108_d_b3, eq67_e2108_d_b4, eq67_e2108_d_b5, eq67_e2108_d_b6, eq67_e2108_d_b7, eq67_e2108_d_b8, eq67_e2108_d_b9, eq67_e2108_d_b10, eq67_e2108_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivatives,
            &eq67_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_19(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let (eq68_e2126, eq68_e2126_d_n0, eq68_e2126_d_n1, eq68_e2126_d_n2, eq68_e2126_d_n3, eq68_e2126_d_n4, eq68_e2126_d_n5, eq68_e2126_d_n6, eq68_e2126_d_n7, eq68_e2126_d_n8, eq68_e2126_d_n9, eq68_e2126_d_n10, eq68_e2126_d_n11, eq68_e2126_d_n12, eq68_e2126_d_n13, eq68_e2126_d_b0, eq68_e2126_d_b1, eq68_e2126_d_b2, eq68_e2126_d_b3, eq68_e2126_d_b4, eq68_e2126_d_b5, eq68_e2126_d_b6, eq68_e2126_d_b7, eq68_e2126_d_b8, eq68_e2126_d_b9, eq68_e2126_d_b10, eq68_e2126_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2117: f64 = (s.v[634] * s.v[1015]);let eq68_e2117_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq68_e2117_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq68_e2117_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq68_e2117_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq68_e2117_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq68_e2117_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq68_e2117_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq68_e2117_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq68_e2117_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq68_e2117_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq68_e2117_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq68_e2117_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq68_e2117_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq68_e2117_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq68_e2117_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq68_e2117_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq68_e2117_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq68_e2117_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq68_e2117_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq68_e2117_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq68_e2117_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq68_e2117_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq68_e2117_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq68_e2117_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq68_e2117_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq68_e2117_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq68_e2120: f64 = (s.v[634] * s.v[1016]);let eq68_e2120_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq68_e2120_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq68_e2120_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq68_e2120_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq68_e2120_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq68_e2120_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq68_e2120_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq68_e2120_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq68_e2120_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq68_e2120_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq68_e2120_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq68_e2120_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq68_e2120_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq68_e2120_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq68_e2120_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq68_e2120_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq68_e2120_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq68_e2120_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq68_e2120_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq68_e2120_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq68_e2120_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq68_e2120_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq68_e2120_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq68_e2120_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq68_e2120_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq68_e2120_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq68_e2121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq68_e2120);let eq68_e2122: f64 = (eq68_e2117 + eq68_e2121);let eq68_e2122_d_n0: f64 = (eq68_e2117_d_n0 + (eq68_e2120_d_n0 * ddt_scale));let eq68_e2122_d_n1: f64 = (eq68_e2117_d_n1 + (eq68_e2120_d_n1 * ddt_scale));let eq68_e2122_d_n2: f64 = (eq68_e2117_d_n2 + (eq68_e2120_d_n2 * ddt_scale));let eq68_e2122_d_n3: f64 = (eq68_e2117_d_n3 + (eq68_e2120_d_n3 * ddt_scale));let eq68_e2122_d_n4: f64 = (eq68_e2117_d_n4 + (eq68_e2120_d_n4 * ddt_scale));let eq68_e2122_d_n5: f64 = (eq68_e2117_d_n5 + (eq68_e2120_d_n5 * ddt_scale));let eq68_e2122_d_n6: f64 = (eq68_e2117_d_n6 + (eq68_e2120_d_n6 * ddt_scale));let eq68_e2122_d_n7: f64 = (eq68_e2117_d_n7 + (eq68_e2120_d_n7 * ddt_scale));let eq68_e2122_d_n8: f64 = (eq68_e2117_d_n8 + (eq68_e2120_d_n8 * ddt_scale));let eq68_e2122_d_n9: f64 = (eq68_e2117_d_n9 + (eq68_e2120_d_n9 * ddt_scale));let eq68_e2122_d_n10: f64 = (eq68_e2117_d_n10 + (eq68_e2120_d_n10 * ddt_scale));let eq68_e2122_d_n11: f64 = (eq68_e2117_d_n11 + (eq68_e2120_d_n11 * ddt_scale));let eq68_e2122_d_n12: f64 = (eq68_e2117_d_n12 + (eq68_e2120_d_n12 * ddt_scale));let eq68_e2122_d_n13: f64 = (eq68_e2117_d_n13 + (eq68_e2120_d_n13 * ddt_scale));let eq68_e2122_d_b0: f64 = (eq68_e2117_d_b0 + (eq68_e2120_d_b0 * ddt_scale));let eq68_e2122_d_b1: f64 = (eq68_e2117_d_b1 + (eq68_e2120_d_b1 * ddt_scale));let eq68_e2122_d_b2: f64 = (eq68_e2117_d_b2 + (eq68_e2120_d_b2 * ddt_scale));let eq68_e2122_d_b3: f64 = (eq68_e2117_d_b3 + (eq68_e2120_d_b3 * ddt_scale));let eq68_e2122_d_b4: f64 = (eq68_e2117_d_b4 + (eq68_e2120_d_b4 * ddt_scale));let eq68_e2122_d_b5: f64 = (eq68_e2117_d_b5 + (eq68_e2120_d_b5 * ddt_scale));let eq68_e2122_d_b6: f64 = (eq68_e2117_d_b6 + (eq68_e2120_d_b6 * ddt_scale));let eq68_e2122_d_b7: f64 = (eq68_e2117_d_b7 + (eq68_e2120_d_b7 * ddt_scale));let eq68_e2122_d_b8: f64 = (eq68_e2117_d_b8 + (eq68_e2120_d_b8 * ddt_scale));let eq68_e2122_d_b9: f64 = (eq68_e2117_d_b9 + (eq68_e2120_d_b9 * ddt_scale));let eq68_e2122_d_b10: f64 = (eq68_e2117_d_b10 + (eq68_e2120_d_b10 * ddt_scale));let eq68_e2122_d_b11: f64 = (eq68_e2117_d_b11 + (eq68_e2120_d_b11 * ddt_scale));let eq68_e2124: f64 = (eq68_e2122 - s.v[1017]);let eq68_e2124_d_n0: f64 = (eq68_e2122_d_n0 - s.dn[1017][0]);let eq68_e2124_d_n1: f64 = (eq68_e2122_d_n1 - s.dn[1017][1]);let eq68_e2124_d_n2: f64 = (eq68_e2122_d_n2 - s.dn[1017][2]);let eq68_e2124_d_n3: f64 = (eq68_e2122_d_n3 - s.dn[1017][3]);let eq68_e2124_d_n4: f64 = (eq68_e2122_d_n4 - s.dn[1017][4]);let eq68_e2124_d_n5: f64 = (eq68_e2122_d_n5 - s.dn[1017][5]);let eq68_e2124_d_n6: f64 = (eq68_e2122_d_n6 - s.dn[1017][6]);let eq68_e2124_d_n7: f64 = (eq68_e2122_d_n7 - s.dn[1017][7]);let eq68_e2124_d_n8: f64 = (eq68_e2122_d_n8 - s.dn[1017][8]);let eq68_e2124_d_n9: f64 = (eq68_e2122_d_n9 - s.dn[1017][9]);let eq68_e2124_d_n10: f64 = (eq68_e2122_d_n10 - s.dn[1017][10]);let eq68_e2124_d_n11: f64 = (eq68_e2122_d_n11 - s.dn[1017][11]);let eq68_e2124_d_n12: f64 = (eq68_e2122_d_n12 - s.dn[1017][12]);let eq68_e2124_d_n13: f64 = (eq68_e2122_d_n13 - s.dn[1017][13]);let eq68_e2124_d_b0: f64 = (eq68_e2122_d_b0 - s.db[1017][0]);let eq68_e2124_d_b1: f64 = (eq68_e2122_d_b1 - s.db[1017][1]);let eq68_e2124_d_b2: f64 = (eq68_e2122_d_b2 - s.db[1017][2]);let eq68_e2124_d_b3: f64 = (eq68_e2122_d_b3 - s.db[1017][3]);let eq68_e2124_d_b4: f64 = (eq68_e2122_d_b4 - s.db[1017][4]);
        let eq68_e2124_d_b5: f64 = (eq68_e2122_d_b5 - s.db[1017][5]);let eq68_e2124_d_b6: f64 = (eq68_e2122_d_b6 - s.db[1017][6]);let eq68_e2124_d_b7: f64 = (eq68_e2122_d_b7 - s.db[1017][7]);let eq68_e2124_d_b8: f64 = (eq68_e2122_d_b8 - s.db[1017][8]);let eq68_e2124_d_b9: f64 = (eq68_e2122_d_b9 - s.db[1017][9]);let eq68_e2124_d_b10: f64 = (eq68_e2122_d_b10 - s.db[1017][10]);let eq68_e2124_d_b11: f64 = (eq68_e2122_d_b11 - s.db[1017][11]);
        (eq68_e2124, eq68_e2124_d_n0, eq68_e2124_d_n1, eq68_e2124_d_n2, eq68_e2124_d_n3, eq68_e2124_d_n4, eq68_e2124_d_n5, eq68_e2124_d_n6, eq68_e2124_d_n7, eq68_e2124_d_n8, eq68_e2124_d_n9, eq68_e2124_d_n10, eq68_e2124_d_n11, eq68_e2124_d_n12, eq68_e2124_d_n13, eq68_e2124_d_b0, eq68_e2124_d_b1, eq68_e2124_d_b2, eq68_e2124_d_b3, eq68_e2124_d_b4, eq68_e2124_d_b5, eq68_e2124_d_b6, eq68_e2124_d_b7, eq68_e2124_d_b8, eq68_e2124_d_b9, eq68_e2124_d_b10, eq68_e2124_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2126;let eq68_node_derivatives: [f64; 14] = [eq68_e2126_d_n0, eq68_e2126_d_n1, eq68_e2126_d_n2, eq68_e2126_d_n3, eq68_e2126_d_n4, eq68_e2126_d_n5, eq68_e2126_d_n6, eq68_e2126_d_n7, eq68_e2126_d_n8, eq68_e2126_d_n9, eq68_e2126_d_n10, eq68_e2126_d_n11, eq68_e2126_d_n12, eq68_e2126_d_n13];let eq68_branch_derivatives: [f64; 12] = [eq68_e2126_d_b0, eq68_e2126_d_b1, eq68_e2126_d_b2, eq68_e2126_d_b3, eq68_e2126_d_b4, eq68_e2126_d_b5, eq68_e2126_d_b6, eq68_e2126_d_b7, eq68_e2126_d_b8, eq68_e2126_d_b9, eq68_e2126_d_b10, eq68_e2126_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivatives,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_20(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let (eq69_e2142, eq69_e2142_d_n0, eq69_e2142_d_n1, eq69_e2142_d_n2, eq69_e2142_d_n3, eq69_e2142_d_n4, eq69_e2142_d_n5, eq69_e2142_d_n6, eq69_e2142_d_n7, eq69_e2142_d_n8, eq69_e2142_d_n9, eq69_e2142_d_n10, eq69_e2142_d_n11, eq69_e2142_d_n12, eq69_e2142_d_n13, eq69_e2142_d_b0, eq69_e2142_d_b1, eq69_e2142_d_b2, eq69_e2142_d_b3, eq69_e2142_d_b4, eq69_e2142_d_b5, eq69_e2142_d_b6, eq69_e2142_d_b7, eq69_e2142_d_b8, eq69_e2142_d_b9, eq69_e2142_d_b10, eq69_e2142_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2133: f64 = (s.v[634] * s.v[1015]);let eq69_e2133_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq69_e2133_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq69_e2133_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq69_e2133_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq69_e2133_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq69_e2133_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq69_e2133_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq69_e2133_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq69_e2133_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq69_e2133_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq69_e2133_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq69_e2133_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq69_e2133_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq69_e2133_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq69_e2133_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq69_e2133_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq69_e2133_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq69_e2133_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq69_e2133_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq69_e2133_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq69_e2133_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq69_e2133_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq69_e2133_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq69_e2133_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq69_e2133_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq69_e2133_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq69_e2136: f64 = (s.v[634] * s.v[1016]);let eq69_e2136_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq69_e2136_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq69_e2136_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq69_e2136_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq69_e2136_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq69_e2136_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq69_e2136_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq69_e2136_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq69_e2136_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq69_e2136_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq69_e2136_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq69_e2136_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq69_e2136_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq69_e2136_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq69_e2136_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq69_e2136_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq69_e2136_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq69_e2136_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq69_e2136_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq69_e2136_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2136_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq69_e2136_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq69_e2136_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq69_e2136_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq69_e2136_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq69_e2136_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq69_e2137: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq69_e2136);let eq69_e2138: f64 = (eq69_e2133 + eq69_e2137);let eq69_e2138_d_n0: f64 = (eq69_e2133_d_n0 + (eq69_e2136_d_n0 * ddt_scale));let eq69_e2138_d_n1: f64 = (eq69_e2133_d_n1 + (eq69_e2136_d_n1 * ddt_scale));let eq69_e2138_d_n2: f64 = (eq69_e2133_d_n2 + (eq69_e2136_d_n2 * ddt_scale));let eq69_e2138_d_n3: f64 = (eq69_e2133_d_n3 + (eq69_e2136_d_n3 * ddt_scale));let eq69_e2138_d_n4: f64 = (eq69_e2133_d_n4 + (eq69_e2136_d_n4 * ddt_scale));let eq69_e2138_d_n5: f64 = (eq69_e2133_d_n5 + (eq69_e2136_d_n5 * ddt_scale));let eq69_e2138_d_n6: f64 = (eq69_e2133_d_n6 + (eq69_e2136_d_n6 * ddt_scale));let eq69_e2138_d_n7: f64 = (eq69_e2133_d_n7 + (eq69_e2136_d_n7 * ddt_scale));let eq69_e2138_d_n8: f64 = (eq69_e2133_d_n8 + (eq69_e2136_d_n8 * ddt_scale));let eq69_e2138_d_n9: f64 = (eq69_e2133_d_n9 + (eq69_e2136_d_n9 * ddt_scale));let eq69_e2138_d_n10: f64 = (eq69_e2133_d_n10 + (eq69_e2136_d_n10 * ddt_scale));let eq69_e2138_d_n11: f64 = (eq69_e2133_d_n11 + (eq69_e2136_d_n11 * ddt_scale));let eq69_e2138_d_n12: f64 = (eq69_e2133_d_n12 + (eq69_e2136_d_n12 * ddt_scale));let eq69_e2138_d_n13: f64 = (eq69_e2133_d_n13 + (eq69_e2136_d_n13 * ddt_scale));let eq69_e2138_d_b0: f64 = (eq69_e2133_d_b0 + (eq69_e2136_d_b0 * ddt_scale));let eq69_e2138_d_b1: f64 = (eq69_e2133_d_b1 + (eq69_e2136_d_b1 * ddt_scale));let eq69_e2138_d_b2: f64 = (eq69_e2133_d_b2 + (eq69_e2136_d_b2 * ddt_scale));let eq69_e2138_d_b3: f64 = (eq69_e2133_d_b3 + (eq69_e2136_d_b3 * ddt_scale));let eq69_e2138_d_b4: f64 = (eq69_e2133_d_b4 + (eq69_e2136_d_b4 * ddt_scale));let eq69_e2138_d_b5: f64 = (eq69_e2133_d_b5 + (eq69_e2136_d_b5 * ddt_scale));let eq69_e2138_d_b6: f64 = (eq69_e2133_d_b6 + (eq69_e2136_d_b6 * ddt_scale));let eq69_e2138_d_b7: f64 = (eq69_e2133_d_b7 + (eq69_e2136_d_b7 * ddt_scale));let eq69_e2138_d_b8: f64 = (eq69_e2133_d_b8 + (eq69_e2136_d_b8 * ddt_scale));let eq69_e2138_d_b9: f64 = (eq69_e2133_d_b9 + (eq69_e2136_d_b9 * ddt_scale));let eq69_e2138_d_b10: f64 = (eq69_e2133_d_b10 + (eq69_e2136_d_b10 * ddt_scale));let eq69_e2138_d_b11: f64 = (eq69_e2133_d_b11 + (eq69_e2136_d_b11 * ddt_scale));let eq69_e2140: f64 = (eq69_e2138 - s.v[1017]);let eq69_e2140_d_n0: f64 = (eq69_e2138_d_n0 - s.dn[1017][0]);let eq69_e2140_d_n1: f64 = (eq69_e2138_d_n1 - s.dn[1017][1]);let eq69_e2140_d_n2: f64 = (eq69_e2138_d_n2 - s.dn[1017][2]);let eq69_e2140_d_n3: f64 = (eq69_e2138_d_n3 - s.dn[1017][3]);let eq69_e2140_d_n4: f64 = (eq69_e2138_d_n4 - s.dn[1017][4]);let eq69_e2140_d_n5: f64 = (eq69_e2138_d_n5 - s.dn[1017][5]);let eq69_e2140_d_n6: f64 = (eq69_e2138_d_n6 - s.dn[1017][6]);let eq69_e2140_d_n7: f64 = (eq69_e2138_d_n7 - s.dn[1017][7]);let eq69_e2140_d_n8: f64 = (eq69_e2138_d_n8 - s.dn[1017][8]);let eq69_e2140_d_n9: f64 = (eq69_e2138_d_n9 - s.dn[1017][9]);let eq69_e2140_d_n10: f64 = (eq69_e2138_d_n10 - s.dn[1017][10]);let eq69_e2140_d_n11: f64 = (eq69_e2138_d_n11 - s.dn[1017][11]);let eq69_e2140_d_n12: f64 = (eq69_e2138_d_n12 - s.dn[1017][12]);let eq69_e2140_d_n13: f64 = (eq69_e2138_d_n13 - s.dn[1017][13]);let eq69_e2140_d_b0: f64 = (eq69_e2138_d_b0 - s.db[1017][0]);let eq69_e2140_d_b1: f64 = (eq69_e2138_d_b1 - s.db[1017][1]);let eq69_e2140_d_b2: f64 = (eq69_e2138_d_b2 - s.db[1017][2]);let eq69_e2140_d_b3: f64 = (eq69_e2138_d_b3 - s.db[1017][3]);let eq69_e2140_d_b4: f64 = (eq69_e2138_d_b4 - s.db[1017][4]);
        let eq69_e2140_d_b5: f64 = (eq69_e2138_d_b5 - s.db[1017][5]);let eq69_e2140_d_b6: f64 = (eq69_e2138_d_b6 - s.db[1017][6]);let eq69_e2140_d_b7: f64 = (eq69_e2138_d_b7 - s.db[1017][7]);let eq69_e2140_d_b8: f64 = (eq69_e2138_d_b8 - s.db[1017][8]);let eq69_e2140_d_b9: f64 = (eq69_e2138_d_b9 - s.db[1017][9]);let eq69_e2140_d_b10: f64 = (eq69_e2138_d_b10 - s.db[1017][10]);let eq69_e2140_d_b11: f64 = (eq69_e2138_d_b11 - s.db[1017][11]);
        (eq69_e2140, eq69_e2140_d_n0, eq69_e2140_d_n1, eq69_e2140_d_n2, eq69_e2140_d_n3, eq69_e2140_d_n4, eq69_e2140_d_n5, eq69_e2140_d_n6, eq69_e2140_d_n7, eq69_e2140_d_n8, eq69_e2140_d_n9, eq69_e2140_d_n10, eq69_e2140_d_n11, eq69_e2140_d_n12, eq69_e2140_d_n13, eq69_e2140_d_b0, eq69_e2140_d_b1, eq69_e2140_d_b2, eq69_e2140_d_b3, eq69_e2140_d_b4, eq69_e2140_d_b5, eq69_e2140_d_b6, eq69_e2140_d_b7, eq69_e2140_d_b8, eq69_e2140_d_b9, eq69_e2140_d_b10, eq69_e2140_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2142;let eq69_node_derivatives: [f64; 14] = [eq69_e2142_d_n0, eq69_e2142_d_n1, eq69_e2142_d_n2, eq69_e2142_d_n3, eq69_e2142_d_n4, eq69_e2142_d_n5, eq69_e2142_d_n6, eq69_e2142_d_n7, eq69_e2142_d_n8, eq69_e2142_d_n9, eq69_e2142_d_n10, eq69_e2142_d_n11, eq69_e2142_d_n12, eq69_e2142_d_n13];let eq69_branch_derivatives: [f64; 12] = [eq69_e2142_d_b0, eq69_e2142_d_b1, eq69_e2142_d_b2, eq69_e2142_d_b3, eq69_e2142_d_b4, eq69_e2142_d_b5, eq69_e2142_d_b6, eq69_e2142_d_b7, eq69_e2142_d_b8, eq69_e2142_d_b9, eq69_e2142_d_b10, eq69_e2142_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_21(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq70_e2151,) = {
    if (((!s.b[2021]) && s.b[2026]) && s.b[2027]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e2151;
        stamper.stamp_potential_const_local(
            6,
            eq70_value,
        );
        let (eq71_e2161,) = {
    if (((!s.b[2021]) && s.b[2026]) && (!s.b[2027])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e2161;
        stamper.stamp_potential_const_local(
            7,
            eq71_value,
        );
        let (eq72_e2169,) = {
    if ((!s.b[2021]) && (!s.b[2026])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e2169;
        stamper.stamp_potential_const_local(
            8,
            eq72_value,
        );
        let (eq73_e2173,) = {
    if s.b[2028] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq73_value: f64 = eq73_e2173;
        stamper.stamp_potential_const_local(
            9,
            eq73_value,
        );
        let (eq74_e2177,) = {
    if s.b[2028] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq74_value: f64 = eq74_e2177;
        stamper.stamp_potential_const_local(
            10,
            eq74_value,
        );
        let (eq75_e2184,) = {
    if ((!s.b[2028]) && s.b[2029]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e2184;
        stamper.stamp_potential_const_local(
            11,
            eq75_value,
        );
        let (eq76_e2192, eq76_e2192_d_n0, eq76_e2192_d_n1, eq76_e2192_d_n2, eq76_e2192_d_n3, eq76_e2192_d_n4, eq76_e2192_d_n5, eq76_e2192_d_n6, eq76_e2192_d_n7, eq76_e2192_d_n8, eq76_e2192_d_n9, eq76_e2192_d_n10, eq76_e2192_d_n11, eq76_e2192_d_n12, eq76_e2192_d_n13, eq76_e2192_d_b0, eq76_e2192_d_b1, eq76_e2192_d_b2, eq76_e2192_d_b3, eq76_e2192_d_b4, eq76_e2192_d_b5, eq76_e2192_d_b6, eq76_e2192_d_b7, eq76_e2192_d_b8, eq76_e2192_d_b9, eq76_e2192_d_b10, eq76_e2192_d_b11,) = {
    if (s.b[2037] && s.b[2038]) {
        let eq76_e2190: f64 = ((nv4 - nv10) * s.v[1021]);let eq76_e2190_d_n0: f64 = ((nv4 - nv10) * s.dn[1021][0]);let eq76_e2190_d_n1: f64 = ((nv4 - nv10) * s.dn[1021][1]);let eq76_e2190_d_n2: f64 = ((nv4 - nv10) * s.dn[1021][2]);let eq76_e2190_d_n3: f64 = ((nv4 - nv10) * s.dn[1021][3]);let eq76_e2190_d_n4: f64 = (s.v[1021] + ((nv4 - nv10) * s.dn[1021][4]));let eq76_e2190_d_n5: f64 = ((nv4 - nv10) * s.dn[1021][5]);let eq76_e2190_d_n6: f64 = ((nv4 - nv10) * s.dn[1021][6]);let eq76_e2190_d_n7: f64 = ((nv4 - nv10) * s.dn[1021][7]);let eq76_e2190_d_n8: f64 = ((nv4 - nv10) * s.dn[1021][8]);let eq76_e2190_d_n9: f64 = ((nv4 - nv10) * s.dn[1021][9]);let eq76_e2190_d_n10: f64 = ((-s.v[1021]) + ((nv4 - nv10) * s.dn[1021][10]));let eq76_e2190_d_n11: f64 = ((nv4 - nv10) * s.dn[1021][11]);let eq76_e2190_d_n12: f64 = ((nv4 - nv10) * s.dn[1021][12]);let eq76_e2190_d_n13: f64 = ((nv4 - nv10) * s.dn[1021][13]);let eq76_e2190_d_b0: f64 = ((nv4 - nv10) * s.db[1021][0]);let eq76_e2190_d_b1: f64 = ((nv4 - nv10) * s.db[1021][1]);let eq76_e2190_d_b2: f64 = ((nv4 - nv10) * s.db[1021][2]);let eq76_e2190_d_b3: f64 = ((nv4 - nv10) * s.db[1021][3]);let eq76_e2190_d_b4: f64 = ((nv4 - nv10) * s.db[1021][4]);let eq76_e2190_d_b5: f64 = ((nv4 - nv10) * s.db[1021][5]);let eq76_e2190_d_b6: f64 = ((nv4 - nv10) * s.db[1021][6]);let eq76_e2190_d_b7: f64 = ((nv4 - nv10) * s.db[1021][7]);let eq76_e2190_d_b8: f64 = ((nv4 - nv10) * s.db[1021][8]);let eq76_e2190_d_b9: f64 = ((nv4 - nv10) * s.db[1021][9]);let eq76_e2190_d_b10: f64 = ((nv4 - nv10) * s.db[1021][10]);let eq76_e2190_d_b11: f64 = ((nv4 - nv10) * s.db[1021][11]);
        (eq76_e2190, eq76_e2190_d_n0, eq76_e2190_d_n1, eq76_e2190_d_n2, eq76_e2190_d_n3, eq76_e2190_d_n4, eq76_e2190_d_n5, eq76_e2190_d_n6, eq76_e2190_d_n7, eq76_e2190_d_n8, eq76_e2190_d_n9, eq76_e2190_d_n10, eq76_e2190_d_n11, eq76_e2190_d_n12, eq76_e2190_d_n13, eq76_e2190_d_b0, eq76_e2190_d_b1, eq76_e2190_d_b2, eq76_e2190_d_b3, eq76_e2190_d_b4, eq76_e2190_d_b5, eq76_e2190_d_b6, eq76_e2190_d_b7, eq76_e2190_d_b8, eq76_e2190_d_b9, eq76_e2190_d_b10, eq76_e2190_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2192;let eq76_node_derivatives: [f64; 14] = [eq76_e2192_d_n0, eq76_e2192_d_n1, eq76_e2192_d_n2, eq76_e2192_d_n3, eq76_e2192_d_n4, eq76_e2192_d_n5, eq76_e2192_d_n6, eq76_e2192_d_n7, eq76_e2192_d_n8, eq76_e2192_d_n9, eq76_e2192_d_n10, eq76_e2192_d_n11, eq76_e2192_d_n12, eq76_e2192_d_n13];let eq76_branch_derivatives: [f64; 12] = [eq76_e2192_d_b0, eq76_e2192_d_b1, eq76_e2192_d_b2, eq76_e2192_d_b3, eq76_e2192_d_b4, eq76_e2192_d_b5, eq76_e2192_d_b6, eq76_e2192_d_b7, eq76_e2192_d_b8, eq76_e2192_d_b9, eq76_e2192_d_b10, eq76_e2192_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(10),
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e2200, eq77_e2200_d_n0, eq77_e2200_d_n1, eq77_e2200_d_n2, eq77_e2200_d_n3, eq77_e2200_d_n4, eq77_e2200_d_n5, eq77_e2200_d_n6, eq77_e2200_d_n7, eq77_e2200_d_n8, eq77_e2200_d_n9, eq77_e2200_d_n10, eq77_e2200_d_n11, eq77_e2200_d_n12, eq77_e2200_d_n13, eq77_e2200_d_b0, eq77_e2200_d_b1, eq77_e2200_d_b2, eq77_e2200_d_b3, eq77_e2200_d_b4, eq77_e2200_d_b5, eq77_e2200_d_b6, eq77_e2200_d_b7, eq77_e2200_d_b8, eq77_e2200_d_b9, eq77_e2200_d_b10, eq77_e2200_d_b11,) = {
    if (s.b[2037] && s.b[2038]) {
        let eq77_e2198: f64 = ((nv4 - nv11) * s.v[1022]);let eq77_e2198_d_n0: f64 = ((nv4 - nv11) * s.dn[1022][0]);let eq77_e2198_d_n1: f64 = ((nv4 - nv11) * s.dn[1022][1]);let eq77_e2198_d_n2: f64 = ((nv4 - nv11) * s.dn[1022][2]);let eq77_e2198_d_n3: f64 = ((nv4 - nv11) * s.dn[1022][3]);let eq77_e2198_d_n4: f64 = (s.v[1022] + ((nv4 - nv11) * s.dn[1022][4]));let eq77_e2198_d_n5: f64 = ((nv4 - nv11) * s.dn[1022][5]);let eq77_e2198_d_n6: f64 = ((nv4 - nv11) * s.dn[1022][6]);let eq77_e2198_d_n7: f64 = ((nv4 - nv11) * s.dn[1022][7]);let eq77_e2198_d_n8: f64 = ((nv4 - nv11) * s.dn[1022][8]);let eq77_e2198_d_n9: f64 = ((nv4 - nv11) * s.dn[1022][9]);let eq77_e2198_d_n10: f64 = ((nv4 - nv11) * s.dn[1022][10]);let eq77_e2198_d_n11: f64 = ((-s.v[1022]) + ((nv4 - nv11) * s.dn[1022][11]));let eq77_e2198_d_n12: f64 = ((nv4 - nv11) * s.dn[1022][12]);let eq77_e2198_d_n13: f64 = ((nv4 - nv11) * s.dn[1022][13]);let eq77_e2198_d_b0: f64 = ((nv4 - nv11) * s.db[1022][0]);let eq77_e2198_d_b1: f64 = ((nv4 - nv11) * s.db[1022][1]);let eq77_e2198_d_b2: f64 = ((nv4 - nv11) * s.db[1022][2]);let eq77_e2198_d_b3: f64 = ((nv4 - nv11) * s.db[1022][3]);let eq77_e2198_d_b4: f64 = ((nv4 - nv11) * s.db[1022][4]);let eq77_e2198_d_b5: f64 = ((nv4 - nv11) * s.db[1022][5]);let eq77_e2198_d_b6: f64 = ((nv4 - nv11) * s.db[1022][6]);let eq77_e2198_d_b7: f64 = ((nv4 - nv11) * s.db[1022][7]);let eq77_e2198_d_b8: f64 = ((nv4 - nv11) * s.db[1022][8]);let eq77_e2198_d_b9: f64 = ((nv4 - nv11) * s.db[1022][9]);let eq77_e2198_d_b10: f64 = ((nv4 - nv11) * s.db[1022][10]);let eq77_e2198_d_b11: f64 = ((nv4 - nv11) * s.db[1022][11]);
        (eq77_e2198, eq77_e2198_d_n0, eq77_e2198_d_n1, eq77_e2198_d_n2, eq77_e2198_d_n3, eq77_e2198_d_n4, eq77_e2198_d_n5, eq77_e2198_d_n6, eq77_e2198_d_n7, eq77_e2198_d_n8, eq77_e2198_d_n9, eq77_e2198_d_n10, eq77_e2198_d_n11, eq77_e2198_d_n12, eq77_e2198_d_n13, eq77_e2198_d_b0, eq77_e2198_d_b1, eq77_e2198_d_b2, eq77_e2198_d_b3, eq77_e2198_d_b4, eq77_e2198_d_b5, eq77_e2198_d_b6, eq77_e2198_d_b7, eq77_e2198_d_b8, eq77_e2198_d_b9, eq77_e2198_d_b10, eq77_e2198_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e2200;let eq77_node_derivatives: [f64; 14] = [eq77_e2200_d_n0, eq77_e2200_d_n1, eq77_e2200_d_n2, eq77_e2200_d_n3, eq77_e2200_d_n4, eq77_e2200_d_n5, eq77_e2200_d_n6, eq77_e2200_d_n7, eq77_e2200_d_n8, eq77_e2200_d_n9, eq77_e2200_d_n10, eq77_e2200_d_n11, eq77_e2200_d_n12, eq77_e2200_d_n13];let eq77_branch_derivatives: [f64; 12] = [eq77_e2200_d_b0, eq77_e2200_d_b1, eq77_e2200_d_b2, eq77_e2200_d_b3, eq77_e2200_d_b4, eq77_e2200_d_b5, eq77_e2200_d_b6, eq77_e2200_d_b7, eq77_e2200_d_b8, eq77_e2200_d_b9, eq77_e2200_d_b10, eq77_e2200_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(11),
            multiplicity * (eq77_value),
            &eq77_node_derivatives,
            &eq77_branch_derivatives,
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
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv10 = ctx.node_voltage(nodes[10]);let eq78_e2203: f64 = (s.v[379] * s.v[496]);let eq78_e2203_d_n0: f64 = ((s.dn[379][0] * s.v[496]) + (s.v[379] * s.dn[496][0]));let eq78_e2203_d_n1: f64 = ((s.dn[379][1] * s.v[496]) + (s.v[379] * s.dn[496][1]));let eq78_e2203_d_n2: f64 = ((s.dn[379][2] * s.v[496]) + (s.v[379] * s.dn[496][2]));let eq78_e2203_d_n3: f64 = ((s.dn[379][3] * s.v[496]) + (s.v[379] * s.dn[496][3]));let eq78_e2203_d_n4: f64 = ((s.dn[379][4] * s.v[496]) + (s.v[379] * s.dn[496][4]));let eq78_e2203_d_n5: f64 = ((s.dn[379][5] * s.v[496]) + (s.v[379] * s.dn[496][5]));let eq78_e2203_d_n6: f64 = ((s.dn[379][6] * s.v[496]) + (s.v[379] * s.dn[496][6]));let eq78_e2203_d_n7: f64 = ((s.dn[379][7] * s.v[496]) + (s.v[379] * s.dn[496][7]));let eq78_e2203_d_n8: f64 = ((s.dn[379][8] * s.v[496]) + (s.v[379] * s.dn[496][8]));let eq78_e2203_d_n9: f64 = ((s.dn[379][9] * s.v[496]) + (s.v[379] * s.dn[496][9]));let eq78_e2203_d_n10: f64 = ((s.dn[379][10] * s.v[496]) + (s.v[379] * s.dn[496][10]));let eq78_e2203_d_n11: f64 = ((s.dn[379][11] * s.v[496]) + (s.v[379] * s.dn[496][11]));let eq78_e2203_d_n12: f64 = ((s.dn[379][12] * s.v[496]) + (s.v[379] * s.dn[496][12]));let eq78_e2203_d_n13: f64 = ((s.dn[379][13] * s.v[496]) + (s.v[379] * s.dn[496][13]));let eq78_e2203_d_b0: f64 = ((s.db[379][0] * s.v[496]) + (s.v[379] * s.db[496][0]));let eq78_e2203_d_b1: f64 = ((s.db[379][1] * s.v[496]) + (s.v[379] * s.db[496][1]));let eq78_e2203_d_b2: f64 = ((s.db[379][2] * s.v[496]) + (s.v[379] * s.db[496][2]));let eq78_e2203_d_b3: f64 = ((s.db[379][3] * s.v[496]) + (s.v[379] * s.db[496][3]));let eq78_e2203_d_b4: f64 = ((s.db[379][4] * s.v[496]) + (s.v[379] * s.db[496][4]));let eq78_e2203_d_b5: f64 = ((s.db[379][5] * s.v[496]) + (s.v[379] * s.db[496][5]));let eq78_e2203_d_b6: f64 = ((s.db[379][6] * s.v[496]) + (s.v[379] * s.db[496][6]));let eq78_e2203_d_b7: f64 = ((s.db[379][7] * s.v[496]) + (s.v[379] * s.db[496][7]));let eq78_e2203_d_b8: f64 = ((s.db[379][8] * s.v[496]) + (s.v[379] * s.db[496][8]));let eq78_e2203_d_b9: f64 = ((s.db[379][9] * s.v[496]) + (s.v[379] * s.db[496][9]));let eq78_e2203_d_b10: f64 = ((s.db[379][10] * s.v[496]) + (s.v[379] * s.db[496][10]));let eq78_e2203_d_b11: f64 = ((s.db[379][11] * s.v[496]) + (s.v[379] * s.db[496][11]));let eq78_e2206: f64 = ((nv10 - nv7) * s.v[1018]);let eq78_e2207: f64 = (eq78_e2203 + eq78_e2206);let eq78_e2207_d_n7: f64 = (eq78_e2203_d_n7 + (-s.v[1018]));let eq78_e2207_d_n10: f64 = (eq78_e2203_d_n10 + s.v[1018]);let eq78_value: f64 = eq78_e2207;let eq78_node_derivatives: [f64; 14] = [eq78_e2203_d_n0, eq78_e2203_d_n1, eq78_e2203_d_n2, eq78_e2203_d_n3, eq78_e2203_d_n4, eq78_e2203_d_n5, eq78_e2203_d_n6, eq78_e2207_d_n7, eq78_e2203_d_n8, eq78_e2203_d_n9, eq78_e2207_d_n10, eq78_e2203_d_n11, eq78_e2203_d_n12, eq78_e2203_d_n13];let eq78_branch_derivatives: [f64; 12] = [eq78_e2203_d_b0, eq78_e2203_d_b1, eq78_e2203_d_b2, eq78_e2203_d_b3, eq78_e2203_d_b4, eq78_e2203_d_b5, eq78_e2203_d_b6, eq78_e2203_d_b7, eq78_e2203_d_b8, eq78_e2203_d_b9, eq78_e2203_d_b10, eq78_e2203_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq78_value),
            &eq78_node_derivatives,
            &eq78_branch_derivatives,
            multiplicity,
        );let eq79_e2210: f64 = (s.v[379] * s.v[497]);let eq79_e2210_d_n0: f64 = ((s.dn[379][0] * s.v[497]) + (s.v[379] * s.dn[497][0]));let eq79_e2210_d_n1: f64 = ((s.dn[379][1] * s.v[497]) + (s.v[379] * s.dn[497][1]));let eq79_e2210_d_n2: f64 = ((s.dn[379][2] * s.v[497]) + (s.v[379] * s.dn[497][2]));let eq79_e2210_d_n3: f64 = ((s.dn[379][3] * s.v[497]) + (s.v[379] * s.dn[497][3]));let eq79_e2210_d_n4: f64 = ((s.dn[379][4] * s.v[497]) + (s.v[379] * s.dn[497][4]));let eq79_e2210_d_n5: f64 = ((s.dn[379][5] * s.v[497]) + (s.v[379] * s.dn[497][5]));let eq79_e2210_d_n6: f64 = ((s.dn[379][6] * s.v[497]) + (s.v[379] * s.dn[497][6]));let eq79_e2210_d_n7: f64 = ((s.dn[379][7] * s.v[497]) + (s.v[379] * s.dn[497][7]));let eq79_e2210_d_n8: f64 = ((s.dn[379][8] * s.v[497]) + (s.v[379] * s.dn[497][8]));let eq79_e2210_d_n9: f64 = ((s.dn[379][9] * s.v[497]) + (s.v[379] * s.dn[497][9]));let eq79_e2210_d_n10: f64 = ((s.dn[379][10] * s.v[497]) + (s.v[379] * s.dn[497][10]));let eq79_e2210_d_n11: f64 = ((s.dn[379][11] * s.v[497]) + (s.v[379] * s.dn[497][11]));let eq79_e2210_d_n12: f64 = ((s.dn[379][12] * s.v[497]) + (s.v[379] * s.dn[497][12]));let eq79_e2210_d_n13: f64 = ((s.dn[379][13] * s.v[497]) + (s.v[379] * s.dn[497][13]));let eq79_e2210_d_b0: f64 = ((s.db[379][0] * s.v[497]) + (s.v[379] * s.db[497][0]));let eq79_e2210_d_b1: f64 = ((s.db[379][1] * s.v[497]) + (s.v[379] * s.db[497][1]));let eq79_e2210_d_b2: f64 = ((s.db[379][2] * s.v[497]) + (s.v[379] * s.db[497][2]));let eq79_e2210_d_b3: f64 = ((s.db[379][3] * s.v[497]) + (s.v[379] * s.db[497][3]));let eq79_e2210_d_b4: f64 = ((s.db[379][4] * s.v[497]) + (s.v[379] * s.db[497][4]));let eq79_e2210_d_b5: f64 = ((s.db[379][5] * s.v[497]) + (s.v[379] * s.db[497][5]));let eq79_e2210_d_b6: f64 = ((s.db[379][6] * s.v[497]) + (s.v[379] * s.db[497][6]));let eq79_e2210_d_b7: f64 = ((s.db[379][7] * s.v[497]) + (s.v[379] * s.db[497][7]));let eq79_e2210_d_b8: f64 = ((s.db[379][8] * s.v[497]) + (s.v[379] * s.db[497][8]));let eq79_e2210_d_b9: f64 = ((s.db[379][9] * s.v[497]) + (s.v[379] * s.db[497][9]));let eq79_e2210_d_b10: f64 = ((s.db[379][10] * s.v[497]) + (s.v[379] * s.db[497][10]));let eq79_e2210_d_b11: f64 = ((s.db[379][11] * s.v[497]) + (s.v[379] * s.db[497][11]));let eq79_e2213: f64 = ((nv10 - nv6) * s.v[1018]);let eq79_e2214: f64 = (eq79_e2210 + eq79_e2213);let eq79_e2214_d_n6: f64 = (eq79_e2210_d_n6 + (-s.v[1018]));let eq79_e2214_d_n10: f64 = (eq79_e2210_d_n10 + s.v[1018]);let eq79_value: f64 = eq79_e2214;let eq79_node_derivatives: [f64; 14] = [eq79_e2210_d_n0, eq79_e2210_d_n1, eq79_e2210_d_n2, eq79_e2210_d_n3, eq79_e2210_d_n4, eq79_e2210_d_n5, eq79_e2214_d_n6, eq79_e2210_d_n7, eq79_e2210_d_n8, eq79_e2210_d_n9, eq79_e2214_d_n10, eq79_e2210_d_n11, eq79_e2210_d_n12, eq79_e2210_d_n13];let eq79_branch_derivatives: [f64; 12] = [eq79_e2210_d_b0, eq79_e2210_d_b1, eq79_e2210_d_b2, eq79_e2210_d_b3, eq79_e2210_d_b4, eq79_e2210_d_b5, eq79_e2210_d_b6, eq79_e2210_d_b7, eq79_e2210_d_b8, eq79_e2210_d_b9, eq79_e2210_d_b10, eq79_e2210_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
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
        let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv11 = ctx.node_voltage(nodes[11]);let eq80_e2217: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, s.v[520]);let eq80_e2218: f64 = (s.v[379] * eq80_e2217);let eq80_e2218_d_n0: f64 = ((s.dn[379][0] * eq80_e2217) + (s.v[379] * (s.dn[520][0] * ddt_scale)));let eq80_e2218_d_n1: f64 = ((s.dn[379][1] * eq80_e2217) + (s.v[379] * (s.dn[520][1] * ddt_scale)));let eq80_e2218_d_n2: f64 = ((s.dn[379][2] * eq80_e2217) + (s.v[379] * (s.dn[520][2] * ddt_scale)));let eq80_e2218_d_n3: f64 = ((s.dn[379][3] * eq80_e2217) + (s.v[379] * (s.dn[520][3] * ddt_scale)));let eq80_e2218_d_n4: f64 = ((s.dn[379][4] * eq80_e2217) + (s.v[379] * (s.dn[520][4] * ddt_scale)));let eq80_e2218_d_n5: f64 = ((s.dn[379][5] * eq80_e2217) + (s.v[379] * (s.dn[520][5] * ddt_scale)));let eq80_e2218_d_n6: f64 = ((s.dn[379][6] * eq80_e2217) + (s.v[379] * (s.dn[520][6] * ddt_scale)));let eq80_e2218_d_n7: f64 = ((s.dn[379][7] * eq80_e2217) + (s.v[379] * (s.dn[520][7] * ddt_scale)));let eq80_e2218_d_n8: f64 = ((s.dn[379][8] * eq80_e2217) + (s.v[379] * (s.dn[520][8] * ddt_scale)));let eq80_e2218_d_n9: f64 = ((s.dn[379][9] * eq80_e2217) + (s.v[379] * (s.dn[520][9] * ddt_scale)));let eq80_e2218_d_n10: f64 = ((s.dn[379][10] * eq80_e2217) + (s.v[379] * (s.dn[520][10] * ddt_scale)));let eq80_e2218_d_n11: f64 = ((s.dn[379][11] * eq80_e2217) + (s.v[379] * (s.dn[520][11] * ddt_scale)));let eq80_e2218_d_n12: f64 = ((s.dn[379][12] * eq80_e2217) + (s.v[379] * (s.dn[520][12] * ddt_scale)));let eq80_e2218_d_n13: f64 = ((s.dn[379][13] * eq80_e2217) + (s.v[379] * (s.dn[520][13] * ddt_scale)));let eq80_e2218_d_b0: f64 = ((s.db[379][0] * eq80_e2217) + (s.v[379] * (s.db[520][0] * ddt_scale)));let eq80_e2218_d_b1: f64 = ((s.db[379][1] * eq80_e2217) + (s.v[379] * (s.db[520][1] * ddt_scale)));let eq80_e2218_d_b2: f64 = ((s.db[379][2] * eq80_e2217) + (s.v[379] * (s.db[520][2] * ddt_scale)));let eq80_e2218_d_b3: f64 = ((s.db[379][3] * eq80_e2217) + (s.v[379] * (s.db[520][3] * ddt_scale)));let eq80_e2218_d_b4: f64 = ((s.db[379][4] * eq80_e2217) + (s.v[379] * (s.db[520][4] * ddt_scale)));let eq80_e2218_d_b5: f64 = ((s.db[379][5] * eq80_e2217) + (s.v[379] * (s.db[520][5] * ddt_scale)));let eq80_e2218_d_b6: f64 = ((s.db[379][6] * eq80_e2217) + (s.v[379] * (s.db[520][6] * ddt_scale)));let eq80_e2218_d_b7: f64 = ((s.db[379][7] * eq80_e2217) + (s.v[379] * (s.db[520][7] * ddt_scale)));let eq80_e2218_d_b8: f64 = ((s.db[379][8] * eq80_e2217) + (s.v[379] * (s.db[520][8] * ddt_scale)));let eq80_e2218_d_b9: f64 = ((s.db[379][9] * eq80_e2217) + (s.v[379] * (s.db[520][9] * ddt_scale)));let eq80_e2218_d_b10: f64 = ((s.db[379][10] * eq80_e2217) + (s.v[379] * (s.db[520][10] * ddt_scale)));let eq80_e2218_d_b11: f64 = ((s.db[379][11] * eq80_e2217) + (s.v[379] * (s.db[520][11] * ddt_scale)));let eq80_value: f64 = eq80_e2218;let eq80_node_derivatives: [f64; 14] = [eq80_e2218_d_n0, eq80_e2218_d_n1, eq80_e2218_d_n2, eq80_e2218_d_n3, eq80_e2218_d_n4, eq80_e2218_d_n5, eq80_e2218_d_n6, eq80_e2218_d_n7, eq80_e2218_d_n8, eq80_e2218_d_n9, eq80_e2218_d_n10, eq80_e2218_d_n11, eq80_e2218_d_n12, eq80_e2218_d_n13];let eq80_branch_derivatives: [f64; 12] = [eq80_e2218_d_b0, eq80_e2218_d_b1, eq80_e2218_d_b2, eq80_e2218_d_b3, eq80_e2218_d_b4, eq80_e2218_d_b5, eq80_e2218_d_b6, eq80_e2218_d_b7, eq80_e2218_d_b8, eq80_e2218_d_b9, eq80_e2218_d_b10, eq80_e2218_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq80_value),
            &eq80_node_derivatives,
            &eq80_branch_derivatives,
            multiplicity,
        );let eq81_e2221: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, s.v[525]);let eq81_e2222: f64 = (s.v[379] * eq81_e2221);let eq81_e2222_d_n0: f64 = ((s.dn[379][0] * eq81_e2221) + (s.v[379] * (s.dn[525][0] * ddt_scale)));let eq81_e2222_d_n1: f64 = ((s.dn[379][1] * eq81_e2221) + (s.v[379] * (s.dn[525][1] * ddt_scale)));let eq81_e2222_d_n2: f64 = ((s.dn[379][2] * eq81_e2221) + (s.v[379] * (s.dn[525][2] * ddt_scale)));let eq81_e2222_d_n3: f64 = ((s.dn[379][3] * eq81_e2221) + (s.v[379] * (s.dn[525][3] * ddt_scale)));let eq81_e2222_d_n4: f64 = ((s.dn[379][4] * eq81_e2221) + (s.v[379] * (s.dn[525][4] * ddt_scale)));let eq81_e2222_d_n5: f64 = ((s.dn[379][5] * eq81_e2221) + (s.v[379] * (s.dn[525][5] * ddt_scale)));let eq81_e2222_d_n6: f64 = ((s.dn[379][6] * eq81_e2221) + (s.v[379] * (s.dn[525][6] * ddt_scale)));let eq81_e2222_d_n7: f64 = ((s.dn[379][7] * eq81_e2221) + (s.v[379] * (s.dn[525][7] * ddt_scale)));let eq81_e2222_d_n8: f64 = ((s.dn[379][8] * eq81_e2221) + (s.v[379] * (s.dn[525][8] * ddt_scale)));let eq81_e2222_d_n9: f64 = ((s.dn[379][9] * eq81_e2221) + (s.v[379] * (s.dn[525][9] * ddt_scale)));let eq81_e2222_d_n10: f64 = ((s.dn[379][10] * eq81_e2221) + (s.v[379] * (s.dn[525][10] * ddt_scale)));let eq81_e2222_d_n11: f64 = ((s.dn[379][11] * eq81_e2221) + (s.v[379] * (s.dn[525][11] * ddt_scale)));let eq81_e2222_d_n12: f64 = ((s.dn[379][12] * eq81_e2221) + (s.v[379] * (s.dn[525][12] * ddt_scale)));let eq81_e2222_d_n13: f64 = ((s.dn[379][13] * eq81_e2221) + (s.v[379] * (s.dn[525][13] * ddt_scale)));let eq81_e2222_d_b0: f64 = ((s.db[379][0] * eq81_e2221) + (s.v[379] * (s.db[525][0] * ddt_scale)));let eq81_e2222_d_b1: f64 = ((s.db[379][1] * eq81_e2221) + (s.v[379] * (s.db[525][1] * ddt_scale)));let eq81_e2222_d_b2: f64 = ((s.db[379][2] * eq81_e2221) + (s.v[379] * (s.db[525][2] * ddt_scale)));let eq81_e2222_d_b3: f64 = ((s.db[379][3] * eq81_e2221) + (s.v[379] * (s.db[525][3] * ddt_scale)));let eq81_e2222_d_b4: f64 = ((s.db[379][4] * eq81_e2221) + (s.v[379] * (s.db[525][4] * ddt_scale)));let eq81_e2222_d_b5: f64 = ((s.db[379][5] * eq81_e2221) + (s.v[379] * (s.db[525][5] * ddt_scale)));let eq81_e2222_d_b6: f64 = ((s.db[379][6] * eq81_e2221) + (s.v[379] * (s.db[525][6] * ddt_scale)));let eq81_e2222_d_b7: f64 = ((s.db[379][7] * eq81_e2221) + (s.v[379] * (s.db[525][7] * ddt_scale)));let eq81_e2222_d_b8: f64 = ((s.db[379][8] * eq81_e2221) + (s.v[379] * (s.db[525][8] * ddt_scale)));let eq81_e2222_d_b9: f64 = ((s.db[379][9] * eq81_e2221) + (s.v[379] * (s.db[525][9] * ddt_scale)));let eq81_e2222_d_b10: f64 = ((s.db[379][10] * eq81_e2221) + (s.v[379] * (s.db[525][10] * ddt_scale)));let eq81_e2222_d_b11: f64 = ((s.db[379][11] * eq81_e2221) + (s.v[379] * (s.db[525][11] * ddt_scale)));let eq81_value: f64 = eq81_e2222;let eq81_node_derivatives: [f64; 14] = [eq81_e2222_d_n0, eq81_e2222_d_n1, eq81_e2222_d_n2, eq81_e2222_d_n3, eq81_e2222_d_n4, eq81_e2222_d_n5, eq81_e2222_d_n6, eq81_e2222_d_n7, eq81_e2222_d_n8, eq81_e2222_d_n9, eq81_e2222_d_n10, eq81_e2222_d_n11, eq81_e2222_d_n12, eq81_e2222_d_n13];let eq81_branch_derivatives: [f64; 12] = [eq81_e2222_d_b0, eq81_e2222_d_b1, eq81_e2222_d_b2, eq81_e2222_d_b3, eq81_e2222_d_b4, eq81_e2222_d_b5, eq81_e2222_d_b6, eq81_e2222_d_b7, eq81_e2222_d_b8, eq81_e2222_d_b9, eq81_e2222_d_b10, eq81_e2222_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq81_value),
            &eq81_node_derivatives,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e2228, eq82_e2228_d_n7, eq82_e2228_d_n11,) = {
    if s.b[2039] {
        let eq82_e2226: f64 = ((nv11 - nv7) * s.v[1018]);
        (eq82_e2226, (-s.v[1018]), s.v[1018],)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e2228;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(7),
            multiplicity * (eq82_value),
            7,
            multiplicity * (eq82_e2228_d_n7),
            11,
            multiplicity * (eq82_e2228_d_n11),
        );
        let (eq83_e2234, eq83_e2234_d_n6, eq83_e2234_d_n11,) = {
    if s.b[2039] {
        let eq83_e2232: f64 = ((nv11 - nv6) * s.v[1018]);
        (eq83_e2232, (-s.v[1018]), s.v[1018],)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e2234;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(6),
            multiplicity * (eq83_value),
            6,
            multiplicity * (eq83_e2234_d_n6),
            11,
            multiplicity * (eq83_e2234_d_n11),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1551, eq7_e1551_d_n0, eq7_e1551_d_n1, eq7_e1551_d_n2, eq7_e1551_d_n3, eq7_e1551_d_n4, eq7_e1551_d_n5, eq7_e1551_d_n6, eq7_e1551_d_n7, eq7_e1551_d_n8, eq7_e1551_d_n9, eq7_e1551_d_n10, eq7_e1551_d_n11, eq7_e1551_d_n12, eq7_e1551_d_n13, eq7_e1551_d_b0, eq7_e1551_d_b1, eq7_e1551_d_b2, eq7_e1551_d_b3, eq7_e1551_d_b4, eq7_e1551_d_b5, eq7_e1551_d_b6, eq7_e1551_d_b7, eq7_e1551_d_b8, eq7_e1551_d_b9, eq7_e1551_d_b10, eq7_e1551_d_b11, eq7_e1551_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq7_e1540: f64 = (s.v[622] * s.v[199]);let eq7_e1542: f64 = (eq7_e1540 * s.v[183]);let eq7_e1542_d_n0: f64 = ((s.dn[622][0] * s.v[199]) * s.v[183]);let eq7_e1542_d_n1: f64 = ((s.dn[622][1] * s.v[199]) * s.v[183]);let eq7_e1542_d_n2: f64 = ((s.dn[622][2] * s.v[199]) * s.v[183]);let eq7_e1542_d_n3: f64 = ((s.dn[622][3] * s.v[199]) * s.v[183]);let eq7_e1542_d_n4: f64 = ((s.dn[622][4] * s.v[199]) * s.v[183]);let eq7_e1542_d_n5: f64 = ((s.dn[622][5] * s.v[199]) * s.v[183]);let eq7_e1542_d_n6: f64 = ((s.dn[622][6] * s.v[199]) * s.v[183]);let eq7_e1542_d_n7: f64 = ((s.dn[622][7] * s.v[199]) * s.v[183]);let eq7_e1542_d_n8: f64 = ((s.dn[622][8] * s.v[199]) * s.v[183]);let eq7_e1542_d_n9: f64 = ((s.dn[622][9] * s.v[199]) * s.v[183]);let eq7_e1542_d_n10: f64 = ((s.dn[622][10] * s.v[199]) * s.v[183]);let eq7_e1542_d_n11: f64 = ((s.dn[622][11] * s.v[199]) * s.v[183]);let eq7_e1542_d_n12: f64 = ((s.dn[622][12] * s.v[199]) * s.v[183]);let eq7_e1542_d_n13: f64 = ((s.dn[622][13] * s.v[199]) * s.v[183]);let eq7_e1542_d_b0: f64 = ((s.db[622][0] * s.v[199]) * s.v[183]);let eq7_e1542_d_b1: f64 = ((s.db[622][1] * s.v[199]) * s.v[183]);let eq7_e1542_d_b2: f64 = ((s.db[622][2] * s.v[199]) * s.v[183]);let eq7_e1542_d_b3: f64 = ((s.db[622][3] * s.v[199]) * s.v[183]);let eq7_e1542_d_b4: f64 = ((s.db[622][4] * s.v[199]) * s.v[183]);let eq7_e1542_d_b5: f64 = ((s.db[622][5] * s.v[199]) * s.v[183]);let eq7_e1542_d_b6: f64 = ((s.db[622][6] * s.v[199]) * s.v[183]);let eq7_e1542_d_b7: f64 = ((s.db[622][7] * s.v[199]) * s.v[183]);let eq7_e1542_d_b8: f64 = ((s.db[622][8] * s.v[199]) * s.v[183]);let eq7_e1542_d_b9: f64 = ((s.db[622][9] * s.v[199]) * s.v[183]);let eq7_e1542_d_b10: f64 = ((s.db[622][10] * s.v[199]) * s.v[183]);let eq7_e1542_d_b11: f64 = ((s.db[622][11] * s.v[199]) * s.v[183]);let eq7_e1544: f64 = (eq7_e1542 * p[2]);let eq7_e1544_d_n0: f64 = (eq7_e1542_d_n0 * p[2]);let eq7_e1544_d_n1: f64 = (eq7_e1542_d_n1 * p[2]);let eq7_e1544_d_n2: f64 = (eq7_e1542_d_n2 * p[2]);let eq7_e1544_d_n3: f64 = (eq7_e1542_d_n3 * p[2]);let eq7_e1544_d_n4: f64 = (eq7_e1542_d_n4 * p[2]);let eq7_e1544_d_n5: f64 = (eq7_e1542_d_n5 * p[2]);let eq7_e1544_d_n6: f64 = (eq7_e1542_d_n6 * p[2]);let eq7_e1544_d_n7: f64 = (eq7_e1542_d_n7 * p[2]);let eq7_e1544_d_n8: f64 = (eq7_e1542_d_n8 * p[2]);let eq7_e1544_d_n9: f64 = (eq7_e1542_d_n9 * p[2]);let eq7_e1544_d_n10: f64 = (eq7_e1542_d_n10 * p[2]);let eq7_e1544_d_n11: f64 = (eq7_e1542_d_n11 * p[2]);let eq7_e1544_d_n12: f64 = (eq7_e1542_d_n12 * p[2]);let eq7_e1544_d_n13: f64 = (eq7_e1542_d_n13 * p[2]);let eq7_e1544_d_b0: f64 = (eq7_e1542_d_b0 * p[2]);let eq7_e1544_d_b1: f64 = (eq7_e1542_d_b1 * p[2]);let eq7_e1544_d_b2: f64 = (eq7_e1542_d_b2 * p[2]);let eq7_e1544_d_b3: f64 = (eq7_e1542_d_b3 * p[2]);let eq7_e1544_d_b4: f64 = (eq7_e1542_d_b4 * p[2]);let eq7_e1544_d_b5: f64 = (eq7_e1542_d_b5 * p[2]);let eq7_e1544_d_b6: f64 = (eq7_e1542_d_b6 * p[2]);let eq7_e1544_d_b7: f64 = (eq7_e1542_d_b7 * p[2]);let eq7_e1544_d_b8: f64 = (eq7_e1542_d_b8 * p[2]);let eq7_e1544_d_b9: f64 = (eq7_e1542_d_b9 * p[2]);let eq7_e1544_d_b10: f64 = (eq7_e1542_d_b10 * p[2]);let eq7_e1544_d_b11: f64 = (eq7_e1542_d_b11 * p[2]);let eq7_e1546: f64 = (eq7_e1544 * s.v[184]);let eq7_e1546_d_n0: f64 = (eq7_e1544_d_n0 * s.v[184]);let eq7_e1546_d_n1: f64 = (eq7_e1544_d_n1 * s.v[184]);let eq7_e1546_d_n2: f64 = (eq7_e1544_d_n2 * s.v[184]);let eq7_e1546_d_n3: f64 = (eq7_e1544_d_n3 * s.v[184]);let eq7_e1546_d_n4: f64 = (eq7_e1544_d_n4 * s.v[184]);let eq7_e1546_d_n5: f64 = (eq7_e1544_d_n5 * s.v[184]);let eq7_e1546_d_n6: f64 = (eq7_e1544_d_n6 * s.v[184]);let eq7_e1546_d_n7: f64 = (eq7_e1544_d_n7 * s.v[184]);let eq7_e1546_d_n8: f64 = (eq7_e1544_d_n8 * s.v[184]);let eq7_e1546_d_n9: f64 = (eq7_e1544_d_n9 * s.v[184]);let eq7_e1546_d_n10: f64 = (eq7_e1544_d_n10 * s.v[184]);let eq7_e1546_d_n11: f64 = (eq7_e1544_d_n11 * s.v[184]);let eq7_e1546_d_n12: f64 = (eq7_e1544_d_n12 * s.v[184]);let eq7_e1546_d_n13: f64 = (eq7_e1544_d_n13 * s.v[184]);let eq7_e1546_d_b0: f64 = (eq7_e1544_d_b0 * s.v[184]);let eq7_e1546_d_b1: f64 = (eq7_e1544_d_b1 * s.v[184]);
        let eq7_e1546_d_b2: f64 = (eq7_e1544_d_b2 * s.v[184]);let eq7_e1546_d_b3: f64 = (eq7_e1544_d_b3 * s.v[184]);let eq7_e1546_d_b4: f64 = (eq7_e1544_d_b4 * s.v[184]);let eq7_e1546_d_b5: f64 = (eq7_e1544_d_b5 * s.v[184]);let eq7_e1546_d_b6: f64 = (eq7_e1544_d_b6 * s.v[184]);let eq7_e1546_d_b7: f64 = (eq7_e1544_d_b7 * s.v[184]);let eq7_e1546_d_b8: f64 = (eq7_e1544_d_b8 * s.v[184]);let eq7_e1546_d_b9: f64 = (eq7_e1544_d_b9 * s.v[184]);let eq7_e1546_d_b10: f64 = (eq7_e1544_d_b10 * s.v[184]);let eq7_e1546_d_b11: f64 = (eq7_e1544_d_b11 * s.v[184]);let eq7_e1548: f64 = (eq7_e1546 * (nv12 - 0.0));let eq7_e1548_d_n0: f64 = (eq7_e1546_d_n0 * (nv12 - 0.0));let eq7_e1548_d_n1: f64 = (eq7_e1546_d_n1 * (nv12 - 0.0));let eq7_e1548_d_n2: f64 = (eq7_e1546_d_n2 * (nv12 - 0.0));let eq7_e1548_d_n3: f64 = (eq7_e1546_d_n3 * (nv12 - 0.0));let eq7_e1548_d_n4: f64 = (eq7_e1546_d_n4 * (nv12 - 0.0));let eq7_e1548_d_n5: f64 = (eq7_e1546_d_n5 * (nv12 - 0.0));let eq7_e1548_d_n6: f64 = (eq7_e1546_d_n6 * (nv12 - 0.0));let eq7_e1548_d_n7: f64 = (eq7_e1546_d_n7 * (nv12 - 0.0));let eq7_e1548_d_n8: f64 = (eq7_e1546_d_n8 * (nv12 - 0.0));let eq7_e1548_d_n9: f64 = (eq7_e1546_d_n9 * (nv12 - 0.0));let eq7_e1548_d_n10: f64 = (eq7_e1546_d_n10 * (nv12 - 0.0));let eq7_e1548_d_n11: f64 = (eq7_e1546_d_n11 * (nv12 - 0.0));let eq7_e1548_d_n12: f64 = ((eq7_e1546_d_n12 * (nv12 - 0.0)) + eq7_e1546);let eq7_e1548_d_n13: f64 = (eq7_e1546_d_n13 * (nv12 - 0.0));let eq7_e1548_d_b0: f64 = (eq7_e1546_d_b0 * (nv12 - 0.0));let eq7_e1548_d_b1: f64 = (eq7_e1546_d_b1 * (nv12 - 0.0));let eq7_e1548_d_b2: f64 = (eq7_e1546_d_b2 * (nv12 - 0.0));let eq7_e1548_d_b3: f64 = (eq7_e1546_d_b3 * (nv12 - 0.0));let eq7_e1548_d_b4: f64 = (eq7_e1546_d_b4 * (nv12 - 0.0));let eq7_e1548_d_b5: f64 = (eq7_e1546_d_b5 * (nv12 - 0.0));let eq7_e1548_d_b6: f64 = (eq7_e1546_d_b6 * (nv12 - 0.0));let eq7_e1548_d_b7: f64 = (eq7_e1546_d_b7 * (nv12 - 0.0));let eq7_e1548_d_b8: f64 = (eq7_e1546_d_b8 * (nv12 - 0.0));let eq7_e1548_d_b9: f64 = (eq7_e1546_d_b9 * (nv12 - 0.0));let eq7_e1548_d_b10: f64 = (eq7_e1546_d_b10 * (nv12 - 0.0));let eq7_e1548_d_b11: f64 = (eq7_e1546_d_b11 * (nv12 - 0.0));let eq7_e1549_q: f64 = eq7_e1548;
        (eq7_e1548, eq7_e1548_d_n0, eq7_e1548_d_n1, eq7_e1548_d_n2, eq7_e1548_d_n3, eq7_e1548_d_n4, eq7_e1548_d_n5, eq7_e1548_d_n6, eq7_e1548_d_n7, eq7_e1548_d_n8, eq7_e1548_d_n9, eq7_e1548_d_n10, eq7_e1548_d_n11, eq7_e1548_d_n12, eq7_e1548_d_n13, eq7_e1548_d_b0, eq7_e1548_d_b1, eq7_e1548_d_b2, eq7_e1548_d_b3, eq7_e1548_d_b4, eq7_e1548_d_b5, eq7_e1548_d_b6, eq7_e1548_d_b7, eq7_e1548_d_b8, eq7_e1548_d_b9, eq7_e1548_d_b10, eq7_e1548_d_b11, eq7_e1549_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [eq7_e1551_d_n0, eq7_e1551_d_n1, eq7_e1551_d_n2, eq7_e1551_d_n3, eq7_e1551_d_n4, eq7_e1551_d_n5, eq7_e1551_d_n6, eq7_e1551_d_n7, eq7_e1551_d_n8, eq7_e1551_d_n9, eq7_e1551_d_n10, eq7_e1551_d_n11, eq7_e1551_d_n12, eq7_e1551_d_n13];let eq7_reactive_branch_derivatives: [f64; 12] = [eq7_e1551_d_b0, eq7_e1551_d_b1, eq7_e1551_d_b2, eq7_e1551_d_b3, eq7_e1551_d_b4, eq7_e1551_d_b5, eq7_e1551_d_b6, eq7_e1551_d_b7, eq7_e1551_d_b8, eq7_e1551_d_b9, eq7_e1551_d_b10, eq7_e1551_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            None,
            &eq7_reactive_node_derivatives,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq10_e1605, eq10_e1605_d_n0, eq10_e1605_d_n1, eq10_e1605_d_n2, eq10_e1605_d_n3, eq10_e1605_d_n4, eq10_e1605_d_n5, eq10_e1605_d_n6, eq10_e1605_d_n7, eq10_e1605_d_n8, eq10_e1605_d_n9, eq10_e1605_d_n10, eq10_e1605_d_n11, eq10_e1605_d_n12, eq10_e1605_d_n13, eq10_e1605_d_b0, eq10_e1605_d_b1, eq10_e1605_d_b2, eq10_e1605_d_b3, eq10_e1605_d_b4, eq10_e1605_d_b5, eq10_e1605_d_b6, eq10_e1605_d_b7, eq10_e1605_d_b8, eq10_e1605_d_b9, eq10_e1605_d_b10, eq10_e1605_d_b11, eq10_e1605_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq10_e1589: f64 = (1.0 + s.v[211]);let eq10_e1591: f64 = (eq10_e1589 * s.v[622]);let eq10_e1591_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1589 * s.dn[622][0]));let eq10_e1591_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1589 * s.dn[622][1]));let eq10_e1591_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1589 * s.dn[622][2]));let eq10_e1591_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1589 * s.dn[622][3]));let eq10_e1591_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1589 * s.dn[622][4]));let eq10_e1591_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1589 * s.dn[622][5]));let eq10_e1591_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1589 * s.dn[622][6]));let eq10_e1591_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1589 * s.dn[622][7]));let eq10_e1591_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1589 * s.dn[622][8]));let eq10_e1591_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1589 * s.dn[622][9]));let eq10_e1591_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1589 * s.dn[622][10]));let eq10_e1591_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1589 * s.dn[622][11]));let eq10_e1591_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1589 * s.dn[622][12]));let eq10_e1591_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1589 * s.dn[622][13]));let eq10_e1591_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq10_e1589 * s.db[622][0]));let eq10_e1591_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq10_e1589 * s.db[622][1]));let eq10_e1591_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq10_e1589 * s.db[622][2]));let eq10_e1591_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq10_e1589 * s.db[622][3]));let eq10_e1591_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq10_e1589 * s.db[622][4]));let eq10_e1591_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq10_e1589 * s.db[622][5]));let eq10_e1591_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq10_e1589 * s.db[622][6]));let eq10_e1591_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq10_e1589 * s.db[622][7]));let eq10_e1591_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq10_e1589 * s.db[622][8]));let eq10_e1591_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq10_e1589 * s.db[622][9]));let eq10_e1591_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq10_e1589 * s.db[622][10]));let eq10_e1591_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq10_e1589 * s.db[622][11]));let eq10_e1593: f64 = (eq10_e1591 * s.v[199]);let eq10_e1593_d_n0: f64 = (eq10_e1591_d_n0 * s.v[199]);let eq10_e1593_d_n1: f64 = (eq10_e1591_d_n1 * s.v[199]);let eq10_e1593_d_n2: f64 = (eq10_e1591_d_n2 * s.v[199]);let eq10_e1593_d_n3: f64 = (eq10_e1591_d_n3 * s.v[199]);let eq10_e1593_d_n4: f64 = (eq10_e1591_d_n4 * s.v[199]);let eq10_e1593_d_n5: f64 = (eq10_e1591_d_n5 * s.v[199]);let eq10_e1593_d_n6: f64 = (eq10_e1591_d_n6 * s.v[199]);let eq10_e1593_d_n7: f64 = (eq10_e1591_d_n7 * s.v[199]);let eq10_e1593_d_n8: f64 = (eq10_e1591_d_n8 * s.v[199]);let eq10_e1593_d_n9: f64 = (eq10_e1591_d_n9 * s.v[199]);let eq10_e1593_d_n10: f64 = (eq10_e1591_d_n10 * s.v[199]);let eq10_e1593_d_n11: f64 = (eq10_e1591_d_n11 * s.v[199]);let eq10_e1593_d_n12: f64 = (eq10_e1591_d_n12 * s.v[199]);let eq10_e1593_d_n13: f64 = (eq10_e1591_d_n13 * s.v[199]);let eq10_e1593_d_b0: f64 = (eq10_e1591_d_b0 * s.v[199]);let eq10_e1593_d_b1: f64 = (eq10_e1591_d_b1 * s.v[199]);let eq10_e1593_d_b2: f64 = (eq10_e1591_d_b2 * s.v[199]);let eq10_e1593_d_b3: f64 = (eq10_e1591_d_b3 * s.v[199]);let eq10_e1593_d_b4: f64 = (eq10_e1591_d_b4 * s.v[199]);let eq10_e1593_d_b5: f64 = (eq10_e1591_d_b5 * s.v[199]);let eq10_e1593_d_b6: f64 = (eq10_e1591_d_b6 * s.v[199]);let eq10_e1593_d_b7: f64 = (eq10_e1591_d_b7 * s.v[199]);let eq10_e1593_d_b8: f64 = (eq10_e1591_d_b8 * s.v[199]);let eq10_e1593_d_b9: f64 = (eq10_e1591_d_b9 * s.v[199]);let eq10_e1593_d_b10: f64 = (eq10_e1591_d_b10 * s.v[199]);let eq10_e1593_d_b11: f64 = (eq10_e1591_d_b11 * s.v[199]);let eq10_e1595: f64 = (eq10_e1593 * s.v[183]);let eq10_e1595_d_n0: f64 = (eq10_e1593_d_n0 * s.v[183]);let eq10_e1595_d_n1: f64 = (eq10_e1593_d_n1 * s.v[183]);let eq10_e1595_d_n2: f64 = (eq10_e1593_d_n2 * s.v[183]);
        let eq10_e1595_d_n3: f64 = (eq10_e1593_d_n3 * s.v[183]);let eq10_e1595_d_n4: f64 = (eq10_e1593_d_n4 * s.v[183]);let eq10_e1595_d_n5: f64 = (eq10_e1593_d_n5 * s.v[183]);let eq10_e1595_d_n6: f64 = (eq10_e1593_d_n6 * s.v[183]);let eq10_e1595_d_n7: f64 = (eq10_e1593_d_n7 * s.v[183]);let eq10_e1595_d_n8: f64 = (eq10_e1593_d_n8 * s.v[183]);let eq10_e1595_d_n9: f64 = (eq10_e1593_d_n9 * s.v[183]);let eq10_e1595_d_n10: f64 = (eq10_e1593_d_n10 * s.v[183]);let eq10_e1595_d_n11: f64 = (eq10_e1593_d_n11 * s.v[183]);let eq10_e1595_d_n12: f64 = (eq10_e1593_d_n12 * s.v[183]);let eq10_e1595_d_n13: f64 = (eq10_e1593_d_n13 * s.v[183]);let eq10_e1595_d_b0: f64 = (eq10_e1593_d_b0 * s.v[183]);let eq10_e1595_d_b1: f64 = (eq10_e1593_d_b1 * s.v[183]);let eq10_e1595_d_b2: f64 = (eq10_e1593_d_b2 * s.v[183]);let eq10_e1595_d_b3: f64 = (eq10_e1593_d_b3 * s.v[183]);let eq10_e1595_d_b4: f64 = (eq10_e1593_d_b4 * s.v[183]);let eq10_e1595_d_b5: f64 = (eq10_e1593_d_b5 * s.v[183]);let eq10_e1595_d_b6: f64 = (eq10_e1593_d_b6 * s.v[183]);let eq10_e1595_d_b7: f64 = (eq10_e1593_d_b7 * s.v[183]);let eq10_e1595_d_b8: f64 = (eq10_e1593_d_b8 * s.v[183]);let eq10_e1595_d_b9: f64 = (eq10_e1593_d_b9 * s.v[183]);let eq10_e1595_d_b10: f64 = (eq10_e1593_d_b10 * s.v[183]);let eq10_e1595_d_b11: f64 = (eq10_e1593_d_b11 * s.v[183]);let eq10_e1597: f64 = (eq10_e1595 * p[2]);let eq10_e1597_d_n0: f64 = (eq10_e1595_d_n0 * p[2]);let eq10_e1597_d_n1: f64 = (eq10_e1595_d_n1 * p[2]);let eq10_e1597_d_n2: f64 = (eq10_e1595_d_n2 * p[2]);let eq10_e1597_d_n3: f64 = (eq10_e1595_d_n3 * p[2]);let eq10_e1597_d_n4: f64 = (eq10_e1595_d_n4 * p[2]);let eq10_e1597_d_n5: f64 = (eq10_e1595_d_n5 * p[2]);let eq10_e1597_d_n6: f64 = (eq10_e1595_d_n6 * p[2]);let eq10_e1597_d_n7: f64 = (eq10_e1595_d_n7 * p[2]);let eq10_e1597_d_n8: f64 = (eq10_e1595_d_n8 * p[2]);let eq10_e1597_d_n9: f64 = (eq10_e1595_d_n9 * p[2]);let eq10_e1597_d_n10: f64 = (eq10_e1595_d_n10 * p[2]);let eq10_e1597_d_n11: f64 = (eq10_e1595_d_n11 * p[2]);let eq10_e1597_d_n12: f64 = (eq10_e1595_d_n12 * p[2]);let eq10_e1597_d_n13: f64 = (eq10_e1595_d_n13 * p[2]);let eq10_e1597_d_b0: f64 = (eq10_e1595_d_b0 * p[2]);let eq10_e1597_d_b1: f64 = (eq10_e1595_d_b1 * p[2]);let eq10_e1597_d_b2: f64 = (eq10_e1595_d_b2 * p[2]);let eq10_e1597_d_b3: f64 = (eq10_e1595_d_b3 * p[2]);let eq10_e1597_d_b4: f64 = (eq10_e1595_d_b4 * p[2]);let eq10_e1597_d_b5: f64 = (eq10_e1595_d_b5 * p[2]);let eq10_e1597_d_b6: f64 = (eq10_e1595_d_b6 * p[2]);let eq10_e1597_d_b7: f64 = (eq10_e1595_d_b7 * p[2]);let eq10_e1597_d_b8: f64 = (eq10_e1595_d_b8 * p[2]);let eq10_e1597_d_b9: f64 = (eq10_e1595_d_b9 * p[2]);let eq10_e1597_d_b10: f64 = (eq10_e1595_d_b10 * p[2]);let eq10_e1597_d_b11: f64 = (eq10_e1595_d_b11 * p[2]);let eq10_e1599: f64 = (eq10_e1597 * s.v[184]);let eq10_e1599_d_n0: f64 = (eq10_e1597_d_n0 * s.v[184]);let eq10_e1599_d_n1: f64 = (eq10_e1597_d_n1 * s.v[184]);let eq10_e1599_d_n2: f64 = (eq10_e1597_d_n2 * s.v[184]);let eq10_e1599_d_n3: f64 = (eq10_e1597_d_n3 * s.v[184]);let eq10_e1599_d_n4: f64 = (eq10_e1597_d_n4 * s.v[184]);let eq10_e1599_d_n5: f64 = (eq10_e1597_d_n5 * s.v[184]);let eq10_e1599_d_n6: f64 = (eq10_e1597_d_n6 * s.v[184]);let eq10_e1599_d_n7: f64 = (eq10_e1597_d_n7 * s.v[184]);let eq10_e1599_d_n8: f64 = (eq10_e1597_d_n8 * s.v[184]);let eq10_e1599_d_n9: f64 = (eq10_e1597_d_n9 * s.v[184]);let eq10_e1599_d_n10: f64 = (eq10_e1597_d_n10 * s.v[184]);let eq10_e1599_d_n11: f64 = (eq10_e1597_d_n11 * s.v[184]);let eq10_e1599_d_n12: f64 = (eq10_e1597_d_n12 * s.v[184]);let eq10_e1599_d_n13: f64 = (eq10_e1597_d_n13 * s.v[184]);let eq10_e1599_d_b0: f64 = (eq10_e1597_d_b0 * s.v[184]);let eq10_e1599_d_b1: f64 = (eq10_e1597_d_b1 * s.v[184]);let eq10_e1599_d_b2: f64 = (eq10_e1597_d_b2 * s.v[184]);let eq10_e1599_d_b3: f64 = (eq10_e1597_d_b3 * s.v[184]);let eq10_e1599_d_b4: f64 = (eq10_e1597_d_b4 * s.v[184]);let eq10_e1599_d_b5: f64 = (eq10_e1597_d_b5 * s.v[184]);let eq10_e1599_d_b6: f64 = (eq10_e1597_d_b6 * s.v[184]);let eq10_e1599_d_b7: f64 = (eq10_e1597_d_b7 * s.v[184]);let eq10_e1599_d_b8: f64 = (eq10_e1597_d_b8 * s.v[184]);
        let eq10_e1599_d_b9: f64 = (eq10_e1597_d_b9 * s.v[184]);let eq10_e1599_d_b10: f64 = (eq10_e1597_d_b10 * s.v[184]);let eq10_e1599_d_b11: f64 = (eq10_e1597_d_b11 * s.v[184]);let eq10_e1601: f64 = (eq10_e1599 * (nv12 - 0.0));let eq10_e1601_d_n0: f64 = (eq10_e1599_d_n0 * (nv12 - 0.0));let eq10_e1601_d_n1: f64 = (eq10_e1599_d_n1 * (nv12 - 0.0));let eq10_e1601_d_n2: f64 = (eq10_e1599_d_n2 * (nv12 - 0.0));let eq10_e1601_d_n3: f64 = (eq10_e1599_d_n3 * (nv12 - 0.0));let eq10_e1601_d_n4: f64 = (eq10_e1599_d_n4 * (nv12 - 0.0));let eq10_e1601_d_n5: f64 = (eq10_e1599_d_n5 * (nv12 - 0.0));let eq10_e1601_d_n6: f64 = (eq10_e1599_d_n6 * (nv12 - 0.0));let eq10_e1601_d_n7: f64 = (eq10_e1599_d_n7 * (nv12 - 0.0));let eq10_e1601_d_n8: f64 = (eq10_e1599_d_n8 * (nv12 - 0.0));let eq10_e1601_d_n9: f64 = (eq10_e1599_d_n9 * (nv12 - 0.0));let eq10_e1601_d_n10: f64 = (eq10_e1599_d_n10 * (nv12 - 0.0));let eq10_e1601_d_n11: f64 = (eq10_e1599_d_n11 * (nv12 - 0.0));let eq10_e1601_d_n12: f64 = ((eq10_e1599_d_n12 * (nv12 - 0.0)) + eq10_e1599);let eq10_e1601_d_n13: f64 = (eq10_e1599_d_n13 * (nv12 - 0.0));let eq10_e1601_d_b0: f64 = (eq10_e1599_d_b0 * (nv12 - 0.0));let eq10_e1601_d_b1: f64 = (eq10_e1599_d_b1 * (nv12 - 0.0));let eq10_e1601_d_b2: f64 = (eq10_e1599_d_b2 * (nv12 - 0.0));let eq10_e1601_d_b3: f64 = (eq10_e1599_d_b3 * (nv12 - 0.0));let eq10_e1601_d_b4: f64 = (eq10_e1599_d_b4 * (nv12 - 0.0));let eq10_e1601_d_b5: f64 = (eq10_e1599_d_b5 * (nv12 - 0.0));let eq10_e1601_d_b6: f64 = (eq10_e1599_d_b6 * (nv12 - 0.0));let eq10_e1601_d_b7: f64 = (eq10_e1599_d_b7 * (nv12 - 0.0));let eq10_e1601_d_b8: f64 = (eq10_e1599_d_b8 * (nv12 - 0.0));let eq10_e1601_d_b9: f64 = (eq10_e1599_d_b9 * (nv12 - 0.0));let eq10_e1601_d_b10: f64 = (eq10_e1599_d_b10 * (nv12 - 0.0));let eq10_e1601_d_b11: f64 = (eq10_e1599_d_b11 * (nv12 - 0.0));let eq10_e1602: f64 = (0.5 * eq10_e1601);let eq10_e1602_d_n0: f64 = (0.5 * eq10_e1601_d_n0);let eq10_e1602_d_n1: f64 = (0.5 * eq10_e1601_d_n1);let eq10_e1602_d_n2: f64 = (0.5 * eq10_e1601_d_n2);let eq10_e1602_d_n3: f64 = (0.5 * eq10_e1601_d_n3);let eq10_e1602_d_n4: f64 = (0.5 * eq10_e1601_d_n4);let eq10_e1602_d_n5: f64 = (0.5 * eq10_e1601_d_n5);let eq10_e1602_d_n6: f64 = (0.5 * eq10_e1601_d_n6);let eq10_e1602_d_n7: f64 = (0.5 * eq10_e1601_d_n7);let eq10_e1602_d_n8: f64 = (0.5 * eq10_e1601_d_n8);let eq10_e1602_d_n9: f64 = (0.5 * eq10_e1601_d_n9);let eq10_e1602_d_n10: f64 = (0.5 * eq10_e1601_d_n10);let eq10_e1602_d_n11: f64 = (0.5 * eq10_e1601_d_n11);let eq10_e1602_d_n12: f64 = (0.5 * eq10_e1601_d_n12);let eq10_e1602_d_n13: f64 = (0.5 * eq10_e1601_d_n13);let eq10_e1602_d_b0: f64 = (0.5 * eq10_e1601_d_b0);let eq10_e1602_d_b1: f64 = (0.5 * eq10_e1601_d_b1);let eq10_e1602_d_b2: f64 = (0.5 * eq10_e1601_d_b2);let eq10_e1602_d_b3: f64 = (0.5 * eq10_e1601_d_b3);let eq10_e1602_d_b4: f64 = (0.5 * eq10_e1601_d_b4);let eq10_e1602_d_b5: f64 = (0.5 * eq10_e1601_d_b5);let eq10_e1602_d_b6: f64 = (0.5 * eq10_e1601_d_b6);let eq10_e1602_d_b7: f64 = (0.5 * eq10_e1601_d_b7);let eq10_e1602_d_b8: f64 = (0.5 * eq10_e1601_d_b8);let eq10_e1602_d_b9: f64 = (0.5 * eq10_e1601_d_b9);let eq10_e1602_d_b10: f64 = (0.5 * eq10_e1601_d_b10);let eq10_e1602_d_b11: f64 = (0.5 * eq10_e1601_d_b11);let eq10_e1603_q: f64 = eq10_e1602;
        (eq10_e1602, eq10_e1602_d_n0, eq10_e1602_d_n1, eq10_e1602_d_n2, eq10_e1602_d_n3, eq10_e1602_d_n4, eq10_e1602_d_n5, eq10_e1602_d_n6, eq10_e1602_d_n7, eq10_e1602_d_n8, eq10_e1602_d_n9, eq10_e1602_d_n10, eq10_e1602_d_n11, eq10_e1602_d_n12, eq10_e1602_d_n13, eq10_e1602_d_b0, eq10_e1602_d_b1, eq10_e1602_d_b2, eq10_e1602_d_b3, eq10_e1602_d_b4, eq10_e1602_d_b5, eq10_e1602_d_b6, eq10_e1602_d_b7, eq10_e1602_d_b8, eq10_e1602_d_b9, eq10_e1602_d_b10, eq10_e1602_d_b11, eq10_e1603_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [eq10_e1605_d_n0, eq10_e1605_d_n1, eq10_e1605_d_n2, eq10_e1605_d_n3, eq10_e1605_d_n4, eq10_e1605_d_n5, eq10_e1605_d_n6, eq10_e1605_d_n7, eq10_e1605_d_n8, eq10_e1605_d_n9, eq10_e1605_d_n10, eq10_e1605_d_n11, eq10_e1605_d_n12, eq10_e1605_d_n13];let eq10_reactive_branch_derivatives: [f64; 12] = [eq10_e1605_d_b0, eq10_e1605_d_b1, eq10_e1605_d_b2, eq10_e1605_d_b3, eq10_e1605_d_b4, eq10_e1605_d_b5, eq10_e1605_d_b6, eq10_e1605_d_b7, eq10_e1605_d_b8, eq10_e1605_d_b9, eq10_e1605_d_b10, eq10_e1605_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(7),
            &eq10_reactive_node_derivatives,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq11_e1631, eq11_e1631_d_n0, eq11_e1631_d_n1, eq11_e1631_d_n2, eq11_e1631_d_n3, eq11_e1631_d_n4, eq11_e1631_d_n5, eq11_e1631_d_n6, eq11_e1631_d_n7, eq11_e1631_d_n8, eq11_e1631_d_n9, eq11_e1631_d_n10, eq11_e1631_d_n11, eq11_e1631_d_n12, eq11_e1631_d_n13, eq11_e1631_d_b0, eq11_e1631_d_b1, eq11_e1631_d_b2, eq11_e1631_d_b3, eq11_e1631_d_b4, eq11_e1631_d_b5, eq11_e1631_d_b6, eq11_e1631_d_b7, eq11_e1631_d_b8, eq11_e1631_d_b9, eq11_e1631_d_b10, eq11_e1631_d_b11, eq11_e1631_q,) = {
    if (s.b[1620] && (s.b[1794] && (!s.b[1793]))) {
        let eq11_e1615: f64 = (1.0 - s.v[211]);let eq11_e1617: f64 = (eq11_e1615 * s.v[622]);let eq11_e1617_d_n0: f64 = (((-s.dn[211][0]) * s.v[622]) + (eq11_e1615 * s.dn[622][0]));let eq11_e1617_d_n1: f64 = (((-s.dn[211][1]) * s.v[622]) + (eq11_e1615 * s.dn[622][1]));let eq11_e1617_d_n2: f64 = (((-s.dn[211][2]) * s.v[622]) + (eq11_e1615 * s.dn[622][2]));let eq11_e1617_d_n3: f64 = (((-s.dn[211][3]) * s.v[622]) + (eq11_e1615 * s.dn[622][3]));let eq11_e1617_d_n4: f64 = (((-s.dn[211][4]) * s.v[622]) + (eq11_e1615 * s.dn[622][4]));let eq11_e1617_d_n5: f64 = (((-s.dn[211][5]) * s.v[622]) + (eq11_e1615 * s.dn[622][5]));let eq11_e1617_d_n6: f64 = (((-s.dn[211][6]) * s.v[622]) + (eq11_e1615 * s.dn[622][6]));let eq11_e1617_d_n7: f64 = (((-s.dn[211][7]) * s.v[622]) + (eq11_e1615 * s.dn[622][7]));let eq11_e1617_d_n8: f64 = (((-s.dn[211][8]) * s.v[622]) + (eq11_e1615 * s.dn[622][8]));let eq11_e1617_d_n9: f64 = (((-s.dn[211][9]) * s.v[622]) + (eq11_e1615 * s.dn[622][9]));let eq11_e1617_d_n10: f64 = (((-s.dn[211][10]) * s.v[622]) + (eq11_e1615 * s.dn[622][10]));let eq11_e1617_d_n11: f64 = (((-s.dn[211][11]) * s.v[622]) + (eq11_e1615 * s.dn[622][11]));let eq11_e1617_d_n12: f64 = (((-s.dn[211][12]) * s.v[622]) + (eq11_e1615 * s.dn[622][12]));let eq11_e1617_d_n13: f64 = (((-s.dn[211][13]) * s.v[622]) + (eq11_e1615 * s.dn[622][13]));let eq11_e1617_d_b0: f64 = (((-s.db[211][0]) * s.v[622]) + (eq11_e1615 * s.db[622][0]));let eq11_e1617_d_b1: f64 = (((-s.db[211][1]) * s.v[622]) + (eq11_e1615 * s.db[622][1]));let eq11_e1617_d_b2: f64 = (((-s.db[211][2]) * s.v[622]) + (eq11_e1615 * s.db[622][2]));let eq11_e1617_d_b3: f64 = (((-s.db[211][3]) * s.v[622]) + (eq11_e1615 * s.db[622][3]));let eq11_e1617_d_b4: f64 = (((-s.db[211][4]) * s.v[622]) + (eq11_e1615 * s.db[622][4]));let eq11_e1617_d_b5: f64 = (((-s.db[211][5]) * s.v[622]) + (eq11_e1615 * s.db[622][5]));let eq11_e1617_d_b6: f64 = (((-s.db[211][6]) * s.v[622]) + (eq11_e1615 * s.db[622][6]));let eq11_e1617_d_b7: f64 = (((-s.db[211][7]) * s.v[622]) + (eq11_e1615 * s.db[622][7]));let eq11_e1617_d_b8: f64 = (((-s.db[211][8]) * s.v[622]) + (eq11_e1615 * s.db[622][8]));let eq11_e1617_d_b9: f64 = (((-s.db[211][9]) * s.v[622]) + (eq11_e1615 * s.db[622][9]));let eq11_e1617_d_b10: f64 = (((-s.db[211][10]) * s.v[622]) + (eq11_e1615 * s.db[622][10]));let eq11_e1617_d_b11: f64 = (((-s.db[211][11]) * s.v[622]) + (eq11_e1615 * s.db[622][11]));let eq11_e1619: f64 = (eq11_e1617 * s.v[199]);let eq11_e1619_d_n0: f64 = (eq11_e1617_d_n0 * s.v[199]);let eq11_e1619_d_n1: f64 = (eq11_e1617_d_n1 * s.v[199]);let eq11_e1619_d_n2: f64 = (eq11_e1617_d_n2 * s.v[199]);let eq11_e1619_d_n3: f64 = (eq11_e1617_d_n3 * s.v[199]);let eq11_e1619_d_n4: f64 = (eq11_e1617_d_n4 * s.v[199]);let eq11_e1619_d_n5: f64 = (eq11_e1617_d_n5 * s.v[199]);let eq11_e1619_d_n6: f64 = (eq11_e1617_d_n6 * s.v[199]);let eq11_e1619_d_n7: f64 = (eq11_e1617_d_n7 * s.v[199]);let eq11_e1619_d_n8: f64 = (eq11_e1617_d_n8 * s.v[199]);let eq11_e1619_d_n9: f64 = (eq11_e1617_d_n9 * s.v[199]);let eq11_e1619_d_n10: f64 = (eq11_e1617_d_n10 * s.v[199]);let eq11_e1619_d_n11: f64 = (eq11_e1617_d_n11 * s.v[199]);let eq11_e1619_d_n12: f64 = (eq11_e1617_d_n12 * s.v[199]);let eq11_e1619_d_n13: f64 = (eq11_e1617_d_n13 * s.v[199]);let eq11_e1619_d_b0: f64 = (eq11_e1617_d_b0 * s.v[199]);let eq11_e1619_d_b1: f64 = (eq11_e1617_d_b1 * s.v[199]);let eq11_e1619_d_b2: f64 = (eq11_e1617_d_b2 * s.v[199]);let eq11_e1619_d_b3: f64 = (eq11_e1617_d_b3 * s.v[199]);let eq11_e1619_d_b4: f64 = (eq11_e1617_d_b4 * s.v[199]);let eq11_e1619_d_b5: f64 = (eq11_e1617_d_b5 * s.v[199]);let eq11_e1619_d_b6: f64 = (eq11_e1617_d_b6 * s.v[199]);let eq11_e1619_d_b7: f64 = (eq11_e1617_d_b7 * s.v[199]);let eq11_e1619_d_b8: f64 = (eq11_e1617_d_b8 * s.v[199]);let eq11_e1619_d_b9: f64 = (eq11_e1617_d_b9 * s.v[199]);let eq11_e1619_d_b10: f64 = (eq11_e1617_d_b10 * s.v[199]);let eq11_e1619_d_b11: f64 = (eq11_e1617_d_b11 * s.v[199]);let eq11_e1621: f64 = (eq11_e1619 * s.v[183]);let eq11_e1621_d_n0: f64 = (eq11_e1619_d_n0 * s.v[183]);let eq11_e1621_d_n1: f64 = (eq11_e1619_d_n1 * s.v[183]);
        let eq11_e1621_d_n2: f64 = (eq11_e1619_d_n2 * s.v[183]);let eq11_e1621_d_n3: f64 = (eq11_e1619_d_n3 * s.v[183]);let eq11_e1621_d_n4: f64 = (eq11_e1619_d_n4 * s.v[183]);let eq11_e1621_d_n5: f64 = (eq11_e1619_d_n5 * s.v[183]);let eq11_e1621_d_n6: f64 = (eq11_e1619_d_n6 * s.v[183]);let eq11_e1621_d_n7: f64 = (eq11_e1619_d_n7 * s.v[183]);let eq11_e1621_d_n8: f64 = (eq11_e1619_d_n8 * s.v[183]);let eq11_e1621_d_n9: f64 = (eq11_e1619_d_n9 * s.v[183]);let eq11_e1621_d_n10: f64 = (eq11_e1619_d_n10 * s.v[183]);let eq11_e1621_d_n11: f64 = (eq11_e1619_d_n11 * s.v[183]);let eq11_e1621_d_n12: f64 = (eq11_e1619_d_n12 * s.v[183]);let eq11_e1621_d_n13: f64 = (eq11_e1619_d_n13 * s.v[183]);let eq11_e1621_d_b0: f64 = (eq11_e1619_d_b0 * s.v[183]);let eq11_e1621_d_b1: f64 = (eq11_e1619_d_b1 * s.v[183]);let eq11_e1621_d_b2: f64 = (eq11_e1619_d_b2 * s.v[183]);let eq11_e1621_d_b3: f64 = (eq11_e1619_d_b3 * s.v[183]);let eq11_e1621_d_b4: f64 = (eq11_e1619_d_b4 * s.v[183]);let eq11_e1621_d_b5: f64 = (eq11_e1619_d_b5 * s.v[183]);let eq11_e1621_d_b6: f64 = (eq11_e1619_d_b6 * s.v[183]);let eq11_e1621_d_b7: f64 = (eq11_e1619_d_b7 * s.v[183]);let eq11_e1621_d_b8: f64 = (eq11_e1619_d_b8 * s.v[183]);let eq11_e1621_d_b9: f64 = (eq11_e1619_d_b9 * s.v[183]);let eq11_e1621_d_b10: f64 = (eq11_e1619_d_b10 * s.v[183]);let eq11_e1621_d_b11: f64 = (eq11_e1619_d_b11 * s.v[183]);let eq11_e1623: f64 = (eq11_e1621 * p[2]);let eq11_e1623_d_n0: f64 = (eq11_e1621_d_n0 * p[2]);let eq11_e1623_d_n1: f64 = (eq11_e1621_d_n1 * p[2]);let eq11_e1623_d_n2: f64 = (eq11_e1621_d_n2 * p[2]);let eq11_e1623_d_n3: f64 = (eq11_e1621_d_n3 * p[2]);let eq11_e1623_d_n4: f64 = (eq11_e1621_d_n4 * p[2]);let eq11_e1623_d_n5: f64 = (eq11_e1621_d_n5 * p[2]);let eq11_e1623_d_n6: f64 = (eq11_e1621_d_n6 * p[2]);let eq11_e1623_d_n7: f64 = (eq11_e1621_d_n7 * p[2]);let eq11_e1623_d_n8: f64 = (eq11_e1621_d_n8 * p[2]);let eq11_e1623_d_n9: f64 = (eq11_e1621_d_n9 * p[2]);let eq11_e1623_d_n10: f64 = (eq11_e1621_d_n10 * p[2]);let eq11_e1623_d_n11: f64 = (eq11_e1621_d_n11 * p[2]);let eq11_e1623_d_n12: f64 = (eq11_e1621_d_n12 * p[2]);let eq11_e1623_d_n13: f64 = (eq11_e1621_d_n13 * p[2]);let eq11_e1623_d_b0: f64 = (eq11_e1621_d_b0 * p[2]);let eq11_e1623_d_b1: f64 = (eq11_e1621_d_b1 * p[2]);let eq11_e1623_d_b2: f64 = (eq11_e1621_d_b2 * p[2]);let eq11_e1623_d_b3: f64 = (eq11_e1621_d_b3 * p[2]);let eq11_e1623_d_b4: f64 = (eq11_e1621_d_b4 * p[2]);let eq11_e1623_d_b5: f64 = (eq11_e1621_d_b5 * p[2]);let eq11_e1623_d_b6: f64 = (eq11_e1621_d_b6 * p[2]);let eq11_e1623_d_b7: f64 = (eq11_e1621_d_b7 * p[2]);let eq11_e1623_d_b8: f64 = (eq11_e1621_d_b8 * p[2]);let eq11_e1623_d_b9: f64 = (eq11_e1621_d_b9 * p[2]);let eq11_e1623_d_b10: f64 = (eq11_e1621_d_b10 * p[2]);let eq11_e1623_d_b11: f64 = (eq11_e1621_d_b11 * p[2]);let eq11_e1625: f64 = (eq11_e1623 * s.v[184]);let eq11_e1625_d_n0: f64 = (eq11_e1623_d_n0 * s.v[184]);let eq11_e1625_d_n1: f64 = (eq11_e1623_d_n1 * s.v[184]);let eq11_e1625_d_n2: f64 = (eq11_e1623_d_n2 * s.v[184]);let eq11_e1625_d_n3: f64 = (eq11_e1623_d_n3 * s.v[184]);let eq11_e1625_d_n4: f64 = (eq11_e1623_d_n4 * s.v[184]);let eq11_e1625_d_n5: f64 = (eq11_e1623_d_n5 * s.v[184]);let eq11_e1625_d_n6: f64 = (eq11_e1623_d_n6 * s.v[184]);let eq11_e1625_d_n7: f64 = (eq11_e1623_d_n7 * s.v[184]);let eq11_e1625_d_n8: f64 = (eq11_e1623_d_n8 * s.v[184]);let eq11_e1625_d_n9: f64 = (eq11_e1623_d_n9 * s.v[184]);let eq11_e1625_d_n10: f64 = (eq11_e1623_d_n10 * s.v[184]);let eq11_e1625_d_n11: f64 = (eq11_e1623_d_n11 * s.v[184]);let eq11_e1625_d_n12: f64 = (eq11_e1623_d_n12 * s.v[184]);let eq11_e1625_d_n13: f64 = (eq11_e1623_d_n13 * s.v[184]);let eq11_e1625_d_b0: f64 = (eq11_e1623_d_b0 * s.v[184]);let eq11_e1625_d_b1: f64 = (eq11_e1623_d_b1 * s.v[184]);let eq11_e1625_d_b2: f64 = (eq11_e1623_d_b2 * s.v[184]);let eq11_e1625_d_b3: f64 = (eq11_e1623_d_b3 * s.v[184]);let eq11_e1625_d_b4: f64 = (eq11_e1623_d_b4 * s.v[184]);let eq11_e1625_d_b5: f64 = (eq11_e1623_d_b5 * s.v[184]);let eq11_e1625_d_b6: f64 = (eq11_e1623_d_b6 * s.v[184]);let eq11_e1625_d_b7: f64 = (eq11_e1623_d_b7 * s.v[184]);
        let eq11_e1625_d_b8: f64 = (eq11_e1623_d_b8 * s.v[184]);let eq11_e1625_d_b9: f64 = (eq11_e1623_d_b9 * s.v[184]);let eq11_e1625_d_b10: f64 = (eq11_e1623_d_b10 * s.v[184]);let eq11_e1625_d_b11: f64 = (eq11_e1623_d_b11 * s.v[184]);let eq11_e1627: f64 = (eq11_e1625 * (nv12 - 0.0));let eq11_e1627_d_n0: f64 = (eq11_e1625_d_n0 * (nv12 - 0.0));let eq11_e1627_d_n1: f64 = (eq11_e1625_d_n1 * (nv12 - 0.0));let eq11_e1627_d_n2: f64 = (eq11_e1625_d_n2 * (nv12 - 0.0));let eq11_e1627_d_n3: f64 = (eq11_e1625_d_n3 * (nv12 - 0.0));let eq11_e1627_d_n4: f64 = (eq11_e1625_d_n4 * (nv12 - 0.0));let eq11_e1627_d_n5: f64 = (eq11_e1625_d_n5 * (nv12 - 0.0));let eq11_e1627_d_n6: f64 = (eq11_e1625_d_n6 * (nv12 - 0.0));let eq11_e1627_d_n7: f64 = (eq11_e1625_d_n7 * (nv12 - 0.0));let eq11_e1627_d_n8: f64 = (eq11_e1625_d_n8 * (nv12 - 0.0));let eq11_e1627_d_n9: f64 = (eq11_e1625_d_n9 * (nv12 - 0.0));let eq11_e1627_d_n10: f64 = (eq11_e1625_d_n10 * (nv12 - 0.0));let eq11_e1627_d_n11: f64 = (eq11_e1625_d_n11 * (nv12 - 0.0));let eq11_e1627_d_n12: f64 = ((eq11_e1625_d_n12 * (nv12 - 0.0)) + eq11_e1625);let eq11_e1627_d_n13: f64 = (eq11_e1625_d_n13 * (nv12 - 0.0));let eq11_e1627_d_b0: f64 = (eq11_e1625_d_b0 * (nv12 - 0.0));let eq11_e1627_d_b1: f64 = (eq11_e1625_d_b1 * (nv12 - 0.0));let eq11_e1627_d_b2: f64 = (eq11_e1625_d_b2 * (nv12 - 0.0));let eq11_e1627_d_b3: f64 = (eq11_e1625_d_b3 * (nv12 - 0.0));let eq11_e1627_d_b4: f64 = (eq11_e1625_d_b4 * (nv12 - 0.0));let eq11_e1627_d_b5: f64 = (eq11_e1625_d_b5 * (nv12 - 0.0));let eq11_e1627_d_b6: f64 = (eq11_e1625_d_b6 * (nv12 - 0.0));let eq11_e1627_d_b7: f64 = (eq11_e1625_d_b7 * (nv12 - 0.0));let eq11_e1627_d_b8: f64 = (eq11_e1625_d_b8 * (nv12 - 0.0));let eq11_e1627_d_b9: f64 = (eq11_e1625_d_b9 * (nv12 - 0.0));let eq11_e1627_d_b10: f64 = (eq11_e1625_d_b10 * (nv12 - 0.0));let eq11_e1627_d_b11: f64 = (eq11_e1625_d_b11 * (nv12 - 0.0));let eq11_e1628: f64 = (0.5 * eq11_e1627);let eq11_e1628_d_n0: f64 = (0.5 * eq11_e1627_d_n0);let eq11_e1628_d_n1: f64 = (0.5 * eq11_e1627_d_n1);let eq11_e1628_d_n2: f64 = (0.5 * eq11_e1627_d_n2);let eq11_e1628_d_n3: f64 = (0.5 * eq11_e1627_d_n3);let eq11_e1628_d_n4: f64 = (0.5 * eq11_e1627_d_n4);let eq11_e1628_d_n5: f64 = (0.5 * eq11_e1627_d_n5);let eq11_e1628_d_n6: f64 = (0.5 * eq11_e1627_d_n6);let eq11_e1628_d_n7: f64 = (0.5 * eq11_e1627_d_n7);let eq11_e1628_d_n8: f64 = (0.5 * eq11_e1627_d_n8);let eq11_e1628_d_n9: f64 = (0.5 * eq11_e1627_d_n9);let eq11_e1628_d_n10: f64 = (0.5 * eq11_e1627_d_n10);let eq11_e1628_d_n11: f64 = (0.5 * eq11_e1627_d_n11);let eq11_e1628_d_n12: f64 = (0.5 * eq11_e1627_d_n12);let eq11_e1628_d_n13: f64 = (0.5 * eq11_e1627_d_n13);let eq11_e1628_d_b0: f64 = (0.5 * eq11_e1627_d_b0);let eq11_e1628_d_b1: f64 = (0.5 * eq11_e1627_d_b1);let eq11_e1628_d_b2: f64 = (0.5 * eq11_e1627_d_b2);let eq11_e1628_d_b3: f64 = (0.5 * eq11_e1627_d_b3);let eq11_e1628_d_b4: f64 = (0.5 * eq11_e1627_d_b4);let eq11_e1628_d_b5: f64 = (0.5 * eq11_e1627_d_b5);let eq11_e1628_d_b6: f64 = (0.5 * eq11_e1627_d_b6);let eq11_e1628_d_b7: f64 = (0.5 * eq11_e1627_d_b7);let eq11_e1628_d_b8: f64 = (0.5 * eq11_e1627_d_b8);let eq11_e1628_d_b9: f64 = (0.5 * eq11_e1627_d_b9);let eq11_e1628_d_b10: f64 = (0.5 * eq11_e1627_d_b10);let eq11_e1628_d_b11: f64 = (0.5 * eq11_e1627_d_b11);let eq11_e1629_q: f64 = eq11_e1628;
        (eq11_e1628, eq11_e1628_d_n0, eq11_e1628_d_n1, eq11_e1628_d_n2, eq11_e1628_d_n3, eq11_e1628_d_n4, eq11_e1628_d_n5, eq11_e1628_d_n6, eq11_e1628_d_n7, eq11_e1628_d_n8, eq11_e1628_d_n9, eq11_e1628_d_n10, eq11_e1628_d_n11, eq11_e1628_d_n12, eq11_e1628_d_n13, eq11_e1628_d_b0, eq11_e1628_d_b1, eq11_e1628_d_b2, eq11_e1628_d_b3, eq11_e1628_d_b4, eq11_e1628_d_b5, eq11_e1628_d_b6, eq11_e1628_d_b7, eq11_e1628_d_b8, eq11_e1628_d_b9, eq11_e1628_d_b10, eq11_e1628_d_b11, eq11_e1629_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [eq11_e1631_d_n0, eq11_e1631_d_n1, eq11_e1631_d_n2, eq11_e1631_d_n3, eq11_e1631_d_n4, eq11_e1631_d_n5, eq11_e1631_d_n6, eq11_e1631_d_n7, eq11_e1631_d_n8, eq11_e1631_d_n9, eq11_e1631_d_n10, eq11_e1631_d_n11, eq11_e1631_d_n12, eq11_e1631_d_n13];let eq11_reactive_branch_derivatives: [f64; 12] = [eq11_e1631_d_b0, eq11_e1631_d_b1, eq11_e1631_d_b2, eq11_e1631_d_b3, eq11_e1631_d_b4, eq11_e1631_d_b5, eq11_e1631_d_b6, eq11_e1631_d_b7, eq11_e1631_d_b8, eq11_e1631_d_b9, eq11_e1631_d_b10, eq11_e1631_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq11_reactive_node_derivatives,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
