#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv25 = ctx.node_voltage(nodes[25]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let eq19_ad_e636: A = {
    if ((!s.b[308]) && s.b[309]) {
        let eq19_ad_e631: A = {
            if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (!(((nv26 - nv27) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), s.ad_value(113)))
            } else {
                let eq19_ad_e630: A = {
                    if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (((nv26 - nv27) / s.v[113]) < (-50.0))) {
                        A::exp_scaled_input(A::constant(50.0), -1.0)
                    } else {
                        {
                            if (((nv26 - nv27) / s.v[113]) > 50.0) {
                                A::scale(A::offset(A::offset(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), s.ad_value(113)), (-50.0)), 1.0), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                };
                eq19_ad_e630
            }
        };
        A::scale(A::offset(eq19_ad_e631, (-1.0)), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq19_ad: A = eq19_ad_e636;
        stamper.stamp_current_dense_local(
            Some(26),
            Some(27),
            multiplicity * eq19_ad.value,
            &eq19_ad.dn,
            &eq19_ad.db,
            multiplicity,
        );
        let (eq20_e645, eq20_e645_d_n25, eq20_e645_d_n27,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq20_e643: f64 = ((nv25 - nv27) / p.p340);
        let eq20_e643_d_n25: f64 = (1.0 / p.p340);
        let eq20_e643_d_n27: f64 = (-1.0 / p.p340);
        (eq20_e643, eq20_e643_d_n25, eq20_e643_d_n27,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e645;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(27),
            multiplicity * (eq20_value),
            25,
            multiplicity * (eq20_e645_d_n25),
            27,
            multiplicity * (eq20_e645_d_n27),
        );
        let (eq21_e654, eq21_e654_d_n25, eq21_e654_d_n26,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq21_e652: f64 = ((nv25 - nv26) / p.p339);
        let eq21_e652_d_n25: f64 = (1.0 / p.p339);
        let eq21_e652_d_n26: f64 = (-1.0 / p.p339);
        (eq21_e652, eq21_e652_d_n25, eq21_e652_d_n26,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e654;
        stamper.stamp_current_node2_local(
            Some(25),
            Some(26),
            multiplicity * (eq21_value),
            25,
            multiplicity * (eq21_e654_d_n25),
            26,
            multiplicity * (eq21_e654_d_n26),
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq22_e682, eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29, eq22_e682_d_b0, eq22_e682_d_b1, eq22_e682_d_b2, eq22_e682_d_b3, eq22_e682_d_b4, eq22_e682_d_b5, eq22_e682_d_b6, eq22_e682_d_b7, eq22_e682_d_b8, eq22_e682_d_b9, eq22_e682_d_b10, eq22_e682_d_b11, eq22_e682_d_b12, eq22_e682_d_b13, eq22_e682_d_b14, eq22_e682_d_b15, eq22_e682_d_b16, eq22_e682_d_b17, eq22_e682_d_b18, eq22_e682_d_b19, eq22_e682_d_b20, eq22_e682_d_b21, eq22_e682_d_b22, eq22_e682_d_b23, eq22_e682_d_b24, eq22_e682_d_b25, eq22_e682_d_b26, eq22_e682_d_b27, eq22_e682_d_b28, eq22_e682_d_b29, eq22_e682_d_b30, eq22_e682_d_b31, eq22_e682_d_b32, eq22_e682_d_b33, eq22_e682_d_b34, eq22_e682_d_b35,) = {
    if ((!s.b[308]) && s.b[309]) {
        let eq22_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[227]);
        let eq22_e661_d_n0: f64 = (s.dn[227][0] * ddt_scale);
        let eq22_e661_d_n1: f64 = (s.dn[227][1] * ddt_scale);
        let eq22_e661_d_n2: f64 = (s.dn[227][2] * ddt_scale);
        let eq22_e661_d_n3: f64 = (s.dn[227][3] * ddt_scale);
        let eq22_e661_d_n4: f64 = (s.dn[227][4] * ddt_scale);
        let eq22_e661_d_n5: f64 = (s.dn[227][5] * ddt_scale);
        let eq22_e661_d_n6: f64 = (s.dn[227][6] * ddt_scale);
        let eq22_e661_d_n7: f64 = (s.dn[227][7] * ddt_scale);
        let eq22_e661_d_n8: f64 = (s.dn[227][8] * ddt_scale);
        let eq22_e661_d_n9: f64 = (s.dn[227][9] * ddt_scale);
        let eq22_e661_d_n10: f64 = (s.dn[227][10] * ddt_scale);
        let eq22_e661_d_n11: f64 = (s.dn[227][11] * ddt_scale);
        let eq22_e661_d_n12: f64 = (s.dn[227][12] * ddt_scale);
        let eq22_e661_d_n13: f64 = (s.dn[227][13] * ddt_scale);
        let eq22_e661_d_n14: f64 = (s.dn[227][14] * ddt_scale);
        let eq22_e661_d_n15: f64 = (s.dn[227][15] * ddt_scale);
        let eq22_e661_d_n16: f64 = (s.dn[227][16] * ddt_scale);
        let eq22_e661_d_n17: f64 = (s.dn[227][17] * ddt_scale);
        let eq22_e661_d_n18: f64 = (s.dn[227][18] * ddt_scale);
        let eq22_e661_d_n19: f64 = (s.dn[227][19] * ddt_scale);
        let eq22_e661_d_n20: f64 = (s.dn[227][20] * ddt_scale);
        let eq22_e661_d_n21: f64 = (s.dn[227][21] * ddt_scale);
        let eq22_e661_d_n22: f64 = (s.dn[227][22] * ddt_scale);
        let eq22_e661_d_n23: f64 = (s.dn[227][23] * ddt_scale);
        let eq22_e661_d_n24: f64 = (s.dn[227][24] * ddt_scale);
        let eq22_e661_d_n25: f64 = (s.dn[227][25] * ddt_scale);
        let eq22_e661_d_n26: f64 = (s.dn[227][26] * ddt_scale);
        let eq22_e661_d_n27: f64 = (s.dn[227][27] * ddt_scale);
        let eq22_e661_d_n28: f64 = (s.dn[227][28] * ddt_scale);
        let eq22_e661_d_n29: f64 = (s.dn[227][29] * ddt_scale);
        let eq22_e661_d_b0: f64 = (s.db[227][0] * ddt_scale);
        let eq22_e661_d_b1: f64 = (s.db[227][1] * ddt_scale);
        let eq22_e661_d_b2: f64 = (s.db[227][2] * ddt_scale);
        let eq22_e661_d_b3: f64 = (s.db[227][3] * ddt_scale);
        let eq22_e661_d_b4: f64 = (s.db[227][4] * ddt_scale);
        let eq22_e661_d_b5: f64 = (s.db[227][5] * ddt_scale);
        let eq22_e661_d_b6: f64 = (s.db[227][6] * ddt_scale);
        let eq22_e661_d_b7: f64 = (s.db[227][7] * ddt_scale);
        let eq22_e661_d_b8: f64 = (s.db[227][8] * ddt_scale);
        let eq22_e661_d_b9: f64 = (s.db[227][9] * ddt_scale);
        let eq22_e661_d_b10: f64 = (s.db[227][10] * ddt_scale);
        let eq22_e661_d_b11: f64 = (s.db[227][11] * ddt_scale);
        let eq22_e661_d_b12: f64 = (s.db[227][12] * ddt_scale);
        let eq22_e661_d_b13: f64 = (s.db[227][13] * ddt_scale);
        let eq22_e661_d_b14: f64 = (s.db[227][14] * ddt_scale);
        let eq22_e661_d_b15: f64 = (s.db[227][15] * ddt_scale);
        let eq22_e661_d_b16: f64 = (s.db[227][16] * ddt_scale);
        let eq22_e661_d_b17: f64 = (s.db[227][17] * ddt_scale);
        let eq22_e661_d_b18: f64 = (s.db[227][18] * ddt_scale);
        let eq22_e661_d_b19: f64 = (s.db[227][19] * ddt_scale);
        let eq22_e661_d_b20: f64 = (s.db[227][20] * ddt_scale);
        let eq22_e661_d_b21: f64 = (s.db[227][21] * ddt_scale);
        let eq22_e661_d_b22: f64 = (s.db[227][22] * ddt_scale);
        let eq22_e661_d_b23: f64 = (s.db[227][23] * ddt_scale);
        let eq22_e661_d_b24: f64 = (s.db[227][24] * ddt_scale);
        let eq22_e661_d_b25: f64 = (s.db[227][25] * ddt_scale);
        let eq22_e661_d_b26: f64 = (s.db[227][26] * ddt_scale);
        let eq22_e661_d_b27: f64 = (s.db[227][27] * ddt_scale);
        let eq22_e661_d_b28: f64 = (s.db[227][28] * ddt_scale);
        let eq22_e661_d_b29: f64 = (s.db[227][29] * ddt_scale);
        let eq22_e661_d_b30: f64 = (s.db[227][30] * ddt_scale);
        let eq22_e661_d_b31: f64 = (s.db[227][31] * ddt_scale);
        let eq22_e661_d_b32: f64 = (s.db[227][32] * ddt_scale);
        let eq22_e661_d_b33: f64 = (s.db[227][33] * ddt_scale);
        let eq22_e661_d_b34: f64 = (s.db[227][34] * ddt_scale);
        let eq22_e661_d_b35: f64 = (s.db[227][35] * ddt_scale);
        let eq22_e662: f64 = (p.p341 * eq22_e661);
        let eq22_e662_d_n0: f64 = (p.p341 * eq22_e661_d_n0);
        let eq22_e662_d_n1: f64 = (p.p341 * eq22_e661_d_n1);
        let eq22_e662_d_n2: f64 = (p.p341 * eq22_e661_d_n2);
        let eq22_e662_d_n3: f64 = (p.p341 * eq22_e661_d_n3);
        let eq22_e662_d_n4: f64 = (p.p341 * eq22_e661_d_n4);
        let eq22_e662_d_n5: f64 = (p.p341 * eq22_e661_d_n5);
        let eq22_e662_d_n6: f64 = (p.p341 * eq22_e661_d_n6);
        let eq22_e662_d_n7: f64 = (p.p341 * eq22_e661_d_n7);
        let eq22_e662_d_n8: f64 = (p.p341 * eq22_e661_d_n8);
        let eq22_e662_d_n9: f64 = (p.p341 * eq22_e661_d_n9);
        let eq22_e662_d_n10: f64 = (p.p341 * eq22_e661_d_n10);
        let eq22_e662_d_n11: f64 = (p.p341 * eq22_e661_d_n11);
        let eq22_e662_d_n12: f64 = (p.p341 * eq22_e661_d_n12);
        let eq22_e662_d_n13: f64 = (p.p341 * eq22_e661_d_n13);
        let eq22_e662_d_n14: f64 = (p.p341 * eq22_e661_d_n14);
        let eq22_e662_d_n15: f64 = (p.p341 * eq22_e661_d_n15);
        let eq22_e662_d_n16: f64 = (p.p341 * eq22_e661_d_n16);
        let eq22_e662_d_n17: f64 = (p.p341 * eq22_e661_d_n17);
        let eq22_e662_d_n18: f64 = (p.p341 * eq22_e661_d_n18);
        let eq22_e662_d_n19: f64 = (p.p341 * eq22_e661_d_n19);
        let eq22_e662_d_n20: f64 = (p.p341 * eq22_e661_d_n20);
        let eq22_e662_d_n21: f64 = (p.p341 * eq22_e661_d_n21);
        let eq22_e662_d_n22: f64 = (p.p341 * eq22_e661_d_n22);
        let eq22_e662_d_n23: f64 = (p.p341 * eq22_e661_d_n23);
        let eq22_e662_d_n24: f64 = (p.p341 * eq22_e661_d_n24);
        let eq22_e662_d_n25: f64 = (p.p341 * eq22_e661_d_n25);
        let eq22_e662_d_n26: f64 = (p.p341 * eq22_e661_d_n26);
        let eq22_e662_d_n27: f64 = (p.p341 * eq22_e661_d_n27);
        let eq22_e662_d_n28: f64 = (p.p341 * eq22_e661_d_n28);
        let eq22_e662_d_n29: f64 = (p.p341 * eq22_e661_d_n29);
        let eq22_e662_d_b0: f64 = (p.p341 * eq22_e661_d_b0);
        let eq22_e662_d_b1: f64 = (p.p341 * eq22_e661_d_b1);
        let eq22_e662_d_b2: f64 = (p.p341 * eq22_e661_d_b2);
        let eq22_e662_d_b3: f64 = (p.p341 * eq22_e661_d_b3);
        let eq22_e662_d_b4: f64 = (p.p341 * eq22_e661_d_b4);
        let eq22_e662_d_b5: f64 = (p.p341 * eq22_e661_d_b5);
        let eq22_e662_d_b6: f64 = (p.p341 * eq22_e661_d_b6);
        let eq22_e662_d_b7: f64 = (p.p341 * eq22_e661_d_b7);
        let eq22_e662_d_b8: f64 = (p.p341 * eq22_e661_d_b8);
        let eq22_e662_d_b9: f64 = (p.p341 * eq22_e661_d_b9);
        let eq22_e662_d_b10: f64 = (p.p341 * eq22_e661_d_b10);
        let eq22_e662_d_b11: f64 = (p.p341 * eq22_e661_d_b11);
        let eq22_e662_d_b12: f64 = (p.p341 * eq22_e661_d_b12);
        let eq22_e662_d_b13: f64 = (p.p341 * eq22_e661_d_b13);
        let eq22_e662_d_b14: f64 = (p.p341 * eq22_e661_d_b14);
        let eq22_e662_d_b15: f64 = (p.p341 * eq22_e661_d_b15);
        let eq22_e662_d_b16: f64 = (p.p341 * eq22_e661_d_b16);
        let eq22_e662_d_b17: f64 = (p.p341 * eq22_e661_d_b17);
        let eq22_e662_d_b18: f64 = (p.p341 * eq22_e661_d_b18);
        let eq22_e662_d_b19: f64 = (p.p341 * eq22_e661_d_b19);
        let eq22_e662_d_b20: f64 = (p.p341 * eq22_e661_d_b20);
        let eq22_e662_d_b21: f64 = (p.p341 * eq22_e661_d_b21);
        let eq22_e662_d_b22: f64 = (p.p341 * eq22_e661_d_b22);
        let eq22_e662_d_b23: f64 = (p.p341 * eq22_e661_d_b23);
        let eq22_e662_d_b24: f64 = (p.p341 * eq22_e661_d_b24);
        let eq22_e662_d_b25: f64 = (p.p341 * eq22_e661_d_b25);
        let eq22_e662_d_b26: f64 = (p.p341 * eq22_e661_d_b26);
        let eq22_e662_d_b27: f64 = (p.p341 * eq22_e661_d_b27);
        let eq22_e662_d_b28: f64 = (p.p341 * eq22_e661_d_b28);
        let eq22_e662_d_b29: f64 = (p.p341 * eq22_e661_d_b29);
        let eq22_e662_d_b30: f64 = (p.p341 * eq22_e661_d_b30);
        let eq22_e662_d_b31: f64 = (p.p341 * eq22_e661_d_b31);
        let eq22_e662_d_b32: f64 = (p.p341 * eq22_e661_d_b32);
        let eq22_e662_d_b33: f64 = (p.p341 * eq22_e661_d_b33);
        let eq22_e662_d_b34: f64 = (p.p341 * eq22_e661_d_b34);
        let eq22_e662_d_b35: f64 = (p.p341 * eq22_e661_d_b35);
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
        (eq22_e680, eq22_e680_d_n0, eq22_e680_d_n1, eq22_e680_d_n2, eq22_e680_d_n3, eq22_e680_d_n4, eq22_e680_d_n5, eq22_e680_d_n6, eq22_e680_d_n7, eq22_e680_d_n8, eq22_e680_d_n9, eq22_e680_d_n10, eq22_e680_d_n11, eq22_e680_d_n12, eq22_e680_d_n13, eq22_e680_d_n14, eq22_e680_d_n15, eq22_e680_d_n16, eq22_e680_d_n17, eq22_e680_d_n18, eq22_e680_d_n19, eq22_e680_d_n20, eq22_e680_d_n21, eq22_e680_d_n22, eq22_e680_d_n23, eq22_e680_d_n24, eq22_e680_d_n25, eq22_e680_d_n26, eq22_e680_d_n27, eq22_e680_d_n28, eq22_e680_d_n29, eq22_e680_d_b0, eq22_e680_d_b1, eq22_e680_d_b2, eq22_e680_d_b3, eq22_e680_d_b4, eq22_e680_d_b5, eq22_e680_d_b6, eq22_e680_d_b7, eq22_e680_d_b8, eq22_e680_d_b9, eq22_e680_d_b10, eq22_e680_d_b11, eq22_e680_d_b12, eq22_e680_d_b13, eq22_e680_d_b14, eq22_e680_d_b15, eq22_e680_d_b16, eq22_e680_d_b17, eq22_e680_d_b18, eq22_e680_d_b19, eq22_e680_d_b20, eq22_e680_d_b21, eq22_e680_d_b22, eq22_e680_d_b23, eq22_e680_d_b24, eq22_e680_d_b25, eq22_e680_d_b26, eq22_e680_d_b27, eq22_e680_d_b28, eq22_e680_d_b29, eq22_e680_d_b30, eq22_e680_d_b31, eq22_e680_d_b32, eq22_e680_d_b33, eq22_e680_d_b34, eq22_e680_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e682;
        let eq22_node_derivatives: [f64; 30] = [eq22_e682_d_n0, eq22_e682_d_n1, eq22_e682_d_n2, eq22_e682_d_n3, eq22_e682_d_n4, eq22_e682_d_n5, eq22_e682_d_n6, eq22_e682_d_n7, eq22_e682_d_n8, eq22_e682_d_n9, eq22_e682_d_n10, eq22_e682_d_n11, eq22_e682_d_n12, eq22_e682_d_n13, eq22_e682_d_n14, eq22_e682_d_n15, eq22_e682_d_n16, eq22_e682_d_n17, eq22_e682_d_n18, eq22_e682_d_n19, eq22_e682_d_n20, eq22_e682_d_n21, eq22_e682_d_n22, eq22_e682_d_n23, eq22_e682_d_n24, eq22_e682_d_n25, eq22_e682_d_n26, eq22_e682_d_n27, eq22_e682_d_n28, eq22_e682_d_n29];
        let eq22_branch_derivatives: [f64; 36] = [eq22_e682_d_b0, eq22_e682_d_b1, eq22_e682_d_b2, eq22_e682_d_b3, eq22_e682_d_b4, eq22_e682_d_b5, eq22_e682_d_b6, eq22_e682_d_b7, eq22_e682_d_b8, eq22_e682_d_b9, eq22_e682_d_b10, eq22_e682_d_b11, eq22_e682_d_b12, eq22_e682_d_b13, eq22_e682_d_b14, eq22_e682_d_b15, eq22_e682_d_b16, eq22_e682_d_b17, eq22_e682_d_b18, eq22_e682_d_b19, eq22_e682_d_b20, eq22_e682_d_b21, eq22_e682_d_b22, eq22_e682_d_b23, eq22_e682_d_b24, eq22_e682_d_b25, eq22_e682_d_b26, eq22_e682_d_b27, eq22_e682_d_b28, eq22_e682_d_b29, eq22_e682_d_b30, eq22_e682_d_b31, eq22_e682_d_b32, eq22_e682_d_b33, eq22_e682_d_b34, eq22_e682_d_b35];
        stamper.stamp_current_dense_local(
            Some(26),
            None,
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e690,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e690;
        stamper.stamp_potential_const_local(
            10,
            eq23_value,
        );
    }

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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq24_e698,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e698;
        stamper.stamp_potential_const_local(
            11,
            eq24_value,
        );
        let (eq25_e706,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e706;
        stamper.stamp_potential_const_local(
            12,
            eq25_value,
        );
        let (eq26_e714,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e714;
        stamper.stamp_potential_const_local(
            13,
            eq26_value,
        );
        let (eq27_e722,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e722;
        stamper.stamp_potential_const_local(
            14,
            eq27_value,
        );
        let (eq28_e730,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e730;
        stamper.stamp_potential_const_local(
            15,
            eq28_value,
        );
        let (eq29_e738,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e738;
        stamper.stamp_potential_const_local(
            16,
            eq29_value,
        );
        let (eq30_e746,) = {
    if ((!s.b[308]) && (!s.b[309])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e746;
        stamper.stamp_potential_const_local(
            17,
            eq30_value,
        );
        let (eq31_e754, eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29, eq31_e754_d_b0, eq31_e754_d_b1, eq31_e754_d_b2, eq31_e754_d_b3, eq31_e754_d_b4, eq31_e754_d_b5, eq31_e754_d_b6, eq31_e754_d_b7, eq31_e754_d_b8, eq31_e754_d_b9, eq31_e754_d_b10, eq31_e754_d_b11, eq31_e754_d_b12, eq31_e754_d_b13, eq31_e754_d_b14, eq31_e754_d_b15, eq31_e754_d_b16, eq31_e754_d_b17, eq31_e754_d_b18, eq31_e754_d_b19, eq31_e754_d_b20, eq31_e754_d_b21, eq31_e754_d_b22, eq31_e754_d_b23, eq31_e754_d_b24, eq31_e754_d_b25, eq31_e754_d_b26, eq31_e754_d_b27, eq31_e754_d_b28, eq31_e754_d_b29, eq31_e754_d_b30, eq31_e754_d_b31, eq31_e754_d_b32, eq31_e754_d_b33, eq31_e754_d_b34, eq31_e754_d_b35,) = {
    if s.b[320] {
        let eq31_e751: f64 = (s.v[0] * (nv17 - nv16));
        let eq31_e751_d_n16: f64 = (-s.v[0]);
        let eq31_e751_d_n17: f64 = s.v[0];
        let eq31_e752: f64 = (s.v[208] + eq31_e751);
        let eq31_e752_d_n16: f64 = (s.dn[208][16] + eq31_e751_d_n16);
        let eq31_e752_d_n17: f64 = (s.dn[208][17] + eq31_e751_d_n17);
        (eq31_e752, s.dn[208][0], s.dn[208][1], s.dn[208][2], s.dn[208][3], s.dn[208][4], s.dn[208][5], s.dn[208][6], s.dn[208][7], s.dn[208][8], s.dn[208][9], s.dn[208][10], s.dn[208][11], s.dn[208][12], s.dn[208][13], s.dn[208][14], s.dn[208][15], eq31_e752_d_n16, eq31_e752_d_n17, s.dn[208][18], s.dn[208][19], s.dn[208][20], s.dn[208][21], s.dn[208][22], s.dn[208][23], s.dn[208][24], s.dn[208][25], s.dn[208][26], s.dn[208][27], s.dn[208][28], s.dn[208][29], s.db[208][0], s.db[208][1], s.db[208][2], s.db[208][3], s.db[208][4], s.db[208][5], s.db[208][6], s.db[208][7], s.db[208][8], s.db[208][9], s.db[208][10], s.db[208][11], s.db[208][12], s.db[208][13], s.db[208][14], s.db[208][15], s.db[208][16], s.db[208][17], s.db[208][18], s.db[208][19], s.db[208][20], s.db[208][21], s.db[208][22], s.db[208][23], s.db[208][24], s.db[208][25], s.db[208][26], s.db[208][27], s.db[208][28], s.db[208][29], s.db[208][30], s.db[208][31], s.db[208][32], s.db[208][33], s.db[208][34], s.db[208][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e754;
        let eq31_node_derivatives: [f64; 30] = [eq31_e754_d_n0, eq31_e754_d_n1, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n5, eq31_e754_d_n6, eq31_e754_d_n7, eq31_e754_d_n8, eq31_e754_d_n9, eq31_e754_d_n10, eq31_e754_d_n11, eq31_e754_d_n12, eq31_e754_d_n13, eq31_e754_d_n14, eq31_e754_d_n15, eq31_e754_d_n16, eq31_e754_d_n17, eq31_e754_d_n18, eq31_e754_d_n19, eq31_e754_d_n20, eq31_e754_d_n21, eq31_e754_d_n22, eq31_e754_d_n23, eq31_e754_d_n24, eq31_e754_d_n25, eq31_e754_d_n26, eq31_e754_d_n27, eq31_e754_d_n28, eq31_e754_d_n29];
        let eq31_branch_derivatives: [f64; 36] = [eq31_e754_d_b0, eq31_e754_d_b1, eq31_e754_d_b2, eq31_e754_d_b3, eq31_e754_d_b4, eq31_e754_d_b5, eq31_e754_d_b6, eq31_e754_d_b7, eq31_e754_d_b8, eq31_e754_d_b9, eq31_e754_d_b10, eq31_e754_d_b11, eq31_e754_d_b12, eq31_e754_d_b13, eq31_e754_d_b14, eq31_e754_d_b15, eq31_e754_d_b16, eq31_e754_d_b17, eq31_e754_d_b18, eq31_e754_d_b19, eq31_e754_d_b20, eq31_e754_d_b21, eq31_e754_d_b22, eq31_e754_d_b23, eq31_e754_d_b24, eq31_e754_d_b25, eq31_e754_d_b26, eq31_e754_d_b27, eq31_e754_d_b28, eq31_e754_d_b29, eq31_e754_d_b30, eq31_e754_d_b31, eq31_e754_d_b32, eq31_e754_d_b33, eq31_e754_d_b34, eq31_e754_d_b35];
        stamper.stamp_current_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e759,) = {
    if (!s.b[320]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e759;
        stamper.stamp_potential_const_local(
            18,
            eq32_value,
        );
        let (eq33_e769, eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29, eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35,) = {
    if s.b[466] {
        let eq33_e762: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[209]);
        let eq33_e762_d_n0: f64 = (s.dn[209][0] * ddt_scale);
        let eq33_e762_d_n1: f64 = (s.dn[209][1] * ddt_scale);
        let eq33_e762_d_n2: f64 = (s.dn[209][2] * ddt_scale);
        let eq33_e762_d_n3: f64 = (s.dn[209][3] * ddt_scale);
        let eq33_e762_d_n4: f64 = (s.dn[209][4] * ddt_scale);
        let eq33_e762_d_n5: f64 = (s.dn[209][5] * ddt_scale);
        let eq33_e762_d_n6: f64 = (s.dn[209][6] * ddt_scale);
        let eq33_e762_d_n7: f64 = (s.dn[209][7] * ddt_scale);
        let eq33_e762_d_n8: f64 = (s.dn[209][8] * ddt_scale);
        let eq33_e762_d_n9: f64 = (s.dn[209][9] * ddt_scale);
        let eq33_e762_d_n10: f64 = (s.dn[209][10] * ddt_scale);
        let eq33_e762_d_n11: f64 = (s.dn[209][11] * ddt_scale);
        let eq33_e762_d_n12: f64 = (s.dn[209][12] * ddt_scale);
        let eq33_e762_d_n13: f64 = (s.dn[209][13] * ddt_scale);
        let eq33_e762_d_n14: f64 = (s.dn[209][14] * ddt_scale);
        let eq33_e762_d_n15: f64 = (s.dn[209][15] * ddt_scale);
        let eq33_e762_d_n16: f64 = (s.dn[209][16] * ddt_scale);
        let eq33_e762_d_n17: f64 = (s.dn[209][17] * ddt_scale);
        let eq33_e762_d_n18: f64 = (s.dn[209][18] * ddt_scale);
        let eq33_e762_d_n19: f64 = (s.dn[209][19] * ddt_scale);
        let eq33_e762_d_n20: f64 = (s.dn[209][20] * ddt_scale);
        let eq33_e762_d_n21: f64 = (s.dn[209][21] * ddt_scale);
        let eq33_e762_d_n22: f64 = (s.dn[209][22] * ddt_scale);
        let eq33_e762_d_n23: f64 = (s.dn[209][23] * ddt_scale);
        let eq33_e762_d_n24: f64 = (s.dn[209][24] * ddt_scale);
        let eq33_e762_d_n25: f64 = (s.dn[209][25] * ddt_scale);
        let eq33_e762_d_n26: f64 = (s.dn[209][26] * ddt_scale);
        let eq33_e762_d_n27: f64 = (s.dn[209][27] * ddt_scale);
        let eq33_e762_d_n28: f64 = (s.dn[209][28] * ddt_scale);
        let eq33_e762_d_n29: f64 = (s.dn[209][29] * ddt_scale);
        let eq33_e762_d_b0: f64 = (s.db[209][0] * ddt_scale);
        let eq33_e762_d_b1: f64 = (s.db[209][1] * ddt_scale);
        let eq33_e762_d_b2: f64 = (s.db[209][2] * ddt_scale);
        let eq33_e762_d_b3: f64 = (s.db[209][3] * ddt_scale);
        let eq33_e762_d_b4: f64 = (s.db[209][4] * ddt_scale);
        let eq33_e762_d_b5: f64 = (s.db[209][5] * ddt_scale);
        let eq33_e762_d_b6: f64 = (s.db[209][6] * ddt_scale);
        let eq33_e762_d_b7: f64 = (s.db[209][7] * ddt_scale);
        let eq33_e762_d_b8: f64 = (s.db[209][8] * ddt_scale);
        let eq33_e762_d_b9: f64 = (s.db[209][9] * ddt_scale);
        let eq33_e762_d_b10: f64 = (s.db[209][10] * ddt_scale);
        let eq33_e762_d_b11: f64 = (s.db[209][11] * ddt_scale);
        let eq33_e762_d_b12: f64 = (s.db[209][12] * ddt_scale);
        let eq33_e762_d_b13: f64 = (s.db[209][13] * ddt_scale);
        let eq33_e762_d_b14: f64 = (s.db[209][14] * ddt_scale);
        let eq33_e762_d_b15: f64 = (s.db[209][15] * ddt_scale);
        let eq33_e762_d_b16: f64 = (s.db[209][16] * ddt_scale);
        let eq33_e762_d_b17: f64 = (s.db[209][17] * ddt_scale);
        let eq33_e762_d_b18: f64 = (s.db[209][18] * ddt_scale);
        let eq33_e762_d_b19: f64 = (s.db[209][19] * ddt_scale);
        let eq33_e762_d_b20: f64 = (s.db[209][20] * ddt_scale);
        let eq33_e762_d_b21: f64 = (s.db[209][21] * ddt_scale);
        let eq33_e762_d_b22: f64 = (s.db[209][22] * ddt_scale);
        let eq33_e762_d_b23: f64 = (s.db[209][23] * ddt_scale);
        let eq33_e762_d_b24: f64 = (s.db[209][24] * ddt_scale);
        let eq33_e762_d_b25: f64 = (s.db[209][25] * ddt_scale);
        let eq33_e762_d_b26: f64 = (s.db[209][26] * ddt_scale);
        let eq33_e762_d_b27: f64 = (s.db[209][27] * ddt_scale);
        let eq33_e762_d_b28: f64 = (s.db[209][28] * ddt_scale);
        let eq33_e762_d_b29: f64 = (s.db[209][29] * ddt_scale);
        let eq33_e762_d_b30: f64 = (s.db[209][30] * ddt_scale);
        let eq33_e762_d_b31: f64 = (s.db[209][31] * ddt_scale);
        let eq33_e762_d_b32: f64 = (s.db[209][32] * ddt_scale);
        let eq33_e762_d_b33: f64 = (s.db[209][33] * ddt_scale);
        let eq33_e762_d_b34: f64 = (s.db[209][34] * ddt_scale);
        let eq33_e762_d_b35: f64 = (s.db[209][35] * ddt_scale);
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e765_d_n7: f64 = p.p355;
        let eq33_e765_d_n16: f64 = (-p.p355);
        let eq33_e766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq33_e765);
        let eq33_e766_d_n7: f64 = (eq33_e765_d_n7 * ddt_scale);
        let eq33_e766_d_n16: f64 = (eq33_e765_d_n16 * ddt_scale);
        let eq33_e767: f64 = (eq33_e762 + eq33_e766);
        let eq33_e767_d_n7: f64 = (eq33_e762_d_n7 + eq33_e766_d_n7);
        let eq33_e767_d_n16: f64 = (eq33_e762_d_n16 + eq33_e766_d_n16);
        (eq33_e767, eq33_e762_d_n0, eq33_e762_d_n1, eq33_e762_d_n2, eq33_e762_d_n3, eq33_e762_d_n4, eq33_e762_d_n5, eq33_e762_d_n6, eq33_e767_d_n7, eq33_e762_d_n8, eq33_e762_d_n9, eq33_e762_d_n10, eq33_e762_d_n11, eq33_e762_d_n12, eq33_e762_d_n13, eq33_e762_d_n14, eq33_e762_d_n15, eq33_e767_d_n16, eq33_e762_d_n17, eq33_e762_d_n18, eq33_e762_d_n19, eq33_e762_d_n20, eq33_e762_d_n21, eq33_e762_d_n22, eq33_e762_d_n23, eq33_e762_d_n24, eq33_e762_d_n25, eq33_e762_d_n26, eq33_e762_d_n27, eq33_e762_d_n28, eq33_e762_d_n29, eq33_e762_d_b0, eq33_e762_d_b1, eq33_e762_d_b2, eq33_e762_d_b3, eq33_e762_d_b4, eq33_e762_d_b5, eq33_e762_d_b6, eq33_e762_d_b7, eq33_e762_d_b8, eq33_e762_d_b9, eq33_e762_d_b10, eq33_e762_d_b11, eq33_e762_d_b12, eq33_e762_d_b13, eq33_e762_d_b14, eq33_e762_d_b15, eq33_e762_d_b16, eq33_e762_d_b17, eq33_e762_d_b18, eq33_e762_d_b19, eq33_e762_d_b20, eq33_e762_d_b21, eq33_e762_d_b22, eq33_e762_d_b23, eq33_e762_d_b24, eq33_e762_d_b25, eq33_e762_d_b26, eq33_e762_d_b27, eq33_e762_d_b28, eq33_e762_d_b29, eq33_e762_d_b30, eq33_e762_d_b31, eq33_e762_d_b32, eq33_e762_d_b33, eq33_e762_d_b34, eq33_e762_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e769;
        let eq33_node_derivatives: [f64; 30] = [eq33_e769_d_n0, eq33_e769_d_n1, eq33_e769_d_n2, eq33_e769_d_n3, eq33_e769_d_n4, eq33_e769_d_n5, eq33_e769_d_n6, eq33_e769_d_n7, eq33_e769_d_n8, eq33_e769_d_n9, eq33_e769_d_n10, eq33_e769_d_n11, eq33_e769_d_n12, eq33_e769_d_n13, eq33_e769_d_n14, eq33_e769_d_n15, eq33_e769_d_n16, eq33_e769_d_n17, eq33_e769_d_n18, eq33_e769_d_n19, eq33_e769_d_n20, eq33_e769_d_n21, eq33_e769_d_n22, eq33_e769_d_n23, eq33_e769_d_n24, eq33_e769_d_n25, eq33_e769_d_n26, eq33_e769_d_n27, eq33_e769_d_n28, eq33_e769_d_n29];
        let eq33_branch_derivatives: [f64; 36] = [eq33_e769_d_b0, eq33_e769_d_b1, eq33_e769_d_b2, eq33_e769_d_b3, eq33_e769_d_b4, eq33_e769_d_b5, eq33_e769_d_b6, eq33_e769_d_b7, eq33_e769_d_b8, eq33_e769_d_b9, eq33_e769_d_b10, eq33_e769_d_b11, eq33_e769_d_b12, eq33_e769_d_b13, eq33_e769_d_b14, eq33_e769_d_b15, eq33_e769_d_b16, eq33_e769_d_b17, eq33_e769_d_b18, eq33_e769_d_b19, eq33_e769_d_b20, eq33_e769_d_b21, eq33_e769_d_b22, eq33_e769_d_b23, eq33_e769_d_b24, eq33_e769_d_b25, eq33_e769_d_b26, eq33_e769_d_b27, eq33_e769_d_b28, eq33_e769_d_b29, eq33_e769_d_b30, eq33_e769_d_b31, eq33_e769_d_b32, eq33_e769_d_b33, eq33_e769_d_b34, eq33_e769_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e779, eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29, eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35,) = {
    if s.b[466] {
        let eq34_e772: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[210]);
        let eq34_e772_d_n0: f64 = (s.dn[210][0] * ddt_scale);
        let eq34_e772_d_n1: f64 = (s.dn[210][1] * ddt_scale);
        let eq34_e772_d_n2: f64 = (s.dn[210][2] * ddt_scale);
        let eq34_e772_d_n3: f64 = (s.dn[210][3] * ddt_scale);
        let eq34_e772_d_n4: f64 = (s.dn[210][4] * ddt_scale);
        let eq34_e772_d_n5: f64 = (s.dn[210][5] * ddt_scale);
        let eq34_e772_d_n6: f64 = (s.dn[210][6] * ddt_scale);
        let eq34_e772_d_n7: f64 = (s.dn[210][7] * ddt_scale);
        let eq34_e772_d_n8: f64 = (s.dn[210][8] * ddt_scale);
        let eq34_e772_d_n9: f64 = (s.dn[210][9] * ddt_scale);
        let eq34_e772_d_n10: f64 = (s.dn[210][10] * ddt_scale);
        let eq34_e772_d_n11: f64 = (s.dn[210][11] * ddt_scale);
        let eq34_e772_d_n12: f64 = (s.dn[210][12] * ddt_scale);
        let eq34_e772_d_n13: f64 = (s.dn[210][13] * ddt_scale);
        let eq34_e772_d_n14: f64 = (s.dn[210][14] * ddt_scale);
        let eq34_e772_d_n15: f64 = (s.dn[210][15] * ddt_scale);
        let eq34_e772_d_n16: f64 = (s.dn[210][16] * ddt_scale);
        let eq34_e772_d_n17: f64 = (s.dn[210][17] * ddt_scale);
        let eq34_e772_d_n18: f64 = (s.dn[210][18] * ddt_scale);
        let eq34_e772_d_n19: f64 = (s.dn[210][19] * ddt_scale);
        let eq34_e772_d_n20: f64 = (s.dn[210][20] * ddt_scale);
        let eq34_e772_d_n21: f64 = (s.dn[210][21] * ddt_scale);
        let eq34_e772_d_n22: f64 = (s.dn[210][22] * ddt_scale);
        let eq34_e772_d_n23: f64 = (s.dn[210][23] * ddt_scale);
        let eq34_e772_d_n24: f64 = (s.dn[210][24] * ddt_scale);
        let eq34_e772_d_n25: f64 = (s.dn[210][25] * ddt_scale);
        let eq34_e772_d_n26: f64 = (s.dn[210][26] * ddt_scale);
        let eq34_e772_d_n27: f64 = (s.dn[210][27] * ddt_scale);
        let eq34_e772_d_n28: f64 = (s.dn[210][28] * ddt_scale);
        let eq34_e772_d_n29: f64 = (s.dn[210][29] * ddt_scale);
        let eq34_e772_d_b0: f64 = (s.db[210][0] * ddt_scale);
        let eq34_e772_d_b1: f64 = (s.db[210][1] * ddt_scale);
        let eq34_e772_d_b2: f64 = (s.db[210][2] * ddt_scale);
        let eq34_e772_d_b3: f64 = (s.db[210][3] * ddt_scale);
        let eq34_e772_d_b4: f64 = (s.db[210][4] * ddt_scale);
        let eq34_e772_d_b5: f64 = (s.db[210][5] * ddt_scale);
        let eq34_e772_d_b6: f64 = (s.db[210][6] * ddt_scale);
        let eq34_e772_d_b7: f64 = (s.db[210][7] * ddt_scale);
        let eq34_e772_d_b8: f64 = (s.db[210][8] * ddt_scale);
        let eq34_e772_d_b9: f64 = (s.db[210][9] * ddt_scale);
        let eq34_e772_d_b10: f64 = (s.db[210][10] * ddt_scale);
        let eq34_e772_d_b11: f64 = (s.db[210][11] * ddt_scale);
        let eq34_e772_d_b12: f64 = (s.db[210][12] * ddt_scale);
        let eq34_e772_d_b13: f64 = (s.db[210][13] * ddt_scale);
        let eq34_e772_d_b14: f64 = (s.db[210][14] * ddt_scale);
        let eq34_e772_d_b15: f64 = (s.db[210][15] * ddt_scale);
        let eq34_e772_d_b16: f64 = (s.db[210][16] * ddt_scale);
        let eq34_e772_d_b17: f64 = (s.db[210][17] * ddt_scale);
        let eq34_e772_d_b18: f64 = (s.db[210][18] * ddt_scale);
        let eq34_e772_d_b19: f64 = (s.db[210][19] * ddt_scale);
        let eq34_e772_d_b20: f64 = (s.db[210][20] * ddt_scale);
        let eq34_e772_d_b21: f64 = (s.db[210][21] * ddt_scale);
        let eq34_e772_d_b22: f64 = (s.db[210][22] * ddt_scale);
        let eq34_e772_d_b23: f64 = (s.db[210][23] * ddt_scale);
        let eq34_e772_d_b24: f64 = (s.db[210][24] * ddt_scale);
        let eq34_e772_d_b25: f64 = (s.db[210][25] * ddt_scale);
        let eq34_e772_d_b26: f64 = (s.db[210][26] * ddt_scale);
        let eq34_e772_d_b27: f64 = (s.db[210][27] * ddt_scale);
        let eq34_e772_d_b28: f64 = (s.db[210][28] * ddt_scale);
        let eq34_e772_d_b29: f64 = (s.db[210][29] * ddt_scale);
        let eq34_e772_d_b30: f64 = (s.db[210][30] * ddt_scale);
        let eq34_e772_d_b31: f64 = (s.db[210][31] * ddt_scale);
        let eq34_e772_d_b32: f64 = (s.db[210][32] * ddt_scale);
        let eq34_e772_d_b33: f64 = (s.db[210][33] * ddt_scale);
        let eq34_e772_d_b34: f64 = (s.db[210][34] * ddt_scale);
        let eq34_e772_d_b35: f64 = (s.db[210][35] * ddt_scale);
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e775_d_n7: f64 = p.p355;
        let eq34_e775_d_n17: f64 = (-p.p355);
        let eq34_e776: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq34_e775);
        let eq34_e776_d_n7: f64 = (eq34_e775_d_n7 * ddt_scale);
        let eq34_e776_d_n17: f64 = (eq34_e775_d_n17 * ddt_scale);
        let eq34_e777: f64 = (eq34_e772 + eq34_e776);
        let eq34_e777_d_n7: f64 = (eq34_e772_d_n7 + eq34_e776_d_n7);
        let eq34_e777_d_n17: f64 = (eq34_e772_d_n17 + eq34_e776_d_n17);
        (eq34_e777, eq34_e772_d_n0, eq34_e772_d_n1, eq34_e772_d_n2, eq34_e772_d_n3, eq34_e772_d_n4, eq34_e772_d_n5, eq34_e772_d_n6, eq34_e777_d_n7, eq34_e772_d_n8, eq34_e772_d_n9, eq34_e772_d_n10, eq34_e772_d_n11, eq34_e772_d_n12, eq34_e772_d_n13, eq34_e772_d_n14, eq34_e772_d_n15, eq34_e772_d_n16, eq34_e777_d_n17, eq34_e772_d_n18, eq34_e772_d_n19, eq34_e772_d_n20, eq34_e772_d_n21, eq34_e772_d_n22, eq34_e772_d_n23, eq34_e772_d_n24, eq34_e772_d_n25, eq34_e772_d_n26, eq34_e772_d_n27, eq34_e772_d_n28, eq34_e772_d_n29, eq34_e772_d_b0, eq34_e772_d_b1, eq34_e772_d_b2, eq34_e772_d_b3, eq34_e772_d_b4, eq34_e772_d_b5, eq34_e772_d_b6, eq34_e772_d_b7, eq34_e772_d_b8, eq34_e772_d_b9, eq34_e772_d_b10, eq34_e772_d_b11, eq34_e772_d_b12, eq34_e772_d_b13, eq34_e772_d_b14, eq34_e772_d_b15, eq34_e772_d_b16, eq34_e772_d_b17, eq34_e772_d_b18, eq34_e772_d_b19, eq34_e772_d_b20, eq34_e772_d_b21, eq34_e772_d_b22, eq34_e772_d_b23, eq34_e772_d_b24, eq34_e772_d_b25, eq34_e772_d_b26, eq34_e772_d_b27, eq34_e772_d_b28, eq34_e772_d_b29, eq34_e772_d_b30, eq34_e772_d_b31, eq34_e772_d_b32, eq34_e772_d_b33, eq34_e772_d_b34, eq34_e772_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e779;
        let eq34_node_derivatives: [f64; 30] = [eq34_e779_d_n0, eq34_e779_d_n1, eq34_e779_d_n2, eq34_e779_d_n3, eq34_e779_d_n4, eq34_e779_d_n5, eq34_e779_d_n6, eq34_e779_d_n7, eq34_e779_d_n8, eq34_e779_d_n9, eq34_e779_d_n10, eq34_e779_d_n11, eq34_e779_d_n12, eq34_e779_d_n13, eq34_e779_d_n14, eq34_e779_d_n15, eq34_e779_d_n16, eq34_e779_d_n17, eq34_e779_d_n18, eq34_e779_d_n19, eq34_e779_d_n20, eq34_e779_d_n21, eq34_e779_d_n22, eq34_e779_d_n23, eq34_e779_d_n24, eq34_e779_d_n25, eq34_e779_d_n26, eq34_e779_d_n27, eq34_e779_d_n28, eq34_e779_d_n29];
        let eq34_branch_derivatives: [f64; 36] = [eq34_e779_d_b0, eq34_e779_d_b1, eq34_e779_d_b2, eq34_e779_d_b3, eq34_e779_d_b4, eq34_e779_d_b5, eq34_e779_d_b6, eq34_e779_d_b7, eq34_e779_d_b8, eq34_e779_d_b9, eq34_e779_d_b10, eq34_e779_d_b11, eq34_e779_d_b12, eq34_e779_d_b13, eq34_e779_d_b14, eq34_e779_d_b15, eq34_e779_d_b16, eq34_e779_d_b17, eq34_e779_d_b18, eq34_e779_d_b19, eq34_e779_d_b20, eq34_e779_d_b21, eq34_e779_d_b22, eq34_e779_d_b23, eq34_e779_d_b24, eq34_e779_d_b25, eq34_e779_d_b26, eq34_e779_d_b27, eq34_e779_d_b28, eq34_e779_d_b29, eq34_e779_d_b30, eq34_e779_d_b31, eq34_e779_d_b32, eq34_e779_d_b33, eq34_e779_d_b34, eq34_e779_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(17),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e789, eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29, eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35,) = {
    if s.b[466] {
        let eq35_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[211]);
        let eq35_e782_d_n0: f64 = (s.dn[211][0] * ddt_scale);
        let eq35_e782_d_n1: f64 = (s.dn[211][1] * ddt_scale);
        let eq35_e782_d_n2: f64 = (s.dn[211][2] * ddt_scale);
        let eq35_e782_d_n3: f64 = (s.dn[211][3] * ddt_scale);
        let eq35_e782_d_n4: f64 = (s.dn[211][4] * ddt_scale);
        let eq35_e782_d_n5: f64 = (s.dn[211][5] * ddt_scale);
        let eq35_e782_d_n6: f64 = (s.dn[211][6] * ddt_scale);
        let eq35_e782_d_n7: f64 = (s.dn[211][7] * ddt_scale);
        let eq35_e782_d_n8: f64 = (s.dn[211][8] * ddt_scale);
        let eq35_e782_d_n9: f64 = (s.dn[211][9] * ddt_scale);
        let eq35_e782_d_n10: f64 = (s.dn[211][10] * ddt_scale);
        let eq35_e782_d_n11: f64 = (s.dn[211][11] * ddt_scale);
        let eq35_e782_d_n12: f64 = (s.dn[211][12] * ddt_scale);
        let eq35_e782_d_n13: f64 = (s.dn[211][13] * ddt_scale);
        let eq35_e782_d_n14: f64 = (s.dn[211][14] * ddt_scale);
        let eq35_e782_d_n15: f64 = (s.dn[211][15] * ddt_scale);
        let eq35_e782_d_n16: f64 = (s.dn[211][16] * ddt_scale);
        let eq35_e782_d_n17: f64 = (s.dn[211][17] * ddt_scale);
        let eq35_e782_d_n18: f64 = (s.dn[211][18] * ddt_scale);
        let eq35_e782_d_n19: f64 = (s.dn[211][19] * ddt_scale);
        let eq35_e782_d_n20: f64 = (s.dn[211][20] * ddt_scale);
        let eq35_e782_d_n21: f64 = (s.dn[211][21] * ddt_scale);
        let eq35_e782_d_n22: f64 = (s.dn[211][22] * ddt_scale);
        let eq35_e782_d_n23: f64 = (s.dn[211][23] * ddt_scale);
        let eq35_e782_d_n24: f64 = (s.dn[211][24] * ddt_scale);
        let eq35_e782_d_n25: f64 = (s.dn[211][25] * ddt_scale);
        let eq35_e782_d_n26: f64 = (s.dn[211][26] * ddt_scale);
        let eq35_e782_d_n27: f64 = (s.dn[211][27] * ddt_scale);
        let eq35_e782_d_n28: f64 = (s.dn[211][28] * ddt_scale);
        let eq35_e782_d_n29: f64 = (s.dn[211][29] * ddt_scale);
        let eq35_e782_d_b0: f64 = (s.db[211][0] * ddt_scale);
        let eq35_e782_d_b1: f64 = (s.db[211][1] * ddt_scale);
        let eq35_e782_d_b2: f64 = (s.db[211][2] * ddt_scale);
        let eq35_e782_d_b3: f64 = (s.db[211][3] * ddt_scale);
        let eq35_e782_d_b4: f64 = (s.db[211][4] * ddt_scale);
        let eq35_e782_d_b5: f64 = (s.db[211][5] * ddt_scale);
        let eq35_e782_d_b6: f64 = (s.db[211][6] * ddt_scale);
        let eq35_e782_d_b7: f64 = (s.db[211][7] * ddt_scale);
        let eq35_e782_d_b8: f64 = (s.db[211][8] * ddt_scale);
        let eq35_e782_d_b9: f64 = (s.db[211][9] * ddt_scale);
        let eq35_e782_d_b10: f64 = (s.db[211][10] * ddt_scale);
        let eq35_e782_d_b11: f64 = (s.db[211][11] * ddt_scale);
        let eq35_e782_d_b12: f64 = (s.db[211][12] * ddt_scale);
        let eq35_e782_d_b13: f64 = (s.db[211][13] * ddt_scale);
        let eq35_e782_d_b14: f64 = (s.db[211][14] * ddt_scale);
        let eq35_e782_d_b15: f64 = (s.db[211][15] * ddt_scale);
        let eq35_e782_d_b16: f64 = (s.db[211][16] * ddt_scale);
        let eq35_e782_d_b17: f64 = (s.db[211][17] * ddt_scale);
        let eq35_e782_d_b18: f64 = (s.db[211][18] * ddt_scale);
        let eq35_e782_d_b19: f64 = (s.db[211][19] * ddt_scale);
        let eq35_e782_d_b20: f64 = (s.db[211][20] * ddt_scale);
        let eq35_e782_d_b21: f64 = (s.db[211][21] * ddt_scale);
        let eq35_e782_d_b22: f64 = (s.db[211][22] * ddt_scale);
        let eq35_e782_d_b23: f64 = (s.db[211][23] * ddt_scale);
        let eq35_e782_d_b24: f64 = (s.db[211][24] * ddt_scale);
        let eq35_e782_d_b25: f64 = (s.db[211][25] * ddt_scale);
        let eq35_e782_d_b26: f64 = (s.db[211][26] * ddt_scale);
        let eq35_e782_d_b27: f64 = (s.db[211][27] * ddt_scale);
        let eq35_e782_d_b28: f64 = (s.db[211][28] * ddt_scale);
        let eq35_e782_d_b29: f64 = (s.db[211][29] * ddt_scale);
        let eq35_e782_d_b30: f64 = (s.db[211][30] * ddt_scale);
        let eq35_e782_d_b31: f64 = (s.db[211][31] * ddt_scale);
        let eq35_e782_d_b32: f64 = (s.db[211][32] * ddt_scale);
        let eq35_e782_d_b33: f64 = (s.db[211][33] * ddt_scale);
        let eq35_e782_d_b34: f64 = (s.db[211][34] * ddt_scale);
        let eq35_e782_d_b35: f64 = (s.db[211][35] * ddt_scale);
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e785_d_n2: f64 = p.p355;
        let eq35_e785_d_n16: f64 = (-p.p355);
        let eq35_e786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq35_e785);
        let eq35_e786_d_n2: f64 = (eq35_e785_d_n2 * ddt_scale);
        let eq35_e786_d_n16: f64 = (eq35_e785_d_n16 * ddt_scale);
        let eq35_e787: f64 = (eq35_e782 + eq35_e786);
        let eq35_e787_d_n2: f64 = (eq35_e782_d_n2 + eq35_e786_d_n2);
        let eq35_e787_d_n16: f64 = (eq35_e782_d_n16 + eq35_e786_d_n16);
        (eq35_e787, eq35_e782_d_n0, eq35_e782_d_n1, eq35_e787_d_n2, eq35_e782_d_n3, eq35_e782_d_n4, eq35_e782_d_n5, eq35_e782_d_n6, eq35_e782_d_n7, eq35_e782_d_n8, eq35_e782_d_n9, eq35_e782_d_n10, eq35_e782_d_n11, eq35_e782_d_n12, eq35_e782_d_n13, eq35_e782_d_n14, eq35_e782_d_n15, eq35_e787_d_n16, eq35_e782_d_n17, eq35_e782_d_n18, eq35_e782_d_n19, eq35_e782_d_n20, eq35_e782_d_n21, eq35_e782_d_n22, eq35_e782_d_n23, eq35_e782_d_n24, eq35_e782_d_n25, eq35_e782_d_n26, eq35_e782_d_n27, eq35_e782_d_n28, eq35_e782_d_n29, eq35_e782_d_b0, eq35_e782_d_b1, eq35_e782_d_b2, eq35_e782_d_b3, eq35_e782_d_b4, eq35_e782_d_b5, eq35_e782_d_b6, eq35_e782_d_b7, eq35_e782_d_b8, eq35_e782_d_b9, eq35_e782_d_b10, eq35_e782_d_b11, eq35_e782_d_b12, eq35_e782_d_b13, eq35_e782_d_b14, eq35_e782_d_b15, eq35_e782_d_b16, eq35_e782_d_b17, eq35_e782_d_b18, eq35_e782_d_b19, eq35_e782_d_b20, eq35_e782_d_b21, eq35_e782_d_b22, eq35_e782_d_b23, eq35_e782_d_b24, eq35_e782_d_b25, eq35_e782_d_b26, eq35_e782_d_b27, eq35_e782_d_b28, eq35_e782_d_b29, eq35_e782_d_b30, eq35_e782_d_b31, eq35_e782_d_b32, eq35_e782_d_b33, eq35_e782_d_b34, eq35_e782_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e789;
        let eq35_node_derivatives: [f64; 30] = [eq35_e789_d_n0, eq35_e789_d_n1, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n5, eq35_e789_d_n6, eq35_e789_d_n7, eq35_e789_d_n8, eq35_e789_d_n9, eq35_e789_d_n10, eq35_e789_d_n11, eq35_e789_d_n12, eq35_e789_d_n13, eq35_e789_d_n14, eq35_e789_d_n15, eq35_e789_d_n16, eq35_e789_d_n17, eq35_e789_d_n18, eq35_e789_d_n19, eq35_e789_d_n20, eq35_e789_d_n21, eq35_e789_d_n22, eq35_e789_d_n23, eq35_e789_d_n24, eq35_e789_d_n25, eq35_e789_d_n26, eq35_e789_d_n27, eq35_e789_d_n28, eq35_e789_d_n29];
        let eq35_branch_derivatives: [f64; 36] = [eq35_e789_d_b0, eq35_e789_d_b1, eq35_e789_d_b2, eq35_e789_d_b3, eq35_e789_d_b4, eq35_e789_d_b5, eq35_e789_d_b6, eq35_e789_d_b7, eq35_e789_d_b8, eq35_e789_d_b9, eq35_e789_d_b10, eq35_e789_d_b11, eq35_e789_d_b12, eq35_e789_d_b13, eq35_e789_d_b14, eq35_e789_d_b15, eq35_e789_d_b16, eq35_e789_d_b17, eq35_e789_d_b18, eq35_e789_d_b19, eq35_e789_d_b20, eq35_e789_d_b21, eq35_e789_d_b22, eq35_e789_d_b23, eq35_e789_d_b24, eq35_e789_d_b25, eq35_e789_d_b26, eq35_e789_d_b27, eq35_e789_d_b28, eq35_e789_d_b29, eq35_e789_d_b30, eq35_e789_d_b31, eq35_e789_d_b32, eq35_e789_d_b33, eq35_e789_d_b34, eq35_e789_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e793,) = {
    if s.b[466] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e793;
        stamper.stamp_current_const_local(
            Some(2),
            Some(17),
            multiplicity * (eq36_value),
        );
        let (eq37_e803, eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29, eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35,) = {
    if s.b[466] {
        let eq37_e796: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[213]);
        let eq37_e796_d_n0: f64 = (s.dn[213][0] * ddt_scale);
        let eq37_e796_d_n1: f64 = (s.dn[213][1] * ddt_scale);
        let eq37_e796_d_n2: f64 = (s.dn[213][2] * ddt_scale);
        let eq37_e796_d_n3: f64 = (s.dn[213][3] * ddt_scale);
        let eq37_e796_d_n4: f64 = (s.dn[213][4] * ddt_scale);
        let eq37_e796_d_n5: f64 = (s.dn[213][5] * ddt_scale);
        let eq37_e796_d_n6: f64 = (s.dn[213][6] * ddt_scale);
        let eq37_e796_d_n7: f64 = (s.dn[213][7] * ddt_scale);
        let eq37_e796_d_n8: f64 = (s.dn[213][8] * ddt_scale);
        let eq37_e796_d_n9: f64 = (s.dn[213][9] * ddt_scale);
        let eq37_e796_d_n10: f64 = (s.dn[213][10] * ddt_scale);
        let eq37_e796_d_n11: f64 = (s.dn[213][11] * ddt_scale);
        let eq37_e796_d_n12: f64 = (s.dn[213][12] * ddt_scale);
        let eq37_e796_d_n13: f64 = (s.dn[213][13] * ddt_scale);
        let eq37_e796_d_n14: f64 = (s.dn[213][14] * ddt_scale);
        let eq37_e796_d_n15: f64 = (s.dn[213][15] * ddt_scale);
        let eq37_e796_d_n16: f64 = (s.dn[213][16] * ddt_scale);
        let eq37_e796_d_n17: f64 = (s.dn[213][17] * ddt_scale);
        let eq37_e796_d_n18: f64 = (s.dn[213][18] * ddt_scale);
        let eq37_e796_d_n19: f64 = (s.dn[213][19] * ddt_scale);
        let eq37_e796_d_n20: f64 = (s.dn[213][20] * ddt_scale);
        let eq37_e796_d_n21: f64 = (s.dn[213][21] * ddt_scale);
        let eq37_e796_d_n22: f64 = (s.dn[213][22] * ddt_scale);
        let eq37_e796_d_n23: f64 = (s.dn[213][23] * ddt_scale);
        let eq37_e796_d_n24: f64 = (s.dn[213][24] * ddt_scale);
        let eq37_e796_d_n25: f64 = (s.dn[213][25] * ddt_scale);
        let eq37_e796_d_n26: f64 = (s.dn[213][26] * ddt_scale);
        let eq37_e796_d_n27: f64 = (s.dn[213][27] * ddt_scale);
        let eq37_e796_d_n28: f64 = (s.dn[213][28] * ddt_scale);
        let eq37_e796_d_n29: f64 = (s.dn[213][29] * ddt_scale);
        let eq37_e796_d_b0: f64 = (s.db[213][0] * ddt_scale);
        let eq37_e796_d_b1: f64 = (s.db[213][1] * ddt_scale);
        let eq37_e796_d_b2: f64 = (s.db[213][2] * ddt_scale);
        let eq37_e796_d_b3: f64 = (s.db[213][3] * ddt_scale);
        let eq37_e796_d_b4: f64 = (s.db[213][4] * ddt_scale);
        let eq37_e796_d_b5: f64 = (s.db[213][5] * ddt_scale);
        let eq37_e796_d_b6: f64 = (s.db[213][6] * ddt_scale);
        let eq37_e796_d_b7: f64 = (s.db[213][7] * ddt_scale);
        let eq37_e796_d_b8: f64 = (s.db[213][8] * ddt_scale);
        let eq37_e796_d_b9: f64 = (s.db[213][9] * ddt_scale);
        let eq37_e796_d_b10: f64 = (s.db[213][10] * ddt_scale);
        let eq37_e796_d_b11: f64 = (s.db[213][11] * ddt_scale);
        let eq37_e796_d_b12: f64 = (s.db[213][12] * ddt_scale);
        let eq37_e796_d_b13: f64 = (s.db[213][13] * ddt_scale);
        let eq37_e796_d_b14: f64 = (s.db[213][14] * ddt_scale);
        let eq37_e796_d_b15: f64 = (s.db[213][15] * ddt_scale);
        let eq37_e796_d_b16: f64 = (s.db[213][16] * ddt_scale);
        let eq37_e796_d_b17: f64 = (s.db[213][17] * ddt_scale);
        let eq37_e796_d_b18: f64 = (s.db[213][18] * ddt_scale);
        let eq37_e796_d_b19: f64 = (s.db[213][19] * ddt_scale);
        let eq37_e796_d_b20: f64 = (s.db[213][20] * ddt_scale);
        let eq37_e796_d_b21: f64 = (s.db[213][21] * ddt_scale);
        let eq37_e796_d_b22: f64 = (s.db[213][22] * ddt_scale);
        let eq37_e796_d_b23: f64 = (s.db[213][23] * ddt_scale);
        let eq37_e796_d_b24: f64 = (s.db[213][24] * ddt_scale);
        let eq37_e796_d_b25: f64 = (s.db[213][25] * ddt_scale);
        let eq37_e796_d_b26: f64 = (s.db[213][26] * ddt_scale);
        let eq37_e796_d_b27: f64 = (s.db[213][27] * ddt_scale);
        let eq37_e796_d_b28: f64 = (s.db[213][28] * ddt_scale);
        let eq37_e796_d_b29: f64 = (s.db[213][29] * ddt_scale);
        let eq37_e796_d_b30: f64 = (s.db[213][30] * ddt_scale);
        let eq37_e796_d_b31: f64 = (s.db[213][31] * ddt_scale);
        let eq37_e796_d_b32: f64 = (s.db[213][32] * ddt_scale);
        let eq37_e796_d_b33: f64 = (s.db[213][33] * ddt_scale);
        let eq37_e796_d_b34: f64 = (s.db[213][34] * ddt_scale);
        let eq37_e796_d_b35: f64 = (s.db[213][35] * ddt_scale);
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e799_d_n7: f64 = p.p355;
        let eq37_e799_d_n9: f64 = (-p.p355);
        let eq37_e800: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq37_e799);
        let eq37_e800_d_n7: f64 = (eq37_e799_d_n7 * ddt_scale);
        let eq37_e800_d_n9: f64 = (eq37_e799_d_n9 * ddt_scale);
        let eq37_e801: f64 = (eq37_e796 + eq37_e800);
        let eq37_e801_d_n7: f64 = (eq37_e796_d_n7 + eq37_e800_d_n7);
        let eq37_e801_d_n9: f64 = (eq37_e796_d_n9 + eq37_e800_d_n9);
        (eq37_e801, eq37_e796_d_n0, eq37_e796_d_n1, eq37_e796_d_n2, eq37_e796_d_n3, eq37_e796_d_n4, eq37_e796_d_n5, eq37_e796_d_n6, eq37_e801_d_n7, eq37_e796_d_n8, eq37_e801_d_n9, eq37_e796_d_n10, eq37_e796_d_n11, eq37_e796_d_n12, eq37_e796_d_n13, eq37_e796_d_n14, eq37_e796_d_n15, eq37_e796_d_n16, eq37_e796_d_n17, eq37_e796_d_n18, eq37_e796_d_n19, eq37_e796_d_n20, eq37_e796_d_n21, eq37_e796_d_n22, eq37_e796_d_n23, eq37_e796_d_n24, eq37_e796_d_n25, eq37_e796_d_n26, eq37_e796_d_n27, eq37_e796_d_n28, eq37_e796_d_n29, eq37_e796_d_b0, eq37_e796_d_b1, eq37_e796_d_b2, eq37_e796_d_b3, eq37_e796_d_b4, eq37_e796_d_b5, eq37_e796_d_b6, eq37_e796_d_b7, eq37_e796_d_b8, eq37_e796_d_b9, eq37_e796_d_b10, eq37_e796_d_b11, eq37_e796_d_b12, eq37_e796_d_b13, eq37_e796_d_b14, eq37_e796_d_b15, eq37_e796_d_b16, eq37_e796_d_b17, eq37_e796_d_b18, eq37_e796_d_b19, eq37_e796_d_b20, eq37_e796_d_b21, eq37_e796_d_b22, eq37_e796_d_b23, eq37_e796_d_b24, eq37_e796_d_b25, eq37_e796_d_b26, eq37_e796_d_b27, eq37_e796_d_b28, eq37_e796_d_b29, eq37_e796_d_b30, eq37_e796_d_b31, eq37_e796_d_b32, eq37_e796_d_b33, eq37_e796_d_b34, eq37_e796_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        let eq37_node_derivatives: [f64; 30] = [eq37_e803_d_n0, eq37_e803_d_n1, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n5, eq37_e803_d_n6, eq37_e803_d_n7, eq37_e803_d_n8, eq37_e803_d_n9, eq37_e803_d_n10, eq37_e803_d_n11, eq37_e803_d_n12, eq37_e803_d_n13, eq37_e803_d_n14, eq37_e803_d_n15, eq37_e803_d_n16, eq37_e803_d_n17, eq37_e803_d_n18, eq37_e803_d_n19, eq37_e803_d_n20, eq37_e803_d_n21, eq37_e803_d_n22, eq37_e803_d_n23, eq37_e803_d_n24, eq37_e803_d_n25, eq37_e803_d_n26, eq37_e803_d_n27, eq37_e803_d_n28, eq37_e803_d_n29];
        let eq37_branch_derivatives: [f64; 36] = [eq37_e803_d_b0, eq37_e803_d_b1, eq37_e803_d_b2, eq37_e803_d_b3, eq37_e803_d_b4, eq37_e803_d_b5, eq37_e803_d_b6, eq37_e803_d_b7, eq37_e803_d_b8, eq37_e803_d_b9, eq37_e803_d_b10, eq37_e803_d_b11, eq37_e803_d_b12, eq37_e803_d_b13, eq37_e803_d_b14, eq37_e803_d_b15, eq37_e803_d_b16, eq37_e803_d_b17, eq37_e803_d_b18, eq37_e803_d_b19, eq37_e803_d_b20, eq37_e803_d_b21, eq37_e803_d_b22, eq37_e803_d_b23, eq37_e803_d_b24, eq37_e803_d_b25, eq37_e803_d_b26, eq37_e803_d_b27, eq37_e803_d_b28, eq37_e803_d_b29, eq37_e803_d_b30, eq37_e803_d_b31, eq37_e803_d_b32, eq37_e803_d_b33, eq37_e803_d_b34, eq37_e803_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq38_e814, eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29, eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35,) = {
    if (!s.b[466]) {
        let eq38_e807: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[209]);
        let eq38_e807_d_n0: f64 = (s.dn[209][0] * ddt_scale);
        let eq38_e807_d_n1: f64 = (s.dn[209][1] * ddt_scale);
        let eq38_e807_d_n2: f64 = (s.dn[209][2] * ddt_scale);
        let eq38_e807_d_n3: f64 = (s.dn[209][3] * ddt_scale);
        let eq38_e807_d_n4: f64 = (s.dn[209][4] * ddt_scale);
        let eq38_e807_d_n5: f64 = (s.dn[209][5] * ddt_scale);
        let eq38_e807_d_n6: f64 = (s.dn[209][6] * ddt_scale);
        let eq38_e807_d_n7: f64 = (s.dn[209][7] * ddt_scale);
        let eq38_e807_d_n8: f64 = (s.dn[209][8] * ddt_scale);
        let eq38_e807_d_n9: f64 = (s.dn[209][9] * ddt_scale);
        let eq38_e807_d_n10: f64 = (s.dn[209][10] * ddt_scale);
        let eq38_e807_d_n11: f64 = (s.dn[209][11] * ddt_scale);
        let eq38_e807_d_n12: f64 = (s.dn[209][12] * ddt_scale);
        let eq38_e807_d_n13: f64 = (s.dn[209][13] * ddt_scale);
        let eq38_e807_d_n14: f64 = (s.dn[209][14] * ddt_scale);
        let eq38_e807_d_n15: f64 = (s.dn[209][15] * ddt_scale);
        let eq38_e807_d_n16: f64 = (s.dn[209][16] * ddt_scale);
        let eq38_e807_d_n17: f64 = (s.dn[209][17] * ddt_scale);
        let eq38_e807_d_n18: f64 = (s.dn[209][18] * ddt_scale);
        let eq38_e807_d_n19: f64 = (s.dn[209][19] * ddt_scale);
        let eq38_e807_d_n20: f64 = (s.dn[209][20] * ddt_scale);
        let eq38_e807_d_n21: f64 = (s.dn[209][21] * ddt_scale);
        let eq38_e807_d_n22: f64 = (s.dn[209][22] * ddt_scale);
        let eq38_e807_d_n23: f64 = (s.dn[209][23] * ddt_scale);
        let eq38_e807_d_n24: f64 = (s.dn[209][24] * ddt_scale);
        let eq38_e807_d_n25: f64 = (s.dn[209][25] * ddt_scale);
        let eq38_e807_d_n26: f64 = (s.dn[209][26] * ddt_scale);
        let eq38_e807_d_n27: f64 = (s.dn[209][27] * ddt_scale);
        let eq38_e807_d_n28: f64 = (s.dn[209][28] * ddt_scale);
        let eq38_e807_d_n29: f64 = (s.dn[209][29] * ddt_scale);
        let eq38_e807_d_b0: f64 = (s.db[209][0] * ddt_scale);
        let eq38_e807_d_b1: f64 = (s.db[209][1] * ddt_scale);
        let eq38_e807_d_b2: f64 = (s.db[209][2] * ddt_scale);
        let eq38_e807_d_b3: f64 = (s.db[209][3] * ddt_scale);
        let eq38_e807_d_b4: f64 = (s.db[209][4] * ddt_scale);
        let eq38_e807_d_b5: f64 = (s.db[209][5] * ddt_scale);
        let eq38_e807_d_b6: f64 = (s.db[209][6] * ddt_scale);
        let eq38_e807_d_b7: f64 = (s.db[209][7] * ddt_scale);
        let eq38_e807_d_b8: f64 = (s.db[209][8] * ddt_scale);
        let eq38_e807_d_b9: f64 = (s.db[209][9] * ddt_scale);
        let eq38_e807_d_b10: f64 = (s.db[209][10] * ddt_scale);
        let eq38_e807_d_b11: f64 = (s.db[209][11] * ddt_scale);
        let eq38_e807_d_b12: f64 = (s.db[209][12] * ddt_scale);
        let eq38_e807_d_b13: f64 = (s.db[209][13] * ddt_scale);
        let eq38_e807_d_b14: f64 = (s.db[209][14] * ddt_scale);
        let eq38_e807_d_b15: f64 = (s.db[209][15] * ddt_scale);
        let eq38_e807_d_b16: f64 = (s.db[209][16] * ddt_scale);
        let eq38_e807_d_b17: f64 = (s.db[209][17] * ddt_scale);
        let eq38_e807_d_b18: f64 = (s.db[209][18] * ddt_scale);
        let eq38_e807_d_b19: f64 = (s.db[209][19] * ddt_scale);
        let eq38_e807_d_b20: f64 = (s.db[209][20] * ddt_scale);
        let eq38_e807_d_b21: f64 = (s.db[209][21] * ddt_scale);
        let eq38_e807_d_b22: f64 = (s.db[209][22] * ddt_scale);
        let eq38_e807_d_b23: f64 = (s.db[209][23] * ddt_scale);
        let eq38_e807_d_b24: f64 = (s.db[209][24] * ddt_scale);
        let eq38_e807_d_b25: f64 = (s.db[209][25] * ddt_scale);
        let eq38_e807_d_b26: f64 = (s.db[209][26] * ddt_scale);
        let eq38_e807_d_b27: f64 = (s.db[209][27] * ddt_scale);
        let eq38_e807_d_b28: f64 = (s.db[209][28] * ddt_scale);
        let eq38_e807_d_b29: f64 = (s.db[209][29] * ddt_scale);
        let eq38_e807_d_b30: f64 = (s.db[209][30] * ddt_scale);
        let eq38_e807_d_b31: f64 = (s.db[209][31] * ddt_scale);
        let eq38_e807_d_b32: f64 = (s.db[209][32] * ddt_scale);
        let eq38_e807_d_b33: f64 = (s.db[209][33] * ddt_scale);
        let eq38_e807_d_b34: f64 = (s.db[209][34] * ddt_scale);
        let eq38_e807_d_b35: f64 = (s.db[209][35] * ddt_scale);
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e810_d_n2: f64 = p.p355;
        let eq38_e810_d_n16: f64 = (-p.p355);
        let eq38_e811: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq38_e810);
        let eq38_e811_d_n2: f64 = (eq38_e810_d_n2 * ddt_scale);
        let eq38_e811_d_n16: f64 = (eq38_e810_d_n16 * ddt_scale);
        let eq38_e812: f64 = (eq38_e807 + eq38_e811);
        let eq38_e812_d_n2: f64 = (eq38_e807_d_n2 + eq38_e811_d_n2);
        let eq38_e812_d_n16: f64 = (eq38_e807_d_n16 + eq38_e811_d_n16);
        (eq38_e812, eq38_e807_d_n0, eq38_e807_d_n1, eq38_e812_d_n2, eq38_e807_d_n3, eq38_e807_d_n4, eq38_e807_d_n5, eq38_e807_d_n6, eq38_e807_d_n7, eq38_e807_d_n8, eq38_e807_d_n9, eq38_e807_d_n10, eq38_e807_d_n11, eq38_e807_d_n12, eq38_e807_d_n13, eq38_e807_d_n14, eq38_e807_d_n15, eq38_e812_d_n16, eq38_e807_d_n17, eq38_e807_d_n18, eq38_e807_d_n19, eq38_e807_d_n20, eq38_e807_d_n21, eq38_e807_d_n22, eq38_e807_d_n23, eq38_e807_d_n24, eq38_e807_d_n25, eq38_e807_d_n26, eq38_e807_d_n27, eq38_e807_d_n28, eq38_e807_d_n29, eq38_e807_d_b0, eq38_e807_d_b1, eq38_e807_d_b2, eq38_e807_d_b3, eq38_e807_d_b4, eq38_e807_d_b5, eq38_e807_d_b6, eq38_e807_d_b7, eq38_e807_d_b8, eq38_e807_d_b9, eq38_e807_d_b10, eq38_e807_d_b11, eq38_e807_d_b12, eq38_e807_d_b13, eq38_e807_d_b14, eq38_e807_d_b15, eq38_e807_d_b16, eq38_e807_d_b17, eq38_e807_d_b18, eq38_e807_d_b19, eq38_e807_d_b20, eq38_e807_d_b21, eq38_e807_d_b22, eq38_e807_d_b23, eq38_e807_d_b24, eq38_e807_d_b25, eq38_e807_d_b26, eq38_e807_d_b27, eq38_e807_d_b28, eq38_e807_d_b29, eq38_e807_d_b30, eq38_e807_d_b31, eq38_e807_d_b32, eq38_e807_d_b33, eq38_e807_d_b34, eq38_e807_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        let eq38_node_derivatives: [f64; 30] = [eq38_e814_d_n0, eq38_e814_d_n1, eq38_e814_d_n2, eq38_e814_d_n3, eq38_e814_d_n4, eq38_e814_d_n5, eq38_e814_d_n6, eq38_e814_d_n7, eq38_e814_d_n8, eq38_e814_d_n9, eq38_e814_d_n10, eq38_e814_d_n11, eq38_e814_d_n12, eq38_e814_d_n13, eq38_e814_d_n14, eq38_e814_d_n15, eq38_e814_d_n16, eq38_e814_d_n17, eq38_e814_d_n18, eq38_e814_d_n19, eq38_e814_d_n20, eq38_e814_d_n21, eq38_e814_d_n22, eq38_e814_d_n23, eq38_e814_d_n24, eq38_e814_d_n25, eq38_e814_d_n26, eq38_e814_d_n27, eq38_e814_d_n28, eq38_e814_d_n29];
        let eq38_branch_derivatives: [f64; 36] = [eq38_e814_d_b0, eq38_e814_d_b1, eq38_e814_d_b2, eq38_e814_d_b3, eq38_e814_d_b4, eq38_e814_d_b5, eq38_e814_d_b6, eq38_e814_d_b7, eq38_e814_d_b8, eq38_e814_d_b9, eq38_e814_d_b10, eq38_e814_d_b11, eq38_e814_d_b12, eq38_e814_d_b13, eq38_e814_d_b14, eq38_e814_d_b15, eq38_e814_d_b16, eq38_e814_d_b17, eq38_e814_d_b18, eq38_e814_d_b19, eq38_e814_d_b20, eq38_e814_d_b21, eq38_e814_d_b22, eq38_e814_d_b23, eq38_e814_d_b24, eq38_e814_d_b25, eq38_e814_d_b26, eq38_e814_d_b27, eq38_e814_d_b28, eq38_e814_d_b29, eq38_e814_d_b30, eq38_e814_d_b31, eq38_e814_d_b32, eq38_e814_d_b33, eq38_e814_d_b34, eq38_e814_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e825, eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29, eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35,) = {
    if (!s.b[466]) {
        let eq39_e818: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[210]);
        let eq39_e818_d_n0: f64 = (s.dn[210][0] * ddt_scale);
        let eq39_e818_d_n1: f64 = (s.dn[210][1] * ddt_scale);
        let eq39_e818_d_n2: f64 = (s.dn[210][2] * ddt_scale);
        let eq39_e818_d_n3: f64 = (s.dn[210][3] * ddt_scale);
        let eq39_e818_d_n4: f64 = (s.dn[210][4] * ddt_scale);
        let eq39_e818_d_n5: f64 = (s.dn[210][5] * ddt_scale);
        let eq39_e818_d_n6: f64 = (s.dn[210][6] * ddt_scale);
        let eq39_e818_d_n7: f64 = (s.dn[210][7] * ddt_scale);
        let eq39_e818_d_n8: f64 = (s.dn[210][8] * ddt_scale);
        let eq39_e818_d_n9: f64 = (s.dn[210][9] * ddt_scale);
        let eq39_e818_d_n10: f64 = (s.dn[210][10] * ddt_scale);
        let eq39_e818_d_n11: f64 = (s.dn[210][11] * ddt_scale);
        let eq39_e818_d_n12: f64 = (s.dn[210][12] * ddt_scale);
        let eq39_e818_d_n13: f64 = (s.dn[210][13] * ddt_scale);
        let eq39_e818_d_n14: f64 = (s.dn[210][14] * ddt_scale);
        let eq39_e818_d_n15: f64 = (s.dn[210][15] * ddt_scale);
        let eq39_e818_d_n16: f64 = (s.dn[210][16] * ddt_scale);
        let eq39_e818_d_n17: f64 = (s.dn[210][17] * ddt_scale);
        let eq39_e818_d_n18: f64 = (s.dn[210][18] * ddt_scale);
        let eq39_e818_d_n19: f64 = (s.dn[210][19] * ddt_scale);
        let eq39_e818_d_n20: f64 = (s.dn[210][20] * ddt_scale);
        let eq39_e818_d_n21: f64 = (s.dn[210][21] * ddt_scale);
        let eq39_e818_d_n22: f64 = (s.dn[210][22] * ddt_scale);
        let eq39_e818_d_n23: f64 = (s.dn[210][23] * ddt_scale);
        let eq39_e818_d_n24: f64 = (s.dn[210][24] * ddt_scale);
        let eq39_e818_d_n25: f64 = (s.dn[210][25] * ddt_scale);
        let eq39_e818_d_n26: f64 = (s.dn[210][26] * ddt_scale);
        let eq39_e818_d_n27: f64 = (s.dn[210][27] * ddt_scale);
        let eq39_e818_d_n28: f64 = (s.dn[210][28] * ddt_scale);
        let eq39_e818_d_n29: f64 = (s.dn[210][29] * ddt_scale);
        let eq39_e818_d_b0: f64 = (s.db[210][0] * ddt_scale);
        let eq39_e818_d_b1: f64 = (s.db[210][1] * ddt_scale);
        let eq39_e818_d_b2: f64 = (s.db[210][2] * ddt_scale);
        let eq39_e818_d_b3: f64 = (s.db[210][3] * ddt_scale);
        let eq39_e818_d_b4: f64 = (s.db[210][4] * ddt_scale);
        let eq39_e818_d_b5: f64 = (s.db[210][5] * ddt_scale);
        let eq39_e818_d_b6: f64 = (s.db[210][6] * ddt_scale);
        let eq39_e818_d_b7: f64 = (s.db[210][7] * ddt_scale);
        let eq39_e818_d_b8: f64 = (s.db[210][8] * ddt_scale);
        let eq39_e818_d_b9: f64 = (s.db[210][9] * ddt_scale);
        let eq39_e818_d_b10: f64 = (s.db[210][10] * ddt_scale);
        let eq39_e818_d_b11: f64 = (s.db[210][11] * ddt_scale);
        let eq39_e818_d_b12: f64 = (s.db[210][12] * ddt_scale);
        let eq39_e818_d_b13: f64 = (s.db[210][13] * ddt_scale);
        let eq39_e818_d_b14: f64 = (s.db[210][14] * ddt_scale);
        let eq39_e818_d_b15: f64 = (s.db[210][15] * ddt_scale);
        let eq39_e818_d_b16: f64 = (s.db[210][16] * ddt_scale);
        let eq39_e818_d_b17: f64 = (s.db[210][17] * ddt_scale);
        let eq39_e818_d_b18: f64 = (s.db[210][18] * ddt_scale);
        let eq39_e818_d_b19: f64 = (s.db[210][19] * ddt_scale);
        let eq39_e818_d_b20: f64 = (s.db[210][20] * ddt_scale);
        let eq39_e818_d_b21: f64 = (s.db[210][21] * ddt_scale);
        let eq39_e818_d_b22: f64 = (s.db[210][22] * ddt_scale);
        let eq39_e818_d_b23: f64 = (s.db[210][23] * ddt_scale);
        let eq39_e818_d_b24: f64 = (s.db[210][24] * ddt_scale);
        let eq39_e818_d_b25: f64 = (s.db[210][25] * ddt_scale);
        let eq39_e818_d_b26: f64 = (s.db[210][26] * ddt_scale);
        let eq39_e818_d_b27: f64 = (s.db[210][27] * ddt_scale);
        let eq39_e818_d_b28: f64 = (s.db[210][28] * ddt_scale);
        let eq39_e818_d_b29: f64 = (s.db[210][29] * ddt_scale);
        let eq39_e818_d_b30: f64 = (s.db[210][30] * ddt_scale);
        let eq39_e818_d_b31: f64 = (s.db[210][31] * ddt_scale);
        let eq39_e818_d_b32: f64 = (s.db[210][32] * ddt_scale);
        let eq39_e818_d_b33: f64 = (s.db[210][33] * ddt_scale);
        let eq39_e818_d_b34: f64 = (s.db[210][34] * ddt_scale);
        let eq39_e818_d_b35: f64 = (s.db[210][35] * ddt_scale);
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e821_d_n2: f64 = p.p355;
        let eq39_e821_d_n17: f64 = (-p.p355);
        let eq39_e822: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq39_e821);
        let eq39_e822_d_n2: f64 = (eq39_e821_d_n2 * ddt_scale);
        let eq39_e822_d_n17: f64 = (eq39_e821_d_n17 * ddt_scale);
        let eq39_e823: f64 = (eq39_e818 + eq39_e822);
        let eq39_e823_d_n2: f64 = (eq39_e818_d_n2 + eq39_e822_d_n2);
        let eq39_e823_d_n17: f64 = (eq39_e818_d_n17 + eq39_e822_d_n17);
        (eq39_e823, eq39_e818_d_n0, eq39_e818_d_n1, eq39_e823_d_n2, eq39_e818_d_n3, eq39_e818_d_n4, eq39_e818_d_n5, eq39_e818_d_n6, eq39_e818_d_n7, eq39_e818_d_n8, eq39_e818_d_n9, eq39_e818_d_n10, eq39_e818_d_n11, eq39_e818_d_n12, eq39_e818_d_n13, eq39_e818_d_n14, eq39_e818_d_n15, eq39_e818_d_n16, eq39_e823_d_n17, eq39_e818_d_n18, eq39_e818_d_n19, eq39_e818_d_n20, eq39_e818_d_n21, eq39_e818_d_n22, eq39_e818_d_n23, eq39_e818_d_n24, eq39_e818_d_n25, eq39_e818_d_n26, eq39_e818_d_n27, eq39_e818_d_n28, eq39_e818_d_n29, eq39_e818_d_b0, eq39_e818_d_b1, eq39_e818_d_b2, eq39_e818_d_b3, eq39_e818_d_b4, eq39_e818_d_b5, eq39_e818_d_b6, eq39_e818_d_b7, eq39_e818_d_b8, eq39_e818_d_b9, eq39_e818_d_b10, eq39_e818_d_b11, eq39_e818_d_b12, eq39_e818_d_b13, eq39_e818_d_b14, eq39_e818_d_b15, eq39_e818_d_b16, eq39_e818_d_b17, eq39_e818_d_b18, eq39_e818_d_b19, eq39_e818_d_b20, eq39_e818_d_b21, eq39_e818_d_b22, eq39_e818_d_b23, eq39_e818_d_b24, eq39_e818_d_b25, eq39_e818_d_b26, eq39_e818_d_b27, eq39_e818_d_b28, eq39_e818_d_b29, eq39_e818_d_b30, eq39_e818_d_b31, eq39_e818_d_b32, eq39_e818_d_b33, eq39_e818_d_b34, eq39_e818_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        let eq39_node_derivatives: [f64; 30] = [eq39_e825_d_n0, eq39_e825_d_n1, eq39_e825_d_n2, eq39_e825_d_n3, eq39_e825_d_n4, eq39_e825_d_n5, eq39_e825_d_n6, eq39_e825_d_n7, eq39_e825_d_n8, eq39_e825_d_n9, eq39_e825_d_n10, eq39_e825_d_n11, eq39_e825_d_n12, eq39_e825_d_n13, eq39_e825_d_n14, eq39_e825_d_n15, eq39_e825_d_n16, eq39_e825_d_n17, eq39_e825_d_n18, eq39_e825_d_n19, eq39_e825_d_n20, eq39_e825_d_n21, eq39_e825_d_n22, eq39_e825_d_n23, eq39_e825_d_n24, eq39_e825_d_n25, eq39_e825_d_n26, eq39_e825_d_n27, eq39_e825_d_n28, eq39_e825_d_n29];
        let eq39_branch_derivatives: [f64; 36] = [eq39_e825_d_b0, eq39_e825_d_b1, eq39_e825_d_b2, eq39_e825_d_b3, eq39_e825_d_b4, eq39_e825_d_b5, eq39_e825_d_b6, eq39_e825_d_b7, eq39_e825_d_b8, eq39_e825_d_b9, eq39_e825_d_b10, eq39_e825_d_b11, eq39_e825_d_b12, eq39_e825_d_b13, eq39_e825_d_b14, eq39_e825_d_b15, eq39_e825_d_b16, eq39_e825_d_b17, eq39_e825_d_b18, eq39_e825_d_b19, eq39_e825_d_b20, eq39_e825_d_b21, eq39_e825_d_b22, eq39_e825_d_b23, eq39_e825_d_b24, eq39_e825_d_b25, eq39_e825_d_b26, eq39_e825_d_b27, eq39_e825_d_b28, eq39_e825_d_b29, eq39_e825_d_b30, eq39_e825_d_b31, eq39_e825_d_b32, eq39_e825_d_b33, eq39_e825_d_b34, eq39_e825_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(17),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e836, eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29, eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35,) = {
    if (!s.b[466]) {
        let eq40_e829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[211]);
        let eq40_e829_d_n0: f64 = (s.dn[211][0] * ddt_scale);
        let eq40_e829_d_n1: f64 = (s.dn[211][1] * ddt_scale);
        let eq40_e829_d_n2: f64 = (s.dn[211][2] * ddt_scale);
        let eq40_e829_d_n3: f64 = (s.dn[211][3] * ddt_scale);
        let eq40_e829_d_n4: f64 = (s.dn[211][4] * ddt_scale);
        let eq40_e829_d_n5: f64 = (s.dn[211][5] * ddt_scale);
        let eq40_e829_d_n6: f64 = (s.dn[211][6] * ddt_scale);
        let eq40_e829_d_n7: f64 = (s.dn[211][7] * ddt_scale);
        let eq40_e829_d_n8: f64 = (s.dn[211][8] * ddt_scale);
        let eq40_e829_d_n9: f64 = (s.dn[211][9] * ddt_scale);
        let eq40_e829_d_n10: f64 = (s.dn[211][10] * ddt_scale);
        let eq40_e829_d_n11: f64 = (s.dn[211][11] * ddt_scale);
        let eq40_e829_d_n12: f64 = (s.dn[211][12] * ddt_scale);
        let eq40_e829_d_n13: f64 = (s.dn[211][13] * ddt_scale);
        let eq40_e829_d_n14: f64 = (s.dn[211][14] * ddt_scale);
        let eq40_e829_d_n15: f64 = (s.dn[211][15] * ddt_scale);
        let eq40_e829_d_n16: f64 = (s.dn[211][16] * ddt_scale);
        let eq40_e829_d_n17: f64 = (s.dn[211][17] * ddt_scale);
        let eq40_e829_d_n18: f64 = (s.dn[211][18] * ddt_scale);
        let eq40_e829_d_n19: f64 = (s.dn[211][19] * ddt_scale);
        let eq40_e829_d_n20: f64 = (s.dn[211][20] * ddt_scale);
        let eq40_e829_d_n21: f64 = (s.dn[211][21] * ddt_scale);
        let eq40_e829_d_n22: f64 = (s.dn[211][22] * ddt_scale);
        let eq40_e829_d_n23: f64 = (s.dn[211][23] * ddt_scale);
        let eq40_e829_d_n24: f64 = (s.dn[211][24] * ddt_scale);
        let eq40_e829_d_n25: f64 = (s.dn[211][25] * ddt_scale);
        let eq40_e829_d_n26: f64 = (s.dn[211][26] * ddt_scale);
        let eq40_e829_d_n27: f64 = (s.dn[211][27] * ddt_scale);
        let eq40_e829_d_n28: f64 = (s.dn[211][28] * ddt_scale);
        let eq40_e829_d_n29: f64 = (s.dn[211][29] * ddt_scale);
        let eq40_e829_d_b0: f64 = (s.db[211][0] * ddt_scale);
        let eq40_e829_d_b1: f64 = (s.db[211][1] * ddt_scale);
        let eq40_e829_d_b2: f64 = (s.db[211][2] * ddt_scale);
        let eq40_e829_d_b3: f64 = (s.db[211][3] * ddt_scale);
        let eq40_e829_d_b4: f64 = (s.db[211][4] * ddt_scale);
        let eq40_e829_d_b5: f64 = (s.db[211][5] * ddt_scale);
        let eq40_e829_d_b6: f64 = (s.db[211][6] * ddt_scale);
        let eq40_e829_d_b7: f64 = (s.db[211][7] * ddt_scale);
        let eq40_e829_d_b8: f64 = (s.db[211][8] * ddt_scale);
        let eq40_e829_d_b9: f64 = (s.db[211][9] * ddt_scale);
        let eq40_e829_d_b10: f64 = (s.db[211][10] * ddt_scale);
        let eq40_e829_d_b11: f64 = (s.db[211][11] * ddt_scale);
        let eq40_e829_d_b12: f64 = (s.db[211][12] * ddt_scale);
        let eq40_e829_d_b13: f64 = (s.db[211][13] * ddt_scale);
        let eq40_e829_d_b14: f64 = (s.db[211][14] * ddt_scale);
        let eq40_e829_d_b15: f64 = (s.db[211][15] * ddt_scale);
        let eq40_e829_d_b16: f64 = (s.db[211][16] * ddt_scale);
        let eq40_e829_d_b17: f64 = (s.db[211][17] * ddt_scale);
        let eq40_e829_d_b18: f64 = (s.db[211][18] * ddt_scale);
        let eq40_e829_d_b19: f64 = (s.db[211][19] * ddt_scale);
        let eq40_e829_d_b20: f64 = (s.db[211][20] * ddt_scale);
        let eq40_e829_d_b21: f64 = (s.db[211][21] * ddt_scale);
        let eq40_e829_d_b22: f64 = (s.db[211][22] * ddt_scale);
        let eq40_e829_d_b23: f64 = (s.db[211][23] * ddt_scale);
        let eq40_e829_d_b24: f64 = (s.db[211][24] * ddt_scale);
        let eq40_e829_d_b25: f64 = (s.db[211][25] * ddt_scale);
        let eq40_e829_d_b26: f64 = (s.db[211][26] * ddt_scale);
        let eq40_e829_d_b27: f64 = (s.db[211][27] * ddt_scale);
        let eq40_e829_d_b28: f64 = (s.db[211][28] * ddt_scale);
        let eq40_e829_d_b29: f64 = (s.db[211][29] * ddt_scale);
        let eq40_e829_d_b30: f64 = (s.db[211][30] * ddt_scale);
        let eq40_e829_d_b31: f64 = (s.db[211][31] * ddt_scale);
        let eq40_e829_d_b32: f64 = (s.db[211][32] * ddt_scale);
        let eq40_e829_d_b33: f64 = (s.db[211][33] * ddt_scale);
        let eq40_e829_d_b34: f64 = (s.db[211][34] * ddt_scale);
        let eq40_e829_d_b35: f64 = (s.db[211][35] * ddt_scale);
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e832_d_n7: f64 = p.p355;
        let eq40_e832_d_n16: f64 = (-p.p355);
        let eq40_e833: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, eq40_e832);
        let eq40_e833_d_n7: f64 = (eq40_e832_d_n7 * ddt_scale);
        let eq40_e833_d_n16: f64 = (eq40_e832_d_n16 * ddt_scale);
        let eq40_e834: f64 = (eq40_e829 + eq40_e833);
        let eq40_e834_d_n7: f64 = (eq40_e829_d_n7 + eq40_e833_d_n7);
        let eq40_e834_d_n16: f64 = (eq40_e829_d_n16 + eq40_e833_d_n16);
        (eq40_e834, eq40_e829_d_n0, eq40_e829_d_n1, eq40_e829_d_n2, eq40_e829_d_n3, eq40_e829_d_n4, eq40_e829_d_n5, eq40_e829_d_n6, eq40_e834_d_n7, eq40_e829_d_n8, eq40_e829_d_n9, eq40_e829_d_n10, eq40_e829_d_n11, eq40_e829_d_n12, eq40_e829_d_n13, eq40_e829_d_n14, eq40_e829_d_n15, eq40_e834_d_n16, eq40_e829_d_n17, eq40_e829_d_n18, eq40_e829_d_n19, eq40_e829_d_n20, eq40_e829_d_n21, eq40_e829_d_n22, eq40_e829_d_n23, eq40_e829_d_n24, eq40_e829_d_n25, eq40_e829_d_n26, eq40_e829_d_n27, eq40_e829_d_n28, eq40_e829_d_n29, eq40_e829_d_b0, eq40_e829_d_b1, eq40_e829_d_b2, eq40_e829_d_b3, eq40_e829_d_b4, eq40_e829_d_b5, eq40_e829_d_b6, eq40_e829_d_b7, eq40_e829_d_b8, eq40_e829_d_b9, eq40_e829_d_b10, eq40_e829_d_b11, eq40_e829_d_b12, eq40_e829_d_b13, eq40_e829_d_b14, eq40_e829_d_b15, eq40_e829_d_b16, eq40_e829_d_b17, eq40_e829_d_b18, eq40_e829_d_b19, eq40_e829_d_b20, eq40_e829_d_b21, eq40_e829_d_b22, eq40_e829_d_b23, eq40_e829_d_b24, eq40_e829_d_b25, eq40_e829_d_b26, eq40_e829_d_b27, eq40_e829_d_b28, eq40_e829_d_b29, eq40_e829_d_b30, eq40_e829_d_b31, eq40_e829_d_b32, eq40_e829_d_b33, eq40_e829_d_b34, eq40_e829_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        let eq40_node_derivatives: [f64; 30] = [eq40_e836_d_n0, eq40_e836_d_n1, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n5, eq40_e836_d_n6, eq40_e836_d_n7, eq40_e836_d_n8, eq40_e836_d_n9, eq40_e836_d_n10, eq40_e836_d_n11, eq40_e836_d_n12, eq40_e836_d_n13, eq40_e836_d_n14, eq40_e836_d_n15, eq40_e836_d_n16, eq40_e836_d_n17, eq40_e836_d_n18, eq40_e836_d_n19, eq40_e836_d_n20, eq40_e836_d_n21, eq40_e836_d_n22, eq40_e836_d_n23, eq40_e836_d_n24, eq40_e836_d_n25, eq40_e836_d_n26, eq40_e836_d_n27, eq40_e836_d_n28, eq40_e836_d_n29];
        let eq40_branch_derivatives: [f64; 36] = [eq40_e836_d_b0, eq40_e836_d_b1, eq40_e836_d_b2, eq40_e836_d_b3, eq40_e836_d_b4, eq40_e836_d_b5, eq40_e836_d_b6, eq40_e836_d_b7, eq40_e836_d_b8, eq40_e836_d_b9, eq40_e836_d_b10, eq40_e836_d_b11, eq40_e836_d_b12, eq40_e836_d_b13, eq40_e836_d_b14, eq40_e836_d_b15, eq40_e836_d_b16, eq40_e836_d_b17, eq40_e836_d_b18, eq40_e836_d_b19, eq40_e836_d_b20, eq40_e836_d_b21, eq40_e836_d_b22, eq40_e836_d_b23, eq40_e836_d_b24, eq40_e836_d_b25, eq40_e836_d_b26, eq40_e836_d_b27, eq40_e836_d_b28, eq40_e836_d_b29, eq40_e836_d_b30, eq40_e836_d_b31, eq40_e836_d_b32, eq40_e836_d_b33, eq40_e836_d_b34, eq40_e836_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e841,) = {
    if (!s.b[466]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e841;
        stamper.stamp_current_const_local(
            Some(7),
            Some(17),
            multiplicity * (eq41_value),
        );
        let (eq42_e846,) = {
    if (!s.b[466]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e846;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq42_value),
        );
        let eq43_e848: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, s.v[212]);
        let eq43_e848_d_n0: f64 = (s.dn[212][0] * ddt_scale);
        let eq43_e848_d_n1: f64 = (s.dn[212][1] * ddt_scale);
        let eq43_e848_d_n2: f64 = (s.dn[212][2] * ddt_scale);
        let eq43_e848_d_n3: f64 = (s.dn[212][3] * ddt_scale);
        let eq43_e848_d_n4: f64 = (s.dn[212][4] * ddt_scale);
        let eq43_e848_d_n5: f64 = (s.dn[212][5] * ddt_scale);
        let eq43_e848_d_n6: f64 = (s.dn[212][6] * ddt_scale);
        let eq43_e848_d_n7: f64 = (s.dn[212][7] * ddt_scale);
        let eq43_e848_d_n8: f64 = (s.dn[212][8] * ddt_scale);
        let eq43_e848_d_n9: f64 = (s.dn[212][9] * ddt_scale);
        let eq43_e848_d_n10: f64 = (s.dn[212][10] * ddt_scale);
        let eq43_e848_d_n11: f64 = (s.dn[212][11] * ddt_scale);
        let eq43_e848_d_n12: f64 = (s.dn[212][12] * ddt_scale);
        let eq43_e848_d_n13: f64 = (s.dn[212][13] * ddt_scale);
        let eq43_e848_d_n14: f64 = (s.dn[212][14] * ddt_scale);
        let eq43_e848_d_n15: f64 = (s.dn[212][15] * ddt_scale);
        let eq43_e848_d_n16: f64 = (s.dn[212][16] * ddt_scale);
        let eq43_e848_d_n17: f64 = (s.dn[212][17] * ddt_scale);
        let eq43_e848_d_n18: f64 = (s.dn[212][18] * ddt_scale);
        let eq43_e848_d_n19: f64 = (s.dn[212][19] * ddt_scale);
        let eq43_e848_d_n20: f64 = (s.dn[212][20] * ddt_scale);
        let eq43_e848_d_n21: f64 = (s.dn[212][21] * ddt_scale);
        let eq43_e848_d_n22: f64 = (s.dn[212][22] * ddt_scale);
        let eq43_e848_d_n23: f64 = (s.dn[212][23] * ddt_scale);
        let eq43_e848_d_n24: f64 = (s.dn[212][24] * ddt_scale);
        let eq43_e848_d_n25: f64 = (s.dn[212][25] * ddt_scale);
        let eq43_e848_d_n26: f64 = (s.dn[212][26] * ddt_scale);
        let eq43_e848_d_n27: f64 = (s.dn[212][27] * ddt_scale);
        let eq43_e848_d_n28: f64 = (s.dn[212][28] * ddt_scale);
        let eq43_e848_d_n29: f64 = (s.dn[212][29] * ddt_scale);
        let eq43_e848_d_b0: f64 = (s.db[212][0] * ddt_scale);
        let eq43_e848_d_b1: f64 = (s.db[212][1] * ddt_scale);
        let eq43_e848_d_b2: f64 = (s.db[212][2] * ddt_scale);
        let eq43_e848_d_b3: f64 = (s.db[212][3] * ddt_scale);
        let eq43_e848_d_b4: f64 = (s.db[212][4] * ddt_scale);
        let eq43_e848_d_b5: f64 = (s.db[212][5] * ddt_scale);
        let eq43_e848_d_b6: f64 = (s.db[212][6] * ddt_scale);
        let eq43_e848_d_b7: f64 = (s.db[212][7] * ddt_scale);
        let eq43_e848_d_b8: f64 = (s.db[212][8] * ddt_scale);
        let eq43_e848_d_b9: f64 = (s.db[212][9] * ddt_scale);
        let eq43_e848_d_b10: f64 = (s.db[212][10] * ddt_scale);
        let eq43_e848_d_b11: f64 = (s.db[212][11] * ddt_scale);
        let eq43_e848_d_b12: f64 = (s.db[212][12] * ddt_scale);
        let eq43_e848_d_b13: f64 = (s.db[212][13] * ddt_scale);
        let eq43_e848_d_b14: f64 = (s.db[212][14] * ddt_scale);
        let eq43_e848_d_b15: f64 = (s.db[212][15] * ddt_scale);
        let eq43_e848_d_b16: f64 = (s.db[212][16] * ddt_scale);
        let eq43_e848_d_b17: f64 = (s.db[212][17] * ddt_scale);
        let eq43_e848_d_b18: f64 = (s.db[212][18] * ddt_scale);
        let eq43_e848_d_b19: f64 = (s.db[212][19] * ddt_scale);
        let eq43_e848_d_b20: f64 = (s.db[212][20] * ddt_scale);
        let eq43_e848_d_b21: f64 = (s.db[212][21] * ddt_scale);
        let eq43_e848_d_b22: f64 = (s.db[212][22] * ddt_scale);
        let eq43_e848_d_b23: f64 = (s.db[212][23] * ddt_scale);
        let eq43_e848_d_b24: f64 = (s.db[212][24] * ddt_scale);
        let eq43_e848_d_b25: f64 = (s.db[212][25] * ddt_scale);
        let eq43_e848_d_b26: f64 = (s.db[212][26] * ddt_scale);
        let eq43_e848_d_b27: f64 = (s.db[212][27] * ddt_scale);
        let eq43_e848_d_b28: f64 = (s.db[212][28] * ddt_scale);
        let eq43_e848_d_b29: f64 = (s.db[212][29] * ddt_scale);
        let eq43_e848_d_b30: f64 = (s.db[212][30] * ddt_scale);
        let eq43_e848_d_b31: f64 = (s.db[212][31] * ddt_scale);
        let eq43_e848_d_b32: f64 = (s.db[212][32] * ddt_scale);
        let eq43_e848_d_b33: f64 = (s.db[212][33] * ddt_scale);
        let eq43_e848_d_b34: f64 = (s.db[212][34] * ddt_scale);
        let eq43_e848_d_b35: f64 = (s.db[212][35] * ddt_scale);
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e851_d_n3: f64 = p.p355;
        let eq43_e851_d_n16: f64 = (-p.p355);
        let eq43_e852: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq43_e851);
        let eq43_e852_d_n3: f64 = (eq43_e851_d_n3 * ddt_scale);
        let eq43_e852_d_n16: f64 = (eq43_e851_d_n16 * ddt_scale);
        let eq43_e853: f64 = (eq43_e848 + eq43_e852);
        let eq43_e853_d_n3: f64 = (eq43_e848_d_n3 + eq43_e852_d_n3);
        let eq43_e853_d_n16: f64 = (eq43_e848_d_n16 + eq43_e852_d_n16);
        let eq43_value: f64 = eq43_e853;
        let eq43_node_derivatives: [f64; 30] = [eq43_e848_d_n0, eq43_e848_d_n1, eq43_e848_d_n2, eq43_e853_d_n3, eq43_e848_d_n4, eq43_e848_d_n5, eq43_e848_d_n6, eq43_e848_d_n7, eq43_e848_d_n8, eq43_e848_d_n9, eq43_e848_d_n10, eq43_e848_d_n11, eq43_e848_d_n12, eq43_e848_d_n13, eq43_e848_d_n14, eq43_e848_d_n15, eq43_e853_d_n16, eq43_e848_d_n17, eq43_e848_d_n18, eq43_e848_d_n19, eq43_e848_d_n20, eq43_e848_d_n21, eq43_e848_d_n22, eq43_e848_d_n23, eq43_e848_d_n24, eq43_e848_d_n25, eq43_e848_d_n26, eq43_e848_d_n27, eq43_e848_d_n28, eq43_e848_d_n29];
        let eq43_branch_derivatives: [f64; 36] = [eq43_e848_d_b0, eq43_e848_d_b1, eq43_e848_d_b2, eq43_e848_d_b3, eq43_e848_d_b4, eq43_e848_d_b5, eq43_e848_d_b6, eq43_e848_d_b7, eq43_e848_d_b8, eq43_e848_d_b9, eq43_e848_d_b10, eq43_e848_d_b11, eq43_e848_d_b12, eq43_e848_d_b13, eq43_e848_d_b14, eq43_e848_d_b15, eq43_e848_d_b16, eq43_e848_d_b17, eq43_e848_d_b18, eq43_e848_d_b19, eq43_e848_d_b20, eq43_e848_d_b21, eq43_e848_d_b22, eq43_e848_d_b23, eq43_e848_d_b24, eq43_e848_d_b25, eq43_e848_d_b26, eq43_e848_d_b27, eq43_e848_d_b28, eq43_e848_d_b29, eq43_e848_d_b30, eq43_e848_d_b31, eq43_e848_d_b32, eq43_e848_d_b33, eq43_e848_d_b34, eq43_e848_d_b35];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(16),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e861, eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29, eq44_e861_d_b0, eq44_e861_d_b1, eq44_e861_d_b2, eq44_e861_d_b3, eq44_e861_d_b4, eq44_e861_d_b5, eq44_e861_d_b6, eq44_e861_d_b7, eq44_e861_d_b8, eq44_e861_d_b9, eq44_e861_d_b10, eq44_e861_d_b11, eq44_e861_d_b12, eq44_e861_d_b13, eq44_e861_d_b14, eq44_e861_d_b15, eq44_e861_d_b16, eq44_e861_d_b17, eq44_e861_d_b18, eq44_e861_d_b19, eq44_e861_d_b20, eq44_e861_d_b21, eq44_e861_d_b22, eq44_e861_d_b23, eq44_e861_d_b24, eq44_e861_d_b25, eq44_e861_d_b26, eq44_e861_d_b27, eq44_e861_d_b28, eq44_e861_d_b29, eq44_e861_d_b30, eq44_e861_d_b31, eq44_e861_d_b32, eq44_e861_d_b33, eq44_e861_d_b34, eq44_e861_d_b35,) = {
    if s.b[467] {
        let eq44_e858: f64 = (s.v[0] * (nv16 - nv15));
        let eq44_e858_d_n15: f64 = (-s.v[0]);
        let eq44_e858_d_n16: f64 = s.v[0];
        let eq44_e859: f64 = (s.v[202] + eq44_e858);
        let eq44_e859_d_n15: f64 = (s.dn[202][15] + eq44_e858_d_n15);
        let eq44_e859_d_n16: f64 = (s.dn[202][16] + eq44_e858_d_n16);
        (eq44_e859, s.dn[202][0], s.dn[202][1], s.dn[202][2], s.dn[202][3], s.dn[202][4], s.dn[202][5], s.dn[202][6], s.dn[202][7], s.dn[202][8], s.dn[202][9], s.dn[202][10], s.dn[202][11], s.dn[202][12], s.dn[202][13], s.dn[202][14], eq44_e859_d_n15, eq44_e859_d_n16, s.dn[202][17], s.dn[202][18], s.dn[202][19], s.dn[202][20], s.dn[202][21], s.dn[202][22], s.dn[202][23], s.dn[202][24], s.dn[202][25], s.dn[202][26], s.dn[202][27], s.dn[202][28], s.dn[202][29], s.db[202][0], s.db[202][1], s.db[202][2], s.db[202][3], s.db[202][4], s.db[202][5], s.db[202][6], s.db[202][7], s.db[202][8], s.db[202][9], s.db[202][10], s.db[202][11], s.db[202][12], s.db[202][13], s.db[202][14], s.db[202][15], s.db[202][16], s.db[202][17], s.db[202][18], s.db[202][19], s.db[202][20], s.db[202][21], s.db[202][22], s.db[202][23], s.db[202][24], s.db[202][25], s.db[202][26], s.db[202][27], s.db[202][28], s.db[202][29], s.db[202][30], s.db[202][31], s.db[202][32], s.db[202][33], s.db[202][34], s.db[202][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        let eq44_node_derivatives: [f64; 30] = [eq44_e861_d_n0, eq44_e861_d_n1, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n5, eq44_e861_d_n6, eq44_e861_d_n7, eq44_e861_d_n8, eq44_e861_d_n9, eq44_e861_d_n10, eq44_e861_d_n11, eq44_e861_d_n12, eq44_e861_d_n13, eq44_e861_d_n14, eq44_e861_d_n15, eq44_e861_d_n16, eq44_e861_d_n17, eq44_e861_d_n18, eq44_e861_d_n19, eq44_e861_d_n20, eq44_e861_d_n21, eq44_e861_d_n22, eq44_e861_d_n23, eq44_e861_d_n24, eq44_e861_d_n25, eq44_e861_d_n26, eq44_e861_d_n27, eq44_e861_d_n28, eq44_e861_d_n29];
        let eq44_branch_derivatives: [f64; 36] = [eq44_e861_d_b0, eq44_e861_d_b1, eq44_e861_d_b2, eq44_e861_d_b3, eq44_e861_d_b4, eq44_e861_d_b5, eq44_e861_d_b6, eq44_e861_d_b7, eq44_e861_d_b8, eq44_e861_d_b9, eq44_e861_d_b10, eq44_e861_d_b11, eq44_e861_d_b12, eq44_e861_d_b13, eq44_e861_d_b14, eq44_e861_d_b15, eq44_e861_d_b16, eq44_e861_d_b17, eq44_e861_d_b18, eq44_e861_d_b19, eq44_e861_d_b20, eq44_e861_d_b21, eq44_e861_d_b22, eq44_e861_d_b23, eq44_e861_d_b24, eq44_e861_d_b25, eq44_e861_d_b26, eq44_e861_d_b27, eq44_e861_d_b28, eq44_e861_d_b29, eq44_e861_d_b30, eq44_e861_d_b31, eq44_e861_d_b32, eq44_e861_d_b33, eq44_e861_d_b34, eq44_e861_d_b35];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e866,) = {
    if (!s.b[467]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e866;
        stamper.stamp_potential_const_local(
            19,
            eq45_value,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq46_e876, eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29, eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35,) = {
    if s.b[613] {
        let eq46_e869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[203]);
        let eq46_e869_d_n0: f64 = (s.dn[203][0] * ddt_scale);
        let eq46_e869_d_n1: f64 = (s.dn[203][1] * ddt_scale);
        let eq46_e869_d_n2: f64 = (s.dn[203][2] * ddt_scale);
        let eq46_e869_d_n3: f64 = (s.dn[203][3] * ddt_scale);
        let eq46_e869_d_n4: f64 = (s.dn[203][4] * ddt_scale);
        let eq46_e869_d_n5: f64 = (s.dn[203][5] * ddt_scale);
        let eq46_e869_d_n6: f64 = (s.dn[203][6] * ddt_scale);
        let eq46_e869_d_n7: f64 = (s.dn[203][7] * ddt_scale);
        let eq46_e869_d_n8: f64 = (s.dn[203][8] * ddt_scale);
        let eq46_e869_d_n9: f64 = (s.dn[203][9] * ddt_scale);
        let eq46_e869_d_n10: f64 = (s.dn[203][10] * ddt_scale);
        let eq46_e869_d_n11: f64 = (s.dn[203][11] * ddt_scale);
        let eq46_e869_d_n12: f64 = (s.dn[203][12] * ddt_scale);
        let eq46_e869_d_n13: f64 = (s.dn[203][13] * ddt_scale);
        let eq46_e869_d_n14: f64 = (s.dn[203][14] * ddt_scale);
        let eq46_e869_d_n15: f64 = (s.dn[203][15] * ddt_scale);
        let eq46_e869_d_n16: f64 = (s.dn[203][16] * ddt_scale);
        let eq46_e869_d_n17: f64 = (s.dn[203][17] * ddt_scale);
        let eq46_e869_d_n18: f64 = (s.dn[203][18] * ddt_scale);
        let eq46_e869_d_n19: f64 = (s.dn[203][19] * ddt_scale);
        let eq46_e869_d_n20: f64 = (s.dn[203][20] * ddt_scale);
        let eq46_e869_d_n21: f64 = (s.dn[203][21] * ddt_scale);
        let eq46_e869_d_n22: f64 = (s.dn[203][22] * ddt_scale);
        let eq46_e869_d_n23: f64 = (s.dn[203][23] * ddt_scale);
        let eq46_e869_d_n24: f64 = (s.dn[203][24] * ddt_scale);
        let eq46_e869_d_n25: f64 = (s.dn[203][25] * ddt_scale);
        let eq46_e869_d_n26: f64 = (s.dn[203][26] * ddt_scale);
        let eq46_e869_d_n27: f64 = (s.dn[203][27] * ddt_scale);
        let eq46_e869_d_n28: f64 = (s.dn[203][28] * ddt_scale);
        let eq46_e869_d_n29: f64 = (s.dn[203][29] * ddt_scale);
        let eq46_e869_d_b0: f64 = (s.db[203][0] * ddt_scale);
        let eq46_e869_d_b1: f64 = (s.db[203][1] * ddt_scale);
        let eq46_e869_d_b2: f64 = (s.db[203][2] * ddt_scale);
        let eq46_e869_d_b3: f64 = (s.db[203][3] * ddt_scale);
        let eq46_e869_d_b4: f64 = (s.db[203][4] * ddt_scale);
        let eq46_e869_d_b5: f64 = (s.db[203][5] * ddt_scale);
        let eq46_e869_d_b6: f64 = (s.db[203][6] * ddt_scale);
        let eq46_e869_d_b7: f64 = (s.db[203][7] * ddt_scale);
        let eq46_e869_d_b8: f64 = (s.db[203][8] * ddt_scale);
        let eq46_e869_d_b9: f64 = (s.db[203][9] * ddt_scale);
        let eq46_e869_d_b10: f64 = (s.db[203][10] * ddt_scale);
        let eq46_e869_d_b11: f64 = (s.db[203][11] * ddt_scale);
        let eq46_e869_d_b12: f64 = (s.db[203][12] * ddt_scale);
        let eq46_e869_d_b13: f64 = (s.db[203][13] * ddt_scale);
        let eq46_e869_d_b14: f64 = (s.db[203][14] * ddt_scale);
        let eq46_e869_d_b15: f64 = (s.db[203][15] * ddt_scale);
        let eq46_e869_d_b16: f64 = (s.db[203][16] * ddt_scale);
        let eq46_e869_d_b17: f64 = (s.db[203][17] * ddt_scale);
        let eq46_e869_d_b18: f64 = (s.db[203][18] * ddt_scale);
        let eq46_e869_d_b19: f64 = (s.db[203][19] * ddt_scale);
        let eq46_e869_d_b20: f64 = (s.db[203][20] * ddt_scale);
        let eq46_e869_d_b21: f64 = (s.db[203][21] * ddt_scale);
        let eq46_e869_d_b22: f64 = (s.db[203][22] * ddt_scale);
        let eq46_e869_d_b23: f64 = (s.db[203][23] * ddt_scale);
        let eq46_e869_d_b24: f64 = (s.db[203][24] * ddt_scale);
        let eq46_e869_d_b25: f64 = (s.db[203][25] * ddt_scale);
        let eq46_e869_d_b26: f64 = (s.db[203][26] * ddt_scale);
        let eq46_e869_d_b27: f64 = (s.db[203][27] * ddt_scale);
        let eq46_e869_d_b28: f64 = (s.db[203][28] * ddt_scale);
        let eq46_e869_d_b29: f64 = (s.db[203][29] * ddt_scale);
        let eq46_e869_d_b30: f64 = (s.db[203][30] * ddt_scale);
        let eq46_e869_d_b31: f64 = (s.db[203][31] * ddt_scale);
        let eq46_e869_d_b32: f64 = (s.db[203][32] * ddt_scale);
        let eq46_e869_d_b33: f64 = (s.db[203][33] * ddt_scale);
        let eq46_e869_d_b34: f64 = (s.db[203][34] * ddt_scale);
        let eq46_e869_d_b35: f64 = (s.db[203][35] * ddt_scale);
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e872_d_n7: f64 = p.p355;
        let eq46_e872_d_n15: f64 = (-p.p355);
        let eq46_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, eq46_e872);
        let eq46_e873_d_n7: f64 = (eq46_e872_d_n7 * ddt_scale);
        let eq46_e873_d_n15: f64 = (eq46_e872_d_n15 * ddt_scale);
        let eq46_e874: f64 = (eq46_e869 + eq46_e873);
        let eq46_e874_d_n7: f64 = (eq46_e869_d_n7 + eq46_e873_d_n7);
        let eq46_e874_d_n15: f64 = (eq46_e869_d_n15 + eq46_e873_d_n15);
        (eq46_e874, eq46_e869_d_n0, eq46_e869_d_n1, eq46_e869_d_n2, eq46_e869_d_n3, eq46_e869_d_n4, eq46_e869_d_n5, eq46_e869_d_n6, eq46_e874_d_n7, eq46_e869_d_n8, eq46_e869_d_n9, eq46_e869_d_n10, eq46_e869_d_n11, eq46_e869_d_n12, eq46_e869_d_n13, eq46_e869_d_n14, eq46_e874_d_n15, eq46_e869_d_n16, eq46_e869_d_n17, eq46_e869_d_n18, eq46_e869_d_n19, eq46_e869_d_n20, eq46_e869_d_n21, eq46_e869_d_n22, eq46_e869_d_n23, eq46_e869_d_n24, eq46_e869_d_n25, eq46_e869_d_n26, eq46_e869_d_n27, eq46_e869_d_n28, eq46_e869_d_n29, eq46_e869_d_b0, eq46_e869_d_b1, eq46_e869_d_b2, eq46_e869_d_b3, eq46_e869_d_b4, eq46_e869_d_b5, eq46_e869_d_b6, eq46_e869_d_b7, eq46_e869_d_b8, eq46_e869_d_b9, eq46_e869_d_b10, eq46_e869_d_b11, eq46_e869_d_b12, eq46_e869_d_b13, eq46_e869_d_b14, eq46_e869_d_b15, eq46_e869_d_b16, eq46_e869_d_b17, eq46_e869_d_b18, eq46_e869_d_b19, eq46_e869_d_b20, eq46_e869_d_b21, eq46_e869_d_b22, eq46_e869_d_b23, eq46_e869_d_b24, eq46_e869_d_b25, eq46_e869_d_b26, eq46_e869_d_b27, eq46_e869_d_b28, eq46_e869_d_b29, eq46_e869_d_b30, eq46_e869_d_b31, eq46_e869_d_b32, eq46_e869_d_b33, eq46_e869_d_b34, eq46_e869_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        let eq46_node_derivatives: [f64; 30] = [eq46_e876_d_n0, eq46_e876_d_n1, eq46_e876_d_n2, eq46_e876_d_n3, eq46_e876_d_n4, eq46_e876_d_n5, eq46_e876_d_n6, eq46_e876_d_n7, eq46_e876_d_n8, eq46_e876_d_n9, eq46_e876_d_n10, eq46_e876_d_n11, eq46_e876_d_n12, eq46_e876_d_n13, eq46_e876_d_n14, eq46_e876_d_n15, eq46_e876_d_n16, eq46_e876_d_n17, eq46_e876_d_n18, eq46_e876_d_n19, eq46_e876_d_n20, eq46_e876_d_n21, eq46_e876_d_n22, eq46_e876_d_n23, eq46_e876_d_n24, eq46_e876_d_n25, eq46_e876_d_n26, eq46_e876_d_n27, eq46_e876_d_n28, eq46_e876_d_n29];
        let eq46_branch_derivatives: [f64; 36] = [eq46_e876_d_b0, eq46_e876_d_b1, eq46_e876_d_b2, eq46_e876_d_b3, eq46_e876_d_b4, eq46_e876_d_b5, eq46_e876_d_b6, eq46_e876_d_b7, eq46_e876_d_b8, eq46_e876_d_b9, eq46_e876_d_b10, eq46_e876_d_b11, eq46_e876_d_b12, eq46_e876_d_b13, eq46_e876_d_b14, eq46_e876_d_b15, eq46_e876_d_b16, eq46_e876_d_b17, eq46_e876_d_b18, eq46_e876_d_b19, eq46_e876_d_b20, eq46_e876_d_b21, eq46_e876_d_b22, eq46_e876_d_b23, eq46_e876_d_b24, eq46_e876_d_b25, eq46_e876_d_b26, eq46_e876_d_b27, eq46_e876_d_b28, eq46_e876_d_b29, eq46_e876_d_b30, eq46_e876_d_b31, eq46_e876_d_b32, eq46_e876_d_b33, eq46_e876_d_b34, eq46_e876_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e886, eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29, eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35,) = {
    if s.b[613] {
        let eq47_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[204]);
        let eq47_e879_d_n0: f64 = (s.dn[204][0] * ddt_scale);
        let eq47_e879_d_n1: f64 = (s.dn[204][1] * ddt_scale);
        let eq47_e879_d_n2: f64 = (s.dn[204][2] * ddt_scale);
        let eq47_e879_d_n3: f64 = (s.dn[204][3] * ddt_scale);
        let eq47_e879_d_n4: f64 = (s.dn[204][4] * ddt_scale);
        let eq47_e879_d_n5: f64 = (s.dn[204][5] * ddt_scale);
        let eq47_e879_d_n6: f64 = (s.dn[204][6] * ddt_scale);
        let eq47_e879_d_n7: f64 = (s.dn[204][7] * ddt_scale);
        let eq47_e879_d_n8: f64 = (s.dn[204][8] * ddt_scale);
        let eq47_e879_d_n9: f64 = (s.dn[204][9] * ddt_scale);
        let eq47_e879_d_n10: f64 = (s.dn[204][10] * ddt_scale);
        let eq47_e879_d_n11: f64 = (s.dn[204][11] * ddt_scale);
        let eq47_e879_d_n12: f64 = (s.dn[204][12] * ddt_scale);
        let eq47_e879_d_n13: f64 = (s.dn[204][13] * ddt_scale);
        let eq47_e879_d_n14: f64 = (s.dn[204][14] * ddt_scale);
        let eq47_e879_d_n15: f64 = (s.dn[204][15] * ddt_scale);
        let eq47_e879_d_n16: f64 = (s.dn[204][16] * ddt_scale);
        let eq47_e879_d_n17: f64 = (s.dn[204][17] * ddt_scale);
        let eq47_e879_d_n18: f64 = (s.dn[204][18] * ddt_scale);
        let eq47_e879_d_n19: f64 = (s.dn[204][19] * ddt_scale);
        let eq47_e879_d_n20: f64 = (s.dn[204][20] * ddt_scale);
        let eq47_e879_d_n21: f64 = (s.dn[204][21] * ddt_scale);
        let eq47_e879_d_n22: f64 = (s.dn[204][22] * ddt_scale);
        let eq47_e879_d_n23: f64 = (s.dn[204][23] * ddt_scale);
        let eq47_e879_d_n24: f64 = (s.dn[204][24] * ddt_scale);
        let eq47_e879_d_n25: f64 = (s.dn[204][25] * ddt_scale);
        let eq47_e879_d_n26: f64 = (s.dn[204][26] * ddt_scale);
        let eq47_e879_d_n27: f64 = (s.dn[204][27] * ddt_scale);
        let eq47_e879_d_n28: f64 = (s.dn[204][28] * ddt_scale);
        let eq47_e879_d_n29: f64 = (s.dn[204][29] * ddt_scale);
        let eq47_e879_d_b0: f64 = (s.db[204][0] * ddt_scale);
        let eq47_e879_d_b1: f64 = (s.db[204][1] * ddt_scale);
        let eq47_e879_d_b2: f64 = (s.db[204][2] * ddt_scale);
        let eq47_e879_d_b3: f64 = (s.db[204][3] * ddt_scale);
        let eq47_e879_d_b4: f64 = (s.db[204][4] * ddt_scale);
        let eq47_e879_d_b5: f64 = (s.db[204][5] * ddt_scale);
        let eq47_e879_d_b6: f64 = (s.db[204][6] * ddt_scale);
        let eq47_e879_d_b7: f64 = (s.db[204][7] * ddt_scale);
        let eq47_e879_d_b8: f64 = (s.db[204][8] * ddt_scale);
        let eq47_e879_d_b9: f64 = (s.db[204][9] * ddt_scale);
        let eq47_e879_d_b10: f64 = (s.db[204][10] * ddt_scale);
        let eq47_e879_d_b11: f64 = (s.db[204][11] * ddt_scale);
        let eq47_e879_d_b12: f64 = (s.db[204][12] * ddt_scale);
        let eq47_e879_d_b13: f64 = (s.db[204][13] * ddt_scale);
        let eq47_e879_d_b14: f64 = (s.db[204][14] * ddt_scale);
        let eq47_e879_d_b15: f64 = (s.db[204][15] * ddt_scale);
        let eq47_e879_d_b16: f64 = (s.db[204][16] * ddt_scale);
        let eq47_e879_d_b17: f64 = (s.db[204][17] * ddt_scale);
        let eq47_e879_d_b18: f64 = (s.db[204][18] * ddt_scale);
        let eq47_e879_d_b19: f64 = (s.db[204][19] * ddt_scale);
        let eq47_e879_d_b20: f64 = (s.db[204][20] * ddt_scale);
        let eq47_e879_d_b21: f64 = (s.db[204][21] * ddt_scale);
        let eq47_e879_d_b22: f64 = (s.db[204][22] * ddt_scale);
        let eq47_e879_d_b23: f64 = (s.db[204][23] * ddt_scale);
        let eq47_e879_d_b24: f64 = (s.db[204][24] * ddt_scale);
        let eq47_e879_d_b25: f64 = (s.db[204][25] * ddt_scale);
        let eq47_e879_d_b26: f64 = (s.db[204][26] * ddt_scale);
        let eq47_e879_d_b27: f64 = (s.db[204][27] * ddt_scale);
        let eq47_e879_d_b28: f64 = (s.db[204][28] * ddt_scale);
        let eq47_e879_d_b29: f64 = (s.db[204][29] * ddt_scale);
        let eq47_e879_d_b30: f64 = (s.db[204][30] * ddt_scale);
        let eq47_e879_d_b31: f64 = (s.db[204][31] * ddt_scale);
        let eq47_e879_d_b32: f64 = (s.db[204][32] * ddt_scale);
        let eq47_e879_d_b33: f64 = (s.db[204][33] * ddt_scale);
        let eq47_e879_d_b34: f64 = (s.db[204][34] * ddt_scale);
        let eq47_e879_d_b35: f64 = (s.db[204][35] * ddt_scale);
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e882_d_n7: f64 = p.p355;
        let eq47_e882_d_n16: f64 = (-p.p355);
        let eq47_e883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, eq47_e882);
        let eq47_e883_d_n7: f64 = (eq47_e882_d_n7 * ddt_scale);
        let eq47_e883_d_n16: f64 = (eq47_e882_d_n16 * ddt_scale);
        let eq47_e884: f64 = (eq47_e879 + eq47_e883);
        let eq47_e884_d_n7: f64 = (eq47_e879_d_n7 + eq47_e883_d_n7);
        let eq47_e884_d_n16: f64 = (eq47_e879_d_n16 + eq47_e883_d_n16);
        (eq47_e884, eq47_e879_d_n0, eq47_e879_d_n1, eq47_e879_d_n2, eq47_e879_d_n3, eq47_e879_d_n4, eq47_e879_d_n5, eq47_e879_d_n6, eq47_e884_d_n7, eq47_e879_d_n8, eq47_e879_d_n9, eq47_e879_d_n10, eq47_e879_d_n11, eq47_e879_d_n12, eq47_e879_d_n13, eq47_e879_d_n14, eq47_e879_d_n15, eq47_e884_d_n16, eq47_e879_d_n17, eq47_e879_d_n18, eq47_e879_d_n19, eq47_e879_d_n20, eq47_e879_d_n21, eq47_e879_d_n22, eq47_e879_d_n23, eq47_e879_d_n24, eq47_e879_d_n25, eq47_e879_d_n26, eq47_e879_d_n27, eq47_e879_d_n28, eq47_e879_d_n29, eq47_e879_d_b0, eq47_e879_d_b1, eq47_e879_d_b2, eq47_e879_d_b3, eq47_e879_d_b4, eq47_e879_d_b5, eq47_e879_d_b6, eq47_e879_d_b7, eq47_e879_d_b8, eq47_e879_d_b9, eq47_e879_d_b10, eq47_e879_d_b11, eq47_e879_d_b12, eq47_e879_d_b13, eq47_e879_d_b14, eq47_e879_d_b15, eq47_e879_d_b16, eq47_e879_d_b17, eq47_e879_d_b18, eq47_e879_d_b19, eq47_e879_d_b20, eq47_e879_d_b21, eq47_e879_d_b22, eq47_e879_d_b23, eq47_e879_d_b24, eq47_e879_d_b25, eq47_e879_d_b26, eq47_e879_d_b27, eq47_e879_d_b28, eq47_e879_d_b29, eq47_e879_d_b30, eq47_e879_d_b31, eq47_e879_d_b32, eq47_e879_d_b33, eq47_e879_d_b34, eq47_e879_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        let eq47_node_derivatives: [f64; 30] = [eq47_e886_d_n0, eq47_e886_d_n1, eq47_e886_d_n2, eq47_e886_d_n3, eq47_e886_d_n4, eq47_e886_d_n5, eq47_e886_d_n6, eq47_e886_d_n7, eq47_e886_d_n8, eq47_e886_d_n9, eq47_e886_d_n10, eq47_e886_d_n11, eq47_e886_d_n12, eq47_e886_d_n13, eq47_e886_d_n14, eq47_e886_d_n15, eq47_e886_d_n16, eq47_e886_d_n17, eq47_e886_d_n18, eq47_e886_d_n19, eq47_e886_d_n20, eq47_e886_d_n21, eq47_e886_d_n22, eq47_e886_d_n23, eq47_e886_d_n24, eq47_e886_d_n25, eq47_e886_d_n26, eq47_e886_d_n27, eq47_e886_d_n28, eq47_e886_d_n29];
        let eq47_branch_derivatives: [f64; 36] = [eq47_e886_d_b0, eq47_e886_d_b1, eq47_e886_d_b2, eq47_e886_d_b3, eq47_e886_d_b4, eq47_e886_d_b5, eq47_e886_d_b6, eq47_e886_d_b7, eq47_e886_d_b8, eq47_e886_d_b9, eq47_e886_d_b10, eq47_e886_d_b11, eq47_e886_d_b12, eq47_e886_d_b13, eq47_e886_d_b14, eq47_e886_d_b15, eq47_e886_d_b16, eq47_e886_d_b17, eq47_e886_d_b18, eq47_e886_d_b19, eq47_e886_d_b20, eq47_e886_d_b21, eq47_e886_d_b22, eq47_e886_d_b23, eq47_e886_d_b24, eq47_e886_d_b25, eq47_e886_d_b26, eq47_e886_d_b27, eq47_e886_d_b28, eq47_e886_d_b29, eq47_e886_d_b30, eq47_e886_d_b31, eq47_e886_d_b32, eq47_e886_d_b33, eq47_e886_d_b34, eq47_e886_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(16),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e896, eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29, eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35,) = {
    if s.b[613] {
        let eq48_e889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 24, s.v[205]);
        let eq48_e889_d_n0: f64 = (s.dn[205][0] * ddt_scale);
        let eq48_e889_d_n1: f64 = (s.dn[205][1] * ddt_scale);
        let eq48_e889_d_n2: f64 = (s.dn[205][2] * ddt_scale);
        let eq48_e889_d_n3: f64 = (s.dn[205][3] * ddt_scale);
        let eq48_e889_d_n4: f64 = (s.dn[205][4] * ddt_scale);
        let eq48_e889_d_n5: f64 = (s.dn[205][5] * ddt_scale);
        let eq48_e889_d_n6: f64 = (s.dn[205][6] * ddt_scale);
        let eq48_e889_d_n7: f64 = (s.dn[205][7] * ddt_scale);
        let eq48_e889_d_n8: f64 = (s.dn[205][8] * ddt_scale);
        let eq48_e889_d_n9: f64 = (s.dn[205][9] * ddt_scale);
        let eq48_e889_d_n10: f64 = (s.dn[205][10] * ddt_scale);
        let eq48_e889_d_n11: f64 = (s.dn[205][11] * ddt_scale);
        let eq48_e889_d_n12: f64 = (s.dn[205][12] * ddt_scale);
        let eq48_e889_d_n13: f64 = (s.dn[205][13] * ddt_scale);
        let eq48_e889_d_n14: f64 = (s.dn[205][14] * ddt_scale);
        let eq48_e889_d_n15: f64 = (s.dn[205][15] * ddt_scale);
        let eq48_e889_d_n16: f64 = (s.dn[205][16] * ddt_scale);
        let eq48_e889_d_n17: f64 = (s.dn[205][17] * ddt_scale);
        let eq48_e889_d_n18: f64 = (s.dn[205][18] * ddt_scale);
        let eq48_e889_d_n19: f64 = (s.dn[205][19] * ddt_scale);
        let eq48_e889_d_n20: f64 = (s.dn[205][20] * ddt_scale);
        let eq48_e889_d_n21: f64 = (s.dn[205][21] * ddt_scale);
        let eq48_e889_d_n22: f64 = (s.dn[205][22] * ddt_scale);
        let eq48_e889_d_n23: f64 = (s.dn[205][23] * ddt_scale);
        let eq48_e889_d_n24: f64 = (s.dn[205][24] * ddt_scale);
        let eq48_e889_d_n25: f64 = (s.dn[205][25] * ddt_scale);
        let eq48_e889_d_n26: f64 = (s.dn[205][26] * ddt_scale);
        let eq48_e889_d_n27: f64 = (s.dn[205][27] * ddt_scale);
        let eq48_e889_d_n28: f64 = (s.dn[205][28] * ddt_scale);
        let eq48_e889_d_n29: f64 = (s.dn[205][29] * ddt_scale);
        let eq48_e889_d_b0: f64 = (s.db[205][0] * ddt_scale);
        let eq48_e889_d_b1: f64 = (s.db[205][1] * ddt_scale);
        let eq48_e889_d_b2: f64 = (s.db[205][2] * ddt_scale);
        let eq48_e889_d_b3: f64 = (s.db[205][3] * ddt_scale);
        let eq48_e889_d_b4: f64 = (s.db[205][4] * ddt_scale);
        let eq48_e889_d_b5: f64 = (s.db[205][5] * ddt_scale);
        let eq48_e889_d_b6: f64 = (s.db[205][6] * ddt_scale);
        let eq48_e889_d_b7: f64 = (s.db[205][7] * ddt_scale);
        let eq48_e889_d_b8: f64 = (s.db[205][8] * ddt_scale);
        let eq48_e889_d_b9: f64 = (s.db[205][9] * ddt_scale);
        let eq48_e889_d_b10: f64 = (s.db[205][10] * ddt_scale);
        let eq48_e889_d_b11: f64 = (s.db[205][11] * ddt_scale);
        let eq48_e889_d_b12: f64 = (s.db[205][12] * ddt_scale);
        let eq48_e889_d_b13: f64 = (s.db[205][13] * ddt_scale);
        let eq48_e889_d_b14: f64 = (s.db[205][14] * ddt_scale);
        let eq48_e889_d_b15: f64 = (s.db[205][15] * ddt_scale);
        let eq48_e889_d_b16: f64 = (s.db[205][16] * ddt_scale);
        let eq48_e889_d_b17: f64 = (s.db[205][17] * ddt_scale);
        let eq48_e889_d_b18: f64 = (s.db[205][18] * ddt_scale);
        let eq48_e889_d_b19: f64 = (s.db[205][19] * ddt_scale);
        let eq48_e889_d_b20: f64 = (s.db[205][20] * ddt_scale);
        let eq48_e889_d_b21: f64 = (s.db[205][21] * ddt_scale);
        let eq48_e889_d_b22: f64 = (s.db[205][22] * ddt_scale);
        let eq48_e889_d_b23: f64 = (s.db[205][23] * ddt_scale);
        let eq48_e889_d_b24: f64 = (s.db[205][24] * ddt_scale);
        let eq48_e889_d_b25: f64 = (s.db[205][25] * ddt_scale);
        let eq48_e889_d_b26: f64 = (s.db[205][26] * ddt_scale);
        let eq48_e889_d_b27: f64 = (s.db[205][27] * ddt_scale);
        let eq48_e889_d_b28: f64 = (s.db[205][28] * ddt_scale);
        let eq48_e889_d_b29: f64 = (s.db[205][29] * ddt_scale);
        let eq48_e889_d_b30: f64 = (s.db[205][30] * ddt_scale);
        let eq48_e889_d_b31: f64 = (s.db[205][31] * ddt_scale);
        let eq48_e889_d_b32: f64 = (s.db[205][32] * ddt_scale);
        let eq48_e889_d_b33: f64 = (s.db[205][33] * ddt_scale);
        let eq48_e889_d_b34: f64 = (s.db[205][34] * ddt_scale);
        let eq48_e889_d_b35: f64 = (s.db[205][35] * ddt_scale);
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e892_d_n2: f64 = p.p355;
        let eq48_e892_d_n15: f64 = (-p.p355);
        let eq48_e893: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 25, eq48_e892);
        let eq48_e893_d_n2: f64 = (eq48_e892_d_n2 * ddt_scale);
        let eq48_e893_d_n15: f64 = (eq48_e892_d_n15 * ddt_scale);
        let eq48_e894: f64 = (eq48_e889 + eq48_e893);
        let eq48_e894_d_n2: f64 = (eq48_e889_d_n2 + eq48_e893_d_n2);
        let eq48_e894_d_n15: f64 = (eq48_e889_d_n15 + eq48_e893_d_n15);
        (eq48_e894, eq48_e889_d_n0, eq48_e889_d_n1, eq48_e894_d_n2, eq48_e889_d_n3, eq48_e889_d_n4, eq48_e889_d_n5, eq48_e889_d_n6, eq48_e889_d_n7, eq48_e889_d_n8, eq48_e889_d_n9, eq48_e889_d_n10, eq48_e889_d_n11, eq48_e889_d_n12, eq48_e889_d_n13, eq48_e889_d_n14, eq48_e894_d_n15, eq48_e889_d_n16, eq48_e889_d_n17, eq48_e889_d_n18, eq48_e889_d_n19, eq48_e889_d_n20, eq48_e889_d_n21, eq48_e889_d_n22, eq48_e889_d_n23, eq48_e889_d_n24, eq48_e889_d_n25, eq48_e889_d_n26, eq48_e889_d_n27, eq48_e889_d_n28, eq48_e889_d_n29, eq48_e889_d_b0, eq48_e889_d_b1, eq48_e889_d_b2, eq48_e889_d_b3, eq48_e889_d_b4, eq48_e889_d_b5, eq48_e889_d_b6, eq48_e889_d_b7, eq48_e889_d_b8, eq48_e889_d_b9, eq48_e889_d_b10, eq48_e889_d_b11, eq48_e889_d_b12, eq48_e889_d_b13, eq48_e889_d_b14, eq48_e889_d_b15, eq48_e889_d_b16, eq48_e889_d_b17, eq48_e889_d_b18, eq48_e889_d_b19, eq48_e889_d_b20, eq48_e889_d_b21, eq48_e889_d_b22, eq48_e889_d_b23, eq48_e889_d_b24, eq48_e889_d_b25, eq48_e889_d_b26, eq48_e889_d_b27, eq48_e889_d_b28, eq48_e889_d_b29, eq48_e889_d_b30, eq48_e889_d_b31, eq48_e889_d_b32, eq48_e889_d_b33, eq48_e889_d_b34, eq48_e889_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        let eq48_node_derivatives: [f64; 30] = [eq48_e896_d_n0, eq48_e896_d_n1, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n5, eq48_e896_d_n6, eq48_e896_d_n7, eq48_e896_d_n8, eq48_e896_d_n9, eq48_e896_d_n10, eq48_e896_d_n11, eq48_e896_d_n12, eq48_e896_d_n13, eq48_e896_d_n14, eq48_e896_d_n15, eq48_e896_d_n16, eq48_e896_d_n17, eq48_e896_d_n18, eq48_e896_d_n19, eq48_e896_d_n20, eq48_e896_d_n21, eq48_e896_d_n22, eq48_e896_d_n23, eq48_e896_d_n24, eq48_e896_d_n25, eq48_e896_d_n26, eq48_e896_d_n27, eq48_e896_d_n28, eq48_e896_d_n29];
        let eq48_branch_derivatives: [f64; 36] = [eq48_e896_d_b0, eq48_e896_d_b1, eq48_e896_d_b2, eq48_e896_d_b3, eq48_e896_d_b4, eq48_e896_d_b5, eq48_e896_d_b6, eq48_e896_d_b7, eq48_e896_d_b8, eq48_e896_d_b9, eq48_e896_d_b10, eq48_e896_d_b11, eq48_e896_d_b12, eq48_e896_d_b13, eq48_e896_d_b14, eq48_e896_d_b15, eq48_e896_d_b16, eq48_e896_d_b17, eq48_e896_d_b18, eq48_e896_d_b19, eq48_e896_d_b20, eq48_e896_d_b21, eq48_e896_d_b22, eq48_e896_d_b23, eq48_e896_d_b24, eq48_e896_d_b25, eq48_e896_d_b26, eq48_e896_d_b27, eq48_e896_d_b28, eq48_e896_d_b29, eq48_e896_d_b30, eq48_e896_d_b31, eq48_e896_d_b32, eq48_e896_d_b33, eq48_e896_d_b34, eq48_e896_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e900,) = {
    if s.b[613] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e900;
        stamper.stamp_current_const_local(
            Some(2),
            Some(16),
            multiplicity * (eq49_value),
        );
        let (eq50_e910, eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29, eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35,) = {
    if s.b[613] {
        let eq50_e903: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 26, s.v[207]);
        let eq50_e903_d_n0: f64 = (s.dn[207][0] * ddt_scale);
        let eq50_e903_d_n1: f64 = (s.dn[207][1] * ddt_scale);
        let eq50_e903_d_n2: f64 = (s.dn[207][2] * ddt_scale);
        let eq50_e903_d_n3: f64 = (s.dn[207][3] * ddt_scale);
        let eq50_e903_d_n4: f64 = (s.dn[207][4] * ddt_scale);
        let eq50_e903_d_n5: f64 = (s.dn[207][5] * ddt_scale);
        let eq50_e903_d_n6: f64 = (s.dn[207][6] * ddt_scale);
        let eq50_e903_d_n7: f64 = (s.dn[207][7] * ddt_scale);
        let eq50_e903_d_n8: f64 = (s.dn[207][8] * ddt_scale);
        let eq50_e903_d_n9: f64 = (s.dn[207][9] * ddt_scale);
        let eq50_e903_d_n10: f64 = (s.dn[207][10] * ddt_scale);
        let eq50_e903_d_n11: f64 = (s.dn[207][11] * ddt_scale);
        let eq50_e903_d_n12: f64 = (s.dn[207][12] * ddt_scale);
        let eq50_e903_d_n13: f64 = (s.dn[207][13] * ddt_scale);
        let eq50_e903_d_n14: f64 = (s.dn[207][14] * ddt_scale);
        let eq50_e903_d_n15: f64 = (s.dn[207][15] * ddt_scale);
        let eq50_e903_d_n16: f64 = (s.dn[207][16] * ddt_scale);
        let eq50_e903_d_n17: f64 = (s.dn[207][17] * ddt_scale);
        let eq50_e903_d_n18: f64 = (s.dn[207][18] * ddt_scale);
        let eq50_e903_d_n19: f64 = (s.dn[207][19] * ddt_scale);
        let eq50_e903_d_n20: f64 = (s.dn[207][20] * ddt_scale);
        let eq50_e903_d_n21: f64 = (s.dn[207][21] * ddt_scale);
        let eq50_e903_d_n22: f64 = (s.dn[207][22] * ddt_scale);
        let eq50_e903_d_n23: f64 = (s.dn[207][23] * ddt_scale);
        let eq50_e903_d_n24: f64 = (s.dn[207][24] * ddt_scale);
        let eq50_e903_d_n25: f64 = (s.dn[207][25] * ddt_scale);
        let eq50_e903_d_n26: f64 = (s.dn[207][26] * ddt_scale);
        let eq50_e903_d_n27: f64 = (s.dn[207][27] * ddt_scale);
        let eq50_e903_d_n28: f64 = (s.dn[207][28] * ddt_scale);
        let eq50_e903_d_n29: f64 = (s.dn[207][29] * ddt_scale);
        let eq50_e903_d_b0: f64 = (s.db[207][0] * ddt_scale);
        let eq50_e903_d_b1: f64 = (s.db[207][1] * ddt_scale);
        let eq50_e903_d_b2: f64 = (s.db[207][2] * ddt_scale);
        let eq50_e903_d_b3: f64 = (s.db[207][3] * ddt_scale);
        let eq50_e903_d_b4: f64 = (s.db[207][4] * ddt_scale);
        let eq50_e903_d_b5: f64 = (s.db[207][5] * ddt_scale);
        let eq50_e903_d_b6: f64 = (s.db[207][6] * ddt_scale);
        let eq50_e903_d_b7: f64 = (s.db[207][7] * ddt_scale);
        let eq50_e903_d_b8: f64 = (s.db[207][8] * ddt_scale);
        let eq50_e903_d_b9: f64 = (s.db[207][9] * ddt_scale);
        let eq50_e903_d_b10: f64 = (s.db[207][10] * ddt_scale);
        let eq50_e903_d_b11: f64 = (s.db[207][11] * ddt_scale);
        let eq50_e903_d_b12: f64 = (s.db[207][12] * ddt_scale);
        let eq50_e903_d_b13: f64 = (s.db[207][13] * ddt_scale);
        let eq50_e903_d_b14: f64 = (s.db[207][14] * ddt_scale);
        let eq50_e903_d_b15: f64 = (s.db[207][15] * ddt_scale);
        let eq50_e903_d_b16: f64 = (s.db[207][16] * ddt_scale);
        let eq50_e903_d_b17: f64 = (s.db[207][17] * ddt_scale);
        let eq50_e903_d_b18: f64 = (s.db[207][18] * ddt_scale);
        let eq50_e903_d_b19: f64 = (s.db[207][19] * ddt_scale);
        let eq50_e903_d_b20: f64 = (s.db[207][20] * ddt_scale);
        let eq50_e903_d_b21: f64 = (s.db[207][21] * ddt_scale);
        let eq50_e903_d_b22: f64 = (s.db[207][22] * ddt_scale);
        let eq50_e903_d_b23: f64 = (s.db[207][23] * ddt_scale);
        let eq50_e903_d_b24: f64 = (s.db[207][24] * ddt_scale);
        let eq50_e903_d_b25: f64 = (s.db[207][25] * ddt_scale);
        let eq50_e903_d_b26: f64 = (s.db[207][26] * ddt_scale);
        let eq50_e903_d_b27: f64 = (s.db[207][27] * ddt_scale);
        let eq50_e903_d_b28: f64 = (s.db[207][28] * ddt_scale);
        let eq50_e903_d_b29: f64 = (s.db[207][29] * ddt_scale);
        let eq50_e903_d_b30: f64 = (s.db[207][30] * ddt_scale);
        let eq50_e903_d_b31: f64 = (s.db[207][31] * ddt_scale);
        let eq50_e903_d_b32: f64 = (s.db[207][32] * ddt_scale);
        let eq50_e903_d_b33: f64 = (s.db[207][33] * ddt_scale);
        let eq50_e903_d_b34: f64 = (s.db[207][34] * ddt_scale);
        let eq50_e903_d_b35: f64 = (s.db[207][35] * ddt_scale);
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e906_d_n7: f64 = p.p355;
        let eq50_e906_d_n9: f64 = (-p.p355);
        let eq50_e907: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 27, eq50_e906);
        let eq50_e907_d_n7: f64 = (eq50_e906_d_n7 * ddt_scale);
        let eq50_e907_d_n9: f64 = (eq50_e906_d_n9 * ddt_scale);
        let eq50_e908: f64 = (eq50_e903 + eq50_e907);
        let eq50_e908_d_n7: f64 = (eq50_e903_d_n7 + eq50_e907_d_n7);
        let eq50_e908_d_n9: f64 = (eq50_e903_d_n9 + eq50_e907_d_n9);
        (eq50_e908, eq50_e903_d_n0, eq50_e903_d_n1, eq50_e903_d_n2, eq50_e903_d_n3, eq50_e903_d_n4, eq50_e903_d_n5, eq50_e903_d_n6, eq50_e908_d_n7, eq50_e903_d_n8, eq50_e908_d_n9, eq50_e903_d_n10, eq50_e903_d_n11, eq50_e903_d_n12, eq50_e903_d_n13, eq50_e903_d_n14, eq50_e903_d_n15, eq50_e903_d_n16, eq50_e903_d_n17, eq50_e903_d_n18, eq50_e903_d_n19, eq50_e903_d_n20, eq50_e903_d_n21, eq50_e903_d_n22, eq50_e903_d_n23, eq50_e903_d_n24, eq50_e903_d_n25, eq50_e903_d_n26, eq50_e903_d_n27, eq50_e903_d_n28, eq50_e903_d_n29, eq50_e903_d_b0, eq50_e903_d_b1, eq50_e903_d_b2, eq50_e903_d_b3, eq50_e903_d_b4, eq50_e903_d_b5, eq50_e903_d_b6, eq50_e903_d_b7, eq50_e903_d_b8, eq50_e903_d_b9, eq50_e903_d_b10, eq50_e903_d_b11, eq50_e903_d_b12, eq50_e903_d_b13, eq50_e903_d_b14, eq50_e903_d_b15, eq50_e903_d_b16, eq50_e903_d_b17, eq50_e903_d_b18, eq50_e903_d_b19, eq50_e903_d_b20, eq50_e903_d_b21, eq50_e903_d_b22, eq50_e903_d_b23, eq50_e903_d_b24, eq50_e903_d_b25, eq50_e903_d_b26, eq50_e903_d_b27, eq50_e903_d_b28, eq50_e903_d_b29, eq50_e903_d_b30, eq50_e903_d_b31, eq50_e903_d_b32, eq50_e903_d_b33, eq50_e903_d_b34, eq50_e903_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        let eq50_node_derivatives: [f64; 30] = [eq50_e910_d_n0, eq50_e910_d_n1, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n5, eq50_e910_d_n6, eq50_e910_d_n7, eq50_e910_d_n8, eq50_e910_d_n9, eq50_e910_d_n10, eq50_e910_d_n11, eq50_e910_d_n12, eq50_e910_d_n13, eq50_e910_d_n14, eq50_e910_d_n15, eq50_e910_d_n16, eq50_e910_d_n17, eq50_e910_d_n18, eq50_e910_d_n19, eq50_e910_d_n20, eq50_e910_d_n21, eq50_e910_d_n22, eq50_e910_d_n23, eq50_e910_d_n24, eq50_e910_d_n25, eq50_e910_d_n26, eq50_e910_d_n27, eq50_e910_d_n28, eq50_e910_d_n29];
        let eq50_branch_derivatives: [f64; 36] = [eq50_e910_d_b0, eq50_e910_d_b1, eq50_e910_d_b2, eq50_e910_d_b3, eq50_e910_d_b4, eq50_e910_d_b5, eq50_e910_d_b6, eq50_e910_d_b7, eq50_e910_d_b8, eq50_e910_d_b9, eq50_e910_d_b10, eq50_e910_d_b11, eq50_e910_d_b12, eq50_e910_d_b13, eq50_e910_d_b14, eq50_e910_d_b15, eq50_e910_d_b16, eq50_e910_d_b17, eq50_e910_d_b18, eq50_e910_d_b19, eq50_e910_d_b20, eq50_e910_d_b21, eq50_e910_d_b22, eq50_e910_d_b23, eq50_e910_d_b24, eq50_e910_d_b25, eq50_e910_d_b26, eq50_e910_d_b27, eq50_e910_d_b28, eq50_e910_d_b29, eq50_e910_d_b30, eq50_e910_d_b31, eq50_e910_d_b32, eq50_e910_d_b33, eq50_e910_d_b34, eq50_e910_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e921, eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29, eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35,) = {
    if (!s.b[613]) {
        let eq51_e914: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 28, s.v[203]);
        let eq51_e914_d_n0: f64 = (s.dn[203][0] * ddt_scale);
        let eq51_e914_d_n1: f64 = (s.dn[203][1] * ddt_scale);
        let eq51_e914_d_n2: f64 = (s.dn[203][2] * ddt_scale);
        let eq51_e914_d_n3: f64 = (s.dn[203][3] * ddt_scale);
        let eq51_e914_d_n4: f64 = (s.dn[203][4] * ddt_scale);
        let eq51_e914_d_n5: f64 = (s.dn[203][5] * ddt_scale);
        let eq51_e914_d_n6: f64 = (s.dn[203][6] * ddt_scale);
        let eq51_e914_d_n7: f64 = (s.dn[203][7] * ddt_scale);
        let eq51_e914_d_n8: f64 = (s.dn[203][8] * ddt_scale);
        let eq51_e914_d_n9: f64 = (s.dn[203][9] * ddt_scale);
        let eq51_e914_d_n10: f64 = (s.dn[203][10] * ddt_scale);
        let eq51_e914_d_n11: f64 = (s.dn[203][11] * ddt_scale);
        let eq51_e914_d_n12: f64 = (s.dn[203][12] * ddt_scale);
        let eq51_e914_d_n13: f64 = (s.dn[203][13] * ddt_scale);
        let eq51_e914_d_n14: f64 = (s.dn[203][14] * ddt_scale);
        let eq51_e914_d_n15: f64 = (s.dn[203][15] * ddt_scale);
        let eq51_e914_d_n16: f64 = (s.dn[203][16] * ddt_scale);
        let eq51_e914_d_n17: f64 = (s.dn[203][17] * ddt_scale);
        let eq51_e914_d_n18: f64 = (s.dn[203][18] * ddt_scale);
        let eq51_e914_d_n19: f64 = (s.dn[203][19] * ddt_scale);
        let eq51_e914_d_n20: f64 = (s.dn[203][20] * ddt_scale);
        let eq51_e914_d_n21: f64 = (s.dn[203][21] * ddt_scale);
        let eq51_e914_d_n22: f64 = (s.dn[203][22] * ddt_scale);
        let eq51_e914_d_n23: f64 = (s.dn[203][23] * ddt_scale);
        let eq51_e914_d_n24: f64 = (s.dn[203][24] * ddt_scale);
        let eq51_e914_d_n25: f64 = (s.dn[203][25] * ddt_scale);
        let eq51_e914_d_n26: f64 = (s.dn[203][26] * ddt_scale);
        let eq51_e914_d_n27: f64 = (s.dn[203][27] * ddt_scale);
        let eq51_e914_d_n28: f64 = (s.dn[203][28] * ddt_scale);
        let eq51_e914_d_n29: f64 = (s.dn[203][29] * ddt_scale);
        let eq51_e914_d_b0: f64 = (s.db[203][0] * ddt_scale);
        let eq51_e914_d_b1: f64 = (s.db[203][1] * ddt_scale);
        let eq51_e914_d_b2: f64 = (s.db[203][2] * ddt_scale);
        let eq51_e914_d_b3: f64 = (s.db[203][3] * ddt_scale);
        let eq51_e914_d_b4: f64 = (s.db[203][4] * ddt_scale);
        let eq51_e914_d_b5: f64 = (s.db[203][5] * ddt_scale);
        let eq51_e914_d_b6: f64 = (s.db[203][6] * ddt_scale);
        let eq51_e914_d_b7: f64 = (s.db[203][7] * ddt_scale);
        let eq51_e914_d_b8: f64 = (s.db[203][8] * ddt_scale);
        let eq51_e914_d_b9: f64 = (s.db[203][9] * ddt_scale);
        let eq51_e914_d_b10: f64 = (s.db[203][10] * ddt_scale);
        let eq51_e914_d_b11: f64 = (s.db[203][11] * ddt_scale);
        let eq51_e914_d_b12: f64 = (s.db[203][12] * ddt_scale);
        let eq51_e914_d_b13: f64 = (s.db[203][13] * ddt_scale);
        let eq51_e914_d_b14: f64 = (s.db[203][14] * ddt_scale);
        let eq51_e914_d_b15: f64 = (s.db[203][15] * ddt_scale);
        let eq51_e914_d_b16: f64 = (s.db[203][16] * ddt_scale);
        let eq51_e914_d_b17: f64 = (s.db[203][17] * ddt_scale);
        let eq51_e914_d_b18: f64 = (s.db[203][18] * ddt_scale);
        let eq51_e914_d_b19: f64 = (s.db[203][19] * ddt_scale);
        let eq51_e914_d_b20: f64 = (s.db[203][20] * ddt_scale);
        let eq51_e914_d_b21: f64 = (s.db[203][21] * ddt_scale);
        let eq51_e914_d_b22: f64 = (s.db[203][22] * ddt_scale);
        let eq51_e914_d_b23: f64 = (s.db[203][23] * ddt_scale);
        let eq51_e914_d_b24: f64 = (s.db[203][24] * ddt_scale);
        let eq51_e914_d_b25: f64 = (s.db[203][25] * ddt_scale);
        let eq51_e914_d_b26: f64 = (s.db[203][26] * ddt_scale);
        let eq51_e914_d_b27: f64 = (s.db[203][27] * ddt_scale);
        let eq51_e914_d_b28: f64 = (s.db[203][28] * ddt_scale);
        let eq51_e914_d_b29: f64 = (s.db[203][29] * ddt_scale);
        let eq51_e914_d_b30: f64 = (s.db[203][30] * ddt_scale);
        let eq51_e914_d_b31: f64 = (s.db[203][31] * ddt_scale);
        let eq51_e914_d_b32: f64 = (s.db[203][32] * ddt_scale);
        let eq51_e914_d_b33: f64 = (s.db[203][33] * ddt_scale);
        let eq51_e914_d_b34: f64 = (s.db[203][34] * ddt_scale);
        let eq51_e914_d_b35: f64 = (s.db[203][35] * ddt_scale);
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e917_d_n2: f64 = p.p355;
        let eq51_e917_d_n15: f64 = (-p.p355);
        let eq51_e918: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 29, eq51_e917);
        let eq51_e918_d_n2: f64 = (eq51_e917_d_n2 * ddt_scale);
        let eq51_e918_d_n15: f64 = (eq51_e917_d_n15 * ddt_scale);
        let eq51_e919: f64 = (eq51_e914 + eq51_e918);
        let eq51_e919_d_n2: f64 = (eq51_e914_d_n2 + eq51_e918_d_n2);
        let eq51_e919_d_n15: f64 = (eq51_e914_d_n15 + eq51_e918_d_n15);
        (eq51_e919, eq51_e914_d_n0, eq51_e914_d_n1, eq51_e919_d_n2, eq51_e914_d_n3, eq51_e914_d_n4, eq51_e914_d_n5, eq51_e914_d_n6, eq51_e914_d_n7, eq51_e914_d_n8, eq51_e914_d_n9, eq51_e914_d_n10, eq51_e914_d_n11, eq51_e914_d_n12, eq51_e914_d_n13, eq51_e914_d_n14, eq51_e919_d_n15, eq51_e914_d_n16, eq51_e914_d_n17, eq51_e914_d_n18, eq51_e914_d_n19, eq51_e914_d_n20, eq51_e914_d_n21, eq51_e914_d_n22, eq51_e914_d_n23, eq51_e914_d_n24, eq51_e914_d_n25, eq51_e914_d_n26, eq51_e914_d_n27, eq51_e914_d_n28, eq51_e914_d_n29, eq51_e914_d_b0, eq51_e914_d_b1, eq51_e914_d_b2, eq51_e914_d_b3, eq51_e914_d_b4, eq51_e914_d_b5, eq51_e914_d_b6, eq51_e914_d_b7, eq51_e914_d_b8, eq51_e914_d_b9, eq51_e914_d_b10, eq51_e914_d_b11, eq51_e914_d_b12, eq51_e914_d_b13, eq51_e914_d_b14, eq51_e914_d_b15, eq51_e914_d_b16, eq51_e914_d_b17, eq51_e914_d_b18, eq51_e914_d_b19, eq51_e914_d_b20, eq51_e914_d_b21, eq51_e914_d_b22, eq51_e914_d_b23, eq51_e914_d_b24, eq51_e914_d_b25, eq51_e914_d_b26, eq51_e914_d_b27, eq51_e914_d_b28, eq51_e914_d_b29, eq51_e914_d_b30, eq51_e914_d_b31, eq51_e914_d_b32, eq51_e914_d_b33, eq51_e914_d_b34, eq51_e914_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        let eq51_node_derivatives: [f64; 30] = [eq51_e921_d_n0, eq51_e921_d_n1, eq51_e921_d_n2, eq51_e921_d_n3, eq51_e921_d_n4, eq51_e921_d_n5, eq51_e921_d_n6, eq51_e921_d_n7, eq51_e921_d_n8, eq51_e921_d_n9, eq51_e921_d_n10, eq51_e921_d_n11, eq51_e921_d_n12, eq51_e921_d_n13, eq51_e921_d_n14, eq51_e921_d_n15, eq51_e921_d_n16, eq51_e921_d_n17, eq51_e921_d_n18, eq51_e921_d_n19, eq51_e921_d_n20, eq51_e921_d_n21, eq51_e921_d_n22, eq51_e921_d_n23, eq51_e921_d_n24, eq51_e921_d_n25, eq51_e921_d_n26, eq51_e921_d_n27, eq51_e921_d_n28, eq51_e921_d_n29];
        let eq51_branch_derivatives: [f64; 36] = [eq51_e921_d_b0, eq51_e921_d_b1, eq51_e921_d_b2, eq51_e921_d_b3, eq51_e921_d_b4, eq51_e921_d_b5, eq51_e921_d_b6, eq51_e921_d_b7, eq51_e921_d_b8, eq51_e921_d_b9, eq51_e921_d_b10, eq51_e921_d_b11, eq51_e921_d_b12, eq51_e921_d_b13, eq51_e921_d_b14, eq51_e921_d_b15, eq51_e921_d_b16, eq51_e921_d_b17, eq51_e921_d_b18, eq51_e921_d_b19, eq51_e921_d_b20, eq51_e921_d_b21, eq51_e921_d_b22, eq51_e921_d_b23, eq51_e921_d_b24, eq51_e921_d_b25, eq51_e921_d_b26, eq51_e921_d_b27, eq51_e921_d_b28, eq51_e921_d_b29, eq51_e921_d_b30, eq51_e921_d_b31, eq51_e921_d_b32, eq51_e921_d_b33, eq51_e921_d_b34, eq51_e921_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq52_e932, eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29, eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35,) = {
    if (!s.b[613]) {
        let eq52_e925: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 30, s.v[204]);
        let eq52_e925_d_n0: f64 = (s.dn[204][0] * ddt_scale);
        let eq52_e925_d_n1: f64 = (s.dn[204][1] * ddt_scale);
        let eq52_e925_d_n2: f64 = (s.dn[204][2] * ddt_scale);
        let eq52_e925_d_n3: f64 = (s.dn[204][3] * ddt_scale);
        let eq52_e925_d_n4: f64 = (s.dn[204][4] * ddt_scale);
        let eq52_e925_d_n5: f64 = (s.dn[204][5] * ddt_scale);
        let eq52_e925_d_n6: f64 = (s.dn[204][6] * ddt_scale);
        let eq52_e925_d_n7: f64 = (s.dn[204][7] * ddt_scale);
        let eq52_e925_d_n8: f64 = (s.dn[204][8] * ddt_scale);
        let eq52_e925_d_n9: f64 = (s.dn[204][9] * ddt_scale);
        let eq52_e925_d_n10: f64 = (s.dn[204][10] * ddt_scale);
        let eq52_e925_d_n11: f64 = (s.dn[204][11] * ddt_scale);
        let eq52_e925_d_n12: f64 = (s.dn[204][12] * ddt_scale);
        let eq52_e925_d_n13: f64 = (s.dn[204][13] * ddt_scale);
        let eq52_e925_d_n14: f64 = (s.dn[204][14] * ddt_scale);
        let eq52_e925_d_n15: f64 = (s.dn[204][15] * ddt_scale);
        let eq52_e925_d_n16: f64 = (s.dn[204][16] * ddt_scale);
        let eq52_e925_d_n17: f64 = (s.dn[204][17] * ddt_scale);
        let eq52_e925_d_n18: f64 = (s.dn[204][18] * ddt_scale);
        let eq52_e925_d_n19: f64 = (s.dn[204][19] * ddt_scale);
        let eq52_e925_d_n20: f64 = (s.dn[204][20] * ddt_scale);
        let eq52_e925_d_n21: f64 = (s.dn[204][21] * ddt_scale);
        let eq52_e925_d_n22: f64 = (s.dn[204][22] * ddt_scale);
        let eq52_e925_d_n23: f64 = (s.dn[204][23] * ddt_scale);
        let eq52_e925_d_n24: f64 = (s.dn[204][24] * ddt_scale);
        let eq52_e925_d_n25: f64 = (s.dn[204][25] * ddt_scale);
        let eq52_e925_d_n26: f64 = (s.dn[204][26] * ddt_scale);
        let eq52_e925_d_n27: f64 = (s.dn[204][27] * ddt_scale);
        let eq52_e925_d_n28: f64 = (s.dn[204][28] * ddt_scale);
        let eq52_e925_d_n29: f64 = (s.dn[204][29] * ddt_scale);
        let eq52_e925_d_b0: f64 = (s.db[204][0] * ddt_scale);
        let eq52_e925_d_b1: f64 = (s.db[204][1] * ddt_scale);
        let eq52_e925_d_b2: f64 = (s.db[204][2] * ddt_scale);
        let eq52_e925_d_b3: f64 = (s.db[204][3] * ddt_scale);
        let eq52_e925_d_b4: f64 = (s.db[204][4] * ddt_scale);
        let eq52_e925_d_b5: f64 = (s.db[204][5] * ddt_scale);
        let eq52_e925_d_b6: f64 = (s.db[204][6] * ddt_scale);
        let eq52_e925_d_b7: f64 = (s.db[204][7] * ddt_scale);
        let eq52_e925_d_b8: f64 = (s.db[204][8] * ddt_scale);
        let eq52_e925_d_b9: f64 = (s.db[204][9] * ddt_scale);
        let eq52_e925_d_b10: f64 = (s.db[204][10] * ddt_scale);
        let eq52_e925_d_b11: f64 = (s.db[204][11] * ddt_scale);
        let eq52_e925_d_b12: f64 = (s.db[204][12] * ddt_scale);
        let eq52_e925_d_b13: f64 = (s.db[204][13] * ddt_scale);
        let eq52_e925_d_b14: f64 = (s.db[204][14] * ddt_scale);
        let eq52_e925_d_b15: f64 = (s.db[204][15] * ddt_scale);
        let eq52_e925_d_b16: f64 = (s.db[204][16] * ddt_scale);
        let eq52_e925_d_b17: f64 = (s.db[204][17] * ddt_scale);
        let eq52_e925_d_b18: f64 = (s.db[204][18] * ddt_scale);
        let eq52_e925_d_b19: f64 = (s.db[204][19] * ddt_scale);
        let eq52_e925_d_b20: f64 = (s.db[204][20] * ddt_scale);
        let eq52_e925_d_b21: f64 = (s.db[204][21] * ddt_scale);
        let eq52_e925_d_b22: f64 = (s.db[204][22] * ddt_scale);
        let eq52_e925_d_b23: f64 = (s.db[204][23] * ddt_scale);
        let eq52_e925_d_b24: f64 = (s.db[204][24] * ddt_scale);
        let eq52_e925_d_b25: f64 = (s.db[204][25] * ddt_scale);
        let eq52_e925_d_b26: f64 = (s.db[204][26] * ddt_scale);
        let eq52_e925_d_b27: f64 = (s.db[204][27] * ddt_scale);
        let eq52_e925_d_b28: f64 = (s.db[204][28] * ddt_scale);
        let eq52_e925_d_b29: f64 = (s.db[204][29] * ddt_scale);
        let eq52_e925_d_b30: f64 = (s.db[204][30] * ddt_scale);
        let eq52_e925_d_b31: f64 = (s.db[204][31] * ddt_scale);
        let eq52_e925_d_b32: f64 = (s.db[204][32] * ddt_scale);
        let eq52_e925_d_b33: f64 = (s.db[204][33] * ddt_scale);
        let eq52_e925_d_b34: f64 = (s.db[204][34] * ddt_scale);
        let eq52_e925_d_b35: f64 = (s.db[204][35] * ddt_scale);
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e928_d_n2: f64 = p.p355;
        let eq52_e928_d_n16: f64 = (-p.p355);
        let eq52_e929: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 31, eq52_e928);
        let eq52_e929_d_n2: f64 = (eq52_e928_d_n2 * ddt_scale);
        let eq52_e929_d_n16: f64 = (eq52_e928_d_n16 * ddt_scale);
        let eq52_e930: f64 = (eq52_e925 + eq52_e929);
        let eq52_e930_d_n2: f64 = (eq52_e925_d_n2 + eq52_e929_d_n2);
        let eq52_e930_d_n16: f64 = (eq52_e925_d_n16 + eq52_e929_d_n16);
        (eq52_e930, eq52_e925_d_n0, eq52_e925_d_n1, eq52_e930_d_n2, eq52_e925_d_n3, eq52_e925_d_n4, eq52_e925_d_n5, eq52_e925_d_n6, eq52_e925_d_n7, eq52_e925_d_n8, eq52_e925_d_n9, eq52_e925_d_n10, eq52_e925_d_n11, eq52_e925_d_n12, eq52_e925_d_n13, eq52_e925_d_n14, eq52_e925_d_n15, eq52_e930_d_n16, eq52_e925_d_n17, eq52_e925_d_n18, eq52_e925_d_n19, eq52_e925_d_n20, eq52_e925_d_n21, eq52_e925_d_n22, eq52_e925_d_n23, eq52_e925_d_n24, eq52_e925_d_n25, eq52_e925_d_n26, eq52_e925_d_n27, eq52_e925_d_n28, eq52_e925_d_n29, eq52_e925_d_b0, eq52_e925_d_b1, eq52_e925_d_b2, eq52_e925_d_b3, eq52_e925_d_b4, eq52_e925_d_b5, eq52_e925_d_b6, eq52_e925_d_b7, eq52_e925_d_b8, eq52_e925_d_b9, eq52_e925_d_b10, eq52_e925_d_b11, eq52_e925_d_b12, eq52_e925_d_b13, eq52_e925_d_b14, eq52_e925_d_b15, eq52_e925_d_b16, eq52_e925_d_b17, eq52_e925_d_b18, eq52_e925_d_b19, eq52_e925_d_b20, eq52_e925_d_b21, eq52_e925_d_b22, eq52_e925_d_b23, eq52_e925_d_b24, eq52_e925_d_b25, eq52_e925_d_b26, eq52_e925_d_b27, eq52_e925_d_b28, eq52_e925_d_b29, eq52_e925_d_b30, eq52_e925_d_b31, eq52_e925_d_b32, eq52_e925_d_b33, eq52_e925_d_b34, eq52_e925_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e932;
        let eq52_node_derivatives: [f64; 30] = [eq52_e932_d_n0, eq52_e932_d_n1, eq52_e932_d_n2, eq52_e932_d_n3, eq52_e932_d_n4, eq52_e932_d_n5, eq52_e932_d_n6, eq52_e932_d_n7, eq52_e932_d_n8, eq52_e932_d_n9, eq52_e932_d_n10, eq52_e932_d_n11, eq52_e932_d_n12, eq52_e932_d_n13, eq52_e932_d_n14, eq52_e932_d_n15, eq52_e932_d_n16, eq52_e932_d_n17, eq52_e932_d_n18, eq52_e932_d_n19, eq52_e932_d_n20, eq52_e932_d_n21, eq52_e932_d_n22, eq52_e932_d_n23, eq52_e932_d_n24, eq52_e932_d_n25, eq52_e932_d_n26, eq52_e932_d_n27, eq52_e932_d_n28, eq52_e932_d_n29];
        let eq52_branch_derivatives: [f64; 36] = [eq52_e932_d_b0, eq52_e932_d_b1, eq52_e932_d_b2, eq52_e932_d_b3, eq52_e932_d_b4, eq52_e932_d_b5, eq52_e932_d_b6, eq52_e932_d_b7, eq52_e932_d_b8, eq52_e932_d_b9, eq52_e932_d_b10, eq52_e932_d_b11, eq52_e932_d_b12, eq52_e932_d_b13, eq52_e932_d_b14, eq52_e932_d_b15, eq52_e932_d_b16, eq52_e932_d_b17, eq52_e932_d_b18, eq52_e932_d_b19, eq52_e932_d_b20, eq52_e932_d_b21, eq52_e932_d_b22, eq52_e932_d_b23, eq52_e932_d_b24, eq52_e932_d_b25, eq52_e932_d_b26, eq52_e932_d_b27, eq52_e932_d_b28, eq52_e932_d_b29, eq52_e932_d_b30, eq52_e932_d_b31, eq52_e932_d_b32, eq52_e932_d_b33, eq52_e932_d_b34, eq52_e932_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(16),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e943, eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29, eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35,) = {
    if (!s.b[613]) {
        let eq53_e936: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 32, s.v[205]);
        let eq53_e936_d_n0: f64 = (s.dn[205][0] * ddt_scale);
        let eq53_e936_d_n1: f64 = (s.dn[205][1] * ddt_scale);
        let eq53_e936_d_n2: f64 = (s.dn[205][2] * ddt_scale);
        let eq53_e936_d_n3: f64 = (s.dn[205][3] * ddt_scale);
        let eq53_e936_d_n4: f64 = (s.dn[205][4] * ddt_scale);
        let eq53_e936_d_n5: f64 = (s.dn[205][5] * ddt_scale);
        let eq53_e936_d_n6: f64 = (s.dn[205][6] * ddt_scale);
        let eq53_e936_d_n7: f64 = (s.dn[205][7] * ddt_scale);
        let eq53_e936_d_n8: f64 = (s.dn[205][8] * ddt_scale);
        let eq53_e936_d_n9: f64 = (s.dn[205][9] * ddt_scale);
        let eq53_e936_d_n10: f64 = (s.dn[205][10] * ddt_scale);
        let eq53_e936_d_n11: f64 = (s.dn[205][11] * ddt_scale);
        let eq53_e936_d_n12: f64 = (s.dn[205][12] * ddt_scale);
        let eq53_e936_d_n13: f64 = (s.dn[205][13] * ddt_scale);
        let eq53_e936_d_n14: f64 = (s.dn[205][14] * ddt_scale);
        let eq53_e936_d_n15: f64 = (s.dn[205][15] * ddt_scale);
        let eq53_e936_d_n16: f64 = (s.dn[205][16] * ddt_scale);
        let eq53_e936_d_n17: f64 = (s.dn[205][17] * ddt_scale);
        let eq53_e936_d_n18: f64 = (s.dn[205][18] * ddt_scale);
        let eq53_e936_d_n19: f64 = (s.dn[205][19] * ddt_scale);
        let eq53_e936_d_n20: f64 = (s.dn[205][20] * ddt_scale);
        let eq53_e936_d_n21: f64 = (s.dn[205][21] * ddt_scale);
        let eq53_e936_d_n22: f64 = (s.dn[205][22] * ddt_scale);
        let eq53_e936_d_n23: f64 = (s.dn[205][23] * ddt_scale);
        let eq53_e936_d_n24: f64 = (s.dn[205][24] * ddt_scale);
        let eq53_e936_d_n25: f64 = (s.dn[205][25] * ddt_scale);
        let eq53_e936_d_n26: f64 = (s.dn[205][26] * ddt_scale);
        let eq53_e936_d_n27: f64 = (s.dn[205][27] * ddt_scale);
        let eq53_e936_d_n28: f64 = (s.dn[205][28] * ddt_scale);
        let eq53_e936_d_n29: f64 = (s.dn[205][29] * ddt_scale);
        let eq53_e936_d_b0: f64 = (s.db[205][0] * ddt_scale);
        let eq53_e936_d_b1: f64 = (s.db[205][1] * ddt_scale);
        let eq53_e936_d_b2: f64 = (s.db[205][2] * ddt_scale);
        let eq53_e936_d_b3: f64 = (s.db[205][3] * ddt_scale);
        let eq53_e936_d_b4: f64 = (s.db[205][4] * ddt_scale);
        let eq53_e936_d_b5: f64 = (s.db[205][5] * ddt_scale);
        let eq53_e936_d_b6: f64 = (s.db[205][6] * ddt_scale);
        let eq53_e936_d_b7: f64 = (s.db[205][7] * ddt_scale);
        let eq53_e936_d_b8: f64 = (s.db[205][8] * ddt_scale);
        let eq53_e936_d_b9: f64 = (s.db[205][9] * ddt_scale);
        let eq53_e936_d_b10: f64 = (s.db[205][10] * ddt_scale);
        let eq53_e936_d_b11: f64 = (s.db[205][11] * ddt_scale);
        let eq53_e936_d_b12: f64 = (s.db[205][12] * ddt_scale);
        let eq53_e936_d_b13: f64 = (s.db[205][13] * ddt_scale);
        let eq53_e936_d_b14: f64 = (s.db[205][14] * ddt_scale);
        let eq53_e936_d_b15: f64 = (s.db[205][15] * ddt_scale);
        let eq53_e936_d_b16: f64 = (s.db[205][16] * ddt_scale);
        let eq53_e936_d_b17: f64 = (s.db[205][17] * ddt_scale);
        let eq53_e936_d_b18: f64 = (s.db[205][18] * ddt_scale);
        let eq53_e936_d_b19: f64 = (s.db[205][19] * ddt_scale);
        let eq53_e936_d_b20: f64 = (s.db[205][20] * ddt_scale);
        let eq53_e936_d_b21: f64 = (s.db[205][21] * ddt_scale);
        let eq53_e936_d_b22: f64 = (s.db[205][22] * ddt_scale);
        let eq53_e936_d_b23: f64 = (s.db[205][23] * ddt_scale);
        let eq53_e936_d_b24: f64 = (s.db[205][24] * ddt_scale);
        let eq53_e936_d_b25: f64 = (s.db[205][25] * ddt_scale);
        let eq53_e936_d_b26: f64 = (s.db[205][26] * ddt_scale);
        let eq53_e936_d_b27: f64 = (s.db[205][27] * ddt_scale);
        let eq53_e936_d_b28: f64 = (s.db[205][28] * ddt_scale);
        let eq53_e936_d_b29: f64 = (s.db[205][29] * ddt_scale);
        let eq53_e936_d_b30: f64 = (s.db[205][30] * ddt_scale);
        let eq53_e936_d_b31: f64 = (s.db[205][31] * ddt_scale);
        let eq53_e936_d_b32: f64 = (s.db[205][32] * ddt_scale);
        let eq53_e936_d_b33: f64 = (s.db[205][33] * ddt_scale);
        let eq53_e936_d_b34: f64 = (s.db[205][34] * ddt_scale);
        let eq53_e936_d_b35: f64 = (s.db[205][35] * ddt_scale);
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e939_d_n7: f64 = p.p355;
        let eq53_e939_d_n15: f64 = (-p.p355);
        let eq53_e940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 33, eq53_e939);
        let eq53_e940_d_n7: f64 = (eq53_e939_d_n7 * ddt_scale);
        let eq53_e940_d_n15: f64 = (eq53_e939_d_n15 * ddt_scale);
        let eq53_e941: f64 = (eq53_e936 + eq53_e940);
        let eq53_e941_d_n7: f64 = (eq53_e936_d_n7 + eq53_e940_d_n7);
        let eq53_e941_d_n15: f64 = (eq53_e936_d_n15 + eq53_e940_d_n15);
        (eq53_e941, eq53_e936_d_n0, eq53_e936_d_n1, eq53_e936_d_n2, eq53_e936_d_n3, eq53_e936_d_n4, eq53_e936_d_n5, eq53_e936_d_n6, eq53_e941_d_n7, eq53_e936_d_n8, eq53_e936_d_n9, eq53_e936_d_n10, eq53_e936_d_n11, eq53_e936_d_n12, eq53_e936_d_n13, eq53_e936_d_n14, eq53_e941_d_n15, eq53_e936_d_n16, eq53_e936_d_n17, eq53_e936_d_n18, eq53_e936_d_n19, eq53_e936_d_n20, eq53_e936_d_n21, eq53_e936_d_n22, eq53_e936_d_n23, eq53_e936_d_n24, eq53_e936_d_n25, eq53_e936_d_n26, eq53_e936_d_n27, eq53_e936_d_n28, eq53_e936_d_n29, eq53_e936_d_b0, eq53_e936_d_b1, eq53_e936_d_b2, eq53_e936_d_b3, eq53_e936_d_b4, eq53_e936_d_b5, eq53_e936_d_b6, eq53_e936_d_b7, eq53_e936_d_b8, eq53_e936_d_b9, eq53_e936_d_b10, eq53_e936_d_b11, eq53_e936_d_b12, eq53_e936_d_b13, eq53_e936_d_b14, eq53_e936_d_b15, eq53_e936_d_b16, eq53_e936_d_b17, eq53_e936_d_b18, eq53_e936_d_b19, eq53_e936_d_b20, eq53_e936_d_b21, eq53_e936_d_b22, eq53_e936_d_b23, eq53_e936_d_b24, eq53_e936_d_b25, eq53_e936_d_b26, eq53_e936_d_b27, eq53_e936_d_b28, eq53_e936_d_b29, eq53_e936_d_b30, eq53_e936_d_b31, eq53_e936_d_b32, eq53_e936_d_b33, eq53_e936_d_b34, eq53_e936_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e943;
        let eq53_node_derivatives: [f64; 30] = [eq53_e943_d_n0, eq53_e943_d_n1, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n5, eq53_e943_d_n6, eq53_e943_d_n7, eq53_e943_d_n8, eq53_e943_d_n9, eq53_e943_d_n10, eq53_e943_d_n11, eq53_e943_d_n12, eq53_e943_d_n13, eq53_e943_d_n14, eq53_e943_d_n15, eq53_e943_d_n16, eq53_e943_d_n17, eq53_e943_d_n18, eq53_e943_d_n19, eq53_e943_d_n20, eq53_e943_d_n21, eq53_e943_d_n22, eq53_e943_d_n23, eq53_e943_d_n24, eq53_e943_d_n25, eq53_e943_d_n26, eq53_e943_d_n27, eq53_e943_d_n28, eq53_e943_d_n29];
        let eq53_branch_derivatives: [f64; 36] = [eq53_e943_d_b0, eq53_e943_d_b1, eq53_e943_d_b2, eq53_e943_d_b3, eq53_e943_d_b4, eq53_e943_d_b5, eq53_e943_d_b6, eq53_e943_d_b7, eq53_e943_d_b8, eq53_e943_d_b9, eq53_e943_d_b10, eq53_e943_d_b11, eq53_e943_d_b12, eq53_e943_d_b13, eq53_e943_d_b14, eq53_e943_d_b15, eq53_e943_d_b16, eq53_e943_d_b17, eq53_e943_d_b18, eq53_e943_d_b19, eq53_e943_d_b20, eq53_e943_d_b21, eq53_e943_d_b22, eq53_e943_d_b23, eq53_e943_d_b24, eq53_e943_d_b25, eq53_e943_d_b26, eq53_e943_d_b27, eq53_e943_d_b28, eq53_e943_d_b29, eq53_e943_d_b30, eq53_e943_d_b31, eq53_e943_d_b32, eq53_e943_d_b33, eq53_e943_d_b34, eq53_e943_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e948,) = {
    if (!s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e948;
        stamper.stamp_current_const_local(
            Some(7),
            Some(16),
            multiplicity * (eq54_value),
        );
        let (eq55_e953,) = {
    if (!s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e953;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq55_value),
        );
        let eq56_e955: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 34, s.v[206]);
        let eq56_e955_d_n0: f64 = (s.dn[206][0] * ddt_scale);
        let eq56_e955_d_n1: f64 = (s.dn[206][1] * ddt_scale);
        let eq56_e955_d_n2: f64 = (s.dn[206][2] * ddt_scale);
        let eq56_e955_d_n3: f64 = (s.dn[206][3] * ddt_scale);
        let eq56_e955_d_n4: f64 = (s.dn[206][4] * ddt_scale);
        let eq56_e955_d_n5: f64 = (s.dn[206][5] * ddt_scale);
        let eq56_e955_d_n6: f64 = (s.dn[206][6] * ddt_scale);
        let eq56_e955_d_n7: f64 = (s.dn[206][7] * ddt_scale);
        let eq56_e955_d_n8: f64 = (s.dn[206][8] * ddt_scale);
        let eq56_e955_d_n9: f64 = (s.dn[206][9] * ddt_scale);
        let eq56_e955_d_n10: f64 = (s.dn[206][10] * ddt_scale);
        let eq56_e955_d_n11: f64 = (s.dn[206][11] * ddt_scale);
        let eq56_e955_d_n12: f64 = (s.dn[206][12] * ddt_scale);
        let eq56_e955_d_n13: f64 = (s.dn[206][13] * ddt_scale);
        let eq56_e955_d_n14: f64 = (s.dn[206][14] * ddt_scale);
        let eq56_e955_d_n15: f64 = (s.dn[206][15] * ddt_scale);
        let eq56_e955_d_n16: f64 = (s.dn[206][16] * ddt_scale);
        let eq56_e955_d_n17: f64 = (s.dn[206][17] * ddt_scale);
        let eq56_e955_d_n18: f64 = (s.dn[206][18] * ddt_scale);
        let eq56_e955_d_n19: f64 = (s.dn[206][19] * ddt_scale);
        let eq56_e955_d_n20: f64 = (s.dn[206][20] * ddt_scale);
        let eq56_e955_d_n21: f64 = (s.dn[206][21] * ddt_scale);
        let eq56_e955_d_n22: f64 = (s.dn[206][22] * ddt_scale);
        let eq56_e955_d_n23: f64 = (s.dn[206][23] * ddt_scale);
        let eq56_e955_d_n24: f64 = (s.dn[206][24] * ddt_scale);
        let eq56_e955_d_n25: f64 = (s.dn[206][25] * ddt_scale);
        let eq56_e955_d_n26: f64 = (s.dn[206][26] * ddt_scale);
        let eq56_e955_d_n27: f64 = (s.dn[206][27] * ddt_scale);
        let eq56_e955_d_n28: f64 = (s.dn[206][28] * ddt_scale);
        let eq56_e955_d_n29: f64 = (s.dn[206][29] * ddt_scale);
        let eq56_e955_d_b0: f64 = (s.db[206][0] * ddt_scale);
        let eq56_e955_d_b1: f64 = (s.db[206][1] * ddt_scale);
        let eq56_e955_d_b2: f64 = (s.db[206][2] * ddt_scale);
        let eq56_e955_d_b3: f64 = (s.db[206][3] * ddt_scale);
        let eq56_e955_d_b4: f64 = (s.db[206][4] * ddt_scale);
        let eq56_e955_d_b5: f64 = (s.db[206][5] * ddt_scale);
        let eq56_e955_d_b6: f64 = (s.db[206][6] * ddt_scale);
        let eq56_e955_d_b7: f64 = (s.db[206][7] * ddt_scale);
        let eq56_e955_d_b8: f64 = (s.db[206][8] * ddt_scale);
        let eq56_e955_d_b9: f64 = (s.db[206][9] * ddt_scale);
        let eq56_e955_d_b10: f64 = (s.db[206][10] * ddt_scale);
        let eq56_e955_d_b11: f64 = (s.db[206][11] * ddt_scale);
        let eq56_e955_d_b12: f64 = (s.db[206][12] * ddt_scale);
        let eq56_e955_d_b13: f64 = (s.db[206][13] * ddt_scale);
        let eq56_e955_d_b14: f64 = (s.db[206][14] * ddt_scale);
        let eq56_e955_d_b15: f64 = (s.db[206][15] * ddt_scale);
        let eq56_e955_d_b16: f64 = (s.db[206][16] * ddt_scale);
        let eq56_e955_d_b17: f64 = (s.db[206][17] * ddt_scale);
        let eq56_e955_d_b18: f64 = (s.db[206][18] * ddt_scale);
        let eq56_e955_d_b19: f64 = (s.db[206][19] * ddt_scale);
        let eq56_e955_d_b20: f64 = (s.db[206][20] * ddt_scale);
        let eq56_e955_d_b21: f64 = (s.db[206][21] * ddt_scale);
        let eq56_e955_d_b22: f64 = (s.db[206][22] * ddt_scale);
        let eq56_e955_d_b23: f64 = (s.db[206][23] * ddt_scale);
        let eq56_e955_d_b24: f64 = (s.db[206][24] * ddt_scale);
        let eq56_e955_d_b25: f64 = (s.db[206][25] * ddt_scale);
        let eq56_e955_d_b26: f64 = (s.db[206][26] * ddt_scale);
        let eq56_e955_d_b27: f64 = (s.db[206][27] * ddt_scale);
        let eq56_e955_d_b28: f64 = (s.db[206][28] * ddt_scale);
        let eq56_e955_d_b29: f64 = (s.db[206][29] * ddt_scale);
        let eq56_e955_d_b30: f64 = (s.db[206][30] * ddt_scale);
        let eq56_e955_d_b31: f64 = (s.db[206][31] * ddt_scale);
        let eq56_e955_d_b32: f64 = (s.db[206][32] * ddt_scale);
        let eq56_e955_d_b33: f64 = (s.db[206][33] * ddt_scale);
        let eq56_e955_d_b34: f64 = (s.db[206][34] * ddt_scale);
        let eq56_e955_d_b35: f64 = (s.db[206][35] * ddt_scale);
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e958_d_n3: f64 = p.p355;
        let eq56_e958_d_n15: f64 = (-p.p355);
        let eq56_e959: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 35, eq56_e958);
        let eq56_e959_d_n3: f64 = (eq56_e958_d_n3 * ddt_scale);
        let eq56_e959_d_n15: f64 = (eq56_e958_d_n15 * ddt_scale);
        let eq56_e960: f64 = (eq56_e955 + eq56_e959);
        let eq56_e960_d_n3: f64 = (eq56_e955_d_n3 + eq56_e959_d_n3);
        let eq56_e960_d_n15: f64 = (eq56_e955_d_n15 + eq56_e959_d_n15);
        let eq56_value: f64 = eq56_e960;
        let eq56_node_derivatives: [f64; 30] = [eq56_e955_d_n0, eq56_e955_d_n1, eq56_e955_d_n2, eq56_e960_d_n3, eq56_e955_d_n4, eq56_e955_d_n5, eq56_e955_d_n6, eq56_e955_d_n7, eq56_e955_d_n8, eq56_e955_d_n9, eq56_e955_d_n10, eq56_e955_d_n11, eq56_e955_d_n12, eq56_e955_d_n13, eq56_e955_d_n14, eq56_e960_d_n15, eq56_e955_d_n16, eq56_e955_d_n17, eq56_e955_d_n18, eq56_e955_d_n19, eq56_e955_d_n20, eq56_e955_d_n21, eq56_e955_d_n22, eq56_e955_d_n23, eq56_e955_d_n24, eq56_e955_d_n25, eq56_e955_d_n26, eq56_e955_d_n27, eq56_e955_d_n28, eq56_e955_d_n29];
        let eq56_branch_derivatives: [f64; 36] = [eq56_e955_d_b0, eq56_e955_d_b1, eq56_e955_d_b2, eq56_e955_d_b3, eq56_e955_d_b4, eq56_e955_d_b5, eq56_e955_d_b6, eq56_e955_d_b7, eq56_e955_d_b8, eq56_e955_d_b9, eq56_e955_d_b10, eq56_e955_d_b11, eq56_e955_d_b12, eq56_e955_d_b13, eq56_e955_d_b14, eq56_e955_d_b15, eq56_e955_d_b16, eq56_e955_d_b17, eq56_e955_d_b18, eq56_e955_d_b19, eq56_e955_d_b20, eq56_e955_d_b21, eq56_e955_d_b22, eq56_e955_d_b23, eq56_e955_d_b24, eq56_e955_d_b25, eq56_e955_d_b26, eq56_e955_d_b27, eq56_e955_d_b28, eq56_e955_d_b29, eq56_e955_d_b30, eq56_e955_d_b31, eq56_e955_d_b32, eq56_e955_d_b33, eq56_e955_d_b34, eq56_e955_d_b35];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(15),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e968, eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29, eq57_e968_d_b0, eq57_e968_d_b1, eq57_e968_d_b2, eq57_e968_d_b3, eq57_e968_d_b4, eq57_e968_d_b5, eq57_e968_d_b6, eq57_e968_d_b7, eq57_e968_d_b8, eq57_e968_d_b9, eq57_e968_d_b10, eq57_e968_d_b11, eq57_e968_d_b12, eq57_e968_d_b13, eq57_e968_d_b14, eq57_e968_d_b15, eq57_e968_d_b16, eq57_e968_d_b17, eq57_e968_d_b18, eq57_e968_d_b19, eq57_e968_d_b20, eq57_e968_d_b21, eq57_e968_d_b22, eq57_e968_d_b23, eq57_e968_d_b24, eq57_e968_d_b25, eq57_e968_d_b26, eq57_e968_d_b27, eq57_e968_d_b28, eq57_e968_d_b29, eq57_e968_d_b30, eq57_e968_d_b31, eq57_e968_d_b32, eq57_e968_d_b33, eq57_e968_d_b34, eq57_e968_d_b35,) = {
    if s.b[614] {
        let eq57_e965: f64 = (s.v[0] * (nv15 - nv14));
        let eq57_e965_d_n14: f64 = (-s.v[0]);
        let eq57_e965_d_n15: f64 = s.v[0];
        let eq57_e966: f64 = (s.v[196] + eq57_e965);
        let eq57_e966_d_n14: f64 = (s.dn[196][14] + eq57_e965_d_n14);
        let eq57_e966_d_n15: f64 = (s.dn[196][15] + eq57_e965_d_n15);
        (eq57_e966, s.dn[196][0], s.dn[196][1], s.dn[196][2], s.dn[196][3], s.dn[196][4], s.dn[196][5], s.dn[196][6], s.dn[196][7], s.dn[196][8], s.dn[196][9], s.dn[196][10], s.dn[196][11], s.dn[196][12], s.dn[196][13], eq57_e966_d_n14, eq57_e966_d_n15, s.dn[196][16], s.dn[196][17], s.dn[196][18], s.dn[196][19], s.dn[196][20], s.dn[196][21], s.dn[196][22], s.dn[196][23], s.dn[196][24], s.dn[196][25], s.dn[196][26], s.dn[196][27], s.dn[196][28], s.dn[196][29], s.db[196][0], s.db[196][1], s.db[196][2], s.db[196][3], s.db[196][4], s.db[196][5], s.db[196][6], s.db[196][7], s.db[196][8], s.db[196][9], s.db[196][10], s.db[196][11], s.db[196][12], s.db[196][13], s.db[196][14], s.db[196][15], s.db[196][16], s.db[196][17], s.db[196][18], s.db[196][19], s.db[196][20], s.db[196][21], s.db[196][22], s.db[196][23], s.db[196][24], s.db[196][25], s.db[196][26], s.db[196][27], s.db[196][28], s.db[196][29], s.db[196][30], s.db[196][31], s.db[196][32], s.db[196][33], s.db[196][34], s.db[196][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e968;
        let eq57_node_derivatives: [f64; 30] = [eq57_e968_d_n0, eq57_e968_d_n1, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n5, eq57_e968_d_n6, eq57_e968_d_n7, eq57_e968_d_n8, eq57_e968_d_n9, eq57_e968_d_n10, eq57_e968_d_n11, eq57_e968_d_n12, eq57_e968_d_n13, eq57_e968_d_n14, eq57_e968_d_n15, eq57_e968_d_n16, eq57_e968_d_n17, eq57_e968_d_n18, eq57_e968_d_n19, eq57_e968_d_n20, eq57_e968_d_n21, eq57_e968_d_n22, eq57_e968_d_n23, eq57_e968_d_n24, eq57_e968_d_n25, eq57_e968_d_n26, eq57_e968_d_n27, eq57_e968_d_n28, eq57_e968_d_n29];
        let eq57_branch_derivatives: [f64; 36] = [eq57_e968_d_b0, eq57_e968_d_b1, eq57_e968_d_b2, eq57_e968_d_b3, eq57_e968_d_b4, eq57_e968_d_b5, eq57_e968_d_b6, eq57_e968_d_b7, eq57_e968_d_b8, eq57_e968_d_b9, eq57_e968_d_b10, eq57_e968_d_b11, eq57_e968_d_b12, eq57_e968_d_b13, eq57_e968_d_b14, eq57_e968_d_b15, eq57_e968_d_b16, eq57_e968_d_b17, eq57_e968_d_b18, eq57_e968_d_b19, eq57_e968_d_b20, eq57_e968_d_b21, eq57_e968_d_b22, eq57_e968_d_b23, eq57_e968_d_b24, eq57_e968_d_b25, eq57_e968_d_b26, eq57_e968_d_b27, eq57_e968_d_b28, eq57_e968_d_b29, eq57_e968_d_b30, eq57_e968_d_b31, eq57_e968_d_b32, eq57_e968_d_b33, eq57_e968_d_b34, eq57_e968_d_b35];
        stamper.stamp_current_dense_local(
            Some(15),
            Some(14),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e973,) = {
    if (!s.b[614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e973;
        stamper.stamp_potential_const_local(
            20,
            eq58_value,
        );
        let (eq59_e983, eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29, eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35,) = {
    if s.b[760] {
        let eq59_e976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 36, s.v[197]);
        let eq59_e976_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq59_e976_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq59_e976_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq59_e976_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq59_e976_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq59_e976_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq59_e976_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq59_e976_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq59_e976_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq59_e976_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq59_e976_d_n10: f64 = (s.dn[197][10] * ddt_scale);
        let eq59_e976_d_n11: f64 = (s.dn[197][11] * ddt_scale);
        let eq59_e976_d_n12: f64 = (s.dn[197][12] * ddt_scale);
        let eq59_e976_d_n13: f64 = (s.dn[197][13] * ddt_scale);
        let eq59_e976_d_n14: f64 = (s.dn[197][14] * ddt_scale);
        let eq59_e976_d_n15: f64 = (s.dn[197][15] * ddt_scale);
        let eq59_e976_d_n16: f64 = (s.dn[197][16] * ddt_scale);
        let eq59_e976_d_n17: f64 = (s.dn[197][17] * ddt_scale);
        let eq59_e976_d_n18: f64 = (s.dn[197][18] * ddt_scale);
        let eq59_e976_d_n19: f64 = (s.dn[197][19] * ddt_scale);
        let eq59_e976_d_n20: f64 = (s.dn[197][20] * ddt_scale);
        let eq59_e976_d_n21: f64 = (s.dn[197][21] * ddt_scale);
        let eq59_e976_d_n22: f64 = (s.dn[197][22] * ddt_scale);
        let eq59_e976_d_n23: f64 = (s.dn[197][23] * ddt_scale);
        let eq59_e976_d_n24: f64 = (s.dn[197][24] * ddt_scale);
        let eq59_e976_d_n25: f64 = (s.dn[197][25] * ddt_scale);
        let eq59_e976_d_n26: f64 = (s.dn[197][26] * ddt_scale);
        let eq59_e976_d_n27: f64 = (s.dn[197][27] * ddt_scale);
        let eq59_e976_d_n28: f64 = (s.dn[197][28] * ddt_scale);
        let eq59_e976_d_n29: f64 = (s.dn[197][29] * ddt_scale);
        let eq59_e976_d_b0: f64 = (s.db[197][0] * ddt_scale);
        let eq59_e976_d_b1: f64 = (s.db[197][1] * ddt_scale);
        let eq59_e976_d_b2: f64 = (s.db[197][2] * ddt_scale);
        let eq59_e976_d_b3: f64 = (s.db[197][3] * ddt_scale);
        let eq59_e976_d_b4: f64 = (s.db[197][4] * ddt_scale);
        let eq59_e976_d_b5: f64 = (s.db[197][5] * ddt_scale);
        let eq59_e976_d_b6: f64 = (s.db[197][6] * ddt_scale);
        let eq59_e976_d_b7: f64 = (s.db[197][7] * ddt_scale);
        let eq59_e976_d_b8: f64 = (s.db[197][8] * ddt_scale);
        let eq59_e976_d_b9: f64 = (s.db[197][9] * ddt_scale);
        let eq59_e976_d_b10: f64 = (s.db[197][10] * ddt_scale);
        let eq59_e976_d_b11: f64 = (s.db[197][11] * ddt_scale);
        let eq59_e976_d_b12: f64 = (s.db[197][12] * ddt_scale);
        let eq59_e976_d_b13: f64 = (s.db[197][13] * ddt_scale);
        let eq59_e976_d_b14: f64 = (s.db[197][14] * ddt_scale);
        let eq59_e976_d_b15: f64 = (s.db[197][15] * ddt_scale);
        let eq59_e976_d_b16: f64 = (s.db[197][16] * ddt_scale);
        let eq59_e976_d_b17: f64 = (s.db[197][17] * ddt_scale);
        let eq59_e976_d_b18: f64 = (s.db[197][18] * ddt_scale);
        let eq59_e976_d_b19: f64 = (s.db[197][19] * ddt_scale);
        let eq59_e976_d_b20: f64 = (s.db[197][20] * ddt_scale);
        let eq59_e976_d_b21: f64 = (s.db[197][21] * ddt_scale);
        let eq59_e976_d_b22: f64 = (s.db[197][22] * ddt_scale);
        let eq59_e976_d_b23: f64 = (s.db[197][23] * ddt_scale);
        let eq59_e976_d_b24: f64 = (s.db[197][24] * ddt_scale);
        let eq59_e976_d_b25: f64 = (s.db[197][25] * ddt_scale);
        let eq59_e976_d_b26: f64 = (s.db[197][26] * ddt_scale);
        let eq59_e976_d_b27: f64 = (s.db[197][27] * ddt_scale);
        let eq59_e976_d_b28: f64 = (s.db[197][28] * ddt_scale);
        let eq59_e976_d_b29: f64 = (s.db[197][29] * ddt_scale);
        let eq59_e976_d_b30: f64 = (s.db[197][30] * ddt_scale);
        let eq59_e976_d_b31: f64 = (s.db[197][31] * ddt_scale);
        let eq59_e976_d_b32: f64 = (s.db[197][32] * ddt_scale);
        let eq59_e976_d_b33: f64 = (s.db[197][33] * ddt_scale);
        let eq59_e976_d_b34: f64 = (s.db[197][34] * ddt_scale);
        let eq59_e976_d_b35: f64 = (s.db[197][35] * ddt_scale);
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e979_d_n7: f64 = p.p355;
        let eq59_e979_d_n14: f64 = (-p.p355);
        let eq59_e980: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 37, eq59_e979);
        let eq59_e980_d_n7: f64 = (eq59_e979_d_n7 * ddt_scale);
        let eq59_e980_d_n14: f64 = (eq59_e979_d_n14 * ddt_scale);
        let eq59_e981: f64 = (eq59_e976 + eq59_e980);
        let eq59_e981_d_n7: f64 = (eq59_e976_d_n7 + eq59_e980_d_n7);
        let eq59_e981_d_n14: f64 = (eq59_e976_d_n14 + eq59_e980_d_n14);
        (eq59_e981, eq59_e976_d_n0, eq59_e976_d_n1, eq59_e976_d_n2, eq59_e976_d_n3, eq59_e976_d_n4, eq59_e976_d_n5, eq59_e976_d_n6, eq59_e981_d_n7, eq59_e976_d_n8, eq59_e976_d_n9, eq59_e976_d_n10, eq59_e976_d_n11, eq59_e976_d_n12, eq59_e976_d_n13, eq59_e981_d_n14, eq59_e976_d_n15, eq59_e976_d_n16, eq59_e976_d_n17, eq59_e976_d_n18, eq59_e976_d_n19, eq59_e976_d_n20, eq59_e976_d_n21, eq59_e976_d_n22, eq59_e976_d_n23, eq59_e976_d_n24, eq59_e976_d_n25, eq59_e976_d_n26, eq59_e976_d_n27, eq59_e976_d_n28, eq59_e976_d_n29, eq59_e976_d_b0, eq59_e976_d_b1, eq59_e976_d_b2, eq59_e976_d_b3, eq59_e976_d_b4, eq59_e976_d_b5, eq59_e976_d_b6, eq59_e976_d_b7, eq59_e976_d_b8, eq59_e976_d_b9, eq59_e976_d_b10, eq59_e976_d_b11, eq59_e976_d_b12, eq59_e976_d_b13, eq59_e976_d_b14, eq59_e976_d_b15, eq59_e976_d_b16, eq59_e976_d_b17, eq59_e976_d_b18, eq59_e976_d_b19, eq59_e976_d_b20, eq59_e976_d_b21, eq59_e976_d_b22, eq59_e976_d_b23, eq59_e976_d_b24, eq59_e976_d_b25, eq59_e976_d_b26, eq59_e976_d_b27, eq59_e976_d_b28, eq59_e976_d_b29, eq59_e976_d_b30, eq59_e976_d_b31, eq59_e976_d_b32, eq59_e976_d_b33, eq59_e976_d_b34, eq59_e976_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e983;
        let eq59_node_derivatives: [f64; 30] = [eq59_e983_d_n0, eq59_e983_d_n1, eq59_e983_d_n2, eq59_e983_d_n3, eq59_e983_d_n4, eq59_e983_d_n5, eq59_e983_d_n6, eq59_e983_d_n7, eq59_e983_d_n8, eq59_e983_d_n9, eq59_e983_d_n10, eq59_e983_d_n11, eq59_e983_d_n12, eq59_e983_d_n13, eq59_e983_d_n14, eq59_e983_d_n15, eq59_e983_d_n16, eq59_e983_d_n17, eq59_e983_d_n18, eq59_e983_d_n19, eq59_e983_d_n20, eq59_e983_d_n21, eq59_e983_d_n22, eq59_e983_d_n23, eq59_e983_d_n24, eq59_e983_d_n25, eq59_e983_d_n26, eq59_e983_d_n27, eq59_e983_d_n28, eq59_e983_d_n29];
        let eq59_branch_derivatives: [f64; 36] = [eq59_e983_d_b0, eq59_e983_d_b1, eq59_e983_d_b2, eq59_e983_d_b3, eq59_e983_d_b4, eq59_e983_d_b5, eq59_e983_d_b6, eq59_e983_d_b7, eq59_e983_d_b8, eq59_e983_d_b9, eq59_e983_d_b10, eq59_e983_d_b11, eq59_e983_d_b12, eq59_e983_d_b13, eq59_e983_d_b14, eq59_e983_d_b15, eq59_e983_d_b16, eq59_e983_d_b17, eq59_e983_d_b18, eq59_e983_d_b19, eq59_e983_d_b20, eq59_e983_d_b21, eq59_e983_d_b22, eq59_e983_d_b23, eq59_e983_d_b24, eq59_e983_d_b25, eq59_e983_d_b26, eq59_e983_d_b27, eq59_e983_d_b28, eq59_e983_d_b29, eq59_e983_d_b30, eq59_e983_d_b31, eq59_e983_d_b32, eq59_e983_d_b33, eq59_e983_d_b34, eq59_e983_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq60_e993, eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29, eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35,) = {
    if s.b[760] {
        let eq60_e986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 38, s.v[198]);
        let eq60_e986_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq60_e986_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq60_e986_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq60_e986_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq60_e986_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq60_e986_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq60_e986_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq60_e986_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq60_e986_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq60_e986_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq60_e986_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq60_e986_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq60_e986_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq60_e986_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq60_e986_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq60_e986_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq60_e986_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq60_e986_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq60_e986_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq60_e986_d_n19: f64 = (s.dn[198][19] * ddt_scale);
        let eq60_e986_d_n20: f64 = (s.dn[198][20] * ddt_scale);
        let eq60_e986_d_n21: f64 = (s.dn[198][21] * ddt_scale);
        let eq60_e986_d_n22: f64 = (s.dn[198][22] * ddt_scale);
        let eq60_e986_d_n23: f64 = (s.dn[198][23] * ddt_scale);
        let eq60_e986_d_n24: f64 = (s.dn[198][24] * ddt_scale);
        let eq60_e986_d_n25: f64 = (s.dn[198][25] * ddt_scale);
        let eq60_e986_d_n26: f64 = (s.dn[198][26] * ddt_scale);
        let eq60_e986_d_n27: f64 = (s.dn[198][27] * ddt_scale);
        let eq60_e986_d_n28: f64 = (s.dn[198][28] * ddt_scale);
        let eq60_e986_d_n29: f64 = (s.dn[198][29] * ddt_scale);
        let eq60_e986_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq60_e986_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq60_e986_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq60_e986_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq60_e986_d_b4: f64 = (s.db[198][4] * ddt_scale);
        let eq60_e986_d_b5: f64 = (s.db[198][5] * ddt_scale);
        let eq60_e986_d_b6: f64 = (s.db[198][6] * ddt_scale);
        let eq60_e986_d_b7: f64 = (s.db[198][7] * ddt_scale);
        let eq60_e986_d_b8: f64 = (s.db[198][8] * ddt_scale);
        let eq60_e986_d_b9: f64 = (s.db[198][9] * ddt_scale);
        let eq60_e986_d_b10: f64 = (s.db[198][10] * ddt_scale);
        let eq60_e986_d_b11: f64 = (s.db[198][11] * ddt_scale);
        let eq60_e986_d_b12: f64 = (s.db[198][12] * ddt_scale);
        let eq60_e986_d_b13: f64 = (s.db[198][13] * ddt_scale);
        let eq60_e986_d_b14: f64 = (s.db[198][14] * ddt_scale);
        let eq60_e986_d_b15: f64 = (s.db[198][15] * ddt_scale);
        let eq60_e986_d_b16: f64 = (s.db[198][16] * ddt_scale);
        let eq60_e986_d_b17: f64 = (s.db[198][17] * ddt_scale);
        let eq60_e986_d_b18: f64 = (s.db[198][18] * ddt_scale);
        let eq60_e986_d_b19: f64 = (s.db[198][19] * ddt_scale);
        let eq60_e986_d_b20: f64 = (s.db[198][20] * ddt_scale);
        let eq60_e986_d_b21: f64 = (s.db[198][21] * ddt_scale);
        let eq60_e986_d_b22: f64 = (s.db[198][22] * ddt_scale);
        let eq60_e986_d_b23: f64 = (s.db[198][23] * ddt_scale);
        let eq60_e986_d_b24: f64 = (s.db[198][24] * ddt_scale);
        let eq60_e986_d_b25: f64 = (s.db[198][25] * ddt_scale);
        let eq60_e986_d_b26: f64 = (s.db[198][26] * ddt_scale);
        let eq60_e986_d_b27: f64 = (s.db[198][27] * ddt_scale);
        let eq60_e986_d_b28: f64 = (s.db[198][28] * ddt_scale);
        let eq60_e986_d_b29: f64 = (s.db[198][29] * ddt_scale);
        let eq60_e986_d_b30: f64 = (s.db[198][30] * ddt_scale);
        let eq60_e986_d_b31: f64 = (s.db[198][31] * ddt_scale);
        let eq60_e986_d_b32: f64 = (s.db[198][32] * ddt_scale);
        let eq60_e986_d_b33: f64 = (s.db[198][33] * ddt_scale);
        let eq60_e986_d_b34: f64 = (s.db[198][34] * ddt_scale);
        let eq60_e986_d_b35: f64 = (s.db[198][35] * ddt_scale);
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e989_d_n7: f64 = p.p355;
        let eq60_e989_d_n15: f64 = (-p.p355);
        let eq60_e990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 39, eq60_e989);
        let eq60_e990_d_n7: f64 = (eq60_e989_d_n7 * ddt_scale);
        let eq60_e990_d_n15: f64 = (eq60_e989_d_n15 * ddt_scale);
        let eq60_e991: f64 = (eq60_e986 + eq60_e990);
        let eq60_e991_d_n7: f64 = (eq60_e986_d_n7 + eq60_e990_d_n7);
        let eq60_e991_d_n15: f64 = (eq60_e986_d_n15 + eq60_e990_d_n15);
        (eq60_e991, eq60_e986_d_n0, eq60_e986_d_n1, eq60_e986_d_n2, eq60_e986_d_n3, eq60_e986_d_n4, eq60_e986_d_n5, eq60_e986_d_n6, eq60_e991_d_n7, eq60_e986_d_n8, eq60_e986_d_n9, eq60_e986_d_n10, eq60_e986_d_n11, eq60_e986_d_n12, eq60_e986_d_n13, eq60_e986_d_n14, eq60_e991_d_n15, eq60_e986_d_n16, eq60_e986_d_n17, eq60_e986_d_n18, eq60_e986_d_n19, eq60_e986_d_n20, eq60_e986_d_n21, eq60_e986_d_n22, eq60_e986_d_n23, eq60_e986_d_n24, eq60_e986_d_n25, eq60_e986_d_n26, eq60_e986_d_n27, eq60_e986_d_n28, eq60_e986_d_n29, eq60_e986_d_b0, eq60_e986_d_b1, eq60_e986_d_b2, eq60_e986_d_b3, eq60_e986_d_b4, eq60_e986_d_b5, eq60_e986_d_b6, eq60_e986_d_b7, eq60_e986_d_b8, eq60_e986_d_b9, eq60_e986_d_b10, eq60_e986_d_b11, eq60_e986_d_b12, eq60_e986_d_b13, eq60_e986_d_b14, eq60_e986_d_b15, eq60_e986_d_b16, eq60_e986_d_b17, eq60_e986_d_b18, eq60_e986_d_b19, eq60_e986_d_b20, eq60_e986_d_b21, eq60_e986_d_b22, eq60_e986_d_b23, eq60_e986_d_b24, eq60_e986_d_b25, eq60_e986_d_b26, eq60_e986_d_b27, eq60_e986_d_b28, eq60_e986_d_b29, eq60_e986_d_b30, eq60_e986_d_b31, eq60_e986_d_b32, eq60_e986_d_b33, eq60_e986_d_b34, eq60_e986_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e993;
        let eq60_node_derivatives: [f64; 30] = [eq60_e993_d_n0, eq60_e993_d_n1, eq60_e993_d_n2, eq60_e993_d_n3, eq60_e993_d_n4, eq60_e993_d_n5, eq60_e993_d_n6, eq60_e993_d_n7, eq60_e993_d_n8, eq60_e993_d_n9, eq60_e993_d_n10, eq60_e993_d_n11, eq60_e993_d_n12, eq60_e993_d_n13, eq60_e993_d_n14, eq60_e993_d_n15, eq60_e993_d_n16, eq60_e993_d_n17, eq60_e993_d_n18, eq60_e993_d_n19, eq60_e993_d_n20, eq60_e993_d_n21, eq60_e993_d_n22, eq60_e993_d_n23, eq60_e993_d_n24, eq60_e993_d_n25, eq60_e993_d_n26, eq60_e993_d_n27, eq60_e993_d_n28, eq60_e993_d_n29];
        let eq60_branch_derivatives: [f64; 36] = [eq60_e993_d_b0, eq60_e993_d_b1, eq60_e993_d_b2, eq60_e993_d_b3, eq60_e993_d_b4, eq60_e993_d_b5, eq60_e993_d_b6, eq60_e993_d_b7, eq60_e993_d_b8, eq60_e993_d_b9, eq60_e993_d_b10, eq60_e993_d_b11, eq60_e993_d_b12, eq60_e993_d_b13, eq60_e993_d_b14, eq60_e993_d_b15, eq60_e993_d_b16, eq60_e993_d_b17, eq60_e993_d_b18, eq60_e993_d_b19, eq60_e993_d_b20, eq60_e993_d_b21, eq60_e993_d_b22, eq60_e993_d_b23, eq60_e993_d_b24, eq60_e993_d_b25, eq60_e993_d_b26, eq60_e993_d_b27, eq60_e993_d_b28, eq60_e993_d_b29, eq60_e993_d_b30, eq60_e993_d_b31, eq60_e993_d_b32, eq60_e993_d_b33, eq60_e993_d_b34, eq60_e993_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(15),
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1003, eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29, eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35,) = {
    if s.b[760] {
        let eq61_e996: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 40, s.v[199]);
        let eq61_e996_d_n0: f64 = (s.dn[199][0] * ddt_scale);
        let eq61_e996_d_n1: f64 = (s.dn[199][1] * ddt_scale);
        let eq61_e996_d_n2: f64 = (s.dn[199][2] * ddt_scale);
        let eq61_e996_d_n3: f64 = (s.dn[199][3] * ddt_scale);
        let eq61_e996_d_n4: f64 = (s.dn[199][4] * ddt_scale);
        let eq61_e996_d_n5: f64 = (s.dn[199][5] * ddt_scale);
        let eq61_e996_d_n6: f64 = (s.dn[199][6] * ddt_scale);
        let eq61_e996_d_n7: f64 = (s.dn[199][7] * ddt_scale);
        let eq61_e996_d_n8: f64 = (s.dn[199][8] * ddt_scale);
        let eq61_e996_d_n9: f64 = (s.dn[199][9] * ddt_scale);
        let eq61_e996_d_n10: f64 = (s.dn[199][10] * ddt_scale);
        let eq61_e996_d_n11: f64 = (s.dn[199][11] * ddt_scale);
        let eq61_e996_d_n12: f64 = (s.dn[199][12] * ddt_scale);
        let eq61_e996_d_n13: f64 = (s.dn[199][13] * ddt_scale);
        let eq61_e996_d_n14: f64 = (s.dn[199][14] * ddt_scale);
        let eq61_e996_d_n15: f64 = (s.dn[199][15] * ddt_scale);
        let eq61_e996_d_n16: f64 = (s.dn[199][16] * ddt_scale);
        let eq61_e996_d_n17: f64 = (s.dn[199][17] * ddt_scale);
        let eq61_e996_d_n18: f64 = (s.dn[199][18] * ddt_scale);
        let eq61_e996_d_n19: f64 = (s.dn[199][19] * ddt_scale);
        let eq61_e996_d_n20: f64 = (s.dn[199][20] * ddt_scale);
        let eq61_e996_d_n21: f64 = (s.dn[199][21] * ddt_scale);
        let eq61_e996_d_n22: f64 = (s.dn[199][22] * ddt_scale);
        let eq61_e996_d_n23: f64 = (s.dn[199][23] * ddt_scale);
        let eq61_e996_d_n24: f64 = (s.dn[199][24] * ddt_scale);
        let eq61_e996_d_n25: f64 = (s.dn[199][25] * ddt_scale);
        let eq61_e996_d_n26: f64 = (s.dn[199][26] * ddt_scale);
        let eq61_e996_d_n27: f64 = (s.dn[199][27] * ddt_scale);
        let eq61_e996_d_n28: f64 = (s.dn[199][28] * ddt_scale);
        let eq61_e996_d_n29: f64 = (s.dn[199][29] * ddt_scale);
        let eq61_e996_d_b0: f64 = (s.db[199][0] * ddt_scale);
        let eq61_e996_d_b1: f64 = (s.db[199][1] * ddt_scale);
        let eq61_e996_d_b2: f64 = (s.db[199][2] * ddt_scale);
        let eq61_e996_d_b3: f64 = (s.db[199][3] * ddt_scale);
        let eq61_e996_d_b4: f64 = (s.db[199][4] * ddt_scale);
        let eq61_e996_d_b5: f64 = (s.db[199][5] * ddt_scale);
        let eq61_e996_d_b6: f64 = (s.db[199][6] * ddt_scale);
        let eq61_e996_d_b7: f64 = (s.db[199][7] * ddt_scale);
        let eq61_e996_d_b8: f64 = (s.db[199][8] * ddt_scale);
        let eq61_e996_d_b9: f64 = (s.db[199][9] * ddt_scale);
        let eq61_e996_d_b10: f64 = (s.db[199][10] * ddt_scale);
        let eq61_e996_d_b11: f64 = (s.db[199][11] * ddt_scale);
        let eq61_e996_d_b12: f64 = (s.db[199][12] * ddt_scale);
        let eq61_e996_d_b13: f64 = (s.db[199][13] * ddt_scale);
        let eq61_e996_d_b14: f64 = (s.db[199][14] * ddt_scale);
        let eq61_e996_d_b15: f64 = (s.db[199][15] * ddt_scale);
        let eq61_e996_d_b16: f64 = (s.db[199][16] * ddt_scale);
        let eq61_e996_d_b17: f64 = (s.db[199][17] * ddt_scale);
        let eq61_e996_d_b18: f64 = (s.db[199][18] * ddt_scale);
        let eq61_e996_d_b19: f64 = (s.db[199][19] * ddt_scale);
        let eq61_e996_d_b20: f64 = (s.db[199][20] * ddt_scale);
        let eq61_e996_d_b21: f64 = (s.db[199][21] * ddt_scale);
        let eq61_e996_d_b22: f64 = (s.db[199][22] * ddt_scale);
        let eq61_e996_d_b23: f64 = (s.db[199][23] * ddt_scale);
        let eq61_e996_d_b24: f64 = (s.db[199][24] * ddt_scale);
        let eq61_e996_d_b25: f64 = (s.db[199][25] * ddt_scale);
        let eq61_e996_d_b26: f64 = (s.db[199][26] * ddt_scale);
        let eq61_e996_d_b27: f64 = (s.db[199][27] * ddt_scale);
        let eq61_e996_d_b28: f64 = (s.db[199][28] * ddt_scale);
        let eq61_e996_d_b29: f64 = (s.db[199][29] * ddt_scale);
        let eq61_e996_d_b30: f64 = (s.db[199][30] * ddt_scale);
        let eq61_e996_d_b31: f64 = (s.db[199][31] * ddt_scale);
        let eq61_e996_d_b32: f64 = (s.db[199][32] * ddt_scale);
        let eq61_e996_d_b33: f64 = (s.db[199][33] * ddt_scale);
        let eq61_e996_d_b34: f64 = (s.db[199][34] * ddt_scale);
        let eq61_e996_d_b35: f64 = (s.db[199][35] * ddt_scale);
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e999_d_n2: f64 = p.p355;
        let eq61_e999_d_n14: f64 = (-p.p355);
        let eq61_e1000: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 41, eq61_e999);
        let eq61_e1000_d_n2: f64 = (eq61_e999_d_n2 * ddt_scale);
        let eq61_e1000_d_n14: f64 = (eq61_e999_d_n14 * ddt_scale);
        let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);
        let eq61_e1001_d_n2: f64 = (eq61_e996_d_n2 + eq61_e1000_d_n2);
        let eq61_e1001_d_n14: f64 = (eq61_e996_d_n14 + eq61_e1000_d_n14);
        (eq61_e1001, eq61_e996_d_n0, eq61_e996_d_n1, eq61_e1001_d_n2, eq61_e996_d_n3, eq61_e996_d_n4, eq61_e996_d_n5, eq61_e996_d_n6, eq61_e996_d_n7, eq61_e996_d_n8, eq61_e996_d_n9, eq61_e996_d_n10, eq61_e996_d_n11, eq61_e996_d_n12, eq61_e996_d_n13, eq61_e1001_d_n14, eq61_e996_d_n15, eq61_e996_d_n16, eq61_e996_d_n17, eq61_e996_d_n18, eq61_e996_d_n19, eq61_e996_d_n20, eq61_e996_d_n21, eq61_e996_d_n22, eq61_e996_d_n23, eq61_e996_d_n24, eq61_e996_d_n25, eq61_e996_d_n26, eq61_e996_d_n27, eq61_e996_d_n28, eq61_e996_d_n29, eq61_e996_d_b0, eq61_e996_d_b1, eq61_e996_d_b2, eq61_e996_d_b3, eq61_e996_d_b4, eq61_e996_d_b5, eq61_e996_d_b6, eq61_e996_d_b7, eq61_e996_d_b8, eq61_e996_d_b9, eq61_e996_d_b10, eq61_e996_d_b11, eq61_e996_d_b12, eq61_e996_d_b13, eq61_e996_d_b14, eq61_e996_d_b15, eq61_e996_d_b16, eq61_e996_d_b17, eq61_e996_d_b18, eq61_e996_d_b19, eq61_e996_d_b20, eq61_e996_d_b21, eq61_e996_d_b22, eq61_e996_d_b23, eq61_e996_d_b24, eq61_e996_d_b25, eq61_e996_d_b26, eq61_e996_d_b27, eq61_e996_d_b28, eq61_e996_d_b29, eq61_e996_d_b30, eq61_e996_d_b31, eq61_e996_d_b32, eq61_e996_d_b33, eq61_e996_d_b34, eq61_e996_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1003;
        let eq61_node_derivatives: [f64; 30] = [eq61_e1003_d_n0, eq61_e1003_d_n1, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n5, eq61_e1003_d_n6, eq61_e1003_d_n7, eq61_e1003_d_n8, eq61_e1003_d_n9, eq61_e1003_d_n10, eq61_e1003_d_n11, eq61_e1003_d_n12, eq61_e1003_d_n13, eq61_e1003_d_n14, eq61_e1003_d_n15, eq61_e1003_d_n16, eq61_e1003_d_n17, eq61_e1003_d_n18, eq61_e1003_d_n19, eq61_e1003_d_n20, eq61_e1003_d_n21, eq61_e1003_d_n22, eq61_e1003_d_n23, eq61_e1003_d_n24, eq61_e1003_d_n25, eq61_e1003_d_n26, eq61_e1003_d_n27, eq61_e1003_d_n28, eq61_e1003_d_n29];
        let eq61_branch_derivatives: [f64; 36] = [eq61_e1003_d_b0, eq61_e1003_d_b1, eq61_e1003_d_b2, eq61_e1003_d_b3, eq61_e1003_d_b4, eq61_e1003_d_b5, eq61_e1003_d_b6, eq61_e1003_d_b7, eq61_e1003_d_b8, eq61_e1003_d_b9, eq61_e1003_d_b10, eq61_e1003_d_b11, eq61_e1003_d_b12, eq61_e1003_d_b13, eq61_e1003_d_b14, eq61_e1003_d_b15, eq61_e1003_d_b16, eq61_e1003_d_b17, eq61_e1003_d_b18, eq61_e1003_d_b19, eq61_e1003_d_b20, eq61_e1003_d_b21, eq61_e1003_d_b22, eq61_e1003_d_b23, eq61_e1003_d_b24, eq61_e1003_d_b25, eq61_e1003_d_b26, eq61_e1003_d_b27, eq61_e1003_d_b28, eq61_e1003_d_b29, eq61_e1003_d_b30, eq61_e1003_d_b31, eq61_e1003_d_b32, eq61_e1003_d_b33, eq61_e1003_d_b34, eq61_e1003_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1007,) = {
    if s.b[760] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1007;
        stamper.stamp_current_const_local(
            Some(2),
            Some(15),
            multiplicity * (eq62_value),
        );
        let (eq63_e1017, eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29, eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35,) = {
    if s.b[760] {
        let eq63_e1010: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 42, s.v[201]);
        let eq63_e1010_d_n0: f64 = (s.dn[201][0] * ddt_scale);
        let eq63_e1010_d_n1: f64 = (s.dn[201][1] * ddt_scale);
        let eq63_e1010_d_n2: f64 = (s.dn[201][2] * ddt_scale);
        let eq63_e1010_d_n3: f64 = (s.dn[201][3] * ddt_scale);
        let eq63_e1010_d_n4: f64 = (s.dn[201][4] * ddt_scale);
        let eq63_e1010_d_n5: f64 = (s.dn[201][5] * ddt_scale);
        let eq63_e1010_d_n6: f64 = (s.dn[201][6] * ddt_scale);
        let eq63_e1010_d_n7: f64 = (s.dn[201][7] * ddt_scale);
        let eq63_e1010_d_n8: f64 = (s.dn[201][8] * ddt_scale);
        let eq63_e1010_d_n9: f64 = (s.dn[201][9] * ddt_scale);
        let eq63_e1010_d_n10: f64 = (s.dn[201][10] * ddt_scale);
        let eq63_e1010_d_n11: f64 = (s.dn[201][11] * ddt_scale);
        let eq63_e1010_d_n12: f64 = (s.dn[201][12] * ddt_scale);
        let eq63_e1010_d_n13: f64 = (s.dn[201][13] * ddt_scale);
        let eq63_e1010_d_n14: f64 = (s.dn[201][14] * ddt_scale);
        let eq63_e1010_d_n15: f64 = (s.dn[201][15] * ddt_scale);
        let eq63_e1010_d_n16: f64 = (s.dn[201][16] * ddt_scale);
        let eq63_e1010_d_n17: f64 = (s.dn[201][17] * ddt_scale);
        let eq63_e1010_d_n18: f64 = (s.dn[201][18] * ddt_scale);
        let eq63_e1010_d_n19: f64 = (s.dn[201][19] * ddt_scale);
        let eq63_e1010_d_n20: f64 = (s.dn[201][20] * ddt_scale);
        let eq63_e1010_d_n21: f64 = (s.dn[201][21] * ddt_scale);
        let eq63_e1010_d_n22: f64 = (s.dn[201][22] * ddt_scale);
        let eq63_e1010_d_n23: f64 = (s.dn[201][23] * ddt_scale);
        let eq63_e1010_d_n24: f64 = (s.dn[201][24] * ddt_scale);
        let eq63_e1010_d_n25: f64 = (s.dn[201][25] * ddt_scale);
        let eq63_e1010_d_n26: f64 = (s.dn[201][26] * ddt_scale);
        let eq63_e1010_d_n27: f64 = (s.dn[201][27] * ddt_scale);
        let eq63_e1010_d_n28: f64 = (s.dn[201][28] * ddt_scale);
        let eq63_e1010_d_n29: f64 = (s.dn[201][29] * ddt_scale);
        let eq63_e1010_d_b0: f64 = (s.db[201][0] * ddt_scale);
        let eq63_e1010_d_b1: f64 = (s.db[201][1] * ddt_scale);
        let eq63_e1010_d_b2: f64 = (s.db[201][2] * ddt_scale);
        let eq63_e1010_d_b3: f64 = (s.db[201][3] * ddt_scale);
        let eq63_e1010_d_b4: f64 = (s.db[201][4] * ddt_scale);
        let eq63_e1010_d_b5: f64 = (s.db[201][5] * ddt_scale);
        let eq63_e1010_d_b6: f64 = (s.db[201][6] * ddt_scale);
        let eq63_e1010_d_b7: f64 = (s.db[201][7] * ddt_scale);
        let eq63_e1010_d_b8: f64 = (s.db[201][8] * ddt_scale);
        let eq63_e1010_d_b9: f64 = (s.db[201][9] * ddt_scale);
        let eq63_e1010_d_b10: f64 = (s.db[201][10] * ddt_scale);
        let eq63_e1010_d_b11: f64 = (s.db[201][11] * ddt_scale);
        let eq63_e1010_d_b12: f64 = (s.db[201][12] * ddt_scale);
        let eq63_e1010_d_b13: f64 = (s.db[201][13] * ddt_scale);
        let eq63_e1010_d_b14: f64 = (s.db[201][14] * ddt_scale);
        let eq63_e1010_d_b15: f64 = (s.db[201][15] * ddt_scale);
        let eq63_e1010_d_b16: f64 = (s.db[201][16] * ddt_scale);
        let eq63_e1010_d_b17: f64 = (s.db[201][17] * ddt_scale);
        let eq63_e1010_d_b18: f64 = (s.db[201][18] * ddt_scale);
        let eq63_e1010_d_b19: f64 = (s.db[201][19] * ddt_scale);
        let eq63_e1010_d_b20: f64 = (s.db[201][20] * ddt_scale);
        let eq63_e1010_d_b21: f64 = (s.db[201][21] * ddt_scale);
        let eq63_e1010_d_b22: f64 = (s.db[201][22] * ddt_scale);
        let eq63_e1010_d_b23: f64 = (s.db[201][23] * ddt_scale);
        let eq63_e1010_d_b24: f64 = (s.db[201][24] * ddt_scale);
        let eq63_e1010_d_b25: f64 = (s.db[201][25] * ddt_scale);
        let eq63_e1010_d_b26: f64 = (s.db[201][26] * ddt_scale);
        let eq63_e1010_d_b27: f64 = (s.db[201][27] * ddt_scale);
        let eq63_e1010_d_b28: f64 = (s.db[201][28] * ddt_scale);
        let eq63_e1010_d_b29: f64 = (s.db[201][29] * ddt_scale);
        let eq63_e1010_d_b30: f64 = (s.db[201][30] * ddt_scale);
        let eq63_e1010_d_b31: f64 = (s.db[201][31] * ddt_scale);
        let eq63_e1010_d_b32: f64 = (s.db[201][32] * ddt_scale);
        let eq63_e1010_d_b33: f64 = (s.db[201][33] * ddt_scale);
        let eq63_e1010_d_b34: f64 = (s.db[201][34] * ddt_scale);
        let eq63_e1010_d_b35: f64 = (s.db[201][35] * ddt_scale);
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1013_d_n7: f64 = p.p355;
        let eq63_e1013_d_n9: f64 = (-p.p355);
        let eq63_e1014: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 43, eq63_e1013);
        let eq63_e1014_d_n7: f64 = (eq63_e1013_d_n7 * ddt_scale);
        let eq63_e1014_d_n9: f64 = (eq63_e1013_d_n9 * ddt_scale);
        let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);
        let eq63_e1015_d_n7: f64 = (eq63_e1010_d_n7 + eq63_e1014_d_n7);
        let eq63_e1015_d_n9: f64 = (eq63_e1010_d_n9 + eq63_e1014_d_n9);
        (eq63_e1015, eq63_e1010_d_n0, eq63_e1010_d_n1, eq63_e1010_d_n2, eq63_e1010_d_n3, eq63_e1010_d_n4, eq63_e1010_d_n5, eq63_e1010_d_n6, eq63_e1015_d_n7, eq63_e1010_d_n8, eq63_e1015_d_n9, eq63_e1010_d_n10, eq63_e1010_d_n11, eq63_e1010_d_n12, eq63_e1010_d_n13, eq63_e1010_d_n14, eq63_e1010_d_n15, eq63_e1010_d_n16, eq63_e1010_d_n17, eq63_e1010_d_n18, eq63_e1010_d_n19, eq63_e1010_d_n20, eq63_e1010_d_n21, eq63_e1010_d_n22, eq63_e1010_d_n23, eq63_e1010_d_n24, eq63_e1010_d_n25, eq63_e1010_d_n26, eq63_e1010_d_n27, eq63_e1010_d_n28, eq63_e1010_d_n29, eq63_e1010_d_b0, eq63_e1010_d_b1, eq63_e1010_d_b2, eq63_e1010_d_b3, eq63_e1010_d_b4, eq63_e1010_d_b5, eq63_e1010_d_b6, eq63_e1010_d_b7, eq63_e1010_d_b8, eq63_e1010_d_b9, eq63_e1010_d_b10, eq63_e1010_d_b11, eq63_e1010_d_b12, eq63_e1010_d_b13, eq63_e1010_d_b14, eq63_e1010_d_b15, eq63_e1010_d_b16, eq63_e1010_d_b17, eq63_e1010_d_b18, eq63_e1010_d_b19, eq63_e1010_d_b20, eq63_e1010_d_b21, eq63_e1010_d_b22, eq63_e1010_d_b23, eq63_e1010_d_b24, eq63_e1010_d_b25, eq63_e1010_d_b26, eq63_e1010_d_b27, eq63_e1010_d_b28, eq63_e1010_d_b29, eq63_e1010_d_b30, eq63_e1010_d_b31, eq63_e1010_d_b32, eq63_e1010_d_b33, eq63_e1010_d_b34, eq63_e1010_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1017;
        let eq63_node_derivatives: [f64; 30] = [eq63_e1017_d_n0, eq63_e1017_d_n1, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n5, eq63_e1017_d_n6, eq63_e1017_d_n7, eq63_e1017_d_n8, eq63_e1017_d_n9, eq63_e1017_d_n10, eq63_e1017_d_n11, eq63_e1017_d_n12, eq63_e1017_d_n13, eq63_e1017_d_n14, eq63_e1017_d_n15, eq63_e1017_d_n16, eq63_e1017_d_n17, eq63_e1017_d_n18, eq63_e1017_d_n19, eq63_e1017_d_n20, eq63_e1017_d_n21, eq63_e1017_d_n22, eq63_e1017_d_n23, eq63_e1017_d_n24, eq63_e1017_d_n25, eq63_e1017_d_n26, eq63_e1017_d_n27, eq63_e1017_d_n28, eq63_e1017_d_n29];
        let eq63_branch_derivatives: [f64; 36] = [eq63_e1017_d_b0, eq63_e1017_d_b1, eq63_e1017_d_b2, eq63_e1017_d_b3, eq63_e1017_d_b4, eq63_e1017_d_b5, eq63_e1017_d_b6, eq63_e1017_d_b7, eq63_e1017_d_b8, eq63_e1017_d_b9, eq63_e1017_d_b10, eq63_e1017_d_b11, eq63_e1017_d_b12, eq63_e1017_d_b13, eq63_e1017_d_b14, eq63_e1017_d_b15, eq63_e1017_d_b16, eq63_e1017_d_b17, eq63_e1017_d_b18, eq63_e1017_d_b19, eq63_e1017_d_b20, eq63_e1017_d_b21, eq63_e1017_d_b22, eq63_e1017_d_b23, eq63_e1017_d_b24, eq63_e1017_d_b25, eq63_e1017_d_b26, eq63_e1017_d_b27, eq63_e1017_d_b28, eq63_e1017_d_b29, eq63_e1017_d_b30, eq63_e1017_d_b31, eq63_e1017_d_b32, eq63_e1017_d_b33, eq63_e1017_d_b34, eq63_e1017_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e1028, eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29, eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35,) = {
    if (!s.b[760]) {
        let eq64_e1021: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 44, s.v[197]);
        let eq64_e1021_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq64_e1021_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq64_e1021_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq64_e1021_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq64_e1021_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq64_e1021_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq64_e1021_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq64_e1021_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq64_e1021_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq64_e1021_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq64_e1021_d_n10: f64 = (s.dn[197][10] * ddt_scale);
        let eq64_e1021_d_n11: f64 = (s.dn[197][11] * ddt_scale);
        let eq64_e1021_d_n12: f64 = (s.dn[197][12] * ddt_scale);
        let eq64_e1021_d_n13: f64 = (s.dn[197][13] * ddt_scale);
        let eq64_e1021_d_n14: f64 = (s.dn[197][14] * ddt_scale);
        let eq64_e1021_d_n15: f64 = (s.dn[197][15] * ddt_scale);
        let eq64_e1021_d_n16: f64 = (s.dn[197][16] * ddt_scale);
        let eq64_e1021_d_n17: f64 = (s.dn[197][17] * ddt_scale);
        let eq64_e1021_d_n18: f64 = (s.dn[197][18] * ddt_scale);
        let eq64_e1021_d_n19: f64 = (s.dn[197][19] * ddt_scale);
        let eq64_e1021_d_n20: f64 = (s.dn[197][20] * ddt_scale);
        let eq64_e1021_d_n21: f64 = (s.dn[197][21] * ddt_scale);
        let eq64_e1021_d_n22: f64 = (s.dn[197][22] * ddt_scale);
        let eq64_e1021_d_n23: f64 = (s.dn[197][23] * ddt_scale);
        let eq64_e1021_d_n24: f64 = (s.dn[197][24] * ddt_scale);
        let eq64_e1021_d_n25: f64 = (s.dn[197][25] * ddt_scale);
        let eq64_e1021_d_n26: f64 = (s.dn[197][26] * ddt_scale);
        let eq64_e1021_d_n27: f64 = (s.dn[197][27] * ddt_scale);
        let eq64_e1021_d_n28: f64 = (s.dn[197][28] * ddt_scale);
        let eq64_e1021_d_n29: f64 = (s.dn[197][29] * ddt_scale);
        let eq64_e1021_d_b0: f64 = (s.db[197][0] * ddt_scale);
        let eq64_e1021_d_b1: f64 = (s.db[197][1] * ddt_scale);
        let eq64_e1021_d_b2: f64 = (s.db[197][2] * ddt_scale);
        let eq64_e1021_d_b3: f64 = (s.db[197][3] * ddt_scale);
        let eq64_e1021_d_b4: f64 = (s.db[197][4] * ddt_scale);
        let eq64_e1021_d_b5: f64 = (s.db[197][5] * ddt_scale);
        let eq64_e1021_d_b6: f64 = (s.db[197][6] * ddt_scale);
        let eq64_e1021_d_b7: f64 = (s.db[197][7] * ddt_scale);
        let eq64_e1021_d_b8: f64 = (s.db[197][8] * ddt_scale);
        let eq64_e1021_d_b9: f64 = (s.db[197][9] * ddt_scale);
        let eq64_e1021_d_b10: f64 = (s.db[197][10] * ddt_scale);
        let eq64_e1021_d_b11: f64 = (s.db[197][11] * ddt_scale);
        let eq64_e1021_d_b12: f64 = (s.db[197][12] * ddt_scale);
        let eq64_e1021_d_b13: f64 = (s.db[197][13] * ddt_scale);
        let eq64_e1021_d_b14: f64 = (s.db[197][14] * ddt_scale);
        let eq64_e1021_d_b15: f64 = (s.db[197][15] * ddt_scale);
        let eq64_e1021_d_b16: f64 = (s.db[197][16] * ddt_scale);
        let eq64_e1021_d_b17: f64 = (s.db[197][17] * ddt_scale);
        let eq64_e1021_d_b18: f64 = (s.db[197][18] * ddt_scale);
        let eq64_e1021_d_b19: f64 = (s.db[197][19] * ddt_scale);
        let eq64_e1021_d_b20: f64 = (s.db[197][20] * ddt_scale);
        let eq64_e1021_d_b21: f64 = (s.db[197][21] * ddt_scale);
        let eq64_e1021_d_b22: f64 = (s.db[197][22] * ddt_scale);
        let eq64_e1021_d_b23: f64 = (s.db[197][23] * ddt_scale);
        let eq64_e1021_d_b24: f64 = (s.db[197][24] * ddt_scale);
        let eq64_e1021_d_b25: f64 = (s.db[197][25] * ddt_scale);
        let eq64_e1021_d_b26: f64 = (s.db[197][26] * ddt_scale);
        let eq64_e1021_d_b27: f64 = (s.db[197][27] * ddt_scale);
        let eq64_e1021_d_b28: f64 = (s.db[197][28] * ddt_scale);
        let eq64_e1021_d_b29: f64 = (s.db[197][29] * ddt_scale);
        let eq64_e1021_d_b30: f64 = (s.db[197][30] * ddt_scale);
        let eq64_e1021_d_b31: f64 = (s.db[197][31] * ddt_scale);
        let eq64_e1021_d_b32: f64 = (s.db[197][32] * ddt_scale);
        let eq64_e1021_d_b33: f64 = (s.db[197][33] * ddt_scale);
        let eq64_e1021_d_b34: f64 = (s.db[197][34] * ddt_scale);
        let eq64_e1021_d_b35: f64 = (s.db[197][35] * ddt_scale);
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1024_d_n2: f64 = p.p355;
        let eq64_e1024_d_n14: f64 = (-p.p355);
        let eq64_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 45, eq64_e1024);
        let eq64_e1025_d_n2: f64 = (eq64_e1024_d_n2 * ddt_scale);
        let eq64_e1025_d_n14: f64 = (eq64_e1024_d_n14 * ddt_scale);
        let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);
        let eq64_e1026_d_n2: f64 = (eq64_e1021_d_n2 + eq64_e1025_d_n2);
        let eq64_e1026_d_n14: f64 = (eq64_e1021_d_n14 + eq64_e1025_d_n14);
        (eq64_e1026, eq64_e1021_d_n0, eq64_e1021_d_n1, eq64_e1026_d_n2, eq64_e1021_d_n3, eq64_e1021_d_n4, eq64_e1021_d_n5, eq64_e1021_d_n6, eq64_e1021_d_n7, eq64_e1021_d_n8, eq64_e1021_d_n9, eq64_e1021_d_n10, eq64_e1021_d_n11, eq64_e1021_d_n12, eq64_e1021_d_n13, eq64_e1026_d_n14, eq64_e1021_d_n15, eq64_e1021_d_n16, eq64_e1021_d_n17, eq64_e1021_d_n18, eq64_e1021_d_n19, eq64_e1021_d_n20, eq64_e1021_d_n21, eq64_e1021_d_n22, eq64_e1021_d_n23, eq64_e1021_d_n24, eq64_e1021_d_n25, eq64_e1021_d_n26, eq64_e1021_d_n27, eq64_e1021_d_n28, eq64_e1021_d_n29, eq64_e1021_d_b0, eq64_e1021_d_b1, eq64_e1021_d_b2, eq64_e1021_d_b3, eq64_e1021_d_b4, eq64_e1021_d_b5, eq64_e1021_d_b6, eq64_e1021_d_b7, eq64_e1021_d_b8, eq64_e1021_d_b9, eq64_e1021_d_b10, eq64_e1021_d_b11, eq64_e1021_d_b12, eq64_e1021_d_b13, eq64_e1021_d_b14, eq64_e1021_d_b15, eq64_e1021_d_b16, eq64_e1021_d_b17, eq64_e1021_d_b18, eq64_e1021_d_b19, eq64_e1021_d_b20, eq64_e1021_d_b21, eq64_e1021_d_b22, eq64_e1021_d_b23, eq64_e1021_d_b24, eq64_e1021_d_b25, eq64_e1021_d_b26, eq64_e1021_d_b27, eq64_e1021_d_b28, eq64_e1021_d_b29, eq64_e1021_d_b30, eq64_e1021_d_b31, eq64_e1021_d_b32, eq64_e1021_d_b33, eq64_e1021_d_b34, eq64_e1021_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1028;
        let eq64_node_derivatives: [f64; 30] = [eq64_e1028_d_n0, eq64_e1028_d_n1, eq64_e1028_d_n2, eq64_e1028_d_n3, eq64_e1028_d_n4, eq64_e1028_d_n5, eq64_e1028_d_n6, eq64_e1028_d_n7, eq64_e1028_d_n8, eq64_e1028_d_n9, eq64_e1028_d_n10, eq64_e1028_d_n11, eq64_e1028_d_n12, eq64_e1028_d_n13, eq64_e1028_d_n14, eq64_e1028_d_n15, eq64_e1028_d_n16, eq64_e1028_d_n17, eq64_e1028_d_n18, eq64_e1028_d_n19, eq64_e1028_d_n20, eq64_e1028_d_n21, eq64_e1028_d_n22, eq64_e1028_d_n23, eq64_e1028_d_n24, eq64_e1028_d_n25, eq64_e1028_d_n26, eq64_e1028_d_n27, eq64_e1028_d_n28, eq64_e1028_d_n29];
        let eq64_branch_derivatives: [f64; 36] = [eq64_e1028_d_b0, eq64_e1028_d_b1, eq64_e1028_d_b2, eq64_e1028_d_b3, eq64_e1028_d_b4, eq64_e1028_d_b5, eq64_e1028_d_b6, eq64_e1028_d_b7, eq64_e1028_d_b8, eq64_e1028_d_b9, eq64_e1028_d_b10, eq64_e1028_d_b11, eq64_e1028_d_b12, eq64_e1028_d_b13, eq64_e1028_d_b14, eq64_e1028_d_b15, eq64_e1028_d_b16, eq64_e1028_d_b17, eq64_e1028_d_b18, eq64_e1028_d_b19, eq64_e1028_d_b20, eq64_e1028_d_b21, eq64_e1028_d_b22, eq64_e1028_d_b23, eq64_e1028_d_b24, eq64_e1028_d_b25, eq64_e1028_d_b26, eq64_e1028_d_b27, eq64_e1028_d_b28, eq64_e1028_d_b29, eq64_e1028_d_b30, eq64_e1028_d_b31, eq64_e1028_d_b32, eq64_e1028_d_b33, eq64_e1028_d_b34, eq64_e1028_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq64_value),
            &eq64_node_derivatives,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq65_e1039, eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29, eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35,) = {
    if (!s.b[760]) {
        let eq65_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 46, s.v[198]);
        let eq65_e1032_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq65_e1032_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq65_e1032_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq65_e1032_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq65_e1032_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq65_e1032_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq65_e1032_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq65_e1032_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq65_e1032_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq65_e1032_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq65_e1032_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq65_e1032_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq65_e1032_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq65_e1032_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq65_e1032_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq65_e1032_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq65_e1032_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq65_e1032_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq65_e1032_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq65_e1032_d_n19: f64 = (s.dn[198][19] * ddt_scale);
        let eq65_e1032_d_n20: f64 = (s.dn[198][20] * ddt_scale);
        let eq65_e1032_d_n21: f64 = (s.dn[198][21] * ddt_scale);
        let eq65_e1032_d_n22: f64 = (s.dn[198][22] * ddt_scale);
        let eq65_e1032_d_n23: f64 = (s.dn[198][23] * ddt_scale);
        let eq65_e1032_d_n24: f64 = (s.dn[198][24] * ddt_scale);
        let eq65_e1032_d_n25: f64 = (s.dn[198][25] * ddt_scale);
        let eq65_e1032_d_n26: f64 = (s.dn[198][26] * ddt_scale);
        let eq65_e1032_d_n27: f64 = (s.dn[198][27] * ddt_scale);
        let eq65_e1032_d_n28: f64 = (s.dn[198][28] * ddt_scale);
        let eq65_e1032_d_n29: f64 = (s.dn[198][29] * ddt_scale);
        let eq65_e1032_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq65_e1032_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq65_e1032_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq65_e1032_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq65_e1032_d_b4: f64 = (s.db[198][4] * ddt_scale);
        let eq65_e1032_d_b5: f64 = (s.db[198][5] * ddt_scale);
        let eq65_e1032_d_b6: f64 = (s.db[198][6] * ddt_scale);
        let eq65_e1032_d_b7: f64 = (s.db[198][7] * ddt_scale);
        let eq65_e1032_d_b8: f64 = (s.db[198][8] * ddt_scale);
        let eq65_e1032_d_b9: f64 = (s.db[198][9] * ddt_scale);
        let eq65_e1032_d_b10: f64 = (s.db[198][10] * ddt_scale);
        let eq65_e1032_d_b11: f64 = (s.db[198][11] * ddt_scale);
        let eq65_e1032_d_b12: f64 = (s.db[198][12] * ddt_scale);
        let eq65_e1032_d_b13: f64 = (s.db[198][13] * ddt_scale);
        let eq65_e1032_d_b14: f64 = (s.db[198][14] * ddt_scale);
        let eq65_e1032_d_b15: f64 = (s.db[198][15] * ddt_scale);
        let eq65_e1032_d_b16: f64 = (s.db[198][16] * ddt_scale);
        let eq65_e1032_d_b17: f64 = (s.db[198][17] * ddt_scale);
        let eq65_e1032_d_b18: f64 = (s.db[198][18] * ddt_scale);
        let eq65_e1032_d_b19: f64 = (s.db[198][19] * ddt_scale);
        let eq65_e1032_d_b20: f64 = (s.db[198][20] * ddt_scale);
        let eq65_e1032_d_b21: f64 = (s.db[198][21] * ddt_scale);
        let eq65_e1032_d_b22: f64 = (s.db[198][22] * ddt_scale);
        let eq65_e1032_d_b23: f64 = (s.db[198][23] * ddt_scale);
        let eq65_e1032_d_b24: f64 = (s.db[198][24] * ddt_scale);
        let eq65_e1032_d_b25: f64 = (s.db[198][25] * ddt_scale);
        let eq65_e1032_d_b26: f64 = (s.db[198][26] * ddt_scale);
        let eq65_e1032_d_b27: f64 = (s.db[198][27] * ddt_scale);
        let eq65_e1032_d_b28: f64 = (s.db[198][28] * ddt_scale);
        let eq65_e1032_d_b29: f64 = (s.db[198][29] * ddt_scale);
        let eq65_e1032_d_b30: f64 = (s.db[198][30] * ddt_scale);
        let eq65_e1032_d_b31: f64 = (s.db[198][31] * ddt_scale);
        let eq65_e1032_d_b32: f64 = (s.db[198][32] * ddt_scale);
        let eq65_e1032_d_b33: f64 = (s.db[198][33] * ddt_scale);
        let eq65_e1032_d_b34: f64 = (s.db[198][34] * ddt_scale);
        let eq65_e1032_d_b35: f64 = (s.db[198][35] * ddt_scale);
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1035_d_n2: f64 = p.p355;
        let eq65_e1035_d_n15: f64 = (-p.p355);
        let eq65_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 47, eq65_e1035);
        let eq65_e1036_d_n2: f64 = (eq65_e1035_d_n2 * ddt_scale);
        let eq65_e1036_d_n15: f64 = (eq65_e1035_d_n15 * ddt_scale);
        let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);
        let eq65_e1037_d_n2: f64 = (eq65_e1032_d_n2 + eq65_e1036_d_n2);
        let eq65_e1037_d_n15: f64 = (eq65_e1032_d_n15 + eq65_e1036_d_n15);
        (eq65_e1037, eq65_e1032_d_n0, eq65_e1032_d_n1, eq65_e1037_d_n2, eq65_e1032_d_n3, eq65_e1032_d_n4, eq65_e1032_d_n5, eq65_e1032_d_n6, eq65_e1032_d_n7, eq65_e1032_d_n8, eq65_e1032_d_n9, eq65_e1032_d_n10, eq65_e1032_d_n11, eq65_e1032_d_n12, eq65_e1032_d_n13, eq65_e1032_d_n14, eq65_e1037_d_n15, eq65_e1032_d_n16, eq65_e1032_d_n17, eq65_e1032_d_n18, eq65_e1032_d_n19, eq65_e1032_d_n20, eq65_e1032_d_n21, eq65_e1032_d_n22, eq65_e1032_d_n23, eq65_e1032_d_n24, eq65_e1032_d_n25, eq65_e1032_d_n26, eq65_e1032_d_n27, eq65_e1032_d_n28, eq65_e1032_d_n29, eq65_e1032_d_b0, eq65_e1032_d_b1, eq65_e1032_d_b2, eq65_e1032_d_b3, eq65_e1032_d_b4, eq65_e1032_d_b5, eq65_e1032_d_b6, eq65_e1032_d_b7, eq65_e1032_d_b8, eq65_e1032_d_b9, eq65_e1032_d_b10, eq65_e1032_d_b11, eq65_e1032_d_b12, eq65_e1032_d_b13, eq65_e1032_d_b14, eq65_e1032_d_b15, eq65_e1032_d_b16, eq65_e1032_d_b17, eq65_e1032_d_b18, eq65_e1032_d_b19, eq65_e1032_d_b20, eq65_e1032_d_b21, eq65_e1032_d_b22, eq65_e1032_d_b23, eq65_e1032_d_b24, eq65_e1032_d_b25, eq65_e1032_d_b26, eq65_e1032_d_b27, eq65_e1032_d_b28, eq65_e1032_d_b29, eq65_e1032_d_b30, eq65_e1032_d_b31, eq65_e1032_d_b32, eq65_e1032_d_b33, eq65_e1032_d_b34, eq65_e1032_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1039;
        let eq65_node_derivatives: [f64; 30] = [eq65_e1039_d_n0, eq65_e1039_d_n1, eq65_e1039_d_n2, eq65_e1039_d_n3, eq65_e1039_d_n4, eq65_e1039_d_n5, eq65_e1039_d_n6, eq65_e1039_d_n7, eq65_e1039_d_n8, eq65_e1039_d_n9, eq65_e1039_d_n10, eq65_e1039_d_n11, eq65_e1039_d_n12, eq65_e1039_d_n13, eq65_e1039_d_n14, eq65_e1039_d_n15, eq65_e1039_d_n16, eq65_e1039_d_n17, eq65_e1039_d_n18, eq65_e1039_d_n19, eq65_e1039_d_n20, eq65_e1039_d_n21, eq65_e1039_d_n22, eq65_e1039_d_n23, eq65_e1039_d_n24, eq65_e1039_d_n25, eq65_e1039_d_n26, eq65_e1039_d_n27, eq65_e1039_d_n28, eq65_e1039_d_n29];
        let eq65_branch_derivatives: [f64; 36] = [eq65_e1039_d_b0, eq65_e1039_d_b1, eq65_e1039_d_b2, eq65_e1039_d_b3, eq65_e1039_d_b4, eq65_e1039_d_b5, eq65_e1039_d_b6, eq65_e1039_d_b7, eq65_e1039_d_b8, eq65_e1039_d_b9, eq65_e1039_d_b10, eq65_e1039_d_b11, eq65_e1039_d_b12, eq65_e1039_d_b13, eq65_e1039_d_b14, eq65_e1039_d_b15, eq65_e1039_d_b16, eq65_e1039_d_b17, eq65_e1039_d_b18, eq65_e1039_d_b19, eq65_e1039_d_b20, eq65_e1039_d_b21, eq65_e1039_d_b22, eq65_e1039_d_b23, eq65_e1039_d_b24, eq65_e1039_d_b25, eq65_e1039_d_b26, eq65_e1039_d_b27, eq65_e1039_d_b28, eq65_e1039_d_b29, eq65_e1039_d_b30, eq65_e1039_d_b31, eq65_e1039_d_b32, eq65_e1039_d_b33, eq65_e1039_d_b34, eq65_e1039_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(15),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e1050, eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29, eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35,) = {
    if (!s.b[760]) {
        let eq66_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 48, s.v[199]);
        let eq66_e1043_d_n0: f64 = (s.dn[199][0] * ddt_scale);
        let eq66_e1043_d_n1: f64 = (s.dn[199][1] * ddt_scale);
        let eq66_e1043_d_n2: f64 = (s.dn[199][2] * ddt_scale);
        let eq66_e1043_d_n3: f64 = (s.dn[199][3] * ddt_scale);
        let eq66_e1043_d_n4: f64 = (s.dn[199][4] * ddt_scale);
        let eq66_e1043_d_n5: f64 = (s.dn[199][5] * ddt_scale);
        let eq66_e1043_d_n6: f64 = (s.dn[199][6] * ddt_scale);
        let eq66_e1043_d_n7: f64 = (s.dn[199][7] * ddt_scale);
        let eq66_e1043_d_n8: f64 = (s.dn[199][8] * ddt_scale);
        let eq66_e1043_d_n9: f64 = (s.dn[199][9] * ddt_scale);
        let eq66_e1043_d_n10: f64 = (s.dn[199][10] * ddt_scale);
        let eq66_e1043_d_n11: f64 = (s.dn[199][11] * ddt_scale);
        let eq66_e1043_d_n12: f64 = (s.dn[199][12] * ddt_scale);
        let eq66_e1043_d_n13: f64 = (s.dn[199][13] * ddt_scale);
        let eq66_e1043_d_n14: f64 = (s.dn[199][14] * ddt_scale);
        let eq66_e1043_d_n15: f64 = (s.dn[199][15] * ddt_scale);
        let eq66_e1043_d_n16: f64 = (s.dn[199][16] * ddt_scale);
        let eq66_e1043_d_n17: f64 = (s.dn[199][17] * ddt_scale);
        let eq66_e1043_d_n18: f64 = (s.dn[199][18] * ddt_scale);
        let eq66_e1043_d_n19: f64 = (s.dn[199][19] * ddt_scale);
        let eq66_e1043_d_n20: f64 = (s.dn[199][20] * ddt_scale);
        let eq66_e1043_d_n21: f64 = (s.dn[199][21] * ddt_scale);
        let eq66_e1043_d_n22: f64 = (s.dn[199][22] * ddt_scale);
        let eq66_e1043_d_n23: f64 = (s.dn[199][23] * ddt_scale);
        let eq66_e1043_d_n24: f64 = (s.dn[199][24] * ddt_scale);
        let eq66_e1043_d_n25: f64 = (s.dn[199][25] * ddt_scale);
        let eq66_e1043_d_n26: f64 = (s.dn[199][26] * ddt_scale);
        let eq66_e1043_d_n27: f64 = (s.dn[199][27] * ddt_scale);
        let eq66_e1043_d_n28: f64 = (s.dn[199][28] * ddt_scale);
        let eq66_e1043_d_n29: f64 = (s.dn[199][29] * ddt_scale);
        let eq66_e1043_d_b0: f64 = (s.db[199][0] * ddt_scale);
        let eq66_e1043_d_b1: f64 = (s.db[199][1] * ddt_scale);
        let eq66_e1043_d_b2: f64 = (s.db[199][2] * ddt_scale);
        let eq66_e1043_d_b3: f64 = (s.db[199][3] * ddt_scale);
        let eq66_e1043_d_b4: f64 = (s.db[199][4] * ddt_scale);
        let eq66_e1043_d_b5: f64 = (s.db[199][5] * ddt_scale);
        let eq66_e1043_d_b6: f64 = (s.db[199][6] * ddt_scale);
        let eq66_e1043_d_b7: f64 = (s.db[199][7] * ddt_scale);
        let eq66_e1043_d_b8: f64 = (s.db[199][8] * ddt_scale);
        let eq66_e1043_d_b9: f64 = (s.db[199][9] * ddt_scale);
        let eq66_e1043_d_b10: f64 = (s.db[199][10] * ddt_scale);
        let eq66_e1043_d_b11: f64 = (s.db[199][11] * ddt_scale);
        let eq66_e1043_d_b12: f64 = (s.db[199][12] * ddt_scale);
        let eq66_e1043_d_b13: f64 = (s.db[199][13] * ddt_scale);
        let eq66_e1043_d_b14: f64 = (s.db[199][14] * ddt_scale);
        let eq66_e1043_d_b15: f64 = (s.db[199][15] * ddt_scale);
        let eq66_e1043_d_b16: f64 = (s.db[199][16] * ddt_scale);
        let eq66_e1043_d_b17: f64 = (s.db[199][17] * ddt_scale);
        let eq66_e1043_d_b18: f64 = (s.db[199][18] * ddt_scale);
        let eq66_e1043_d_b19: f64 = (s.db[199][19] * ddt_scale);
        let eq66_e1043_d_b20: f64 = (s.db[199][20] * ddt_scale);
        let eq66_e1043_d_b21: f64 = (s.db[199][21] * ddt_scale);
        let eq66_e1043_d_b22: f64 = (s.db[199][22] * ddt_scale);
        let eq66_e1043_d_b23: f64 = (s.db[199][23] * ddt_scale);
        let eq66_e1043_d_b24: f64 = (s.db[199][24] * ddt_scale);
        let eq66_e1043_d_b25: f64 = (s.db[199][25] * ddt_scale);
        let eq66_e1043_d_b26: f64 = (s.db[199][26] * ddt_scale);
        let eq66_e1043_d_b27: f64 = (s.db[199][27] * ddt_scale);
        let eq66_e1043_d_b28: f64 = (s.db[199][28] * ddt_scale);
        let eq66_e1043_d_b29: f64 = (s.db[199][29] * ddt_scale);
        let eq66_e1043_d_b30: f64 = (s.db[199][30] * ddt_scale);
        let eq66_e1043_d_b31: f64 = (s.db[199][31] * ddt_scale);
        let eq66_e1043_d_b32: f64 = (s.db[199][32] * ddt_scale);
        let eq66_e1043_d_b33: f64 = (s.db[199][33] * ddt_scale);
        let eq66_e1043_d_b34: f64 = (s.db[199][34] * ddt_scale);
        let eq66_e1043_d_b35: f64 = (s.db[199][35] * ddt_scale);
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1046_d_n7: f64 = p.p355;
        let eq66_e1046_d_n14: f64 = (-p.p355);
        let eq66_e1047: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 49, eq66_e1046);
        let eq66_e1047_d_n7: f64 = (eq66_e1046_d_n7 * ddt_scale);
        let eq66_e1047_d_n14: f64 = (eq66_e1046_d_n14 * ddt_scale);
        let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);
        let eq66_e1048_d_n7: f64 = (eq66_e1043_d_n7 + eq66_e1047_d_n7);
        let eq66_e1048_d_n14: f64 = (eq66_e1043_d_n14 + eq66_e1047_d_n14);
        (eq66_e1048, eq66_e1043_d_n0, eq66_e1043_d_n1, eq66_e1043_d_n2, eq66_e1043_d_n3, eq66_e1043_d_n4, eq66_e1043_d_n5, eq66_e1043_d_n6, eq66_e1048_d_n7, eq66_e1043_d_n8, eq66_e1043_d_n9, eq66_e1043_d_n10, eq66_e1043_d_n11, eq66_e1043_d_n12, eq66_e1043_d_n13, eq66_e1048_d_n14, eq66_e1043_d_n15, eq66_e1043_d_n16, eq66_e1043_d_n17, eq66_e1043_d_n18, eq66_e1043_d_n19, eq66_e1043_d_n20, eq66_e1043_d_n21, eq66_e1043_d_n22, eq66_e1043_d_n23, eq66_e1043_d_n24, eq66_e1043_d_n25, eq66_e1043_d_n26, eq66_e1043_d_n27, eq66_e1043_d_n28, eq66_e1043_d_n29, eq66_e1043_d_b0, eq66_e1043_d_b1, eq66_e1043_d_b2, eq66_e1043_d_b3, eq66_e1043_d_b4, eq66_e1043_d_b5, eq66_e1043_d_b6, eq66_e1043_d_b7, eq66_e1043_d_b8, eq66_e1043_d_b9, eq66_e1043_d_b10, eq66_e1043_d_b11, eq66_e1043_d_b12, eq66_e1043_d_b13, eq66_e1043_d_b14, eq66_e1043_d_b15, eq66_e1043_d_b16, eq66_e1043_d_b17, eq66_e1043_d_b18, eq66_e1043_d_b19, eq66_e1043_d_b20, eq66_e1043_d_b21, eq66_e1043_d_b22, eq66_e1043_d_b23, eq66_e1043_d_b24, eq66_e1043_d_b25, eq66_e1043_d_b26, eq66_e1043_d_b27, eq66_e1043_d_b28, eq66_e1043_d_b29, eq66_e1043_d_b30, eq66_e1043_d_b31, eq66_e1043_d_b32, eq66_e1043_d_b33, eq66_e1043_d_b34, eq66_e1043_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1050;
        let eq66_node_derivatives: [f64; 30] = [eq66_e1050_d_n0, eq66_e1050_d_n1, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n5, eq66_e1050_d_n6, eq66_e1050_d_n7, eq66_e1050_d_n8, eq66_e1050_d_n9, eq66_e1050_d_n10, eq66_e1050_d_n11, eq66_e1050_d_n12, eq66_e1050_d_n13, eq66_e1050_d_n14, eq66_e1050_d_n15, eq66_e1050_d_n16, eq66_e1050_d_n17, eq66_e1050_d_n18, eq66_e1050_d_n19, eq66_e1050_d_n20, eq66_e1050_d_n21, eq66_e1050_d_n22, eq66_e1050_d_n23, eq66_e1050_d_n24, eq66_e1050_d_n25, eq66_e1050_d_n26, eq66_e1050_d_n27, eq66_e1050_d_n28, eq66_e1050_d_n29];
        let eq66_branch_derivatives: [f64; 36] = [eq66_e1050_d_b0, eq66_e1050_d_b1, eq66_e1050_d_b2, eq66_e1050_d_b3, eq66_e1050_d_b4, eq66_e1050_d_b5, eq66_e1050_d_b6, eq66_e1050_d_b7, eq66_e1050_d_b8, eq66_e1050_d_b9, eq66_e1050_d_b10, eq66_e1050_d_b11, eq66_e1050_d_b12, eq66_e1050_d_b13, eq66_e1050_d_b14, eq66_e1050_d_b15, eq66_e1050_d_b16, eq66_e1050_d_b17, eq66_e1050_d_b18, eq66_e1050_d_b19, eq66_e1050_d_b20, eq66_e1050_d_b21, eq66_e1050_d_b22, eq66_e1050_d_b23, eq66_e1050_d_b24, eq66_e1050_d_b25, eq66_e1050_d_b26, eq66_e1050_d_b27, eq66_e1050_d_b28, eq66_e1050_d_b29, eq66_e1050_d_b30, eq66_e1050_d_b31, eq66_e1050_d_b32, eq66_e1050_d_b33, eq66_e1050_d_b34, eq66_e1050_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1055,) = {
    if (!s.b[760]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1055;
        stamper.stamp_current_const_local(
            Some(7),
            Some(15),
            multiplicity * (eq67_value),
        );
        let (eq68_e1060,) = {
    if (!s.b[760]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1060;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq68_value),
        );
        let eq69_e1062: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 50, s.v[200]);
        let eq69_e1062_d_n0: f64 = (s.dn[200][0] * ddt_scale);
        let eq69_e1062_d_n1: f64 = (s.dn[200][1] * ddt_scale);
        let eq69_e1062_d_n2: f64 = (s.dn[200][2] * ddt_scale);
        let eq69_e1062_d_n3: f64 = (s.dn[200][3] * ddt_scale);
        let eq69_e1062_d_n4: f64 = (s.dn[200][4] * ddt_scale);
        let eq69_e1062_d_n5: f64 = (s.dn[200][5] * ddt_scale);
        let eq69_e1062_d_n6: f64 = (s.dn[200][6] * ddt_scale);
        let eq69_e1062_d_n7: f64 = (s.dn[200][7] * ddt_scale);
        let eq69_e1062_d_n8: f64 = (s.dn[200][8] * ddt_scale);
        let eq69_e1062_d_n9: f64 = (s.dn[200][9] * ddt_scale);
        let eq69_e1062_d_n10: f64 = (s.dn[200][10] * ddt_scale);
        let eq69_e1062_d_n11: f64 = (s.dn[200][11] * ddt_scale);
        let eq69_e1062_d_n12: f64 = (s.dn[200][12] * ddt_scale);
        let eq69_e1062_d_n13: f64 = (s.dn[200][13] * ddt_scale);
        let eq69_e1062_d_n14: f64 = (s.dn[200][14] * ddt_scale);
        let eq69_e1062_d_n15: f64 = (s.dn[200][15] * ddt_scale);
        let eq69_e1062_d_n16: f64 = (s.dn[200][16] * ddt_scale);
        let eq69_e1062_d_n17: f64 = (s.dn[200][17] * ddt_scale);
        let eq69_e1062_d_n18: f64 = (s.dn[200][18] * ddt_scale);
        let eq69_e1062_d_n19: f64 = (s.dn[200][19] * ddt_scale);
        let eq69_e1062_d_n20: f64 = (s.dn[200][20] * ddt_scale);
        let eq69_e1062_d_n21: f64 = (s.dn[200][21] * ddt_scale);
        let eq69_e1062_d_n22: f64 = (s.dn[200][22] * ddt_scale);
        let eq69_e1062_d_n23: f64 = (s.dn[200][23] * ddt_scale);
        let eq69_e1062_d_n24: f64 = (s.dn[200][24] * ddt_scale);
        let eq69_e1062_d_n25: f64 = (s.dn[200][25] * ddt_scale);
        let eq69_e1062_d_n26: f64 = (s.dn[200][26] * ddt_scale);
        let eq69_e1062_d_n27: f64 = (s.dn[200][27] * ddt_scale);
        let eq69_e1062_d_n28: f64 = (s.dn[200][28] * ddt_scale);
        let eq69_e1062_d_n29: f64 = (s.dn[200][29] * ddt_scale);
        let eq69_e1062_d_b0: f64 = (s.db[200][0] * ddt_scale);
        let eq69_e1062_d_b1: f64 = (s.db[200][1] * ddt_scale);
        let eq69_e1062_d_b2: f64 = (s.db[200][2] * ddt_scale);
        let eq69_e1062_d_b3: f64 = (s.db[200][3] * ddt_scale);
        let eq69_e1062_d_b4: f64 = (s.db[200][4] * ddt_scale);
        let eq69_e1062_d_b5: f64 = (s.db[200][5] * ddt_scale);
        let eq69_e1062_d_b6: f64 = (s.db[200][6] * ddt_scale);
        let eq69_e1062_d_b7: f64 = (s.db[200][7] * ddt_scale);
        let eq69_e1062_d_b8: f64 = (s.db[200][8] * ddt_scale);
        let eq69_e1062_d_b9: f64 = (s.db[200][9] * ddt_scale);
        let eq69_e1062_d_b10: f64 = (s.db[200][10] * ddt_scale);
        let eq69_e1062_d_b11: f64 = (s.db[200][11] * ddt_scale);
        let eq69_e1062_d_b12: f64 = (s.db[200][12] * ddt_scale);
        let eq69_e1062_d_b13: f64 = (s.db[200][13] * ddt_scale);
        let eq69_e1062_d_b14: f64 = (s.db[200][14] * ddt_scale);
        let eq69_e1062_d_b15: f64 = (s.db[200][15] * ddt_scale);
        let eq69_e1062_d_b16: f64 = (s.db[200][16] * ddt_scale);
        let eq69_e1062_d_b17: f64 = (s.db[200][17] * ddt_scale);
        let eq69_e1062_d_b18: f64 = (s.db[200][18] * ddt_scale);
        let eq69_e1062_d_b19: f64 = (s.db[200][19] * ddt_scale);
        let eq69_e1062_d_b20: f64 = (s.db[200][20] * ddt_scale);
        let eq69_e1062_d_b21: f64 = (s.db[200][21] * ddt_scale);
        let eq69_e1062_d_b22: f64 = (s.db[200][22] * ddt_scale);
        let eq69_e1062_d_b23: f64 = (s.db[200][23] * ddt_scale);
        let eq69_e1062_d_b24: f64 = (s.db[200][24] * ddt_scale);
        let eq69_e1062_d_b25: f64 = (s.db[200][25] * ddt_scale);
        let eq69_e1062_d_b26: f64 = (s.db[200][26] * ddt_scale);
        let eq69_e1062_d_b27: f64 = (s.db[200][27] * ddt_scale);
        let eq69_e1062_d_b28: f64 = (s.db[200][28] * ddt_scale);
        let eq69_e1062_d_b29: f64 = (s.db[200][29] * ddt_scale);
        let eq69_e1062_d_b30: f64 = (s.db[200][30] * ddt_scale);
        let eq69_e1062_d_b31: f64 = (s.db[200][31] * ddt_scale);
        let eq69_e1062_d_b32: f64 = (s.db[200][32] * ddt_scale);
        let eq69_e1062_d_b33: f64 = (s.db[200][33] * ddt_scale);
        let eq69_e1062_d_b34: f64 = (s.db[200][34] * ddt_scale);
        let eq69_e1062_d_b35: f64 = (s.db[200][35] * ddt_scale);
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1065_d_n3: f64 = p.p355;
        let eq69_e1065_d_n14: f64 = (-p.p355);
        let eq69_e1066: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 51, eq69_e1065);
        let eq69_e1066_d_n3: f64 = (eq69_e1065_d_n3 * ddt_scale);
        let eq69_e1066_d_n14: f64 = (eq69_e1065_d_n14 * ddt_scale);
        let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);
        let eq69_e1067_d_n3: f64 = (eq69_e1062_d_n3 + eq69_e1066_d_n3);
        let eq69_e1067_d_n14: f64 = (eq69_e1062_d_n14 + eq69_e1066_d_n14);
        let eq69_value: f64 = eq69_e1067;
        let eq69_node_derivatives: [f64; 30] = [eq69_e1062_d_n0, eq69_e1062_d_n1, eq69_e1062_d_n2, eq69_e1067_d_n3, eq69_e1062_d_n4, eq69_e1062_d_n5, eq69_e1062_d_n6, eq69_e1062_d_n7, eq69_e1062_d_n8, eq69_e1062_d_n9, eq69_e1062_d_n10, eq69_e1062_d_n11, eq69_e1062_d_n12, eq69_e1062_d_n13, eq69_e1067_d_n14, eq69_e1062_d_n15, eq69_e1062_d_n16, eq69_e1062_d_n17, eq69_e1062_d_n18, eq69_e1062_d_n19, eq69_e1062_d_n20, eq69_e1062_d_n21, eq69_e1062_d_n22, eq69_e1062_d_n23, eq69_e1062_d_n24, eq69_e1062_d_n25, eq69_e1062_d_n26, eq69_e1062_d_n27, eq69_e1062_d_n28, eq69_e1062_d_n29];
        let eq69_branch_derivatives: [f64; 36] = [eq69_e1062_d_b0, eq69_e1062_d_b1, eq69_e1062_d_b2, eq69_e1062_d_b3, eq69_e1062_d_b4, eq69_e1062_d_b5, eq69_e1062_d_b6, eq69_e1062_d_b7, eq69_e1062_d_b8, eq69_e1062_d_b9, eq69_e1062_d_b10, eq69_e1062_d_b11, eq69_e1062_d_b12, eq69_e1062_d_b13, eq69_e1062_d_b14, eq69_e1062_d_b15, eq69_e1062_d_b16, eq69_e1062_d_b17, eq69_e1062_d_b18, eq69_e1062_d_b19, eq69_e1062_d_b20, eq69_e1062_d_b21, eq69_e1062_d_b22, eq69_e1062_d_b23, eq69_e1062_d_b24, eq69_e1062_d_b25, eq69_e1062_d_b26, eq69_e1062_d_b27, eq69_e1062_d_b28, eq69_e1062_d_b29, eq69_e1062_d_b30, eq69_e1062_d_b31, eq69_e1062_d_b32, eq69_e1062_d_b33, eq69_e1062_d_b34, eq69_e1062_d_b35];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(14),
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e1075, eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29, eq70_e1075_d_b0, eq70_e1075_d_b1, eq70_e1075_d_b2, eq70_e1075_d_b3, eq70_e1075_d_b4, eq70_e1075_d_b5, eq70_e1075_d_b6, eq70_e1075_d_b7, eq70_e1075_d_b8, eq70_e1075_d_b9, eq70_e1075_d_b10, eq70_e1075_d_b11, eq70_e1075_d_b12, eq70_e1075_d_b13, eq70_e1075_d_b14, eq70_e1075_d_b15, eq70_e1075_d_b16, eq70_e1075_d_b17, eq70_e1075_d_b18, eq70_e1075_d_b19, eq70_e1075_d_b20, eq70_e1075_d_b21, eq70_e1075_d_b22, eq70_e1075_d_b23, eq70_e1075_d_b24, eq70_e1075_d_b25, eq70_e1075_d_b26, eq70_e1075_d_b27, eq70_e1075_d_b28, eq70_e1075_d_b29, eq70_e1075_d_b30, eq70_e1075_d_b31, eq70_e1075_d_b32, eq70_e1075_d_b33, eq70_e1075_d_b34, eq70_e1075_d_b35,) = {
    if s.b[761] {
        let eq70_e1072: f64 = (s.v[0] * (nv14 - nv5));
        let eq70_e1072_d_n5: f64 = (-s.v[0]);
        let eq70_e1072_d_n14: f64 = s.v[0];
        let eq70_e1073: f64 = (s.v[190] + eq70_e1072);
        let eq70_e1073_d_n5: f64 = (s.dn[190][5] + eq70_e1072_d_n5);
        let eq70_e1073_d_n14: f64 = (s.dn[190][14] + eq70_e1072_d_n14);
        (eq70_e1073, s.dn[190][0], s.dn[190][1], s.dn[190][2], s.dn[190][3], s.dn[190][4], eq70_e1073_d_n5, s.dn[190][6], s.dn[190][7], s.dn[190][8], s.dn[190][9], s.dn[190][10], s.dn[190][11], s.dn[190][12], s.dn[190][13], eq70_e1073_d_n14, s.dn[190][15], s.dn[190][16], s.dn[190][17], s.dn[190][18], s.dn[190][19], s.dn[190][20], s.dn[190][21], s.dn[190][22], s.dn[190][23], s.dn[190][24], s.dn[190][25], s.dn[190][26], s.dn[190][27], s.dn[190][28], s.dn[190][29], s.db[190][0], s.db[190][1], s.db[190][2], s.db[190][3], s.db[190][4], s.db[190][5], s.db[190][6], s.db[190][7], s.db[190][8], s.db[190][9], s.db[190][10], s.db[190][11], s.db[190][12], s.db[190][13], s.db[190][14], s.db[190][15], s.db[190][16], s.db[190][17], s.db[190][18], s.db[190][19], s.db[190][20], s.db[190][21], s.db[190][22], s.db[190][23], s.db[190][24], s.db[190][25], s.db[190][26], s.db[190][27], s.db[190][28], s.db[190][29], s.db[190][30], s.db[190][31], s.db[190][32], s.db[190][33], s.db[190][34], s.db[190][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        let eq70_node_derivatives: [f64; 30] = [eq70_e1075_d_n0, eq70_e1075_d_n1, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n6, eq70_e1075_d_n7, eq70_e1075_d_n8, eq70_e1075_d_n9, eq70_e1075_d_n10, eq70_e1075_d_n11, eq70_e1075_d_n12, eq70_e1075_d_n13, eq70_e1075_d_n14, eq70_e1075_d_n15, eq70_e1075_d_n16, eq70_e1075_d_n17, eq70_e1075_d_n18, eq70_e1075_d_n19, eq70_e1075_d_n20, eq70_e1075_d_n21, eq70_e1075_d_n22, eq70_e1075_d_n23, eq70_e1075_d_n24, eq70_e1075_d_n25, eq70_e1075_d_n26, eq70_e1075_d_n27, eq70_e1075_d_n28, eq70_e1075_d_n29];
        let eq70_branch_derivatives: [f64; 36] = [eq70_e1075_d_b0, eq70_e1075_d_b1, eq70_e1075_d_b2, eq70_e1075_d_b3, eq70_e1075_d_b4, eq70_e1075_d_b5, eq70_e1075_d_b6, eq70_e1075_d_b7, eq70_e1075_d_b8, eq70_e1075_d_b9, eq70_e1075_d_b10, eq70_e1075_d_b11, eq70_e1075_d_b12, eq70_e1075_d_b13, eq70_e1075_d_b14, eq70_e1075_d_b15, eq70_e1075_d_b16, eq70_e1075_d_b17, eq70_e1075_d_b18, eq70_e1075_d_b19, eq70_e1075_d_b20, eq70_e1075_d_b21, eq70_e1075_d_b22, eq70_e1075_d_b23, eq70_e1075_d_b24, eq70_e1075_d_b25, eq70_e1075_d_b26, eq70_e1075_d_b27, eq70_e1075_d_b28, eq70_e1075_d_b29, eq70_e1075_d_b30, eq70_e1075_d_b31, eq70_e1075_d_b32, eq70_e1075_d_b33, eq70_e1075_d_b34, eq70_e1075_d_b35];
        stamper.stamp_current_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq70_value),
            &eq70_node_derivatives,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e1080,) = {
    if (!s.b[761]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e1080;
        stamper.stamp_potential_const_local(
            21,
            eq71_value,
        );
        let (eq72_e1090, eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29, eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35,) = {
    if s.b[907] {
        let eq72_e1083: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 52, s.v[191]);
        let eq72_e1083_d_n0: f64 = (s.dn[191][0] * ddt_scale);
        let eq72_e1083_d_n1: f64 = (s.dn[191][1] * ddt_scale);
        let eq72_e1083_d_n2: f64 = (s.dn[191][2] * ddt_scale);
        let eq72_e1083_d_n3: f64 = (s.dn[191][3] * ddt_scale);
        let eq72_e1083_d_n4: f64 = (s.dn[191][4] * ddt_scale);
        let eq72_e1083_d_n5: f64 = (s.dn[191][5] * ddt_scale);
        let eq72_e1083_d_n6: f64 = (s.dn[191][6] * ddt_scale);
        let eq72_e1083_d_n7: f64 = (s.dn[191][7] * ddt_scale);
        let eq72_e1083_d_n8: f64 = (s.dn[191][8] * ddt_scale);
        let eq72_e1083_d_n9: f64 = (s.dn[191][9] * ddt_scale);
        let eq72_e1083_d_n10: f64 = (s.dn[191][10] * ddt_scale);
        let eq72_e1083_d_n11: f64 = (s.dn[191][11] * ddt_scale);
        let eq72_e1083_d_n12: f64 = (s.dn[191][12] * ddt_scale);
        let eq72_e1083_d_n13: f64 = (s.dn[191][13] * ddt_scale);
        let eq72_e1083_d_n14: f64 = (s.dn[191][14] * ddt_scale);
        let eq72_e1083_d_n15: f64 = (s.dn[191][15] * ddt_scale);
        let eq72_e1083_d_n16: f64 = (s.dn[191][16] * ddt_scale);
        let eq72_e1083_d_n17: f64 = (s.dn[191][17] * ddt_scale);
        let eq72_e1083_d_n18: f64 = (s.dn[191][18] * ddt_scale);
        let eq72_e1083_d_n19: f64 = (s.dn[191][19] * ddt_scale);
        let eq72_e1083_d_n20: f64 = (s.dn[191][20] * ddt_scale);
        let eq72_e1083_d_n21: f64 = (s.dn[191][21] * ddt_scale);
        let eq72_e1083_d_n22: f64 = (s.dn[191][22] * ddt_scale);
        let eq72_e1083_d_n23: f64 = (s.dn[191][23] * ddt_scale);
        let eq72_e1083_d_n24: f64 = (s.dn[191][24] * ddt_scale);
        let eq72_e1083_d_n25: f64 = (s.dn[191][25] * ddt_scale);
        let eq72_e1083_d_n26: f64 = (s.dn[191][26] * ddt_scale);
        let eq72_e1083_d_n27: f64 = (s.dn[191][27] * ddt_scale);
        let eq72_e1083_d_n28: f64 = (s.dn[191][28] * ddt_scale);
        let eq72_e1083_d_n29: f64 = (s.dn[191][29] * ddt_scale);
        let eq72_e1083_d_b0: f64 = (s.db[191][0] * ddt_scale);
        let eq72_e1083_d_b1: f64 = (s.db[191][1] * ddt_scale);
        let eq72_e1083_d_b2: f64 = (s.db[191][2] * ddt_scale);
        let eq72_e1083_d_b3: f64 = (s.db[191][3] * ddt_scale);
        let eq72_e1083_d_b4: f64 = (s.db[191][4] * ddt_scale);
        let eq72_e1083_d_b5: f64 = (s.db[191][5] * ddt_scale);
        let eq72_e1083_d_b6: f64 = (s.db[191][6] * ddt_scale);
        let eq72_e1083_d_b7: f64 = (s.db[191][7] * ddt_scale);
        let eq72_e1083_d_b8: f64 = (s.db[191][8] * ddt_scale);
        let eq72_e1083_d_b9: f64 = (s.db[191][9] * ddt_scale);
        let eq72_e1083_d_b10: f64 = (s.db[191][10] * ddt_scale);
        let eq72_e1083_d_b11: f64 = (s.db[191][11] * ddt_scale);
        let eq72_e1083_d_b12: f64 = (s.db[191][12] * ddt_scale);
        let eq72_e1083_d_b13: f64 = (s.db[191][13] * ddt_scale);
        let eq72_e1083_d_b14: f64 = (s.db[191][14] * ddt_scale);
        let eq72_e1083_d_b15: f64 = (s.db[191][15] * ddt_scale);
        let eq72_e1083_d_b16: f64 = (s.db[191][16] * ddt_scale);
        let eq72_e1083_d_b17: f64 = (s.db[191][17] * ddt_scale);
        let eq72_e1083_d_b18: f64 = (s.db[191][18] * ddt_scale);
        let eq72_e1083_d_b19: f64 = (s.db[191][19] * ddt_scale);
        let eq72_e1083_d_b20: f64 = (s.db[191][20] * ddt_scale);
        let eq72_e1083_d_b21: f64 = (s.db[191][21] * ddt_scale);
        let eq72_e1083_d_b22: f64 = (s.db[191][22] * ddt_scale);
        let eq72_e1083_d_b23: f64 = (s.db[191][23] * ddt_scale);
        let eq72_e1083_d_b24: f64 = (s.db[191][24] * ddt_scale);
        let eq72_e1083_d_b25: f64 = (s.db[191][25] * ddt_scale);
        let eq72_e1083_d_b26: f64 = (s.db[191][26] * ddt_scale);
        let eq72_e1083_d_b27: f64 = (s.db[191][27] * ddt_scale);
        let eq72_e1083_d_b28: f64 = (s.db[191][28] * ddt_scale);
        let eq72_e1083_d_b29: f64 = (s.db[191][29] * ddt_scale);
        let eq72_e1083_d_b30: f64 = (s.db[191][30] * ddt_scale);
        let eq72_e1083_d_b31: f64 = (s.db[191][31] * ddt_scale);
        let eq72_e1083_d_b32: f64 = (s.db[191][32] * ddt_scale);
        let eq72_e1083_d_b33: f64 = (s.db[191][33] * ddt_scale);
        let eq72_e1083_d_b34: f64 = (s.db[191][34] * ddt_scale);
        let eq72_e1083_d_b35: f64 = (s.db[191][35] * ddt_scale);
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1086_d_n5: f64 = (-p.p355);
        let eq72_e1086_d_n7: f64 = p.p355;
        let eq72_e1087: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 53, eq72_e1086);
        let eq72_e1087_d_n5: f64 = (eq72_e1086_d_n5 * ddt_scale);
        let eq72_e1087_d_n7: f64 = (eq72_e1086_d_n7 * ddt_scale);
        let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);
        let eq72_e1088_d_n5: f64 = (eq72_e1083_d_n5 + eq72_e1087_d_n5);
        let eq72_e1088_d_n7: f64 = (eq72_e1083_d_n7 + eq72_e1087_d_n7);
        (eq72_e1088, eq72_e1083_d_n0, eq72_e1083_d_n1, eq72_e1083_d_n2, eq72_e1083_d_n3, eq72_e1083_d_n4, eq72_e1088_d_n5, eq72_e1083_d_n6, eq72_e1088_d_n7, eq72_e1083_d_n8, eq72_e1083_d_n9, eq72_e1083_d_n10, eq72_e1083_d_n11, eq72_e1083_d_n12, eq72_e1083_d_n13, eq72_e1083_d_n14, eq72_e1083_d_n15, eq72_e1083_d_n16, eq72_e1083_d_n17, eq72_e1083_d_n18, eq72_e1083_d_n19, eq72_e1083_d_n20, eq72_e1083_d_n21, eq72_e1083_d_n22, eq72_e1083_d_n23, eq72_e1083_d_n24, eq72_e1083_d_n25, eq72_e1083_d_n26, eq72_e1083_d_n27, eq72_e1083_d_n28, eq72_e1083_d_n29, eq72_e1083_d_b0, eq72_e1083_d_b1, eq72_e1083_d_b2, eq72_e1083_d_b3, eq72_e1083_d_b4, eq72_e1083_d_b5, eq72_e1083_d_b6, eq72_e1083_d_b7, eq72_e1083_d_b8, eq72_e1083_d_b9, eq72_e1083_d_b10, eq72_e1083_d_b11, eq72_e1083_d_b12, eq72_e1083_d_b13, eq72_e1083_d_b14, eq72_e1083_d_b15, eq72_e1083_d_b16, eq72_e1083_d_b17, eq72_e1083_d_b18, eq72_e1083_d_b19, eq72_e1083_d_b20, eq72_e1083_d_b21, eq72_e1083_d_b22, eq72_e1083_d_b23, eq72_e1083_d_b24, eq72_e1083_d_b25, eq72_e1083_d_b26, eq72_e1083_d_b27, eq72_e1083_d_b28, eq72_e1083_d_b29, eq72_e1083_d_b30, eq72_e1083_d_b31, eq72_e1083_d_b32, eq72_e1083_d_b33, eq72_e1083_d_b34, eq72_e1083_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        let eq72_node_derivatives: [f64; 30] = [eq72_e1090_d_n0, eq72_e1090_d_n1, eq72_e1090_d_n2, eq72_e1090_d_n3, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n6, eq72_e1090_d_n7, eq72_e1090_d_n8, eq72_e1090_d_n9, eq72_e1090_d_n10, eq72_e1090_d_n11, eq72_e1090_d_n12, eq72_e1090_d_n13, eq72_e1090_d_n14, eq72_e1090_d_n15, eq72_e1090_d_n16, eq72_e1090_d_n17, eq72_e1090_d_n18, eq72_e1090_d_n19, eq72_e1090_d_n20, eq72_e1090_d_n21, eq72_e1090_d_n22, eq72_e1090_d_n23, eq72_e1090_d_n24, eq72_e1090_d_n25, eq72_e1090_d_n26, eq72_e1090_d_n27, eq72_e1090_d_n28, eq72_e1090_d_n29];
        let eq72_branch_derivatives: [f64; 36] = [eq72_e1090_d_b0, eq72_e1090_d_b1, eq72_e1090_d_b2, eq72_e1090_d_b3, eq72_e1090_d_b4, eq72_e1090_d_b5, eq72_e1090_d_b6, eq72_e1090_d_b7, eq72_e1090_d_b8, eq72_e1090_d_b9, eq72_e1090_d_b10, eq72_e1090_d_b11, eq72_e1090_d_b12, eq72_e1090_d_b13, eq72_e1090_d_b14, eq72_e1090_d_b15, eq72_e1090_d_b16, eq72_e1090_d_b17, eq72_e1090_d_b18, eq72_e1090_d_b19, eq72_e1090_d_b20, eq72_e1090_d_b21, eq72_e1090_d_b22, eq72_e1090_d_b23, eq72_e1090_d_b24, eq72_e1090_d_b25, eq72_e1090_d_b26, eq72_e1090_d_b27, eq72_e1090_d_b28, eq72_e1090_d_b29, eq72_e1090_d_b30, eq72_e1090_d_b31, eq72_e1090_d_b32, eq72_e1090_d_b33, eq72_e1090_d_b34, eq72_e1090_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq72_value),
            &eq72_node_derivatives,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1100, eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29, eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35,) = {
    if s.b[907] {
        let eq73_e1093: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 54, s.v[192]);
        let eq73_e1093_d_n0: f64 = (s.dn[192][0] * ddt_scale);
        let eq73_e1093_d_n1: f64 = (s.dn[192][1] * ddt_scale);
        let eq73_e1093_d_n2: f64 = (s.dn[192][2] * ddt_scale);
        let eq73_e1093_d_n3: f64 = (s.dn[192][3] * ddt_scale);
        let eq73_e1093_d_n4: f64 = (s.dn[192][4] * ddt_scale);
        let eq73_e1093_d_n5: f64 = (s.dn[192][5] * ddt_scale);
        let eq73_e1093_d_n6: f64 = (s.dn[192][6] * ddt_scale);
        let eq73_e1093_d_n7: f64 = (s.dn[192][7] * ddt_scale);
        let eq73_e1093_d_n8: f64 = (s.dn[192][8] * ddt_scale);
        let eq73_e1093_d_n9: f64 = (s.dn[192][9] * ddt_scale);
        let eq73_e1093_d_n10: f64 = (s.dn[192][10] * ddt_scale);
        let eq73_e1093_d_n11: f64 = (s.dn[192][11] * ddt_scale);
        let eq73_e1093_d_n12: f64 = (s.dn[192][12] * ddt_scale);
        let eq73_e1093_d_n13: f64 = (s.dn[192][13] * ddt_scale);
        let eq73_e1093_d_n14: f64 = (s.dn[192][14] * ddt_scale);
        let eq73_e1093_d_n15: f64 = (s.dn[192][15] * ddt_scale);
        let eq73_e1093_d_n16: f64 = (s.dn[192][16] * ddt_scale);
        let eq73_e1093_d_n17: f64 = (s.dn[192][17] * ddt_scale);
        let eq73_e1093_d_n18: f64 = (s.dn[192][18] * ddt_scale);
        let eq73_e1093_d_n19: f64 = (s.dn[192][19] * ddt_scale);
        let eq73_e1093_d_n20: f64 = (s.dn[192][20] * ddt_scale);
        let eq73_e1093_d_n21: f64 = (s.dn[192][21] * ddt_scale);
        let eq73_e1093_d_n22: f64 = (s.dn[192][22] * ddt_scale);
        let eq73_e1093_d_n23: f64 = (s.dn[192][23] * ddt_scale);
        let eq73_e1093_d_n24: f64 = (s.dn[192][24] * ddt_scale);
        let eq73_e1093_d_n25: f64 = (s.dn[192][25] * ddt_scale);
        let eq73_e1093_d_n26: f64 = (s.dn[192][26] * ddt_scale);
        let eq73_e1093_d_n27: f64 = (s.dn[192][27] * ddt_scale);
        let eq73_e1093_d_n28: f64 = (s.dn[192][28] * ddt_scale);
        let eq73_e1093_d_n29: f64 = (s.dn[192][29] * ddt_scale);
        let eq73_e1093_d_b0: f64 = (s.db[192][0] * ddt_scale);
        let eq73_e1093_d_b1: f64 = (s.db[192][1] * ddt_scale);
        let eq73_e1093_d_b2: f64 = (s.db[192][2] * ddt_scale);
        let eq73_e1093_d_b3: f64 = (s.db[192][3] * ddt_scale);
        let eq73_e1093_d_b4: f64 = (s.db[192][4] * ddt_scale);
        let eq73_e1093_d_b5: f64 = (s.db[192][5] * ddt_scale);
        let eq73_e1093_d_b6: f64 = (s.db[192][6] * ddt_scale);
        let eq73_e1093_d_b7: f64 = (s.db[192][7] * ddt_scale);
        let eq73_e1093_d_b8: f64 = (s.db[192][8] * ddt_scale);
        let eq73_e1093_d_b9: f64 = (s.db[192][9] * ddt_scale);
        let eq73_e1093_d_b10: f64 = (s.db[192][10] * ddt_scale);
        let eq73_e1093_d_b11: f64 = (s.db[192][11] * ddt_scale);
        let eq73_e1093_d_b12: f64 = (s.db[192][12] * ddt_scale);
        let eq73_e1093_d_b13: f64 = (s.db[192][13] * ddt_scale);
        let eq73_e1093_d_b14: f64 = (s.db[192][14] * ddt_scale);
        let eq73_e1093_d_b15: f64 = (s.db[192][15] * ddt_scale);
        let eq73_e1093_d_b16: f64 = (s.db[192][16] * ddt_scale);
        let eq73_e1093_d_b17: f64 = (s.db[192][17] * ddt_scale);
        let eq73_e1093_d_b18: f64 = (s.db[192][18] * ddt_scale);
        let eq73_e1093_d_b19: f64 = (s.db[192][19] * ddt_scale);
        let eq73_e1093_d_b20: f64 = (s.db[192][20] * ddt_scale);
        let eq73_e1093_d_b21: f64 = (s.db[192][21] * ddt_scale);
        let eq73_e1093_d_b22: f64 = (s.db[192][22] * ddt_scale);
        let eq73_e1093_d_b23: f64 = (s.db[192][23] * ddt_scale);
        let eq73_e1093_d_b24: f64 = (s.db[192][24] * ddt_scale);
        let eq73_e1093_d_b25: f64 = (s.db[192][25] * ddt_scale);
        let eq73_e1093_d_b26: f64 = (s.db[192][26] * ddt_scale);
        let eq73_e1093_d_b27: f64 = (s.db[192][27] * ddt_scale);
        let eq73_e1093_d_b28: f64 = (s.db[192][28] * ddt_scale);
        let eq73_e1093_d_b29: f64 = (s.db[192][29] * ddt_scale);
        let eq73_e1093_d_b30: f64 = (s.db[192][30] * ddt_scale);
        let eq73_e1093_d_b31: f64 = (s.db[192][31] * ddt_scale);
        let eq73_e1093_d_b32: f64 = (s.db[192][32] * ddt_scale);
        let eq73_e1093_d_b33: f64 = (s.db[192][33] * ddt_scale);
        let eq73_e1093_d_b34: f64 = (s.db[192][34] * ddt_scale);
        let eq73_e1093_d_b35: f64 = (s.db[192][35] * ddt_scale);
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1096_d_n7: f64 = p.p355;
        let eq73_e1096_d_n14: f64 = (-p.p355);
        let eq73_e1097: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 55, eq73_e1096);
        let eq73_e1097_d_n7: f64 = (eq73_e1096_d_n7 * ddt_scale);
        let eq73_e1097_d_n14: f64 = (eq73_e1096_d_n14 * ddt_scale);
        let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);
        let eq73_e1098_d_n7: f64 = (eq73_e1093_d_n7 + eq73_e1097_d_n7);
        let eq73_e1098_d_n14: f64 = (eq73_e1093_d_n14 + eq73_e1097_d_n14);
        (eq73_e1098, eq73_e1093_d_n0, eq73_e1093_d_n1, eq73_e1093_d_n2, eq73_e1093_d_n3, eq73_e1093_d_n4, eq73_e1093_d_n5, eq73_e1093_d_n6, eq73_e1098_d_n7, eq73_e1093_d_n8, eq73_e1093_d_n9, eq73_e1093_d_n10, eq73_e1093_d_n11, eq73_e1093_d_n12, eq73_e1093_d_n13, eq73_e1098_d_n14, eq73_e1093_d_n15, eq73_e1093_d_n16, eq73_e1093_d_n17, eq73_e1093_d_n18, eq73_e1093_d_n19, eq73_e1093_d_n20, eq73_e1093_d_n21, eq73_e1093_d_n22, eq73_e1093_d_n23, eq73_e1093_d_n24, eq73_e1093_d_n25, eq73_e1093_d_n26, eq73_e1093_d_n27, eq73_e1093_d_n28, eq73_e1093_d_n29, eq73_e1093_d_b0, eq73_e1093_d_b1, eq73_e1093_d_b2, eq73_e1093_d_b3, eq73_e1093_d_b4, eq73_e1093_d_b5, eq73_e1093_d_b6, eq73_e1093_d_b7, eq73_e1093_d_b8, eq73_e1093_d_b9, eq73_e1093_d_b10, eq73_e1093_d_b11, eq73_e1093_d_b12, eq73_e1093_d_b13, eq73_e1093_d_b14, eq73_e1093_d_b15, eq73_e1093_d_b16, eq73_e1093_d_b17, eq73_e1093_d_b18, eq73_e1093_d_b19, eq73_e1093_d_b20, eq73_e1093_d_b21, eq73_e1093_d_b22, eq73_e1093_d_b23, eq73_e1093_d_b24, eq73_e1093_d_b25, eq73_e1093_d_b26, eq73_e1093_d_b27, eq73_e1093_d_b28, eq73_e1093_d_b29, eq73_e1093_d_b30, eq73_e1093_d_b31, eq73_e1093_d_b32, eq73_e1093_d_b33, eq73_e1093_d_b34, eq73_e1093_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        let eq73_node_derivatives: [f64; 30] = [eq73_e1100_d_n0, eq73_e1100_d_n1, eq73_e1100_d_n2, eq73_e1100_d_n3, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n6, eq73_e1100_d_n7, eq73_e1100_d_n8, eq73_e1100_d_n9, eq73_e1100_d_n10, eq73_e1100_d_n11, eq73_e1100_d_n12, eq73_e1100_d_n13, eq73_e1100_d_n14, eq73_e1100_d_n15, eq73_e1100_d_n16, eq73_e1100_d_n17, eq73_e1100_d_n18, eq73_e1100_d_n19, eq73_e1100_d_n20, eq73_e1100_d_n21, eq73_e1100_d_n22, eq73_e1100_d_n23, eq73_e1100_d_n24, eq73_e1100_d_n25, eq73_e1100_d_n26, eq73_e1100_d_n27, eq73_e1100_d_n28, eq73_e1100_d_n29];
        let eq73_branch_derivatives: [f64; 36] = [eq73_e1100_d_b0, eq73_e1100_d_b1, eq73_e1100_d_b2, eq73_e1100_d_b3, eq73_e1100_d_b4, eq73_e1100_d_b5, eq73_e1100_d_b6, eq73_e1100_d_b7, eq73_e1100_d_b8, eq73_e1100_d_b9, eq73_e1100_d_b10, eq73_e1100_d_b11, eq73_e1100_d_b12, eq73_e1100_d_b13, eq73_e1100_d_b14, eq73_e1100_d_b15, eq73_e1100_d_b16, eq73_e1100_d_b17, eq73_e1100_d_b18, eq73_e1100_d_b19, eq73_e1100_d_b20, eq73_e1100_d_b21, eq73_e1100_d_b22, eq73_e1100_d_b23, eq73_e1100_d_b24, eq73_e1100_d_b25, eq73_e1100_d_b26, eq73_e1100_d_b27, eq73_e1100_d_b28, eq73_e1100_d_b29, eq73_e1100_d_b30, eq73_e1100_d_b31, eq73_e1100_d_b32, eq73_e1100_d_b33, eq73_e1100_d_b34, eq73_e1100_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(14),
            multiplicity * (eq73_value),
            &eq73_node_derivatives,
            &eq73_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq74_e1110, eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29, eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35,) = {
    if s.b[907] {
        let eq74_e1103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 56, s.v[193]);
        let eq74_e1103_d_n0: f64 = (s.dn[193][0] * ddt_scale);
        let eq74_e1103_d_n1: f64 = (s.dn[193][1] * ddt_scale);
        let eq74_e1103_d_n2: f64 = (s.dn[193][2] * ddt_scale);
        let eq74_e1103_d_n3: f64 = (s.dn[193][3] * ddt_scale);
        let eq74_e1103_d_n4: f64 = (s.dn[193][4] * ddt_scale);
        let eq74_e1103_d_n5: f64 = (s.dn[193][5] * ddt_scale);
        let eq74_e1103_d_n6: f64 = (s.dn[193][6] * ddt_scale);
        let eq74_e1103_d_n7: f64 = (s.dn[193][7] * ddt_scale);
        let eq74_e1103_d_n8: f64 = (s.dn[193][8] * ddt_scale);
        let eq74_e1103_d_n9: f64 = (s.dn[193][9] * ddt_scale);
        let eq74_e1103_d_n10: f64 = (s.dn[193][10] * ddt_scale);
        let eq74_e1103_d_n11: f64 = (s.dn[193][11] * ddt_scale);
        let eq74_e1103_d_n12: f64 = (s.dn[193][12] * ddt_scale);
        let eq74_e1103_d_n13: f64 = (s.dn[193][13] * ddt_scale);
        let eq74_e1103_d_n14: f64 = (s.dn[193][14] * ddt_scale);
        let eq74_e1103_d_n15: f64 = (s.dn[193][15] * ddt_scale);
        let eq74_e1103_d_n16: f64 = (s.dn[193][16] * ddt_scale);
        let eq74_e1103_d_n17: f64 = (s.dn[193][17] * ddt_scale);
        let eq74_e1103_d_n18: f64 = (s.dn[193][18] * ddt_scale);
        let eq74_e1103_d_n19: f64 = (s.dn[193][19] * ddt_scale);
        let eq74_e1103_d_n20: f64 = (s.dn[193][20] * ddt_scale);
        let eq74_e1103_d_n21: f64 = (s.dn[193][21] * ddt_scale);
        let eq74_e1103_d_n22: f64 = (s.dn[193][22] * ddt_scale);
        let eq74_e1103_d_n23: f64 = (s.dn[193][23] * ddt_scale);
        let eq74_e1103_d_n24: f64 = (s.dn[193][24] * ddt_scale);
        let eq74_e1103_d_n25: f64 = (s.dn[193][25] * ddt_scale);
        let eq74_e1103_d_n26: f64 = (s.dn[193][26] * ddt_scale);
        let eq74_e1103_d_n27: f64 = (s.dn[193][27] * ddt_scale);
        let eq74_e1103_d_n28: f64 = (s.dn[193][28] * ddt_scale);
        let eq74_e1103_d_n29: f64 = (s.dn[193][29] * ddt_scale);
        let eq74_e1103_d_b0: f64 = (s.db[193][0] * ddt_scale);
        let eq74_e1103_d_b1: f64 = (s.db[193][1] * ddt_scale);
        let eq74_e1103_d_b2: f64 = (s.db[193][2] * ddt_scale);
        let eq74_e1103_d_b3: f64 = (s.db[193][3] * ddt_scale);
        let eq74_e1103_d_b4: f64 = (s.db[193][4] * ddt_scale);
        let eq74_e1103_d_b5: f64 = (s.db[193][5] * ddt_scale);
        let eq74_e1103_d_b6: f64 = (s.db[193][6] * ddt_scale);
        let eq74_e1103_d_b7: f64 = (s.db[193][7] * ddt_scale);
        let eq74_e1103_d_b8: f64 = (s.db[193][8] * ddt_scale);
        let eq74_e1103_d_b9: f64 = (s.db[193][9] * ddt_scale);
        let eq74_e1103_d_b10: f64 = (s.db[193][10] * ddt_scale);
        let eq74_e1103_d_b11: f64 = (s.db[193][11] * ddt_scale);
        let eq74_e1103_d_b12: f64 = (s.db[193][12] * ddt_scale);
        let eq74_e1103_d_b13: f64 = (s.db[193][13] * ddt_scale);
        let eq74_e1103_d_b14: f64 = (s.db[193][14] * ddt_scale);
        let eq74_e1103_d_b15: f64 = (s.db[193][15] * ddt_scale);
        let eq74_e1103_d_b16: f64 = (s.db[193][16] * ddt_scale);
        let eq74_e1103_d_b17: f64 = (s.db[193][17] * ddt_scale);
        let eq74_e1103_d_b18: f64 = (s.db[193][18] * ddt_scale);
        let eq74_e1103_d_b19: f64 = (s.db[193][19] * ddt_scale);
        let eq74_e1103_d_b20: f64 = (s.db[193][20] * ddt_scale);
        let eq74_e1103_d_b21: f64 = (s.db[193][21] * ddt_scale);
        let eq74_e1103_d_b22: f64 = (s.db[193][22] * ddt_scale);
        let eq74_e1103_d_b23: f64 = (s.db[193][23] * ddt_scale);
        let eq74_e1103_d_b24: f64 = (s.db[193][24] * ddt_scale);
        let eq74_e1103_d_b25: f64 = (s.db[193][25] * ddt_scale);
        let eq74_e1103_d_b26: f64 = (s.db[193][26] * ddt_scale);
        let eq74_e1103_d_b27: f64 = (s.db[193][27] * ddt_scale);
        let eq74_e1103_d_b28: f64 = (s.db[193][28] * ddt_scale);
        let eq74_e1103_d_b29: f64 = (s.db[193][29] * ddt_scale);
        let eq74_e1103_d_b30: f64 = (s.db[193][30] * ddt_scale);
        let eq74_e1103_d_b31: f64 = (s.db[193][31] * ddt_scale);
        let eq74_e1103_d_b32: f64 = (s.db[193][32] * ddt_scale);
        let eq74_e1103_d_b33: f64 = (s.db[193][33] * ddt_scale);
        let eq74_e1103_d_b34: f64 = (s.db[193][34] * ddt_scale);
        let eq74_e1103_d_b35: f64 = (s.db[193][35] * ddt_scale);
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1106_d_n2: f64 = p.p355;
        let eq74_e1106_d_n5: f64 = (-p.p355);
        let eq74_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 57, eq74_e1106);
        let eq74_e1107_d_n2: f64 = (eq74_e1106_d_n2 * ddt_scale);
        let eq74_e1107_d_n5: f64 = (eq74_e1106_d_n5 * ddt_scale);
        let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);
        let eq74_e1108_d_n2: f64 = (eq74_e1103_d_n2 + eq74_e1107_d_n2);
        let eq74_e1108_d_n5: f64 = (eq74_e1103_d_n5 + eq74_e1107_d_n5);
        (eq74_e1108, eq74_e1103_d_n0, eq74_e1103_d_n1, eq74_e1108_d_n2, eq74_e1103_d_n3, eq74_e1103_d_n4, eq74_e1108_d_n5, eq74_e1103_d_n6, eq74_e1103_d_n7, eq74_e1103_d_n8, eq74_e1103_d_n9, eq74_e1103_d_n10, eq74_e1103_d_n11, eq74_e1103_d_n12, eq74_e1103_d_n13, eq74_e1103_d_n14, eq74_e1103_d_n15, eq74_e1103_d_n16, eq74_e1103_d_n17, eq74_e1103_d_n18, eq74_e1103_d_n19, eq74_e1103_d_n20, eq74_e1103_d_n21, eq74_e1103_d_n22, eq74_e1103_d_n23, eq74_e1103_d_n24, eq74_e1103_d_n25, eq74_e1103_d_n26, eq74_e1103_d_n27, eq74_e1103_d_n28, eq74_e1103_d_n29, eq74_e1103_d_b0, eq74_e1103_d_b1, eq74_e1103_d_b2, eq74_e1103_d_b3, eq74_e1103_d_b4, eq74_e1103_d_b5, eq74_e1103_d_b6, eq74_e1103_d_b7, eq74_e1103_d_b8, eq74_e1103_d_b9, eq74_e1103_d_b10, eq74_e1103_d_b11, eq74_e1103_d_b12, eq74_e1103_d_b13, eq74_e1103_d_b14, eq74_e1103_d_b15, eq74_e1103_d_b16, eq74_e1103_d_b17, eq74_e1103_d_b18, eq74_e1103_d_b19, eq74_e1103_d_b20, eq74_e1103_d_b21, eq74_e1103_d_b22, eq74_e1103_d_b23, eq74_e1103_d_b24, eq74_e1103_d_b25, eq74_e1103_d_b26, eq74_e1103_d_b27, eq74_e1103_d_b28, eq74_e1103_d_b29, eq74_e1103_d_b30, eq74_e1103_d_b31, eq74_e1103_d_b32, eq74_e1103_d_b33, eq74_e1103_d_b34, eq74_e1103_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        let eq74_node_derivatives: [f64; 30] = [eq74_e1110_d_n0, eq74_e1110_d_n1, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n6, eq74_e1110_d_n7, eq74_e1110_d_n8, eq74_e1110_d_n9, eq74_e1110_d_n10, eq74_e1110_d_n11, eq74_e1110_d_n12, eq74_e1110_d_n13, eq74_e1110_d_n14, eq74_e1110_d_n15, eq74_e1110_d_n16, eq74_e1110_d_n17, eq74_e1110_d_n18, eq74_e1110_d_n19, eq74_e1110_d_n20, eq74_e1110_d_n21, eq74_e1110_d_n22, eq74_e1110_d_n23, eq74_e1110_d_n24, eq74_e1110_d_n25, eq74_e1110_d_n26, eq74_e1110_d_n27, eq74_e1110_d_n28, eq74_e1110_d_n29];
        let eq74_branch_derivatives: [f64; 36] = [eq74_e1110_d_b0, eq74_e1110_d_b1, eq74_e1110_d_b2, eq74_e1110_d_b3, eq74_e1110_d_b4, eq74_e1110_d_b5, eq74_e1110_d_b6, eq74_e1110_d_b7, eq74_e1110_d_b8, eq74_e1110_d_b9, eq74_e1110_d_b10, eq74_e1110_d_b11, eq74_e1110_d_b12, eq74_e1110_d_b13, eq74_e1110_d_b14, eq74_e1110_d_b15, eq74_e1110_d_b16, eq74_e1110_d_b17, eq74_e1110_d_b18, eq74_e1110_d_b19, eq74_e1110_d_b20, eq74_e1110_d_b21, eq74_e1110_d_b22, eq74_e1110_d_b23, eq74_e1110_d_b24, eq74_e1110_d_b25, eq74_e1110_d_b26, eq74_e1110_d_b27, eq74_e1110_d_b28, eq74_e1110_d_b29, eq74_e1110_d_b30, eq74_e1110_d_b31, eq74_e1110_d_b32, eq74_e1110_d_b33, eq74_e1110_d_b34, eq74_e1110_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq74_value),
            &eq74_node_derivatives,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1114,) = {
    if s.b[907] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq75_value: f64 = eq75_e1114;
        stamper.stamp_current_const_local(
            Some(2),
            Some(14),
            multiplicity * (eq75_value),
        );
        let (eq76_e1124, eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29, eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35,) = {
    if s.b[907] {
        let eq76_e1117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 58, s.v[195]);
        let eq76_e1117_d_n0: f64 = (s.dn[195][0] * ddt_scale);
        let eq76_e1117_d_n1: f64 = (s.dn[195][1] * ddt_scale);
        let eq76_e1117_d_n2: f64 = (s.dn[195][2] * ddt_scale);
        let eq76_e1117_d_n3: f64 = (s.dn[195][3] * ddt_scale);
        let eq76_e1117_d_n4: f64 = (s.dn[195][4] * ddt_scale);
        let eq76_e1117_d_n5: f64 = (s.dn[195][5] * ddt_scale);
        let eq76_e1117_d_n6: f64 = (s.dn[195][6] * ddt_scale);
        let eq76_e1117_d_n7: f64 = (s.dn[195][7] * ddt_scale);
        let eq76_e1117_d_n8: f64 = (s.dn[195][8] * ddt_scale);
        let eq76_e1117_d_n9: f64 = (s.dn[195][9] * ddt_scale);
        let eq76_e1117_d_n10: f64 = (s.dn[195][10] * ddt_scale);
        let eq76_e1117_d_n11: f64 = (s.dn[195][11] * ddt_scale);
        let eq76_e1117_d_n12: f64 = (s.dn[195][12] * ddt_scale);
        let eq76_e1117_d_n13: f64 = (s.dn[195][13] * ddt_scale);
        let eq76_e1117_d_n14: f64 = (s.dn[195][14] * ddt_scale);
        let eq76_e1117_d_n15: f64 = (s.dn[195][15] * ddt_scale);
        let eq76_e1117_d_n16: f64 = (s.dn[195][16] * ddt_scale);
        let eq76_e1117_d_n17: f64 = (s.dn[195][17] * ddt_scale);
        let eq76_e1117_d_n18: f64 = (s.dn[195][18] * ddt_scale);
        let eq76_e1117_d_n19: f64 = (s.dn[195][19] * ddt_scale);
        let eq76_e1117_d_n20: f64 = (s.dn[195][20] * ddt_scale);
        let eq76_e1117_d_n21: f64 = (s.dn[195][21] * ddt_scale);
        let eq76_e1117_d_n22: f64 = (s.dn[195][22] * ddt_scale);
        let eq76_e1117_d_n23: f64 = (s.dn[195][23] * ddt_scale);
        let eq76_e1117_d_n24: f64 = (s.dn[195][24] * ddt_scale);
        let eq76_e1117_d_n25: f64 = (s.dn[195][25] * ddt_scale);
        let eq76_e1117_d_n26: f64 = (s.dn[195][26] * ddt_scale);
        let eq76_e1117_d_n27: f64 = (s.dn[195][27] * ddt_scale);
        let eq76_e1117_d_n28: f64 = (s.dn[195][28] * ddt_scale);
        let eq76_e1117_d_n29: f64 = (s.dn[195][29] * ddt_scale);
        let eq76_e1117_d_b0: f64 = (s.db[195][0] * ddt_scale);
        let eq76_e1117_d_b1: f64 = (s.db[195][1] * ddt_scale);
        let eq76_e1117_d_b2: f64 = (s.db[195][2] * ddt_scale);
        let eq76_e1117_d_b3: f64 = (s.db[195][3] * ddt_scale);
        let eq76_e1117_d_b4: f64 = (s.db[195][4] * ddt_scale);
        let eq76_e1117_d_b5: f64 = (s.db[195][5] * ddt_scale);
        let eq76_e1117_d_b6: f64 = (s.db[195][6] * ddt_scale);
        let eq76_e1117_d_b7: f64 = (s.db[195][7] * ddt_scale);
        let eq76_e1117_d_b8: f64 = (s.db[195][8] * ddt_scale);
        let eq76_e1117_d_b9: f64 = (s.db[195][9] * ddt_scale);
        let eq76_e1117_d_b10: f64 = (s.db[195][10] * ddt_scale);
        let eq76_e1117_d_b11: f64 = (s.db[195][11] * ddt_scale);
        let eq76_e1117_d_b12: f64 = (s.db[195][12] * ddt_scale);
        let eq76_e1117_d_b13: f64 = (s.db[195][13] * ddt_scale);
        let eq76_e1117_d_b14: f64 = (s.db[195][14] * ddt_scale);
        let eq76_e1117_d_b15: f64 = (s.db[195][15] * ddt_scale);
        let eq76_e1117_d_b16: f64 = (s.db[195][16] * ddt_scale);
        let eq76_e1117_d_b17: f64 = (s.db[195][17] * ddt_scale);
        let eq76_e1117_d_b18: f64 = (s.db[195][18] * ddt_scale);
        let eq76_e1117_d_b19: f64 = (s.db[195][19] * ddt_scale);
        let eq76_e1117_d_b20: f64 = (s.db[195][20] * ddt_scale);
        let eq76_e1117_d_b21: f64 = (s.db[195][21] * ddt_scale);
        let eq76_e1117_d_b22: f64 = (s.db[195][22] * ddt_scale);
        let eq76_e1117_d_b23: f64 = (s.db[195][23] * ddt_scale);
        let eq76_e1117_d_b24: f64 = (s.db[195][24] * ddt_scale);
        let eq76_e1117_d_b25: f64 = (s.db[195][25] * ddt_scale);
        let eq76_e1117_d_b26: f64 = (s.db[195][26] * ddt_scale);
        let eq76_e1117_d_b27: f64 = (s.db[195][27] * ddt_scale);
        let eq76_e1117_d_b28: f64 = (s.db[195][28] * ddt_scale);
        let eq76_e1117_d_b29: f64 = (s.db[195][29] * ddt_scale);
        let eq76_e1117_d_b30: f64 = (s.db[195][30] * ddt_scale);
        let eq76_e1117_d_b31: f64 = (s.db[195][31] * ddt_scale);
        let eq76_e1117_d_b32: f64 = (s.db[195][32] * ddt_scale);
        let eq76_e1117_d_b33: f64 = (s.db[195][33] * ddt_scale);
        let eq76_e1117_d_b34: f64 = (s.db[195][34] * ddt_scale);
        let eq76_e1117_d_b35: f64 = (s.db[195][35] * ddt_scale);
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1120_d_n7: f64 = p.p355;
        let eq76_e1120_d_n9: f64 = (-p.p355);
        let eq76_e1121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 59, eq76_e1120);
        let eq76_e1121_d_n7: f64 = (eq76_e1120_d_n7 * ddt_scale);
        let eq76_e1121_d_n9: f64 = (eq76_e1120_d_n9 * ddt_scale);
        let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);
        let eq76_e1122_d_n7: f64 = (eq76_e1117_d_n7 + eq76_e1121_d_n7);
        let eq76_e1122_d_n9: f64 = (eq76_e1117_d_n9 + eq76_e1121_d_n9);
        (eq76_e1122, eq76_e1117_d_n0, eq76_e1117_d_n1, eq76_e1117_d_n2, eq76_e1117_d_n3, eq76_e1117_d_n4, eq76_e1117_d_n5, eq76_e1117_d_n6, eq76_e1122_d_n7, eq76_e1117_d_n8, eq76_e1122_d_n9, eq76_e1117_d_n10, eq76_e1117_d_n11, eq76_e1117_d_n12, eq76_e1117_d_n13, eq76_e1117_d_n14, eq76_e1117_d_n15, eq76_e1117_d_n16, eq76_e1117_d_n17, eq76_e1117_d_n18, eq76_e1117_d_n19, eq76_e1117_d_n20, eq76_e1117_d_n21, eq76_e1117_d_n22, eq76_e1117_d_n23, eq76_e1117_d_n24, eq76_e1117_d_n25, eq76_e1117_d_n26, eq76_e1117_d_n27, eq76_e1117_d_n28, eq76_e1117_d_n29, eq76_e1117_d_b0, eq76_e1117_d_b1, eq76_e1117_d_b2, eq76_e1117_d_b3, eq76_e1117_d_b4, eq76_e1117_d_b5, eq76_e1117_d_b6, eq76_e1117_d_b7, eq76_e1117_d_b8, eq76_e1117_d_b9, eq76_e1117_d_b10, eq76_e1117_d_b11, eq76_e1117_d_b12, eq76_e1117_d_b13, eq76_e1117_d_b14, eq76_e1117_d_b15, eq76_e1117_d_b16, eq76_e1117_d_b17, eq76_e1117_d_b18, eq76_e1117_d_b19, eq76_e1117_d_b20, eq76_e1117_d_b21, eq76_e1117_d_b22, eq76_e1117_d_b23, eq76_e1117_d_b24, eq76_e1117_d_b25, eq76_e1117_d_b26, eq76_e1117_d_b27, eq76_e1117_d_b28, eq76_e1117_d_b29, eq76_e1117_d_b30, eq76_e1117_d_b31, eq76_e1117_d_b32, eq76_e1117_d_b33, eq76_e1117_d_b34, eq76_e1117_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        let eq76_node_derivatives: [f64; 30] = [eq76_e1124_d_n0, eq76_e1124_d_n1, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n6, eq76_e1124_d_n7, eq76_e1124_d_n8, eq76_e1124_d_n9, eq76_e1124_d_n10, eq76_e1124_d_n11, eq76_e1124_d_n12, eq76_e1124_d_n13, eq76_e1124_d_n14, eq76_e1124_d_n15, eq76_e1124_d_n16, eq76_e1124_d_n17, eq76_e1124_d_n18, eq76_e1124_d_n19, eq76_e1124_d_n20, eq76_e1124_d_n21, eq76_e1124_d_n22, eq76_e1124_d_n23, eq76_e1124_d_n24, eq76_e1124_d_n25, eq76_e1124_d_n26, eq76_e1124_d_n27, eq76_e1124_d_n28, eq76_e1124_d_n29];
        let eq76_branch_derivatives: [f64; 36] = [eq76_e1124_d_b0, eq76_e1124_d_b1, eq76_e1124_d_b2, eq76_e1124_d_b3, eq76_e1124_d_b4, eq76_e1124_d_b5, eq76_e1124_d_b6, eq76_e1124_d_b7, eq76_e1124_d_b8, eq76_e1124_d_b9, eq76_e1124_d_b10, eq76_e1124_d_b11, eq76_e1124_d_b12, eq76_e1124_d_b13, eq76_e1124_d_b14, eq76_e1124_d_b15, eq76_e1124_d_b16, eq76_e1124_d_b17, eq76_e1124_d_b18, eq76_e1124_d_b19, eq76_e1124_d_b20, eq76_e1124_d_b21, eq76_e1124_d_b22, eq76_e1124_d_b23, eq76_e1124_d_b24, eq76_e1124_d_b25, eq76_e1124_d_b26, eq76_e1124_d_b27, eq76_e1124_d_b28, eq76_e1124_d_b29, eq76_e1124_d_b30, eq76_e1124_d_b31, eq76_e1124_d_b32, eq76_e1124_d_b33, eq76_e1124_d_b34, eq76_e1124_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1135, eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29, eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35,) = {
    if (!s.b[907]) {
        let eq77_e1128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 60, s.v[191]);
        let eq77_e1128_d_n0: f64 = (s.dn[191][0] * ddt_scale);
        let eq77_e1128_d_n1: f64 = (s.dn[191][1] * ddt_scale);
        let eq77_e1128_d_n2: f64 = (s.dn[191][2] * ddt_scale);
        let eq77_e1128_d_n3: f64 = (s.dn[191][3] * ddt_scale);
        let eq77_e1128_d_n4: f64 = (s.dn[191][4] * ddt_scale);
        let eq77_e1128_d_n5: f64 = (s.dn[191][5] * ddt_scale);
        let eq77_e1128_d_n6: f64 = (s.dn[191][6] * ddt_scale);
        let eq77_e1128_d_n7: f64 = (s.dn[191][7] * ddt_scale);
        let eq77_e1128_d_n8: f64 = (s.dn[191][8] * ddt_scale);
        let eq77_e1128_d_n9: f64 = (s.dn[191][9] * ddt_scale);
        let eq77_e1128_d_n10: f64 = (s.dn[191][10] * ddt_scale);
        let eq77_e1128_d_n11: f64 = (s.dn[191][11] * ddt_scale);
        let eq77_e1128_d_n12: f64 = (s.dn[191][12] * ddt_scale);
        let eq77_e1128_d_n13: f64 = (s.dn[191][13] * ddt_scale);
        let eq77_e1128_d_n14: f64 = (s.dn[191][14] * ddt_scale);
        let eq77_e1128_d_n15: f64 = (s.dn[191][15] * ddt_scale);
        let eq77_e1128_d_n16: f64 = (s.dn[191][16] * ddt_scale);
        let eq77_e1128_d_n17: f64 = (s.dn[191][17] * ddt_scale);
        let eq77_e1128_d_n18: f64 = (s.dn[191][18] * ddt_scale);
        let eq77_e1128_d_n19: f64 = (s.dn[191][19] * ddt_scale);
        let eq77_e1128_d_n20: f64 = (s.dn[191][20] * ddt_scale);
        let eq77_e1128_d_n21: f64 = (s.dn[191][21] * ddt_scale);
        let eq77_e1128_d_n22: f64 = (s.dn[191][22] * ddt_scale);
        let eq77_e1128_d_n23: f64 = (s.dn[191][23] * ddt_scale);
        let eq77_e1128_d_n24: f64 = (s.dn[191][24] * ddt_scale);
        let eq77_e1128_d_n25: f64 = (s.dn[191][25] * ddt_scale);
        let eq77_e1128_d_n26: f64 = (s.dn[191][26] * ddt_scale);
        let eq77_e1128_d_n27: f64 = (s.dn[191][27] * ddt_scale);
        let eq77_e1128_d_n28: f64 = (s.dn[191][28] * ddt_scale);
        let eq77_e1128_d_n29: f64 = (s.dn[191][29] * ddt_scale);
        let eq77_e1128_d_b0: f64 = (s.db[191][0] * ddt_scale);
        let eq77_e1128_d_b1: f64 = (s.db[191][1] * ddt_scale);
        let eq77_e1128_d_b2: f64 = (s.db[191][2] * ddt_scale);
        let eq77_e1128_d_b3: f64 = (s.db[191][3] * ddt_scale);
        let eq77_e1128_d_b4: f64 = (s.db[191][4] * ddt_scale);
        let eq77_e1128_d_b5: f64 = (s.db[191][5] * ddt_scale);
        let eq77_e1128_d_b6: f64 = (s.db[191][6] * ddt_scale);
        let eq77_e1128_d_b7: f64 = (s.db[191][7] * ddt_scale);
        let eq77_e1128_d_b8: f64 = (s.db[191][8] * ddt_scale);
        let eq77_e1128_d_b9: f64 = (s.db[191][9] * ddt_scale);
        let eq77_e1128_d_b10: f64 = (s.db[191][10] * ddt_scale);
        let eq77_e1128_d_b11: f64 = (s.db[191][11] * ddt_scale);
        let eq77_e1128_d_b12: f64 = (s.db[191][12] * ddt_scale);
        let eq77_e1128_d_b13: f64 = (s.db[191][13] * ddt_scale);
        let eq77_e1128_d_b14: f64 = (s.db[191][14] * ddt_scale);
        let eq77_e1128_d_b15: f64 = (s.db[191][15] * ddt_scale);
        let eq77_e1128_d_b16: f64 = (s.db[191][16] * ddt_scale);
        let eq77_e1128_d_b17: f64 = (s.db[191][17] * ddt_scale);
        let eq77_e1128_d_b18: f64 = (s.db[191][18] * ddt_scale);
        let eq77_e1128_d_b19: f64 = (s.db[191][19] * ddt_scale);
        let eq77_e1128_d_b20: f64 = (s.db[191][20] * ddt_scale);
        let eq77_e1128_d_b21: f64 = (s.db[191][21] * ddt_scale);
        let eq77_e1128_d_b22: f64 = (s.db[191][22] * ddt_scale);
        let eq77_e1128_d_b23: f64 = (s.db[191][23] * ddt_scale);
        let eq77_e1128_d_b24: f64 = (s.db[191][24] * ddt_scale);
        let eq77_e1128_d_b25: f64 = (s.db[191][25] * ddt_scale);
        let eq77_e1128_d_b26: f64 = (s.db[191][26] * ddt_scale);
        let eq77_e1128_d_b27: f64 = (s.db[191][27] * ddt_scale);
        let eq77_e1128_d_b28: f64 = (s.db[191][28] * ddt_scale);
        let eq77_e1128_d_b29: f64 = (s.db[191][29] * ddt_scale);
        let eq77_e1128_d_b30: f64 = (s.db[191][30] * ddt_scale);
        let eq77_e1128_d_b31: f64 = (s.db[191][31] * ddt_scale);
        let eq77_e1128_d_b32: f64 = (s.db[191][32] * ddt_scale);
        let eq77_e1128_d_b33: f64 = (s.db[191][33] * ddt_scale);
        let eq77_e1128_d_b34: f64 = (s.db[191][34] * ddt_scale);
        let eq77_e1128_d_b35: f64 = (s.db[191][35] * ddt_scale);
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1131_d_n2: f64 = p.p355;
        let eq77_e1131_d_n5: f64 = (-p.p355);
        let eq77_e1132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 61, eq77_e1131);
        let eq77_e1132_d_n2: f64 = (eq77_e1131_d_n2 * ddt_scale);
        let eq77_e1132_d_n5: f64 = (eq77_e1131_d_n5 * ddt_scale);
        let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);
        let eq77_e1133_d_n2: f64 = (eq77_e1128_d_n2 + eq77_e1132_d_n2);
        let eq77_e1133_d_n5: f64 = (eq77_e1128_d_n5 + eq77_e1132_d_n5);
        (eq77_e1133, eq77_e1128_d_n0, eq77_e1128_d_n1, eq77_e1133_d_n2, eq77_e1128_d_n3, eq77_e1128_d_n4, eq77_e1133_d_n5, eq77_e1128_d_n6, eq77_e1128_d_n7, eq77_e1128_d_n8, eq77_e1128_d_n9, eq77_e1128_d_n10, eq77_e1128_d_n11, eq77_e1128_d_n12, eq77_e1128_d_n13, eq77_e1128_d_n14, eq77_e1128_d_n15, eq77_e1128_d_n16, eq77_e1128_d_n17, eq77_e1128_d_n18, eq77_e1128_d_n19, eq77_e1128_d_n20, eq77_e1128_d_n21, eq77_e1128_d_n22, eq77_e1128_d_n23, eq77_e1128_d_n24, eq77_e1128_d_n25, eq77_e1128_d_n26, eq77_e1128_d_n27, eq77_e1128_d_n28, eq77_e1128_d_n29, eq77_e1128_d_b0, eq77_e1128_d_b1, eq77_e1128_d_b2, eq77_e1128_d_b3, eq77_e1128_d_b4, eq77_e1128_d_b5, eq77_e1128_d_b6, eq77_e1128_d_b7, eq77_e1128_d_b8, eq77_e1128_d_b9, eq77_e1128_d_b10, eq77_e1128_d_b11, eq77_e1128_d_b12, eq77_e1128_d_b13, eq77_e1128_d_b14, eq77_e1128_d_b15, eq77_e1128_d_b16, eq77_e1128_d_b17, eq77_e1128_d_b18, eq77_e1128_d_b19, eq77_e1128_d_b20, eq77_e1128_d_b21, eq77_e1128_d_b22, eq77_e1128_d_b23, eq77_e1128_d_b24, eq77_e1128_d_b25, eq77_e1128_d_b26, eq77_e1128_d_b27, eq77_e1128_d_b28, eq77_e1128_d_b29, eq77_e1128_d_b30, eq77_e1128_d_b31, eq77_e1128_d_b32, eq77_e1128_d_b33, eq77_e1128_d_b34, eq77_e1128_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        let eq77_node_derivatives: [f64; 30] = [eq77_e1135_d_n0, eq77_e1135_d_n1, eq77_e1135_d_n2, eq77_e1135_d_n3, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n6, eq77_e1135_d_n7, eq77_e1135_d_n8, eq77_e1135_d_n9, eq77_e1135_d_n10, eq77_e1135_d_n11, eq77_e1135_d_n12, eq77_e1135_d_n13, eq77_e1135_d_n14, eq77_e1135_d_n15, eq77_e1135_d_n16, eq77_e1135_d_n17, eq77_e1135_d_n18, eq77_e1135_d_n19, eq77_e1135_d_n20, eq77_e1135_d_n21, eq77_e1135_d_n22, eq77_e1135_d_n23, eq77_e1135_d_n24, eq77_e1135_d_n25, eq77_e1135_d_n26, eq77_e1135_d_n27, eq77_e1135_d_n28, eq77_e1135_d_n29];
        let eq77_branch_derivatives: [f64; 36] = [eq77_e1135_d_b0, eq77_e1135_d_b1, eq77_e1135_d_b2, eq77_e1135_d_b3, eq77_e1135_d_b4, eq77_e1135_d_b5, eq77_e1135_d_b6, eq77_e1135_d_b7, eq77_e1135_d_b8, eq77_e1135_d_b9, eq77_e1135_d_b10, eq77_e1135_d_b11, eq77_e1135_d_b12, eq77_e1135_d_b13, eq77_e1135_d_b14, eq77_e1135_d_b15, eq77_e1135_d_b16, eq77_e1135_d_b17, eq77_e1135_d_b18, eq77_e1135_d_b19, eq77_e1135_d_b20, eq77_e1135_d_b21, eq77_e1135_d_b22, eq77_e1135_d_b23, eq77_e1135_d_b24, eq77_e1135_d_b25, eq77_e1135_d_b26, eq77_e1135_d_b27, eq77_e1135_d_b28, eq77_e1135_d_b29, eq77_e1135_d_b30, eq77_e1135_d_b31, eq77_e1135_d_b32, eq77_e1135_d_b33, eq77_e1135_d_b34, eq77_e1135_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivatives,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq78_e1146, eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29, eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35,) = {
    if (!s.b[907]) {
        let eq78_e1139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 62, s.v[192]);
        let eq78_e1139_d_n0: f64 = (s.dn[192][0] * ddt_scale);
        let eq78_e1139_d_n1: f64 = (s.dn[192][1] * ddt_scale);
        let eq78_e1139_d_n2: f64 = (s.dn[192][2] * ddt_scale);
        let eq78_e1139_d_n3: f64 = (s.dn[192][3] * ddt_scale);
        let eq78_e1139_d_n4: f64 = (s.dn[192][4] * ddt_scale);
        let eq78_e1139_d_n5: f64 = (s.dn[192][5] * ddt_scale);
        let eq78_e1139_d_n6: f64 = (s.dn[192][6] * ddt_scale);
        let eq78_e1139_d_n7: f64 = (s.dn[192][7] * ddt_scale);
        let eq78_e1139_d_n8: f64 = (s.dn[192][8] * ddt_scale);
        let eq78_e1139_d_n9: f64 = (s.dn[192][9] * ddt_scale);
        let eq78_e1139_d_n10: f64 = (s.dn[192][10] * ddt_scale);
        let eq78_e1139_d_n11: f64 = (s.dn[192][11] * ddt_scale);
        let eq78_e1139_d_n12: f64 = (s.dn[192][12] * ddt_scale);
        let eq78_e1139_d_n13: f64 = (s.dn[192][13] * ddt_scale);
        let eq78_e1139_d_n14: f64 = (s.dn[192][14] * ddt_scale);
        let eq78_e1139_d_n15: f64 = (s.dn[192][15] * ddt_scale);
        let eq78_e1139_d_n16: f64 = (s.dn[192][16] * ddt_scale);
        let eq78_e1139_d_n17: f64 = (s.dn[192][17] * ddt_scale);
        let eq78_e1139_d_n18: f64 = (s.dn[192][18] * ddt_scale);
        let eq78_e1139_d_n19: f64 = (s.dn[192][19] * ddt_scale);
        let eq78_e1139_d_n20: f64 = (s.dn[192][20] * ddt_scale);
        let eq78_e1139_d_n21: f64 = (s.dn[192][21] * ddt_scale);
        let eq78_e1139_d_n22: f64 = (s.dn[192][22] * ddt_scale);
        let eq78_e1139_d_n23: f64 = (s.dn[192][23] * ddt_scale);
        let eq78_e1139_d_n24: f64 = (s.dn[192][24] * ddt_scale);
        let eq78_e1139_d_n25: f64 = (s.dn[192][25] * ddt_scale);
        let eq78_e1139_d_n26: f64 = (s.dn[192][26] * ddt_scale);
        let eq78_e1139_d_n27: f64 = (s.dn[192][27] * ddt_scale);
        let eq78_e1139_d_n28: f64 = (s.dn[192][28] * ddt_scale);
        let eq78_e1139_d_n29: f64 = (s.dn[192][29] * ddt_scale);
        let eq78_e1139_d_b0: f64 = (s.db[192][0] * ddt_scale);
        let eq78_e1139_d_b1: f64 = (s.db[192][1] * ddt_scale);
        let eq78_e1139_d_b2: f64 = (s.db[192][2] * ddt_scale);
        let eq78_e1139_d_b3: f64 = (s.db[192][3] * ddt_scale);
        let eq78_e1139_d_b4: f64 = (s.db[192][4] * ddt_scale);
        let eq78_e1139_d_b5: f64 = (s.db[192][5] * ddt_scale);
        let eq78_e1139_d_b6: f64 = (s.db[192][6] * ddt_scale);
        let eq78_e1139_d_b7: f64 = (s.db[192][7] * ddt_scale);
        let eq78_e1139_d_b8: f64 = (s.db[192][8] * ddt_scale);
        let eq78_e1139_d_b9: f64 = (s.db[192][9] * ddt_scale);
        let eq78_e1139_d_b10: f64 = (s.db[192][10] * ddt_scale);
        let eq78_e1139_d_b11: f64 = (s.db[192][11] * ddt_scale);
        let eq78_e1139_d_b12: f64 = (s.db[192][12] * ddt_scale);
        let eq78_e1139_d_b13: f64 = (s.db[192][13] * ddt_scale);
        let eq78_e1139_d_b14: f64 = (s.db[192][14] * ddt_scale);
        let eq78_e1139_d_b15: f64 = (s.db[192][15] * ddt_scale);
        let eq78_e1139_d_b16: f64 = (s.db[192][16] * ddt_scale);
        let eq78_e1139_d_b17: f64 = (s.db[192][17] * ddt_scale);
        let eq78_e1139_d_b18: f64 = (s.db[192][18] * ddt_scale);
        let eq78_e1139_d_b19: f64 = (s.db[192][19] * ddt_scale);
        let eq78_e1139_d_b20: f64 = (s.db[192][20] * ddt_scale);
        let eq78_e1139_d_b21: f64 = (s.db[192][21] * ddt_scale);
        let eq78_e1139_d_b22: f64 = (s.db[192][22] * ddt_scale);
        let eq78_e1139_d_b23: f64 = (s.db[192][23] * ddt_scale);
        let eq78_e1139_d_b24: f64 = (s.db[192][24] * ddt_scale);
        let eq78_e1139_d_b25: f64 = (s.db[192][25] * ddt_scale);
        let eq78_e1139_d_b26: f64 = (s.db[192][26] * ddt_scale);
        let eq78_e1139_d_b27: f64 = (s.db[192][27] * ddt_scale);
        let eq78_e1139_d_b28: f64 = (s.db[192][28] * ddt_scale);
        let eq78_e1139_d_b29: f64 = (s.db[192][29] * ddt_scale);
        let eq78_e1139_d_b30: f64 = (s.db[192][30] * ddt_scale);
        let eq78_e1139_d_b31: f64 = (s.db[192][31] * ddt_scale);
        let eq78_e1139_d_b32: f64 = (s.db[192][32] * ddt_scale);
        let eq78_e1139_d_b33: f64 = (s.db[192][33] * ddt_scale);
        let eq78_e1139_d_b34: f64 = (s.db[192][34] * ddt_scale);
        let eq78_e1139_d_b35: f64 = (s.db[192][35] * ddt_scale);
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1142_d_n2: f64 = p.p355;
        let eq78_e1142_d_n14: f64 = (-p.p355);
        let eq78_e1143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 63, eq78_e1142);
        let eq78_e1143_d_n2: f64 = (eq78_e1142_d_n2 * ddt_scale);
        let eq78_e1143_d_n14: f64 = (eq78_e1142_d_n14 * ddt_scale);
        let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);
        let eq78_e1144_d_n2: f64 = (eq78_e1139_d_n2 + eq78_e1143_d_n2);
        let eq78_e1144_d_n14: f64 = (eq78_e1139_d_n14 + eq78_e1143_d_n14);
        (eq78_e1144, eq78_e1139_d_n0, eq78_e1139_d_n1, eq78_e1144_d_n2, eq78_e1139_d_n3, eq78_e1139_d_n4, eq78_e1139_d_n5, eq78_e1139_d_n6, eq78_e1139_d_n7, eq78_e1139_d_n8, eq78_e1139_d_n9, eq78_e1139_d_n10, eq78_e1139_d_n11, eq78_e1139_d_n12, eq78_e1139_d_n13, eq78_e1144_d_n14, eq78_e1139_d_n15, eq78_e1139_d_n16, eq78_e1139_d_n17, eq78_e1139_d_n18, eq78_e1139_d_n19, eq78_e1139_d_n20, eq78_e1139_d_n21, eq78_e1139_d_n22, eq78_e1139_d_n23, eq78_e1139_d_n24, eq78_e1139_d_n25, eq78_e1139_d_n26, eq78_e1139_d_n27, eq78_e1139_d_n28, eq78_e1139_d_n29, eq78_e1139_d_b0, eq78_e1139_d_b1, eq78_e1139_d_b2, eq78_e1139_d_b3, eq78_e1139_d_b4, eq78_e1139_d_b5, eq78_e1139_d_b6, eq78_e1139_d_b7, eq78_e1139_d_b8, eq78_e1139_d_b9, eq78_e1139_d_b10, eq78_e1139_d_b11, eq78_e1139_d_b12, eq78_e1139_d_b13, eq78_e1139_d_b14, eq78_e1139_d_b15, eq78_e1139_d_b16, eq78_e1139_d_b17, eq78_e1139_d_b18, eq78_e1139_d_b19, eq78_e1139_d_b20, eq78_e1139_d_b21, eq78_e1139_d_b22, eq78_e1139_d_b23, eq78_e1139_d_b24, eq78_e1139_d_b25, eq78_e1139_d_b26, eq78_e1139_d_b27, eq78_e1139_d_b28, eq78_e1139_d_b29, eq78_e1139_d_b30, eq78_e1139_d_b31, eq78_e1139_d_b32, eq78_e1139_d_b33, eq78_e1139_d_b34, eq78_e1139_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        let eq78_node_derivatives: [f64; 30] = [eq78_e1146_d_n0, eq78_e1146_d_n1, eq78_e1146_d_n2, eq78_e1146_d_n3, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n6, eq78_e1146_d_n7, eq78_e1146_d_n8, eq78_e1146_d_n9, eq78_e1146_d_n10, eq78_e1146_d_n11, eq78_e1146_d_n12, eq78_e1146_d_n13, eq78_e1146_d_n14, eq78_e1146_d_n15, eq78_e1146_d_n16, eq78_e1146_d_n17, eq78_e1146_d_n18, eq78_e1146_d_n19, eq78_e1146_d_n20, eq78_e1146_d_n21, eq78_e1146_d_n22, eq78_e1146_d_n23, eq78_e1146_d_n24, eq78_e1146_d_n25, eq78_e1146_d_n26, eq78_e1146_d_n27, eq78_e1146_d_n28, eq78_e1146_d_n29];
        let eq78_branch_derivatives: [f64; 36] = [eq78_e1146_d_b0, eq78_e1146_d_b1, eq78_e1146_d_b2, eq78_e1146_d_b3, eq78_e1146_d_b4, eq78_e1146_d_b5, eq78_e1146_d_b6, eq78_e1146_d_b7, eq78_e1146_d_b8, eq78_e1146_d_b9, eq78_e1146_d_b10, eq78_e1146_d_b11, eq78_e1146_d_b12, eq78_e1146_d_b13, eq78_e1146_d_b14, eq78_e1146_d_b15, eq78_e1146_d_b16, eq78_e1146_d_b17, eq78_e1146_d_b18, eq78_e1146_d_b19, eq78_e1146_d_b20, eq78_e1146_d_b21, eq78_e1146_d_b22, eq78_e1146_d_b23, eq78_e1146_d_b24, eq78_e1146_d_b25, eq78_e1146_d_b26, eq78_e1146_d_b27, eq78_e1146_d_b28, eq78_e1146_d_b29, eq78_e1146_d_b30, eq78_e1146_d_b31, eq78_e1146_d_b32, eq78_e1146_d_b33, eq78_e1146_d_b34, eq78_e1146_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(14),
            multiplicity * (eq78_value),
            &eq78_node_derivatives,
            &eq78_branch_derivatives,
            multiplicity,
        );
        let (eq79_e1157, eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29, eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35,) = {
    if (!s.b[907]) {
        let eq79_e1150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 64, s.v[193]);
        let eq79_e1150_d_n0: f64 = (s.dn[193][0] * ddt_scale);
        let eq79_e1150_d_n1: f64 = (s.dn[193][1] * ddt_scale);
        let eq79_e1150_d_n2: f64 = (s.dn[193][2] * ddt_scale);
        let eq79_e1150_d_n3: f64 = (s.dn[193][3] * ddt_scale);
        let eq79_e1150_d_n4: f64 = (s.dn[193][4] * ddt_scale);
        let eq79_e1150_d_n5: f64 = (s.dn[193][5] * ddt_scale);
        let eq79_e1150_d_n6: f64 = (s.dn[193][6] * ddt_scale);
        let eq79_e1150_d_n7: f64 = (s.dn[193][7] * ddt_scale);
        let eq79_e1150_d_n8: f64 = (s.dn[193][8] * ddt_scale);
        let eq79_e1150_d_n9: f64 = (s.dn[193][9] * ddt_scale);
        let eq79_e1150_d_n10: f64 = (s.dn[193][10] * ddt_scale);
        let eq79_e1150_d_n11: f64 = (s.dn[193][11] * ddt_scale);
        let eq79_e1150_d_n12: f64 = (s.dn[193][12] * ddt_scale);
        let eq79_e1150_d_n13: f64 = (s.dn[193][13] * ddt_scale);
        let eq79_e1150_d_n14: f64 = (s.dn[193][14] * ddt_scale);
        let eq79_e1150_d_n15: f64 = (s.dn[193][15] * ddt_scale);
        let eq79_e1150_d_n16: f64 = (s.dn[193][16] * ddt_scale);
        let eq79_e1150_d_n17: f64 = (s.dn[193][17] * ddt_scale);
        let eq79_e1150_d_n18: f64 = (s.dn[193][18] * ddt_scale);
        let eq79_e1150_d_n19: f64 = (s.dn[193][19] * ddt_scale);
        let eq79_e1150_d_n20: f64 = (s.dn[193][20] * ddt_scale);
        let eq79_e1150_d_n21: f64 = (s.dn[193][21] * ddt_scale);
        let eq79_e1150_d_n22: f64 = (s.dn[193][22] * ddt_scale);
        let eq79_e1150_d_n23: f64 = (s.dn[193][23] * ddt_scale);
        let eq79_e1150_d_n24: f64 = (s.dn[193][24] * ddt_scale);
        let eq79_e1150_d_n25: f64 = (s.dn[193][25] * ddt_scale);
        let eq79_e1150_d_n26: f64 = (s.dn[193][26] * ddt_scale);
        let eq79_e1150_d_n27: f64 = (s.dn[193][27] * ddt_scale);
        let eq79_e1150_d_n28: f64 = (s.dn[193][28] * ddt_scale);
        let eq79_e1150_d_n29: f64 = (s.dn[193][29] * ddt_scale);
        let eq79_e1150_d_b0: f64 = (s.db[193][0] * ddt_scale);
        let eq79_e1150_d_b1: f64 = (s.db[193][1] * ddt_scale);
        let eq79_e1150_d_b2: f64 = (s.db[193][2] * ddt_scale);
        let eq79_e1150_d_b3: f64 = (s.db[193][3] * ddt_scale);
        let eq79_e1150_d_b4: f64 = (s.db[193][4] * ddt_scale);
        let eq79_e1150_d_b5: f64 = (s.db[193][5] * ddt_scale);
        let eq79_e1150_d_b6: f64 = (s.db[193][6] * ddt_scale);
        let eq79_e1150_d_b7: f64 = (s.db[193][7] * ddt_scale);
        let eq79_e1150_d_b8: f64 = (s.db[193][8] * ddt_scale);
        let eq79_e1150_d_b9: f64 = (s.db[193][9] * ddt_scale);
        let eq79_e1150_d_b10: f64 = (s.db[193][10] * ddt_scale);
        let eq79_e1150_d_b11: f64 = (s.db[193][11] * ddt_scale);
        let eq79_e1150_d_b12: f64 = (s.db[193][12] * ddt_scale);
        let eq79_e1150_d_b13: f64 = (s.db[193][13] * ddt_scale);
        let eq79_e1150_d_b14: f64 = (s.db[193][14] * ddt_scale);
        let eq79_e1150_d_b15: f64 = (s.db[193][15] * ddt_scale);
        let eq79_e1150_d_b16: f64 = (s.db[193][16] * ddt_scale);
        let eq79_e1150_d_b17: f64 = (s.db[193][17] * ddt_scale);
        let eq79_e1150_d_b18: f64 = (s.db[193][18] * ddt_scale);
        let eq79_e1150_d_b19: f64 = (s.db[193][19] * ddt_scale);
        let eq79_e1150_d_b20: f64 = (s.db[193][20] * ddt_scale);
        let eq79_e1150_d_b21: f64 = (s.db[193][21] * ddt_scale);
        let eq79_e1150_d_b22: f64 = (s.db[193][22] * ddt_scale);
        let eq79_e1150_d_b23: f64 = (s.db[193][23] * ddt_scale);
        let eq79_e1150_d_b24: f64 = (s.db[193][24] * ddt_scale);
        let eq79_e1150_d_b25: f64 = (s.db[193][25] * ddt_scale);
        let eq79_e1150_d_b26: f64 = (s.db[193][26] * ddt_scale);
        let eq79_e1150_d_b27: f64 = (s.db[193][27] * ddt_scale);
        let eq79_e1150_d_b28: f64 = (s.db[193][28] * ddt_scale);
        let eq79_e1150_d_b29: f64 = (s.db[193][29] * ddt_scale);
        let eq79_e1150_d_b30: f64 = (s.db[193][30] * ddt_scale);
        let eq79_e1150_d_b31: f64 = (s.db[193][31] * ddt_scale);
        let eq79_e1150_d_b32: f64 = (s.db[193][32] * ddt_scale);
        let eq79_e1150_d_b33: f64 = (s.db[193][33] * ddt_scale);
        let eq79_e1150_d_b34: f64 = (s.db[193][34] * ddt_scale);
        let eq79_e1150_d_b35: f64 = (s.db[193][35] * ddt_scale);
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1153_d_n5: f64 = (-p.p355);
        let eq79_e1153_d_n7: f64 = p.p355;
        let eq79_e1154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 65, eq79_e1153);
        let eq79_e1154_d_n5: f64 = (eq79_e1153_d_n5 * ddt_scale);
        let eq79_e1154_d_n7: f64 = (eq79_e1153_d_n7 * ddt_scale);
        let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);
        let eq79_e1155_d_n5: f64 = (eq79_e1150_d_n5 + eq79_e1154_d_n5);
        let eq79_e1155_d_n7: f64 = (eq79_e1150_d_n7 + eq79_e1154_d_n7);
        (eq79_e1155, eq79_e1150_d_n0, eq79_e1150_d_n1, eq79_e1150_d_n2, eq79_e1150_d_n3, eq79_e1150_d_n4, eq79_e1155_d_n5, eq79_e1150_d_n6, eq79_e1155_d_n7, eq79_e1150_d_n8, eq79_e1150_d_n9, eq79_e1150_d_n10, eq79_e1150_d_n11, eq79_e1150_d_n12, eq79_e1150_d_n13, eq79_e1150_d_n14, eq79_e1150_d_n15, eq79_e1150_d_n16, eq79_e1150_d_n17, eq79_e1150_d_n18, eq79_e1150_d_n19, eq79_e1150_d_n20, eq79_e1150_d_n21, eq79_e1150_d_n22, eq79_e1150_d_n23, eq79_e1150_d_n24, eq79_e1150_d_n25, eq79_e1150_d_n26, eq79_e1150_d_n27, eq79_e1150_d_n28, eq79_e1150_d_n29, eq79_e1150_d_b0, eq79_e1150_d_b1, eq79_e1150_d_b2, eq79_e1150_d_b3, eq79_e1150_d_b4, eq79_e1150_d_b5, eq79_e1150_d_b6, eq79_e1150_d_b7, eq79_e1150_d_b8, eq79_e1150_d_b9, eq79_e1150_d_b10, eq79_e1150_d_b11, eq79_e1150_d_b12, eq79_e1150_d_b13, eq79_e1150_d_b14, eq79_e1150_d_b15, eq79_e1150_d_b16, eq79_e1150_d_b17, eq79_e1150_d_b18, eq79_e1150_d_b19, eq79_e1150_d_b20, eq79_e1150_d_b21, eq79_e1150_d_b22, eq79_e1150_d_b23, eq79_e1150_d_b24, eq79_e1150_d_b25, eq79_e1150_d_b26, eq79_e1150_d_b27, eq79_e1150_d_b28, eq79_e1150_d_b29, eq79_e1150_d_b30, eq79_e1150_d_b31, eq79_e1150_d_b32, eq79_e1150_d_b33, eq79_e1150_d_b34, eq79_e1150_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        let eq79_node_derivatives: [f64; 30] = [eq79_e1157_d_n0, eq79_e1157_d_n1, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n6, eq79_e1157_d_n7, eq79_e1157_d_n8, eq79_e1157_d_n9, eq79_e1157_d_n10, eq79_e1157_d_n11, eq79_e1157_d_n12, eq79_e1157_d_n13, eq79_e1157_d_n14, eq79_e1157_d_n15, eq79_e1157_d_n16, eq79_e1157_d_n17, eq79_e1157_d_n18, eq79_e1157_d_n19, eq79_e1157_d_n20, eq79_e1157_d_n21, eq79_e1157_d_n22, eq79_e1157_d_n23, eq79_e1157_d_n24, eq79_e1157_d_n25, eq79_e1157_d_n26, eq79_e1157_d_n27, eq79_e1157_d_n28, eq79_e1157_d_n29];
        let eq79_branch_derivatives: [f64; 36] = [eq79_e1157_d_b0, eq79_e1157_d_b1, eq79_e1157_d_b2, eq79_e1157_d_b3, eq79_e1157_d_b4, eq79_e1157_d_b5, eq79_e1157_d_b6, eq79_e1157_d_b7, eq79_e1157_d_b8, eq79_e1157_d_b9, eq79_e1157_d_b10, eq79_e1157_d_b11, eq79_e1157_d_b12, eq79_e1157_d_b13, eq79_e1157_d_b14, eq79_e1157_d_b15, eq79_e1157_d_b16, eq79_e1157_d_b17, eq79_e1157_d_b18, eq79_e1157_d_b19, eq79_e1157_d_b20, eq79_e1157_d_b21, eq79_e1157_d_b22, eq79_e1157_d_b23, eq79_e1157_d_b24, eq79_e1157_d_b25, eq79_e1157_d_b26, eq79_e1157_d_b27, eq79_e1157_d_b28, eq79_e1157_d_b29, eq79_e1157_d_b30, eq79_e1157_d_b31, eq79_e1157_d_b32, eq79_e1157_d_b33, eq79_e1157_d_b34, eq79_e1157_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
            multiplicity,
        );
        let (eq80_e1162,) = {
    if (!s.b[907]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e1162;
        stamper.stamp_current_const_local(
            Some(7),
            Some(14),
            multiplicity * (eq80_value),
        );
        let (eq81_e1167,) = {
    if (!s.b[907]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e1167;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq81_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq82_e1169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 66, s.v[194]);
        let eq82_e1169_d_n0: f64 = (s.dn[194][0] * ddt_scale);
        let eq82_e1169_d_n1: f64 = (s.dn[194][1] * ddt_scale);
        let eq82_e1169_d_n2: f64 = (s.dn[194][2] * ddt_scale);
        let eq82_e1169_d_n3: f64 = (s.dn[194][3] * ddt_scale);
        let eq82_e1169_d_n4: f64 = (s.dn[194][4] * ddt_scale);
        let eq82_e1169_d_n5: f64 = (s.dn[194][5] * ddt_scale);
        let eq82_e1169_d_n6: f64 = (s.dn[194][6] * ddt_scale);
        let eq82_e1169_d_n7: f64 = (s.dn[194][7] * ddt_scale);
        let eq82_e1169_d_n8: f64 = (s.dn[194][8] * ddt_scale);
        let eq82_e1169_d_n9: f64 = (s.dn[194][9] * ddt_scale);
        let eq82_e1169_d_n10: f64 = (s.dn[194][10] * ddt_scale);
        let eq82_e1169_d_n11: f64 = (s.dn[194][11] * ddt_scale);
        let eq82_e1169_d_n12: f64 = (s.dn[194][12] * ddt_scale);
        let eq82_e1169_d_n13: f64 = (s.dn[194][13] * ddt_scale);
        let eq82_e1169_d_n14: f64 = (s.dn[194][14] * ddt_scale);
        let eq82_e1169_d_n15: f64 = (s.dn[194][15] * ddt_scale);
        let eq82_e1169_d_n16: f64 = (s.dn[194][16] * ddt_scale);
        let eq82_e1169_d_n17: f64 = (s.dn[194][17] * ddt_scale);
        let eq82_e1169_d_n18: f64 = (s.dn[194][18] * ddt_scale);
        let eq82_e1169_d_n19: f64 = (s.dn[194][19] * ddt_scale);
        let eq82_e1169_d_n20: f64 = (s.dn[194][20] * ddt_scale);
        let eq82_e1169_d_n21: f64 = (s.dn[194][21] * ddt_scale);
        let eq82_e1169_d_n22: f64 = (s.dn[194][22] * ddt_scale);
        let eq82_e1169_d_n23: f64 = (s.dn[194][23] * ddt_scale);
        let eq82_e1169_d_n24: f64 = (s.dn[194][24] * ddt_scale);
        let eq82_e1169_d_n25: f64 = (s.dn[194][25] * ddt_scale);
        let eq82_e1169_d_n26: f64 = (s.dn[194][26] * ddt_scale);
        let eq82_e1169_d_n27: f64 = (s.dn[194][27] * ddt_scale);
        let eq82_e1169_d_n28: f64 = (s.dn[194][28] * ddt_scale);
        let eq82_e1169_d_n29: f64 = (s.dn[194][29] * ddt_scale);
        let eq82_e1169_d_b0: f64 = (s.db[194][0] * ddt_scale);
        let eq82_e1169_d_b1: f64 = (s.db[194][1] * ddt_scale);
        let eq82_e1169_d_b2: f64 = (s.db[194][2] * ddt_scale);
        let eq82_e1169_d_b3: f64 = (s.db[194][3] * ddt_scale);
        let eq82_e1169_d_b4: f64 = (s.db[194][4] * ddt_scale);
        let eq82_e1169_d_b5: f64 = (s.db[194][5] * ddt_scale);
        let eq82_e1169_d_b6: f64 = (s.db[194][6] * ddt_scale);
        let eq82_e1169_d_b7: f64 = (s.db[194][7] * ddt_scale);
        let eq82_e1169_d_b8: f64 = (s.db[194][8] * ddt_scale);
        let eq82_e1169_d_b9: f64 = (s.db[194][9] * ddt_scale);
        let eq82_e1169_d_b10: f64 = (s.db[194][10] * ddt_scale);
        let eq82_e1169_d_b11: f64 = (s.db[194][11] * ddt_scale);
        let eq82_e1169_d_b12: f64 = (s.db[194][12] * ddt_scale);
        let eq82_e1169_d_b13: f64 = (s.db[194][13] * ddt_scale);
        let eq82_e1169_d_b14: f64 = (s.db[194][14] * ddt_scale);
        let eq82_e1169_d_b15: f64 = (s.db[194][15] * ddt_scale);
        let eq82_e1169_d_b16: f64 = (s.db[194][16] * ddt_scale);
        let eq82_e1169_d_b17: f64 = (s.db[194][17] * ddt_scale);
        let eq82_e1169_d_b18: f64 = (s.db[194][18] * ddt_scale);
        let eq82_e1169_d_b19: f64 = (s.db[194][19] * ddt_scale);
        let eq82_e1169_d_b20: f64 = (s.db[194][20] * ddt_scale);
        let eq82_e1169_d_b21: f64 = (s.db[194][21] * ddt_scale);
        let eq82_e1169_d_b22: f64 = (s.db[194][22] * ddt_scale);
        let eq82_e1169_d_b23: f64 = (s.db[194][23] * ddt_scale);
        let eq82_e1169_d_b24: f64 = (s.db[194][24] * ddt_scale);
        let eq82_e1169_d_b25: f64 = (s.db[194][25] * ddt_scale);
        let eq82_e1169_d_b26: f64 = (s.db[194][26] * ddt_scale);
        let eq82_e1169_d_b27: f64 = (s.db[194][27] * ddt_scale);
        let eq82_e1169_d_b28: f64 = (s.db[194][28] * ddt_scale);
        let eq82_e1169_d_b29: f64 = (s.db[194][29] * ddt_scale);
        let eq82_e1169_d_b30: f64 = (s.db[194][30] * ddt_scale);
        let eq82_e1169_d_b31: f64 = (s.db[194][31] * ddt_scale);
        let eq82_e1169_d_b32: f64 = (s.db[194][32] * ddt_scale);
        let eq82_e1169_d_b33: f64 = (s.db[194][33] * ddt_scale);
        let eq82_e1169_d_b34: f64 = (s.db[194][34] * ddt_scale);
        let eq82_e1169_d_b35: f64 = (s.db[194][35] * ddt_scale);
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1172_d_n3: f64 = p.p355;
        let eq82_e1172_d_n5: f64 = (-p.p355);
        let eq82_e1173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 67, eq82_e1172);
        let eq82_e1173_d_n3: f64 = (eq82_e1172_d_n3 * ddt_scale);
        let eq82_e1173_d_n5: f64 = (eq82_e1172_d_n5 * ddt_scale);
        let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);
        let eq82_e1174_d_n3: f64 = (eq82_e1169_d_n3 + eq82_e1173_d_n3);
        let eq82_e1174_d_n5: f64 = (eq82_e1169_d_n5 + eq82_e1173_d_n5);
        let eq82_value: f64 = eq82_e1174;
        let eq82_node_derivatives: [f64; 30] = [eq82_e1169_d_n0, eq82_e1169_d_n1, eq82_e1169_d_n2, eq82_e1174_d_n3, eq82_e1169_d_n4, eq82_e1174_d_n5, eq82_e1169_d_n6, eq82_e1169_d_n7, eq82_e1169_d_n8, eq82_e1169_d_n9, eq82_e1169_d_n10, eq82_e1169_d_n11, eq82_e1169_d_n12, eq82_e1169_d_n13, eq82_e1169_d_n14, eq82_e1169_d_n15, eq82_e1169_d_n16, eq82_e1169_d_n17, eq82_e1169_d_n18, eq82_e1169_d_n19, eq82_e1169_d_n20, eq82_e1169_d_n21, eq82_e1169_d_n22, eq82_e1169_d_n23, eq82_e1169_d_n24, eq82_e1169_d_n25, eq82_e1169_d_n26, eq82_e1169_d_n27, eq82_e1169_d_n28, eq82_e1169_d_n29];
        let eq82_branch_derivatives: [f64; 36] = [eq82_e1169_d_b0, eq82_e1169_d_b1, eq82_e1169_d_b2, eq82_e1169_d_b3, eq82_e1169_d_b4, eq82_e1169_d_b5, eq82_e1169_d_b6, eq82_e1169_d_b7, eq82_e1169_d_b8, eq82_e1169_d_b9, eq82_e1169_d_b10, eq82_e1169_d_b11, eq82_e1169_d_b12, eq82_e1169_d_b13, eq82_e1169_d_b14, eq82_e1169_d_b15, eq82_e1169_d_b16, eq82_e1169_d_b17, eq82_e1169_d_b18, eq82_e1169_d_b19, eq82_e1169_d_b20, eq82_e1169_d_b21, eq82_e1169_d_b22, eq82_e1169_d_b23, eq82_e1169_d_b24, eq82_e1169_d_b25, eq82_e1169_d_b26, eq82_e1169_d_b27, eq82_e1169_d_b28, eq82_e1169_d_b29, eq82_e1169_d_b30, eq82_e1169_d_b31, eq82_e1169_d_b32, eq82_e1169_d_b33, eq82_e1169_d_b34, eq82_e1169_d_b35];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq82_value),
            &eq82_node_derivatives,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1182, eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29, eq83_e1182_d_b0, eq83_e1182_d_b1, eq83_e1182_d_b2, eq83_e1182_d_b3, eq83_e1182_d_b4, eq83_e1182_d_b5, eq83_e1182_d_b6, eq83_e1182_d_b7, eq83_e1182_d_b8, eq83_e1182_d_b9, eq83_e1182_d_b10, eq83_e1182_d_b11, eq83_e1182_d_b12, eq83_e1182_d_b13, eq83_e1182_d_b14, eq83_e1182_d_b15, eq83_e1182_d_b16, eq83_e1182_d_b17, eq83_e1182_d_b18, eq83_e1182_d_b19, eq83_e1182_d_b20, eq83_e1182_d_b21, eq83_e1182_d_b22, eq83_e1182_d_b23, eq83_e1182_d_b24, eq83_e1182_d_b25, eq83_e1182_d_b26, eq83_e1182_d_b27, eq83_e1182_d_b28, eq83_e1182_d_b29, eq83_e1182_d_b30, eq83_e1182_d_b31, eq83_e1182_d_b32, eq83_e1182_d_b33, eq83_e1182_d_b34, eq83_e1182_d_b35,) = {
    if s.b[908] {
        let eq83_e1179: f64 = (s.v[0] * (nv9 - nv10));
        let eq83_e1179_d_n9: f64 = s.v[0];
        let eq83_e1179_d_n10: f64 = (-s.v[0]);
        let eq83_e1180: f64 = (s.v[166] + eq83_e1179);
        let eq83_e1180_d_n9: f64 = (s.dn[166][9] + eq83_e1179_d_n9);
        let eq83_e1180_d_n10: f64 = (s.dn[166][10] + eq83_e1179_d_n10);
        (eq83_e1180, s.dn[166][0], s.dn[166][1], s.dn[166][2], s.dn[166][3], s.dn[166][4], s.dn[166][5], s.dn[166][6], s.dn[166][7], s.dn[166][8], eq83_e1180_d_n9, eq83_e1180_d_n10, s.dn[166][11], s.dn[166][12], s.dn[166][13], s.dn[166][14], s.dn[166][15], s.dn[166][16], s.dn[166][17], s.dn[166][18], s.dn[166][19], s.dn[166][20], s.dn[166][21], s.dn[166][22], s.dn[166][23], s.dn[166][24], s.dn[166][25], s.dn[166][26], s.dn[166][27], s.dn[166][28], s.dn[166][29], s.db[166][0], s.db[166][1], s.db[166][2], s.db[166][3], s.db[166][4], s.db[166][5], s.db[166][6], s.db[166][7], s.db[166][8], s.db[166][9], s.db[166][10], s.db[166][11], s.db[166][12], s.db[166][13], s.db[166][14], s.db[166][15], s.db[166][16], s.db[166][17], s.db[166][18], s.db[166][19], s.db[166][20], s.db[166][21], s.db[166][22], s.db[166][23], s.db[166][24], s.db[166][25], s.db[166][26], s.db[166][27], s.db[166][28], s.db[166][29], s.db[166][30], s.db[166][31], s.db[166][32], s.db[166][33], s.db[166][34], s.db[166][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        let eq83_node_derivatives: [f64; 30] = [eq83_e1182_d_n0, eq83_e1182_d_n1, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n5, eq83_e1182_d_n6, eq83_e1182_d_n7, eq83_e1182_d_n8, eq83_e1182_d_n9, eq83_e1182_d_n10, eq83_e1182_d_n11, eq83_e1182_d_n12, eq83_e1182_d_n13, eq83_e1182_d_n14, eq83_e1182_d_n15, eq83_e1182_d_n16, eq83_e1182_d_n17, eq83_e1182_d_n18, eq83_e1182_d_n19, eq83_e1182_d_n20, eq83_e1182_d_n21, eq83_e1182_d_n22, eq83_e1182_d_n23, eq83_e1182_d_n24, eq83_e1182_d_n25, eq83_e1182_d_n26, eq83_e1182_d_n27, eq83_e1182_d_n28, eq83_e1182_d_n29];
        let eq83_branch_derivatives: [f64; 36] = [eq83_e1182_d_b0, eq83_e1182_d_b1, eq83_e1182_d_b2, eq83_e1182_d_b3, eq83_e1182_d_b4, eq83_e1182_d_b5, eq83_e1182_d_b6, eq83_e1182_d_b7, eq83_e1182_d_b8, eq83_e1182_d_b9, eq83_e1182_d_b10, eq83_e1182_d_b11, eq83_e1182_d_b12, eq83_e1182_d_b13, eq83_e1182_d_b14, eq83_e1182_d_b15, eq83_e1182_d_b16, eq83_e1182_d_b17, eq83_e1182_d_b18, eq83_e1182_d_b19, eq83_e1182_d_b20, eq83_e1182_d_b21, eq83_e1182_d_b22, eq83_e1182_d_b23, eq83_e1182_d_b24, eq83_e1182_d_b25, eq83_e1182_d_b26, eq83_e1182_d_b27, eq83_e1182_d_b28, eq83_e1182_d_b29, eq83_e1182_d_b30, eq83_e1182_d_b31, eq83_e1182_d_b32, eq83_e1182_d_b33, eq83_e1182_d_b34, eq83_e1182_d_b35];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq83_value),
            &eq83_node_derivatives,
            &eq83_branch_derivatives,
            multiplicity,
        );
        let (eq84_e1187,) = {
    if (!s.b[908]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq84_value: f64 = eq84_e1187;
        stamper.stamp_potential_const_local(
            22,
            eq84_value,
        );
        let (eq85_e1197, eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29, eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35,) = {
    if s.b[1054] {
        let eq85_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 68, s.v[167]);
        let eq85_e1190_d_n0: f64 = (s.dn[167][0] * ddt_scale);
        let eq85_e1190_d_n1: f64 = (s.dn[167][1] * ddt_scale);
        let eq85_e1190_d_n2: f64 = (s.dn[167][2] * ddt_scale);
        let eq85_e1190_d_n3: f64 = (s.dn[167][3] * ddt_scale);
        let eq85_e1190_d_n4: f64 = (s.dn[167][4] * ddt_scale);
        let eq85_e1190_d_n5: f64 = (s.dn[167][5] * ddt_scale);
        let eq85_e1190_d_n6: f64 = (s.dn[167][6] * ddt_scale);
        let eq85_e1190_d_n7: f64 = (s.dn[167][7] * ddt_scale);
        let eq85_e1190_d_n8: f64 = (s.dn[167][8] * ddt_scale);
        let eq85_e1190_d_n9: f64 = (s.dn[167][9] * ddt_scale);
        let eq85_e1190_d_n10: f64 = (s.dn[167][10] * ddt_scale);
        let eq85_e1190_d_n11: f64 = (s.dn[167][11] * ddt_scale);
        let eq85_e1190_d_n12: f64 = (s.dn[167][12] * ddt_scale);
        let eq85_e1190_d_n13: f64 = (s.dn[167][13] * ddt_scale);
        let eq85_e1190_d_n14: f64 = (s.dn[167][14] * ddt_scale);
        let eq85_e1190_d_n15: f64 = (s.dn[167][15] * ddt_scale);
        let eq85_e1190_d_n16: f64 = (s.dn[167][16] * ddt_scale);
        let eq85_e1190_d_n17: f64 = (s.dn[167][17] * ddt_scale);
        let eq85_e1190_d_n18: f64 = (s.dn[167][18] * ddt_scale);
        let eq85_e1190_d_n19: f64 = (s.dn[167][19] * ddt_scale);
        let eq85_e1190_d_n20: f64 = (s.dn[167][20] * ddt_scale);
        let eq85_e1190_d_n21: f64 = (s.dn[167][21] * ddt_scale);
        let eq85_e1190_d_n22: f64 = (s.dn[167][22] * ddt_scale);
        let eq85_e1190_d_n23: f64 = (s.dn[167][23] * ddt_scale);
        let eq85_e1190_d_n24: f64 = (s.dn[167][24] * ddt_scale);
        let eq85_e1190_d_n25: f64 = (s.dn[167][25] * ddt_scale);
        let eq85_e1190_d_n26: f64 = (s.dn[167][26] * ddt_scale);
        let eq85_e1190_d_n27: f64 = (s.dn[167][27] * ddt_scale);
        let eq85_e1190_d_n28: f64 = (s.dn[167][28] * ddt_scale);
        let eq85_e1190_d_n29: f64 = (s.dn[167][29] * ddt_scale);
        let eq85_e1190_d_b0: f64 = (s.db[167][0] * ddt_scale);
        let eq85_e1190_d_b1: f64 = (s.db[167][1] * ddt_scale);
        let eq85_e1190_d_b2: f64 = (s.db[167][2] * ddt_scale);
        let eq85_e1190_d_b3: f64 = (s.db[167][3] * ddt_scale);
        let eq85_e1190_d_b4: f64 = (s.db[167][4] * ddt_scale);
        let eq85_e1190_d_b5: f64 = (s.db[167][5] * ddt_scale);
        let eq85_e1190_d_b6: f64 = (s.db[167][6] * ddt_scale);
        let eq85_e1190_d_b7: f64 = (s.db[167][7] * ddt_scale);
        let eq85_e1190_d_b8: f64 = (s.db[167][8] * ddt_scale);
        let eq85_e1190_d_b9: f64 = (s.db[167][9] * ddt_scale);
        let eq85_e1190_d_b10: f64 = (s.db[167][10] * ddt_scale);
        let eq85_e1190_d_b11: f64 = (s.db[167][11] * ddt_scale);
        let eq85_e1190_d_b12: f64 = (s.db[167][12] * ddt_scale);
        let eq85_e1190_d_b13: f64 = (s.db[167][13] * ddt_scale);
        let eq85_e1190_d_b14: f64 = (s.db[167][14] * ddt_scale);
        let eq85_e1190_d_b15: f64 = (s.db[167][15] * ddt_scale);
        let eq85_e1190_d_b16: f64 = (s.db[167][16] * ddt_scale);
        let eq85_e1190_d_b17: f64 = (s.db[167][17] * ddt_scale);
        let eq85_e1190_d_b18: f64 = (s.db[167][18] * ddt_scale);
        let eq85_e1190_d_b19: f64 = (s.db[167][19] * ddt_scale);
        let eq85_e1190_d_b20: f64 = (s.db[167][20] * ddt_scale);
        let eq85_e1190_d_b21: f64 = (s.db[167][21] * ddt_scale);
        let eq85_e1190_d_b22: f64 = (s.db[167][22] * ddt_scale);
        let eq85_e1190_d_b23: f64 = (s.db[167][23] * ddt_scale);
        let eq85_e1190_d_b24: f64 = (s.db[167][24] * ddt_scale);
        let eq85_e1190_d_b25: f64 = (s.db[167][25] * ddt_scale);
        let eq85_e1190_d_b26: f64 = (s.db[167][26] * ddt_scale);
        let eq85_e1190_d_b27: f64 = (s.db[167][27] * ddt_scale);
        let eq85_e1190_d_b28: f64 = (s.db[167][28] * ddt_scale);
        let eq85_e1190_d_b29: f64 = (s.db[167][29] * ddt_scale);
        let eq85_e1190_d_b30: f64 = (s.db[167][30] * ddt_scale);
        let eq85_e1190_d_b31: f64 = (s.db[167][31] * ddt_scale);
        let eq85_e1190_d_b32: f64 = (s.db[167][32] * ddt_scale);
        let eq85_e1190_d_b33: f64 = (s.db[167][33] * ddt_scale);
        let eq85_e1190_d_b34: f64 = (s.db[167][34] * ddt_scale);
        let eq85_e1190_d_b35: f64 = (s.db[167][35] * ddt_scale);
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1193_d_n7: f64 = p.p355;
        let eq85_e1193_d_n10: f64 = (-p.p355);
        let eq85_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 69, eq85_e1193);
        let eq85_e1194_d_n7: f64 = (eq85_e1193_d_n7 * ddt_scale);
        let eq85_e1194_d_n10: f64 = (eq85_e1193_d_n10 * ddt_scale);
        let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);
        let eq85_e1195_d_n7: f64 = (eq85_e1190_d_n7 + eq85_e1194_d_n7);
        let eq85_e1195_d_n10: f64 = (eq85_e1190_d_n10 + eq85_e1194_d_n10);
        (eq85_e1195, eq85_e1190_d_n0, eq85_e1190_d_n1, eq85_e1190_d_n2, eq85_e1190_d_n3, eq85_e1190_d_n4, eq85_e1190_d_n5, eq85_e1190_d_n6, eq85_e1195_d_n7, eq85_e1190_d_n8, eq85_e1190_d_n9, eq85_e1195_d_n10, eq85_e1190_d_n11, eq85_e1190_d_n12, eq85_e1190_d_n13, eq85_e1190_d_n14, eq85_e1190_d_n15, eq85_e1190_d_n16, eq85_e1190_d_n17, eq85_e1190_d_n18, eq85_e1190_d_n19, eq85_e1190_d_n20, eq85_e1190_d_n21, eq85_e1190_d_n22, eq85_e1190_d_n23, eq85_e1190_d_n24, eq85_e1190_d_n25, eq85_e1190_d_n26, eq85_e1190_d_n27, eq85_e1190_d_n28, eq85_e1190_d_n29, eq85_e1190_d_b0, eq85_e1190_d_b1, eq85_e1190_d_b2, eq85_e1190_d_b3, eq85_e1190_d_b4, eq85_e1190_d_b5, eq85_e1190_d_b6, eq85_e1190_d_b7, eq85_e1190_d_b8, eq85_e1190_d_b9, eq85_e1190_d_b10, eq85_e1190_d_b11, eq85_e1190_d_b12, eq85_e1190_d_b13, eq85_e1190_d_b14, eq85_e1190_d_b15, eq85_e1190_d_b16, eq85_e1190_d_b17, eq85_e1190_d_b18, eq85_e1190_d_b19, eq85_e1190_d_b20, eq85_e1190_d_b21, eq85_e1190_d_b22, eq85_e1190_d_b23, eq85_e1190_d_b24, eq85_e1190_d_b25, eq85_e1190_d_b26, eq85_e1190_d_b27, eq85_e1190_d_b28, eq85_e1190_d_b29, eq85_e1190_d_b30, eq85_e1190_d_b31, eq85_e1190_d_b32, eq85_e1190_d_b33, eq85_e1190_d_b34, eq85_e1190_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        let eq85_node_derivatives: [f64; 30] = [eq85_e1197_d_n0, eq85_e1197_d_n1, eq85_e1197_d_n2, eq85_e1197_d_n3, eq85_e1197_d_n4, eq85_e1197_d_n5, eq85_e1197_d_n6, eq85_e1197_d_n7, eq85_e1197_d_n8, eq85_e1197_d_n9, eq85_e1197_d_n10, eq85_e1197_d_n11, eq85_e1197_d_n12, eq85_e1197_d_n13, eq85_e1197_d_n14, eq85_e1197_d_n15, eq85_e1197_d_n16, eq85_e1197_d_n17, eq85_e1197_d_n18, eq85_e1197_d_n19, eq85_e1197_d_n20, eq85_e1197_d_n21, eq85_e1197_d_n22, eq85_e1197_d_n23, eq85_e1197_d_n24, eq85_e1197_d_n25, eq85_e1197_d_n26, eq85_e1197_d_n27, eq85_e1197_d_n28, eq85_e1197_d_n29];
        let eq85_branch_derivatives: [f64; 36] = [eq85_e1197_d_b0, eq85_e1197_d_b1, eq85_e1197_d_b2, eq85_e1197_d_b3, eq85_e1197_d_b4, eq85_e1197_d_b5, eq85_e1197_d_b6, eq85_e1197_d_b7, eq85_e1197_d_b8, eq85_e1197_d_b9, eq85_e1197_d_b10, eq85_e1197_d_b11, eq85_e1197_d_b12, eq85_e1197_d_b13, eq85_e1197_d_b14, eq85_e1197_d_b15, eq85_e1197_d_b16, eq85_e1197_d_b17, eq85_e1197_d_b18, eq85_e1197_d_b19, eq85_e1197_d_b20, eq85_e1197_d_b21, eq85_e1197_d_b22, eq85_e1197_d_b23, eq85_e1197_d_b24, eq85_e1197_d_b25, eq85_e1197_d_b26, eq85_e1197_d_b27, eq85_e1197_d_b28, eq85_e1197_d_b29, eq85_e1197_d_b30, eq85_e1197_d_b31, eq85_e1197_d_b32, eq85_e1197_d_b33, eq85_e1197_d_b34, eq85_e1197_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq85_value),
            &eq85_node_derivatives,
            &eq85_branch_derivatives,
            multiplicity,
        );
        let (eq86_e1207, eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29, eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35,) = {
    if s.b[1054] {
        let eq86_e1200: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 70, s.v[168]);
        let eq86_e1200_d_n0: f64 = (s.dn[168][0] * ddt_scale);
        let eq86_e1200_d_n1: f64 = (s.dn[168][1] * ddt_scale);
        let eq86_e1200_d_n2: f64 = (s.dn[168][2] * ddt_scale);
        let eq86_e1200_d_n3: f64 = (s.dn[168][3] * ddt_scale);
        let eq86_e1200_d_n4: f64 = (s.dn[168][4] * ddt_scale);
        let eq86_e1200_d_n5: f64 = (s.dn[168][5] * ddt_scale);
        let eq86_e1200_d_n6: f64 = (s.dn[168][6] * ddt_scale);
        let eq86_e1200_d_n7: f64 = (s.dn[168][7] * ddt_scale);
        let eq86_e1200_d_n8: f64 = (s.dn[168][8] * ddt_scale);
        let eq86_e1200_d_n9: f64 = (s.dn[168][9] * ddt_scale);
        let eq86_e1200_d_n10: f64 = (s.dn[168][10] * ddt_scale);
        let eq86_e1200_d_n11: f64 = (s.dn[168][11] * ddt_scale);
        let eq86_e1200_d_n12: f64 = (s.dn[168][12] * ddt_scale);
        let eq86_e1200_d_n13: f64 = (s.dn[168][13] * ddt_scale);
        let eq86_e1200_d_n14: f64 = (s.dn[168][14] * ddt_scale);
        let eq86_e1200_d_n15: f64 = (s.dn[168][15] * ddt_scale);
        let eq86_e1200_d_n16: f64 = (s.dn[168][16] * ddt_scale);
        let eq86_e1200_d_n17: f64 = (s.dn[168][17] * ddt_scale);
        let eq86_e1200_d_n18: f64 = (s.dn[168][18] * ddt_scale);
        let eq86_e1200_d_n19: f64 = (s.dn[168][19] * ddt_scale);
        let eq86_e1200_d_n20: f64 = (s.dn[168][20] * ddt_scale);
        let eq86_e1200_d_n21: f64 = (s.dn[168][21] * ddt_scale);
        let eq86_e1200_d_n22: f64 = (s.dn[168][22] * ddt_scale);
        let eq86_e1200_d_n23: f64 = (s.dn[168][23] * ddt_scale);
        let eq86_e1200_d_n24: f64 = (s.dn[168][24] * ddt_scale);
        let eq86_e1200_d_n25: f64 = (s.dn[168][25] * ddt_scale);
        let eq86_e1200_d_n26: f64 = (s.dn[168][26] * ddt_scale);
        let eq86_e1200_d_n27: f64 = (s.dn[168][27] * ddt_scale);
        let eq86_e1200_d_n28: f64 = (s.dn[168][28] * ddt_scale);
        let eq86_e1200_d_n29: f64 = (s.dn[168][29] * ddt_scale);
        let eq86_e1200_d_b0: f64 = (s.db[168][0] * ddt_scale);
        let eq86_e1200_d_b1: f64 = (s.db[168][1] * ddt_scale);
        let eq86_e1200_d_b2: f64 = (s.db[168][2] * ddt_scale);
        let eq86_e1200_d_b3: f64 = (s.db[168][3] * ddt_scale);
        let eq86_e1200_d_b4: f64 = (s.db[168][4] * ddt_scale);
        let eq86_e1200_d_b5: f64 = (s.db[168][5] * ddt_scale);
        let eq86_e1200_d_b6: f64 = (s.db[168][6] * ddt_scale);
        let eq86_e1200_d_b7: f64 = (s.db[168][7] * ddt_scale);
        let eq86_e1200_d_b8: f64 = (s.db[168][8] * ddt_scale);
        let eq86_e1200_d_b9: f64 = (s.db[168][9] * ddt_scale);
        let eq86_e1200_d_b10: f64 = (s.db[168][10] * ddt_scale);
        let eq86_e1200_d_b11: f64 = (s.db[168][11] * ddt_scale);
        let eq86_e1200_d_b12: f64 = (s.db[168][12] * ddt_scale);
        let eq86_e1200_d_b13: f64 = (s.db[168][13] * ddt_scale);
        let eq86_e1200_d_b14: f64 = (s.db[168][14] * ddt_scale);
        let eq86_e1200_d_b15: f64 = (s.db[168][15] * ddt_scale);
        let eq86_e1200_d_b16: f64 = (s.db[168][16] * ddt_scale);
        let eq86_e1200_d_b17: f64 = (s.db[168][17] * ddt_scale);
        let eq86_e1200_d_b18: f64 = (s.db[168][18] * ddt_scale);
        let eq86_e1200_d_b19: f64 = (s.db[168][19] * ddt_scale);
        let eq86_e1200_d_b20: f64 = (s.db[168][20] * ddt_scale);
        let eq86_e1200_d_b21: f64 = (s.db[168][21] * ddt_scale);
        let eq86_e1200_d_b22: f64 = (s.db[168][22] * ddt_scale);
        let eq86_e1200_d_b23: f64 = (s.db[168][23] * ddt_scale);
        let eq86_e1200_d_b24: f64 = (s.db[168][24] * ddt_scale);
        let eq86_e1200_d_b25: f64 = (s.db[168][25] * ddt_scale);
        let eq86_e1200_d_b26: f64 = (s.db[168][26] * ddt_scale);
        let eq86_e1200_d_b27: f64 = (s.db[168][27] * ddt_scale);
        let eq86_e1200_d_b28: f64 = (s.db[168][28] * ddt_scale);
        let eq86_e1200_d_b29: f64 = (s.db[168][29] * ddt_scale);
        let eq86_e1200_d_b30: f64 = (s.db[168][30] * ddt_scale);
        let eq86_e1200_d_b31: f64 = (s.db[168][31] * ddt_scale);
        let eq86_e1200_d_b32: f64 = (s.db[168][32] * ddt_scale);
        let eq86_e1200_d_b33: f64 = (s.db[168][33] * ddt_scale);
        let eq86_e1200_d_b34: f64 = (s.db[168][34] * ddt_scale);
        let eq86_e1200_d_b35: f64 = (s.db[168][35] * ddt_scale);
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1203_d_n7: f64 = p.p355;
        let eq86_e1203_d_n9: f64 = (-p.p355);
        let eq86_e1204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 71, eq86_e1203);
        let eq86_e1204_d_n7: f64 = (eq86_e1203_d_n7 * ddt_scale);
        let eq86_e1204_d_n9: f64 = (eq86_e1203_d_n9 * ddt_scale);
        let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);
        let eq86_e1205_d_n7: f64 = (eq86_e1200_d_n7 + eq86_e1204_d_n7);
        let eq86_e1205_d_n9: f64 = (eq86_e1200_d_n9 + eq86_e1204_d_n9);
        (eq86_e1205, eq86_e1200_d_n0, eq86_e1200_d_n1, eq86_e1200_d_n2, eq86_e1200_d_n3, eq86_e1200_d_n4, eq86_e1200_d_n5, eq86_e1200_d_n6, eq86_e1205_d_n7, eq86_e1200_d_n8, eq86_e1205_d_n9, eq86_e1200_d_n10, eq86_e1200_d_n11, eq86_e1200_d_n12, eq86_e1200_d_n13, eq86_e1200_d_n14, eq86_e1200_d_n15, eq86_e1200_d_n16, eq86_e1200_d_n17, eq86_e1200_d_n18, eq86_e1200_d_n19, eq86_e1200_d_n20, eq86_e1200_d_n21, eq86_e1200_d_n22, eq86_e1200_d_n23, eq86_e1200_d_n24, eq86_e1200_d_n25, eq86_e1200_d_n26, eq86_e1200_d_n27, eq86_e1200_d_n28, eq86_e1200_d_n29, eq86_e1200_d_b0, eq86_e1200_d_b1, eq86_e1200_d_b2, eq86_e1200_d_b3, eq86_e1200_d_b4, eq86_e1200_d_b5, eq86_e1200_d_b6, eq86_e1200_d_b7, eq86_e1200_d_b8, eq86_e1200_d_b9, eq86_e1200_d_b10, eq86_e1200_d_b11, eq86_e1200_d_b12, eq86_e1200_d_b13, eq86_e1200_d_b14, eq86_e1200_d_b15, eq86_e1200_d_b16, eq86_e1200_d_b17, eq86_e1200_d_b18, eq86_e1200_d_b19, eq86_e1200_d_b20, eq86_e1200_d_b21, eq86_e1200_d_b22, eq86_e1200_d_b23, eq86_e1200_d_b24, eq86_e1200_d_b25, eq86_e1200_d_b26, eq86_e1200_d_b27, eq86_e1200_d_b28, eq86_e1200_d_b29, eq86_e1200_d_b30, eq86_e1200_d_b31, eq86_e1200_d_b32, eq86_e1200_d_b33, eq86_e1200_d_b34, eq86_e1200_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        let eq86_node_derivatives: [f64; 30] = [eq86_e1207_d_n0, eq86_e1207_d_n1, eq86_e1207_d_n2, eq86_e1207_d_n3, eq86_e1207_d_n4, eq86_e1207_d_n5, eq86_e1207_d_n6, eq86_e1207_d_n7, eq86_e1207_d_n8, eq86_e1207_d_n9, eq86_e1207_d_n10, eq86_e1207_d_n11, eq86_e1207_d_n12, eq86_e1207_d_n13, eq86_e1207_d_n14, eq86_e1207_d_n15, eq86_e1207_d_n16, eq86_e1207_d_n17, eq86_e1207_d_n18, eq86_e1207_d_n19, eq86_e1207_d_n20, eq86_e1207_d_n21, eq86_e1207_d_n22, eq86_e1207_d_n23, eq86_e1207_d_n24, eq86_e1207_d_n25, eq86_e1207_d_n26, eq86_e1207_d_n27, eq86_e1207_d_n28, eq86_e1207_d_n29];
        let eq86_branch_derivatives: [f64; 36] = [eq86_e1207_d_b0, eq86_e1207_d_b1, eq86_e1207_d_b2, eq86_e1207_d_b3, eq86_e1207_d_b4, eq86_e1207_d_b5, eq86_e1207_d_b6, eq86_e1207_d_b7, eq86_e1207_d_b8, eq86_e1207_d_b9, eq86_e1207_d_b10, eq86_e1207_d_b11, eq86_e1207_d_b12, eq86_e1207_d_b13, eq86_e1207_d_b14, eq86_e1207_d_b15, eq86_e1207_d_b16, eq86_e1207_d_b17, eq86_e1207_d_b18, eq86_e1207_d_b19, eq86_e1207_d_b20, eq86_e1207_d_b21, eq86_e1207_d_b22, eq86_e1207_d_b23, eq86_e1207_d_b24, eq86_e1207_d_b25, eq86_e1207_d_b26, eq86_e1207_d_b27, eq86_e1207_d_b28, eq86_e1207_d_b29, eq86_e1207_d_b30, eq86_e1207_d_b31, eq86_e1207_d_b32, eq86_e1207_d_b33, eq86_e1207_d_b34, eq86_e1207_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq86_value),
            &eq86_node_derivatives,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1217, eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29, eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35,) = {
    if s.b[1054] {
        let eq87_e1210: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 72, s.v[169]);
        let eq87_e1210_d_n0: f64 = (s.dn[169][0] * ddt_scale);
        let eq87_e1210_d_n1: f64 = (s.dn[169][1] * ddt_scale);
        let eq87_e1210_d_n2: f64 = (s.dn[169][2] * ddt_scale);
        let eq87_e1210_d_n3: f64 = (s.dn[169][3] * ddt_scale);
        let eq87_e1210_d_n4: f64 = (s.dn[169][4] * ddt_scale);
        let eq87_e1210_d_n5: f64 = (s.dn[169][5] * ddt_scale);
        let eq87_e1210_d_n6: f64 = (s.dn[169][6] * ddt_scale);
        let eq87_e1210_d_n7: f64 = (s.dn[169][7] * ddt_scale);
        let eq87_e1210_d_n8: f64 = (s.dn[169][8] * ddt_scale);
        let eq87_e1210_d_n9: f64 = (s.dn[169][9] * ddt_scale);
        let eq87_e1210_d_n10: f64 = (s.dn[169][10] * ddt_scale);
        let eq87_e1210_d_n11: f64 = (s.dn[169][11] * ddt_scale);
        let eq87_e1210_d_n12: f64 = (s.dn[169][12] * ddt_scale);
        let eq87_e1210_d_n13: f64 = (s.dn[169][13] * ddt_scale);
        let eq87_e1210_d_n14: f64 = (s.dn[169][14] * ddt_scale);
        let eq87_e1210_d_n15: f64 = (s.dn[169][15] * ddt_scale);
        let eq87_e1210_d_n16: f64 = (s.dn[169][16] * ddt_scale);
        let eq87_e1210_d_n17: f64 = (s.dn[169][17] * ddt_scale);
        let eq87_e1210_d_n18: f64 = (s.dn[169][18] * ddt_scale);
        let eq87_e1210_d_n19: f64 = (s.dn[169][19] * ddt_scale);
        let eq87_e1210_d_n20: f64 = (s.dn[169][20] * ddt_scale);
        let eq87_e1210_d_n21: f64 = (s.dn[169][21] * ddt_scale);
        let eq87_e1210_d_n22: f64 = (s.dn[169][22] * ddt_scale);
        let eq87_e1210_d_n23: f64 = (s.dn[169][23] * ddt_scale);
        let eq87_e1210_d_n24: f64 = (s.dn[169][24] * ddt_scale);
        let eq87_e1210_d_n25: f64 = (s.dn[169][25] * ddt_scale);
        let eq87_e1210_d_n26: f64 = (s.dn[169][26] * ddt_scale);
        let eq87_e1210_d_n27: f64 = (s.dn[169][27] * ddt_scale);
        let eq87_e1210_d_n28: f64 = (s.dn[169][28] * ddt_scale);
        let eq87_e1210_d_n29: f64 = (s.dn[169][29] * ddt_scale);
        let eq87_e1210_d_b0: f64 = (s.db[169][0] * ddt_scale);
        let eq87_e1210_d_b1: f64 = (s.db[169][1] * ddt_scale);
        let eq87_e1210_d_b2: f64 = (s.db[169][2] * ddt_scale);
        let eq87_e1210_d_b3: f64 = (s.db[169][3] * ddt_scale);
        let eq87_e1210_d_b4: f64 = (s.db[169][4] * ddt_scale);
        let eq87_e1210_d_b5: f64 = (s.db[169][5] * ddt_scale);
        let eq87_e1210_d_b6: f64 = (s.db[169][6] * ddt_scale);
        let eq87_e1210_d_b7: f64 = (s.db[169][7] * ddt_scale);
        let eq87_e1210_d_b8: f64 = (s.db[169][8] * ddt_scale);
        let eq87_e1210_d_b9: f64 = (s.db[169][9] * ddt_scale);
        let eq87_e1210_d_b10: f64 = (s.db[169][10] * ddt_scale);
        let eq87_e1210_d_b11: f64 = (s.db[169][11] * ddt_scale);
        let eq87_e1210_d_b12: f64 = (s.db[169][12] * ddt_scale);
        let eq87_e1210_d_b13: f64 = (s.db[169][13] * ddt_scale);
        let eq87_e1210_d_b14: f64 = (s.db[169][14] * ddt_scale);
        let eq87_e1210_d_b15: f64 = (s.db[169][15] * ddt_scale);
        let eq87_e1210_d_b16: f64 = (s.db[169][16] * ddt_scale);
        let eq87_e1210_d_b17: f64 = (s.db[169][17] * ddt_scale);
        let eq87_e1210_d_b18: f64 = (s.db[169][18] * ddt_scale);
        let eq87_e1210_d_b19: f64 = (s.db[169][19] * ddt_scale);
        let eq87_e1210_d_b20: f64 = (s.db[169][20] * ddt_scale);
        let eq87_e1210_d_b21: f64 = (s.db[169][21] * ddt_scale);
        let eq87_e1210_d_b22: f64 = (s.db[169][22] * ddt_scale);
        let eq87_e1210_d_b23: f64 = (s.db[169][23] * ddt_scale);
        let eq87_e1210_d_b24: f64 = (s.db[169][24] * ddt_scale);
        let eq87_e1210_d_b25: f64 = (s.db[169][25] * ddt_scale);
        let eq87_e1210_d_b26: f64 = (s.db[169][26] * ddt_scale);
        let eq87_e1210_d_b27: f64 = (s.db[169][27] * ddt_scale);
        let eq87_e1210_d_b28: f64 = (s.db[169][28] * ddt_scale);
        let eq87_e1210_d_b29: f64 = (s.db[169][29] * ddt_scale);
        let eq87_e1210_d_b30: f64 = (s.db[169][30] * ddt_scale);
        let eq87_e1210_d_b31: f64 = (s.db[169][31] * ddt_scale);
        let eq87_e1210_d_b32: f64 = (s.db[169][32] * ddt_scale);
        let eq87_e1210_d_b33: f64 = (s.db[169][33] * ddt_scale);
        let eq87_e1210_d_b34: f64 = (s.db[169][34] * ddt_scale);
        let eq87_e1210_d_b35: f64 = (s.db[169][35] * ddt_scale);
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1213_d_n2: f64 = p.p355;
        let eq87_e1213_d_n10: f64 = (-p.p355);
        let eq87_e1214: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 73, eq87_e1213);
        let eq87_e1214_d_n2: f64 = (eq87_e1213_d_n2 * ddt_scale);
        let eq87_e1214_d_n10: f64 = (eq87_e1213_d_n10 * ddt_scale);
        let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);
        let eq87_e1215_d_n2: f64 = (eq87_e1210_d_n2 + eq87_e1214_d_n2);
        let eq87_e1215_d_n10: f64 = (eq87_e1210_d_n10 + eq87_e1214_d_n10);
        (eq87_e1215, eq87_e1210_d_n0, eq87_e1210_d_n1, eq87_e1215_d_n2, eq87_e1210_d_n3, eq87_e1210_d_n4, eq87_e1210_d_n5, eq87_e1210_d_n6, eq87_e1210_d_n7, eq87_e1210_d_n8, eq87_e1210_d_n9, eq87_e1215_d_n10, eq87_e1210_d_n11, eq87_e1210_d_n12, eq87_e1210_d_n13, eq87_e1210_d_n14, eq87_e1210_d_n15, eq87_e1210_d_n16, eq87_e1210_d_n17, eq87_e1210_d_n18, eq87_e1210_d_n19, eq87_e1210_d_n20, eq87_e1210_d_n21, eq87_e1210_d_n22, eq87_e1210_d_n23, eq87_e1210_d_n24, eq87_e1210_d_n25, eq87_e1210_d_n26, eq87_e1210_d_n27, eq87_e1210_d_n28, eq87_e1210_d_n29, eq87_e1210_d_b0, eq87_e1210_d_b1, eq87_e1210_d_b2, eq87_e1210_d_b3, eq87_e1210_d_b4, eq87_e1210_d_b5, eq87_e1210_d_b6, eq87_e1210_d_b7, eq87_e1210_d_b8, eq87_e1210_d_b9, eq87_e1210_d_b10, eq87_e1210_d_b11, eq87_e1210_d_b12, eq87_e1210_d_b13, eq87_e1210_d_b14, eq87_e1210_d_b15, eq87_e1210_d_b16, eq87_e1210_d_b17, eq87_e1210_d_b18, eq87_e1210_d_b19, eq87_e1210_d_b20, eq87_e1210_d_b21, eq87_e1210_d_b22, eq87_e1210_d_b23, eq87_e1210_d_b24, eq87_e1210_d_b25, eq87_e1210_d_b26, eq87_e1210_d_b27, eq87_e1210_d_b28, eq87_e1210_d_b29, eq87_e1210_d_b30, eq87_e1210_d_b31, eq87_e1210_d_b32, eq87_e1210_d_b33, eq87_e1210_d_b34, eq87_e1210_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        let eq87_node_derivatives: [f64; 30] = [eq87_e1217_d_n0, eq87_e1217_d_n1, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n5, eq87_e1217_d_n6, eq87_e1217_d_n7, eq87_e1217_d_n8, eq87_e1217_d_n9, eq87_e1217_d_n10, eq87_e1217_d_n11, eq87_e1217_d_n12, eq87_e1217_d_n13, eq87_e1217_d_n14, eq87_e1217_d_n15, eq87_e1217_d_n16, eq87_e1217_d_n17, eq87_e1217_d_n18, eq87_e1217_d_n19, eq87_e1217_d_n20, eq87_e1217_d_n21, eq87_e1217_d_n22, eq87_e1217_d_n23, eq87_e1217_d_n24, eq87_e1217_d_n25, eq87_e1217_d_n26, eq87_e1217_d_n27, eq87_e1217_d_n28, eq87_e1217_d_n29];
        let eq87_branch_derivatives: [f64; 36] = [eq87_e1217_d_b0, eq87_e1217_d_b1, eq87_e1217_d_b2, eq87_e1217_d_b3, eq87_e1217_d_b4, eq87_e1217_d_b5, eq87_e1217_d_b6, eq87_e1217_d_b7, eq87_e1217_d_b8, eq87_e1217_d_b9, eq87_e1217_d_b10, eq87_e1217_d_b11, eq87_e1217_d_b12, eq87_e1217_d_b13, eq87_e1217_d_b14, eq87_e1217_d_b15, eq87_e1217_d_b16, eq87_e1217_d_b17, eq87_e1217_d_b18, eq87_e1217_d_b19, eq87_e1217_d_b20, eq87_e1217_d_b21, eq87_e1217_d_b22, eq87_e1217_d_b23, eq87_e1217_d_b24, eq87_e1217_d_b25, eq87_e1217_d_b26, eq87_e1217_d_b27, eq87_e1217_d_b28, eq87_e1217_d_b29, eq87_e1217_d_b30, eq87_e1217_d_b31, eq87_e1217_d_b32, eq87_e1217_d_b33, eq87_e1217_d_b34, eq87_e1217_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(10),
            multiplicity * (eq87_value),
            &eq87_node_derivatives,
            &eq87_branch_derivatives,
            multiplicity,
        );
        let (eq88_e1221,) = {
    if s.b[1054] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1221;
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (eq88_value),
        );
        let (eq89_e1231, eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29, eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35,) = {
    if s.b[1054] {
        let eq89_e1224: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 74, s.v[171]);
        let eq89_e1224_d_n0: f64 = (s.dn[171][0] * ddt_scale);
        let eq89_e1224_d_n1: f64 = (s.dn[171][1] * ddt_scale);
        let eq89_e1224_d_n2: f64 = (s.dn[171][2] * ddt_scale);
        let eq89_e1224_d_n3: f64 = (s.dn[171][3] * ddt_scale);
        let eq89_e1224_d_n4: f64 = (s.dn[171][4] * ddt_scale);
        let eq89_e1224_d_n5: f64 = (s.dn[171][5] * ddt_scale);
        let eq89_e1224_d_n6: f64 = (s.dn[171][6] * ddt_scale);
        let eq89_e1224_d_n7: f64 = (s.dn[171][7] * ddt_scale);
        let eq89_e1224_d_n8: f64 = (s.dn[171][8] * ddt_scale);
        let eq89_e1224_d_n9: f64 = (s.dn[171][9] * ddt_scale);
        let eq89_e1224_d_n10: f64 = (s.dn[171][10] * ddt_scale);
        let eq89_e1224_d_n11: f64 = (s.dn[171][11] * ddt_scale);
        let eq89_e1224_d_n12: f64 = (s.dn[171][12] * ddt_scale);
        let eq89_e1224_d_n13: f64 = (s.dn[171][13] * ddt_scale);
        let eq89_e1224_d_n14: f64 = (s.dn[171][14] * ddt_scale);
        let eq89_e1224_d_n15: f64 = (s.dn[171][15] * ddt_scale);
        let eq89_e1224_d_n16: f64 = (s.dn[171][16] * ddt_scale);
        let eq89_e1224_d_n17: f64 = (s.dn[171][17] * ddt_scale);
        let eq89_e1224_d_n18: f64 = (s.dn[171][18] * ddt_scale);
        let eq89_e1224_d_n19: f64 = (s.dn[171][19] * ddt_scale);
        let eq89_e1224_d_n20: f64 = (s.dn[171][20] * ddt_scale);
        let eq89_e1224_d_n21: f64 = (s.dn[171][21] * ddt_scale);
        let eq89_e1224_d_n22: f64 = (s.dn[171][22] * ddt_scale);
        let eq89_e1224_d_n23: f64 = (s.dn[171][23] * ddt_scale);
        let eq89_e1224_d_n24: f64 = (s.dn[171][24] * ddt_scale);
        let eq89_e1224_d_n25: f64 = (s.dn[171][25] * ddt_scale);
        let eq89_e1224_d_n26: f64 = (s.dn[171][26] * ddt_scale);
        let eq89_e1224_d_n27: f64 = (s.dn[171][27] * ddt_scale);
        let eq89_e1224_d_n28: f64 = (s.dn[171][28] * ddt_scale);
        let eq89_e1224_d_n29: f64 = (s.dn[171][29] * ddt_scale);
        let eq89_e1224_d_b0: f64 = (s.db[171][0] * ddt_scale);
        let eq89_e1224_d_b1: f64 = (s.db[171][1] * ddt_scale);
        let eq89_e1224_d_b2: f64 = (s.db[171][2] * ddt_scale);
        let eq89_e1224_d_b3: f64 = (s.db[171][3] * ddt_scale);
        let eq89_e1224_d_b4: f64 = (s.db[171][4] * ddt_scale);
        let eq89_e1224_d_b5: f64 = (s.db[171][5] * ddt_scale);
        let eq89_e1224_d_b6: f64 = (s.db[171][6] * ddt_scale);
        let eq89_e1224_d_b7: f64 = (s.db[171][7] * ddt_scale);
        let eq89_e1224_d_b8: f64 = (s.db[171][8] * ddt_scale);
        let eq89_e1224_d_b9: f64 = (s.db[171][9] * ddt_scale);
        let eq89_e1224_d_b10: f64 = (s.db[171][10] * ddt_scale);
        let eq89_e1224_d_b11: f64 = (s.db[171][11] * ddt_scale);
        let eq89_e1224_d_b12: f64 = (s.db[171][12] * ddt_scale);
        let eq89_e1224_d_b13: f64 = (s.db[171][13] * ddt_scale);
        let eq89_e1224_d_b14: f64 = (s.db[171][14] * ddt_scale);
        let eq89_e1224_d_b15: f64 = (s.db[171][15] * ddt_scale);
        let eq89_e1224_d_b16: f64 = (s.db[171][16] * ddt_scale);
        let eq89_e1224_d_b17: f64 = (s.db[171][17] * ddt_scale);
        let eq89_e1224_d_b18: f64 = (s.db[171][18] * ddt_scale);
        let eq89_e1224_d_b19: f64 = (s.db[171][19] * ddt_scale);
        let eq89_e1224_d_b20: f64 = (s.db[171][20] * ddt_scale);
        let eq89_e1224_d_b21: f64 = (s.db[171][21] * ddt_scale);
        let eq89_e1224_d_b22: f64 = (s.db[171][22] * ddt_scale);
        let eq89_e1224_d_b23: f64 = (s.db[171][23] * ddt_scale);
        let eq89_e1224_d_b24: f64 = (s.db[171][24] * ddt_scale);
        let eq89_e1224_d_b25: f64 = (s.db[171][25] * ddt_scale);
        let eq89_e1224_d_b26: f64 = (s.db[171][26] * ddt_scale);
        let eq89_e1224_d_b27: f64 = (s.db[171][27] * ddt_scale);
        let eq89_e1224_d_b28: f64 = (s.db[171][28] * ddt_scale);
        let eq89_e1224_d_b29: f64 = (s.db[171][29] * ddt_scale);
        let eq89_e1224_d_b30: f64 = (s.db[171][30] * ddt_scale);
        let eq89_e1224_d_b31: f64 = (s.db[171][31] * ddt_scale);
        let eq89_e1224_d_b32: f64 = (s.db[171][32] * ddt_scale);
        let eq89_e1224_d_b33: f64 = (s.db[171][33] * ddt_scale);
        let eq89_e1224_d_b34: f64 = (s.db[171][34] * ddt_scale);
        let eq89_e1224_d_b35: f64 = (s.db[171][35] * ddt_scale);
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1227_d_n7: f64 = p.p355;
        let eq89_e1227_d_n9: f64 = (-p.p355);
        let eq89_e1228: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 75, eq89_e1227);
        let eq89_e1228_d_n7: f64 = (eq89_e1227_d_n7 * ddt_scale);
        let eq89_e1228_d_n9: f64 = (eq89_e1227_d_n9 * ddt_scale);
        let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);
        let eq89_e1229_d_n7: f64 = (eq89_e1224_d_n7 + eq89_e1228_d_n7);
        let eq89_e1229_d_n9: f64 = (eq89_e1224_d_n9 + eq89_e1228_d_n9);
        (eq89_e1229, eq89_e1224_d_n0, eq89_e1224_d_n1, eq89_e1224_d_n2, eq89_e1224_d_n3, eq89_e1224_d_n4, eq89_e1224_d_n5, eq89_e1224_d_n6, eq89_e1229_d_n7, eq89_e1224_d_n8, eq89_e1229_d_n9, eq89_e1224_d_n10, eq89_e1224_d_n11, eq89_e1224_d_n12, eq89_e1224_d_n13, eq89_e1224_d_n14, eq89_e1224_d_n15, eq89_e1224_d_n16, eq89_e1224_d_n17, eq89_e1224_d_n18, eq89_e1224_d_n19, eq89_e1224_d_n20, eq89_e1224_d_n21, eq89_e1224_d_n22, eq89_e1224_d_n23, eq89_e1224_d_n24, eq89_e1224_d_n25, eq89_e1224_d_n26, eq89_e1224_d_n27, eq89_e1224_d_n28, eq89_e1224_d_n29, eq89_e1224_d_b0, eq89_e1224_d_b1, eq89_e1224_d_b2, eq89_e1224_d_b3, eq89_e1224_d_b4, eq89_e1224_d_b5, eq89_e1224_d_b6, eq89_e1224_d_b7, eq89_e1224_d_b8, eq89_e1224_d_b9, eq89_e1224_d_b10, eq89_e1224_d_b11, eq89_e1224_d_b12, eq89_e1224_d_b13, eq89_e1224_d_b14, eq89_e1224_d_b15, eq89_e1224_d_b16, eq89_e1224_d_b17, eq89_e1224_d_b18, eq89_e1224_d_b19, eq89_e1224_d_b20, eq89_e1224_d_b21, eq89_e1224_d_b22, eq89_e1224_d_b23, eq89_e1224_d_b24, eq89_e1224_d_b25, eq89_e1224_d_b26, eq89_e1224_d_b27, eq89_e1224_d_b28, eq89_e1224_d_b29, eq89_e1224_d_b30, eq89_e1224_d_b31, eq89_e1224_d_b32, eq89_e1224_d_b33, eq89_e1224_d_b34, eq89_e1224_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        let eq89_node_derivatives: [f64; 30] = [eq89_e1231_d_n0, eq89_e1231_d_n1, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n5, eq89_e1231_d_n6, eq89_e1231_d_n7, eq89_e1231_d_n8, eq89_e1231_d_n9, eq89_e1231_d_n10, eq89_e1231_d_n11, eq89_e1231_d_n12, eq89_e1231_d_n13, eq89_e1231_d_n14, eq89_e1231_d_n15, eq89_e1231_d_n16, eq89_e1231_d_n17, eq89_e1231_d_n18, eq89_e1231_d_n19, eq89_e1231_d_n20, eq89_e1231_d_n21, eq89_e1231_d_n22, eq89_e1231_d_n23, eq89_e1231_d_n24, eq89_e1231_d_n25, eq89_e1231_d_n26, eq89_e1231_d_n27, eq89_e1231_d_n28, eq89_e1231_d_n29];
        let eq89_branch_derivatives: [f64; 36] = [eq89_e1231_d_b0, eq89_e1231_d_b1, eq89_e1231_d_b2, eq89_e1231_d_b3, eq89_e1231_d_b4, eq89_e1231_d_b5, eq89_e1231_d_b6, eq89_e1231_d_b7, eq89_e1231_d_b8, eq89_e1231_d_b9, eq89_e1231_d_b10, eq89_e1231_d_b11, eq89_e1231_d_b12, eq89_e1231_d_b13, eq89_e1231_d_b14, eq89_e1231_d_b15, eq89_e1231_d_b16, eq89_e1231_d_b17, eq89_e1231_d_b18, eq89_e1231_d_b19, eq89_e1231_d_b20, eq89_e1231_d_b21, eq89_e1231_d_b22, eq89_e1231_d_b23, eq89_e1231_d_b24, eq89_e1231_d_b25, eq89_e1231_d_b26, eq89_e1231_d_b27, eq89_e1231_d_b28, eq89_e1231_d_b29, eq89_e1231_d_b30, eq89_e1231_d_b31, eq89_e1231_d_b32, eq89_e1231_d_b33, eq89_e1231_d_b34, eq89_e1231_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq89_value),
            &eq89_node_derivatives,
            &eq89_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
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
        let (eq90_e1242, eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29, eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35,) = {
    if (!s.b[1054]) {
        let eq90_e1235: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 76, s.v[167]);
        let eq90_e1235_d_n0: f64 = (s.dn[167][0] * ddt_scale);
        let eq90_e1235_d_n1: f64 = (s.dn[167][1] * ddt_scale);
        let eq90_e1235_d_n2: f64 = (s.dn[167][2] * ddt_scale);
        let eq90_e1235_d_n3: f64 = (s.dn[167][3] * ddt_scale);
        let eq90_e1235_d_n4: f64 = (s.dn[167][4] * ddt_scale);
        let eq90_e1235_d_n5: f64 = (s.dn[167][5] * ddt_scale);
        let eq90_e1235_d_n6: f64 = (s.dn[167][6] * ddt_scale);
        let eq90_e1235_d_n7: f64 = (s.dn[167][7] * ddt_scale);
        let eq90_e1235_d_n8: f64 = (s.dn[167][8] * ddt_scale);
        let eq90_e1235_d_n9: f64 = (s.dn[167][9] * ddt_scale);
        let eq90_e1235_d_n10: f64 = (s.dn[167][10] * ddt_scale);
        let eq90_e1235_d_n11: f64 = (s.dn[167][11] * ddt_scale);
        let eq90_e1235_d_n12: f64 = (s.dn[167][12] * ddt_scale);
        let eq90_e1235_d_n13: f64 = (s.dn[167][13] * ddt_scale);
        let eq90_e1235_d_n14: f64 = (s.dn[167][14] * ddt_scale);
        let eq90_e1235_d_n15: f64 = (s.dn[167][15] * ddt_scale);
        let eq90_e1235_d_n16: f64 = (s.dn[167][16] * ddt_scale);
        let eq90_e1235_d_n17: f64 = (s.dn[167][17] * ddt_scale);
        let eq90_e1235_d_n18: f64 = (s.dn[167][18] * ddt_scale);
        let eq90_e1235_d_n19: f64 = (s.dn[167][19] * ddt_scale);
        let eq90_e1235_d_n20: f64 = (s.dn[167][20] * ddt_scale);
        let eq90_e1235_d_n21: f64 = (s.dn[167][21] * ddt_scale);
        let eq90_e1235_d_n22: f64 = (s.dn[167][22] * ddt_scale);
        let eq90_e1235_d_n23: f64 = (s.dn[167][23] * ddt_scale);
        let eq90_e1235_d_n24: f64 = (s.dn[167][24] * ddt_scale);
        let eq90_e1235_d_n25: f64 = (s.dn[167][25] * ddt_scale);
        let eq90_e1235_d_n26: f64 = (s.dn[167][26] * ddt_scale);
        let eq90_e1235_d_n27: f64 = (s.dn[167][27] * ddt_scale);
        let eq90_e1235_d_n28: f64 = (s.dn[167][28] * ddt_scale);
        let eq90_e1235_d_n29: f64 = (s.dn[167][29] * ddt_scale);
        let eq90_e1235_d_b0: f64 = (s.db[167][0] * ddt_scale);
        let eq90_e1235_d_b1: f64 = (s.db[167][1] * ddt_scale);
        let eq90_e1235_d_b2: f64 = (s.db[167][2] * ddt_scale);
        let eq90_e1235_d_b3: f64 = (s.db[167][3] * ddt_scale);
        let eq90_e1235_d_b4: f64 = (s.db[167][4] * ddt_scale);
        let eq90_e1235_d_b5: f64 = (s.db[167][5] * ddt_scale);
        let eq90_e1235_d_b6: f64 = (s.db[167][6] * ddt_scale);
        let eq90_e1235_d_b7: f64 = (s.db[167][7] * ddt_scale);
        let eq90_e1235_d_b8: f64 = (s.db[167][8] * ddt_scale);
        let eq90_e1235_d_b9: f64 = (s.db[167][9] * ddt_scale);
        let eq90_e1235_d_b10: f64 = (s.db[167][10] * ddt_scale);
        let eq90_e1235_d_b11: f64 = (s.db[167][11] * ddt_scale);
        let eq90_e1235_d_b12: f64 = (s.db[167][12] * ddt_scale);
        let eq90_e1235_d_b13: f64 = (s.db[167][13] * ddt_scale);
        let eq90_e1235_d_b14: f64 = (s.db[167][14] * ddt_scale);
        let eq90_e1235_d_b15: f64 = (s.db[167][15] * ddt_scale);
        let eq90_e1235_d_b16: f64 = (s.db[167][16] * ddt_scale);
        let eq90_e1235_d_b17: f64 = (s.db[167][17] * ddt_scale);
        let eq90_e1235_d_b18: f64 = (s.db[167][18] * ddt_scale);
        let eq90_e1235_d_b19: f64 = (s.db[167][19] * ddt_scale);
        let eq90_e1235_d_b20: f64 = (s.db[167][20] * ddt_scale);
        let eq90_e1235_d_b21: f64 = (s.db[167][21] * ddt_scale);
        let eq90_e1235_d_b22: f64 = (s.db[167][22] * ddt_scale);
        let eq90_e1235_d_b23: f64 = (s.db[167][23] * ddt_scale);
        let eq90_e1235_d_b24: f64 = (s.db[167][24] * ddt_scale);
        let eq90_e1235_d_b25: f64 = (s.db[167][25] * ddt_scale);
        let eq90_e1235_d_b26: f64 = (s.db[167][26] * ddt_scale);
        let eq90_e1235_d_b27: f64 = (s.db[167][27] * ddt_scale);
        let eq90_e1235_d_b28: f64 = (s.db[167][28] * ddt_scale);
        let eq90_e1235_d_b29: f64 = (s.db[167][29] * ddt_scale);
        let eq90_e1235_d_b30: f64 = (s.db[167][30] * ddt_scale);
        let eq90_e1235_d_b31: f64 = (s.db[167][31] * ddt_scale);
        let eq90_e1235_d_b32: f64 = (s.db[167][32] * ddt_scale);
        let eq90_e1235_d_b33: f64 = (s.db[167][33] * ddt_scale);
        let eq90_e1235_d_b34: f64 = (s.db[167][34] * ddt_scale);
        let eq90_e1235_d_b35: f64 = (s.db[167][35] * ddt_scale);
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1238_d_n2: f64 = p.p355;
        let eq90_e1238_d_n10: f64 = (-p.p355);
        let eq90_e1239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 77, eq90_e1238);
        let eq90_e1239_d_n2: f64 = (eq90_e1238_d_n2 * ddt_scale);
        let eq90_e1239_d_n10: f64 = (eq90_e1238_d_n10 * ddt_scale);
        let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);
        let eq90_e1240_d_n2: f64 = (eq90_e1235_d_n2 + eq90_e1239_d_n2);
        let eq90_e1240_d_n10: f64 = (eq90_e1235_d_n10 + eq90_e1239_d_n10);
        (eq90_e1240, eq90_e1235_d_n0, eq90_e1235_d_n1, eq90_e1240_d_n2, eq90_e1235_d_n3, eq90_e1235_d_n4, eq90_e1235_d_n5, eq90_e1235_d_n6, eq90_e1235_d_n7, eq90_e1235_d_n8, eq90_e1235_d_n9, eq90_e1240_d_n10, eq90_e1235_d_n11, eq90_e1235_d_n12, eq90_e1235_d_n13, eq90_e1235_d_n14, eq90_e1235_d_n15, eq90_e1235_d_n16, eq90_e1235_d_n17, eq90_e1235_d_n18, eq90_e1235_d_n19, eq90_e1235_d_n20, eq90_e1235_d_n21, eq90_e1235_d_n22, eq90_e1235_d_n23, eq90_e1235_d_n24, eq90_e1235_d_n25, eq90_e1235_d_n26, eq90_e1235_d_n27, eq90_e1235_d_n28, eq90_e1235_d_n29, eq90_e1235_d_b0, eq90_e1235_d_b1, eq90_e1235_d_b2, eq90_e1235_d_b3, eq90_e1235_d_b4, eq90_e1235_d_b5, eq90_e1235_d_b6, eq90_e1235_d_b7, eq90_e1235_d_b8, eq90_e1235_d_b9, eq90_e1235_d_b10, eq90_e1235_d_b11, eq90_e1235_d_b12, eq90_e1235_d_b13, eq90_e1235_d_b14, eq90_e1235_d_b15, eq90_e1235_d_b16, eq90_e1235_d_b17, eq90_e1235_d_b18, eq90_e1235_d_b19, eq90_e1235_d_b20, eq90_e1235_d_b21, eq90_e1235_d_b22, eq90_e1235_d_b23, eq90_e1235_d_b24, eq90_e1235_d_b25, eq90_e1235_d_b26, eq90_e1235_d_b27, eq90_e1235_d_b28, eq90_e1235_d_b29, eq90_e1235_d_b30, eq90_e1235_d_b31, eq90_e1235_d_b32, eq90_e1235_d_b33, eq90_e1235_d_b34, eq90_e1235_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        let eq90_node_derivatives: [f64; 30] = [eq90_e1242_d_n0, eq90_e1242_d_n1, eq90_e1242_d_n2, eq90_e1242_d_n3, eq90_e1242_d_n4, eq90_e1242_d_n5, eq90_e1242_d_n6, eq90_e1242_d_n7, eq90_e1242_d_n8, eq90_e1242_d_n9, eq90_e1242_d_n10, eq90_e1242_d_n11, eq90_e1242_d_n12, eq90_e1242_d_n13, eq90_e1242_d_n14, eq90_e1242_d_n15, eq90_e1242_d_n16, eq90_e1242_d_n17, eq90_e1242_d_n18, eq90_e1242_d_n19, eq90_e1242_d_n20, eq90_e1242_d_n21, eq90_e1242_d_n22, eq90_e1242_d_n23, eq90_e1242_d_n24, eq90_e1242_d_n25, eq90_e1242_d_n26, eq90_e1242_d_n27, eq90_e1242_d_n28, eq90_e1242_d_n29];
        let eq90_branch_derivatives: [f64; 36] = [eq90_e1242_d_b0, eq90_e1242_d_b1, eq90_e1242_d_b2, eq90_e1242_d_b3, eq90_e1242_d_b4, eq90_e1242_d_b5, eq90_e1242_d_b6, eq90_e1242_d_b7, eq90_e1242_d_b8, eq90_e1242_d_b9, eq90_e1242_d_b10, eq90_e1242_d_b11, eq90_e1242_d_b12, eq90_e1242_d_b13, eq90_e1242_d_b14, eq90_e1242_d_b15, eq90_e1242_d_b16, eq90_e1242_d_b17, eq90_e1242_d_b18, eq90_e1242_d_b19, eq90_e1242_d_b20, eq90_e1242_d_b21, eq90_e1242_d_b22, eq90_e1242_d_b23, eq90_e1242_d_b24, eq90_e1242_d_b25, eq90_e1242_d_b26, eq90_e1242_d_b27, eq90_e1242_d_b28, eq90_e1242_d_b29, eq90_e1242_d_b30, eq90_e1242_d_b31, eq90_e1242_d_b32, eq90_e1242_d_b33, eq90_e1242_d_b34, eq90_e1242_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(10),
            multiplicity * (eq90_value),
            &eq90_node_derivatives,
            &eq90_branch_derivatives,
            multiplicity,
        );
        let (eq91_e1253, eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29, eq91_e1253_d_b0, eq91_e1253_d_b1, eq91_e1253_d_b2, eq91_e1253_d_b3, eq91_e1253_d_b4, eq91_e1253_d_b5, eq91_e1253_d_b6, eq91_e1253_d_b7, eq91_e1253_d_b8, eq91_e1253_d_b9, eq91_e1253_d_b10, eq91_e1253_d_b11, eq91_e1253_d_b12, eq91_e1253_d_b13, eq91_e1253_d_b14, eq91_e1253_d_b15, eq91_e1253_d_b16, eq91_e1253_d_b17, eq91_e1253_d_b18, eq91_e1253_d_b19, eq91_e1253_d_b20, eq91_e1253_d_b21, eq91_e1253_d_b22, eq91_e1253_d_b23, eq91_e1253_d_b24, eq91_e1253_d_b25, eq91_e1253_d_b26, eq91_e1253_d_b27, eq91_e1253_d_b28, eq91_e1253_d_b29, eq91_e1253_d_b30, eq91_e1253_d_b31, eq91_e1253_d_b32, eq91_e1253_d_b33, eq91_e1253_d_b34, eq91_e1253_d_b35,) = {
    if (!s.b[1054]) {
        let eq91_e1246: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 78, s.v[168]);
        let eq91_e1246_d_n0: f64 = (s.dn[168][0] * ddt_scale);
        let eq91_e1246_d_n1: f64 = (s.dn[168][1] * ddt_scale);
        let eq91_e1246_d_n2: f64 = (s.dn[168][2] * ddt_scale);
        let eq91_e1246_d_n3: f64 = (s.dn[168][3] * ddt_scale);
        let eq91_e1246_d_n4: f64 = (s.dn[168][4] * ddt_scale);
        let eq91_e1246_d_n5: f64 = (s.dn[168][5] * ddt_scale);
        let eq91_e1246_d_n6: f64 = (s.dn[168][6] * ddt_scale);
        let eq91_e1246_d_n7: f64 = (s.dn[168][7] * ddt_scale);
        let eq91_e1246_d_n8: f64 = (s.dn[168][8] * ddt_scale);
        let eq91_e1246_d_n9: f64 = (s.dn[168][9] * ddt_scale);
        let eq91_e1246_d_n10: f64 = (s.dn[168][10] * ddt_scale);
        let eq91_e1246_d_n11: f64 = (s.dn[168][11] * ddt_scale);
        let eq91_e1246_d_n12: f64 = (s.dn[168][12] * ddt_scale);
        let eq91_e1246_d_n13: f64 = (s.dn[168][13] * ddt_scale);
        let eq91_e1246_d_n14: f64 = (s.dn[168][14] * ddt_scale);
        let eq91_e1246_d_n15: f64 = (s.dn[168][15] * ddt_scale);
        let eq91_e1246_d_n16: f64 = (s.dn[168][16] * ddt_scale);
        let eq91_e1246_d_n17: f64 = (s.dn[168][17] * ddt_scale);
        let eq91_e1246_d_n18: f64 = (s.dn[168][18] * ddt_scale);
        let eq91_e1246_d_n19: f64 = (s.dn[168][19] * ddt_scale);
        let eq91_e1246_d_n20: f64 = (s.dn[168][20] * ddt_scale);
        let eq91_e1246_d_n21: f64 = (s.dn[168][21] * ddt_scale);
        let eq91_e1246_d_n22: f64 = (s.dn[168][22] * ddt_scale);
        let eq91_e1246_d_n23: f64 = (s.dn[168][23] * ddt_scale);
        let eq91_e1246_d_n24: f64 = (s.dn[168][24] * ddt_scale);
        let eq91_e1246_d_n25: f64 = (s.dn[168][25] * ddt_scale);
        let eq91_e1246_d_n26: f64 = (s.dn[168][26] * ddt_scale);
        let eq91_e1246_d_n27: f64 = (s.dn[168][27] * ddt_scale);
        let eq91_e1246_d_n28: f64 = (s.dn[168][28] * ddt_scale);
        let eq91_e1246_d_n29: f64 = (s.dn[168][29] * ddt_scale);
        let eq91_e1246_d_b0: f64 = (s.db[168][0] * ddt_scale);
        let eq91_e1246_d_b1: f64 = (s.db[168][1] * ddt_scale);
        let eq91_e1246_d_b2: f64 = (s.db[168][2] * ddt_scale);
        let eq91_e1246_d_b3: f64 = (s.db[168][3] * ddt_scale);
        let eq91_e1246_d_b4: f64 = (s.db[168][4] * ddt_scale);
        let eq91_e1246_d_b5: f64 = (s.db[168][5] * ddt_scale);
        let eq91_e1246_d_b6: f64 = (s.db[168][6] * ddt_scale);
        let eq91_e1246_d_b7: f64 = (s.db[168][7] * ddt_scale);
        let eq91_e1246_d_b8: f64 = (s.db[168][8] * ddt_scale);
        let eq91_e1246_d_b9: f64 = (s.db[168][9] * ddt_scale);
        let eq91_e1246_d_b10: f64 = (s.db[168][10] * ddt_scale);
        let eq91_e1246_d_b11: f64 = (s.db[168][11] * ddt_scale);
        let eq91_e1246_d_b12: f64 = (s.db[168][12] * ddt_scale);
        let eq91_e1246_d_b13: f64 = (s.db[168][13] * ddt_scale);
        let eq91_e1246_d_b14: f64 = (s.db[168][14] * ddt_scale);
        let eq91_e1246_d_b15: f64 = (s.db[168][15] * ddt_scale);
        let eq91_e1246_d_b16: f64 = (s.db[168][16] * ddt_scale);
        let eq91_e1246_d_b17: f64 = (s.db[168][17] * ddt_scale);
        let eq91_e1246_d_b18: f64 = (s.db[168][18] * ddt_scale);
        let eq91_e1246_d_b19: f64 = (s.db[168][19] * ddt_scale);
        let eq91_e1246_d_b20: f64 = (s.db[168][20] * ddt_scale);
        let eq91_e1246_d_b21: f64 = (s.db[168][21] * ddt_scale);
        let eq91_e1246_d_b22: f64 = (s.db[168][22] * ddt_scale);
        let eq91_e1246_d_b23: f64 = (s.db[168][23] * ddt_scale);
        let eq91_e1246_d_b24: f64 = (s.db[168][24] * ddt_scale);
        let eq91_e1246_d_b25: f64 = (s.db[168][25] * ddt_scale);
        let eq91_e1246_d_b26: f64 = (s.db[168][26] * ddt_scale);
        let eq91_e1246_d_b27: f64 = (s.db[168][27] * ddt_scale);
        let eq91_e1246_d_b28: f64 = (s.db[168][28] * ddt_scale);
        let eq91_e1246_d_b29: f64 = (s.db[168][29] * ddt_scale);
        let eq91_e1246_d_b30: f64 = (s.db[168][30] * ddt_scale);
        let eq91_e1246_d_b31: f64 = (s.db[168][31] * ddt_scale);
        let eq91_e1246_d_b32: f64 = (s.db[168][32] * ddt_scale);
        let eq91_e1246_d_b33: f64 = (s.db[168][33] * ddt_scale);
        let eq91_e1246_d_b34: f64 = (s.db[168][34] * ddt_scale);
        let eq91_e1246_d_b35: f64 = (s.db[168][35] * ddt_scale);
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1249_d_n2: f64 = p.p355;
        let eq91_e1249_d_n9: f64 = (-p.p355);
        let eq91_e1250: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 79, eq91_e1249);
        let eq91_e1250_d_n2: f64 = (eq91_e1249_d_n2 * ddt_scale);
        let eq91_e1250_d_n9: f64 = (eq91_e1249_d_n9 * ddt_scale);
        let eq91_e1251: f64 = (eq91_e1246 + eq91_e1250);
        let eq91_e1251_d_n2: f64 = (eq91_e1246_d_n2 + eq91_e1250_d_n2);
        let eq91_e1251_d_n9: f64 = (eq91_e1246_d_n9 + eq91_e1250_d_n9);
        (eq91_e1251, eq91_e1246_d_n0, eq91_e1246_d_n1, eq91_e1251_d_n2, eq91_e1246_d_n3, eq91_e1246_d_n4, eq91_e1246_d_n5, eq91_e1246_d_n6, eq91_e1246_d_n7, eq91_e1246_d_n8, eq91_e1251_d_n9, eq91_e1246_d_n10, eq91_e1246_d_n11, eq91_e1246_d_n12, eq91_e1246_d_n13, eq91_e1246_d_n14, eq91_e1246_d_n15, eq91_e1246_d_n16, eq91_e1246_d_n17, eq91_e1246_d_n18, eq91_e1246_d_n19, eq91_e1246_d_n20, eq91_e1246_d_n21, eq91_e1246_d_n22, eq91_e1246_d_n23, eq91_e1246_d_n24, eq91_e1246_d_n25, eq91_e1246_d_n26, eq91_e1246_d_n27, eq91_e1246_d_n28, eq91_e1246_d_n29, eq91_e1246_d_b0, eq91_e1246_d_b1, eq91_e1246_d_b2, eq91_e1246_d_b3, eq91_e1246_d_b4, eq91_e1246_d_b5, eq91_e1246_d_b6, eq91_e1246_d_b7, eq91_e1246_d_b8, eq91_e1246_d_b9, eq91_e1246_d_b10, eq91_e1246_d_b11, eq91_e1246_d_b12, eq91_e1246_d_b13, eq91_e1246_d_b14, eq91_e1246_d_b15, eq91_e1246_d_b16, eq91_e1246_d_b17, eq91_e1246_d_b18, eq91_e1246_d_b19, eq91_e1246_d_b20, eq91_e1246_d_b21, eq91_e1246_d_b22, eq91_e1246_d_b23, eq91_e1246_d_b24, eq91_e1246_d_b25, eq91_e1246_d_b26, eq91_e1246_d_b27, eq91_e1246_d_b28, eq91_e1246_d_b29, eq91_e1246_d_b30, eq91_e1246_d_b31, eq91_e1246_d_b32, eq91_e1246_d_b33, eq91_e1246_d_b34, eq91_e1246_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_value: f64 = eq91_e1253;
        let eq91_node_derivatives: [f64; 30] = [eq91_e1253_d_n0, eq91_e1253_d_n1, eq91_e1253_d_n2, eq91_e1253_d_n3, eq91_e1253_d_n4, eq91_e1253_d_n5, eq91_e1253_d_n6, eq91_e1253_d_n7, eq91_e1253_d_n8, eq91_e1253_d_n9, eq91_e1253_d_n10, eq91_e1253_d_n11, eq91_e1253_d_n12, eq91_e1253_d_n13, eq91_e1253_d_n14, eq91_e1253_d_n15, eq91_e1253_d_n16, eq91_e1253_d_n17, eq91_e1253_d_n18, eq91_e1253_d_n19, eq91_e1253_d_n20, eq91_e1253_d_n21, eq91_e1253_d_n22, eq91_e1253_d_n23, eq91_e1253_d_n24, eq91_e1253_d_n25, eq91_e1253_d_n26, eq91_e1253_d_n27, eq91_e1253_d_n28, eq91_e1253_d_n29];
        let eq91_branch_derivatives: [f64; 36] = [eq91_e1253_d_b0, eq91_e1253_d_b1, eq91_e1253_d_b2, eq91_e1253_d_b3, eq91_e1253_d_b4, eq91_e1253_d_b5, eq91_e1253_d_b6, eq91_e1253_d_b7, eq91_e1253_d_b8, eq91_e1253_d_b9, eq91_e1253_d_b10, eq91_e1253_d_b11, eq91_e1253_d_b12, eq91_e1253_d_b13, eq91_e1253_d_b14, eq91_e1253_d_b15, eq91_e1253_d_b16, eq91_e1253_d_b17, eq91_e1253_d_b18, eq91_e1253_d_b19, eq91_e1253_d_b20, eq91_e1253_d_b21, eq91_e1253_d_b22, eq91_e1253_d_b23, eq91_e1253_d_b24, eq91_e1253_d_b25, eq91_e1253_d_b26, eq91_e1253_d_b27, eq91_e1253_d_b28, eq91_e1253_d_b29, eq91_e1253_d_b30, eq91_e1253_d_b31, eq91_e1253_d_b32, eq91_e1253_d_b33, eq91_e1253_d_b34, eq91_e1253_d_b35];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq91_value),
            &eq91_node_derivatives,
            &eq91_branch_derivatives,
            multiplicity,
        );
        let (eq92_e1264, eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29, eq92_e1264_d_b0, eq92_e1264_d_b1, eq92_e1264_d_b2, eq92_e1264_d_b3, eq92_e1264_d_b4, eq92_e1264_d_b5, eq92_e1264_d_b6, eq92_e1264_d_b7, eq92_e1264_d_b8, eq92_e1264_d_b9, eq92_e1264_d_b10, eq92_e1264_d_b11, eq92_e1264_d_b12, eq92_e1264_d_b13, eq92_e1264_d_b14, eq92_e1264_d_b15, eq92_e1264_d_b16, eq92_e1264_d_b17, eq92_e1264_d_b18, eq92_e1264_d_b19, eq92_e1264_d_b20, eq92_e1264_d_b21, eq92_e1264_d_b22, eq92_e1264_d_b23, eq92_e1264_d_b24, eq92_e1264_d_b25, eq92_e1264_d_b26, eq92_e1264_d_b27, eq92_e1264_d_b28, eq92_e1264_d_b29, eq92_e1264_d_b30, eq92_e1264_d_b31, eq92_e1264_d_b32, eq92_e1264_d_b33, eq92_e1264_d_b34, eq92_e1264_d_b35,) = {
    if (!s.b[1054]) {
        let eq92_e1257: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 80, s.v[169]);
        let eq92_e1257_d_n0: f64 = (s.dn[169][0] * ddt_scale);
        let eq92_e1257_d_n1: f64 = (s.dn[169][1] * ddt_scale);
        let eq92_e1257_d_n2: f64 = (s.dn[169][2] * ddt_scale);
        let eq92_e1257_d_n3: f64 = (s.dn[169][3] * ddt_scale);
        let eq92_e1257_d_n4: f64 = (s.dn[169][4] * ddt_scale);
        let eq92_e1257_d_n5: f64 = (s.dn[169][5] * ddt_scale);
        let eq92_e1257_d_n6: f64 = (s.dn[169][6] * ddt_scale);
        let eq92_e1257_d_n7: f64 = (s.dn[169][7] * ddt_scale);
        let eq92_e1257_d_n8: f64 = (s.dn[169][8] * ddt_scale);
        let eq92_e1257_d_n9: f64 = (s.dn[169][9] * ddt_scale);
        let eq92_e1257_d_n10: f64 = (s.dn[169][10] * ddt_scale);
        let eq92_e1257_d_n11: f64 = (s.dn[169][11] * ddt_scale);
        let eq92_e1257_d_n12: f64 = (s.dn[169][12] * ddt_scale);
        let eq92_e1257_d_n13: f64 = (s.dn[169][13] * ddt_scale);
        let eq92_e1257_d_n14: f64 = (s.dn[169][14] * ddt_scale);
        let eq92_e1257_d_n15: f64 = (s.dn[169][15] * ddt_scale);
        let eq92_e1257_d_n16: f64 = (s.dn[169][16] * ddt_scale);
        let eq92_e1257_d_n17: f64 = (s.dn[169][17] * ddt_scale);
        let eq92_e1257_d_n18: f64 = (s.dn[169][18] * ddt_scale);
        let eq92_e1257_d_n19: f64 = (s.dn[169][19] * ddt_scale);
        let eq92_e1257_d_n20: f64 = (s.dn[169][20] * ddt_scale);
        let eq92_e1257_d_n21: f64 = (s.dn[169][21] * ddt_scale);
        let eq92_e1257_d_n22: f64 = (s.dn[169][22] * ddt_scale);
        let eq92_e1257_d_n23: f64 = (s.dn[169][23] * ddt_scale);
        let eq92_e1257_d_n24: f64 = (s.dn[169][24] * ddt_scale);
        let eq92_e1257_d_n25: f64 = (s.dn[169][25] * ddt_scale);
        let eq92_e1257_d_n26: f64 = (s.dn[169][26] * ddt_scale);
        let eq92_e1257_d_n27: f64 = (s.dn[169][27] * ddt_scale);
        let eq92_e1257_d_n28: f64 = (s.dn[169][28] * ddt_scale);
        let eq92_e1257_d_n29: f64 = (s.dn[169][29] * ddt_scale);
        let eq92_e1257_d_b0: f64 = (s.db[169][0] * ddt_scale);
        let eq92_e1257_d_b1: f64 = (s.db[169][1] * ddt_scale);
        let eq92_e1257_d_b2: f64 = (s.db[169][2] * ddt_scale);
        let eq92_e1257_d_b3: f64 = (s.db[169][3] * ddt_scale);
        let eq92_e1257_d_b4: f64 = (s.db[169][4] * ddt_scale);
        let eq92_e1257_d_b5: f64 = (s.db[169][5] * ddt_scale);
        let eq92_e1257_d_b6: f64 = (s.db[169][6] * ddt_scale);
        let eq92_e1257_d_b7: f64 = (s.db[169][7] * ddt_scale);
        let eq92_e1257_d_b8: f64 = (s.db[169][8] * ddt_scale);
        let eq92_e1257_d_b9: f64 = (s.db[169][9] * ddt_scale);
        let eq92_e1257_d_b10: f64 = (s.db[169][10] * ddt_scale);
        let eq92_e1257_d_b11: f64 = (s.db[169][11] * ddt_scale);
        let eq92_e1257_d_b12: f64 = (s.db[169][12] * ddt_scale);
        let eq92_e1257_d_b13: f64 = (s.db[169][13] * ddt_scale);
        let eq92_e1257_d_b14: f64 = (s.db[169][14] * ddt_scale);
        let eq92_e1257_d_b15: f64 = (s.db[169][15] * ddt_scale);
        let eq92_e1257_d_b16: f64 = (s.db[169][16] * ddt_scale);
        let eq92_e1257_d_b17: f64 = (s.db[169][17] * ddt_scale);
        let eq92_e1257_d_b18: f64 = (s.db[169][18] * ddt_scale);
        let eq92_e1257_d_b19: f64 = (s.db[169][19] * ddt_scale);
        let eq92_e1257_d_b20: f64 = (s.db[169][20] * ddt_scale);
        let eq92_e1257_d_b21: f64 = (s.db[169][21] * ddt_scale);
        let eq92_e1257_d_b22: f64 = (s.db[169][22] * ddt_scale);
        let eq92_e1257_d_b23: f64 = (s.db[169][23] * ddt_scale);
        let eq92_e1257_d_b24: f64 = (s.db[169][24] * ddt_scale);
        let eq92_e1257_d_b25: f64 = (s.db[169][25] * ddt_scale);
        let eq92_e1257_d_b26: f64 = (s.db[169][26] * ddt_scale);
        let eq92_e1257_d_b27: f64 = (s.db[169][27] * ddt_scale);
        let eq92_e1257_d_b28: f64 = (s.db[169][28] * ddt_scale);
        let eq92_e1257_d_b29: f64 = (s.db[169][29] * ddt_scale);
        let eq92_e1257_d_b30: f64 = (s.db[169][30] * ddt_scale);
        let eq92_e1257_d_b31: f64 = (s.db[169][31] * ddt_scale);
        let eq92_e1257_d_b32: f64 = (s.db[169][32] * ddt_scale);
        let eq92_e1257_d_b33: f64 = (s.db[169][33] * ddt_scale);
        let eq92_e1257_d_b34: f64 = (s.db[169][34] * ddt_scale);
        let eq92_e1257_d_b35: f64 = (s.db[169][35] * ddt_scale);
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1260_d_n7: f64 = p.p355;
        let eq92_e1260_d_n10: f64 = (-p.p355);
        let eq92_e1261: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 81, eq92_e1260);
        let eq92_e1261_d_n7: f64 = (eq92_e1260_d_n7 * ddt_scale);
        let eq92_e1261_d_n10: f64 = (eq92_e1260_d_n10 * ddt_scale);
        let eq92_e1262: f64 = (eq92_e1257 + eq92_e1261);
        let eq92_e1262_d_n7: f64 = (eq92_e1257_d_n7 + eq92_e1261_d_n7);
        let eq92_e1262_d_n10: f64 = (eq92_e1257_d_n10 + eq92_e1261_d_n10);
        (eq92_e1262, eq92_e1257_d_n0, eq92_e1257_d_n1, eq92_e1257_d_n2, eq92_e1257_d_n3, eq92_e1257_d_n4, eq92_e1257_d_n5, eq92_e1257_d_n6, eq92_e1262_d_n7, eq92_e1257_d_n8, eq92_e1257_d_n9, eq92_e1262_d_n10, eq92_e1257_d_n11, eq92_e1257_d_n12, eq92_e1257_d_n13, eq92_e1257_d_n14, eq92_e1257_d_n15, eq92_e1257_d_n16, eq92_e1257_d_n17, eq92_e1257_d_n18, eq92_e1257_d_n19, eq92_e1257_d_n20, eq92_e1257_d_n21, eq92_e1257_d_n22, eq92_e1257_d_n23, eq92_e1257_d_n24, eq92_e1257_d_n25, eq92_e1257_d_n26, eq92_e1257_d_n27, eq92_e1257_d_n28, eq92_e1257_d_n29, eq92_e1257_d_b0, eq92_e1257_d_b1, eq92_e1257_d_b2, eq92_e1257_d_b3, eq92_e1257_d_b4, eq92_e1257_d_b5, eq92_e1257_d_b6, eq92_e1257_d_b7, eq92_e1257_d_b8, eq92_e1257_d_b9, eq92_e1257_d_b10, eq92_e1257_d_b11, eq92_e1257_d_b12, eq92_e1257_d_b13, eq92_e1257_d_b14, eq92_e1257_d_b15, eq92_e1257_d_b16, eq92_e1257_d_b17, eq92_e1257_d_b18, eq92_e1257_d_b19, eq92_e1257_d_b20, eq92_e1257_d_b21, eq92_e1257_d_b22, eq92_e1257_d_b23, eq92_e1257_d_b24, eq92_e1257_d_b25, eq92_e1257_d_b26, eq92_e1257_d_b27, eq92_e1257_d_b28, eq92_e1257_d_b29, eq92_e1257_d_b30, eq92_e1257_d_b31, eq92_e1257_d_b32, eq92_e1257_d_b33, eq92_e1257_d_b34, eq92_e1257_d_b35,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e1264;
        let eq92_node_derivatives: [f64; 30] = [eq92_e1264_d_n0, eq92_e1264_d_n1, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n5, eq92_e1264_d_n6, eq92_e1264_d_n7, eq92_e1264_d_n8, eq92_e1264_d_n9, eq92_e1264_d_n10, eq92_e1264_d_n11, eq92_e1264_d_n12, eq92_e1264_d_n13, eq92_e1264_d_n14, eq92_e1264_d_n15, eq92_e1264_d_n16, eq92_e1264_d_n17, eq92_e1264_d_n18, eq92_e1264_d_n19, eq92_e1264_d_n20, eq92_e1264_d_n21, eq92_e1264_d_n22, eq92_e1264_d_n23, eq92_e1264_d_n24, eq92_e1264_d_n25, eq92_e1264_d_n26, eq92_e1264_d_n27, eq92_e1264_d_n28, eq92_e1264_d_n29];
        let eq92_branch_derivatives: [f64; 36] = [eq92_e1264_d_b0, eq92_e1264_d_b1, eq92_e1264_d_b2, eq92_e1264_d_b3, eq92_e1264_d_b4, eq92_e1264_d_b5, eq92_e1264_d_b6, eq92_e1264_d_b7, eq92_e1264_d_b8, eq92_e1264_d_b9, eq92_e1264_d_b10, eq92_e1264_d_b11, eq92_e1264_d_b12, eq92_e1264_d_b13, eq92_e1264_d_b14, eq92_e1264_d_b15, eq92_e1264_d_b16, eq92_e1264_d_b17, eq92_e1264_d_b18, eq92_e1264_d_b19, eq92_e1264_d_b20, eq92_e1264_d_b21, eq92_e1264_d_b22, eq92_e1264_d_b23, eq92_e1264_d_b24, eq92_e1264_d_b25, eq92_e1264_d_b26, eq92_e1264_d_b27, eq92_e1264_d_b28, eq92_e1264_d_b29, eq92_e1264_d_b30, eq92_e1264_d_b31, eq92_e1264_d_b32, eq92_e1264_d_b33, eq92_e1264_d_b34, eq92_e1264_d_b35];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq92_value),
            &eq92_node_derivatives,
            &eq92_branch_derivatives,
            multiplicity,
        );
        let (eq93_e1269,) = {
    if (!s.b[1054]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq93_value: f64 = eq93_e1269;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq93_value),
        );
        let (eq94_e1274,) = {
    if (!s.b[1054]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1274;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq94_value),
        );
        let eq95_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 82, s.v[170]);
        let eq95_e1276_d_n0: f64 = (s.dn[170][0] * ddt_scale);
        let eq95_e1276_d_n1: f64 = (s.dn[170][1] * ddt_scale);
        let eq95_e1276_d_n2: f64 = (s.dn[170][2] * ddt_scale);
        let eq95_e1276_d_n3: f64 = (s.dn[170][3] * ddt_scale);
        let eq95_e1276_d_n4: f64 = (s.dn[170][4] * ddt_scale);
        let eq95_e1276_d_n5: f64 = (s.dn[170][5] * ddt_scale);
        let eq95_e1276_d_n6: f64 = (s.dn[170][6] * ddt_scale);
        let eq95_e1276_d_n7: f64 = (s.dn[170][7] * ddt_scale);
        let eq95_e1276_d_n8: f64 = (s.dn[170][8] * ddt_scale);
        let eq95_e1276_d_n9: f64 = (s.dn[170][9] * ddt_scale);
        let eq95_e1276_d_n10: f64 = (s.dn[170][10] * ddt_scale);
        let eq95_e1276_d_n11: f64 = (s.dn[170][11] * ddt_scale);
        let eq95_e1276_d_n12: f64 = (s.dn[170][12] * ddt_scale);
        let eq95_e1276_d_n13: f64 = (s.dn[170][13] * ddt_scale);
        let eq95_e1276_d_n14: f64 = (s.dn[170][14] * ddt_scale);
        let eq95_e1276_d_n15: f64 = (s.dn[170][15] * ddt_scale);
        let eq95_e1276_d_n16: f64 = (s.dn[170][16] * ddt_scale);
        let eq95_e1276_d_n17: f64 = (s.dn[170][17] * ddt_scale);
        let eq95_e1276_d_n18: f64 = (s.dn[170][18] * ddt_scale);
        let eq95_e1276_d_n19: f64 = (s.dn[170][19] * ddt_scale);
        let eq95_e1276_d_n20: f64 = (s.dn[170][20] * ddt_scale);
        let eq95_e1276_d_n21: f64 = (s.dn[170][21] * ddt_scale);
        let eq95_e1276_d_n22: f64 = (s.dn[170][22] * ddt_scale);
        let eq95_e1276_d_n23: f64 = (s.dn[170][23] * ddt_scale);
        let eq95_e1276_d_n24: f64 = (s.dn[170][24] * ddt_scale);
        let eq95_e1276_d_n25: f64 = (s.dn[170][25] * ddt_scale);
        let eq95_e1276_d_n26: f64 = (s.dn[170][26] * ddt_scale);
        let eq95_e1276_d_n27: f64 = (s.dn[170][27] * ddt_scale);
        let eq95_e1276_d_n28: f64 = (s.dn[170][28] * ddt_scale);
        let eq95_e1276_d_n29: f64 = (s.dn[170][29] * ddt_scale);
        let eq95_e1276_d_b0: f64 = (s.db[170][0] * ddt_scale);
        let eq95_e1276_d_b1: f64 = (s.db[170][1] * ddt_scale);
        let eq95_e1276_d_b2: f64 = (s.db[170][2] * ddt_scale);
        let eq95_e1276_d_b3: f64 = (s.db[170][3] * ddt_scale);
        let eq95_e1276_d_b4: f64 = (s.db[170][4] * ddt_scale);
        let eq95_e1276_d_b5: f64 = (s.db[170][5] * ddt_scale);
        let eq95_e1276_d_b6: f64 = (s.db[170][6] * ddt_scale);
        let eq95_e1276_d_b7: f64 = (s.db[170][7] * ddt_scale);
        let eq95_e1276_d_b8: f64 = (s.db[170][8] * ddt_scale);
        let eq95_e1276_d_b9: f64 = (s.db[170][9] * ddt_scale);
        let eq95_e1276_d_b10: f64 = (s.db[170][10] * ddt_scale);
        let eq95_e1276_d_b11: f64 = (s.db[170][11] * ddt_scale);
        let eq95_e1276_d_b12: f64 = (s.db[170][12] * ddt_scale);
        let eq95_e1276_d_b13: f64 = (s.db[170][13] * ddt_scale);
        let eq95_e1276_d_b14: f64 = (s.db[170][14] * ddt_scale);
        let eq95_e1276_d_b15: f64 = (s.db[170][15] * ddt_scale);
        let eq95_e1276_d_b16: f64 = (s.db[170][16] * ddt_scale);
        let eq95_e1276_d_b17: f64 = (s.db[170][17] * ddt_scale);
        let eq95_e1276_d_b18: f64 = (s.db[170][18] * ddt_scale);
        let eq95_e1276_d_b19: f64 = (s.db[170][19] * ddt_scale);
        let eq95_e1276_d_b20: f64 = (s.db[170][20] * ddt_scale);
        let eq95_e1276_d_b21: f64 = (s.db[170][21] * ddt_scale);
        let eq95_e1276_d_b22: f64 = (s.db[170][22] * ddt_scale);
        let eq95_e1276_d_b23: f64 = (s.db[170][23] * ddt_scale);
        let eq95_e1276_d_b24: f64 = (s.db[170][24] * ddt_scale);
        let eq95_e1276_d_b25: f64 = (s.db[170][25] * ddt_scale);
        let eq95_e1276_d_b26: f64 = (s.db[170][26] * ddt_scale);
        let eq95_e1276_d_b27: f64 = (s.db[170][27] * ddt_scale);
        let eq95_e1276_d_b28: f64 = (s.db[170][28] * ddt_scale);
        let eq95_e1276_d_b29: f64 = (s.db[170][29] * ddt_scale);
        let eq95_e1276_d_b30: f64 = (s.db[170][30] * ddt_scale);
        let eq95_e1276_d_b31: f64 = (s.db[170][31] * ddt_scale);
        let eq95_e1276_d_b32: f64 = (s.db[170][32] * ddt_scale);
        let eq95_e1276_d_b33: f64 = (s.db[170][33] * ddt_scale);
        let eq95_e1276_d_b34: f64 = (s.db[170][34] * ddt_scale);
        let eq95_e1276_d_b35: f64 = (s.db[170][35] * ddt_scale);
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1279_d_n3: f64 = p.p355;
        let eq95_e1279_d_n10: f64 = (-p.p355);
        let eq95_e1280: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 83, eq95_e1279);
        let eq95_e1280_d_n3: f64 = (eq95_e1279_d_n3 * ddt_scale);
        let eq95_e1280_d_n10: f64 = (eq95_e1279_d_n10 * ddt_scale);
        let eq95_e1281: f64 = (eq95_e1276 + eq95_e1280);
        let eq95_e1281_d_n3: f64 = (eq95_e1276_d_n3 + eq95_e1280_d_n3);
        let eq95_e1281_d_n10: f64 = (eq95_e1276_d_n10 + eq95_e1280_d_n10);
        let eq95_value: f64 = eq95_e1281;
        let eq95_node_derivatives: [f64; 30] = [eq95_e1276_d_n0, eq95_e1276_d_n1, eq95_e1276_d_n2, eq95_e1281_d_n3, eq95_e1276_d_n4, eq95_e1276_d_n5, eq95_e1276_d_n6, eq95_e1276_d_n7, eq95_e1276_d_n8, eq95_e1276_d_n9, eq95_e1281_d_n10, eq95_e1276_d_n11, eq95_e1276_d_n12, eq95_e1276_d_n13, eq95_e1276_d_n14, eq95_e1276_d_n15, eq95_e1276_d_n16, eq95_e1276_d_n17, eq95_e1276_d_n18, eq95_e1276_d_n19, eq95_e1276_d_n20, eq95_e1276_d_n21, eq95_e1276_d_n22, eq95_e1276_d_n23, eq95_e1276_d_n24, eq95_e1276_d_n25, eq95_e1276_d_n26, eq95_e1276_d_n27, eq95_e1276_d_n28, eq95_e1276_d_n29];
        let eq95_branch_derivatives: [f64; 36] = [eq95_e1276_d_b0, eq95_e1276_d_b1, eq95_e1276_d_b2, eq95_e1276_d_b3, eq95_e1276_d_b4, eq95_e1276_d_b5, eq95_e1276_d_b6, eq95_e1276_d_b7, eq95_e1276_d_b8, eq95_e1276_d_b9, eq95_e1276_d_b10, eq95_e1276_d_b11, eq95_e1276_d_b12, eq95_e1276_d_b13, eq95_e1276_d_b14, eq95_e1276_d_b15, eq95_e1276_d_b16, eq95_e1276_d_b17, eq95_e1276_d_b18, eq95_e1276_d_b19, eq95_e1276_d_b20, eq95_e1276_d_b21, eq95_e1276_d_b22, eq95_e1276_d_b23, eq95_e1276_d_b24, eq95_e1276_d_b25, eq95_e1276_d_b26, eq95_e1276_d_b27, eq95_e1276_d_b28, eq95_e1276_d_b29, eq95_e1276_d_b30, eq95_e1276_d_b31, eq95_e1276_d_b32, eq95_e1276_d_b33, eq95_e1276_d_b34, eq95_e1276_d_b35];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq95_value),
            &eq95_node_derivatives,
            &eq95_branch_derivatives,
            multiplicity,
        );
        let (eq96_e1289, eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29, eq96_e1289_d_b0, eq96_e1289_d_b1, eq96_e1289_d_b2, eq96_e1289_d_b3, eq96_e1289_d_b4, eq96_e1289_d_b5, eq96_e1289_d_b6, eq96_e1289_d_b7, eq96_e1289_d_b8, eq96_e1289_d_b9, eq96_e1289_d_b10, eq96_e1289_d_b11, eq96_e1289_d_b12, eq96_e1289_d_b13, eq96_e1289_d_b14, eq96_e1289_d_b15, eq96_e1289_d_b16, eq96_e1289_d_b17, eq96_e1289_d_b18, eq96_e1289_d_b19, eq96_e1289_d_b20, eq96_e1289_d_b21, eq96_e1289_d_b22, eq96_e1289_d_b23, eq96_e1289_d_b24, eq96_e1289_d_b25, eq96_e1289_d_b26, eq96_e1289_d_b27, eq96_e1289_d_b28, eq96_e1289_d_b29, eq96_e1289_d_b30, eq96_e1289_d_b31, eq96_e1289_d_b32, eq96_e1289_d_b33, eq96_e1289_d_b34, eq96_e1289_d_b35,) = {
    if s.b[1055] {
        let eq96_e1286: f64 = (s.v[0] * (nv10 - nv11));
        let eq96_e1286_d_n10: f64 = s.v[0];
        let eq96_e1286_d_n11: f64 = (-s.v[0]);
        let eq96_e1287: f64 = (s.v[172] + eq96_e1286);
        let eq96_e1287_d_n10: f64 = (s.dn[172][10] + eq96_e1286_d_n10);
        let eq96_e1287_d_n11: f64 = (s.dn[172][11] + eq96_e1286_d_n11);
        (eq96_e1287, s.dn[172][0], s.dn[172][1], s.dn[172][2], s.dn[172][3], s.dn[172][4], s.dn[172][5], s.dn[172][6], s.dn[172][7], s.dn[172][8], s.dn[172][9], eq96_e1287_d_n10, eq96_e1287_d_n11, s.dn[172][12], s.dn[172][13], s.dn[172][14], s.dn[172][15], s.dn[172][16], s.dn[172][17], s.dn[172][18], s.dn[172][19], s.dn[172][20], s.dn[172][21], s.dn[172][22], s.dn[172][23], s.dn[172][24], s.dn[172][25], s.dn[172][26], s.dn[172][27], s.dn[172][28], s.dn[172][29], s.db[172][0], s.db[172][1], s.db[172][2], s.db[172][3], s.db[172][4], s.db[172][5], s.db[172][6], s.db[172][7], s.db[172][8], s.db[172][9], s.db[172][10], s.db[172][11], s.db[172][12], s.db[172][13], s.db[172][14], s.db[172][15], s.db[172][16], s.db[172][17], s.db[172][18], s.db[172][19], s.db[172][20], s.db[172][21], s.db[172][22], s.db[172][23], s.db[172][24], s.db[172][25], s.db[172][26], s.db[172][27], s.db[172][28], s.db[172][29], s.db[172][30], s.db[172][31], s.db[172][32], s.db[172][33], s.db[172][34], s.db[172][35],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1289;
        let eq96_node_derivatives: [f64; 30] = [eq96_e1289_d_n0, eq96_e1289_d_n1, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n5, eq96_e1289_d_n6, eq96_e1289_d_n7, eq96_e1289_d_n8, eq96_e1289_d_n9, eq96_e1289_d_n10, eq96_e1289_d_n11, eq96_e1289_d_n12, eq96_e1289_d_n13, eq96_e1289_d_n14, eq96_e1289_d_n15, eq96_e1289_d_n16, eq96_e1289_d_n17, eq96_e1289_d_n18, eq96_e1289_d_n19, eq96_e1289_d_n20, eq96_e1289_d_n21, eq96_e1289_d_n22, eq96_e1289_d_n23, eq96_e1289_d_n24, eq96_e1289_d_n25, eq96_e1289_d_n26, eq96_e1289_d_n27, eq96_e1289_d_n28, eq96_e1289_d_n29];
        let eq96_branch_derivatives: [f64; 36] = [eq96_e1289_d_b0, eq96_e1289_d_b1, eq96_e1289_d_b2, eq96_e1289_d_b3, eq96_e1289_d_b4, eq96_e1289_d_b5, eq96_e1289_d_b6, eq96_e1289_d_b7, eq96_e1289_d_b8, eq96_e1289_d_b9, eq96_e1289_d_b10, eq96_e1289_d_b11, eq96_e1289_d_b12, eq96_e1289_d_b13, eq96_e1289_d_b14, eq96_e1289_d_b15, eq96_e1289_d_b16, eq96_e1289_d_b17, eq96_e1289_d_b18, eq96_e1289_d_b19, eq96_e1289_d_b20, eq96_e1289_d_b21, eq96_e1289_d_b22, eq96_e1289_d_b23, eq96_e1289_d_b24, eq96_e1289_d_b25, eq96_e1289_d_b26, eq96_e1289_d_b27, eq96_e1289_d_b28, eq96_e1289_d_b29, eq96_e1289_d_b30, eq96_e1289_d_b31, eq96_e1289_d_b32, eq96_e1289_d_b33, eq96_e1289_d_b34, eq96_e1289_d_b35];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq96_value),
            &eq96_node_derivatives,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e1294,) = {
    if (!s.b[1055]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1294;
        stamper.stamp_potential_const_local(
            23,
            eq97_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq98_e1304, eq98_e1304_d_n0, eq98_e1304_d_n1, eq98_e1304_d_n2, eq98_e1304_d_n3, eq98_e1304_d_n4, eq98_e1304_d_n5, eq98_e1304_d_n6, eq98_e1304_d_n7, eq98_e1304_d_n8, eq98_e1304_d_n9, eq98_e1304_d_n10, eq98_e1304_d_n11, eq98_e1304_d_n12, eq98_e1304_d_n13, eq98_e1304_d_n14, eq98_e1304_d_n15, eq98_e1304_d_n16, eq98_e1304_d_n17, eq98_e1304_d_n18, eq98_e1304_d_n19, eq98_e1304_d_n20, eq98_e1304_d_n21, eq98_e1304_d_n22, eq98_e1304_d_n23, eq98_e1304_d_n24, eq98_e1304_d_n25, eq98_e1304_d_n26, eq98_e1304_d_n27, eq98_e1304_d_n28, eq98_e1304_d_n29, eq98_e1304_d_b0, eq98_e1304_d_b1, eq98_e1304_d_b2, eq98_e1304_d_b3, eq98_e1304_d_b4, eq98_e1304_d_b5, eq98_e1304_d_b6, eq98_e1304_d_b7, eq98_e1304_d_b8, eq98_e1304_d_b9, eq98_e1304_d_b10, eq98_e1304_d_b11, eq98_e1304_d_b12, eq98_e1304_d_b13, eq98_e1304_d_b14, eq98_e1304_d_b15, eq98_e1304_d_b16, eq98_e1304_d_b17, eq98_e1304_d_b18, eq98_e1304_d_b19, eq98_e1304_d_b20, eq98_e1304_d_b21, eq98_e1304_d_b22, eq98_e1304_d_b23, eq98_e1304_d_b24, eq98_e1304_d_b25, eq98_e1304_d_b26, eq98_e1304_d_b27, eq98_e1304_d_b28, eq98_e1304_d_b29, eq98_e1304_d_b30, eq98_e1304_d_b31, eq98_e1304_d_b32, eq98_e1304_d_b33, eq98_e1304_d_b34, eq98_e1304_d_b35,) = {
    if s.b[1201] {
        let eq98_e1297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 84, s.v[173]);
        let eq98_e1297_d_n0: f64 = (s.dn[173][0] * ddt_scale);
        let eq98_e1297_d_n1: f64 = (s.dn[173][1] * ddt_scale);
        let eq98_e1297_d_n2: f64 = (s.dn[173][2] * ddt_scale);
        let eq98_e1297_d_n3: f64 = (s.dn[173][3] * ddt_scale);
        let eq98_e1297_d_n4: f64 = (s.dn[173][4] * ddt_scale);
        let eq98_e1297_d_n5: f64 = (s.dn[173][5] * ddt_scale);
        let eq98_e1297_d_n6: f64 = (s.dn[173][6] * ddt_scale);
        let eq98_e1297_d_n7: f64 = (s.dn[173][7] * ddt_scale);
        let eq98_e1297_d_n8: f64 = (s.dn[173][8] * ddt_scale);
        let eq98_e1297_d_n9: f64 = (s.dn[173][9] * ddt_scale);
        let eq98_e1297_d_n10: f64 = (s.dn[173][10] * ddt_scale);
        let eq98_e1297_d_n11: f64 = (s.dn[173][11] * ddt_scale);
        let eq98_e1297_d_n12: f64 = (s.dn[173][12] * ddt_scale);
        let eq98_e1297_d_n13: f64 = (s.dn[173][13] * ddt_scale);
        let eq98_e1297_d_n14: f64 = (s.dn[173][14] * ddt_scale);
        let eq98_e1297_d_n15: f64 = (s.dn[173][15] * ddt_scale);
        let eq98_e1297_d_n16: f64 = (s.dn[173][16] * ddt_scale);
        let eq98_e1297_d_n17: f64 = (s.dn[173][17] * ddt_scale);
        let eq98_e1297_d_n18: f64 = (s.dn[173][18] * ddt_scale);
        let eq98_e1297_d_n19: f64 = (s.dn[173][19] * ddt_scale);
        let eq98_e1297_d_n20: f64 = (s.dn[173][20] * ddt_scale);
        let eq98_e1297_d_n21: f64 = (s.dn[173][21] * ddt_scale);
        let eq98_e1297_d_n22: f64 = (s.dn[173][22] * ddt_scale);
        let eq98_e1297_d_n23: f64 = (s.dn[173][23] * ddt_scale);
        let eq98_e1297_d_n24: f64 = (s.dn[173][24] * ddt_scale);
        let eq98_e1297_d_n25: f64 = (s.dn[173][25] * ddt_scale);
        let eq98_e1297_d_n26: f64 = (s.dn[173][26] * ddt_scale);
        let eq98_e1297_d_n27: f64 = (s.dn[173][27] * ddt_scale);
        let eq98_e1297_d_n28: f64 = (s.dn[173][28] * ddt_scale);
        let eq98_e1297_d_n29: f64 = (s.dn[173][29] * ddt_scale);
        let eq98_e1297_d_b0: f64 = (s.db[173][0] * ddt_scale);
        let eq98_e1297_d_b1: f64 = (s.db[173][1] * ddt_scale);
        let eq98_e1297_d_b2: f64 = (s.db[173][2] * ddt_scale);
        let eq98_e1297_d_b3: f64 = (s.db[173][3] * ddt_scale);
        let eq98_e1297_d_b4: f64 = (s.db[173][4] * ddt_scale);
        let eq98_e1297_d_b5: f64 = (s.db[173][5] * ddt_scale);
        let eq98_e1297_d_b6: f64 = (s.db[173][6] * ddt_scale);
        let eq98_e1297_d_b7: f64 = (s.db[173][7] * ddt_scale);
        let eq98_e1297_d_b8: f64 = (s.db[173][8] * ddt_scale);
        let eq98_e1297_d_b9: f64 = (s.db[173][9] * ddt_scale);
        let eq98_e1297_d_b10: f64 = (s.db[173][10] * ddt_scale);
        let eq98_e1297_d_b11: f64 = (s.db[173][11] * ddt_scale);
        let eq98_e1297_d_b12: f64 = (s.db[173][12] * ddt_scale);
        let eq98_e1297_d_b13: f64 = (s.db[173][13] * ddt_scale);
        let eq98_e1297_d_b14: f64 = (s.db[173][14] * ddt_scale);
        let eq98_e1297_d_b15: f64 = (s.db[173][15] * ddt_scale);
        let eq98_e1297_d_b16: f64 = (s.db[173][16] * ddt_scale);
        let eq98_e1297_d_b17: f64 = (s.db[173][17] * ddt_scale);
        let eq98_e1297_d_b18: f64 = (s.db[173][18] * ddt_scale);
        let eq98_e1297_d_b19: f64 = (s.db[173][19] * ddt_scale);
        let eq98_e1297_d_b20: f64 = (s.db[173][20] * ddt_scale);
        let eq98_e1297_d_b21: f64 = (s.db[173][21] * ddt_scale);
        let eq98_e1297_d_b22: f64 = (s.db[173][22] * ddt_scale);
        let eq98_e1297_d_b23: f64 = (s.db[173][23] * ddt_scale);
        let eq98_e1297_d_b24: f64 = (s.db[173][24] * ddt_scale);
        let eq98_e1297_d_b25: f64 = (s.db[173][25] * ddt_scale);
        let eq98_e1297_d_b26: f64 = (s.db[173][26] * ddt_scale);
        let eq98_e1297_d_b27: f64 = (s.db[173][27] * ddt_scale);
        let eq98_e1297_d_b28: f64 = (s.db[173][28] * ddt_scale);
        let eq98_e1297_d_b29: f64 = (s.db[173][29] * ddt_scale);
        let eq98_e1297_d_b30: f64 = (s.db[173][30] * ddt_scale);
        let eq98_e1297_d_b31: f64 = (s.db[173][31] * ddt_scale);
        let eq98_e1297_d_b32: f64 = (s.db[173][32] * ddt_scale);
        let eq98_e1297_d_b33: f64 = (s.db[173][33] * ddt_scale);
        let eq98_e1297_d_b34: f64 = (s.db[173][34] * ddt_scale);
        let eq98_e1297_d_b35: f64 = (s.db[173][35] * ddt_scale);
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1300_d_n7: f64 = p.p355;
        let eq98_e1300_d_n11: f64 = (-p.p355);
        let eq98_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 85, eq98_e1300);
        let eq98_e1301_d_n7: f64 = (eq98_e1300_d_n7 * ddt_scale);
        let eq98_e1301_d_n11: f64 = (eq98_e1300_d_n11 * ddt_scale);
        let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);
        let eq98_e1302_d_n7: f64 = (eq98_e1297_d_n7 + eq98_e1301_d_n7);
        let eq98_e1302_d_n11: f64 = (eq98_e1297_d_n11 + eq98_e1301_d_n11);
        (eq98_e1302, eq98_e1297_d_n0, eq98_e1297_d_n1, eq98_e1297_d_n2, eq98_e1297_d_n3, eq98_e1297_d_n4, eq98_e1297_d_n5, eq98_e1297_d_n6, eq98_e1302_d_n7, eq98_e1297_d_n8, eq98_e1297_d_n9, eq98_e1297_d_n10, eq98_e1302_d_n11, eq98_e1297_d_n12, eq98_e1297_d_n13, eq98_e1297_d_n14, eq98_e1297_d_n15, eq98_e1297_d_n16, eq98_e1297_d_n17, eq98_e1297_d_n18, eq98_e1297_d_n19, eq98_e1297_d_n20, eq98_e1297_d_n21, eq98_e1297_d_n22, eq98_e1297_d_n23, eq98_e1297_d_n24, eq98_e1297_d_n25, eq98_e1297_d_n26, eq98_e1297_d_n27, eq98_e1297_d_n28, eq98_e1297_d_n29, eq98_e1297_d_b0, eq98_e1297_d_b1, eq98_e1297_d_b2, eq98_e1297_d_b3, eq98_e1297_d_b4, eq98_e1297_d_b5, eq98_e1297_d_b6, eq98_e1297_d_b7, eq98_e1297_d_b8, eq98_e1297_d_b9, eq98_e1297_d_b10, eq98_e1297_d_b11, eq98_e1297_d_b12, eq98_e1297_d_b13, eq98_e1297_d_b14, eq98_e1297_d_b15, eq98_e1297_d_b16, eq98_e1297_d_b17, eq98_e1297_d_b18, eq98_e1297_d_b19, eq98_e1297_d_b20, eq98_e1297_d_b21, eq98_e1297_d_b22, eq98_e1297_d_b23, eq98_e1297_d_b24, eq98_e1297_d_b25, eq98_e1297_d_b26, eq98_e1297_d_b27, eq98_e1297_d_b28, eq98_e1297_d_b29, eq98_e1297_d_b30, eq98_e1297_d_b31, eq98_e1297_d_b32, eq98_e1297_d_b33, eq98_e1297_d_b34, eq98_e1297_d_b35,)
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
        let eq99_e1307_d_n0: f64 = (s.dn[174][0] * ddt_scale);
        let eq99_e1307_d_n1: f64 = (s.dn[174][1] * ddt_scale);
        let eq99_e1307_d_n2: f64 = (s.dn[174][2] * ddt_scale);
        let eq99_e1307_d_n3: f64 = (s.dn[174][3] * ddt_scale);
        let eq99_e1307_d_n4: f64 = (s.dn[174][4] * ddt_scale);
        let eq99_e1307_d_n5: f64 = (s.dn[174][5] * ddt_scale);
        let eq99_e1307_d_n6: f64 = (s.dn[174][6] * ddt_scale);
        let eq99_e1307_d_n7: f64 = (s.dn[174][7] * ddt_scale);
        let eq99_e1307_d_n8: f64 = (s.dn[174][8] * ddt_scale);
        let eq99_e1307_d_n9: f64 = (s.dn[174][9] * ddt_scale);
        let eq99_e1307_d_n10: f64 = (s.dn[174][10] * ddt_scale);
        let eq99_e1307_d_n11: f64 = (s.dn[174][11] * ddt_scale);
        let eq99_e1307_d_n12: f64 = (s.dn[174][12] * ddt_scale);
        let eq99_e1307_d_n13: f64 = (s.dn[174][13] * ddt_scale);
        let eq99_e1307_d_n14: f64 = (s.dn[174][14] * ddt_scale);
        let eq99_e1307_d_n15: f64 = (s.dn[174][15] * ddt_scale);
        let eq99_e1307_d_n16: f64 = (s.dn[174][16] * ddt_scale);
        let eq99_e1307_d_n17: f64 = (s.dn[174][17] * ddt_scale);
        let eq99_e1307_d_n18: f64 = (s.dn[174][18] * ddt_scale);
        let eq99_e1307_d_n19: f64 = (s.dn[174][19] * ddt_scale);
        let eq99_e1307_d_n20: f64 = (s.dn[174][20] * ddt_scale);
        let eq99_e1307_d_n21: f64 = (s.dn[174][21] * ddt_scale);
        let eq99_e1307_d_n22: f64 = (s.dn[174][22] * ddt_scale);
        let eq99_e1307_d_n23: f64 = (s.dn[174][23] * ddt_scale);
        let eq99_e1307_d_n24: f64 = (s.dn[174][24] * ddt_scale);
        let eq99_e1307_d_n25: f64 = (s.dn[174][25] * ddt_scale);
        let eq99_e1307_d_n26: f64 = (s.dn[174][26] * ddt_scale);
        let eq99_e1307_d_n27: f64 = (s.dn[174][27] * ddt_scale);
        let eq99_e1307_d_n28: f64 = (s.dn[174][28] * ddt_scale);
        let eq99_e1307_d_n29: f64 = (s.dn[174][29] * ddt_scale);
        let eq99_e1307_d_b0: f64 = (s.db[174][0] * ddt_scale);
        let eq99_e1307_d_b1: f64 = (s.db[174][1] * ddt_scale);
        let eq99_e1307_d_b2: f64 = (s.db[174][2] * ddt_scale);
        let eq99_e1307_d_b3: f64 = (s.db[174][3] * ddt_scale);
        let eq99_e1307_d_b4: f64 = (s.db[174][4] * ddt_scale);
        let eq99_e1307_d_b5: f64 = (s.db[174][5] * ddt_scale);
        let eq99_e1307_d_b6: f64 = (s.db[174][6] * ddt_scale);
        let eq99_e1307_d_b7: f64 = (s.db[174][7] * ddt_scale);
        let eq99_e1307_d_b8: f64 = (s.db[174][8] * ddt_scale);
        let eq99_e1307_d_b9: f64 = (s.db[174][9] * ddt_scale);
        let eq99_e1307_d_b10: f64 = (s.db[174][10] * ddt_scale);
        let eq99_e1307_d_b11: f64 = (s.db[174][11] * ddt_scale);
        let eq99_e1307_d_b12: f64 = (s.db[174][12] * ddt_scale);
        let eq99_e1307_d_b13: f64 = (s.db[174][13] * ddt_scale);
        let eq99_e1307_d_b14: f64 = (s.db[174][14] * ddt_scale);
        let eq99_e1307_d_b15: f64 = (s.db[174][15] * ddt_scale);
        let eq99_e1307_d_b16: f64 = (s.db[174][16] * ddt_scale);
        let eq99_e1307_d_b17: f64 = (s.db[174][17] * ddt_scale);
        let eq99_e1307_d_b18: f64 = (s.db[174][18] * ddt_scale);
        let eq99_e1307_d_b19: f64 = (s.db[174][19] * ddt_scale);
        let eq99_e1307_d_b20: f64 = (s.db[174][20] * ddt_scale);
        let eq99_e1307_d_b21: f64 = (s.db[174][21] * ddt_scale);
        let eq99_e1307_d_b22: f64 = (s.db[174][22] * ddt_scale);
        let eq99_e1307_d_b23: f64 = (s.db[174][23] * ddt_scale);
        let eq99_e1307_d_b24: f64 = (s.db[174][24] * ddt_scale);
        let eq99_e1307_d_b25: f64 = (s.db[174][25] * ddt_scale);
        let eq99_e1307_d_b26: f64 = (s.db[174][26] * ddt_scale);
        let eq99_e1307_d_b27: f64 = (s.db[174][27] * ddt_scale);
        let eq99_e1307_d_b28: f64 = (s.db[174][28] * ddt_scale);
        let eq99_e1307_d_b29: f64 = (s.db[174][29] * ddt_scale);
        let eq99_e1307_d_b30: f64 = (s.db[174][30] * ddt_scale);
        let eq99_e1307_d_b31: f64 = (s.db[174][31] * ddt_scale);
        let eq99_e1307_d_b32: f64 = (s.db[174][32] * ddt_scale);
        let eq99_e1307_d_b33: f64 = (s.db[174][33] * ddt_scale);
        let eq99_e1307_d_b34: f64 = (s.db[174][34] * ddt_scale);
        let eq99_e1307_d_b35: f64 = (s.db[174][35] * ddt_scale);
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1310_d_n7: f64 = p.p355;
        let eq99_e1310_d_n10: f64 = (-p.p355);
        let eq99_e1311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 87, eq99_e1310);
        let eq99_e1311_d_n7: f64 = (eq99_e1310_d_n7 * ddt_scale);
        let eq99_e1311_d_n10: f64 = (eq99_e1310_d_n10 * ddt_scale);
        let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);
        let eq99_e1312_d_n7: f64 = (eq99_e1307_d_n7 + eq99_e1311_d_n7);
        let eq99_e1312_d_n10: f64 = (eq99_e1307_d_n10 + eq99_e1311_d_n10);
        (eq99_e1312, eq99_e1307_d_n0, eq99_e1307_d_n1, eq99_e1307_d_n2, eq99_e1307_d_n3, eq99_e1307_d_n4, eq99_e1307_d_n5, eq99_e1307_d_n6, eq99_e1312_d_n7, eq99_e1307_d_n8, eq99_e1307_d_n9, eq99_e1312_d_n10, eq99_e1307_d_n11, eq99_e1307_d_n12, eq99_e1307_d_n13, eq99_e1307_d_n14, eq99_e1307_d_n15, eq99_e1307_d_n16, eq99_e1307_d_n17, eq99_e1307_d_n18, eq99_e1307_d_n19, eq99_e1307_d_n20, eq99_e1307_d_n21, eq99_e1307_d_n22, eq99_e1307_d_n23, eq99_e1307_d_n24, eq99_e1307_d_n25, eq99_e1307_d_n26, eq99_e1307_d_n27, eq99_e1307_d_n28, eq99_e1307_d_n29, eq99_e1307_d_b0, eq99_e1307_d_b1, eq99_e1307_d_b2, eq99_e1307_d_b3, eq99_e1307_d_b4, eq99_e1307_d_b5, eq99_e1307_d_b6, eq99_e1307_d_b7, eq99_e1307_d_b8, eq99_e1307_d_b9, eq99_e1307_d_b10, eq99_e1307_d_b11, eq99_e1307_d_b12, eq99_e1307_d_b13, eq99_e1307_d_b14, eq99_e1307_d_b15, eq99_e1307_d_b16, eq99_e1307_d_b17, eq99_e1307_d_b18, eq99_e1307_d_b19, eq99_e1307_d_b20, eq99_e1307_d_b21, eq99_e1307_d_b22, eq99_e1307_d_b23, eq99_e1307_d_b24, eq99_e1307_d_b25, eq99_e1307_d_b26, eq99_e1307_d_b27, eq99_e1307_d_b28, eq99_e1307_d_b29, eq99_e1307_d_b30, eq99_e1307_d_b31, eq99_e1307_d_b32, eq99_e1307_d_b33, eq99_e1307_d_b34, eq99_e1307_d_b35,)
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
        let eq100_e1317_d_n0: f64 = (s.dn[175][0] * ddt_scale);
        let eq100_e1317_d_n1: f64 = (s.dn[175][1] * ddt_scale);
        let eq100_e1317_d_n2: f64 = (s.dn[175][2] * ddt_scale);
        let eq100_e1317_d_n3: f64 = (s.dn[175][3] * ddt_scale);
        let eq100_e1317_d_n4: f64 = (s.dn[175][4] * ddt_scale);
        let eq100_e1317_d_n5: f64 = (s.dn[175][5] * ddt_scale);
        let eq100_e1317_d_n6: f64 = (s.dn[175][6] * ddt_scale);
        let eq100_e1317_d_n7: f64 = (s.dn[175][7] * ddt_scale);
        let eq100_e1317_d_n8: f64 = (s.dn[175][8] * ddt_scale);
        let eq100_e1317_d_n9: f64 = (s.dn[175][9] * ddt_scale);
        let eq100_e1317_d_n10: f64 = (s.dn[175][10] * ddt_scale);
        let eq100_e1317_d_n11: f64 = (s.dn[175][11] * ddt_scale);
        let eq100_e1317_d_n12: f64 = (s.dn[175][12] * ddt_scale);
        let eq100_e1317_d_n13: f64 = (s.dn[175][13] * ddt_scale);
        let eq100_e1317_d_n14: f64 = (s.dn[175][14] * ddt_scale);
        let eq100_e1317_d_n15: f64 = (s.dn[175][15] * ddt_scale);
        let eq100_e1317_d_n16: f64 = (s.dn[175][16] * ddt_scale);
        let eq100_e1317_d_n17: f64 = (s.dn[175][17] * ddt_scale);
        let eq100_e1317_d_n18: f64 = (s.dn[175][18] * ddt_scale);
        let eq100_e1317_d_n19: f64 = (s.dn[175][19] * ddt_scale);
        let eq100_e1317_d_n20: f64 = (s.dn[175][20] * ddt_scale);
        let eq100_e1317_d_n21: f64 = (s.dn[175][21] * ddt_scale);
        let eq100_e1317_d_n22: f64 = (s.dn[175][22] * ddt_scale);
        let eq100_e1317_d_n23: f64 = (s.dn[175][23] * ddt_scale);
        let eq100_e1317_d_n24: f64 = (s.dn[175][24] * ddt_scale);
        let eq100_e1317_d_n25: f64 = (s.dn[175][25] * ddt_scale);
        let eq100_e1317_d_n26: f64 = (s.dn[175][26] * ddt_scale);
        let eq100_e1317_d_n27: f64 = (s.dn[175][27] * ddt_scale);
        let eq100_e1317_d_n28: f64 = (s.dn[175][28] * ddt_scale);
        let eq100_e1317_d_n29: f64 = (s.dn[175][29] * ddt_scale);
        let eq100_e1317_d_b0: f64 = (s.db[175][0] * ddt_scale);
        let eq100_e1317_d_b1: f64 = (s.db[175][1] * ddt_scale);
        let eq100_e1317_d_b2: f64 = (s.db[175][2] * ddt_scale);
        let eq100_e1317_d_b3: f64 = (s.db[175][3] * ddt_scale);
        let eq100_e1317_d_b4: f64 = (s.db[175][4] * ddt_scale);
        let eq100_e1317_d_b5: f64 = (s.db[175][5] * ddt_scale);
        let eq100_e1317_d_b6: f64 = (s.db[175][6] * ddt_scale);
        let eq100_e1317_d_b7: f64 = (s.db[175][7] * ddt_scale);
        let eq100_e1317_d_b8: f64 = (s.db[175][8] * ddt_scale);
        let eq100_e1317_d_b9: f64 = (s.db[175][9] * ddt_scale);
        let eq100_e1317_d_b10: f64 = (s.db[175][10] * ddt_scale);
        let eq100_e1317_d_b11: f64 = (s.db[175][11] * ddt_scale);
        let eq100_e1317_d_b12: f64 = (s.db[175][12] * ddt_scale);
        let eq100_e1317_d_b13: f64 = (s.db[175][13] * ddt_scale);
        let eq100_e1317_d_b14: f64 = (s.db[175][14] * ddt_scale);
        let eq100_e1317_d_b15: f64 = (s.db[175][15] * ddt_scale);
        let eq100_e1317_d_b16: f64 = (s.db[175][16] * ddt_scale);
        let eq100_e1317_d_b17: f64 = (s.db[175][17] * ddt_scale);
        let eq100_e1317_d_b18: f64 = (s.db[175][18] * ddt_scale);
        let eq100_e1317_d_b19: f64 = (s.db[175][19] * ddt_scale);
        let eq100_e1317_d_b20: f64 = (s.db[175][20] * ddt_scale);
        let eq100_e1317_d_b21: f64 = (s.db[175][21] * ddt_scale);
        let eq100_e1317_d_b22: f64 = (s.db[175][22] * ddt_scale);
        let eq100_e1317_d_b23: f64 = (s.db[175][23] * ddt_scale);
        let eq100_e1317_d_b24: f64 = (s.db[175][24] * ddt_scale);
        let eq100_e1317_d_b25: f64 = (s.db[175][25] * ddt_scale);
        let eq100_e1317_d_b26: f64 = (s.db[175][26] * ddt_scale);
        let eq100_e1317_d_b27: f64 = (s.db[175][27] * ddt_scale);
        let eq100_e1317_d_b28: f64 = (s.db[175][28] * ddt_scale);
        let eq100_e1317_d_b29: f64 = (s.db[175][29] * ddt_scale);
        let eq100_e1317_d_b30: f64 = (s.db[175][30] * ddt_scale);
        let eq100_e1317_d_b31: f64 = (s.db[175][31] * ddt_scale);
        let eq100_e1317_d_b32: f64 = (s.db[175][32] * ddt_scale);
        let eq100_e1317_d_b33: f64 = (s.db[175][33] * ddt_scale);
        let eq100_e1317_d_b34: f64 = (s.db[175][34] * ddt_scale);
        let eq100_e1317_d_b35: f64 = (s.db[175][35] * ddt_scale);
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1320_d_n2: f64 = p.p355;
        let eq100_e1320_d_n11: f64 = (-p.p355);
        let eq100_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 89, eq100_e1320);
        let eq100_e1321_d_n2: f64 = (eq100_e1320_d_n2 * ddt_scale);
        let eq100_e1321_d_n11: f64 = (eq100_e1320_d_n11 * ddt_scale);
        let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);
        let eq100_e1322_d_n2: f64 = (eq100_e1317_d_n2 + eq100_e1321_d_n2);
        let eq100_e1322_d_n11: f64 = (eq100_e1317_d_n11 + eq100_e1321_d_n11);
        (eq100_e1322, eq100_e1317_d_n0, eq100_e1317_d_n1, eq100_e1322_d_n2, eq100_e1317_d_n3, eq100_e1317_d_n4, eq100_e1317_d_n5, eq100_e1317_d_n6, eq100_e1317_d_n7, eq100_e1317_d_n8, eq100_e1317_d_n9, eq100_e1317_d_n10, eq100_e1322_d_n11, eq100_e1317_d_n12, eq100_e1317_d_n13, eq100_e1317_d_n14, eq100_e1317_d_n15, eq100_e1317_d_n16, eq100_e1317_d_n17, eq100_e1317_d_n18, eq100_e1317_d_n19, eq100_e1317_d_n20, eq100_e1317_d_n21, eq100_e1317_d_n22, eq100_e1317_d_n23, eq100_e1317_d_n24, eq100_e1317_d_n25, eq100_e1317_d_n26, eq100_e1317_d_n27, eq100_e1317_d_n28, eq100_e1317_d_n29, eq100_e1317_d_b0, eq100_e1317_d_b1, eq100_e1317_d_b2, eq100_e1317_d_b3, eq100_e1317_d_b4, eq100_e1317_d_b5, eq100_e1317_d_b6, eq100_e1317_d_b7, eq100_e1317_d_b8, eq100_e1317_d_b9, eq100_e1317_d_b10, eq100_e1317_d_b11, eq100_e1317_d_b12, eq100_e1317_d_b13, eq100_e1317_d_b14, eq100_e1317_d_b15, eq100_e1317_d_b16, eq100_e1317_d_b17, eq100_e1317_d_b18, eq100_e1317_d_b19, eq100_e1317_d_b20, eq100_e1317_d_b21, eq100_e1317_d_b22, eq100_e1317_d_b23, eq100_e1317_d_b24, eq100_e1317_d_b25, eq100_e1317_d_b26, eq100_e1317_d_b27, eq100_e1317_d_b28, eq100_e1317_d_b29, eq100_e1317_d_b30, eq100_e1317_d_b31, eq100_e1317_d_b32, eq100_e1317_d_b33, eq100_e1317_d_b34, eq100_e1317_d_b35,)
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
        let (eq101_e1328,) = {
    if s.b[1201] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1328;
        stamper.stamp_current_const_local(
            Some(2),
            Some(10),
            multiplicity * (eq101_value),
        );
        let (eq102_e1338, eq102_e1338_d_n0, eq102_e1338_d_n1, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n5, eq102_e1338_d_n6, eq102_e1338_d_n7, eq102_e1338_d_n8, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11, eq102_e1338_d_n12, eq102_e1338_d_n13, eq102_e1338_d_n14, eq102_e1338_d_n15, eq102_e1338_d_n16, eq102_e1338_d_n17, eq102_e1338_d_n18, eq102_e1338_d_n19, eq102_e1338_d_n20, eq102_e1338_d_n21, eq102_e1338_d_n22, eq102_e1338_d_n23, eq102_e1338_d_n24, eq102_e1338_d_n25, eq102_e1338_d_n26, eq102_e1338_d_n27, eq102_e1338_d_n28, eq102_e1338_d_n29, eq102_e1338_d_b0, eq102_e1338_d_b1, eq102_e1338_d_b2, eq102_e1338_d_b3, eq102_e1338_d_b4, eq102_e1338_d_b5, eq102_e1338_d_b6, eq102_e1338_d_b7, eq102_e1338_d_b8, eq102_e1338_d_b9, eq102_e1338_d_b10, eq102_e1338_d_b11, eq102_e1338_d_b12, eq102_e1338_d_b13, eq102_e1338_d_b14, eq102_e1338_d_b15, eq102_e1338_d_b16, eq102_e1338_d_b17, eq102_e1338_d_b18, eq102_e1338_d_b19, eq102_e1338_d_b20, eq102_e1338_d_b21, eq102_e1338_d_b22, eq102_e1338_d_b23, eq102_e1338_d_b24, eq102_e1338_d_b25, eq102_e1338_d_b26, eq102_e1338_d_b27, eq102_e1338_d_b28, eq102_e1338_d_b29, eq102_e1338_d_b30, eq102_e1338_d_b31, eq102_e1338_d_b32, eq102_e1338_d_b33, eq102_e1338_d_b34, eq102_e1338_d_b35,) = {
    if s.b[1201] {
        let eq102_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 90, s.v[177]);
        let eq102_e1331_d_n0: f64 = (s.dn[177][0] * ddt_scale);
        let eq102_e1331_d_n1: f64 = (s.dn[177][1] * ddt_scale);
        let eq102_e1331_d_n2: f64 = (s.dn[177][2] * ddt_scale);
        let eq102_e1331_d_n3: f64 = (s.dn[177][3] * ddt_scale);
        let eq102_e1331_d_n4: f64 = (s.dn[177][4] * ddt_scale);
        let eq102_e1331_d_n5: f64 = (s.dn[177][5] * ddt_scale);
        let eq102_e1331_d_n6: f64 = (s.dn[177][6] * ddt_scale);
        let eq102_e1331_d_n7: f64 = (s.dn[177][7] * ddt_scale);
        let eq102_e1331_d_n8: f64 = (s.dn[177][8] * ddt_scale);
        let eq102_e1331_d_n9: f64 = (s.dn[177][9] * ddt_scale);
        let eq102_e1331_d_n10: f64 = (s.dn[177][10] * ddt_scale);
        let eq102_e1331_d_n11: f64 = (s.dn[177][11] * ddt_scale);
        let eq102_e1331_d_n12: f64 = (s.dn[177][12] * ddt_scale);
        let eq102_e1331_d_n13: f64 = (s.dn[177][13] * ddt_scale);
        let eq102_e1331_d_n14: f64 = (s.dn[177][14] * ddt_scale);
        let eq102_e1331_d_n15: f64 = (s.dn[177][15] * ddt_scale);
        let eq102_e1331_d_n16: f64 = (s.dn[177][16] * ddt_scale);
        let eq102_e1331_d_n17: f64 = (s.dn[177][17] * ddt_scale);
        let eq102_e1331_d_n18: f64 = (s.dn[177][18] * ddt_scale);
        let eq102_e1331_d_n19: f64 = (s.dn[177][19] * ddt_scale);
        let eq102_e1331_d_n20: f64 = (s.dn[177][20] * ddt_scale);
        let eq102_e1331_d_n21: f64 = (s.dn[177][21] * ddt_scale);
        let eq102_e1331_d_n22: f64 = (s.dn[177][22] * ddt_scale);
        let eq102_e1331_d_n23: f64 = (s.dn[177][23] * ddt_scale);
        let eq102_e1331_d_n24: f64 = (s.dn[177][24] * ddt_scale);
        let eq102_e1331_d_n25: f64 = (s.dn[177][25] * ddt_scale);
        let eq102_e1331_d_n26: f64 = (s.dn[177][26] * ddt_scale);
        let eq102_e1331_d_n27: f64 = (s.dn[177][27] * ddt_scale);
        let eq102_e1331_d_n28: f64 = (s.dn[177][28] * ddt_scale);
        let eq102_e1331_d_n29: f64 = (s.dn[177][29] * ddt_scale);
        let eq102_e1331_d_b0: f64 = (s.db[177][0] * ddt_scale);
        let eq102_e1331_d_b1: f64 = (s.db[177][1] * ddt_scale);
        let eq102_e1331_d_b2: f64 = (s.db[177][2] * ddt_scale);
        let eq102_e1331_d_b3: f64 = (s.db[177][3] * ddt_scale);
        let eq102_e1331_d_b4: f64 = (s.db[177][4] * ddt_scale);
        let eq102_e1331_d_b5: f64 = (s.db[177][5] * ddt_scale);
        let eq102_e1331_d_b6: f64 = (s.db[177][6] * ddt_scale);
        let eq102_e1331_d_b7: f64 = (s.db[177][7] * ddt_scale);
        let eq102_e1331_d_b8: f64 = (s.db[177][8] * ddt_scale);
        let eq102_e1331_d_b9: f64 = (s.db[177][9] * ddt_scale);
        let eq102_e1331_d_b10: f64 = (s.db[177][10] * ddt_scale);
        let eq102_e1331_d_b11: f64 = (s.db[177][11] * ddt_scale);
        let eq102_e1331_d_b12: f64 = (s.db[177][12] * ddt_scale);
        let eq102_e1331_d_b13: f64 = (s.db[177][13] * ddt_scale);
        let eq102_e1331_d_b14: f64 = (s.db[177][14] * ddt_scale);
        let eq102_e1331_d_b15: f64 = (s.db[177][15] * ddt_scale);
        let eq102_e1331_d_b16: f64 = (s.db[177][16] * ddt_scale);
        let eq102_e1331_d_b17: f64 = (s.db[177][17] * ddt_scale);
        let eq102_e1331_d_b18: f64 = (s.db[177][18] * ddt_scale);
        let eq102_e1331_d_b19: f64 = (s.db[177][19] * ddt_scale);
        let eq102_e1331_d_b20: f64 = (s.db[177][20] * ddt_scale);
        let eq102_e1331_d_b21: f64 = (s.db[177][21] * ddt_scale);
        let eq102_e1331_d_b22: f64 = (s.db[177][22] * ddt_scale);
        let eq102_e1331_d_b23: f64 = (s.db[177][23] * ddt_scale);
        let eq102_e1331_d_b24: f64 = (s.db[177][24] * ddt_scale);
        let eq102_e1331_d_b25: f64 = (s.db[177][25] * ddt_scale);
        let eq102_e1331_d_b26: f64 = (s.db[177][26] * ddt_scale);
        let eq102_e1331_d_b27: f64 = (s.db[177][27] * ddt_scale);
        let eq102_e1331_d_b28: f64 = (s.db[177][28] * ddt_scale);
        let eq102_e1331_d_b29: f64 = (s.db[177][29] * ddt_scale);
        let eq102_e1331_d_b30: f64 = (s.db[177][30] * ddt_scale);
        let eq102_e1331_d_b31: f64 = (s.db[177][31] * ddt_scale);
        let eq102_e1331_d_b32: f64 = (s.db[177][32] * ddt_scale);
        let eq102_e1331_d_b33: f64 = (s.db[177][33] * ddt_scale);
        let eq102_e1331_d_b34: f64 = (s.db[177][34] * ddt_scale);
        let eq102_e1331_d_b35: f64 = (s.db[177][35] * ddt_scale);
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1334_d_n7: f64 = p.p355;
        let eq102_e1334_d_n9: f64 = (-p.p355);
        let eq102_e1335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 91, eq102_e1334);
        let eq102_e1335_d_n7: f64 = (eq102_e1334_d_n7 * ddt_scale);
        let eq102_e1335_d_n9: f64 = (eq102_e1334_d_n9 * ddt_scale);
        let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);
        let eq102_e1336_d_n7: f64 = (eq102_e1331_d_n7 + eq102_e1335_d_n7);
        let eq102_e1336_d_n9: f64 = (eq102_e1331_d_n9 + eq102_e1335_d_n9);
        (eq102_e1336, eq102_e1331_d_n0, eq102_e1331_d_n1, eq102_e1331_d_n2, eq102_e1331_d_n3, eq102_e1331_d_n4, eq102_e1331_d_n5, eq102_e1331_d_n6, eq102_e1336_d_n7, eq102_e1331_d_n8, eq102_e1336_d_n9, eq102_e1331_d_n10, eq102_e1331_d_n11, eq102_e1331_d_n12, eq102_e1331_d_n13, eq102_e1331_d_n14, eq102_e1331_d_n15, eq102_e1331_d_n16, eq102_e1331_d_n17, eq102_e1331_d_n18, eq102_e1331_d_n19, eq102_e1331_d_n20, eq102_e1331_d_n21, eq102_e1331_d_n22, eq102_e1331_d_n23, eq102_e1331_d_n24, eq102_e1331_d_n25, eq102_e1331_d_n26, eq102_e1331_d_n27, eq102_e1331_d_n28, eq102_e1331_d_n29, eq102_e1331_d_b0, eq102_e1331_d_b1, eq102_e1331_d_b2, eq102_e1331_d_b3, eq102_e1331_d_b4, eq102_e1331_d_b5, eq102_e1331_d_b6, eq102_e1331_d_b7, eq102_e1331_d_b8, eq102_e1331_d_b9, eq102_e1331_d_b10, eq102_e1331_d_b11, eq102_e1331_d_b12, eq102_e1331_d_b13, eq102_e1331_d_b14, eq102_e1331_d_b15, eq102_e1331_d_b16, eq102_e1331_d_b17, eq102_e1331_d_b18, eq102_e1331_d_b19, eq102_e1331_d_b20, eq102_e1331_d_b21, eq102_e1331_d_b22, eq102_e1331_d_b23, eq102_e1331_d_b24, eq102_e1331_d_b25, eq102_e1331_d_b26, eq102_e1331_d_b27, eq102_e1331_d_b28, eq102_e1331_d_b29, eq102_e1331_d_b30, eq102_e1331_d_b31, eq102_e1331_d_b32, eq102_e1331_d_b33, eq102_e1331_d_b34, eq102_e1331_d_b35,)
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
        let eq103_e1342_d_n0: f64 = (s.dn[173][0] * ddt_scale);
        let eq103_e1342_d_n1: f64 = (s.dn[173][1] * ddt_scale);
        let eq103_e1342_d_n2: f64 = (s.dn[173][2] * ddt_scale);
        let eq103_e1342_d_n3: f64 = (s.dn[173][3] * ddt_scale);
        let eq103_e1342_d_n4: f64 = (s.dn[173][4] * ddt_scale);
        let eq103_e1342_d_n5: f64 = (s.dn[173][5] * ddt_scale);
        let eq103_e1342_d_n6: f64 = (s.dn[173][6] * ddt_scale);
        let eq103_e1342_d_n7: f64 = (s.dn[173][7] * ddt_scale);
        let eq103_e1342_d_n8: f64 = (s.dn[173][8] * ddt_scale);
        let eq103_e1342_d_n9: f64 = (s.dn[173][9] * ddt_scale);
        let eq103_e1342_d_n10: f64 = (s.dn[173][10] * ddt_scale);
        let eq103_e1342_d_n11: f64 = (s.dn[173][11] * ddt_scale);
        let eq103_e1342_d_n12: f64 = (s.dn[173][12] * ddt_scale);
        let eq103_e1342_d_n13: f64 = (s.dn[173][13] * ddt_scale);
        let eq103_e1342_d_n14: f64 = (s.dn[173][14] * ddt_scale);
        let eq103_e1342_d_n15: f64 = (s.dn[173][15] * ddt_scale);
        let eq103_e1342_d_n16: f64 = (s.dn[173][16] * ddt_scale);
        let eq103_e1342_d_n17: f64 = (s.dn[173][17] * ddt_scale);
        let eq103_e1342_d_n18: f64 = (s.dn[173][18] * ddt_scale);
        let eq103_e1342_d_n19: f64 = (s.dn[173][19] * ddt_scale);
        let eq103_e1342_d_n20: f64 = (s.dn[173][20] * ddt_scale);
        let eq103_e1342_d_n21: f64 = (s.dn[173][21] * ddt_scale);
        let eq103_e1342_d_n22: f64 = (s.dn[173][22] * ddt_scale);
        let eq103_e1342_d_n23: f64 = (s.dn[173][23] * ddt_scale);
        let eq103_e1342_d_n24: f64 = (s.dn[173][24] * ddt_scale);
        let eq103_e1342_d_n25: f64 = (s.dn[173][25] * ddt_scale);
        let eq103_e1342_d_n26: f64 = (s.dn[173][26] * ddt_scale);
        let eq103_e1342_d_n27: f64 = (s.dn[173][27] * ddt_scale);
        let eq103_e1342_d_n28: f64 = (s.dn[173][28] * ddt_scale);
        let eq103_e1342_d_n29: f64 = (s.dn[173][29] * ddt_scale);
        let eq103_e1342_d_b0: f64 = (s.db[173][0] * ddt_scale);
        let eq103_e1342_d_b1: f64 = (s.db[173][1] * ddt_scale);
        let eq103_e1342_d_b2: f64 = (s.db[173][2] * ddt_scale);
        let eq103_e1342_d_b3: f64 = (s.db[173][3] * ddt_scale);
        let eq103_e1342_d_b4: f64 = (s.db[173][4] * ddt_scale);
        let eq103_e1342_d_b5: f64 = (s.db[173][5] * ddt_scale);
        let eq103_e1342_d_b6: f64 = (s.db[173][6] * ddt_scale);
        let eq103_e1342_d_b7: f64 = (s.db[173][7] * ddt_scale);
        let eq103_e1342_d_b8: f64 = (s.db[173][8] * ddt_scale);
        let eq103_e1342_d_b9: f64 = (s.db[173][9] * ddt_scale);
        let eq103_e1342_d_b10: f64 = (s.db[173][10] * ddt_scale);
        let eq103_e1342_d_b11: f64 = (s.db[173][11] * ddt_scale);
        let eq103_e1342_d_b12: f64 = (s.db[173][12] * ddt_scale);
        let eq103_e1342_d_b13: f64 = (s.db[173][13] * ddt_scale);
        let eq103_e1342_d_b14: f64 = (s.db[173][14] * ddt_scale);
        let eq103_e1342_d_b15: f64 = (s.db[173][15] * ddt_scale);
        let eq103_e1342_d_b16: f64 = (s.db[173][16] * ddt_scale);
        let eq103_e1342_d_b17: f64 = (s.db[173][17] * ddt_scale);
        let eq103_e1342_d_b18: f64 = (s.db[173][18] * ddt_scale);
        let eq103_e1342_d_b19: f64 = (s.db[173][19] * ddt_scale);
        let eq103_e1342_d_b20: f64 = (s.db[173][20] * ddt_scale);
        let eq103_e1342_d_b21: f64 = (s.db[173][21] * ddt_scale);
        let eq103_e1342_d_b22: f64 = (s.db[173][22] * ddt_scale);
        let eq103_e1342_d_b23: f64 = (s.db[173][23] * ddt_scale);
        let eq103_e1342_d_b24: f64 = (s.db[173][24] * ddt_scale);
        let eq103_e1342_d_b25: f64 = (s.db[173][25] * ddt_scale);
        let eq103_e1342_d_b26: f64 = (s.db[173][26] * ddt_scale);
        let eq103_e1342_d_b27: f64 = (s.db[173][27] * ddt_scale);
        let eq103_e1342_d_b28: f64 = (s.db[173][28] * ddt_scale);
        let eq103_e1342_d_b29: f64 = (s.db[173][29] * ddt_scale);
        let eq103_e1342_d_b30: f64 = (s.db[173][30] * ddt_scale);
        let eq103_e1342_d_b31: f64 = (s.db[173][31] * ddt_scale);
        let eq103_e1342_d_b32: f64 = (s.db[173][32] * ddt_scale);
        let eq103_e1342_d_b33: f64 = (s.db[173][33] * ddt_scale);
        let eq103_e1342_d_b34: f64 = (s.db[173][34] * ddt_scale);
        let eq103_e1342_d_b35: f64 = (s.db[173][35] * ddt_scale);
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1345_d_n2: f64 = p.p355;
        let eq103_e1345_d_n11: f64 = (-p.p355);
        let eq103_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 93, eq103_e1345);
        let eq103_e1346_d_n2: f64 = (eq103_e1345_d_n2 * ddt_scale);
        let eq103_e1346_d_n11: f64 = (eq103_e1345_d_n11 * ddt_scale);
        let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);
        let eq103_e1347_d_n2: f64 = (eq103_e1342_d_n2 + eq103_e1346_d_n2);
        let eq103_e1347_d_n11: f64 = (eq103_e1342_d_n11 + eq103_e1346_d_n11);
        (eq103_e1347, eq103_e1342_d_n0, eq103_e1342_d_n1, eq103_e1347_d_n2, eq103_e1342_d_n3, eq103_e1342_d_n4, eq103_e1342_d_n5, eq103_e1342_d_n6, eq103_e1342_d_n7, eq103_e1342_d_n8, eq103_e1342_d_n9, eq103_e1342_d_n10, eq103_e1347_d_n11, eq103_e1342_d_n12, eq103_e1342_d_n13, eq103_e1342_d_n14, eq103_e1342_d_n15, eq103_e1342_d_n16, eq103_e1342_d_n17, eq103_e1342_d_n18, eq103_e1342_d_n19, eq103_e1342_d_n20, eq103_e1342_d_n21, eq103_e1342_d_n22, eq103_e1342_d_n23, eq103_e1342_d_n24, eq103_e1342_d_n25, eq103_e1342_d_n26, eq103_e1342_d_n27, eq103_e1342_d_n28, eq103_e1342_d_n29, eq103_e1342_d_b0, eq103_e1342_d_b1, eq103_e1342_d_b2, eq103_e1342_d_b3, eq103_e1342_d_b4, eq103_e1342_d_b5, eq103_e1342_d_b6, eq103_e1342_d_b7, eq103_e1342_d_b8, eq103_e1342_d_b9, eq103_e1342_d_b10, eq103_e1342_d_b11, eq103_e1342_d_b12, eq103_e1342_d_b13, eq103_e1342_d_b14, eq103_e1342_d_b15, eq103_e1342_d_b16, eq103_e1342_d_b17, eq103_e1342_d_b18, eq103_e1342_d_b19, eq103_e1342_d_b20, eq103_e1342_d_b21, eq103_e1342_d_b22, eq103_e1342_d_b23, eq103_e1342_d_b24, eq103_e1342_d_b25, eq103_e1342_d_b26, eq103_e1342_d_b27, eq103_e1342_d_b28, eq103_e1342_d_b29, eq103_e1342_d_b30, eq103_e1342_d_b31, eq103_e1342_d_b32, eq103_e1342_d_b33, eq103_e1342_d_b34, eq103_e1342_d_b35,)
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
    }

    pub(super) fn stamp_transient_equations_block_14(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq104_e1360, eq104_e1360_d_n0, eq104_e1360_d_n1, eq104_e1360_d_n2, eq104_e1360_d_n3, eq104_e1360_d_n4, eq104_e1360_d_n5, eq104_e1360_d_n6, eq104_e1360_d_n7, eq104_e1360_d_n8, eq104_e1360_d_n9, eq104_e1360_d_n10, eq104_e1360_d_n11, eq104_e1360_d_n12, eq104_e1360_d_n13, eq104_e1360_d_n14, eq104_e1360_d_n15, eq104_e1360_d_n16, eq104_e1360_d_n17, eq104_e1360_d_n18, eq104_e1360_d_n19, eq104_e1360_d_n20, eq104_e1360_d_n21, eq104_e1360_d_n22, eq104_e1360_d_n23, eq104_e1360_d_n24, eq104_e1360_d_n25, eq104_e1360_d_n26, eq104_e1360_d_n27, eq104_e1360_d_n28, eq104_e1360_d_n29, eq104_e1360_d_b0, eq104_e1360_d_b1, eq104_e1360_d_b2, eq104_e1360_d_b3, eq104_e1360_d_b4, eq104_e1360_d_b5, eq104_e1360_d_b6, eq104_e1360_d_b7, eq104_e1360_d_b8, eq104_e1360_d_b9, eq104_e1360_d_b10, eq104_e1360_d_b11, eq104_e1360_d_b12, eq104_e1360_d_b13, eq104_e1360_d_b14, eq104_e1360_d_b15, eq104_e1360_d_b16, eq104_e1360_d_b17, eq104_e1360_d_b18, eq104_e1360_d_b19, eq104_e1360_d_b20, eq104_e1360_d_b21, eq104_e1360_d_b22, eq104_e1360_d_b23, eq104_e1360_d_b24, eq104_e1360_d_b25, eq104_e1360_d_b26, eq104_e1360_d_b27, eq104_e1360_d_b28, eq104_e1360_d_b29, eq104_e1360_d_b30, eq104_e1360_d_b31, eq104_e1360_d_b32, eq104_e1360_d_b33, eq104_e1360_d_b34, eq104_e1360_d_b35,) = {
    if (!s.b[1201]) {
        let eq104_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 94, s.v[174]);
        let eq104_e1353_d_n0: f64 = (s.dn[174][0] * ddt_scale);
        let eq104_e1353_d_n1: f64 = (s.dn[174][1] * ddt_scale);
        let eq104_e1353_d_n2: f64 = (s.dn[174][2] * ddt_scale);
        let eq104_e1353_d_n3: f64 = (s.dn[174][3] * ddt_scale);
        let eq104_e1353_d_n4: f64 = (s.dn[174][4] * ddt_scale);
        let eq104_e1353_d_n5: f64 = (s.dn[174][5] * ddt_scale);
        let eq104_e1353_d_n6: f64 = (s.dn[174][6] * ddt_scale);
        let eq104_e1353_d_n7: f64 = (s.dn[174][7] * ddt_scale);
        let eq104_e1353_d_n8: f64 = (s.dn[174][8] * ddt_scale);
        let eq104_e1353_d_n9: f64 = (s.dn[174][9] * ddt_scale);
        let eq104_e1353_d_n10: f64 = (s.dn[174][10] * ddt_scale);
        let eq104_e1353_d_n11: f64 = (s.dn[174][11] * ddt_scale);
        let eq104_e1353_d_n12: f64 = (s.dn[174][12] * ddt_scale);
        let eq104_e1353_d_n13: f64 = (s.dn[174][13] * ddt_scale);
        let eq104_e1353_d_n14: f64 = (s.dn[174][14] * ddt_scale);
        let eq104_e1353_d_n15: f64 = (s.dn[174][15] * ddt_scale);
        let eq104_e1353_d_n16: f64 = (s.dn[174][16] * ddt_scale);
        let eq104_e1353_d_n17: f64 = (s.dn[174][17] * ddt_scale);
        let eq104_e1353_d_n18: f64 = (s.dn[174][18] * ddt_scale);
        let eq104_e1353_d_n19: f64 = (s.dn[174][19] * ddt_scale);
        let eq104_e1353_d_n20: f64 = (s.dn[174][20] * ddt_scale);
        let eq104_e1353_d_n21: f64 = (s.dn[174][21] * ddt_scale);
        let eq104_e1353_d_n22: f64 = (s.dn[174][22] * ddt_scale);
        let eq104_e1353_d_n23: f64 = (s.dn[174][23] * ddt_scale);
        let eq104_e1353_d_n24: f64 = (s.dn[174][24] * ddt_scale);
        let eq104_e1353_d_n25: f64 = (s.dn[174][25] * ddt_scale);
        let eq104_e1353_d_n26: f64 = (s.dn[174][26] * ddt_scale);
        let eq104_e1353_d_n27: f64 = (s.dn[174][27] * ddt_scale);
        let eq104_e1353_d_n28: f64 = (s.dn[174][28] * ddt_scale);
        let eq104_e1353_d_n29: f64 = (s.dn[174][29] * ddt_scale);
        let eq104_e1353_d_b0: f64 = (s.db[174][0] * ddt_scale);
        let eq104_e1353_d_b1: f64 = (s.db[174][1] * ddt_scale);
        let eq104_e1353_d_b2: f64 = (s.db[174][2] * ddt_scale);
        let eq104_e1353_d_b3: f64 = (s.db[174][3] * ddt_scale);
        let eq104_e1353_d_b4: f64 = (s.db[174][4] * ddt_scale);
        let eq104_e1353_d_b5: f64 = (s.db[174][5] * ddt_scale);
        let eq104_e1353_d_b6: f64 = (s.db[174][6] * ddt_scale);
        let eq104_e1353_d_b7: f64 = (s.db[174][7] * ddt_scale);
        let eq104_e1353_d_b8: f64 = (s.db[174][8] * ddt_scale);
        let eq104_e1353_d_b9: f64 = (s.db[174][9] * ddt_scale);
        let eq104_e1353_d_b10: f64 = (s.db[174][10] * ddt_scale);
        let eq104_e1353_d_b11: f64 = (s.db[174][11] * ddt_scale);
        let eq104_e1353_d_b12: f64 = (s.db[174][12] * ddt_scale);
        let eq104_e1353_d_b13: f64 = (s.db[174][13] * ddt_scale);
        let eq104_e1353_d_b14: f64 = (s.db[174][14] * ddt_scale);
        let eq104_e1353_d_b15: f64 = (s.db[174][15] * ddt_scale);
        let eq104_e1353_d_b16: f64 = (s.db[174][16] * ddt_scale);
        let eq104_e1353_d_b17: f64 = (s.db[174][17] * ddt_scale);
        let eq104_e1353_d_b18: f64 = (s.db[174][18] * ddt_scale);
        let eq104_e1353_d_b19: f64 = (s.db[174][19] * ddt_scale);
        let eq104_e1353_d_b20: f64 = (s.db[174][20] * ddt_scale);
        let eq104_e1353_d_b21: f64 = (s.db[174][21] * ddt_scale);
        let eq104_e1353_d_b22: f64 = (s.db[174][22] * ddt_scale);
        let eq104_e1353_d_b23: f64 = (s.db[174][23] * ddt_scale);
        let eq104_e1353_d_b24: f64 = (s.db[174][24] * ddt_scale);
        let eq104_e1353_d_b25: f64 = (s.db[174][25] * ddt_scale);
        let eq104_e1353_d_b26: f64 = (s.db[174][26] * ddt_scale);
        let eq104_e1353_d_b27: f64 = (s.db[174][27] * ddt_scale);
        let eq104_e1353_d_b28: f64 = (s.db[174][28] * ddt_scale);
        let eq104_e1353_d_b29: f64 = (s.db[174][29] * ddt_scale);
        let eq104_e1353_d_b30: f64 = (s.db[174][30] * ddt_scale);
        let eq104_e1353_d_b31: f64 = (s.db[174][31] * ddt_scale);
        let eq104_e1353_d_b32: f64 = (s.db[174][32] * ddt_scale);
        let eq104_e1353_d_b33: f64 = (s.db[174][33] * ddt_scale);
        let eq104_e1353_d_b34: f64 = (s.db[174][34] * ddt_scale);
        let eq104_e1353_d_b35: f64 = (s.db[174][35] * ddt_scale);
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1356_d_n2: f64 = p.p355;
        let eq104_e1356_d_n10: f64 = (-p.p355);
        let eq104_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 95, eq104_e1356);
        let eq104_e1357_d_n2: f64 = (eq104_e1356_d_n2 * ddt_scale);
        let eq104_e1357_d_n10: f64 = (eq104_e1356_d_n10 * ddt_scale);
        let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);
        let eq104_e1358_d_n2: f64 = (eq104_e1353_d_n2 + eq104_e1357_d_n2);
        let eq104_e1358_d_n10: f64 = (eq104_e1353_d_n10 + eq104_e1357_d_n10);
        (eq104_e1358, eq104_e1353_d_n0, eq104_e1353_d_n1, eq104_e1358_d_n2, eq104_e1353_d_n3, eq104_e1353_d_n4, eq104_e1353_d_n5, eq104_e1353_d_n6, eq104_e1353_d_n7, eq104_e1353_d_n8, eq104_e1353_d_n9, eq104_e1358_d_n10, eq104_e1353_d_n11, eq104_e1353_d_n12, eq104_e1353_d_n13, eq104_e1353_d_n14, eq104_e1353_d_n15, eq104_e1353_d_n16, eq104_e1353_d_n17, eq104_e1353_d_n18, eq104_e1353_d_n19, eq104_e1353_d_n20, eq104_e1353_d_n21, eq104_e1353_d_n22, eq104_e1353_d_n23, eq104_e1353_d_n24, eq104_e1353_d_n25, eq104_e1353_d_n26, eq104_e1353_d_n27, eq104_e1353_d_n28, eq104_e1353_d_n29, eq104_e1353_d_b0, eq104_e1353_d_b1, eq104_e1353_d_b2, eq104_e1353_d_b3, eq104_e1353_d_b4, eq104_e1353_d_b5, eq104_e1353_d_b6, eq104_e1353_d_b7, eq104_e1353_d_b8, eq104_e1353_d_b9, eq104_e1353_d_b10, eq104_e1353_d_b11, eq104_e1353_d_b12, eq104_e1353_d_b13, eq104_e1353_d_b14, eq104_e1353_d_b15, eq104_e1353_d_b16, eq104_e1353_d_b17, eq104_e1353_d_b18, eq104_e1353_d_b19, eq104_e1353_d_b20, eq104_e1353_d_b21, eq104_e1353_d_b22, eq104_e1353_d_b23, eq104_e1353_d_b24, eq104_e1353_d_b25, eq104_e1353_d_b26, eq104_e1353_d_b27, eq104_e1353_d_b28, eq104_e1353_d_b29, eq104_e1353_d_b30, eq104_e1353_d_b31, eq104_e1353_d_b32, eq104_e1353_d_b33, eq104_e1353_d_b34, eq104_e1353_d_b35,)
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
        let eq105_e1364_d_n0: f64 = (s.dn[175][0] * ddt_scale);
        let eq105_e1364_d_n1: f64 = (s.dn[175][1] * ddt_scale);
        let eq105_e1364_d_n2: f64 = (s.dn[175][2] * ddt_scale);
        let eq105_e1364_d_n3: f64 = (s.dn[175][3] * ddt_scale);
        let eq105_e1364_d_n4: f64 = (s.dn[175][4] * ddt_scale);
        let eq105_e1364_d_n5: f64 = (s.dn[175][5] * ddt_scale);
        let eq105_e1364_d_n6: f64 = (s.dn[175][6] * ddt_scale);
        let eq105_e1364_d_n7: f64 = (s.dn[175][7] * ddt_scale);
        let eq105_e1364_d_n8: f64 = (s.dn[175][8] * ddt_scale);
        let eq105_e1364_d_n9: f64 = (s.dn[175][9] * ddt_scale);
        let eq105_e1364_d_n10: f64 = (s.dn[175][10] * ddt_scale);
        let eq105_e1364_d_n11: f64 = (s.dn[175][11] * ddt_scale);
        let eq105_e1364_d_n12: f64 = (s.dn[175][12] * ddt_scale);
        let eq105_e1364_d_n13: f64 = (s.dn[175][13] * ddt_scale);
        let eq105_e1364_d_n14: f64 = (s.dn[175][14] * ddt_scale);
        let eq105_e1364_d_n15: f64 = (s.dn[175][15] * ddt_scale);
        let eq105_e1364_d_n16: f64 = (s.dn[175][16] * ddt_scale);
        let eq105_e1364_d_n17: f64 = (s.dn[175][17] * ddt_scale);
        let eq105_e1364_d_n18: f64 = (s.dn[175][18] * ddt_scale);
        let eq105_e1364_d_n19: f64 = (s.dn[175][19] * ddt_scale);
        let eq105_e1364_d_n20: f64 = (s.dn[175][20] * ddt_scale);
        let eq105_e1364_d_n21: f64 = (s.dn[175][21] * ddt_scale);
        let eq105_e1364_d_n22: f64 = (s.dn[175][22] * ddt_scale);
        let eq105_e1364_d_n23: f64 = (s.dn[175][23] * ddt_scale);
        let eq105_e1364_d_n24: f64 = (s.dn[175][24] * ddt_scale);
        let eq105_e1364_d_n25: f64 = (s.dn[175][25] * ddt_scale);
        let eq105_e1364_d_n26: f64 = (s.dn[175][26] * ddt_scale);
        let eq105_e1364_d_n27: f64 = (s.dn[175][27] * ddt_scale);
        let eq105_e1364_d_n28: f64 = (s.dn[175][28] * ddt_scale);
        let eq105_e1364_d_n29: f64 = (s.dn[175][29] * ddt_scale);
        let eq105_e1364_d_b0: f64 = (s.db[175][0] * ddt_scale);
        let eq105_e1364_d_b1: f64 = (s.db[175][1] * ddt_scale);
        let eq105_e1364_d_b2: f64 = (s.db[175][2] * ddt_scale);
        let eq105_e1364_d_b3: f64 = (s.db[175][3] * ddt_scale);
        let eq105_e1364_d_b4: f64 = (s.db[175][4] * ddt_scale);
        let eq105_e1364_d_b5: f64 = (s.db[175][5] * ddt_scale);
        let eq105_e1364_d_b6: f64 = (s.db[175][6] * ddt_scale);
        let eq105_e1364_d_b7: f64 = (s.db[175][7] * ddt_scale);
        let eq105_e1364_d_b8: f64 = (s.db[175][8] * ddt_scale);
        let eq105_e1364_d_b9: f64 = (s.db[175][9] * ddt_scale);
        let eq105_e1364_d_b10: f64 = (s.db[175][10] * ddt_scale);
        let eq105_e1364_d_b11: f64 = (s.db[175][11] * ddt_scale);
        let eq105_e1364_d_b12: f64 = (s.db[175][12] * ddt_scale);
        let eq105_e1364_d_b13: f64 = (s.db[175][13] * ddt_scale);
        let eq105_e1364_d_b14: f64 = (s.db[175][14] * ddt_scale);
        let eq105_e1364_d_b15: f64 = (s.db[175][15] * ddt_scale);
        let eq105_e1364_d_b16: f64 = (s.db[175][16] * ddt_scale);
        let eq105_e1364_d_b17: f64 = (s.db[175][17] * ddt_scale);
        let eq105_e1364_d_b18: f64 = (s.db[175][18] * ddt_scale);
        let eq105_e1364_d_b19: f64 = (s.db[175][19] * ddt_scale);
        let eq105_e1364_d_b20: f64 = (s.db[175][20] * ddt_scale);
        let eq105_e1364_d_b21: f64 = (s.db[175][21] * ddt_scale);
        let eq105_e1364_d_b22: f64 = (s.db[175][22] * ddt_scale);
        let eq105_e1364_d_b23: f64 = (s.db[175][23] * ddt_scale);
        let eq105_e1364_d_b24: f64 = (s.db[175][24] * ddt_scale);
        let eq105_e1364_d_b25: f64 = (s.db[175][25] * ddt_scale);
        let eq105_e1364_d_b26: f64 = (s.db[175][26] * ddt_scale);
        let eq105_e1364_d_b27: f64 = (s.db[175][27] * ddt_scale);
        let eq105_e1364_d_b28: f64 = (s.db[175][28] * ddt_scale);
        let eq105_e1364_d_b29: f64 = (s.db[175][29] * ddt_scale);
        let eq105_e1364_d_b30: f64 = (s.db[175][30] * ddt_scale);
        let eq105_e1364_d_b31: f64 = (s.db[175][31] * ddt_scale);
        let eq105_e1364_d_b32: f64 = (s.db[175][32] * ddt_scale);
        let eq105_e1364_d_b33: f64 = (s.db[175][33] * ddt_scale);
        let eq105_e1364_d_b34: f64 = (s.db[175][34] * ddt_scale);
        let eq105_e1364_d_b35: f64 = (s.db[175][35] * ddt_scale);
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1367_d_n7: f64 = p.p355;
        let eq105_e1367_d_n11: f64 = (-p.p355);
        let eq105_e1368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 97, eq105_e1367);
        let eq105_e1368_d_n7: f64 = (eq105_e1367_d_n7 * ddt_scale);
        let eq105_e1368_d_n11: f64 = (eq105_e1367_d_n11 * ddt_scale);
        let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);
        let eq105_e1369_d_n7: f64 = (eq105_e1364_d_n7 + eq105_e1368_d_n7);
        let eq105_e1369_d_n11: f64 = (eq105_e1364_d_n11 + eq105_e1368_d_n11);
        (eq105_e1369, eq105_e1364_d_n0, eq105_e1364_d_n1, eq105_e1364_d_n2, eq105_e1364_d_n3, eq105_e1364_d_n4, eq105_e1364_d_n5, eq105_e1364_d_n6, eq105_e1369_d_n7, eq105_e1364_d_n8, eq105_e1364_d_n9, eq105_e1364_d_n10, eq105_e1369_d_n11, eq105_e1364_d_n12, eq105_e1364_d_n13, eq105_e1364_d_n14, eq105_e1364_d_n15, eq105_e1364_d_n16, eq105_e1364_d_n17, eq105_e1364_d_n18, eq105_e1364_d_n19, eq105_e1364_d_n20, eq105_e1364_d_n21, eq105_e1364_d_n22, eq105_e1364_d_n23, eq105_e1364_d_n24, eq105_e1364_d_n25, eq105_e1364_d_n26, eq105_e1364_d_n27, eq105_e1364_d_n28, eq105_e1364_d_n29, eq105_e1364_d_b0, eq105_e1364_d_b1, eq105_e1364_d_b2, eq105_e1364_d_b3, eq105_e1364_d_b4, eq105_e1364_d_b5, eq105_e1364_d_b6, eq105_e1364_d_b7, eq105_e1364_d_b8, eq105_e1364_d_b9, eq105_e1364_d_b10, eq105_e1364_d_b11, eq105_e1364_d_b12, eq105_e1364_d_b13, eq105_e1364_d_b14, eq105_e1364_d_b15, eq105_e1364_d_b16, eq105_e1364_d_b17, eq105_e1364_d_b18, eq105_e1364_d_b19, eq105_e1364_d_b20, eq105_e1364_d_b21, eq105_e1364_d_b22, eq105_e1364_d_b23, eq105_e1364_d_b24, eq105_e1364_d_b25, eq105_e1364_d_b26, eq105_e1364_d_b27, eq105_e1364_d_b28, eq105_e1364_d_b29, eq105_e1364_d_b30, eq105_e1364_d_b31, eq105_e1364_d_b32, eq105_e1364_d_b33, eq105_e1364_d_b34, eq105_e1364_d_b35,)
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
        let (eq106_e1376,) = {
    if (!s.b[1201]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq106_value: f64 = eq106_e1376;
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (eq106_value),
        );
        let (eq107_e1381,) = {
    if (!s.b[1201]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq107_value: f64 = eq107_e1381;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq107_value),
        );
        let eq108_e1383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 98, s.v[176]);
        let eq108_e1383_d_n0: f64 = (s.dn[176][0] * ddt_scale);
        let eq108_e1383_d_n1: f64 = (s.dn[176][1] * ddt_scale);
        let eq108_e1383_d_n2: f64 = (s.dn[176][2] * ddt_scale);
        let eq108_e1383_d_n3: f64 = (s.dn[176][3] * ddt_scale);
        let eq108_e1383_d_n4: f64 = (s.dn[176][4] * ddt_scale);
        let eq108_e1383_d_n5: f64 = (s.dn[176][5] * ddt_scale);
        let eq108_e1383_d_n6: f64 = (s.dn[176][6] * ddt_scale);
        let eq108_e1383_d_n7: f64 = (s.dn[176][7] * ddt_scale);
        let eq108_e1383_d_n8: f64 = (s.dn[176][8] * ddt_scale);
        let eq108_e1383_d_n9: f64 = (s.dn[176][9] * ddt_scale);
        let eq108_e1383_d_n10: f64 = (s.dn[176][10] * ddt_scale);
        let eq108_e1383_d_n11: f64 = (s.dn[176][11] * ddt_scale);
        let eq108_e1383_d_n12: f64 = (s.dn[176][12] * ddt_scale);
        let eq108_e1383_d_n13: f64 = (s.dn[176][13] * ddt_scale);
        let eq108_e1383_d_n14: f64 = (s.dn[176][14] * ddt_scale);
        let eq108_e1383_d_n15: f64 = (s.dn[176][15] * ddt_scale);
        let eq108_e1383_d_n16: f64 = (s.dn[176][16] * ddt_scale);
        let eq108_e1383_d_n17: f64 = (s.dn[176][17] * ddt_scale);
        let eq108_e1383_d_n18: f64 = (s.dn[176][18] * ddt_scale);
        let eq108_e1383_d_n19: f64 = (s.dn[176][19] * ddt_scale);
        let eq108_e1383_d_n20: f64 = (s.dn[176][20] * ddt_scale);
        let eq108_e1383_d_n21: f64 = (s.dn[176][21] * ddt_scale);
        let eq108_e1383_d_n22: f64 = (s.dn[176][22] * ddt_scale);
        let eq108_e1383_d_n23: f64 = (s.dn[176][23] * ddt_scale);
        let eq108_e1383_d_n24: f64 = (s.dn[176][24] * ddt_scale);
        let eq108_e1383_d_n25: f64 = (s.dn[176][25] * ddt_scale);
        let eq108_e1383_d_n26: f64 = (s.dn[176][26] * ddt_scale);
        let eq108_e1383_d_n27: f64 = (s.dn[176][27] * ddt_scale);
        let eq108_e1383_d_n28: f64 = (s.dn[176][28] * ddt_scale);
        let eq108_e1383_d_n29: f64 = (s.dn[176][29] * ddt_scale);
        let eq108_e1383_d_b0: f64 = (s.db[176][0] * ddt_scale);
        let eq108_e1383_d_b1: f64 = (s.db[176][1] * ddt_scale);
        let eq108_e1383_d_b2: f64 = (s.db[176][2] * ddt_scale);
        let eq108_e1383_d_b3: f64 = (s.db[176][3] * ddt_scale);
        let eq108_e1383_d_b4: f64 = (s.db[176][4] * ddt_scale);
        let eq108_e1383_d_b5: f64 = (s.db[176][5] * ddt_scale);
        let eq108_e1383_d_b6: f64 = (s.db[176][6] * ddt_scale);
        let eq108_e1383_d_b7: f64 = (s.db[176][7] * ddt_scale);
        let eq108_e1383_d_b8: f64 = (s.db[176][8] * ddt_scale);
        let eq108_e1383_d_b9: f64 = (s.db[176][9] * ddt_scale);
        let eq108_e1383_d_b10: f64 = (s.db[176][10] * ddt_scale);
        let eq108_e1383_d_b11: f64 = (s.db[176][11] * ddt_scale);
        let eq108_e1383_d_b12: f64 = (s.db[176][12] * ddt_scale);
        let eq108_e1383_d_b13: f64 = (s.db[176][13] * ddt_scale);
        let eq108_e1383_d_b14: f64 = (s.db[176][14] * ddt_scale);
        let eq108_e1383_d_b15: f64 = (s.db[176][15] * ddt_scale);
        let eq108_e1383_d_b16: f64 = (s.db[176][16] * ddt_scale);
        let eq108_e1383_d_b17: f64 = (s.db[176][17] * ddt_scale);
        let eq108_e1383_d_b18: f64 = (s.db[176][18] * ddt_scale);
        let eq108_e1383_d_b19: f64 = (s.db[176][19] * ddt_scale);
        let eq108_e1383_d_b20: f64 = (s.db[176][20] * ddt_scale);
        let eq108_e1383_d_b21: f64 = (s.db[176][21] * ddt_scale);
        let eq108_e1383_d_b22: f64 = (s.db[176][22] * ddt_scale);
        let eq108_e1383_d_b23: f64 = (s.db[176][23] * ddt_scale);
        let eq108_e1383_d_b24: f64 = (s.db[176][24] * ddt_scale);
        let eq108_e1383_d_b25: f64 = (s.db[176][25] * ddt_scale);
        let eq108_e1383_d_b26: f64 = (s.db[176][26] * ddt_scale);
        let eq108_e1383_d_b27: f64 = (s.db[176][27] * ddt_scale);
        let eq108_e1383_d_b28: f64 = (s.db[176][28] * ddt_scale);
        let eq108_e1383_d_b29: f64 = (s.db[176][29] * ddt_scale);
        let eq108_e1383_d_b30: f64 = (s.db[176][30] * ddt_scale);
        let eq108_e1383_d_b31: f64 = (s.db[176][31] * ddt_scale);
        let eq108_e1383_d_b32: f64 = (s.db[176][32] * ddt_scale);
        let eq108_e1383_d_b33: f64 = (s.db[176][33] * ddt_scale);
        let eq108_e1383_d_b34: f64 = (s.db[176][34] * ddt_scale);
        let eq108_e1383_d_b35: f64 = (s.db[176][35] * ddt_scale);
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1386_d_n3: f64 = p.p355;
        let eq108_e1386_d_n11: f64 = (-p.p355);
        let eq108_e1387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 99, eq108_e1386);
        let eq108_e1387_d_n3: f64 = (eq108_e1386_d_n3 * ddt_scale);
        let eq108_e1387_d_n11: f64 = (eq108_e1386_d_n11 * ddt_scale);
        let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);
        let eq108_e1388_d_n3: f64 = (eq108_e1383_d_n3 + eq108_e1387_d_n3);
        let eq108_e1388_d_n11: f64 = (eq108_e1383_d_n11 + eq108_e1387_d_n11);
        let eq108_value: f64 = eq108_e1388;
        let eq108_node_derivatives: [f64; 30] = [eq108_e1383_d_n0, eq108_e1383_d_n1, eq108_e1383_d_n2, eq108_e1388_d_n3, eq108_e1383_d_n4, eq108_e1383_d_n5, eq108_e1383_d_n6, eq108_e1383_d_n7, eq108_e1383_d_n8, eq108_e1383_d_n9, eq108_e1383_d_n10, eq108_e1388_d_n11, eq108_e1383_d_n12, eq108_e1383_d_n13, eq108_e1383_d_n14, eq108_e1383_d_n15, eq108_e1383_d_n16, eq108_e1383_d_n17, eq108_e1383_d_n18, eq108_e1383_d_n19, eq108_e1383_d_n20, eq108_e1383_d_n21, eq108_e1383_d_n22, eq108_e1383_d_n23, eq108_e1383_d_n24, eq108_e1383_d_n25, eq108_e1383_d_n26, eq108_e1383_d_n27, eq108_e1383_d_n28, eq108_e1383_d_n29];
        let eq108_branch_derivatives: [f64; 36] = [eq108_e1383_d_b0, eq108_e1383_d_b1, eq108_e1383_d_b2, eq108_e1383_d_b3, eq108_e1383_d_b4, eq108_e1383_d_b5, eq108_e1383_d_b6, eq108_e1383_d_b7, eq108_e1383_d_b8, eq108_e1383_d_b9, eq108_e1383_d_b10, eq108_e1383_d_b11, eq108_e1383_d_b12, eq108_e1383_d_b13, eq108_e1383_d_b14, eq108_e1383_d_b15, eq108_e1383_d_b16, eq108_e1383_d_b17, eq108_e1383_d_b18, eq108_e1383_d_b19, eq108_e1383_d_b20, eq108_e1383_d_b21, eq108_e1383_d_b22, eq108_e1383_d_b23, eq108_e1383_d_b24, eq108_e1383_d_b25, eq108_e1383_d_b26, eq108_e1383_d_b27, eq108_e1383_d_b28, eq108_e1383_d_b29, eq108_e1383_d_b30, eq108_e1383_d_b31, eq108_e1383_d_b32, eq108_e1383_d_b33, eq108_e1383_d_b34, eq108_e1383_d_b35];
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
        let eq109_e1393_d_n11: f64 = s.v[0];
        let eq109_e1393_d_n12: f64 = (-s.v[0]);
        let eq109_e1394: f64 = (s.v[178] + eq109_e1393);
        let eq109_e1394_d_n11: f64 = (s.dn[178][11] + eq109_e1393_d_n11);
        let eq109_e1394_d_n12: f64 = (s.dn[178][12] + eq109_e1393_d_n12);
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
        let (eq110_e1401,) = {
    if (!s.b[1202]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq110_value: f64 = eq110_e1401;
        stamper.stamp_potential_const_local(
            24,
            eq110_value,
        );
        let (eq111_e1411, eq111_e1411_d_n0, eq111_e1411_d_n1, eq111_e1411_d_n2, eq111_e1411_d_n3, eq111_e1411_d_n4, eq111_e1411_d_n5, eq111_e1411_d_n6, eq111_e1411_d_n7, eq111_e1411_d_n8, eq111_e1411_d_n9, eq111_e1411_d_n10, eq111_e1411_d_n11, eq111_e1411_d_n12, eq111_e1411_d_n13, eq111_e1411_d_n14, eq111_e1411_d_n15, eq111_e1411_d_n16, eq111_e1411_d_n17, eq111_e1411_d_n18, eq111_e1411_d_n19, eq111_e1411_d_n20, eq111_e1411_d_n21, eq111_e1411_d_n22, eq111_e1411_d_n23, eq111_e1411_d_n24, eq111_e1411_d_n25, eq111_e1411_d_n26, eq111_e1411_d_n27, eq111_e1411_d_n28, eq111_e1411_d_n29, eq111_e1411_d_b0, eq111_e1411_d_b1, eq111_e1411_d_b2, eq111_e1411_d_b3, eq111_e1411_d_b4, eq111_e1411_d_b5, eq111_e1411_d_b6, eq111_e1411_d_b7, eq111_e1411_d_b8, eq111_e1411_d_b9, eq111_e1411_d_b10, eq111_e1411_d_b11, eq111_e1411_d_b12, eq111_e1411_d_b13, eq111_e1411_d_b14, eq111_e1411_d_b15, eq111_e1411_d_b16, eq111_e1411_d_b17, eq111_e1411_d_b18, eq111_e1411_d_b19, eq111_e1411_d_b20, eq111_e1411_d_b21, eq111_e1411_d_b22, eq111_e1411_d_b23, eq111_e1411_d_b24, eq111_e1411_d_b25, eq111_e1411_d_b26, eq111_e1411_d_b27, eq111_e1411_d_b28, eq111_e1411_d_b29, eq111_e1411_d_b30, eq111_e1411_d_b31, eq111_e1411_d_b32, eq111_e1411_d_b33, eq111_e1411_d_b34, eq111_e1411_d_b35,) = {
    if s.b[1348] {
        let eq111_e1404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 100, s.v[179]);
        let eq111_e1404_d_n0: f64 = (s.dn[179][0] * ddt_scale);
        let eq111_e1404_d_n1: f64 = (s.dn[179][1] * ddt_scale);
        let eq111_e1404_d_n2: f64 = (s.dn[179][2] * ddt_scale);
        let eq111_e1404_d_n3: f64 = (s.dn[179][3] * ddt_scale);
        let eq111_e1404_d_n4: f64 = (s.dn[179][4] * ddt_scale);
        let eq111_e1404_d_n5: f64 = (s.dn[179][5] * ddt_scale);
        let eq111_e1404_d_n6: f64 = (s.dn[179][6] * ddt_scale);
        let eq111_e1404_d_n7: f64 = (s.dn[179][7] * ddt_scale);
        let eq111_e1404_d_n8: f64 = (s.dn[179][8] * ddt_scale);
        let eq111_e1404_d_n9: f64 = (s.dn[179][9] * ddt_scale);
        let eq111_e1404_d_n10: f64 = (s.dn[179][10] * ddt_scale);
        let eq111_e1404_d_n11: f64 = (s.dn[179][11] * ddt_scale);
        let eq111_e1404_d_n12: f64 = (s.dn[179][12] * ddt_scale);
        let eq111_e1404_d_n13: f64 = (s.dn[179][13] * ddt_scale);
        let eq111_e1404_d_n14: f64 = (s.dn[179][14] * ddt_scale);
        let eq111_e1404_d_n15: f64 = (s.dn[179][15] * ddt_scale);
        let eq111_e1404_d_n16: f64 = (s.dn[179][16] * ddt_scale);
        let eq111_e1404_d_n17: f64 = (s.dn[179][17] * ddt_scale);
        let eq111_e1404_d_n18: f64 = (s.dn[179][18] * ddt_scale);
        let eq111_e1404_d_n19: f64 = (s.dn[179][19] * ddt_scale);
        let eq111_e1404_d_n20: f64 = (s.dn[179][20] * ddt_scale);
        let eq111_e1404_d_n21: f64 = (s.dn[179][21] * ddt_scale);
        let eq111_e1404_d_n22: f64 = (s.dn[179][22] * ddt_scale);
        let eq111_e1404_d_n23: f64 = (s.dn[179][23] * ddt_scale);
        let eq111_e1404_d_n24: f64 = (s.dn[179][24] * ddt_scale);
        let eq111_e1404_d_n25: f64 = (s.dn[179][25] * ddt_scale);
        let eq111_e1404_d_n26: f64 = (s.dn[179][26] * ddt_scale);
        let eq111_e1404_d_n27: f64 = (s.dn[179][27] * ddt_scale);
        let eq111_e1404_d_n28: f64 = (s.dn[179][28] * ddt_scale);
        let eq111_e1404_d_n29: f64 = (s.dn[179][29] * ddt_scale);
        let eq111_e1404_d_b0: f64 = (s.db[179][0] * ddt_scale);
        let eq111_e1404_d_b1: f64 = (s.db[179][1] * ddt_scale);
        let eq111_e1404_d_b2: f64 = (s.db[179][2] * ddt_scale);
        let eq111_e1404_d_b3: f64 = (s.db[179][3] * ddt_scale);
        let eq111_e1404_d_b4: f64 = (s.db[179][4] * ddt_scale);
        let eq111_e1404_d_b5: f64 = (s.db[179][5] * ddt_scale);
        let eq111_e1404_d_b6: f64 = (s.db[179][6] * ddt_scale);
        let eq111_e1404_d_b7: f64 = (s.db[179][7] * ddt_scale);
        let eq111_e1404_d_b8: f64 = (s.db[179][8] * ddt_scale);
        let eq111_e1404_d_b9: f64 = (s.db[179][9] * ddt_scale);
        let eq111_e1404_d_b10: f64 = (s.db[179][10] * ddt_scale);
        let eq111_e1404_d_b11: f64 = (s.db[179][11] * ddt_scale);
        let eq111_e1404_d_b12: f64 = (s.db[179][12] * ddt_scale);
        let eq111_e1404_d_b13: f64 = (s.db[179][13] * ddt_scale);
        let eq111_e1404_d_b14: f64 = (s.db[179][14] * ddt_scale);
        let eq111_e1404_d_b15: f64 = (s.db[179][15] * ddt_scale);
        let eq111_e1404_d_b16: f64 = (s.db[179][16] * ddt_scale);
        let eq111_e1404_d_b17: f64 = (s.db[179][17] * ddt_scale);
        let eq111_e1404_d_b18: f64 = (s.db[179][18] * ddt_scale);
        let eq111_e1404_d_b19: f64 = (s.db[179][19] * ddt_scale);
        let eq111_e1404_d_b20: f64 = (s.db[179][20] * ddt_scale);
        let eq111_e1404_d_b21: f64 = (s.db[179][21] * ddt_scale);
        let eq111_e1404_d_b22: f64 = (s.db[179][22] * ddt_scale);
        let eq111_e1404_d_b23: f64 = (s.db[179][23] * ddt_scale);
        let eq111_e1404_d_b24: f64 = (s.db[179][24] * ddt_scale);
        let eq111_e1404_d_b25: f64 = (s.db[179][25] * ddt_scale);
        let eq111_e1404_d_b26: f64 = (s.db[179][26] * ddt_scale);
        let eq111_e1404_d_b27: f64 = (s.db[179][27] * ddt_scale);
        let eq111_e1404_d_b28: f64 = (s.db[179][28] * ddt_scale);
        let eq111_e1404_d_b29: f64 = (s.db[179][29] * ddt_scale);
        let eq111_e1404_d_b30: f64 = (s.db[179][30] * ddt_scale);
        let eq111_e1404_d_b31: f64 = (s.db[179][31] * ddt_scale);
        let eq111_e1404_d_b32: f64 = (s.db[179][32] * ddt_scale);
        let eq111_e1404_d_b33: f64 = (s.db[179][33] * ddt_scale);
        let eq111_e1404_d_b34: f64 = (s.db[179][34] * ddt_scale);
        let eq111_e1404_d_b35: f64 = (s.db[179][35] * ddt_scale);
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1407_d_n7: f64 = p.p355;
        let eq111_e1407_d_n12: f64 = (-p.p355);
        let eq111_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 101, eq111_e1407);
        let eq111_e1408_d_n7: f64 = (eq111_e1407_d_n7 * ddt_scale);
        let eq111_e1408_d_n12: f64 = (eq111_e1407_d_n12 * ddt_scale);
        let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);
        let eq111_e1409_d_n7: f64 = (eq111_e1404_d_n7 + eq111_e1408_d_n7);
        let eq111_e1409_d_n12: f64 = (eq111_e1404_d_n12 + eq111_e1408_d_n12);
        (eq111_e1409, eq111_e1404_d_n0, eq111_e1404_d_n1, eq111_e1404_d_n2, eq111_e1404_d_n3, eq111_e1404_d_n4, eq111_e1404_d_n5, eq111_e1404_d_n6, eq111_e1409_d_n7, eq111_e1404_d_n8, eq111_e1404_d_n9, eq111_e1404_d_n10, eq111_e1404_d_n11, eq111_e1409_d_n12, eq111_e1404_d_n13, eq111_e1404_d_n14, eq111_e1404_d_n15, eq111_e1404_d_n16, eq111_e1404_d_n17, eq111_e1404_d_n18, eq111_e1404_d_n19, eq111_e1404_d_n20, eq111_e1404_d_n21, eq111_e1404_d_n22, eq111_e1404_d_n23, eq111_e1404_d_n24, eq111_e1404_d_n25, eq111_e1404_d_n26, eq111_e1404_d_n27, eq111_e1404_d_n28, eq111_e1404_d_n29, eq111_e1404_d_b0, eq111_e1404_d_b1, eq111_e1404_d_b2, eq111_e1404_d_b3, eq111_e1404_d_b4, eq111_e1404_d_b5, eq111_e1404_d_b6, eq111_e1404_d_b7, eq111_e1404_d_b8, eq111_e1404_d_b9, eq111_e1404_d_b10, eq111_e1404_d_b11, eq111_e1404_d_b12, eq111_e1404_d_b13, eq111_e1404_d_b14, eq111_e1404_d_b15, eq111_e1404_d_b16, eq111_e1404_d_b17, eq111_e1404_d_b18, eq111_e1404_d_b19, eq111_e1404_d_b20, eq111_e1404_d_b21, eq111_e1404_d_b22, eq111_e1404_d_b23, eq111_e1404_d_b24, eq111_e1404_d_b25, eq111_e1404_d_b26, eq111_e1404_d_b27, eq111_e1404_d_b28, eq111_e1404_d_b29, eq111_e1404_d_b30, eq111_e1404_d_b31, eq111_e1404_d_b32, eq111_e1404_d_b33, eq111_e1404_d_b34, eq111_e1404_d_b35,)
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
    }

    pub(super) fn stamp_transient_equations_block_15(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq112_e1421, eq112_e1421_d_n0, eq112_e1421_d_n1, eq112_e1421_d_n2, eq112_e1421_d_n3, eq112_e1421_d_n4, eq112_e1421_d_n5, eq112_e1421_d_n6, eq112_e1421_d_n7, eq112_e1421_d_n8, eq112_e1421_d_n9, eq112_e1421_d_n10, eq112_e1421_d_n11, eq112_e1421_d_n12, eq112_e1421_d_n13, eq112_e1421_d_n14, eq112_e1421_d_n15, eq112_e1421_d_n16, eq112_e1421_d_n17, eq112_e1421_d_n18, eq112_e1421_d_n19, eq112_e1421_d_n20, eq112_e1421_d_n21, eq112_e1421_d_n22, eq112_e1421_d_n23, eq112_e1421_d_n24, eq112_e1421_d_n25, eq112_e1421_d_n26, eq112_e1421_d_n27, eq112_e1421_d_n28, eq112_e1421_d_n29, eq112_e1421_d_b0, eq112_e1421_d_b1, eq112_e1421_d_b2, eq112_e1421_d_b3, eq112_e1421_d_b4, eq112_e1421_d_b5, eq112_e1421_d_b6, eq112_e1421_d_b7, eq112_e1421_d_b8, eq112_e1421_d_b9, eq112_e1421_d_b10, eq112_e1421_d_b11, eq112_e1421_d_b12, eq112_e1421_d_b13, eq112_e1421_d_b14, eq112_e1421_d_b15, eq112_e1421_d_b16, eq112_e1421_d_b17, eq112_e1421_d_b18, eq112_e1421_d_b19, eq112_e1421_d_b20, eq112_e1421_d_b21, eq112_e1421_d_b22, eq112_e1421_d_b23, eq112_e1421_d_b24, eq112_e1421_d_b25, eq112_e1421_d_b26, eq112_e1421_d_b27, eq112_e1421_d_b28, eq112_e1421_d_b29, eq112_e1421_d_b30, eq112_e1421_d_b31, eq112_e1421_d_b32, eq112_e1421_d_b33, eq112_e1421_d_b34, eq112_e1421_d_b35,) = {
    if s.b[1348] {
        let eq112_e1414: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 102, s.v[180]);
        let eq112_e1414_d_n0: f64 = (s.dn[180][0] * ddt_scale);
        let eq112_e1414_d_n1: f64 = (s.dn[180][1] * ddt_scale);
        let eq112_e1414_d_n2: f64 = (s.dn[180][2] * ddt_scale);
        let eq112_e1414_d_n3: f64 = (s.dn[180][3] * ddt_scale);
        let eq112_e1414_d_n4: f64 = (s.dn[180][4] * ddt_scale);
        let eq112_e1414_d_n5: f64 = (s.dn[180][5] * ddt_scale);
        let eq112_e1414_d_n6: f64 = (s.dn[180][6] * ddt_scale);
        let eq112_e1414_d_n7: f64 = (s.dn[180][7] * ddt_scale);
        let eq112_e1414_d_n8: f64 = (s.dn[180][8] * ddt_scale);
        let eq112_e1414_d_n9: f64 = (s.dn[180][9] * ddt_scale);
        let eq112_e1414_d_n10: f64 = (s.dn[180][10] * ddt_scale);
        let eq112_e1414_d_n11: f64 = (s.dn[180][11] * ddt_scale);
        let eq112_e1414_d_n12: f64 = (s.dn[180][12] * ddt_scale);
        let eq112_e1414_d_n13: f64 = (s.dn[180][13] * ddt_scale);
        let eq112_e1414_d_n14: f64 = (s.dn[180][14] * ddt_scale);
        let eq112_e1414_d_n15: f64 = (s.dn[180][15] * ddt_scale);
        let eq112_e1414_d_n16: f64 = (s.dn[180][16] * ddt_scale);
        let eq112_e1414_d_n17: f64 = (s.dn[180][17] * ddt_scale);
        let eq112_e1414_d_n18: f64 = (s.dn[180][18] * ddt_scale);
        let eq112_e1414_d_n19: f64 = (s.dn[180][19] * ddt_scale);
        let eq112_e1414_d_n20: f64 = (s.dn[180][20] * ddt_scale);
        let eq112_e1414_d_n21: f64 = (s.dn[180][21] * ddt_scale);
        let eq112_e1414_d_n22: f64 = (s.dn[180][22] * ddt_scale);
        let eq112_e1414_d_n23: f64 = (s.dn[180][23] * ddt_scale);
        let eq112_e1414_d_n24: f64 = (s.dn[180][24] * ddt_scale);
        let eq112_e1414_d_n25: f64 = (s.dn[180][25] * ddt_scale);
        let eq112_e1414_d_n26: f64 = (s.dn[180][26] * ddt_scale);
        let eq112_e1414_d_n27: f64 = (s.dn[180][27] * ddt_scale);
        let eq112_e1414_d_n28: f64 = (s.dn[180][28] * ddt_scale);
        let eq112_e1414_d_n29: f64 = (s.dn[180][29] * ddt_scale);
        let eq112_e1414_d_b0: f64 = (s.db[180][0] * ddt_scale);
        let eq112_e1414_d_b1: f64 = (s.db[180][1] * ddt_scale);
        let eq112_e1414_d_b2: f64 = (s.db[180][2] * ddt_scale);
        let eq112_e1414_d_b3: f64 = (s.db[180][3] * ddt_scale);
        let eq112_e1414_d_b4: f64 = (s.db[180][4] * ddt_scale);
        let eq112_e1414_d_b5: f64 = (s.db[180][5] * ddt_scale);
        let eq112_e1414_d_b6: f64 = (s.db[180][6] * ddt_scale);
        let eq112_e1414_d_b7: f64 = (s.db[180][7] * ddt_scale);
        let eq112_e1414_d_b8: f64 = (s.db[180][8] * ddt_scale);
        let eq112_e1414_d_b9: f64 = (s.db[180][9] * ddt_scale);
        let eq112_e1414_d_b10: f64 = (s.db[180][10] * ddt_scale);
        let eq112_e1414_d_b11: f64 = (s.db[180][11] * ddt_scale);
        let eq112_e1414_d_b12: f64 = (s.db[180][12] * ddt_scale);
        let eq112_e1414_d_b13: f64 = (s.db[180][13] * ddt_scale);
        let eq112_e1414_d_b14: f64 = (s.db[180][14] * ddt_scale);
        let eq112_e1414_d_b15: f64 = (s.db[180][15] * ddt_scale);
        let eq112_e1414_d_b16: f64 = (s.db[180][16] * ddt_scale);
        let eq112_e1414_d_b17: f64 = (s.db[180][17] * ddt_scale);
        let eq112_e1414_d_b18: f64 = (s.db[180][18] * ddt_scale);
        let eq112_e1414_d_b19: f64 = (s.db[180][19] * ddt_scale);
        let eq112_e1414_d_b20: f64 = (s.db[180][20] * ddt_scale);
        let eq112_e1414_d_b21: f64 = (s.db[180][21] * ddt_scale);
        let eq112_e1414_d_b22: f64 = (s.db[180][22] * ddt_scale);
        let eq112_e1414_d_b23: f64 = (s.db[180][23] * ddt_scale);
        let eq112_e1414_d_b24: f64 = (s.db[180][24] * ddt_scale);
        let eq112_e1414_d_b25: f64 = (s.db[180][25] * ddt_scale);
        let eq112_e1414_d_b26: f64 = (s.db[180][26] * ddt_scale);
        let eq112_e1414_d_b27: f64 = (s.db[180][27] * ddt_scale);
        let eq112_e1414_d_b28: f64 = (s.db[180][28] * ddt_scale);
        let eq112_e1414_d_b29: f64 = (s.db[180][29] * ddt_scale);
        let eq112_e1414_d_b30: f64 = (s.db[180][30] * ddt_scale);
        let eq112_e1414_d_b31: f64 = (s.db[180][31] * ddt_scale);
        let eq112_e1414_d_b32: f64 = (s.db[180][32] * ddt_scale);
        let eq112_e1414_d_b33: f64 = (s.db[180][33] * ddt_scale);
        let eq112_e1414_d_b34: f64 = (s.db[180][34] * ddt_scale);
        let eq112_e1414_d_b35: f64 = (s.db[180][35] * ddt_scale);
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1417_d_n7: f64 = p.p355;
        let eq112_e1417_d_n11: f64 = (-p.p355);
        let eq112_e1418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 103, eq112_e1417);
        let eq112_e1418_d_n7: f64 = (eq112_e1417_d_n7 * ddt_scale);
        let eq112_e1418_d_n11: f64 = (eq112_e1417_d_n11 * ddt_scale);
        let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);
        let eq112_e1419_d_n7: f64 = (eq112_e1414_d_n7 + eq112_e1418_d_n7);
        let eq112_e1419_d_n11: f64 = (eq112_e1414_d_n11 + eq112_e1418_d_n11);
        (eq112_e1419, eq112_e1414_d_n0, eq112_e1414_d_n1, eq112_e1414_d_n2, eq112_e1414_d_n3, eq112_e1414_d_n4, eq112_e1414_d_n5, eq112_e1414_d_n6, eq112_e1419_d_n7, eq112_e1414_d_n8, eq112_e1414_d_n9, eq112_e1414_d_n10, eq112_e1419_d_n11, eq112_e1414_d_n12, eq112_e1414_d_n13, eq112_e1414_d_n14, eq112_e1414_d_n15, eq112_e1414_d_n16, eq112_e1414_d_n17, eq112_e1414_d_n18, eq112_e1414_d_n19, eq112_e1414_d_n20, eq112_e1414_d_n21, eq112_e1414_d_n22, eq112_e1414_d_n23, eq112_e1414_d_n24, eq112_e1414_d_n25, eq112_e1414_d_n26, eq112_e1414_d_n27, eq112_e1414_d_n28, eq112_e1414_d_n29, eq112_e1414_d_b0, eq112_e1414_d_b1, eq112_e1414_d_b2, eq112_e1414_d_b3, eq112_e1414_d_b4, eq112_e1414_d_b5, eq112_e1414_d_b6, eq112_e1414_d_b7, eq112_e1414_d_b8, eq112_e1414_d_b9, eq112_e1414_d_b10, eq112_e1414_d_b11, eq112_e1414_d_b12, eq112_e1414_d_b13, eq112_e1414_d_b14, eq112_e1414_d_b15, eq112_e1414_d_b16, eq112_e1414_d_b17, eq112_e1414_d_b18, eq112_e1414_d_b19, eq112_e1414_d_b20, eq112_e1414_d_b21, eq112_e1414_d_b22, eq112_e1414_d_b23, eq112_e1414_d_b24, eq112_e1414_d_b25, eq112_e1414_d_b26, eq112_e1414_d_b27, eq112_e1414_d_b28, eq112_e1414_d_b29, eq112_e1414_d_b30, eq112_e1414_d_b31, eq112_e1414_d_b32, eq112_e1414_d_b33, eq112_e1414_d_b34, eq112_e1414_d_b35,)
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
        let eq113_e1424_d_n0: f64 = (s.dn[181][0] * ddt_scale);
        let eq113_e1424_d_n1: f64 = (s.dn[181][1] * ddt_scale);
        let eq113_e1424_d_n2: f64 = (s.dn[181][2] * ddt_scale);
        let eq113_e1424_d_n3: f64 = (s.dn[181][3] * ddt_scale);
        let eq113_e1424_d_n4: f64 = (s.dn[181][4] * ddt_scale);
        let eq113_e1424_d_n5: f64 = (s.dn[181][5] * ddt_scale);
        let eq113_e1424_d_n6: f64 = (s.dn[181][6] * ddt_scale);
        let eq113_e1424_d_n7: f64 = (s.dn[181][7] * ddt_scale);
        let eq113_e1424_d_n8: f64 = (s.dn[181][8] * ddt_scale);
        let eq113_e1424_d_n9: f64 = (s.dn[181][9] * ddt_scale);
        let eq113_e1424_d_n10: f64 = (s.dn[181][10] * ddt_scale);
        let eq113_e1424_d_n11: f64 = (s.dn[181][11] * ddt_scale);
        let eq113_e1424_d_n12: f64 = (s.dn[181][12] * ddt_scale);
        let eq113_e1424_d_n13: f64 = (s.dn[181][13] * ddt_scale);
        let eq113_e1424_d_n14: f64 = (s.dn[181][14] * ddt_scale);
        let eq113_e1424_d_n15: f64 = (s.dn[181][15] * ddt_scale);
        let eq113_e1424_d_n16: f64 = (s.dn[181][16] * ddt_scale);
        let eq113_e1424_d_n17: f64 = (s.dn[181][17] * ddt_scale);
        let eq113_e1424_d_n18: f64 = (s.dn[181][18] * ddt_scale);
        let eq113_e1424_d_n19: f64 = (s.dn[181][19] * ddt_scale);
        let eq113_e1424_d_n20: f64 = (s.dn[181][20] * ddt_scale);
        let eq113_e1424_d_n21: f64 = (s.dn[181][21] * ddt_scale);
        let eq113_e1424_d_n22: f64 = (s.dn[181][22] * ddt_scale);
        let eq113_e1424_d_n23: f64 = (s.dn[181][23] * ddt_scale);
        let eq113_e1424_d_n24: f64 = (s.dn[181][24] * ddt_scale);
        let eq113_e1424_d_n25: f64 = (s.dn[181][25] * ddt_scale);
        let eq113_e1424_d_n26: f64 = (s.dn[181][26] * ddt_scale);
        let eq113_e1424_d_n27: f64 = (s.dn[181][27] * ddt_scale);
        let eq113_e1424_d_n28: f64 = (s.dn[181][28] * ddt_scale);
        let eq113_e1424_d_n29: f64 = (s.dn[181][29] * ddt_scale);
        let eq113_e1424_d_b0: f64 = (s.db[181][0] * ddt_scale);
        let eq113_e1424_d_b1: f64 = (s.db[181][1] * ddt_scale);
        let eq113_e1424_d_b2: f64 = (s.db[181][2] * ddt_scale);
        let eq113_e1424_d_b3: f64 = (s.db[181][3] * ddt_scale);
        let eq113_e1424_d_b4: f64 = (s.db[181][4] * ddt_scale);
        let eq113_e1424_d_b5: f64 = (s.db[181][5] * ddt_scale);
        let eq113_e1424_d_b6: f64 = (s.db[181][6] * ddt_scale);
        let eq113_e1424_d_b7: f64 = (s.db[181][7] * ddt_scale);
        let eq113_e1424_d_b8: f64 = (s.db[181][8] * ddt_scale);
        let eq113_e1424_d_b9: f64 = (s.db[181][9] * ddt_scale);
        let eq113_e1424_d_b10: f64 = (s.db[181][10] * ddt_scale);
        let eq113_e1424_d_b11: f64 = (s.db[181][11] * ddt_scale);
        let eq113_e1424_d_b12: f64 = (s.db[181][12] * ddt_scale);
        let eq113_e1424_d_b13: f64 = (s.db[181][13] * ddt_scale);
        let eq113_e1424_d_b14: f64 = (s.db[181][14] * ddt_scale);
        let eq113_e1424_d_b15: f64 = (s.db[181][15] * ddt_scale);
        let eq113_e1424_d_b16: f64 = (s.db[181][16] * ddt_scale);
        let eq113_e1424_d_b17: f64 = (s.db[181][17] * ddt_scale);
        let eq113_e1424_d_b18: f64 = (s.db[181][18] * ddt_scale);
        let eq113_e1424_d_b19: f64 = (s.db[181][19] * ddt_scale);
        let eq113_e1424_d_b20: f64 = (s.db[181][20] * ddt_scale);
        let eq113_e1424_d_b21: f64 = (s.db[181][21] * ddt_scale);
        let eq113_e1424_d_b22: f64 = (s.db[181][22] * ddt_scale);
        let eq113_e1424_d_b23: f64 = (s.db[181][23] * ddt_scale);
        let eq113_e1424_d_b24: f64 = (s.db[181][24] * ddt_scale);
        let eq113_e1424_d_b25: f64 = (s.db[181][25] * ddt_scale);
        let eq113_e1424_d_b26: f64 = (s.db[181][26] * ddt_scale);
        let eq113_e1424_d_b27: f64 = (s.db[181][27] * ddt_scale);
        let eq113_e1424_d_b28: f64 = (s.db[181][28] * ddt_scale);
        let eq113_e1424_d_b29: f64 = (s.db[181][29] * ddt_scale);
        let eq113_e1424_d_b30: f64 = (s.db[181][30] * ddt_scale);
        let eq113_e1424_d_b31: f64 = (s.db[181][31] * ddt_scale);
        let eq113_e1424_d_b32: f64 = (s.db[181][32] * ddt_scale);
        let eq113_e1424_d_b33: f64 = (s.db[181][33] * ddt_scale);
        let eq113_e1424_d_b34: f64 = (s.db[181][34] * ddt_scale);
        let eq113_e1424_d_b35: f64 = (s.db[181][35] * ddt_scale);
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1427_d_n2: f64 = p.p355;
        let eq113_e1427_d_n12: f64 = (-p.p355);
        let eq113_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 105, eq113_e1427);
        let eq113_e1428_d_n2: f64 = (eq113_e1427_d_n2 * ddt_scale);
        let eq113_e1428_d_n12: f64 = (eq113_e1427_d_n12 * ddt_scale);
        let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);
        let eq113_e1429_d_n2: f64 = (eq113_e1424_d_n2 + eq113_e1428_d_n2);
        let eq113_e1429_d_n12: f64 = (eq113_e1424_d_n12 + eq113_e1428_d_n12);
        (eq113_e1429, eq113_e1424_d_n0, eq113_e1424_d_n1, eq113_e1429_d_n2, eq113_e1424_d_n3, eq113_e1424_d_n4, eq113_e1424_d_n5, eq113_e1424_d_n6, eq113_e1424_d_n7, eq113_e1424_d_n8, eq113_e1424_d_n9, eq113_e1424_d_n10, eq113_e1424_d_n11, eq113_e1429_d_n12, eq113_e1424_d_n13, eq113_e1424_d_n14, eq113_e1424_d_n15, eq113_e1424_d_n16, eq113_e1424_d_n17, eq113_e1424_d_n18, eq113_e1424_d_n19, eq113_e1424_d_n20, eq113_e1424_d_n21, eq113_e1424_d_n22, eq113_e1424_d_n23, eq113_e1424_d_n24, eq113_e1424_d_n25, eq113_e1424_d_n26, eq113_e1424_d_n27, eq113_e1424_d_n28, eq113_e1424_d_n29, eq113_e1424_d_b0, eq113_e1424_d_b1, eq113_e1424_d_b2, eq113_e1424_d_b3, eq113_e1424_d_b4, eq113_e1424_d_b5, eq113_e1424_d_b6, eq113_e1424_d_b7, eq113_e1424_d_b8, eq113_e1424_d_b9, eq113_e1424_d_b10, eq113_e1424_d_b11, eq113_e1424_d_b12, eq113_e1424_d_b13, eq113_e1424_d_b14, eq113_e1424_d_b15, eq113_e1424_d_b16, eq113_e1424_d_b17, eq113_e1424_d_b18, eq113_e1424_d_b19, eq113_e1424_d_b20, eq113_e1424_d_b21, eq113_e1424_d_b22, eq113_e1424_d_b23, eq113_e1424_d_b24, eq113_e1424_d_b25, eq113_e1424_d_b26, eq113_e1424_d_b27, eq113_e1424_d_b28, eq113_e1424_d_b29, eq113_e1424_d_b30, eq113_e1424_d_b31, eq113_e1424_d_b32, eq113_e1424_d_b33, eq113_e1424_d_b34, eq113_e1424_d_b35,)
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
        let (eq114_e1435,) = {
    if s.b[1348] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq114_value: f64 = eq114_e1435;
        stamper.stamp_current_const_local(
            Some(2),
            Some(11),
            multiplicity * (eq114_value),
        );
        let (eq115_e1445, eq115_e1445_d_n0, eq115_e1445_d_n1, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n5, eq115_e1445_d_n6, eq115_e1445_d_n7, eq115_e1445_d_n8, eq115_e1445_d_n9, eq115_e1445_d_n10, eq115_e1445_d_n11, eq115_e1445_d_n12, eq115_e1445_d_n13, eq115_e1445_d_n14, eq115_e1445_d_n15, eq115_e1445_d_n16, eq115_e1445_d_n17, eq115_e1445_d_n18, eq115_e1445_d_n19, eq115_e1445_d_n20, eq115_e1445_d_n21, eq115_e1445_d_n22, eq115_e1445_d_n23, eq115_e1445_d_n24, eq115_e1445_d_n25, eq115_e1445_d_n26, eq115_e1445_d_n27, eq115_e1445_d_n28, eq115_e1445_d_n29, eq115_e1445_d_b0, eq115_e1445_d_b1, eq115_e1445_d_b2, eq115_e1445_d_b3, eq115_e1445_d_b4, eq115_e1445_d_b5, eq115_e1445_d_b6, eq115_e1445_d_b7, eq115_e1445_d_b8, eq115_e1445_d_b9, eq115_e1445_d_b10, eq115_e1445_d_b11, eq115_e1445_d_b12, eq115_e1445_d_b13, eq115_e1445_d_b14, eq115_e1445_d_b15, eq115_e1445_d_b16, eq115_e1445_d_b17, eq115_e1445_d_b18, eq115_e1445_d_b19, eq115_e1445_d_b20, eq115_e1445_d_b21, eq115_e1445_d_b22, eq115_e1445_d_b23, eq115_e1445_d_b24, eq115_e1445_d_b25, eq115_e1445_d_b26, eq115_e1445_d_b27, eq115_e1445_d_b28, eq115_e1445_d_b29, eq115_e1445_d_b30, eq115_e1445_d_b31, eq115_e1445_d_b32, eq115_e1445_d_b33, eq115_e1445_d_b34, eq115_e1445_d_b35,) = {
    if s.b[1348] {
        let eq115_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 106, s.v[183]);
        let eq115_e1438_d_n0: f64 = (s.dn[183][0] * ddt_scale);
        let eq115_e1438_d_n1: f64 = (s.dn[183][1] * ddt_scale);
        let eq115_e1438_d_n2: f64 = (s.dn[183][2] * ddt_scale);
        let eq115_e1438_d_n3: f64 = (s.dn[183][3] * ddt_scale);
        let eq115_e1438_d_n4: f64 = (s.dn[183][4] * ddt_scale);
        let eq115_e1438_d_n5: f64 = (s.dn[183][5] * ddt_scale);
        let eq115_e1438_d_n6: f64 = (s.dn[183][6] * ddt_scale);
        let eq115_e1438_d_n7: f64 = (s.dn[183][7] * ddt_scale);
        let eq115_e1438_d_n8: f64 = (s.dn[183][8] * ddt_scale);
        let eq115_e1438_d_n9: f64 = (s.dn[183][9] * ddt_scale);
        let eq115_e1438_d_n10: f64 = (s.dn[183][10] * ddt_scale);
        let eq115_e1438_d_n11: f64 = (s.dn[183][11] * ddt_scale);
        let eq115_e1438_d_n12: f64 = (s.dn[183][12] * ddt_scale);
        let eq115_e1438_d_n13: f64 = (s.dn[183][13] * ddt_scale);
        let eq115_e1438_d_n14: f64 = (s.dn[183][14] * ddt_scale);
        let eq115_e1438_d_n15: f64 = (s.dn[183][15] * ddt_scale);
        let eq115_e1438_d_n16: f64 = (s.dn[183][16] * ddt_scale);
        let eq115_e1438_d_n17: f64 = (s.dn[183][17] * ddt_scale);
        let eq115_e1438_d_n18: f64 = (s.dn[183][18] * ddt_scale);
        let eq115_e1438_d_n19: f64 = (s.dn[183][19] * ddt_scale);
        let eq115_e1438_d_n20: f64 = (s.dn[183][20] * ddt_scale);
        let eq115_e1438_d_n21: f64 = (s.dn[183][21] * ddt_scale);
        let eq115_e1438_d_n22: f64 = (s.dn[183][22] * ddt_scale);
        let eq115_e1438_d_n23: f64 = (s.dn[183][23] * ddt_scale);
        let eq115_e1438_d_n24: f64 = (s.dn[183][24] * ddt_scale);
        let eq115_e1438_d_n25: f64 = (s.dn[183][25] * ddt_scale);
        let eq115_e1438_d_n26: f64 = (s.dn[183][26] * ddt_scale);
        let eq115_e1438_d_n27: f64 = (s.dn[183][27] * ddt_scale);
        let eq115_e1438_d_n28: f64 = (s.dn[183][28] * ddt_scale);
        let eq115_e1438_d_n29: f64 = (s.dn[183][29] * ddt_scale);
        let eq115_e1438_d_b0: f64 = (s.db[183][0] * ddt_scale);
        let eq115_e1438_d_b1: f64 = (s.db[183][1] * ddt_scale);
        let eq115_e1438_d_b2: f64 = (s.db[183][2] * ddt_scale);
        let eq115_e1438_d_b3: f64 = (s.db[183][3] * ddt_scale);
        let eq115_e1438_d_b4: f64 = (s.db[183][4] * ddt_scale);
        let eq115_e1438_d_b5: f64 = (s.db[183][5] * ddt_scale);
        let eq115_e1438_d_b6: f64 = (s.db[183][6] * ddt_scale);
        let eq115_e1438_d_b7: f64 = (s.db[183][7] * ddt_scale);
        let eq115_e1438_d_b8: f64 = (s.db[183][8] * ddt_scale);
        let eq115_e1438_d_b9: f64 = (s.db[183][9] * ddt_scale);
        let eq115_e1438_d_b10: f64 = (s.db[183][10] * ddt_scale);
        let eq115_e1438_d_b11: f64 = (s.db[183][11] * ddt_scale);
        let eq115_e1438_d_b12: f64 = (s.db[183][12] * ddt_scale);
        let eq115_e1438_d_b13: f64 = (s.db[183][13] * ddt_scale);
        let eq115_e1438_d_b14: f64 = (s.db[183][14] * ddt_scale);
        let eq115_e1438_d_b15: f64 = (s.db[183][15] * ddt_scale);
        let eq115_e1438_d_b16: f64 = (s.db[183][16] * ddt_scale);
        let eq115_e1438_d_b17: f64 = (s.db[183][17] * ddt_scale);
        let eq115_e1438_d_b18: f64 = (s.db[183][18] * ddt_scale);
        let eq115_e1438_d_b19: f64 = (s.db[183][19] * ddt_scale);
        let eq115_e1438_d_b20: f64 = (s.db[183][20] * ddt_scale);
        let eq115_e1438_d_b21: f64 = (s.db[183][21] * ddt_scale);
        let eq115_e1438_d_b22: f64 = (s.db[183][22] * ddt_scale);
        let eq115_e1438_d_b23: f64 = (s.db[183][23] * ddt_scale);
        let eq115_e1438_d_b24: f64 = (s.db[183][24] * ddt_scale);
        let eq115_e1438_d_b25: f64 = (s.db[183][25] * ddt_scale);
        let eq115_e1438_d_b26: f64 = (s.db[183][26] * ddt_scale);
        let eq115_e1438_d_b27: f64 = (s.db[183][27] * ddt_scale);
        let eq115_e1438_d_b28: f64 = (s.db[183][28] * ddt_scale);
        let eq115_e1438_d_b29: f64 = (s.db[183][29] * ddt_scale);
        let eq115_e1438_d_b30: f64 = (s.db[183][30] * ddt_scale);
        let eq115_e1438_d_b31: f64 = (s.db[183][31] * ddt_scale);
        let eq115_e1438_d_b32: f64 = (s.db[183][32] * ddt_scale);
        let eq115_e1438_d_b33: f64 = (s.db[183][33] * ddt_scale);
        let eq115_e1438_d_b34: f64 = (s.db[183][34] * ddt_scale);
        let eq115_e1438_d_b35: f64 = (s.db[183][35] * ddt_scale);
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1441_d_n7: f64 = p.p355;
        let eq115_e1441_d_n9: f64 = (-p.p355);
        let eq115_e1442: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 107, eq115_e1441);
        let eq115_e1442_d_n7: f64 = (eq115_e1441_d_n7 * ddt_scale);
        let eq115_e1442_d_n9: f64 = (eq115_e1441_d_n9 * ddt_scale);
        let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);
        let eq115_e1443_d_n7: f64 = (eq115_e1438_d_n7 + eq115_e1442_d_n7);
        let eq115_e1443_d_n9: f64 = (eq115_e1438_d_n9 + eq115_e1442_d_n9);
        (eq115_e1443, eq115_e1438_d_n0, eq115_e1438_d_n1, eq115_e1438_d_n2, eq115_e1438_d_n3, eq115_e1438_d_n4, eq115_e1438_d_n5, eq115_e1438_d_n6, eq115_e1443_d_n7, eq115_e1438_d_n8, eq115_e1443_d_n9, eq115_e1438_d_n10, eq115_e1438_d_n11, eq115_e1438_d_n12, eq115_e1438_d_n13, eq115_e1438_d_n14, eq115_e1438_d_n15, eq115_e1438_d_n16, eq115_e1438_d_n17, eq115_e1438_d_n18, eq115_e1438_d_n19, eq115_e1438_d_n20, eq115_e1438_d_n21, eq115_e1438_d_n22, eq115_e1438_d_n23, eq115_e1438_d_n24, eq115_e1438_d_n25, eq115_e1438_d_n26, eq115_e1438_d_n27, eq115_e1438_d_n28, eq115_e1438_d_n29, eq115_e1438_d_b0, eq115_e1438_d_b1, eq115_e1438_d_b2, eq115_e1438_d_b3, eq115_e1438_d_b4, eq115_e1438_d_b5, eq115_e1438_d_b6, eq115_e1438_d_b7, eq115_e1438_d_b8, eq115_e1438_d_b9, eq115_e1438_d_b10, eq115_e1438_d_b11, eq115_e1438_d_b12, eq115_e1438_d_b13, eq115_e1438_d_b14, eq115_e1438_d_b15, eq115_e1438_d_b16, eq115_e1438_d_b17, eq115_e1438_d_b18, eq115_e1438_d_b19, eq115_e1438_d_b20, eq115_e1438_d_b21, eq115_e1438_d_b22, eq115_e1438_d_b23, eq115_e1438_d_b24, eq115_e1438_d_b25, eq115_e1438_d_b26, eq115_e1438_d_b27, eq115_e1438_d_b28, eq115_e1438_d_b29, eq115_e1438_d_b30, eq115_e1438_d_b31, eq115_e1438_d_b32, eq115_e1438_d_b33, eq115_e1438_d_b34, eq115_e1438_d_b35,)
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
        let eq116_e1449_d_n0: f64 = (s.dn[179][0] * ddt_scale);
        let eq116_e1449_d_n1: f64 = (s.dn[179][1] * ddt_scale);
        let eq116_e1449_d_n2: f64 = (s.dn[179][2] * ddt_scale);
        let eq116_e1449_d_n3: f64 = (s.dn[179][3] * ddt_scale);
        let eq116_e1449_d_n4: f64 = (s.dn[179][4] * ddt_scale);
        let eq116_e1449_d_n5: f64 = (s.dn[179][5] * ddt_scale);
        let eq116_e1449_d_n6: f64 = (s.dn[179][6] * ddt_scale);
        let eq116_e1449_d_n7: f64 = (s.dn[179][7] * ddt_scale);
        let eq116_e1449_d_n8: f64 = (s.dn[179][8] * ddt_scale);
        let eq116_e1449_d_n9: f64 = (s.dn[179][9] * ddt_scale);
        let eq116_e1449_d_n10: f64 = (s.dn[179][10] * ddt_scale);
        let eq116_e1449_d_n11: f64 = (s.dn[179][11] * ddt_scale);
        let eq116_e1449_d_n12: f64 = (s.dn[179][12] * ddt_scale);
        let eq116_e1449_d_n13: f64 = (s.dn[179][13] * ddt_scale);
        let eq116_e1449_d_n14: f64 = (s.dn[179][14] * ddt_scale);
        let eq116_e1449_d_n15: f64 = (s.dn[179][15] * ddt_scale);
        let eq116_e1449_d_n16: f64 = (s.dn[179][16] * ddt_scale);
        let eq116_e1449_d_n17: f64 = (s.dn[179][17] * ddt_scale);
        let eq116_e1449_d_n18: f64 = (s.dn[179][18] * ddt_scale);
        let eq116_e1449_d_n19: f64 = (s.dn[179][19] * ddt_scale);
        let eq116_e1449_d_n20: f64 = (s.dn[179][20] * ddt_scale);
        let eq116_e1449_d_n21: f64 = (s.dn[179][21] * ddt_scale);
        let eq116_e1449_d_n22: f64 = (s.dn[179][22] * ddt_scale);
        let eq116_e1449_d_n23: f64 = (s.dn[179][23] * ddt_scale);
        let eq116_e1449_d_n24: f64 = (s.dn[179][24] * ddt_scale);
        let eq116_e1449_d_n25: f64 = (s.dn[179][25] * ddt_scale);
        let eq116_e1449_d_n26: f64 = (s.dn[179][26] * ddt_scale);
        let eq116_e1449_d_n27: f64 = (s.dn[179][27] * ddt_scale);
        let eq116_e1449_d_n28: f64 = (s.dn[179][28] * ddt_scale);
        let eq116_e1449_d_n29: f64 = (s.dn[179][29] * ddt_scale);
        let eq116_e1449_d_b0: f64 = (s.db[179][0] * ddt_scale);
        let eq116_e1449_d_b1: f64 = (s.db[179][1] * ddt_scale);
        let eq116_e1449_d_b2: f64 = (s.db[179][2] * ddt_scale);
        let eq116_e1449_d_b3: f64 = (s.db[179][3] * ddt_scale);
        let eq116_e1449_d_b4: f64 = (s.db[179][4] * ddt_scale);
        let eq116_e1449_d_b5: f64 = (s.db[179][5] * ddt_scale);
        let eq116_e1449_d_b6: f64 = (s.db[179][6] * ddt_scale);
        let eq116_e1449_d_b7: f64 = (s.db[179][7] * ddt_scale);
        let eq116_e1449_d_b8: f64 = (s.db[179][8] * ddt_scale);
        let eq116_e1449_d_b9: f64 = (s.db[179][9] * ddt_scale);
        let eq116_e1449_d_b10: f64 = (s.db[179][10] * ddt_scale);
        let eq116_e1449_d_b11: f64 = (s.db[179][11] * ddt_scale);
        let eq116_e1449_d_b12: f64 = (s.db[179][12] * ddt_scale);
        let eq116_e1449_d_b13: f64 = (s.db[179][13] * ddt_scale);
        let eq116_e1449_d_b14: f64 = (s.db[179][14] * ddt_scale);
        let eq116_e1449_d_b15: f64 = (s.db[179][15] * ddt_scale);
        let eq116_e1449_d_b16: f64 = (s.db[179][16] * ddt_scale);
        let eq116_e1449_d_b17: f64 = (s.db[179][17] * ddt_scale);
        let eq116_e1449_d_b18: f64 = (s.db[179][18] * ddt_scale);
        let eq116_e1449_d_b19: f64 = (s.db[179][19] * ddt_scale);
        let eq116_e1449_d_b20: f64 = (s.db[179][20] * ddt_scale);
        let eq116_e1449_d_b21: f64 = (s.db[179][21] * ddt_scale);
        let eq116_e1449_d_b22: f64 = (s.db[179][22] * ddt_scale);
        let eq116_e1449_d_b23: f64 = (s.db[179][23] * ddt_scale);
        let eq116_e1449_d_b24: f64 = (s.db[179][24] * ddt_scale);
        let eq116_e1449_d_b25: f64 = (s.db[179][25] * ddt_scale);
        let eq116_e1449_d_b26: f64 = (s.db[179][26] * ddt_scale);
        let eq116_e1449_d_b27: f64 = (s.db[179][27] * ddt_scale);
        let eq116_e1449_d_b28: f64 = (s.db[179][28] * ddt_scale);
        let eq116_e1449_d_b29: f64 = (s.db[179][29] * ddt_scale);
        let eq116_e1449_d_b30: f64 = (s.db[179][30] * ddt_scale);
        let eq116_e1449_d_b31: f64 = (s.db[179][31] * ddt_scale);
        let eq116_e1449_d_b32: f64 = (s.db[179][32] * ddt_scale);
        let eq116_e1449_d_b33: f64 = (s.db[179][33] * ddt_scale);
        let eq116_e1449_d_b34: f64 = (s.db[179][34] * ddt_scale);
        let eq116_e1449_d_b35: f64 = (s.db[179][35] * ddt_scale);
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1452_d_n2: f64 = p.p355;
        let eq116_e1452_d_n12: f64 = (-p.p355);
        let eq116_e1453: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 109, eq116_e1452);
        let eq116_e1453_d_n2: f64 = (eq116_e1452_d_n2 * ddt_scale);
        let eq116_e1453_d_n12: f64 = (eq116_e1452_d_n12 * ddt_scale);
        let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);
        let eq116_e1454_d_n2: f64 = (eq116_e1449_d_n2 + eq116_e1453_d_n2);
        let eq116_e1454_d_n12: f64 = (eq116_e1449_d_n12 + eq116_e1453_d_n12);
        (eq116_e1454, eq116_e1449_d_n0, eq116_e1449_d_n1, eq116_e1454_d_n2, eq116_e1449_d_n3, eq116_e1449_d_n4, eq116_e1449_d_n5, eq116_e1449_d_n6, eq116_e1449_d_n7, eq116_e1449_d_n8, eq116_e1449_d_n9, eq116_e1449_d_n10, eq116_e1449_d_n11, eq116_e1454_d_n12, eq116_e1449_d_n13, eq116_e1449_d_n14, eq116_e1449_d_n15, eq116_e1449_d_n16, eq116_e1449_d_n17, eq116_e1449_d_n18, eq116_e1449_d_n19, eq116_e1449_d_n20, eq116_e1449_d_n21, eq116_e1449_d_n22, eq116_e1449_d_n23, eq116_e1449_d_n24, eq116_e1449_d_n25, eq116_e1449_d_n26, eq116_e1449_d_n27, eq116_e1449_d_n28, eq116_e1449_d_n29, eq116_e1449_d_b0, eq116_e1449_d_b1, eq116_e1449_d_b2, eq116_e1449_d_b3, eq116_e1449_d_b4, eq116_e1449_d_b5, eq116_e1449_d_b6, eq116_e1449_d_b7, eq116_e1449_d_b8, eq116_e1449_d_b9, eq116_e1449_d_b10, eq116_e1449_d_b11, eq116_e1449_d_b12, eq116_e1449_d_b13, eq116_e1449_d_b14, eq116_e1449_d_b15, eq116_e1449_d_b16, eq116_e1449_d_b17, eq116_e1449_d_b18, eq116_e1449_d_b19, eq116_e1449_d_b20, eq116_e1449_d_b21, eq116_e1449_d_b22, eq116_e1449_d_b23, eq116_e1449_d_b24, eq116_e1449_d_b25, eq116_e1449_d_b26, eq116_e1449_d_b27, eq116_e1449_d_b28, eq116_e1449_d_b29, eq116_e1449_d_b30, eq116_e1449_d_b31, eq116_e1449_d_b32, eq116_e1449_d_b33, eq116_e1449_d_b34, eq116_e1449_d_b35,)
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
        let eq117_e1460_d_n0: f64 = (s.dn[180][0] * ddt_scale);
        let eq117_e1460_d_n1: f64 = (s.dn[180][1] * ddt_scale);
        let eq117_e1460_d_n2: f64 = (s.dn[180][2] * ddt_scale);
        let eq117_e1460_d_n3: f64 = (s.dn[180][3] * ddt_scale);
        let eq117_e1460_d_n4: f64 = (s.dn[180][4] * ddt_scale);
        let eq117_e1460_d_n5: f64 = (s.dn[180][5] * ddt_scale);
        let eq117_e1460_d_n6: f64 = (s.dn[180][6] * ddt_scale);
        let eq117_e1460_d_n7: f64 = (s.dn[180][7] * ddt_scale);
        let eq117_e1460_d_n8: f64 = (s.dn[180][8] * ddt_scale);
        let eq117_e1460_d_n9: f64 = (s.dn[180][9] * ddt_scale);
        let eq117_e1460_d_n10: f64 = (s.dn[180][10] * ddt_scale);
        let eq117_e1460_d_n11: f64 = (s.dn[180][11] * ddt_scale);
        let eq117_e1460_d_n12: f64 = (s.dn[180][12] * ddt_scale);
        let eq117_e1460_d_n13: f64 = (s.dn[180][13] * ddt_scale);
        let eq117_e1460_d_n14: f64 = (s.dn[180][14] * ddt_scale);
        let eq117_e1460_d_n15: f64 = (s.dn[180][15] * ddt_scale);
        let eq117_e1460_d_n16: f64 = (s.dn[180][16] * ddt_scale);
        let eq117_e1460_d_n17: f64 = (s.dn[180][17] * ddt_scale);
        let eq117_e1460_d_n18: f64 = (s.dn[180][18] * ddt_scale);
        let eq117_e1460_d_n19: f64 = (s.dn[180][19] * ddt_scale);
        let eq117_e1460_d_n20: f64 = (s.dn[180][20] * ddt_scale);
        let eq117_e1460_d_n21: f64 = (s.dn[180][21] * ddt_scale);
        let eq117_e1460_d_n22: f64 = (s.dn[180][22] * ddt_scale);
        let eq117_e1460_d_n23: f64 = (s.dn[180][23] * ddt_scale);
        let eq117_e1460_d_n24: f64 = (s.dn[180][24] * ddt_scale);
        let eq117_e1460_d_n25: f64 = (s.dn[180][25] * ddt_scale);
        let eq117_e1460_d_n26: f64 = (s.dn[180][26] * ddt_scale);
        let eq117_e1460_d_n27: f64 = (s.dn[180][27] * ddt_scale);
        let eq117_e1460_d_n28: f64 = (s.dn[180][28] * ddt_scale);
        let eq117_e1460_d_n29: f64 = (s.dn[180][29] * ddt_scale);
        let eq117_e1460_d_b0: f64 = (s.db[180][0] * ddt_scale);
        let eq117_e1460_d_b1: f64 = (s.db[180][1] * ddt_scale);
        let eq117_e1460_d_b2: f64 = (s.db[180][2] * ddt_scale);
        let eq117_e1460_d_b3: f64 = (s.db[180][3] * ddt_scale);
        let eq117_e1460_d_b4: f64 = (s.db[180][4] * ddt_scale);
        let eq117_e1460_d_b5: f64 = (s.db[180][5] * ddt_scale);
        let eq117_e1460_d_b6: f64 = (s.db[180][6] * ddt_scale);
        let eq117_e1460_d_b7: f64 = (s.db[180][7] * ddt_scale);
        let eq117_e1460_d_b8: f64 = (s.db[180][8] * ddt_scale);
        let eq117_e1460_d_b9: f64 = (s.db[180][9] * ddt_scale);
        let eq117_e1460_d_b10: f64 = (s.db[180][10] * ddt_scale);
        let eq117_e1460_d_b11: f64 = (s.db[180][11] * ddt_scale);
        let eq117_e1460_d_b12: f64 = (s.db[180][12] * ddt_scale);
        let eq117_e1460_d_b13: f64 = (s.db[180][13] * ddt_scale);
        let eq117_e1460_d_b14: f64 = (s.db[180][14] * ddt_scale);
        let eq117_e1460_d_b15: f64 = (s.db[180][15] * ddt_scale);
        let eq117_e1460_d_b16: f64 = (s.db[180][16] * ddt_scale);
        let eq117_e1460_d_b17: f64 = (s.db[180][17] * ddt_scale);
        let eq117_e1460_d_b18: f64 = (s.db[180][18] * ddt_scale);
        let eq117_e1460_d_b19: f64 = (s.db[180][19] * ddt_scale);
        let eq117_e1460_d_b20: f64 = (s.db[180][20] * ddt_scale);
        let eq117_e1460_d_b21: f64 = (s.db[180][21] * ddt_scale);
        let eq117_e1460_d_b22: f64 = (s.db[180][22] * ddt_scale);
        let eq117_e1460_d_b23: f64 = (s.db[180][23] * ddt_scale);
        let eq117_e1460_d_b24: f64 = (s.db[180][24] * ddt_scale);
        let eq117_e1460_d_b25: f64 = (s.db[180][25] * ddt_scale);
        let eq117_e1460_d_b26: f64 = (s.db[180][26] * ddt_scale);
        let eq117_e1460_d_b27: f64 = (s.db[180][27] * ddt_scale);
        let eq117_e1460_d_b28: f64 = (s.db[180][28] * ddt_scale);
        let eq117_e1460_d_b29: f64 = (s.db[180][29] * ddt_scale);
        let eq117_e1460_d_b30: f64 = (s.db[180][30] * ddt_scale);
        let eq117_e1460_d_b31: f64 = (s.db[180][31] * ddt_scale);
        let eq117_e1460_d_b32: f64 = (s.db[180][32] * ddt_scale);
        let eq117_e1460_d_b33: f64 = (s.db[180][33] * ddt_scale);
        let eq117_e1460_d_b34: f64 = (s.db[180][34] * ddt_scale);
        let eq117_e1460_d_b35: f64 = (s.db[180][35] * ddt_scale);
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1463_d_n2: f64 = p.p355;
        let eq117_e1463_d_n11: f64 = (-p.p355);
        let eq117_e1464: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 111, eq117_e1463);
        let eq117_e1464_d_n2: f64 = (eq117_e1463_d_n2 * ddt_scale);
        let eq117_e1464_d_n11: f64 = (eq117_e1463_d_n11 * ddt_scale);
        let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);
        let eq117_e1465_d_n2: f64 = (eq117_e1460_d_n2 + eq117_e1464_d_n2);
        let eq117_e1465_d_n11: f64 = (eq117_e1460_d_n11 + eq117_e1464_d_n11);
        (eq117_e1465, eq117_e1460_d_n0, eq117_e1460_d_n1, eq117_e1465_d_n2, eq117_e1460_d_n3, eq117_e1460_d_n4, eq117_e1460_d_n5, eq117_e1460_d_n6, eq117_e1460_d_n7, eq117_e1460_d_n8, eq117_e1460_d_n9, eq117_e1460_d_n10, eq117_e1465_d_n11, eq117_e1460_d_n12, eq117_e1460_d_n13, eq117_e1460_d_n14, eq117_e1460_d_n15, eq117_e1460_d_n16, eq117_e1460_d_n17, eq117_e1460_d_n18, eq117_e1460_d_n19, eq117_e1460_d_n20, eq117_e1460_d_n21, eq117_e1460_d_n22, eq117_e1460_d_n23, eq117_e1460_d_n24, eq117_e1460_d_n25, eq117_e1460_d_n26, eq117_e1460_d_n27, eq117_e1460_d_n28, eq117_e1460_d_n29, eq117_e1460_d_b0, eq117_e1460_d_b1, eq117_e1460_d_b2, eq117_e1460_d_b3, eq117_e1460_d_b4, eq117_e1460_d_b5, eq117_e1460_d_b6, eq117_e1460_d_b7, eq117_e1460_d_b8, eq117_e1460_d_b9, eq117_e1460_d_b10, eq117_e1460_d_b11, eq117_e1460_d_b12, eq117_e1460_d_b13, eq117_e1460_d_b14, eq117_e1460_d_b15, eq117_e1460_d_b16, eq117_e1460_d_b17, eq117_e1460_d_b18, eq117_e1460_d_b19, eq117_e1460_d_b20, eq117_e1460_d_b21, eq117_e1460_d_b22, eq117_e1460_d_b23, eq117_e1460_d_b24, eq117_e1460_d_b25, eq117_e1460_d_b26, eq117_e1460_d_b27, eq117_e1460_d_b28, eq117_e1460_d_b29, eq117_e1460_d_b30, eq117_e1460_d_b31, eq117_e1460_d_b32, eq117_e1460_d_b33, eq117_e1460_d_b34, eq117_e1460_d_b35,)
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
    }

    pub(super) fn stamp_transient_equations_block_16(
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29, eq118_e1478_d_b0, eq118_e1478_d_b1, eq118_e1478_d_b2, eq118_e1478_d_b3, eq118_e1478_d_b4, eq118_e1478_d_b5, eq118_e1478_d_b6, eq118_e1478_d_b7, eq118_e1478_d_b8, eq118_e1478_d_b9, eq118_e1478_d_b10, eq118_e1478_d_b11, eq118_e1478_d_b12, eq118_e1478_d_b13, eq118_e1478_d_b14, eq118_e1478_d_b15, eq118_e1478_d_b16, eq118_e1478_d_b17, eq118_e1478_d_b18, eq118_e1478_d_b19, eq118_e1478_d_b20, eq118_e1478_d_b21, eq118_e1478_d_b22, eq118_e1478_d_b23, eq118_e1478_d_b24, eq118_e1478_d_b25, eq118_e1478_d_b26, eq118_e1478_d_b27, eq118_e1478_d_b28, eq118_e1478_d_b29, eq118_e1478_d_b30, eq118_e1478_d_b31, eq118_e1478_d_b32, eq118_e1478_d_b33, eq118_e1478_d_b34, eq118_e1478_d_b35,) = {
    if (!s.b[1348]) {
        let eq118_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 112, s.v[181]);
        let eq118_e1471_d_n0: f64 = (s.dn[181][0] * ddt_scale);
        let eq118_e1471_d_n1: f64 = (s.dn[181][1] * ddt_scale);
        let eq118_e1471_d_n2: f64 = (s.dn[181][2] * ddt_scale);
        let eq118_e1471_d_n3: f64 = (s.dn[181][3] * ddt_scale);
        let eq118_e1471_d_n4: f64 = (s.dn[181][4] * ddt_scale);
        let eq118_e1471_d_n5: f64 = (s.dn[181][5] * ddt_scale);
        let eq118_e1471_d_n6: f64 = (s.dn[181][6] * ddt_scale);
        let eq118_e1471_d_n7: f64 = (s.dn[181][7] * ddt_scale);
        let eq118_e1471_d_n8: f64 = (s.dn[181][8] * ddt_scale);
        let eq118_e1471_d_n9: f64 = (s.dn[181][9] * ddt_scale);
        let eq118_e1471_d_n10: f64 = (s.dn[181][10] * ddt_scale);
        let eq118_e1471_d_n11: f64 = (s.dn[181][11] * ddt_scale);
        let eq118_e1471_d_n12: f64 = (s.dn[181][12] * ddt_scale);
        let eq118_e1471_d_n13: f64 = (s.dn[181][13] * ddt_scale);
        let eq118_e1471_d_n14: f64 = (s.dn[181][14] * ddt_scale);
        let eq118_e1471_d_n15: f64 = (s.dn[181][15] * ddt_scale);
        let eq118_e1471_d_n16: f64 = (s.dn[181][16] * ddt_scale);
        let eq118_e1471_d_n17: f64 = (s.dn[181][17] * ddt_scale);
        let eq118_e1471_d_n18: f64 = (s.dn[181][18] * ddt_scale);
        let eq118_e1471_d_n19: f64 = (s.dn[181][19] * ddt_scale);
        let eq118_e1471_d_n20: f64 = (s.dn[181][20] * ddt_scale);
        let eq118_e1471_d_n21: f64 = (s.dn[181][21] * ddt_scale);
        let eq118_e1471_d_n22: f64 = (s.dn[181][22] * ddt_scale);
        let eq118_e1471_d_n23: f64 = (s.dn[181][23] * ddt_scale);
        let eq118_e1471_d_n24: f64 = (s.dn[181][24] * ddt_scale);
        let eq118_e1471_d_n25: f64 = (s.dn[181][25] * ddt_scale);
        let eq118_e1471_d_n26: f64 = (s.dn[181][26] * ddt_scale);
        let eq118_e1471_d_n27: f64 = (s.dn[181][27] * ddt_scale);
        let eq118_e1471_d_n28: f64 = (s.dn[181][28] * ddt_scale);
        let eq118_e1471_d_n29: f64 = (s.dn[181][29] * ddt_scale);
        let eq118_e1471_d_b0: f64 = (s.db[181][0] * ddt_scale);
        let eq118_e1471_d_b1: f64 = (s.db[181][1] * ddt_scale);
        let eq118_e1471_d_b2: f64 = (s.db[181][2] * ddt_scale);
        let eq118_e1471_d_b3: f64 = (s.db[181][3] * ddt_scale);
        let eq118_e1471_d_b4: f64 = (s.db[181][4] * ddt_scale);
        let eq118_e1471_d_b5: f64 = (s.db[181][5] * ddt_scale);
        let eq118_e1471_d_b6: f64 = (s.db[181][6] * ddt_scale);
        let eq118_e1471_d_b7: f64 = (s.db[181][7] * ddt_scale);
        let eq118_e1471_d_b8: f64 = (s.db[181][8] * ddt_scale);
        let eq118_e1471_d_b9: f64 = (s.db[181][9] * ddt_scale);
        let eq118_e1471_d_b10: f64 = (s.db[181][10] * ddt_scale);
        let eq118_e1471_d_b11: f64 = (s.db[181][11] * ddt_scale);
        let eq118_e1471_d_b12: f64 = (s.db[181][12] * ddt_scale);
        let eq118_e1471_d_b13: f64 = (s.db[181][13] * ddt_scale);
        let eq118_e1471_d_b14: f64 = (s.db[181][14] * ddt_scale);
        let eq118_e1471_d_b15: f64 = (s.db[181][15] * ddt_scale);
        let eq118_e1471_d_b16: f64 = (s.db[181][16] * ddt_scale);
        let eq118_e1471_d_b17: f64 = (s.db[181][17] * ddt_scale);
        let eq118_e1471_d_b18: f64 = (s.db[181][18] * ddt_scale);
        let eq118_e1471_d_b19: f64 = (s.db[181][19] * ddt_scale);
        let eq118_e1471_d_b20: f64 = (s.db[181][20] * ddt_scale);
        let eq118_e1471_d_b21: f64 = (s.db[181][21] * ddt_scale);
        let eq118_e1471_d_b22: f64 = (s.db[181][22] * ddt_scale);
        let eq118_e1471_d_b23: f64 = (s.db[181][23] * ddt_scale);
        let eq118_e1471_d_b24: f64 = (s.db[181][24] * ddt_scale);
        let eq118_e1471_d_b25: f64 = (s.db[181][25] * ddt_scale);
        let eq118_e1471_d_b26: f64 = (s.db[181][26] * ddt_scale);
        let eq118_e1471_d_b27: f64 = (s.db[181][27] * ddt_scale);
        let eq118_e1471_d_b28: f64 = (s.db[181][28] * ddt_scale);
        let eq118_e1471_d_b29: f64 = (s.db[181][29] * ddt_scale);
        let eq118_e1471_d_b30: f64 = (s.db[181][30] * ddt_scale);
        let eq118_e1471_d_b31: f64 = (s.db[181][31] * ddt_scale);
        let eq118_e1471_d_b32: f64 = (s.db[181][32] * ddt_scale);
        let eq118_e1471_d_b33: f64 = (s.db[181][33] * ddt_scale);
        let eq118_e1471_d_b34: f64 = (s.db[181][34] * ddt_scale);
        let eq118_e1471_d_b35: f64 = (s.db[181][35] * ddt_scale);
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1474_d_n7: f64 = p.p355;
        let eq118_e1474_d_n12: f64 = (-p.p355);
        let eq118_e1475: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 113, eq118_e1474);
        let eq118_e1475_d_n7: f64 = (eq118_e1474_d_n7 * ddt_scale);
        let eq118_e1475_d_n12: f64 = (eq118_e1474_d_n12 * ddt_scale);
        let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);
        let eq118_e1476_d_n7: f64 = (eq118_e1471_d_n7 + eq118_e1475_d_n7);
        let eq118_e1476_d_n12: f64 = (eq118_e1471_d_n12 + eq118_e1475_d_n12);
        (eq118_e1476, eq118_e1471_d_n0, eq118_e1471_d_n1, eq118_e1471_d_n2, eq118_e1471_d_n3, eq118_e1471_d_n4, eq118_e1471_d_n5, eq118_e1471_d_n6, eq118_e1476_d_n7, eq118_e1471_d_n8, eq118_e1471_d_n9, eq118_e1471_d_n10, eq118_e1471_d_n11, eq118_e1476_d_n12, eq118_e1471_d_n13, eq118_e1471_d_n14, eq118_e1471_d_n15, eq118_e1471_d_n16, eq118_e1471_d_n17, eq118_e1471_d_n18, eq118_e1471_d_n19, eq118_e1471_d_n20, eq118_e1471_d_n21, eq118_e1471_d_n22, eq118_e1471_d_n23, eq118_e1471_d_n24, eq118_e1471_d_n25, eq118_e1471_d_n26, eq118_e1471_d_n27, eq118_e1471_d_n28, eq118_e1471_d_n29, eq118_e1471_d_b0, eq118_e1471_d_b1, eq118_e1471_d_b2, eq118_e1471_d_b3, eq118_e1471_d_b4, eq118_e1471_d_b5, eq118_e1471_d_b6, eq118_e1471_d_b7, eq118_e1471_d_b8, eq118_e1471_d_b9, eq118_e1471_d_b10, eq118_e1471_d_b11, eq118_e1471_d_b12, eq118_e1471_d_b13, eq118_e1471_d_b14, eq118_e1471_d_b15, eq118_e1471_d_b16, eq118_e1471_d_b17, eq118_e1471_d_b18, eq118_e1471_d_b19, eq118_e1471_d_b20, eq118_e1471_d_b21, eq118_e1471_d_b22, eq118_e1471_d_b23, eq118_e1471_d_b24, eq118_e1471_d_b25, eq118_e1471_d_b26, eq118_e1471_d_b27, eq118_e1471_d_b28, eq118_e1471_d_b29, eq118_e1471_d_b30, eq118_e1471_d_b31, eq118_e1471_d_b32, eq118_e1471_d_b33, eq118_e1471_d_b34, eq118_e1471_d_b35,)
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
        let (eq119_e1483,) = {
    if (!s.b[1348]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq119_value: f64 = eq119_e1483;
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (eq119_value),
        );
        let (eq120_e1488,) = {
    if (!s.b[1348]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq120_value: f64 = eq120_e1488;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq120_value),
        );
        let eq121_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 114, s.v[182]);
        let eq121_e1490_d_n0: f64 = (s.dn[182][0] * ddt_scale);
        let eq121_e1490_d_n1: f64 = (s.dn[182][1] * ddt_scale);
        let eq121_e1490_d_n2: f64 = (s.dn[182][2] * ddt_scale);
        let eq121_e1490_d_n3: f64 = (s.dn[182][3] * ddt_scale);
        let eq121_e1490_d_n4: f64 = (s.dn[182][4] * ddt_scale);
        let eq121_e1490_d_n5: f64 = (s.dn[182][5] * ddt_scale);
        let eq121_e1490_d_n6: f64 = (s.dn[182][6] * ddt_scale);
        let eq121_e1490_d_n7: f64 = (s.dn[182][7] * ddt_scale);
        let eq121_e1490_d_n8: f64 = (s.dn[182][8] * ddt_scale);
        let eq121_e1490_d_n9: f64 = (s.dn[182][9] * ddt_scale);
        let eq121_e1490_d_n10: f64 = (s.dn[182][10] * ddt_scale);
        let eq121_e1490_d_n11: f64 = (s.dn[182][11] * ddt_scale);
        let eq121_e1490_d_n12: f64 = (s.dn[182][12] * ddt_scale);
        let eq121_e1490_d_n13: f64 = (s.dn[182][13] * ddt_scale);
        let eq121_e1490_d_n14: f64 = (s.dn[182][14] * ddt_scale);
        let eq121_e1490_d_n15: f64 = (s.dn[182][15] * ddt_scale);
        let eq121_e1490_d_n16: f64 = (s.dn[182][16] * ddt_scale);
        let eq121_e1490_d_n17: f64 = (s.dn[182][17] * ddt_scale);
        let eq121_e1490_d_n18: f64 = (s.dn[182][18] * ddt_scale);
        let eq121_e1490_d_n19: f64 = (s.dn[182][19] * ddt_scale);
        let eq121_e1490_d_n20: f64 = (s.dn[182][20] * ddt_scale);
        let eq121_e1490_d_n21: f64 = (s.dn[182][21] * ddt_scale);
        let eq121_e1490_d_n22: f64 = (s.dn[182][22] * ddt_scale);
        let eq121_e1490_d_n23: f64 = (s.dn[182][23] * ddt_scale);
        let eq121_e1490_d_n24: f64 = (s.dn[182][24] * ddt_scale);
        let eq121_e1490_d_n25: f64 = (s.dn[182][25] * ddt_scale);
        let eq121_e1490_d_n26: f64 = (s.dn[182][26] * ddt_scale);
        let eq121_e1490_d_n27: f64 = (s.dn[182][27] * ddt_scale);
        let eq121_e1490_d_n28: f64 = (s.dn[182][28] * ddt_scale);
        let eq121_e1490_d_n29: f64 = (s.dn[182][29] * ddt_scale);
        let eq121_e1490_d_b0: f64 = (s.db[182][0] * ddt_scale);
        let eq121_e1490_d_b1: f64 = (s.db[182][1] * ddt_scale);
        let eq121_e1490_d_b2: f64 = (s.db[182][2] * ddt_scale);
        let eq121_e1490_d_b3: f64 = (s.db[182][3] * ddt_scale);
        let eq121_e1490_d_b4: f64 = (s.db[182][4] * ddt_scale);
        let eq121_e1490_d_b5: f64 = (s.db[182][5] * ddt_scale);
        let eq121_e1490_d_b6: f64 = (s.db[182][6] * ddt_scale);
        let eq121_e1490_d_b7: f64 = (s.db[182][7] * ddt_scale);
        let eq121_e1490_d_b8: f64 = (s.db[182][8] * ddt_scale);
        let eq121_e1490_d_b9: f64 = (s.db[182][9] * ddt_scale);
        let eq121_e1490_d_b10: f64 = (s.db[182][10] * ddt_scale);
        let eq121_e1490_d_b11: f64 = (s.db[182][11] * ddt_scale);
        let eq121_e1490_d_b12: f64 = (s.db[182][12] * ddt_scale);
        let eq121_e1490_d_b13: f64 = (s.db[182][13] * ddt_scale);
        let eq121_e1490_d_b14: f64 = (s.db[182][14] * ddt_scale);
        let eq121_e1490_d_b15: f64 = (s.db[182][15] * ddt_scale);
        let eq121_e1490_d_b16: f64 = (s.db[182][16] * ddt_scale);
        let eq121_e1490_d_b17: f64 = (s.db[182][17] * ddt_scale);
        let eq121_e1490_d_b18: f64 = (s.db[182][18] * ddt_scale);
        let eq121_e1490_d_b19: f64 = (s.db[182][19] * ddt_scale);
        let eq121_e1490_d_b20: f64 = (s.db[182][20] * ddt_scale);
        let eq121_e1490_d_b21: f64 = (s.db[182][21] * ddt_scale);
        let eq121_e1490_d_b22: f64 = (s.db[182][22] * ddt_scale);
        let eq121_e1490_d_b23: f64 = (s.db[182][23] * ddt_scale);
        let eq121_e1490_d_b24: f64 = (s.db[182][24] * ddt_scale);
        let eq121_e1490_d_b25: f64 = (s.db[182][25] * ddt_scale);
        let eq121_e1490_d_b26: f64 = (s.db[182][26] * ddt_scale);
        let eq121_e1490_d_b27: f64 = (s.db[182][27] * ddt_scale);
        let eq121_e1490_d_b28: f64 = (s.db[182][28] * ddt_scale);
        let eq121_e1490_d_b29: f64 = (s.db[182][29] * ddt_scale);
        let eq121_e1490_d_b30: f64 = (s.db[182][30] * ddt_scale);
        let eq121_e1490_d_b31: f64 = (s.db[182][31] * ddt_scale);
        let eq121_e1490_d_b32: f64 = (s.db[182][32] * ddt_scale);
        let eq121_e1490_d_b33: f64 = (s.db[182][33] * ddt_scale);
        let eq121_e1490_d_b34: f64 = (s.db[182][34] * ddt_scale);
        let eq121_e1490_d_b35: f64 = (s.db[182][35] * ddt_scale);
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1493_d_n3: f64 = p.p355;
        let eq121_e1493_d_n12: f64 = (-p.p355);
        let eq121_e1494: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 115, eq121_e1493);
        let eq121_e1494_d_n3: f64 = (eq121_e1493_d_n3 * ddt_scale);
        let eq121_e1494_d_n12: f64 = (eq121_e1493_d_n12 * ddt_scale);
        let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);
        let eq121_e1495_d_n3: f64 = (eq121_e1490_d_n3 + eq121_e1494_d_n3);
        let eq121_e1495_d_n12: f64 = (eq121_e1490_d_n12 + eq121_e1494_d_n12);
        let eq121_value: f64 = eq121_e1495;
        let eq121_node_derivatives: [f64; 30] = [eq121_e1490_d_n0, eq121_e1490_d_n1, eq121_e1490_d_n2, eq121_e1495_d_n3, eq121_e1490_d_n4, eq121_e1490_d_n5, eq121_e1490_d_n6, eq121_e1490_d_n7, eq121_e1490_d_n8, eq121_e1490_d_n9, eq121_e1490_d_n10, eq121_e1490_d_n11, eq121_e1495_d_n12, eq121_e1490_d_n13, eq121_e1490_d_n14, eq121_e1490_d_n15, eq121_e1490_d_n16, eq121_e1490_d_n17, eq121_e1490_d_n18, eq121_e1490_d_n19, eq121_e1490_d_n20, eq121_e1490_d_n21, eq121_e1490_d_n22, eq121_e1490_d_n23, eq121_e1490_d_n24, eq121_e1490_d_n25, eq121_e1490_d_n26, eq121_e1490_d_n27, eq121_e1490_d_n28, eq121_e1490_d_n29];
        let eq121_branch_derivatives: [f64; 36] = [eq121_e1490_d_b0, eq121_e1490_d_b1, eq121_e1490_d_b2, eq121_e1490_d_b3, eq121_e1490_d_b4, eq121_e1490_d_b5, eq121_e1490_d_b6, eq121_e1490_d_b7, eq121_e1490_d_b8, eq121_e1490_d_b9, eq121_e1490_d_b10, eq121_e1490_d_b11, eq121_e1490_d_b12, eq121_e1490_d_b13, eq121_e1490_d_b14, eq121_e1490_d_b15, eq121_e1490_d_b16, eq121_e1490_d_b17, eq121_e1490_d_b18, eq121_e1490_d_b19, eq121_e1490_d_b20, eq121_e1490_d_b21, eq121_e1490_d_b22, eq121_e1490_d_b23, eq121_e1490_d_b24, eq121_e1490_d_b25, eq121_e1490_d_b26, eq121_e1490_d_b27, eq121_e1490_d_b28, eq121_e1490_d_b29, eq121_e1490_d_b30, eq121_e1490_d_b31, eq121_e1490_d_b32, eq121_e1490_d_b33, eq121_e1490_d_b34, eq121_e1490_d_b35];
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
        let eq122_e1500_d_n12: f64 = s.v[0];
        let eq122_e1500_d_n13: f64 = (-s.v[0]);
        let eq122_e1501: f64 = (s.v[184] + eq122_e1500);
        let eq122_e1501_d_n12: f64 = (s.dn[184][12] + eq122_e1500_d_n12);
        let eq122_e1501_d_n13: f64 = (s.dn[184][13] + eq122_e1500_d_n13);
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
        let (eq123_e1508,) = {
    if (!s.b[1349]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq123_value: f64 = eq123_e1508;
        stamper.stamp_potential_const_local(
            25,
            eq123_value,
        );
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29, eq124_e1518_d_b0, eq124_e1518_d_b1, eq124_e1518_d_b2, eq124_e1518_d_b3, eq124_e1518_d_b4, eq124_e1518_d_b5, eq124_e1518_d_b6, eq124_e1518_d_b7, eq124_e1518_d_b8, eq124_e1518_d_b9, eq124_e1518_d_b10, eq124_e1518_d_b11, eq124_e1518_d_b12, eq124_e1518_d_b13, eq124_e1518_d_b14, eq124_e1518_d_b15, eq124_e1518_d_b16, eq124_e1518_d_b17, eq124_e1518_d_b18, eq124_e1518_d_b19, eq124_e1518_d_b20, eq124_e1518_d_b21, eq124_e1518_d_b22, eq124_e1518_d_b23, eq124_e1518_d_b24, eq124_e1518_d_b25, eq124_e1518_d_b26, eq124_e1518_d_b27, eq124_e1518_d_b28, eq124_e1518_d_b29, eq124_e1518_d_b30, eq124_e1518_d_b31, eq124_e1518_d_b32, eq124_e1518_d_b33, eq124_e1518_d_b34, eq124_e1518_d_b35,) = {
    if s.b[1495] {
        let eq124_e1511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 116, s.v[185]);
        let eq124_e1511_d_n0: f64 = (s.dn[185][0] * ddt_scale);
        let eq124_e1511_d_n1: f64 = (s.dn[185][1] * ddt_scale);
        let eq124_e1511_d_n2: f64 = (s.dn[185][2] * ddt_scale);
        let eq124_e1511_d_n3: f64 = (s.dn[185][3] * ddt_scale);
        let eq124_e1511_d_n4: f64 = (s.dn[185][4] * ddt_scale);
        let eq124_e1511_d_n5: f64 = (s.dn[185][5] * ddt_scale);
        let eq124_e1511_d_n6: f64 = (s.dn[185][6] * ddt_scale);
        let eq124_e1511_d_n7: f64 = (s.dn[185][7] * ddt_scale);
        let eq124_e1511_d_n8: f64 = (s.dn[185][8] * ddt_scale);
        let eq124_e1511_d_n9: f64 = (s.dn[185][9] * ddt_scale);
        let eq124_e1511_d_n10: f64 = (s.dn[185][10] * ddt_scale);
        let eq124_e1511_d_n11: f64 = (s.dn[185][11] * ddt_scale);
        let eq124_e1511_d_n12: f64 = (s.dn[185][12] * ddt_scale);
        let eq124_e1511_d_n13: f64 = (s.dn[185][13] * ddt_scale);
        let eq124_e1511_d_n14: f64 = (s.dn[185][14] * ddt_scale);
        let eq124_e1511_d_n15: f64 = (s.dn[185][15] * ddt_scale);
        let eq124_e1511_d_n16: f64 = (s.dn[185][16] * ddt_scale);
        let eq124_e1511_d_n17: f64 = (s.dn[185][17] * ddt_scale);
        let eq124_e1511_d_n18: f64 = (s.dn[185][18] * ddt_scale);
        let eq124_e1511_d_n19: f64 = (s.dn[185][19] * ddt_scale);
        let eq124_e1511_d_n20: f64 = (s.dn[185][20] * ddt_scale);
        let eq124_e1511_d_n21: f64 = (s.dn[185][21] * ddt_scale);
        let eq124_e1511_d_n22: f64 = (s.dn[185][22] * ddt_scale);
        let eq124_e1511_d_n23: f64 = (s.dn[185][23] * ddt_scale);
        let eq124_e1511_d_n24: f64 = (s.dn[185][24] * ddt_scale);
        let eq124_e1511_d_n25: f64 = (s.dn[185][25] * ddt_scale);
        let eq124_e1511_d_n26: f64 = (s.dn[185][26] * ddt_scale);
        let eq124_e1511_d_n27: f64 = (s.dn[185][27] * ddt_scale);
        let eq124_e1511_d_n28: f64 = (s.dn[185][28] * ddt_scale);
        let eq124_e1511_d_n29: f64 = (s.dn[185][29] * ddt_scale);
        let eq124_e1511_d_b0: f64 = (s.db[185][0] * ddt_scale);
        let eq124_e1511_d_b1: f64 = (s.db[185][1] * ddt_scale);
        let eq124_e1511_d_b2: f64 = (s.db[185][2] * ddt_scale);
        let eq124_e1511_d_b3: f64 = (s.db[185][3] * ddt_scale);
        let eq124_e1511_d_b4: f64 = (s.db[185][4] * ddt_scale);
        let eq124_e1511_d_b5: f64 = (s.db[185][5] * ddt_scale);
        let eq124_e1511_d_b6: f64 = (s.db[185][6] * ddt_scale);
        let eq124_e1511_d_b7: f64 = (s.db[185][7] * ddt_scale);
        let eq124_e1511_d_b8: f64 = (s.db[185][8] * ddt_scale);
        let eq124_e1511_d_b9: f64 = (s.db[185][9] * ddt_scale);
        let eq124_e1511_d_b10: f64 = (s.db[185][10] * ddt_scale);
        let eq124_e1511_d_b11: f64 = (s.db[185][11] * ddt_scale);
        let eq124_e1511_d_b12: f64 = (s.db[185][12] * ddt_scale);
        let eq124_e1511_d_b13: f64 = (s.db[185][13] * ddt_scale);
        let eq124_e1511_d_b14: f64 = (s.db[185][14] * ddt_scale);
        let eq124_e1511_d_b15: f64 = (s.db[185][15] * ddt_scale);
        let eq124_e1511_d_b16: f64 = (s.db[185][16] * ddt_scale);
        let eq124_e1511_d_b17: f64 = (s.db[185][17] * ddt_scale);
        let eq124_e1511_d_b18: f64 = (s.db[185][18] * ddt_scale);
        let eq124_e1511_d_b19: f64 = (s.db[185][19] * ddt_scale);
        let eq124_e1511_d_b20: f64 = (s.db[185][20] * ddt_scale);
        let eq124_e1511_d_b21: f64 = (s.db[185][21] * ddt_scale);
        let eq124_e1511_d_b22: f64 = (s.db[185][22] * ddt_scale);
        let eq124_e1511_d_b23: f64 = (s.db[185][23] * ddt_scale);
        let eq124_e1511_d_b24: f64 = (s.db[185][24] * ddt_scale);
        let eq124_e1511_d_b25: f64 = (s.db[185][25] * ddt_scale);
        let eq124_e1511_d_b26: f64 = (s.db[185][26] * ddt_scale);
        let eq124_e1511_d_b27: f64 = (s.db[185][27] * ddt_scale);
        let eq124_e1511_d_b28: f64 = (s.db[185][28] * ddt_scale);
        let eq124_e1511_d_b29: f64 = (s.db[185][29] * ddt_scale);
        let eq124_e1511_d_b30: f64 = (s.db[185][30] * ddt_scale);
        let eq124_e1511_d_b31: f64 = (s.db[185][31] * ddt_scale);
        let eq124_e1511_d_b32: f64 = (s.db[185][32] * ddt_scale);
        let eq124_e1511_d_b33: f64 = (s.db[185][33] * ddt_scale);
        let eq124_e1511_d_b34: f64 = (s.db[185][34] * ddt_scale);
        let eq124_e1511_d_b35: f64 = (s.db[185][35] * ddt_scale);
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1514_d_n7: f64 = p.p355;
        let eq124_e1514_d_n13: f64 = (-p.p355);
        let eq124_e1515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 117, eq124_e1514);
        let eq124_e1515_d_n7: f64 = (eq124_e1514_d_n7 * ddt_scale);
        let eq124_e1515_d_n13: f64 = (eq124_e1514_d_n13 * ddt_scale);
        let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);
        let eq124_e1516_d_n7: f64 = (eq124_e1511_d_n7 + eq124_e1515_d_n7);
        let eq124_e1516_d_n13: f64 = (eq124_e1511_d_n13 + eq124_e1515_d_n13);
        (eq124_e1516, eq124_e1511_d_n0, eq124_e1511_d_n1, eq124_e1511_d_n2, eq124_e1511_d_n3, eq124_e1511_d_n4, eq124_e1511_d_n5, eq124_e1511_d_n6, eq124_e1516_d_n7, eq124_e1511_d_n8, eq124_e1511_d_n9, eq124_e1511_d_n10, eq124_e1511_d_n11, eq124_e1511_d_n12, eq124_e1516_d_n13, eq124_e1511_d_n14, eq124_e1511_d_n15, eq124_e1511_d_n16, eq124_e1511_d_n17, eq124_e1511_d_n18, eq124_e1511_d_n19, eq124_e1511_d_n20, eq124_e1511_d_n21, eq124_e1511_d_n22, eq124_e1511_d_n23, eq124_e1511_d_n24, eq124_e1511_d_n25, eq124_e1511_d_n26, eq124_e1511_d_n27, eq124_e1511_d_n28, eq124_e1511_d_n29, eq124_e1511_d_b0, eq124_e1511_d_b1, eq124_e1511_d_b2, eq124_e1511_d_b3, eq124_e1511_d_b4, eq124_e1511_d_b5, eq124_e1511_d_b6, eq124_e1511_d_b7, eq124_e1511_d_b8, eq124_e1511_d_b9, eq124_e1511_d_b10, eq124_e1511_d_b11, eq124_e1511_d_b12, eq124_e1511_d_b13, eq124_e1511_d_b14, eq124_e1511_d_b15, eq124_e1511_d_b16, eq124_e1511_d_b17, eq124_e1511_d_b18, eq124_e1511_d_b19, eq124_e1511_d_b20, eq124_e1511_d_b21, eq124_e1511_d_b22, eq124_e1511_d_b23, eq124_e1511_d_b24, eq124_e1511_d_b25, eq124_e1511_d_b26, eq124_e1511_d_b27, eq124_e1511_d_b28, eq124_e1511_d_b29, eq124_e1511_d_b30, eq124_e1511_d_b31, eq124_e1511_d_b32, eq124_e1511_d_b33, eq124_e1511_d_b34, eq124_e1511_d_b35,)
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
        let eq125_e1521_d_n0: f64 = (s.dn[186][0] * ddt_scale);
        let eq125_e1521_d_n1: f64 = (s.dn[186][1] * ddt_scale);
        let eq125_e1521_d_n2: f64 = (s.dn[186][2] * ddt_scale);
        let eq125_e1521_d_n3: f64 = (s.dn[186][3] * ddt_scale);
        let eq125_e1521_d_n4: f64 = (s.dn[186][4] * ddt_scale);
        let eq125_e1521_d_n5: f64 = (s.dn[186][5] * ddt_scale);
        let eq125_e1521_d_n6: f64 = (s.dn[186][6] * ddt_scale);
        let eq125_e1521_d_n7: f64 = (s.dn[186][7] * ddt_scale);
        let eq125_e1521_d_n8: f64 = (s.dn[186][8] * ddt_scale);
        let eq125_e1521_d_n9: f64 = (s.dn[186][9] * ddt_scale);
        let eq125_e1521_d_n10: f64 = (s.dn[186][10] * ddt_scale);
        let eq125_e1521_d_n11: f64 = (s.dn[186][11] * ddt_scale);
        let eq125_e1521_d_n12: f64 = (s.dn[186][12] * ddt_scale);
        let eq125_e1521_d_n13: f64 = (s.dn[186][13] * ddt_scale);
        let eq125_e1521_d_n14: f64 = (s.dn[186][14] * ddt_scale);
        let eq125_e1521_d_n15: f64 = (s.dn[186][15] * ddt_scale);
        let eq125_e1521_d_n16: f64 = (s.dn[186][16] * ddt_scale);
        let eq125_e1521_d_n17: f64 = (s.dn[186][17] * ddt_scale);
        let eq125_e1521_d_n18: f64 = (s.dn[186][18] * ddt_scale);
        let eq125_e1521_d_n19: f64 = (s.dn[186][19] * ddt_scale);
        let eq125_e1521_d_n20: f64 = (s.dn[186][20] * ddt_scale);
        let eq125_e1521_d_n21: f64 = (s.dn[186][21] * ddt_scale);
        let eq125_e1521_d_n22: f64 = (s.dn[186][22] * ddt_scale);
        let eq125_e1521_d_n23: f64 = (s.dn[186][23] * ddt_scale);
        let eq125_e1521_d_n24: f64 = (s.dn[186][24] * ddt_scale);
        let eq125_e1521_d_n25: f64 = (s.dn[186][25] * ddt_scale);
        let eq125_e1521_d_n26: f64 = (s.dn[186][26] * ddt_scale);
        let eq125_e1521_d_n27: f64 = (s.dn[186][27] * ddt_scale);
        let eq125_e1521_d_n28: f64 = (s.dn[186][28] * ddt_scale);
        let eq125_e1521_d_n29: f64 = (s.dn[186][29] * ddt_scale);
        let eq125_e1521_d_b0: f64 = (s.db[186][0] * ddt_scale);
        let eq125_e1521_d_b1: f64 = (s.db[186][1] * ddt_scale);
        let eq125_e1521_d_b2: f64 = (s.db[186][2] * ddt_scale);
        let eq125_e1521_d_b3: f64 = (s.db[186][3] * ddt_scale);
        let eq125_e1521_d_b4: f64 = (s.db[186][4] * ddt_scale);
        let eq125_e1521_d_b5: f64 = (s.db[186][5] * ddt_scale);
        let eq125_e1521_d_b6: f64 = (s.db[186][6] * ddt_scale);
        let eq125_e1521_d_b7: f64 = (s.db[186][7] * ddt_scale);
        let eq125_e1521_d_b8: f64 = (s.db[186][8] * ddt_scale);
        let eq125_e1521_d_b9: f64 = (s.db[186][9] * ddt_scale);
        let eq125_e1521_d_b10: f64 = (s.db[186][10] * ddt_scale);
        let eq125_e1521_d_b11: f64 = (s.db[186][11] * ddt_scale);
        let eq125_e1521_d_b12: f64 = (s.db[186][12] * ddt_scale);
        let eq125_e1521_d_b13: f64 = (s.db[186][13] * ddt_scale);
        let eq125_e1521_d_b14: f64 = (s.db[186][14] * ddt_scale);
        let eq125_e1521_d_b15: f64 = (s.db[186][15] * ddt_scale);
        let eq125_e1521_d_b16: f64 = (s.db[186][16] * ddt_scale);
        let eq125_e1521_d_b17: f64 = (s.db[186][17] * ddt_scale);
        let eq125_e1521_d_b18: f64 = (s.db[186][18] * ddt_scale);
        let eq125_e1521_d_b19: f64 = (s.db[186][19] * ddt_scale);
        let eq125_e1521_d_b20: f64 = (s.db[186][20] * ddt_scale);
        let eq125_e1521_d_b21: f64 = (s.db[186][21] * ddt_scale);
        let eq125_e1521_d_b22: f64 = (s.db[186][22] * ddt_scale);
        let eq125_e1521_d_b23: f64 = (s.db[186][23] * ddt_scale);
        let eq125_e1521_d_b24: f64 = (s.db[186][24] * ddt_scale);
        let eq125_e1521_d_b25: f64 = (s.db[186][25] * ddt_scale);
        let eq125_e1521_d_b26: f64 = (s.db[186][26] * ddt_scale);
        let eq125_e1521_d_b27: f64 = (s.db[186][27] * ddt_scale);
        let eq125_e1521_d_b28: f64 = (s.db[186][28] * ddt_scale);
        let eq125_e1521_d_b29: f64 = (s.db[186][29] * ddt_scale);
        let eq125_e1521_d_b30: f64 = (s.db[186][30] * ddt_scale);
        let eq125_e1521_d_b31: f64 = (s.db[186][31] * ddt_scale);
        let eq125_e1521_d_b32: f64 = (s.db[186][32] * ddt_scale);
        let eq125_e1521_d_b33: f64 = (s.db[186][33] * ddt_scale);
        let eq125_e1521_d_b34: f64 = (s.db[186][34] * ddt_scale);
        let eq125_e1521_d_b35: f64 = (s.db[186][35] * ddt_scale);
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1524_d_n7: f64 = p.p355;
        let eq125_e1524_d_n12: f64 = (-p.p355);
        let eq125_e1525: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 119, eq125_e1524);
        let eq125_e1525_d_n7: f64 = (eq125_e1524_d_n7 * ddt_scale);
        let eq125_e1525_d_n12: f64 = (eq125_e1524_d_n12 * ddt_scale);
        let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);
        let eq125_e1526_d_n7: f64 = (eq125_e1521_d_n7 + eq125_e1525_d_n7);
        let eq125_e1526_d_n12: f64 = (eq125_e1521_d_n12 + eq125_e1525_d_n12);
        (eq125_e1526, eq125_e1521_d_n0, eq125_e1521_d_n1, eq125_e1521_d_n2, eq125_e1521_d_n3, eq125_e1521_d_n4, eq125_e1521_d_n5, eq125_e1521_d_n6, eq125_e1526_d_n7, eq125_e1521_d_n8, eq125_e1521_d_n9, eq125_e1521_d_n10, eq125_e1521_d_n11, eq125_e1526_d_n12, eq125_e1521_d_n13, eq125_e1521_d_n14, eq125_e1521_d_n15, eq125_e1521_d_n16, eq125_e1521_d_n17, eq125_e1521_d_n18, eq125_e1521_d_n19, eq125_e1521_d_n20, eq125_e1521_d_n21, eq125_e1521_d_n22, eq125_e1521_d_n23, eq125_e1521_d_n24, eq125_e1521_d_n25, eq125_e1521_d_n26, eq125_e1521_d_n27, eq125_e1521_d_n28, eq125_e1521_d_n29, eq125_e1521_d_b0, eq125_e1521_d_b1, eq125_e1521_d_b2, eq125_e1521_d_b3, eq125_e1521_d_b4, eq125_e1521_d_b5, eq125_e1521_d_b6, eq125_e1521_d_b7, eq125_e1521_d_b8, eq125_e1521_d_b9, eq125_e1521_d_b10, eq125_e1521_d_b11, eq125_e1521_d_b12, eq125_e1521_d_b13, eq125_e1521_d_b14, eq125_e1521_d_b15, eq125_e1521_d_b16, eq125_e1521_d_b17, eq125_e1521_d_b18, eq125_e1521_d_b19, eq125_e1521_d_b20, eq125_e1521_d_b21, eq125_e1521_d_b22, eq125_e1521_d_b23, eq125_e1521_d_b24, eq125_e1521_d_b25, eq125_e1521_d_b26, eq125_e1521_d_b27, eq125_e1521_d_b28, eq125_e1521_d_b29, eq125_e1521_d_b30, eq125_e1521_d_b31, eq125_e1521_d_b32, eq125_e1521_d_b33, eq125_e1521_d_b34, eq125_e1521_d_b35,)
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
    }

    pub(super) fn stamp_transient_equations_block_17(
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29, eq126_e1538_d_b0, eq126_e1538_d_b1, eq126_e1538_d_b2, eq126_e1538_d_b3, eq126_e1538_d_b4, eq126_e1538_d_b5, eq126_e1538_d_b6, eq126_e1538_d_b7, eq126_e1538_d_b8, eq126_e1538_d_b9, eq126_e1538_d_b10, eq126_e1538_d_b11, eq126_e1538_d_b12, eq126_e1538_d_b13, eq126_e1538_d_b14, eq126_e1538_d_b15, eq126_e1538_d_b16, eq126_e1538_d_b17, eq126_e1538_d_b18, eq126_e1538_d_b19, eq126_e1538_d_b20, eq126_e1538_d_b21, eq126_e1538_d_b22, eq126_e1538_d_b23, eq126_e1538_d_b24, eq126_e1538_d_b25, eq126_e1538_d_b26, eq126_e1538_d_b27, eq126_e1538_d_b28, eq126_e1538_d_b29, eq126_e1538_d_b30, eq126_e1538_d_b31, eq126_e1538_d_b32, eq126_e1538_d_b33, eq126_e1538_d_b34, eq126_e1538_d_b35,) = {
    if s.b[1495] {
        let eq126_e1531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 120, s.v[187]);
        let eq126_e1531_d_n0: f64 = (s.dn[187][0] * ddt_scale);
        let eq126_e1531_d_n1: f64 = (s.dn[187][1] * ddt_scale);
        let eq126_e1531_d_n2: f64 = (s.dn[187][2] * ddt_scale);
        let eq126_e1531_d_n3: f64 = (s.dn[187][3] * ddt_scale);
        let eq126_e1531_d_n4: f64 = (s.dn[187][4] * ddt_scale);
        let eq126_e1531_d_n5: f64 = (s.dn[187][5] * ddt_scale);
        let eq126_e1531_d_n6: f64 = (s.dn[187][6] * ddt_scale);
        let eq126_e1531_d_n7: f64 = (s.dn[187][7] * ddt_scale);
        let eq126_e1531_d_n8: f64 = (s.dn[187][8] * ddt_scale);
        let eq126_e1531_d_n9: f64 = (s.dn[187][9] * ddt_scale);
        let eq126_e1531_d_n10: f64 = (s.dn[187][10] * ddt_scale);
        let eq126_e1531_d_n11: f64 = (s.dn[187][11] * ddt_scale);
        let eq126_e1531_d_n12: f64 = (s.dn[187][12] * ddt_scale);
        let eq126_e1531_d_n13: f64 = (s.dn[187][13] * ddt_scale);
        let eq126_e1531_d_n14: f64 = (s.dn[187][14] * ddt_scale);
        let eq126_e1531_d_n15: f64 = (s.dn[187][15] * ddt_scale);
        let eq126_e1531_d_n16: f64 = (s.dn[187][16] * ddt_scale);
        let eq126_e1531_d_n17: f64 = (s.dn[187][17] * ddt_scale);
        let eq126_e1531_d_n18: f64 = (s.dn[187][18] * ddt_scale);
        let eq126_e1531_d_n19: f64 = (s.dn[187][19] * ddt_scale);
        let eq126_e1531_d_n20: f64 = (s.dn[187][20] * ddt_scale);
        let eq126_e1531_d_n21: f64 = (s.dn[187][21] * ddt_scale);
        let eq126_e1531_d_n22: f64 = (s.dn[187][22] * ddt_scale);
        let eq126_e1531_d_n23: f64 = (s.dn[187][23] * ddt_scale);
        let eq126_e1531_d_n24: f64 = (s.dn[187][24] * ddt_scale);
        let eq126_e1531_d_n25: f64 = (s.dn[187][25] * ddt_scale);
        let eq126_e1531_d_n26: f64 = (s.dn[187][26] * ddt_scale);
        let eq126_e1531_d_n27: f64 = (s.dn[187][27] * ddt_scale);
        let eq126_e1531_d_n28: f64 = (s.dn[187][28] * ddt_scale);
        let eq126_e1531_d_n29: f64 = (s.dn[187][29] * ddt_scale);
        let eq126_e1531_d_b0: f64 = (s.db[187][0] * ddt_scale);
        let eq126_e1531_d_b1: f64 = (s.db[187][1] * ddt_scale);
        let eq126_e1531_d_b2: f64 = (s.db[187][2] * ddt_scale);
        let eq126_e1531_d_b3: f64 = (s.db[187][3] * ddt_scale);
        let eq126_e1531_d_b4: f64 = (s.db[187][4] * ddt_scale);
        let eq126_e1531_d_b5: f64 = (s.db[187][5] * ddt_scale);
        let eq126_e1531_d_b6: f64 = (s.db[187][6] * ddt_scale);
        let eq126_e1531_d_b7: f64 = (s.db[187][7] * ddt_scale);
        let eq126_e1531_d_b8: f64 = (s.db[187][8] * ddt_scale);
        let eq126_e1531_d_b9: f64 = (s.db[187][9] * ddt_scale);
        let eq126_e1531_d_b10: f64 = (s.db[187][10] * ddt_scale);
        let eq126_e1531_d_b11: f64 = (s.db[187][11] * ddt_scale);
        let eq126_e1531_d_b12: f64 = (s.db[187][12] * ddt_scale);
        let eq126_e1531_d_b13: f64 = (s.db[187][13] * ddt_scale);
        let eq126_e1531_d_b14: f64 = (s.db[187][14] * ddt_scale);
        let eq126_e1531_d_b15: f64 = (s.db[187][15] * ddt_scale);
        let eq126_e1531_d_b16: f64 = (s.db[187][16] * ddt_scale);
        let eq126_e1531_d_b17: f64 = (s.db[187][17] * ddt_scale);
        let eq126_e1531_d_b18: f64 = (s.db[187][18] * ddt_scale);
        let eq126_e1531_d_b19: f64 = (s.db[187][19] * ddt_scale);
        let eq126_e1531_d_b20: f64 = (s.db[187][20] * ddt_scale);
        let eq126_e1531_d_b21: f64 = (s.db[187][21] * ddt_scale);
        let eq126_e1531_d_b22: f64 = (s.db[187][22] * ddt_scale);
        let eq126_e1531_d_b23: f64 = (s.db[187][23] * ddt_scale);
        let eq126_e1531_d_b24: f64 = (s.db[187][24] * ddt_scale);
        let eq126_e1531_d_b25: f64 = (s.db[187][25] * ddt_scale);
        let eq126_e1531_d_b26: f64 = (s.db[187][26] * ddt_scale);
        let eq126_e1531_d_b27: f64 = (s.db[187][27] * ddt_scale);
        let eq126_e1531_d_b28: f64 = (s.db[187][28] * ddt_scale);
        let eq126_e1531_d_b29: f64 = (s.db[187][29] * ddt_scale);
        let eq126_e1531_d_b30: f64 = (s.db[187][30] * ddt_scale);
        let eq126_e1531_d_b31: f64 = (s.db[187][31] * ddt_scale);
        let eq126_e1531_d_b32: f64 = (s.db[187][32] * ddt_scale);
        let eq126_e1531_d_b33: f64 = (s.db[187][33] * ddt_scale);
        let eq126_e1531_d_b34: f64 = (s.db[187][34] * ddt_scale);
        let eq126_e1531_d_b35: f64 = (s.db[187][35] * ddt_scale);
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1534_d_n2: f64 = p.p355;
        let eq126_e1534_d_n13: f64 = (-p.p355);
        let eq126_e1535: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 121, eq126_e1534);
        let eq126_e1535_d_n2: f64 = (eq126_e1534_d_n2 * ddt_scale);
        let eq126_e1535_d_n13: f64 = (eq126_e1534_d_n13 * ddt_scale);
        let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);
        let eq126_e1536_d_n2: f64 = (eq126_e1531_d_n2 + eq126_e1535_d_n2);
        let eq126_e1536_d_n13: f64 = (eq126_e1531_d_n13 + eq126_e1535_d_n13);
        (eq126_e1536, eq126_e1531_d_n0, eq126_e1531_d_n1, eq126_e1536_d_n2, eq126_e1531_d_n3, eq126_e1531_d_n4, eq126_e1531_d_n5, eq126_e1531_d_n6, eq126_e1531_d_n7, eq126_e1531_d_n8, eq126_e1531_d_n9, eq126_e1531_d_n10, eq126_e1531_d_n11, eq126_e1531_d_n12, eq126_e1536_d_n13, eq126_e1531_d_n14, eq126_e1531_d_n15, eq126_e1531_d_n16, eq126_e1531_d_n17, eq126_e1531_d_n18, eq126_e1531_d_n19, eq126_e1531_d_n20, eq126_e1531_d_n21, eq126_e1531_d_n22, eq126_e1531_d_n23, eq126_e1531_d_n24, eq126_e1531_d_n25, eq126_e1531_d_n26, eq126_e1531_d_n27, eq126_e1531_d_n28, eq126_e1531_d_n29, eq126_e1531_d_b0, eq126_e1531_d_b1, eq126_e1531_d_b2, eq126_e1531_d_b3, eq126_e1531_d_b4, eq126_e1531_d_b5, eq126_e1531_d_b6, eq126_e1531_d_b7, eq126_e1531_d_b8, eq126_e1531_d_b9, eq126_e1531_d_b10, eq126_e1531_d_b11, eq126_e1531_d_b12, eq126_e1531_d_b13, eq126_e1531_d_b14, eq126_e1531_d_b15, eq126_e1531_d_b16, eq126_e1531_d_b17, eq126_e1531_d_b18, eq126_e1531_d_b19, eq126_e1531_d_b20, eq126_e1531_d_b21, eq126_e1531_d_b22, eq126_e1531_d_b23, eq126_e1531_d_b24, eq126_e1531_d_b25, eq126_e1531_d_b26, eq126_e1531_d_b27, eq126_e1531_d_b28, eq126_e1531_d_b29, eq126_e1531_d_b30, eq126_e1531_d_b31, eq126_e1531_d_b32, eq126_e1531_d_b33, eq126_e1531_d_b34, eq126_e1531_d_b35,)
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
        let (eq127_e1542,) = {
    if s.b[1495] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq127_value: f64 = eq127_e1542;
        stamper.stamp_current_const_local(
            Some(2),
            Some(12),
            multiplicity * (eq127_value),
        );
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29, eq128_e1552_d_b0, eq128_e1552_d_b1, eq128_e1552_d_b2, eq128_e1552_d_b3, eq128_e1552_d_b4, eq128_e1552_d_b5, eq128_e1552_d_b6, eq128_e1552_d_b7, eq128_e1552_d_b8, eq128_e1552_d_b9, eq128_e1552_d_b10, eq128_e1552_d_b11, eq128_e1552_d_b12, eq128_e1552_d_b13, eq128_e1552_d_b14, eq128_e1552_d_b15, eq128_e1552_d_b16, eq128_e1552_d_b17, eq128_e1552_d_b18, eq128_e1552_d_b19, eq128_e1552_d_b20, eq128_e1552_d_b21, eq128_e1552_d_b22, eq128_e1552_d_b23, eq128_e1552_d_b24, eq128_e1552_d_b25, eq128_e1552_d_b26, eq128_e1552_d_b27, eq128_e1552_d_b28, eq128_e1552_d_b29, eq128_e1552_d_b30, eq128_e1552_d_b31, eq128_e1552_d_b32, eq128_e1552_d_b33, eq128_e1552_d_b34, eq128_e1552_d_b35,) = {
    if s.b[1495] {
        let eq128_e1545: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 122, s.v[189]);
        let eq128_e1545_d_n0: f64 = (s.dn[189][0] * ddt_scale);
        let eq128_e1545_d_n1: f64 = (s.dn[189][1] * ddt_scale);
        let eq128_e1545_d_n2: f64 = (s.dn[189][2] * ddt_scale);
        let eq128_e1545_d_n3: f64 = (s.dn[189][3] * ddt_scale);
        let eq128_e1545_d_n4: f64 = (s.dn[189][4] * ddt_scale);
        let eq128_e1545_d_n5: f64 = (s.dn[189][5] * ddt_scale);
        let eq128_e1545_d_n6: f64 = (s.dn[189][6] * ddt_scale);
        let eq128_e1545_d_n7: f64 = (s.dn[189][7] * ddt_scale);
        let eq128_e1545_d_n8: f64 = (s.dn[189][8] * ddt_scale);
        let eq128_e1545_d_n9: f64 = (s.dn[189][9] * ddt_scale);
        let eq128_e1545_d_n10: f64 = (s.dn[189][10] * ddt_scale);
        let eq128_e1545_d_n11: f64 = (s.dn[189][11] * ddt_scale);
        let eq128_e1545_d_n12: f64 = (s.dn[189][12] * ddt_scale);
        let eq128_e1545_d_n13: f64 = (s.dn[189][13] * ddt_scale);
        let eq128_e1545_d_n14: f64 = (s.dn[189][14] * ddt_scale);
        let eq128_e1545_d_n15: f64 = (s.dn[189][15] * ddt_scale);
        let eq128_e1545_d_n16: f64 = (s.dn[189][16] * ddt_scale);
        let eq128_e1545_d_n17: f64 = (s.dn[189][17] * ddt_scale);
        let eq128_e1545_d_n18: f64 = (s.dn[189][18] * ddt_scale);
        let eq128_e1545_d_n19: f64 = (s.dn[189][19] * ddt_scale);
        let eq128_e1545_d_n20: f64 = (s.dn[189][20] * ddt_scale);
        let eq128_e1545_d_n21: f64 = (s.dn[189][21] * ddt_scale);
        let eq128_e1545_d_n22: f64 = (s.dn[189][22] * ddt_scale);
        let eq128_e1545_d_n23: f64 = (s.dn[189][23] * ddt_scale);
        let eq128_e1545_d_n24: f64 = (s.dn[189][24] * ddt_scale);
        let eq128_e1545_d_n25: f64 = (s.dn[189][25] * ddt_scale);
        let eq128_e1545_d_n26: f64 = (s.dn[189][26] * ddt_scale);
        let eq128_e1545_d_n27: f64 = (s.dn[189][27] * ddt_scale);
        let eq128_e1545_d_n28: f64 = (s.dn[189][28] * ddt_scale);
        let eq128_e1545_d_n29: f64 = (s.dn[189][29] * ddt_scale);
        let eq128_e1545_d_b0: f64 = (s.db[189][0] * ddt_scale);
        let eq128_e1545_d_b1: f64 = (s.db[189][1] * ddt_scale);
        let eq128_e1545_d_b2: f64 = (s.db[189][2] * ddt_scale);
        let eq128_e1545_d_b3: f64 = (s.db[189][3] * ddt_scale);
        let eq128_e1545_d_b4: f64 = (s.db[189][4] * ddt_scale);
        let eq128_e1545_d_b5: f64 = (s.db[189][5] * ddt_scale);
        let eq128_e1545_d_b6: f64 = (s.db[189][6] * ddt_scale);
        let eq128_e1545_d_b7: f64 = (s.db[189][7] * ddt_scale);
        let eq128_e1545_d_b8: f64 = (s.db[189][8] * ddt_scale);
        let eq128_e1545_d_b9: f64 = (s.db[189][9] * ddt_scale);
        let eq128_e1545_d_b10: f64 = (s.db[189][10] * ddt_scale);
        let eq128_e1545_d_b11: f64 = (s.db[189][11] * ddt_scale);
        let eq128_e1545_d_b12: f64 = (s.db[189][12] * ddt_scale);
        let eq128_e1545_d_b13: f64 = (s.db[189][13] * ddt_scale);
        let eq128_e1545_d_b14: f64 = (s.db[189][14] * ddt_scale);
        let eq128_e1545_d_b15: f64 = (s.db[189][15] * ddt_scale);
        let eq128_e1545_d_b16: f64 = (s.db[189][16] * ddt_scale);
        let eq128_e1545_d_b17: f64 = (s.db[189][17] * ddt_scale);
        let eq128_e1545_d_b18: f64 = (s.db[189][18] * ddt_scale);
        let eq128_e1545_d_b19: f64 = (s.db[189][19] * ddt_scale);
        let eq128_e1545_d_b20: f64 = (s.db[189][20] * ddt_scale);
        let eq128_e1545_d_b21: f64 = (s.db[189][21] * ddt_scale);
        let eq128_e1545_d_b22: f64 = (s.db[189][22] * ddt_scale);
        let eq128_e1545_d_b23: f64 = (s.db[189][23] * ddt_scale);
        let eq128_e1545_d_b24: f64 = (s.db[189][24] * ddt_scale);
        let eq128_e1545_d_b25: f64 = (s.db[189][25] * ddt_scale);
        let eq128_e1545_d_b26: f64 = (s.db[189][26] * ddt_scale);
        let eq128_e1545_d_b27: f64 = (s.db[189][27] * ddt_scale);
        let eq128_e1545_d_b28: f64 = (s.db[189][28] * ddt_scale);
        let eq128_e1545_d_b29: f64 = (s.db[189][29] * ddt_scale);
        let eq128_e1545_d_b30: f64 = (s.db[189][30] * ddt_scale);
        let eq128_e1545_d_b31: f64 = (s.db[189][31] * ddt_scale);
        let eq128_e1545_d_b32: f64 = (s.db[189][32] * ddt_scale);
        let eq128_e1545_d_b33: f64 = (s.db[189][33] * ddt_scale);
        let eq128_e1545_d_b34: f64 = (s.db[189][34] * ddt_scale);
        let eq128_e1545_d_b35: f64 = (s.db[189][35] * ddt_scale);
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1548_d_n7: f64 = p.p355;
        let eq128_e1548_d_n9: f64 = (-p.p355);
        let eq128_e1549: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 123, eq128_e1548);
        let eq128_e1549_d_n7: f64 = (eq128_e1548_d_n7 * ddt_scale);
        let eq128_e1549_d_n9: f64 = (eq128_e1548_d_n9 * ddt_scale);
        let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);
        let eq128_e1550_d_n7: f64 = (eq128_e1545_d_n7 + eq128_e1549_d_n7);
        let eq128_e1550_d_n9: f64 = (eq128_e1545_d_n9 + eq128_e1549_d_n9);
        (eq128_e1550, eq128_e1545_d_n0, eq128_e1545_d_n1, eq128_e1545_d_n2, eq128_e1545_d_n3, eq128_e1545_d_n4, eq128_e1545_d_n5, eq128_e1545_d_n6, eq128_e1550_d_n7, eq128_e1545_d_n8, eq128_e1550_d_n9, eq128_e1545_d_n10, eq128_e1545_d_n11, eq128_e1545_d_n12, eq128_e1545_d_n13, eq128_e1545_d_n14, eq128_e1545_d_n15, eq128_e1545_d_n16, eq128_e1545_d_n17, eq128_e1545_d_n18, eq128_e1545_d_n19, eq128_e1545_d_n20, eq128_e1545_d_n21, eq128_e1545_d_n22, eq128_e1545_d_n23, eq128_e1545_d_n24, eq128_e1545_d_n25, eq128_e1545_d_n26, eq128_e1545_d_n27, eq128_e1545_d_n28, eq128_e1545_d_n29, eq128_e1545_d_b0, eq128_e1545_d_b1, eq128_e1545_d_b2, eq128_e1545_d_b3, eq128_e1545_d_b4, eq128_e1545_d_b5, eq128_e1545_d_b6, eq128_e1545_d_b7, eq128_e1545_d_b8, eq128_e1545_d_b9, eq128_e1545_d_b10, eq128_e1545_d_b11, eq128_e1545_d_b12, eq128_e1545_d_b13, eq128_e1545_d_b14, eq128_e1545_d_b15, eq128_e1545_d_b16, eq128_e1545_d_b17, eq128_e1545_d_b18, eq128_e1545_d_b19, eq128_e1545_d_b20, eq128_e1545_d_b21, eq128_e1545_d_b22, eq128_e1545_d_b23, eq128_e1545_d_b24, eq128_e1545_d_b25, eq128_e1545_d_b26, eq128_e1545_d_b27, eq128_e1545_d_b28, eq128_e1545_d_b29, eq128_e1545_d_b30, eq128_e1545_d_b31, eq128_e1545_d_b32, eq128_e1545_d_b33, eq128_e1545_d_b34, eq128_e1545_d_b35,)
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
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29, eq129_e1563_d_b0, eq129_e1563_d_b1, eq129_e1563_d_b2, eq129_e1563_d_b3, eq129_e1563_d_b4, eq129_e1563_d_b5, eq129_e1563_d_b6, eq129_e1563_d_b7, eq129_e1563_d_b8, eq129_e1563_d_b9, eq129_e1563_d_b10, eq129_e1563_d_b11, eq129_e1563_d_b12, eq129_e1563_d_b13, eq129_e1563_d_b14, eq129_e1563_d_b15, eq129_e1563_d_b16, eq129_e1563_d_b17, eq129_e1563_d_b18, eq129_e1563_d_b19, eq129_e1563_d_b20, eq129_e1563_d_b21, eq129_e1563_d_b22, eq129_e1563_d_b23, eq129_e1563_d_b24, eq129_e1563_d_b25, eq129_e1563_d_b26, eq129_e1563_d_b27, eq129_e1563_d_b28, eq129_e1563_d_b29, eq129_e1563_d_b30, eq129_e1563_d_b31, eq129_e1563_d_b32, eq129_e1563_d_b33, eq129_e1563_d_b34, eq129_e1563_d_b35,) = {
    if (!s.b[1495]) {
        let eq129_e1556: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 124, s.v[185]);
        let eq129_e1556_d_n0: f64 = (s.dn[185][0] * ddt_scale);
        let eq129_e1556_d_n1: f64 = (s.dn[185][1] * ddt_scale);
        let eq129_e1556_d_n2: f64 = (s.dn[185][2] * ddt_scale);
        let eq129_e1556_d_n3: f64 = (s.dn[185][3] * ddt_scale);
        let eq129_e1556_d_n4: f64 = (s.dn[185][4] * ddt_scale);
        let eq129_e1556_d_n5: f64 = (s.dn[185][5] * ddt_scale);
        let eq129_e1556_d_n6: f64 = (s.dn[185][6] * ddt_scale);
        let eq129_e1556_d_n7: f64 = (s.dn[185][7] * ddt_scale);
        let eq129_e1556_d_n8: f64 = (s.dn[185][8] * ddt_scale);
        let eq129_e1556_d_n9: f64 = (s.dn[185][9] * ddt_scale);
        let eq129_e1556_d_n10: f64 = (s.dn[185][10] * ddt_scale);
        let eq129_e1556_d_n11: f64 = (s.dn[185][11] * ddt_scale);
        let eq129_e1556_d_n12: f64 = (s.dn[185][12] * ddt_scale);
        let eq129_e1556_d_n13: f64 = (s.dn[185][13] * ddt_scale);
        let eq129_e1556_d_n14: f64 = (s.dn[185][14] * ddt_scale);
        let eq129_e1556_d_n15: f64 = (s.dn[185][15] * ddt_scale);
        let eq129_e1556_d_n16: f64 = (s.dn[185][16] * ddt_scale);
        let eq129_e1556_d_n17: f64 = (s.dn[185][17] * ddt_scale);
        let eq129_e1556_d_n18: f64 = (s.dn[185][18] * ddt_scale);
        let eq129_e1556_d_n19: f64 = (s.dn[185][19] * ddt_scale);
        let eq129_e1556_d_n20: f64 = (s.dn[185][20] * ddt_scale);
        let eq129_e1556_d_n21: f64 = (s.dn[185][21] * ddt_scale);
        let eq129_e1556_d_n22: f64 = (s.dn[185][22] * ddt_scale);
        let eq129_e1556_d_n23: f64 = (s.dn[185][23] * ddt_scale);
        let eq129_e1556_d_n24: f64 = (s.dn[185][24] * ddt_scale);
        let eq129_e1556_d_n25: f64 = (s.dn[185][25] * ddt_scale);
        let eq129_e1556_d_n26: f64 = (s.dn[185][26] * ddt_scale);
        let eq129_e1556_d_n27: f64 = (s.dn[185][27] * ddt_scale);
        let eq129_e1556_d_n28: f64 = (s.dn[185][28] * ddt_scale);
        let eq129_e1556_d_n29: f64 = (s.dn[185][29] * ddt_scale);
        let eq129_e1556_d_b0: f64 = (s.db[185][0] * ddt_scale);
        let eq129_e1556_d_b1: f64 = (s.db[185][1] * ddt_scale);
        let eq129_e1556_d_b2: f64 = (s.db[185][2] * ddt_scale);
        let eq129_e1556_d_b3: f64 = (s.db[185][3] * ddt_scale);
        let eq129_e1556_d_b4: f64 = (s.db[185][4] * ddt_scale);
        let eq129_e1556_d_b5: f64 = (s.db[185][5] * ddt_scale);
        let eq129_e1556_d_b6: f64 = (s.db[185][6] * ddt_scale);
        let eq129_e1556_d_b7: f64 = (s.db[185][7] * ddt_scale);
        let eq129_e1556_d_b8: f64 = (s.db[185][8] * ddt_scale);
        let eq129_e1556_d_b9: f64 = (s.db[185][9] * ddt_scale);
        let eq129_e1556_d_b10: f64 = (s.db[185][10] * ddt_scale);
        let eq129_e1556_d_b11: f64 = (s.db[185][11] * ddt_scale);
        let eq129_e1556_d_b12: f64 = (s.db[185][12] * ddt_scale);
        let eq129_e1556_d_b13: f64 = (s.db[185][13] * ddt_scale);
        let eq129_e1556_d_b14: f64 = (s.db[185][14] * ddt_scale);
        let eq129_e1556_d_b15: f64 = (s.db[185][15] * ddt_scale);
        let eq129_e1556_d_b16: f64 = (s.db[185][16] * ddt_scale);
        let eq129_e1556_d_b17: f64 = (s.db[185][17] * ddt_scale);
        let eq129_e1556_d_b18: f64 = (s.db[185][18] * ddt_scale);
        let eq129_e1556_d_b19: f64 = (s.db[185][19] * ddt_scale);
        let eq129_e1556_d_b20: f64 = (s.db[185][20] * ddt_scale);
        let eq129_e1556_d_b21: f64 = (s.db[185][21] * ddt_scale);
        let eq129_e1556_d_b22: f64 = (s.db[185][22] * ddt_scale);
        let eq129_e1556_d_b23: f64 = (s.db[185][23] * ddt_scale);
        let eq129_e1556_d_b24: f64 = (s.db[185][24] * ddt_scale);
        let eq129_e1556_d_b25: f64 = (s.db[185][25] * ddt_scale);
        let eq129_e1556_d_b26: f64 = (s.db[185][26] * ddt_scale);
        let eq129_e1556_d_b27: f64 = (s.db[185][27] * ddt_scale);
        let eq129_e1556_d_b28: f64 = (s.db[185][28] * ddt_scale);
        let eq129_e1556_d_b29: f64 = (s.db[185][29] * ddt_scale);
        let eq129_e1556_d_b30: f64 = (s.db[185][30] * ddt_scale);
        let eq129_e1556_d_b31: f64 = (s.db[185][31] * ddt_scale);
        let eq129_e1556_d_b32: f64 = (s.db[185][32] * ddt_scale);
        let eq129_e1556_d_b33: f64 = (s.db[185][33] * ddt_scale);
        let eq129_e1556_d_b34: f64 = (s.db[185][34] * ddt_scale);
        let eq129_e1556_d_b35: f64 = (s.db[185][35] * ddt_scale);
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1559_d_n2: f64 = p.p355;
        let eq129_e1559_d_n13: f64 = (-p.p355);
        let eq129_e1560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 125, eq129_e1559);
        let eq129_e1560_d_n2: f64 = (eq129_e1559_d_n2 * ddt_scale);
        let eq129_e1560_d_n13: f64 = (eq129_e1559_d_n13 * ddt_scale);
        let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);
        let eq129_e1561_d_n2: f64 = (eq129_e1556_d_n2 + eq129_e1560_d_n2);
        let eq129_e1561_d_n13: f64 = (eq129_e1556_d_n13 + eq129_e1560_d_n13);
        (eq129_e1561, eq129_e1556_d_n0, eq129_e1556_d_n1, eq129_e1561_d_n2, eq129_e1556_d_n3, eq129_e1556_d_n4, eq129_e1556_d_n5, eq129_e1556_d_n6, eq129_e1556_d_n7, eq129_e1556_d_n8, eq129_e1556_d_n9, eq129_e1556_d_n10, eq129_e1556_d_n11, eq129_e1556_d_n12, eq129_e1561_d_n13, eq129_e1556_d_n14, eq129_e1556_d_n15, eq129_e1556_d_n16, eq129_e1556_d_n17, eq129_e1556_d_n18, eq129_e1556_d_n19, eq129_e1556_d_n20, eq129_e1556_d_n21, eq129_e1556_d_n22, eq129_e1556_d_n23, eq129_e1556_d_n24, eq129_e1556_d_n25, eq129_e1556_d_n26, eq129_e1556_d_n27, eq129_e1556_d_n28, eq129_e1556_d_n29, eq129_e1556_d_b0, eq129_e1556_d_b1, eq129_e1556_d_b2, eq129_e1556_d_b3, eq129_e1556_d_b4, eq129_e1556_d_b5, eq129_e1556_d_b6, eq129_e1556_d_b7, eq129_e1556_d_b8, eq129_e1556_d_b9, eq129_e1556_d_b10, eq129_e1556_d_b11, eq129_e1556_d_b12, eq129_e1556_d_b13, eq129_e1556_d_b14, eq129_e1556_d_b15, eq129_e1556_d_b16, eq129_e1556_d_b17, eq129_e1556_d_b18, eq129_e1556_d_b19, eq129_e1556_d_b20, eq129_e1556_d_b21, eq129_e1556_d_b22, eq129_e1556_d_b23, eq129_e1556_d_b24, eq129_e1556_d_b25, eq129_e1556_d_b26, eq129_e1556_d_b27, eq129_e1556_d_b28, eq129_e1556_d_b29, eq129_e1556_d_b30, eq129_e1556_d_b31, eq129_e1556_d_b32, eq129_e1556_d_b33, eq129_e1556_d_b34, eq129_e1556_d_b35,)
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
        let eq130_e1567_d_n0: f64 = (s.dn[186][0] * ddt_scale);
        let eq130_e1567_d_n1: f64 = (s.dn[186][1] * ddt_scale);
        let eq130_e1567_d_n2: f64 = (s.dn[186][2] * ddt_scale);
        let eq130_e1567_d_n3: f64 = (s.dn[186][3] * ddt_scale);
        let eq130_e1567_d_n4: f64 = (s.dn[186][4] * ddt_scale);
        let eq130_e1567_d_n5: f64 = (s.dn[186][5] * ddt_scale);
        let eq130_e1567_d_n6: f64 = (s.dn[186][6] * ddt_scale);
        let eq130_e1567_d_n7: f64 = (s.dn[186][7] * ddt_scale);
        let eq130_e1567_d_n8: f64 = (s.dn[186][8] * ddt_scale);
        let eq130_e1567_d_n9: f64 = (s.dn[186][9] * ddt_scale);
        let eq130_e1567_d_n10: f64 = (s.dn[186][10] * ddt_scale);
        let eq130_e1567_d_n11: f64 = (s.dn[186][11] * ddt_scale);
        let eq130_e1567_d_n12: f64 = (s.dn[186][12] * ddt_scale);
        let eq130_e1567_d_n13: f64 = (s.dn[186][13] * ddt_scale);
        let eq130_e1567_d_n14: f64 = (s.dn[186][14] * ddt_scale);
        let eq130_e1567_d_n15: f64 = (s.dn[186][15] * ddt_scale);
        let eq130_e1567_d_n16: f64 = (s.dn[186][16] * ddt_scale);
        let eq130_e1567_d_n17: f64 = (s.dn[186][17] * ddt_scale);
        let eq130_e1567_d_n18: f64 = (s.dn[186][18] * ddt_scale);
        let eq130_e1567_d_n19: f64 = (s.dn[186][19] * ddt_scale);
        let eq130_e1567_d_n20: f64 = (s.dn[186][20] * ddt_scale);
        let eq130_e1567_d_n21: f64 = (s.dn[186][21] * ddt_scale);
        let eq130_e1567_d_n22: f64 = (s.dn[186][22] * ddt_scale);
        let eq130_e1567_d_n23: f64 = (s.dn[186][23] * ddt_scale);
        let eq130_e1567_d_n24: f64 = (s.dn[186][24] * ddt_scale);
        let eq130_e1567_d_n25: f64 = (s.dn[186][25] * ddt_scale);
        let eq130_e1567_d_n26: f64 = (s.dn[186][26] * ddt_scale);
        let eq130_e1567_d_n27: f64 = (s.dn[186][27] * ddt_scale);
        let eq130_e1567_d_n28: f64 = (s.dn[186][28] * ddt_scale);
        let eq130_e1567_d_n29: f64 = (s.dn[186][29] * ddt_scale);
        let eq130_e1567_d_b0: f64 = (s.db[186][0] * ddt_scale);
        let eq130_e1567_d_b1: f64 = (s.db[186][1] * ddt_scale);
        let eq130_e1567_d_b2: f64 = (s.db[186][2] * ddt_scale);
        let eq130_e1567_d_b3: f64 = (s.db[186][3] * ddt_scale);
        let eq130_e1567_d_b4: f64 = (s.db[186][4] * ddt_scale);
        let eq130_e1567_d_b5: f64 = (s.db[186][5] * ddt_scale);
        let eq130_e1567_d_b6: f64 = (s.db[186][6] * ddt_scale);
        let eq130_e1567_d_b7: f64 = (s.db[186][7] * ddt_scale);
        let eq130_e1567_d_b8: f64 = (s.db[186][8] * ddt_scale);
        let eq130_e1567_d_b9: f64 = (s.db[186][9] * ddt_scale);
        let eq130_e1567_d_b10: f64 = (s.db[186][10] * ddt_scale);
        let eq130_e1567_d_b11: f64 = (s.db[186][11] * ddt_scale);
        let eq130_e1567_d_b12: f64 = (s.db[186][12] * ddt_scale);
        let eq130_e1567_d_b13: f64 = (s.db[186][13] * ddt_scale);
        let eq130_e1567_d_b14: f64 = (s.db[186][14] * ddt_scale);
        let eq130_e1567_d_b15: f64 = (s.db[186][15] * ddt_scale);
        let eq130_e1567_d_b16: f64 = (s.db[186][16] * ddt_scale);
        let eq130_e1567_d_b17: f64 = (s.db[186][17] * ddt_scale);
        let eq130_e1567_d_b18: f64 = (s.db[186][18] * ddt_scale);
        let eq130_e1567_d_b19: f64 = (s.db[186][19] * ddt_scale);
        let eq130_e1567_d_b20: f64 = (s.db[186][20] * ddt_scale);
        let eq130_e1567_d_b21: f64 = (s.db[186][21] * ddt_scale);
        let eq130_e1567_d_b22: f64 = (s.db[186][22] * ddt_scale);
        let eq130_e1567_d_b23: f64 = (s.db[186][23] * ddt_scale);
        let eq130_e1567_d_b24: f64 = (s.db[186][24] * ddt_scale);
        let eq130_e1567_d_b25: f64 = (s.db[186][25] * ddt_scale);
        let eq130_e1567_d_b26: f64 = (s.db[186][26] * ddt_scale);
        let eq130_e1567_d_b27: f64 = (s.db[186][27] * ddt_scale);
        let eq130_e1567_d_b28: f64 = (s.db[186][28] * ddt_scale);
        let eq130_e1567_d_b29: f64 = (s.db[186][29] * ddt_scale);
        let eq130_e1567_d_b30: f64 = (s.db[186][30] * ddt_scale);
        let eq130_e1567_d_b31: f64 = (s.db[186][31] * ddt_scale);
        let eq130_e1567_d_b32: f64 = (s.db[186][32] * ddt_scale);
        let eq130_e1567_d_b33: f64 = (s.db[186][33] * ddt_scale);
        let eq130_e1567_d_b34: f64 = (s.db[186][34] * ddt_scale);
        let eq130_e1567_d_b35: f64 = (s.db[186][35] * ddt_scale);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1570_d_n2: f64 = p.p355;
        let eq130_e1570_d_n12: f64 = (-p.p355);
        let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 127, eq130_e1570);
        let eq130_e1571_d_n2: f64 = (eq130_e1570_d_n2 * ddt_scale);
        let eq130_e1571_d_n12: f64 = (eq130_e1570_d_n12 * ddt_scale);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n2: f64 = (eq130_e1567_d_n2 + eq130_e1571_d_n2);
        let eq130_e1572_d_n12: f64 = (eq130_e1567_d_n12 + eq130_e1571_d_n12);
        (eq130_e1572, eq130_e1567_d_n0, eq130_e1567_d_n1, eq130_e1572_d_n2, eq130_e1567_d_n3, eq130_e1567_d_n4, eq130_e1567_d_n5, eq130_e1567_d_n6, eq130_e1567_d_n7, eq130_e1567_d_n8, eq130_e1567_d_n9, eq130_e1567_d_n10, eq130_e1567_d_n11, eq130_e1572_d_n12, eq130_e1567_d_n13, eq130_e1567_d_n14, eq130_e1567_d_n15, eq130_e1567_d_n16, eq130_e1567_d_n17, eq130_e1567_d_n18, eq130_e1567_d_n19, eq130_e1567_d_n20, eq130_e1567_d_n21, eq130_e1567_d_n22, eq130_e1567_d_n23, eq130_e1567_d_n24, eq130_e1567_d_n25, eq130_e1567_d_n26, eq130_e1567_d_n27, eq130_e1567_d_n28, eq130_e1567_d_n29, eq130_e1567_d_b0, eq130_e1567_d_b1, eq130_e1567_d_b2, eq130_e1567_d_b3, eq130_e1567_d_b4, eq130_e1567_d_b5, eq130_e1567_d_b6, eq130_e1567_d_b7, eq130_e1567_d_b8, eq130_e1567_d_b9, eq130_e1567_d_b10, eq130_e1567_d_b11, eq130_e1567_d_b12, eq130_e1567_d_b13, eq130_e1567_d_b14, eq130_e1567_d_b15, eq130_e1567_d_b16, eq130_e1567_d_b17, eq130_e1567_d_b18, eq130_e1567_d_b19, eq130_e1567_d_b20, eq130_e1567_d_b21, eq130_e1567_d_b22, eq130_e1567_d_b23, eq130_e1567_d_b24, eq130_e1567_d_b25, eq130_e1567_d_b26, eq130_e1567_d_b27, eq130_e1567_d_b28, eq130_e1567_d_b29, eq130_e1567_d_b30, eq130_e1567_d_b31, eq130_e1567_d_b32, eq130_e1567_d_b33, eq130_e1567_d_b34, eq130_e1567_d_b35,)
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
        let eq131_e1578_d_n0: f64 = (s.dn[187][0] * ddt_scale);
        let eq131_e1578_d_n1: f64 = (s.dn[187][1] * ddt_scale);
        let eq131_e1578_d_n2: f64 = (s.dn[187][2] * ddt_scale);
        let eq131_e1578_d_n3: f64 = (s.dn[187][3] * ddt_scale);
        let eq131_e1578_d_n4: f64 = (s.dn[187][4] * ddt_scale);
        let eq131_e1578_d_n5: f64 = (s.dn[187][5] * ddt_scale);
        let eq131_e1578_d_n6: f64 = (s.dn[187][6] * ddt_scale);
        let eq131_e1578_d_n7: f64 = (s.dn[187][7] * ddt_scale);
        let eq131_e1578_d_n8: f64 = (s.dn[187][8] * ddt_scale);
        let eq131_e1578_d_n9: f64 = (s.dn[187][9] * ddt_scale);
        let eq131_e1578_d_n10: f64 = (s.dn[187][10] * ddt_scale);
        let eq131_e1578_d_n11: f64 = (s.dn[187][11] * ddt_scale);
        let eq131_e1578_d_n12: f64 = (s.dn[187][12] * ddt_scale);
        let eq131_e1578_d_n13: f64 = (s.dn[187][13] * ddt_scale);
        let eq131_e1578_d_n14: f64 = (s.dn[187][14] * ddt_scale);
        let eq131_e1578_d_n15: f64 = (s.dn[187][15] * ddt_scale);
        let eq131_e1578_d_n16: f64 = (s.dn[187][16] * ddt_scale);
        let eq131_e1578_d_n17: f64 = (s.dn[187][17] * ddt_scale);
        let eq131_e1578_d_n18: f64 = (s.dn[187][18] * ddt_scale);
        let eq131_e1578_d_n19: f64 = (s.dn[187][19] * ddt_scale);
        let eq131_e1578_d_n20: f64 = (s.dn[187][20] * ddt_scale);
        let eq131_e1578_d_n21: f64 = (s.dn[187][21] * ddt_scale);
        let eq131_e1578_d_n22: f64 = (s.dn[187][22] * ddt_scale);
        let eq131_e1578_d_n23: f64 = (s.dn[187][23] * ddt_scale);
        let eq131_e1578_d_n24: f64 = (s.dn[187][24] * ddt_scale);
        let eq131_e1578_d_n25: f64 = (s.dn[187][25] * ddt_scale);
        let eq131_e1578_d_n26: f64 = (s.dn[187][26] * ddt_scale);
        let eq131_e1578_d_n27: f64 = (s.dn[187][27] * ddt_scale);
        let eq131_e1578_d_n28: f64 = (s.dn[187][28] * ddt_scale);
        let eq131_e1578_d_n29: f64 = (s.dn[187][29] * ddt_scale);
        let eq131_e1578_d_b0: f64 = (s.db[187][0] * ddt_scale);
        let eq131_e1578_d_b1: f64 = (s.db[187][1] * ddt_scale);
        let eq131_e1578_d_b2: f64 = (s.db[187][2] * ddt_scale);
        let eq131_e1578_d_b3: f64 = (s.db[187][3] * ddt_scale);
        let eq131_e1578_d_b4: f64 = (s.db[187][4] * ddt_scale);
        let eq131_e1578_d_b5: f64 = (s.db[187][5] * ddt_scale);
        let eq131_e1578_d_b6: f64 = (s.db[187][6] * ddt_scale);
        let eq131_e1578_d_b7: f64 = (s.db[187][7] * ddt_scale);
        let eq131_e1578_d_b8: f64 = (s.db[187][8] * ddt_scale);
        let eq131_e1578_d_b9: f64 = (s.db[187][9] * ddt_scale);
        let eq131_e1578_d_b10: f64 = (s.db[187][10] * ddt_scale);
        let eq131_e1578_d_b11: f64 = (s.db[187][11] * ddt_scale);
        let eq131_e1578_d_b12: f64 = (s.db[187][12] * ddt_scale);
        let eq131_e1578_d_b13: f64 = (s.db[187][13] * ddt_scale);
        let eq131_e1578_d_b14: f64 = (s.db[187][14] * ddt_scale);
        let eq131_e1578_d_b15: f64 = (s.db[187][15] * ddt_scale);
        let eq131_e1578_d_b16: f64 = (s.db[187][16] * ddt_scale);
        let eq131_e1578_d_b17: f64 = (s.db[187][17] * ddt_scale);
        let eq131_e1578_d_b18: f64 = (s.db[187][18] * ddt_scale);
        let eq131_e1578_d_b19: f64 = (s.db[187][19] * ddt_scale);
        let eq131_e1578_d_b20: f64 = (s.db[187][20] * ddt_scale);
        let eq131_e1578_d_b21: f64 = (s.db[187][21] * ddt_scale);
        let eq131_e1578_d_b22: f64 = (s.db[187][22] * ddt_scale);
        let eq131_e1578_d_b23: f64 = (s.db[187][23] * ddt_scale);
        let eq131_e1578_d_b24: f64 = (s.db[187][24] * ddt_scale);
        let eq131_e1578_d_b25: f64 = (s.db[187][25] * ddt_scale);
        let eq131_e1578_d_b26: f64 = (s.db[187][26] * ddt_scale);
        let eq131_e1578_d_b27: f64 = (s.db[187][27] * ddt_scale);
        let eq131_e1578_d_b28: f64 = (s.db[187][28] * ddt_scale);
        let eq131_e1578_d_b29: f64 = (s.db[187][29] * ddt_scale);
        let eq131_e1578_d_b30: f64 = (s.db[187][30] * ddt_scale);
        let eq131_e1578_d_b31: f64 = (s.db[187][31] * ddt_scale);
        let eq131_e1578_d_b32: f64 = (s.db[187][32] * ddt_scale);
        let eq131_e1578_d_b33: f64 = (s.db[187][33] * ddt_scale);
        let eq131_e1578_d_b34: f64 = (s.db[187][34] * ddt_scale);
        let eq131_e1578_d_b35: f64 = (s.db[187][35] * ddt_scale);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1581_d_n7: f64 = p.p355;
        let eq131_e1581_d_n13: f64 = (-p.p355);
        let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 129, eq131_e1581);
        let eq131_e1582_d_n7: f64 = (eq131_e1581_d_n7 * ddt_scale);
        let eq131_e1582_d_n13: f64 = (eq131_e1581_d_n13 * ddt_scale);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n7: f64 = (eq131_e1578_d_n7 + eq131_e1582_d_n7);
        let eq131_e1583_d_n13: f64 = (eq131_e1578_d_n13 + eq131_e1582_d_n13);
        (eq131_e1583, eq131_e1578_d_n0, eq131_e1578_d_n1, eq131_e1578_d_n2, eq131_e1578_d_n3, eq131_e1578_d_n4, eq131_e1578_d_n5, eq131_e1578_d_n6, eq131_e1583_d_n7, eq131_e1578_d_n8, eq131_e1578_d_n9, eq131_e1578_d_n10, eq131_e1578_d_n11, eq131_e1578_d_n12, eq131_e1583_d_n13, eq131_e1578_d_n14, eq131_e1578_d_n15, eq131_e1578_d_n16, eq131_e1578_d_n17, eq131_e1578_d_n18, eq131_e1578_d_n19, eq131_e1578_d_n20, eq131_e1578_d_n21, eq131_e1578_d_n22, eq131_e1578_d_n23, eq131_e1578_d_n24, eq131_e1578_d_n25, eq131_e1578_d_n26, eq131_e1578_d_n27, eq131_e1578_d_n28, eq131_e1578_d_n29, eq131_e1578_d_b0, eq131_e1578_d_b1, eq131_e1578_d_b2, eq131_e1578_d_b3, eq131_e1578_d_b4, eq131_e1578_d_b5, eq131_e1578_d_b6, eq131_e1578_d_b7, eq131_e1578_d_b8, eq131_e1578_d_b9, eq131_e1578_d_b10, eq131_e1578_d_b11, eq131_e1578_d_b12, eq131_e1578_d_b13, eq131_e1578_d_b14, eq131_e1578_d_b15, eq131_e1578_d_b16, eq131_e1578_d_b17, eq131_e1578_d_b18, eq131_e1578_d_b19, eq131_e1578_d_b20, eq131_e1578_d_b21, eq131_e1578_d_b22, eq131_e1578_d_b23, eq131_e1578_d_b24, eq131_e1578_d_b25, eq131_e1578_d_b26, eq131_e1578_d_b27, eq131_e1578_d_b28, eq131_e1578_d_b29, eq131_e1578_d_b30, eq131_e1578_d_b31, eq131_e1578_d_b32, eq131_e1578_d_b33, eq131_e1578_d_b34, eq131_e1578_d_b35,)
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
        let (eq132_e1590,) = {
    if (!s.b[1495]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq132_value: f64 = eq132_e1590;
        stamper.stamp_current_const_local(
            Some(7),
            Some(12),
            multiplicity * (eq132_value),
        );
        let (eq133_e1595,) = {
    if (!s.b[1495]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq133_value: f64 = eq133_e1595;
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (eq133_value),
        );
    }
}
