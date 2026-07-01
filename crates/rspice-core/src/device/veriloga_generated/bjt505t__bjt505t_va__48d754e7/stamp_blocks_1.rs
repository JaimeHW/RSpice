#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign6820_e7121, assign6820_e7121_d_n0, assign6820_e7121_d_n1, assign6820_e7121_d_n3, assign6820_e7121_d_n4, assign6820_e7121_d_n5, assign6820_e7121_d_n6, assign6820_e7121_d_n7, assign6820_e7121_d_n8, assign6820_e7121_d_n9, assign6820_e7121_d_n10, assign6820_e7121_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6820_e7113: f64 = (0.2 * locals.var_vb1b2);
        let assign6820_e7116: f64 = (locals.var_dqtevb2e1 + locals.var_dqbevb2e1);
        let assign6820_e7118: f64 = (assign6820_e7116 + locals.var_dqevb2e1);
        let assign6820_e7119: f64 = (assign6820_e7113 * assign6820_e7118);
        (assign6820_e7119, (assign6820_e7113 * ((locals.var_dqtevb2e1_dn0 + locals.var_dqbevb2e1_dn0) + locals.var_dqevb2e1_dn0)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn1 + locals.var_dqbevb2e1_dn1) + locals.var_dqevb2e1_dn1)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn3 + locals.var_dqbevb2e1_dn3) + locals.var_dqevb2e1_dn3)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn4 + locals.var_dqbevb2e1_dn4) + locals.var_dqevb2e1_dn4)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn5 + locals.var_dqbevb2e1_dn5) + locals.var_dqevb2e1_dn5)), (((0.2 * locals.var_vb1b2_dn6) * assign6820_e7118) + (assign6820_e7113 * ((locals.var_dqtevb2e1_dn6 + locals.var_dqbevb2e1_dn6) + locals.var_dqevb2e1_dn6))), (((0.2 * locals.var_vb1b2_dn7) * assign6820_e7118) + (assign6820_e7113 * ((locals.var_dqtevb2e1_dn7 + locals.var_dqbevb2e1_dn7) + locals.var_dqevb2e1_dn7))), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn8 + locals.var_dqbevb2e1_dn8) + locals.var_dqevb2e1_dn8)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn9 + locals.var_dqbevb2e1_dn9) + locals.var_dqevb2e1_dn9)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn10 + locals.var_dqbevb2e1_dn10) + locals.var_dqevb2e1_dn10)), (assign6820_e7113 * ((locals.var_dqtevb2e1_dn11 + locals.var_dqbevb2e1_dn11) + locals.var_dqevb2e1_dn11)),)
    } else {
        (locals.var_qb1b2, locals.var_qb1b2_dn0, locals.var_qb1b2_dn1, locals.var_qb1b2_dn3, locals.var_qb1b2_dn4, locals.var_qb1b2_dn5, locals.var_qb1b2_dn6, locals.var_qb1b2_dn7, locals.var_qb1b2_dn8, locals.var_qb1b2_dn9, locals.var_qb1b2_dn10, locals.var_qb1b2_dn11,)
    }
};
        locals.var_qb1b2 = assign6820_e7121;
        locals.var_qb1b2_dn0 = assign6820_e7121_d_n0;
        locals.var_qb1b2_dn1 = assign6820_e7121_d_n1;
        locals.var_qb1b2_dn3 = assign6820_e7121_d_n3;
        locals.var_qb1b2_dn4 = assign6820_e7121_d_n4;
        locals.var_qb1b2_dn5 = assign6820_e7121_d_n5;
        locals.var_qb1b2_dn6 = assign6820_e7121_d_n6;
        locals.var_qb1b2_dn7 = assign6820_e7121_d_n7;
        locals.var_qb1b2_dn8 = assign6820_e7121_d_n8;
        locals.var_qb1b2_dn9 = assign6820_e7121_d_n9;
        locals.var_qb1b2_dn10 = assign6820_e7121_d_n10;
        locals.var_qb1b2_dn11 = assign6820_e7121_d_n11;

        let (assign6830_e7129, assign6830_e7129_d_n0, assign6830_e7129_d_n1, assign6830_e7129_d_n3, assign6830_e7129_d_n4, assign6830_e7129_d_n5, assign6830_e7129_d_n6, assign6830_e7129_d_n7, assign6830_e7129_d_n8, assign6830_e7129_d_n9, assign6830_e7129_d_n10, assign6830_e7129_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6830_e7125: f64 = (1.0 - p.p95);
        let assign6830_e7127: f64 = (assign6830_e7125 * locals.var_qe_qs);
        (assign6830_e7127, (assign6830_e7125 * locals.var_qe_qs_dn0), (assign6830_e7125 * locals.var_qe_qs_dn1), (assign6830_e7125 * locals.var_qe_qs_dn3), (assign6830_e7125 * locals.var_qe_qs_dn4), (assign6830_e7125 * locals.var_qe_qs_dn5), (assign6830_e7125 * locals.var_qe_qs_dn6), (assign6830_e7125 * locals.var_qe_qs_dn7), (assign6830_e7125 * locals.var_qe_qs_dn8), (assign6830_e7125 * locals.var_qe_qs_dn9), (assign6830_e7125 * locals.var_qe_qs_dn10), (assign6830_e7125 * locals.var_qe_qs_dn11),)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10, locals.var_qe_dn11,)
    }
};
        locals.var_qe = assign6830_e7129;
        locals.var_qe_dn0 = assign6830_e7129_d_n0;
        locals.var_qe_dn1 = assign6830_e7129_d_n1;
        locals.var_qe_dn3 = assign6830_e7129_d_n3;
        locals.var_qe_dn4 = assign6830_e7129_d_n4;
        locals.var_qe_dn5 = assign6830_e7129_d_n5;
        locals.var_qe_dn6 = assign6830_e7129_d_n6;
        locals.var_qe_dn7 = assign6830_e7129_d_n7;
        locals.var_qe_dn8 = assign6830_e7129_d_n8;
        locals.var_qe_dn9 = assign6830_e7129_d_n9;
        locals.var_qe_dn10 = assign6830_e7129_d_n10;
        locals.var_qe_dn11 = assign6830_e7129_d_n11;

        let (assign6840_e7137, assign6840_e7137_d_n0, assign6840_e7137_d_n1, assign6840_e7137_d_n3, assign6840_e7137_d_n4, assign6840_e7137_d_n5, assign6840_e7137_d_n6, assign6840_e7137_d_n7, assign6840_e7137_d_n8, assign6840_e7137_d_n9, assign6840_e7137_d_n10, assign6840_e7137_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6840_e7134: f64 = (p.p95 * locals.var_qe_qs);
        let assign6840_e7135: f64 = (locals.var_qbe_qs + assign6840_e7134);
        (assign6840_e7135, (locals.var_qbe_qs_dn0 + (p.p95 * locals.var_qe_qs_dn0)), (locals.var_qbe_qs_dn1 + (p.p95 * locals.var_qe_qs_dn1)), (locals.var_qbe_qs_dn3 + (p.p95 * locals.var_qe_qs_dn3)), (locals.var_qbe_qs_dn4 + (p.p95 * locals.var_qe_qs_dn4)), (locals.var_qbe_qs_dn5 + (p.p95 * locals.var_qe_qs_dn5)), (locals.var_qbe_qs_dn6 + (p.p95 * locals.var_qe_qs_dn6)), (locals.var_qbe_qs_dn7 + (p.p95 * locals.var_qe_qs_dn7)), (locals.var_qbe_qs_dn8 + (p.p95 * locals.var_qe_qs_dn8)), (locals.var_qbe_qs_dn9 + (p.p95 * locals.var_qe_qs_dn9)), (locals.var_qbe_qs_dn10 + (p.p95 * locals.var_qe_qs_dn10)), (locals.var_qbe_qs_dn11 + (p.p95 * locals.var_qe_qs_dn11)),)
    } else {
        (locals.var_qbe_qs_eff, locals.var_qbe_qs_eff_dn0, locals.var_qbe_qs_eff_dn1, locals.var_qbe_qs_eff_dn3, locals.var_qbe_qs_eff_dn4, locals.var_qbe_qs_eff_dn5, locals.var_qbe_qs_eff_dn6, locals.var_qbe_qs_eff_dn7, locals.var_qbe_qs_eff_dn8, locals.var_qbe_qs_eff_dn9, locals.var_qbe_qs_eff_dn10, locals.var_qbe_qs_eff_dn11,)
    }
};
        locals.var_qbe_qs_eff = assign6840_e7137;
        locals.var_qbe_qs_eff_dn0 = assign6840_e7137_d_n0;
        locals.var_qbe_qs_eff_dn1 = assign6840_e7137_d_n1;
        locals.var_qbe_qs_eff_dn3 = assign6840_e7137_d_n3;
        locals.var_qbe_qs_eff_dn4 = assign6840_e7137_d_n4;
        locals.var_qbe_qs_eff_dn5 = assign6840_e7137_d_n5;
        locals.var_qbe_qs_eff_dn6 = assign6840_e7137_d_n6;
        locals.var_qbe_qs_eff_dn7 = assign6840_e7137_d_n7;
        locals.var_qbe_qs_eff_dn8 = assign6840_e7137_d_n8;
        locals.var_qbe_qs_eff_dn9 = assign6840_e7137_d_n9;
        locals.var_qbe_qs_eff_dn10 = assign6840_e7137_d_n10;
        locals.var_qbe_qs_eff_dn11 = assign6840_e7137_d_n11;

        let (assign6850_e7145, assign6850_e7145_d_n0, assign6850_e7145_d_n1, assign6850_e7145_d_n3, assign6850_e7145_d_n4, assign6850_e7145_d_n5, assign6850_e7145_d_n6, assign6850_e7145_d_n7, assign6850_e7145_d_n8, assign6850_e7145_d_n9, assign6850_e7145_d_n10, assign6850_e7145_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6850_e7141: f64 = (p.p94 * locals.var_qbe_qs_eff);
        let assign6850_e7143: f64 = (assign6850_e7141 + locals.var_qbc_qs);
        (assign6850_e7143, ((p.p94 * locals.var_qbe_qs_eff_dn0) + locals.var_qbc_qs_dn0), ((p.p94 * locals.var_qbe_qs_eff_dn1) + locals.var_qbc_qs_dn1), ((p.p94 * locals.var_qbe_qs_eff_dn3) + locals.var_qbc_qs_dn3), ((p.p94 * locals.var_qbe_qs_eff_dn4) + locals.var_qbc_qs_dn4), ((p.p94 * locals.var_qbe_qs_eff_dn5) + locals.var_qbc_qs_dn5), ((p.p94 * locals.var_qbe_qs_eff_dn6) + locals.var_qbc_qs_dn6), ((p.p94 * locals.var_qbe_qs_eff_dn7) + locals.var_qbc_qs_dn7), ((p.p94 * locals.var_qbe_qs_eff_dn8) + locals.var_qbc_qs_dn8), ((p.p94 * locals.var_qbe_qs_eff_dn9) + locals.var_qbc_qs_dn9), ((p.p94 * locals.var_qbe_qs_eff_dn10) + locals.var_qbc_qs_dn10), ((p.p94 * locals.var_qbe_qs_eff_dn11) + locals.var_qbc_qs_dn11),)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10, locals.var_qbc_dn11,)
    }
};
        locals.var_qbc = assign6850_e7145;
        locals.var_qbc_dn0 = assign6850_e7145_d_n0;
        locals.var_qbc_dn1 = assign6850_e7145_d_n1;
        locals.var_qbc_dn3 = assign6850_e7145_d_n3;
        locals.var_qbc_dn4 = assign6850_e7145_d_n4;
        locals.var_qbc_dn5 = assign6850_e7145_d_n5;
        locals.var_qbc_dn6 = assign6850_e7145_d_n6;
        locals.var_qbc_dn7 = assign6850_e7145_d_n7;
        locals.var_qbc_dn8 = assign6850_e7145_d_n8;
        locals.var_qbc_dn9 = assign6850_e7145_d_n9;
        locals.var_qbc_dn10 = assign6850_e7145_d_n10;
        locals.var_qbc_dn11 = assign6850_e7145_d_n11;

        let (assign6860_e7153, assign6860_e7153_d_n0, assign6860_e7153_d_n1, assign6860_e7153_d_n3, assign6860_e7153_d_n4, assign6860_e7153_d_n5, assign6860_e7153_d_n6, assign6860_e7153_d_n7, assign6860_e7153_d_n8, assign6860_e7153_d_n9, assign6860_e7153_d_n10, assign6860_e7153_d_n11,) = {
    if (locals.var_guard123 != 0.0) {
        let assign6860_e7149: f64 = (1.0 - p.p94);
        let assign6860_e7151: f64 = (assign6860_e7149 * locals.var_qbe_qs_eff);
        (assign6860_e7151, (assign6860_e7149 * locals.var_qbe_qs_eff_dn0), (assign6860_e7149 * locals.var_qbe_qs_eff_dn1), (assign6860_e7149 * locals.var_qbe_qs_eff_dn3), (assign6860_e7149 * locals.var_qbe_qs_eff_dn4), (assign6860_e7149 * locals.var_qbe_qs_eff_dn5), (assign6860_e7149 * locals.var_qbe_qs_eff_dn6), (assign6860_e7149 * locals.var_qbe_qs_eff_dn7), (assign6860_e7149 * locals.var_qbe_qs_eff_dn8), (assign6860_e7149 * locals.var_qbe_qs_eff_dn9), (assign6860_e7149 * locals.var_qbe_qs_eff_dn10), (assign6860_e7149 * locals.var_qbe_qs_eff_dn11),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10, locals.var_qbe_dn11,)
    }
};
        locals.var_qbe = assign6860_e7153;
        locals.var_qbe_dn0 = assign6860_e7153_d_n0;
        locals.var_qbe_dn1 = assign6860_e7153_d_n1;
        locals.var_qbe_dn3 = assign6860_e7153_d_n3;
        locals.var_qbe_dn4 = assign6860_e7153_d_n4;
        locals.var_qbe_dn5 = assign6860_e7153_d_n5;
        locals.var_qbe_dn6 = assign6860_e7153_d_n6;
        locals.var_qbe_dn7 = assign6860_e7153_d_n7;
        locals.var_qbe_dn8 = assign6860_e7153_d_n8;
        locals.var_qbe_dn9 = assign6860_e7153_d_n9;
        locals.var_qbe_dn10 = assign6860_e7153_d_n10;
        locals.var_qbe_dn11 = assign6860_e7153_d_n11;

        let (assign6870_e7158, assign6870_e7158_d_n0, assign6870_e7158_d_n1, assign6870_e7158_d_n3, assign6870_e7158_d_n4, assign6870_e7158_d_n5, assign6870_e7158_d_n6, assign6870_e7158_d_n7, assign6870_e7158_d_n8, assign6870_e7158_d_n9, assign6870_e7158_d_n10, assign6870_e7158_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qbe_qs, locals.var_qbe_qs_dn0, locals.var_qbe_qs_dn1, locals.var_qbe_qs_dn3, locals.var_qbe_qs_dn4, locals.var_qbe_qs_dn5, locals.var_qbe_qs_dn6, locals.var_qbe_qs_dn7, locals.var_qbe_qs_dn8, locals.var_qbe_qs_dn9, locals.var_qbe_qs_dn10, locals.var_qbe_qs_dn11,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn1, locals.var_qbe_dn3, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn8, locals.var_qbe_dn9, locals.var_qbe_dn10, locals.var_qbe_dn11,)
    }
};
        locals.var_qbe = assign6870_e7158;
        locals.var_qbe_dn0 = assign6870_e7158_d_n0;
        locals.var_qbe_dn1 = assign6870_e7158_d_n1;
        locals.var_qbe_dn3 = assign6870_e7158_d_n3;
        locals.var_qbe_dn4 = assign6870_e7158_d_n4;
        locals.var_qbe_dn5 = assign6870_e7158_d_n5;
        locals.var_qbe_dn6 = assign6870_e7158_d_n6;
        locals.var_qbe_dn7 = assign6870_e7158_d_n7;
        locals.var_qbe_dn8 = assign6870_e7158_d_n8;
        locals.var_qbe_dn9 = assign6870_e7158_d_n9;
        locals.var_qbe_dn10 = assign6870_e7158_d_n10;
        locals.var_qbe_dn11 = assign6870_e7158_d_n11;

        let (assign6880_e7163, assign6880_e7163_d_n0, assign6880_e7163_d_n1, assign6880_e7163_d_n3, assign6880_e7163_d_n4, assign6880_e7163_d_n5, assign6880_e7163_d_n6, assign6880_e7163_d_n7, assign6880_e7163_d_n8, assign6880_e7163_d_n9, assign6880_e7163_d_n10, assign6880_e7163_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qbc_qs, locals.var_qbc_qs_dn0, locals.var_qbc_qs_dn1, locals.var_qbc_qs_dn3, locals.var_qbc_qs_dn4, locals.var_qbc_qs_dn5, locals.var_qbc_qs_dn6, locals.var_qbc_qs_dn7, locals.var_qbc_qs_dn8, locals.var_qbc_qs_dn9, locals.var_qbc_qs_dn10, locals.var_qbc_qs_dn11,)
    } else {
        (locals.var_qbc, locals.var_qbc_dn0, locals.var_qbc_dn1, locals.var_qbc_dn3, locals.var_qbc_dn4, locals.var_qbc_dn5, locals.var_qbc_dn6, locals.var_qbc_dn7, locals.var_qbc_dn8, locals.var_qbc_dn9, locals.var_qbc_dn10, locals.var_qbc_dn11,)
    }
};
        locals.var_qbc = assign6880_e7163;
        locals.var_qbc_dn0 = assign6880_e7163_d_n0;
        locals.var_qbc_dn1 = assign6880_e7163_d_n1;
        locals.var_qbc_dn3 = assign6880_e7163_d_n3;
        locals.var_qbc_dn4 = assign6880_e7163_d_n4;
        locals.var_qbc_dn5 = assign6880_e7163_d_n5;
        locals.var_qbc_dn6 = assign6880_e7163_d_n6;
        locals.var_qbc_dn7 = assign6880_e7163_d_n7;
        locals.var_qbc_dn8 = assign6880_e7163_d_n8;
        locals.var_qbc_dn9 = assign6880_e7163_d_n9;
        locals.var_qbc_dn10 = assign6880_e7163_d_n10;
        locals.var_qbc_dn11 = assign6880_e7163_d_n11;

        let (assign6890_e7168, assign6890_e7168_d_n0, assign6890_e7168_d_n1, assign6890_e7168_d_n3, assign6890_e7168_d_n4, assign6890_e7168_d_n5, assign6890_e7168_d_n6, assign6890_e7168_d_n7, assign6890_e7168_d_n8, assign6890_e7168_d_n9, assign6890_e7168_d_n10, assign6890_e7168_d_n11,) = {
    if (locals.var_guard123 == 0.0) {
        (locals.var_qe_qs, locals.var_qe_qs_dn0, locals.var_qe_qs_dn1, locals.var_qe_qs_dn3, locals.var_qe_qs_dn4, locals.var_qe_qs_dn5, locals.var_qe_qs_dn6, locals.var_qe_qs_dn7, locals.var_qe_qs_dn8, locals.var_qe_qs_dn9, locals.var_qe_qs_dn10, locals.var_qe_qs_dn11,)
    } else {
        (locals.var_qe, locals.var_qe_dn0, locals.var_qe_dn1, locals.var_qe_dn3, locals.var_qe_dn4, locals.var_qe_dn5, locals.var_qe_dn6, locals.var_qe_dn7, locals.var_qe_dn8, locals.var_qe_dn9, locals.var_qe_dn10, locals.var_qe_dn11,)
    }
};
        locals.var_qe = assign6890_e7168;
        locals.var_qe_dn0 = assign6890_e7168_d_n0;
        locals.var_qe_dn1 = assign6890_e7168_d_n1;
        locals.var_qe_dn3 = assign6890_e7168_d_n3;
        locals.var_qe_dn4 = assign6890_e7168_d_n4;
        locals.var_qe_dn5 = assign6890_e7168_d_n5;
        locals.var_qe_dn6 = assign6890_e7168_d_n6;
        locals.var_qe_dn7 = assign6890_e7168_d_n7;
        locals.var_qe_dn8 = assign6890_e7168_d_n8;
        locals.var_qe_dn9 = assign6890_e7168_d_n9;
        locals.var_qe_dn10 = assign6890_e7168_d_n10;
        locals.var_qe_dn11 = assign6890_e7168_d_n11;

        let assign6910_e7174: f64 = (p.p147 * (nv4 - 0.0));
        let assign6910_e7175: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, assign6910_e7174);
        let assign6910_e7177: f64 = (assign6910_e7175 * p.p1);
        locals.var_i_cth = assign6910_e7177;
        locals.var_i_cth_dn4 = ((p.p147 * ddt_scale) * p.p1);

        let assign7090_e7293: f64 = (locals.var_if_ + locals.var_ir);
        let assign7090_e7295: f64 = (assign7090_e7293 / locals.var_qbi);
        locals.var_in_n = assign7090_e7295;
        locals.var_in_n_dn0 = ((((locals.var_if__dn0 + locals.var_ir_dn0) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn1 = ((((locals.var_if__dn1 + locals.var_ir_dn1) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn3 = ((((locals.var_if__dn3 + locals.var_ir_dn3) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn4 = ((((locals.var_if__dn4 + locals.var_ir_dn4) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn5 = ((((locals.var_if__dn5 + locals.var_ir_dn5) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn6 = ((((locals.var_if__dn6 + locals.var_ir_dn6) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn7 = ((((locals.var_if__dn7 + locals.var_ir_dn7) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn8 = ((((locals.var_if__dn8 + locals.var_ir_dn8) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn9 = ((((locals.var_if__dn9 + locals.var_ir_dn9) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn10 = ((((locals.var_if__dn10 + locals.var_ir_dn10) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in_n_dn11 = ((((locals.var_if__dn11 + locals.var_ir_dn11) * locals.var_qbi) - (assign7090_e7293 * locals.var_qbi_dn11)) / (locals.var_qbi * locals.var_qbi));

        let assign7150_e7328: f64 = if locals.var_in_n > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7150_e7328;

        let (assign7160_e7336, assign7160_e7336_d_n0, assign7160_e7336_d_n1, assign7160_e7336_d_n3, assign7160_e7336_d_n4, assign7160_e7336_d_n5, assign7160_e7336_d_n6, assign7160_e7336_d_n7, assign7160_e7336_d_n8, assign7160_e7336_d_n9, assign7160_e7336_d_n10, assign7160_e7336_d_n11,) = {
    if (locals.var_guard132 != 0.0) {
        let assign7160_e7332: f64 = (locals.var_qbe + locals.var_qbc);
        let assign7160_e7334: f64 = (assign7160_e7332 / locals.var_in_n);
        (assign7160_e7334, ((((locals.var_qbe_dn0 + locals.var_qbc_dn0) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn0)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn1 + locals.var_qbc_dn1) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn1)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn3 + locals.var_qbc_dn3) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn3)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn4 + locals.var_qbc_dn4) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn4)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn5 + locals.var_qbc_dn5) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn5)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn6 + locals.var_qbc_dn6) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn6)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn7 + locals.var_qbc_dn7) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn7)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn8 + locals.var_qbc_dn8) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn8)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn9 + locals.var_qbc_dn9) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn9)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn10 + locals.var_qbc_dn10) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn10)) / (locals.var_in_n * locals.var_in_n)), ((((locals.var_qbe_dn11 + locals.var_qbc_dn11) * locals.var_in_n) - (assign7160_e7332 * locals.var_in_n_dn11)) / (locals.var_in_n * locals.var_in_n)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10, locals.var_taub_n_dn11,)
    }
};
        locals.var_taub_n = assign7160_e7336;
        locals.var_taub_n_dn0 = assign7160_e7336_d_n0;
        locals.var_taub_n_dn1 = assign7160_e7336_d_n1;
        locals.var_taub_n_dn3 = assign7160_e7336_d_n3;
        locals.var_taub_n_dn4 = assign7160_e7336_d_n4;
        locals.var_taub_n_dn5 = assign7160_e7336_d_n5;
        locals.var_taub_n_dn6 = assign7160_e7336_d_n6;
        locals.var_taub_n_dn7 = assign7160_e7336_d_n7;
        locals.var_taub_n_dn8 = assign7160_e7336_d_n8;
        locals.var_taub_n_dn9 = assign7160_e7336_d_n9;
        locals.var_taub_n_dn10 = assign7160_e7336_d_n10;
        locals.var_taub_n_dn11 = assign7160_e7336_d_n11;

        let (assign7170_e7345, assign7170_e7345_d_n0, assign7170_e7345_d_n1, assign7170_e7345_d_n3, assign7170_e7345_d_n4, assign7170_e7345_d_n5, assign7170_e7345_d_n6, assign7170_e7345_d_n7, assign7170_e7345_d_n8, assign7170_e7345_d_n9, assign7170_e7345_d_n10, assign7170_e7345_d_n11,) = {
    if (locals.var_guard132 == 0.0) {
        let assign7170_e7341: f64 = (locals.var_taub_t * locals.var_q1q);
        let assign7170_e7343: f64 = (assign7170_e7341 * locals.var_qbi);
        (assign7170_e7343, (((locals.var_taub_t * locals.var_q1q_dn0) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn0)), (((locals.var_taub_t * locals.var_q1q_dn1) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn1)), (((locals.var_taub_t * locals.var_q1q_dn3) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn3)), ((((locals.var_taub_t_dn4 * locals.var_q1q) + (locals.var_taub_t * locals.var_q1q_dn4)) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn4)), (((locals.var_taub_t * locals.var_q1q_dn5) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn5)), (((locals.var_taub_t * locals.var_q1q_dn6) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn6)), (((locals.var_taub_t * locals.var_q1q_dn7) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn7)), (((locals.var_taub_t * locals.var_q1q_dn8) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn8)), (((locals.var_taub_t * locals.var_q1q_dn9) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn9)), (((locals.var_taub_t * locals.var_q1q_dn10) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn10)), (((locals.var_taub_t * locals.var_q1q_dn11) * locals.var_qbi) + (assign7170_e7341 * locals.var_qbi_dn11)),)
    } else {
        (locals.var_taub_n, locals.var_taub_n_dn0, locals.var_taub_n_dn1, locals.var_taub_n_dn3, locals.var_taub_n_dn4, locals.var_taub_n_dn5, locals.var_taub_n_dn6, locals.var_taub_n_dn7, locals.var_taub_n_dn8, locals.var_taub_n_dn9, locals.var_taub_n_dn10, locals.var_taub_n_dn11,)
    }
};
        locals.var_taub_n = assign7170_e7345;
        locals.var_taub_n_dn0 = assign7170_e7345_d_n0;
        locals.var_taub_n_dn1 = assign7170_e7345_d_n1;
        locals.var_taub_n_dn3 = assign7170_e7345_d_n3;
        locals.var_taub_n_dn4 = assign7170_e7345_d_n4;
        locals.var_taub_n_dn5 = assign7170_e7345_d_n5;
        locals.var_taub_n_dn6 = assign7170_e7345_d_n6;
        locals.var_taub_n_dn7 = assign7170_e7345_d_n7;
        locals.var_taub_n_dn8 = assign7170_e7345_d_n8;
        locals.var_taub_n_dn9 = assign7170_e7345_d_n9;
        locals.var_taub_n_dn10 = assign7170_e7345_d_n10;
        locals.var_taub_n_dn11 = assign7170_e7345_d_n11;

        let assign7180_e7348: f64 = if p.p131 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7180_e7348;

        let (assign7190_e7354, assign7190_e7354_d_n0, assign7190_e7354_d_n1, assign7190_e7354_d_n3, assign7190_e7354_d_n4, assign7190_e7354_d_n5, assign7190_e7354_d_n6, assign7190_e7354_d_n7, assign7190_e7354_d_n8, assign7190_e7354_d_n9, assign7190_e7354_d_n10, assign7190_e7354_d_n11,) = {
    if (locals.var_guard133 != 0.0) {
        let assign7190_e7352: f64 = (p.p94 * locals.var_taub_n);
        (assign7190_e7352, (p.p94 * locals.var_taub_n_dn0), (p.p94 * locals.var_taub_n_dn1), (p.p94 * locals.var_taub_n_dn3), (p.p94 * locals.var_taub_n_dn4), (p.p94 * locals.var_taub_n_dn5), (p.p94 * locals.var_taub_n_dn6), (p.p94 * locals.var_taub_n_dn7), (p.p94 * locals.var_taub_n_dn8), (p.p94 * locals.var_taub_n_dn9), (p.p94 * locals.var_taub_n_dn10), (p.p94 * locals.var_taub_n_dn11),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7190_e7354;
        locals.var_taun_dn0 = assign7190_e7354_d_n0;
        locals.var_taun_dn1 = assign7190_e7354_d_n1;
        locals.var_taun_dn3 = assign7190_e7354_d_n3;
        locals.var_taun_dn4 = assign7190_e7354_d_n4;
        locals.var_taun_dn5 = assign7190_e7354_d_n5;
        locals.var_taun_dn6 = assign7190_e7354_d_n6;
        locals.var_taun_dn7 = assign7190_e7354_d_n7;
        locals.var_taun_dn8 = assign7190_e7354_d_n8;
        locals.var_taun_dn9 = assign7190_e7354_d_n9;
        locals.var_taun_dn10 = assign7190_e7354_d_n10;
        locals.var_taun_dn11 = assign7190_e7354_d_n11;

        let assign7200_e7357: f64 = if p.p131 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7200_e7357;

        let (assign7210_e7366, assign7210_e7366_d_n0, assign7210_e7366_d_n1, assign7210_e7366_d_n3, assign7210_e7366_d_n4, assign7210_e7366_d_n5, assign7210_e7366_d_n6, assign7210_e7366_d_n7, assign7210_e7366_d_n8, assign7210_e7366_d_n9, assign7210_e7366_d_n10, assign7210_e7366_d_n11,) = {
    if ((locals.var_guard133 == 0.0) && (locals.var_guard134 != 0.0)) {
        let assign7210_e7364: f64 = (p.p132 * locals.var_taub_n);
        (assign7210_e7364, (p.p132 * locals.var_taub_n_dn0), (p.p132 * locals.var_taub_n_dn1), (p.p132 * locals.var_taub_n_dn3), (p.p132 * locals.var_taub_n_dn4), (p.p132 * locals.var_taub_n_dn5), (p.p132 * locals.var_taub_n_dn6), (p.p132 * locals.var_taub_n_dn7), (p.p132 * locals.var_taub_n_dn8), (p.p132 * locals.var_taub_n_dn9), (p.p132 * locals.var_taub_n_dn10), (p.p132 * locals.var_taub_n_dn11),)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7210_e7366;
        locals.var_taun_dn0 = assign7210_e7366_d_n0;
        locals.var_taun_dn1 = assign7210_e7366_d_n1;
        locals.var_taun_dn3 = assign7210_e7366_d_n3;
        locals.var_taun_dn4 = assign7210_e7366_d_n4;
        locals.var_taun_dn5 = assign7210_e7366_d_n5;
        locals.var_taun_dn6 = assign7210_e7366_d_n6;
        locals.var_taun_dn7 = assign7210_e7366_d_n7;
        locals.var_taun_dn8 = assign7210_e7366_d_n8;
        locals.var_taun_dn9 = assign7210_e7366_d_n9;
        locals.var_taun_dn10 = assign7210_e7366_d_n10;
        locals.var_taun_dn11 = assign7210_e7366_d_n11;

        let (assign7220_e7374, assign7220_e7374_d_n0, assign7220_e7374_d_n1, assign7220_e7374_d_n3, assign7220_e7374_d_n4, assign7220_e7374_d_n5, assign7220_e7374_d_n6, assign7220_e7374_d_n7, assign7220_e7374_d_n8, assign7220_e7374_d_n9, assign7220_e7374_d_n10, assign7220_e7374_d_n11,) = {
    if ((locals.var_guard133 == 0.0) && (locals.var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taun, locals.var_taun_dn0, locals.var_taun_dn1, locals.var_taun_dn3, locals.var_taun_dn4, locals.var_taun_dn5, locals.var_taun_dn6, locals.var_taun_dn7, locals.var_taun_dn8, locals.var_taun_dn9, locals.var_taun_dn10, locals.var_taun_dn11,)
    }
};
        locals.var_taun = assign7220_e7374;
        locals.var_taun_dn0 = assign7220_e7374_d_n0;
        locals.var_taun_dn1 = assign7220_e7374_d_n1;
        locals.var_taun_dn3 = assign7220_e7374_d_n3;
        locals.var_taun_dn4 = assign7220_e7374_d_n4;
        locals.var_taun_dn5 = assign7220_e7374_d_n5;
        locals.var_taun_dn6 = assign7220_e7374_d_n6;
        locals.var_taun_dn7 = assign7220_e7374_d_n7;
        locals.var_taun_dn8 = assign7220_e7374_d_n8;
        locals.var_taun_dn9 = assign7220_e7374_d_n9;
        locals.var_taun_dn10 = assign7220_e7374_d_n10;
        locals.var_taun_dn11 = assign7220_e7374_d_n11;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign00_e607: f64 = if p.p3 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e607;
        locals.var_guard1_rv = 0.0;

        let (assign10_e611,) = {
    if (locals.var_guard1 != 0.0) {
        (70300000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign10_e611;
        locals.var_an_rv = 0.0;

        let (assign20_e615,) = {
    if (locals.var_guard1 != 0.0) {
        (123000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign20_e615;
        locals.var_bn_rv = 0.0;

        let (assign30_e620,) = {
    if (locals.var_guard1 == 0.0) {
        (158000000.0,)
    } else {
        (locals.var_an,)
    }
};
        locals.var_an = assign30_e620;
        locals.var_an_rv = 0.0;

        let (assign40_e625,) = {
    if (locals.var_guard1 == 0.0) {
        (204000000.0,)
    } else {
        (locals.var_bn,)
    }
};
        locals.var_bn = assign40_e625;
        locals.var_bn_rv = 0.0;

        let assign50_e628: f64 = (1.0 - p.p33);
        locals.var_xext1 = assign50_e628;
        locals.var_xext1_rv = 0.0;

        let assign60_e631: f64 = (p.p4 + 273.15);
        locals.var_trk = assign60_e631;
        locals.var_trk_rv = 0.0;

        let assign70_e632: f64 = ctx_temp;
        let assign70_e634: f64 = (assign70_e632 + p.p0);
        locals.var_tamb = assign70_e634;
        locals.var_tamb_rv = 0.0;

        let assign90_e640: f64 = if p.p154 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign90_e640;
        locals.var_guard2_rv = 0.0;

        let (assign100_e644,) = {
    if (locals.var_guard2 != 0.0) {
        (1e-12,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign100_e644;
        locals.var_minr_rv = 0.0;

        let (assign110_e649,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p154,)
    } else {
        (locals.var_minr,)
    }
};
        locals.var_minr = assign110_e649;
        locals.var_minr_rv = 0.0;

        let assign120_e652: f64 = (locals.var_minr * p.p1);
        locals.var_minr_m = assign120_e652;
        locals.var_minr_m_rv = 0.0;

        locals.var_eps_nf = 0.001;
        locals.var_eps_nf_rv = 0.0;

        locals.var_eps_bavl_t = 0.001;
        locals.var_eps_bavl_t_rv = 0.0;

        let assign190_e673: f64 = (2.0 - p.p67);
        let assign190_e674: f64 = (2.0_f64).powf(assign190_e673);
        locals.var_pow2_2m_pe = assign190_e674;
        locals.var_pow2_2m_pe_rv = 0.0;

        let assign210_e681: f64 = (p.p115 * locals.var_trk);
        let assign210_e683: f64 = (assign210_e681 * locals.var_trk);
        let assign210_e686: f64 = (locals.var_trk + p.p116);
        let assign210_e687: f64 = (assign210_e683 / assign210_e686);
        let assign210_e688: f64 = (p.p114 + assign210_e687);
        let assign210_e690: f64 = (assign210_e688 - 0.05);
        let assign210_e692: f64 = (assign210_e690 / 0.1);
        locals.var_dxa = assign210_e692;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign220_e696: f64 = (p.p115 * locals.var_trk);
        let assign220_e698: f64 = (assign220_e696 * locals.var_trk);
        let assign220_e701: f64 = (locals.var_trk + p.p116);
        let assign220_e702: f64 = (assign220_e698 / assign220_e701);
        let assign220_e703: f64 = (p.p114 + assign220_e702);
        let assign220_e705: f64 = if assign220_e703 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign220_e705;
        locals.var_guard4_rv = 0.0;

        let (assign230_e717, assign230_e717_d_n0, assign230_e717_d_n1, assign230_e717_d_n3, assign230_e717_d_n4, assign230_e717_d_n5, assign230_e717_d_n6, assign230_e717_d_n7, assign230_e717_d_n8, assign230_e717_d_n9, assign230_e717_d_n10, assign230_e717_d_n11,) = {
    if (locals.var_guard4 != 0.0) {
        let assign230_e711: f64 = (locals.var_dxa).exp();
        let assign230_e712: f64 = (1.0 + assign230_e711);
        let assign230_e713: f64 = (assign230_e712).ln();
        let assign230_e714: f64 = (0.1 * assign230_e713);
        let assign230_e715: f64 = (0.05 + assign230_e714);
        (assign230_e715, (0.1 * ((assign230_e711 * locals.var_dxa_dn0) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn1) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn3) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn4) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn5) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn6) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn7) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn8) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn9) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn10) / assign230_e712)), (0.1 * ((assign230_e711 * locals.var_dxa_dn11) / assign230_e712)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9, locals.var_vgzebok_dn10, locals.var_vgzebok_dn11,)
    }
};
        locals.var_vgzebok = assign230_e717;
        locals.var_vgzebok_dn0 = assign230_e717_d_n0;
        locals.var_vgzebok_dn1 = assign230_e717_d_n1;
        locals.var_vgzebok_dn3 = assign230_e717_d_n3;
        locals.var_vgzebok_dn4 = assign230_e717_d_n4;
        locals.var_vgzebok_dn5 = assign230_e717_d_n5;
        locals.var_vgzebok_dn6 = assign230_e717_d_n6;
        locals.var_vgzebok_dn7 = assign230_e717_d_n7;
        locals.var_vgzebok_dn8 = assign230_e717_d_n8;
        locals.var_vgzebok_dn9 = assign230_e717_d_n9;
        locals.var_vgzebok_dn10 = assign230_e717_d_n10;
        locals.var_vgzebok_dn11 = assign230_e717_d_n11;
        locals.var_vgzebok_rv = 0.0;

        let (assign240_e741, assign240_e741_d_n0, assign240_e741_d_n1, assign240_e741_d_n3, assign240_e741_d_n4, assign240_e741_d_n5, assign240_e741_d_n6, assign240_e741_d_n7, assign240_e741_d_n8, assign240_e741_d_n9, assign240_e741_d_n10, assign240_e741_d_n11,) = {
    if (locals.var_guard4 == 0.0) {
        let assign240_e723: f64 = (p.p115 * locals.var_trk);
        let assign240_e725: f64 = (assign240_e723 * locals.var_trk);
        let assign240_e728: f64 = (locals.var_trk + p.p116);
        let assign240_e729: f64 = (assign240_e725 / assign240_e728);
        let assign240_e730: f64 = (p.p114 + assign240_e729);
        let assign240_e734: f64 = (-locals.var_dxa);
        let assign240_e735: f64 = (assign240_e734).exp();
        let assign240_e736: f64 = (1.0 + assign240_e735);
        let assign240_e737: f64 = (assign240_e736).ln();
        let assign240_e738: f64 = (0.1 * assign240_e737);
        let assign240_e739: f64 = (assign240_e730 + assign240_e738);
        (assign240_e739, (0.1 * ((assign240_e735 * (-locals.var_dxa_dn0)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn1)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn3)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn4)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn5)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn6)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn7)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn8)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn9)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn10)) / assign240_e736)), (0.1 * ((assign240_e735 * (-locals.var_dxa_dn11)) / assign240_e736)),)
    } else {
        (locals.var_vgzebok, locals.var_vgzebok_dn0, locals.var_vgzebok_dn1, locals.var_vgzebok_dn3, locals.var_vgzebok_dn4, locals.var_vgzebok_dn5, locals.var_vgzebok_dn6, locals.var_vgzebok_dn7, locals.var_vgzebok_dn8, locals.var_vgzebok_dn9, locals.var_vgzebok_dn10, locals.var_vgzebok_dn11,)
    }
};
        locals.var_vgzebok = assign240_e741;
        locals.var_vgzebok_dn0 = assign240_e741_d_n0;
        locals.var_vgzebok_dn1 = assign240_e741_d_n1;
        locals.var_vgzebok_dn3 = assign240_e741_d_n3;
        locals.var_vgzebok_dn4 = assign240_e741_d_n4;
        locals.var_vgzebok_dn5 = assign240_e741_d_n5;
        locals.var_vgzebok_dn6 = assign240_e741_d_n6;
        locals.var_vgzebok_dn7 = assign240_e741_d_n7;
        locals.var_vgzebok_dn8 = assign240_e741_d_n8;
        locals.var_vgzebok_dn9 = assign240_e741_d_n9;
        locals.var_vgzebok_dn10 = assign240_e741_d_n10;
        locals.var_vgzebok_dn11 = assign240_e741_d_n11;
        locals.var_vgzebok_rv = 0.0;

        locals.var_vgzeb_tr = p.p114;
        locals.var_vgzeb_tr_rv = 0.0;

        let assign260_e745: f64 = (1.0 / locals.var_vgzeb_tr);
        locals.var_inv_vgzeb_tr = assign260_e745;
        locals.var_inv_vgzeb_tr_rv = 0.0;

        locals.var_vdc_zener = p.p71;
        locals.var_vdc_zener_rv = 0.0;

        locals.var_pc_zener = p.p72;
        locals.var_pc_zener_rv = 0.0;

        let assign300_e754: f64 = (2.0 - locals.var_pc_zener);
        let assign300_e755: f64 = (2.0_f64).powf(assign300_e754);
        locals.var_pow2_2m_pc = assign300_e755;
        locals.var_pow2_2m_pc_rv = 0.0;

        let assign320_e762: f64 = (p.p118 * locals.var_trk);
        let assign320_e764: f64 = (assign320_e762 * locals.var_trk);
        let assign320_e767: f64 = (locals.var_trk + p.p119);
        let assign320_e768: f64 = (assign320_e764 / assign320_e767);
        let assign320_e769: f64 = (p.p117 + assign320_e768);
        let assign320_e771: f64 = (assign320_e769 - 0.05);
        let assign320_e773: f64 = (assign320_e771 / 0.1);
        locals.var_dxa = assign320_e773;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign330_e777: f64 = (p.p118 * locals.var_trk);
        let assign330_e779: f64 = (assign330_e777 * locals.var_trk);
        let assign330_e782: f64 = (locals.var_trk + p.p119);
        let assign330_e783: f64 = (assign330_e779 / assign330_e782);
        let assign330_e784: f64 = (p.p117 + assign330_e783);
        let assign330_e786: f64 = if assign330_e784 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign330_e786;
        locals.var_guard5_rv = 0.0;

        let (assign340_e798, assign340_e798_d_n0, assign340_e798_d_n1, assign340_e798_d_n3, assign340_e798_d_n4, assign340_e798_d_n5, assign340_e798_d_n6, assign340_e798_d_n7, assign340_e798_d_n8, assign340_e798_d_n9, assign340_e798_d_n10, assign340_e798_d_n11,) = {
    if (locals.var_guard5 != 0.0) {
        let assign340_e792: f64 = (locals.var_dxa).exp();
        let assign340_e793: f64 = (1.0 + assign340_e792);
        let assign340_e794: f64 = (assign340_e793).ln();
        let assign340_e795: f64 = (0.1 * assign340_e794);
        let assign340_e796: f64 = (0.05 + assign340_e795);
        (assign340_e796, (0.1 * ((assign340_e792 * locals.var_dxa_dn0) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn1) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn3) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn4) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn5) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn6) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn7) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn8) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn9) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn10) / assign340_e793)), (0.1 * ((assign340_e792 * locals.var_dxa_dn11) / assign340_e793)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9, locals.var_vgzcbok_dn10, locals.var_vgzcbok_dn11,)
    }
};
        locals.var_vgzcbok = assign340_e798;
        locals.var_vgzcbok_dn0 = assign340_e798_d_n0;
        locals.var_vgzcbok_dn1 = assign340_e798_d_n1;
        locals.var_vgzcbok_dn3 = assign340_e798_d_n3;
        locals.var_vgzcbok_dn4 = assign340_e798_d_n4;
        locals.var_vgzcbok_dn5 = assign340_e798_d_n5;
        locals.var_vgzcbok_dn6 = assign340_e798_d_n6;
        locals.var_vgzcbok_dn7 = assign340_e798_d_n7;
        locals.var_vgzcbok_dn8 = assign340_e798_d_n8;
        locals.var_vgzcbok_dn9 = assign340_e798_d_n9;
        locals.var_vgzcbok_dn10 = assign340_e798_d_n10;
        locals.var_vgzcbok_dn11 = assign340_e798_d_n11;
        locals.var_vgzcbok_rv = 0.0;

        let (assign350_e822, assign350_e822_d_n0, assign350_e822_d_n1, assign350_e822_d_n3, assign350_e822_d_n4, assign350_e822_d_n5, assign350_e822_d_n6, assign350_e822_d_n7, assign350_e822_d_n8, assign350_e822_d_n9, assign350_e822_d_n10, assign350_e822_d_n11,) = {
    if (locals.var_guard5 == 0.0) {
        let assign350_e804: f64 = (p.p118 * locals.var_trk);
        let assign350_e806: f64 = (assign350_e804 * locals.var_trk);
        let assign350_e809: f64 = (locals.var_trk + p.p119);
        let assign350_e810: f64 = (assign350_e806 / assign350_e809);
        let assign350_e811: f64 = (p.p117 + assign350_e810);
        let assign350_e815: f64 = (-locals.var_dxa);
        let assign350_e816: f64 = (assign350_e815).exp();
        let assign350_e817: f64 = (1.0 + assign350_e816);
        let assign350_e818: f64 = (assign350_e817).ln();
        let assign350_e819: f64 = (0.1 * assign350_e818);
        let assign350_e820: f64 = (assign350_e811 + assign350_e819);
        (assign350_e820, (0.1 * ((assign350_e816 * (-locals.var_dxa_dn0)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn1)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn3)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn4)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn5)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn6)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn7)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn8)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn9)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn10)) / assign350_e817)), (0.1 * ((assign350_e816 * (-locals.var_dxa_dn11)) / assign350_e817)),)
    } else {
        (locals.var_vgzcbok, locals.var_vgzcbok_dn0, locals.var_vgzcbok_dn1, locals.var_vgzcbok_dn3, locals.var_vgzcbok_dn4, locals.var_vgzcbok_dn5, locals.var_vgzcbok_dn6, locals.var_vgzcbok_dn7, locals.var_vgzcbok_dn8, locals.var_vgzcbok_dn9, locals.var_vgzcbok_dn10, locals.var_vgzcbok_dn11,)
    }
};
        locals.var_vgzcbok = assign350_e822;
        locals.var_vgzcbok_dn0 = assign350_e822_d_n0;
        locals.var_vgzcbok_dn1 = assign350_e822_d_n1;
        locals.var_vgzcbok_dn3 = assign350_e822_d_n3;
        locals.var_vgzcbok_dn4 = assign350_e822_d_n4;
        locals.var_vgzcbok_dn5 = assign350_e822_d_n5;
        locals.var_vgzcbok_dn6 = assign350_e822_d_n6;
        locals.var_vgzcbok_dn7 = assign350_e822_d_n7;
        locals.var_vgzcbok_dn8 = assign350_e822_d_n8;
        locals.var_vgzcbok_dn9 = assign350_e822_d_n9;
        locals.var_vgzcbok_dn10 = assign350_e822_d_n10;
        locals.var_vgzcbok_dn11 = assign350_e822_d_n11;
        locals.var_vgzcbok_rv = 0.0;

        locals.var_vgzcb_tr = p.p117;
        locals.var_vgzcb_tr_rv = 0.0;

        let assign370_e826: f64 = (1.0 / locals.var_vgzcb_tr);
        locals.var_inv_vgzcb_tr = assign370_e826;
        locals.var_inv_vgzcb_tr_rv = 0.0;

        locals.var_fex = 1.0;
        locals.var_fex_dn0 = 0.0;
        locals.var_fex_dn1 = 0.0;
        locals.var_fex_dn3 = 0.0;
        locals.var_fex_dn4 = 0.0;
        locals.var_fex_dn5 = 0.0;
        locals.var_fex_dn6 = 0.0;
        locals.var_fex_dn7 = 0.0;
        locals.var_fex_dn8 = 0.0;
        locals.var_fex_dn9 = 0.0;
        locals.var_fex_dn10 = 0.0;
        locals.var_fex_dn11 = 0.0;
        locals.var_fex_rv = 0.0;

        locals.var_gem = 0.0;
        locals.var_gem_dn0 = 0.0;
        locals.var_gem_dn1 = 0.0;
        locals.var_gem_dn3 = 0.0;
        locals.var_gem_dn4 = 0.0;
        locals.var_gem_dn5 = 0.0;
        locals.var_gem_dn6 = 0.0;
        locals.var_gem_dn7 = 0.0;
        locals.var_gem_dn8 = 0.0;
        locals.var_gem_dn9 = 0.0;
        locals.var_gem_dn10 = 0.0;
        locals.var_gem_dn11 = 0.0;
        locals.var_gem_rv = 0.0;

        locals.var_xqex = 0.0;
        locals.var_xqex_dn0 = 0.0;
        locals.var_xqex_dn1 = 0.0;
        locals.var_xqex_dn3 = 0.0;
        locals.var_xqex_dn4 = 0.0;
        locals.var_xqex_dn5 = 0.0;
        locals.var_xqex_dn6 = 0.0;
        locals.var_xqex_dn7 = 0.0;
        locals.var_xqex_dn8 = 0.0;
        locals.var_xqex_dn9 = 0.0;
        locals.var_xqex_dn10 = 0.0;
        locals.var_xqex_dn11 = 0.0;
        locals.var_xqex_rv = 0.0;

        locals.var_qb1b2 = 0.0;
        locals.var_qb1b2_dn0 = 0.0;
        locals.var_qb1b2_dn1 = 0.0;
        locals.var_qb1b2_dn3 = 0.0;
        locals.var_qb1b2_dn4 = 0.0;
        locals.var_qb1b2_dn5 = 0.0;
        locals.var_qb1b2_dn6 = 0.0;
        locals.var_qb1b2_dn7 = 0.0;
        locals.var_qb1b2_dn8 = 0.0;
        locals.var_qb1b2_dn9 = 0.0;
        locals.var_qb1b2_dn10 = 0.0;
        locals.var_qb1b2_dn11 = 0.0;
        locals.var_qb1b2_rv = 0.0;

        locals.var_ibi_t = 0.0;
        locals.var_ibi_t_dn4 = 0.0;
        locals.var_ibi_t_rv = 0.0;

        locals.var_tki = (nv4 - 0.0);
        locals.var_tki_dn4 = 1.0;
        locals.var_tki_rv = 0.0;

        let assign540_e851: f64 = if locals.var_tki < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign540_e851;
        locals.var_guard6_rv = 0.0;

        let (assign550_e859, assign550_e859_d_n4,) = {
    if (locals.var_guard6 != 0.0) {
        let assign550_e855: f64 = (1.0 - locals.var_tki);
        let assign550_e856: f64 = (assign550_e855).ln();
        let assign550_e857: f64 = (-assign550_e856);
        (assign550_e857, (-((-locals.var_tki_dn4) / assign550_e855)),)
    } else {
        (locals.var_tki, locals.var_tki_dn4,)
    }
};
        locals.var_tki = assign550_e859;
        locals.var_tki_dn4 = assign550_e859_d_n4;
        locals.var_tki_rv = 0.0;

        let assign560_e862: f64 = if locals.var_tki < p.p125 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign560_e862;
        locals.var_guard7_rv = 0.0;

        let (assign570_e866, assign570_e866_d_n4,) = {
    if (locals.var_guard7 != 0.0) {
        (locals.var_tki, locals.var_tki_dn4,)
    } else {
        (locals.var_vdt, locals.var_vdt_dn4,)
    }
};
        locals.var_vdt = assign570_e866;
        locals.var_vdt_dn4 = assign570_e866_d_n4;
        locals.var_vdt_rv = 0.0;

        let (assign580_e878, assign580_e878_d_n4,) = {
    if (locals.var_guard7 == 0.0) {
        let assign580_e873: f64 = (locals.var_tki - p.p125);
        let assign580_e874: f64 = (1.0 + assign580_e873);
        let assign580_e875: f64 = (assign580_e874).ln();
        let assign580_e876: f64 = (p.p125 + assign580_e875);
        (assign580_e876, (locals.var_tki_dn4 / assign580_e874),)
    } else {
        (locals.var_vdt, locals.var_vdt_dn4,)
    }
};
        locals.var_vdt = assign580_e878;
        locals.var_vdt_dn4 = assign580_e878_d_n4;
        locals.var_vdt_rv = 0.0;

        let assign590_e881: f64 = (locals.var_tamb + locals.var_vdt);
        locals.var_tk = assign590_e881;
        locals.var_tk_dn4 = locals.var_vdt_dn4;
        locals.var_tk_rv = 0.0;

        let assign600_e884: f64 = (locals.var_tk / locals.var_trk);
        locals.var_tn = assign600_e884;
        locals.var_tn_dn4 = (locals.var_tk_dn4 / locals.var_trk);
        locals.var_tn_rv = 0.0;

        let assign610_e887: f64 = (8.617086918058125e-5 * locals.var_tk);
        locals.var_vt = assign610_e887;
        locals.var_vt_dn4 = (8.617086918058125e-5 * locals.var_tk_dn4);
        locals.var_vt_rv = 0.0;

        let assign620_e890: f64 = (8.617086918058125e-5 * locals.var_trk);
        locals.var_vtr = assign620_e890;
        locals.var_vtr_rv = 0.0;

        let assign630_e893: f64 = (1.0 / locals.var_vt);
        locals.var_vtinv = assign630_e893;
        locals.var_vtinv_dn4 = (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt)));
        locals.var_vtinv_rv = 0.0;

        let assign640_e896: f64 = (1.0 / locals.var_vtr);
        locals.var_vtrinv = assign640_e896;
        locals.var_vtrinv_rv = 0.0;

        let assign650_e899: f64 = (locals.var_vtinv - locals.var_vtrinv);
        locals.var_vdtinv = assign650_e899;
        locals.var_vdtinv_dn4 = locals.var_vtinv_dn4;
        locals.var_vdtinv_rv = 0.0;

        let assign660_e902: f64 = (locals.var_tk - locals.var_trk);
        locals.var_dt = assign660_e902;
        locals.var_dt_dn4 = locals.var_tk_dn4;
        locals.var_dt_rv = 0.0;

        let assign670_e904: f64 = (locals.var_tn).ln();
        locals.var_lntn = assign670_e904;
        locals.var_lntn_dn4 = (locals.var_tn_dn4 / locals.var_tn);
        locals.var_lntn_rv = 0.0;

        let assign680_e908: f64 = (p.p115 * locals.var_tk);
        let assign680_e910: f64 = (assign680_e908 * locals.var_tk);
        let assign680_e913: f64 = (locals.var_tk + p.p116);
        let assign680_e914: f64 = (assign680_e910 / assign680_e913);
        let assign680_e915: f64 = (locals.var_vgzebok - assign680_e914);
        let assign680_e917: f64 = (assign680_e915 - 0.05);
        let assign680_e919: f64 = (assign680_e917 / 0.1);
        locals.var_dxa = assign680_e919;
        locals.var_dxa_dn0 = (locals.var_vgzebok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzebok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzebok_dn3 / 0.1);
        locals.var_dxa_dn4 = ((locals.var_vgzebok_dn4 - ((((((p.p115 * locals.var_tk_dn4) * locals.var_tk) + (assign680_e908 * locals.var_tk_dn4)) * assign680_e913) - (assign680_e910 * locals.var_tk_dn4)) / (assign680_e913 * assign680_e913))) / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzebok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzebok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzebok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzebok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzebok_dn9 / 0.1);
        locals.var_dxa_dn10 = (locals.var_vgzebok_dn10 / 0.1);
        locals.var_dxa_dn11 = (locals.var_vgzebok_dn11 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign690_e923: f64 = (p.p115 * locals.var_tk);
        let assign690_e925: f64 = (assign690_e923 * locals.var_tk);
        let assign690_e928: f64 = (locals.var_tk + p.p116);
        let assign690_e929: f64 = (assign690_e925 / assign690_e928);
        let assign690_e930: f64 = (locals.var_vgzebok - assign690_e929);
        let assign690_e932: f64 = if assign690_e930 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign690_e932;
        locals.var_guard8_rv = 0.0;

        let (assign700_e944, assign700_e944_d_n0, assign700_e944_d_n1, assign700_e944_d_n3, assign700_e944_d_n4, assign700_e944_d_n5, assign700_e944_d_n6, assign700_e944_d_n7, assign700_e944_d_n8, assign700_e944_d_n9, assign700_e944_d_n10, assign700_e944_d_n11,) = {
    if (locals.var_guard8 != 0.0) {
        let assign700_e938: f64 = (locals.var_dxa).exp();
        let assign700_e939: f64 = (1.0 + assign700_e938);
        let assign700_e940: f64 = (assign700_e939).ln();
        let assign700_e941: f64 = (0.1 * assign700_e940);
        let assign700_e942: f64 = (0.05 + assign700_e941);
        (assign700_e942, (0.1 * ((assign700_e938 * locals.var_dxa_dn0) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn1) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn3) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn4) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn5) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn6) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn7) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn8) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn9) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn10) / assign700_e939)), (0.1 * ((assign700_e938 * locals.var_dxa_dn11) / assign700_e939)),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9, locals.var_vgzeb_t_dn10, locals.var_vgzeb_t_dn11,)
    }
};
        locals.var_vgzeb_t = assign700_e944;
        locals.var_vgzeb_t_dn0 = assign700_e944_d_n0;
        locals.var_vgzeb_t_dn1 = assign700_e944_d_n1;
        locals.var_vgzeb_t_dn3 = assign700_e944_d_n3;
        locals.var_vgzeb_t_dn4 = assign700_e944_d_n4;
        locals.var_vgzeb_t_dn5 = assign700_e944_d_n5;
        locals.var_vgzeb_t_dn6 = assign700_e944_d_n6;
        locals.var_vgzeb_t_dn7 = assign700_e944_d_n7;
        locals.var_vgzeb_t_dn8 = assign700_e944_d_n8;
        locals.var_vgzeb_t_dn9 = assign700_e944_d_n9;
        locals.var_vgzeb_t_dn10 = assign700_e944_d_n10;
        locals.var_vgzeb_t_dn11 = assign700_e944_d_n11;
        locals.var_vgzeb_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign710_e968, assign710_e968_d_n0, assign710_e968_d_n1, assign710_e968_d_n3, assign710_e968_d_n4, assign710_e968_d_n5, assign710_e968_d_n6, assign710_e968_d_n7, assign710_e968_d_n8, assign710_e968_d_n9, assign710_e968_d_n10, assign710_e968_d_n11,) = {
    if (locals.var_guard8 == 0.0) {
        let assign710_e950: f64 = (p.p115 * locals.var_tk);
        let assign710_e952: f64 = (assign710_e950 * locals.var_tk);
        let assign710_e955: f64 = (locals.var_tk + p.p116);
        let assign710_e956: f64 = (assign710_e952 / assign710_e955);
        let assign710_e957: f64 = (locals.var_vgzebok - assign710_e956);
        let assign710_e961: f64 = (-locals.var_dxa);
        let assign710_e962: f64 = (assign710_e961).exp();
        let assign710_e963: f64 = (1.0 + assign710_e962);
        let assign710_e964: f64 = (assign710_e963).ln();
        let assign710_e965: f64 = (0.1 * assign710_e964);
        let assign710_e966: f64 = (assign710_e957 + assign710_e965);
        (assign710_e966, (locals.var_vgzebok_dn0 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn0)) / assign710_e963))), (locals.var_vgzebok_dn1 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn1)) / assign710_e963))), (locals.var_vgzebok_dn3 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn3)) / assign710_e963))), ((locals.var_vgzebok_dn4 - ((((((p.p115 * locals.var_tk_dn4) * locals.var_tk) + (assign710_e950 * locals.var_tk_dn4)) * assign710_e955) - (assign710_e952 * locals.var_tk_dn4)) / (assign710_e955 * assign710_e955))) + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn4)) / assign710_e963))), (locals.var_vgzebok_dn5 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn5)) / assign710_e963))), (locals.var_vgzebok_dn6 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn6)) / assign710_e963))), (locals.var_vgzebok_dn7 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn7)) / assign710_e963))), (locals.var_vgzebok_dn8 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn8)) / assign710_e963))), (locals.var_vgzebok_dn9 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn9)) / assign710_e963))), (locals.var_vgzebok_dn10 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn10)) / assign710_e963))), (locals.var_vgzebok_dn11 + (0.1 * ((assign710_e962 * (-locals.var_dxa_dn11)) / assign710_e963))),)
    } else {
        (locals.var_vgzeb_t, locals.var_vgzeb_t_dn0, locals.var_vgzeb_t_dn1, locals.var_vgzeb_t_dn3, locals.var_vgzeb_t_dn4, locals.var_vgzeb_t_dn5, locals.var_vgzeb_t_dn6, locals.var_vgzeb_t_dn7, locals.var_vgzeb_t_dn8, locals.var_vgzeb_t_dn9, locals.var_vgzeb_t_dn10, locals.var_vgzeb_t_dn11,)
    }
};
        locals.var_vgzeb_t = assign710_e968;
        locals.var_vgzeb_t_dn0 = assign710_e968_d_n0;
        locals.var_vgzeb_t_dn1 = assign710_e968_d_n1;
        locals.var_vgzeb_t_dn3 = assign710_e968_d_n3;
        locals.var_vgzeb_t_dn4 = assign710_e968_d_n4;
        locals.var_vgzeb_t_dn5 = assign710_e968_d_n5;
        locals.var_vgzeb_t_dn6 = assign710_e968_d_n6;
        locals.var_vgzeb_t_dn7 = assign710_e968_d_n7;
        locals.var_vgzeb_t_dn8 = assign710_e968_d_n8;
        locals.var_vgzeb_t_dn9 = assign710_e968_d_n9;
        locals.var_vgzeb_t_dn10 = assign710_e968_d_n10;
        locals.var_vgzeb_t_dn11 = assign710_e968_d_n11;
        locals.var_vgzeb_t_rv = 0.0;

        let assign720_e972: f64 = (p.p118 * locals.var_tk);
        let assign720_e974: f64 = (assign720_e972 * locals.var_tk);
        let assign720_e977: f64 = (locals.var_tk + p.p119);
        let assign720_e978: f64 = (assign720_e974 / assign720_e977);
        let assign720_e979: f64 = (locals.var_vgzcbok - assign720_e978);
        let assign720_e981: f64 = (assign720_e979 - 0.05);
        let assign720_e983: f64 = (assign720_e981 / 0.1);
        locals.var_dxa = assign720_e983;
        locals.var_dxa_dn0 = (locals.var_vgzcbok_dn0 / 0.1);
        locals.var_dxa_dn1 = (locals.var_vgzcbok_dn1 / 0.1);
        locals.var_dxa_dn3 = (locals.var_vgzcbok_dn3 / 0.1);
        locals.var_dxa_dn4 = ((locals.var_vgzcbok_dn4 - ((((((p.p118 * locals.var_tk_dn4) * locals.var_tk) + (assign720_e972 * locals.var_tk_dn4)) * assign720_e977) - (assign720_e974 * locals.var_tk_dn4)) / (assign720_e977 * assign720_e977))) / 0.1);
        locals.var_dxa_dn5 = (locals.var_vgzcbok_dn5 / 0.1);
        locals.var_dxa_dn6 = (locals.var_vgzcbok_dn6 / 0.1);
        locals.var_dxa_dn7 = (locals.var_vgzcbok_dn7 / 0.1);
        locals.var_dxa_dn8 = (locals.var_vgzcbok_dn8 / 0.1);
        locals.var_dxa_dn9 = (locals.var_vgzcbok_dn9 / 0.1);
        locals.var_dxa_dn10 = (locals.var_vgzcbok_dn10 / 0.1);
        locals.var_dxa_dn11 = (locals.var_vgzcbok_dn11 / 0.1);
        locals.var_dxa_rv = 0.0;

        let assign730_e987: f64 = (p.p118 * locals.var_tk);
        let assign730_e989: f64 = (assign730_e987 * locals.var_tk);
        let assign730_e992: f64 = (locals.var_tk + p.p119);
        let assign730_e993: f64 = (assign730_e989 / assign730_e992);
        let assign730_e994: f64 = (locals.var_vgzcbok - assign730_e993);
        let assign730_e996: f64 = if assign730_e994 < 0.05 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign730_e996;
        locals.var_guard9_rv = 0.0;

        let (assign740_e1008, assign740_e1008_d_n0, assign740_e1008_d_n1, assign740_e1008_d_n3, assign740_e1008_d_n4, assign740_e1008_d_n5, assign740_e1008_d_n6, assign740_e1008_d_n7, assign740_e1008_d_n8, assign740_e1008_d_n9, assign740_e1008_d_n10, assign740_e1008_d_n11,) = {
    if (locals.var_guard9 != 0.0) {
        let assign740_e1002: f64 = (locals.var_dxa).exp();
        let assign740_e1003: f64 = (1.0 + assign740_e1002);
        let assign740_e1004: f64 = (assign740_e1003).ln();
        let assign740_e1005: f64 = (0.1 * assign740_e1004);
        let assign740_e1006: f64 = (0.05 + assign740_e1005);
        (assign740_e1006, (0.1 * ((assign740_e1002 * locals.var_dxa_dn0) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn1) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn3) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn4) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn5) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn6) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn7) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn8) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn9) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn10) / assign740_e1003)), (0.1 * ((assign740_e1002 * locals.var_dxa_dn11) / assign740_e1003)),)
    } else {
        (locals.var_vgzcb_t, locals.var_vgzcb_t_dn0, locals.var_vgzcb_t_dn1, locals.var_vgzcb_t_dn3, locals.var_vgzcb_t_dn4, locals.var_vgzcb_t_dn5, locals.var_vgzcb_t_dn6, locals.var_vgzcb_t_dn7, locals.var_vgzcb_t_dn8, locals.var_vgzcb_t_dn9, locals.var_vgzcb_t_dn10, locals.var_vgzcb_t_dn11,)
    }
};
        locals.var_vgzcb_t = assign740_e1008;
        locals.var_vgzcb_t_dn0 = assign740_e1008_d_n0;
        locals.var_vgzcb_t_dn1 = assign740_e1008_d_n1;
        locals.var_vgzcb_t_dn3 = assign740_e1008_d_n3;
        locals.var_vgzcb_t_dn4 = assign740_e1008_d_n4;
        locals.var_vgzcb_t_dn5 = assign740_e1008_d_n5;
        locals.var_vgzcb_t_dn6 = assign740_e1008_d_n6;
        locals.var_vgzcb_t_dn7 = assign740_e1008_d_n7;
        locals.var_vgzcb_t_dn8 = assign740_e1008_d_n8;
        locals.var_vgzcb_t_dn9 = assign740_e1008_d_n9;
        locals.var_vgzcb_t_dn10 = assign740_e1008_d_n10;
        locals.var_vgzcb_t_dn11 = assign740_e1008_d_n11;
        locals.var_vgzcb_t_rv = 0.0;

        let (assign750_e1032, assign750_e1032_d_n0, assign750_e1032_d_n1, assign750_e1032_d_n3, assign750_e1032_d_n4, assign750_e1032_d_n5, assign750_e1032_d_n6, assign750_e1032_d_n7, assign750_e1032_d_n8, assign750_e1032_d_n9, assign750_e1032_d_n10, assign750_e1032_d_n11,) = {
    if (locals.var_guard9 == 0.0) {
        let assign750_e1014: f64 = (p.p118 * locals.var_tk);
        let assign750_e1016: f64 = (assign750_e1014 * locals.var_tk);
        let assign750_e1019: f64 = (locals.var_tk + p.p119);
        let assign750_e1020: f64 = (assign750_e1016 / assign750_e1019);
        let assign750_e1021: f64 = (locals.var_vgzcbok - assign750_e1020);
        let assign750_e1025: f64 = (-locals.var_dxa);
        let assign750_e1026: f64 = (assign750_e1025).exp();
        let assign750_e1027: f64 = (1.0 + assign750_e1026);
        let assign750_e1028: f64 = (assign750_e1027).ln();
        let assign750_e1029: f64 = (0.1 * assign750_e1028);
        let assign750_e1030: f64 = (assign750_e1021 + assign750_e1029);
        (assign750_e1030, (locals.var_vgzcbok_dn0 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn0)) / assign750_e1027))), (locals.var_vgzcbok_dn1 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn1)) / assign750_e1027))), (locals.var_vgzcbok_dn3 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn3)) / assign750_e1027))), ((locals.var_vgzcbok_dn4 - ((((((p.p118 * locals.var_tk_dn4) * locals.var_tk) + (assign750_e1014 * locals.var_tk_dn4)) * assign750_e1019) - (assign750_e1016 * locals.var_tk_dn4)) / (assign750_e1019 * assign750_e1019))) + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn4)) / assign750_e1027))), (locals.var_vgzcbok_dn5 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn5)) / assign750_e1027))), (locals.var_vgzcbok_dn6 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn6)) / assign750_e1027))), (locals.var_vgzcbok_dn7 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn7)) / assign750_e1027))), (locals.var_vgzcbok_dn8 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn8)) / assign750_e1027))), (locals.var_vgzcbok_dn9 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn9)) / assign750_e1027))), (locals.var_vgzcbok_dn10 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn10)) / assign750_e1027))), (locals.var_vgzcbok_dn11 + (0.1 * ((assign750_e1026 * (-locals.var_dxa_dn11)) / assign750_e1027))),)
    } else {
        (locals.var_vgzcb_t, locals.var_vgzcb_t_dn0, locals.var_vgzcb_t_dn1, locals.var_vgzcb_t_dn3, locals.var_vgzcb_t_dn4, locals.var_vgzcb_t_dn5, locals.var_vgzcb_t_dn6, locals.var_vgzcb_t_dn7, locals.var_vgzcb_t_dn8, locals.var_vgzcb_t_dn9, locals.var_vgzcb_t_dn10, locals.var_vgzcb_t_dn11,)
    }
};
        locals.var_vgzcb_t = assign750_e1032;
        locals.var_vgzcb_t_dn0 = assign750_e1032_d_n0;
        locals.var_vgzcb_t_dn1 = assign750_e1032_d_n1;
        locals.var_vgzcb_t_dn3 = assign750_e1032_d_n3;
        locals.var_vgzcb_t_dn4 = assign750_e1032_d_n4;
        locals.var_vgzcb_t_dn5 = assign750_e1032_d_n5;
        locals.var_vgzcb_t_dn6 = assign750_e1032_d_n6;
        locals.var_vgzcb_t_dn7 = assign750_e1032_d_n7;
        locals.var_vgzcb_t_dn8 = assign750_e1032_d_n8;
        locals.var_vgzcb_t_dn9 = assign750_e1032_d_n9;
        locals.var_vgzcb_t_dn10 = assign750_e1032_d_n10;
        locals.var_vgzcb_t_dn11 = assign750_e1032_d_n11;
        locals.var_vgzcb_t_rv = 0.0;

        let assign760_e1034: f64 = (-3.0);
        let assign760_e1036: f64 = (assign760_e1034 * locals.var_vt);
        let assign760_e1038: f64 = (assign760_e1036 * locals.var_lntn);
        let assign760_e1041: f64 = (p.p66 * locals.var_tn);
        let assign760_e1042: f64 = (assign760_e1038 + assign760_e1041);
        let assign760_e1045: f64 = (1.0 - locals.var_tn);
        let assign760_e1047: f64 = (assign760_e1045 * p.p105);
        let assign760_e1048: f64 = (assign760_e1042 + assign760_e1047);
        locals.var_udet = assign760_e1048;
        locals.var_udet_dn4 = (((((assign760_e1034 * locals.var_vt_dn4) * locals.var_lntn) + (assign760_e1036 * locals.var_lntn_dn4)) + (p.p66 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p105));
        locals.var_udet_rv = 0.0;

        let assign770_e1051: f64 = (0.05 - locals.var_udet);
        let assign770_e1053: f64 = (assign770_e1051 / locals.var_vt);
        locals.var_dxa = assign770_e1053;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udet_dn4) * locals.var_vt) - (assign770_e1051 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign780_e1056: f64 = if 0.05 < locals.var_udet { 1.0 } else { 0.0 };
        locals.var_guard10 = assign780_e1056;
        locals.var_guard10_rv = 0.0;

        let (assign790_e1068, assign790_e1068_d_n0, assign790_e1068_d_n1, assign790_e1068_d_n3, assign790_e1068_d_n4, assign790_e1068_d_n5, assign790_e1068_d_n6, assign790_e1068_d_n7, assign790_e1068_d_n8, assign790_e1068_d_n9, assign790_e1068_d_n10, assign790_e1068_d_n11,) = {
    if (locals.var_guard10 != 0.0) {
        let assign790_e1062: f64 = (locals.var_dxa).exp();
        let assign790_e1063: f64 = (1.0 + assign790_e1062);
        let assign790_e1064: f64 = (assign790_e1063).ln();
        let assign790_e1065: f64 = (locals.var_vt * assign790_e1064);
        let assign790_e1066: f64 = (locals.var_udet + assign790_e1065);
        (assign790_e1066, (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn0) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn1) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn3) / assign790_e1063)), (locals.var_udet_dn4 + ((locals.var_vt_dn4 * assign790_e1064) + (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn4) / assign790_e1063)))), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn5) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn6) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn7) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn8) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn9) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn10) / assign790_e1063)), (locals.var_vt * ((assign790_e1062 * locals.var_dxa_dn11) / assign790_e1063)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9, locals.var_vde_t_dn10, locals.var_vde_t_dn11,)
    }
};
        locals.var_vde_t = assign790_e1068;
        locals.var_vde_t_dn0 = assign790_e1068_d_n0;
        locals.var_vde_t_dn1 = assign790_e1068_d_n1;
        locals.var_vde_t_dn3 = assign790_e1068_d_n3;
        locals.var_vde_t_dn4 = assign790_e1068_d_n4;
        locals.var_vde_t_dn5 = assign790_e1068_d_n5;
        locals.var_vde_t_dn6 = assign790_e1068_d_n6;
        locals.var_vde_t_dn7 = assign790_e1068_d_n7;
        locals.var_vde_t_dn8 = assign790_e1068_d_n8;
        locals.var_vde_t_dn9 = assign790_e1068_d_n9;
        locals.var_vde_t_dn10 = assign790_e1068_d_n10;
        locals.var_vde_t_dn11 = assign790_e1068_d_n11;
        locals.var_vde_t_rv = 0.0;

        let (assign800_e1082, assign800_e1082_d_n0, assign800_e1082_d_n1, assign800_e1082_d_n3, assign800_e1082_d_n4, assign800_e1082_d_n5, assign800_e1082_d_n6, assign800_e1082_d_n7, assign800_e1082_d_n8, assign800_e1082_d_n9, assign800_e1082_d_n10, assign800_e1082_d_n11,) = {
    if (locals.var_guard10 == 0.0) {
        let assign800_e1075: f64 = (-locals.var_dxa);
        let assign800_e1076: f64 = (assign800_e1075).exp();
        let assign800_e1077: f64 = (1.0 + assign800_e1076);
        let assign800_e1078: f64 = (assign800_e1077).ln();
        let assign800_e1079: f64 = (locals.var_vt * assign800_e1078);
        let assign800_e1080: f64 = (0.05 + assign800_e1079);
        (assign800_e1080, (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn0)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn1)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn3)) / assign800_e1077)), ((locals.var_vt_dn4 * assign800_e1078) + (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn4)) / assign800_e1077))), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn5)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn6)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn7)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn8)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn9)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn10)) / assign800_e1077)), (locals.var_vt * ((assign800_e1076 * (-locals.var_dxa_dn11)) / assign800_e1077)),)
    } else {
        (locals.var_vde_t, locals.var_vde_t_dn0, locals.var_vde_t_dn1, locals.var_vde_t_dn3, locals.var_vde_t_dn4, locals.var_vde_t_dn5, locals.var_vde_t_dn6, locals.var_vde_t_dn7, locals.var_vde_t_dn8, locals.var_vde_t_dn9, locals.var_vde_t_dn10, locals.var_vde_t_dn11,)
    }
};
        locals.var_vde_t = assign800_e1082;
        locals.var_vde_t_dn0 = assign800_e1082_d_n0;
        locals.var_vde_t_dn1 = assign800_e1082_d_n1;
        locals.var_vde_t_dn3 = assign800_e1082_d_n3;
        locals.var_vde_t_dn4 = assign800_e1082_d_n4;
        locals.var_vde_t_dn5 = assign800_e1082_d_n5;
        locals.var_vde_t_dn6 = assign800_e1082_d_n6;
        locals.var_vde_t_dn7 = assign800_e1082_d_n7;
        locals.var_vde_t_dn8 = assign800_e1082_d_n8;
        locals.var_vde_t_dn9 = assign800_e1082_d_n9;
        locals.var_vde_t_dn10 = assign800_e1082_d_n10;
        locals.var_vde_t_dn11 = assign800_e1082_d_n11;
        locals.var_vde_t_rv = 0.0;

        let assign810_e1084: f64 = (-3.0);
        let assign810_e1086: f64 = (assign810_e1084 * locals.var_vt);
        let assign810_e1088: f64 = (assign810_e1086 * locals.var_lntn);
        let assign810_e1091: f64 = (p.p64 * locals.var_tn);
        let assign810_e1092: f64 = (assign810_e1088 + assign810_e1091);
        let assign810_e1095: f64 = (1.0 - locals.var_tn);
        let assign810_e1097: f64 = (assign810_e1095 * p.p110);
        let assign810_e1098: f64 = (assign810_e1092 + assign810_e1097);
        locals.var_udct = assign810_e1098;
        locals.var_udct_dn4 = (((((assign810_e1084 * locals.var_vt_dn4) * locals.var_lntn) + (assign810_e1086 * locals.var_lntn_dn4)) + (p.p64 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p110));
        locals.var_udct_rv = 0.0;

        let assign820_e1101: f64 = (0.05 - locals.var_udct);
        let assign820_e1103: f64 = (assign820_e1101 / locals.var_vt);
        locals.var_dxa = assign820_e1103;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udct_dn4) * locals.var_vt) - (assign820_e1101 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign830_e1106: f64 = if 0.05 < locals.var_udct { 1.0 } else { 0.0 };
        locals.var_guard11 = assign830_e1106;
        locals.var_guard11_rv = 0.0;

        let (assign840_e1118, assign840_e1118_d_n0, assign840_e1118_d_n1, assign840_e1118_d_n3, assign840_e1118_d_n4, assign840_e1118_d_n5, assign840_e1118_d_n6, assign840_e1118_d_n7, assign840_e1118_d_n8, assign840_e1118_d_n9, assign840_e1118_d_n10, assign840_e1118_d_n11,) = {
    if (locals.var_guard11 != 0.0) {
        let assign840_e1112: f64 = (locals.var_dxa).exp();
        let assign840_e1113: f64 = (1.0 + assign840_e1112);
        let assign840_e1114: f64 = (assign840_e1113).ln();
        let assign840_e1115: f64 = (locals.var_vt * assign840_e1114);
        let assign840_e1116: f64 = (locals.var_udct + assign840_e1115);
        (assign840_e1116, (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn0) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn1) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn3) / assign840_e1113)), (locals.var_udct_dn4 + ((locals.var_vt_dn4 * assign840_e1114) + (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn4) / assign840_e1113)))), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn5) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn6) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn7) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn8) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn9) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn10) / assign840_e1113)), (locals.var_vt * ((assign840_e1112 * locals.var_dxa_dn11) / assign840_e1113)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9, locals.var_vdc_t_dn10, locals.var_vdc_t_dn11,)
    }
};
        locals.var_vdc_t = assign840_e1118;
        locals.var_vdc_t_dn0 = assign840_e1118_d_n0;
        locals.var_vdc_t_dn1 = assign840_e1118_d_n1;
        locals.var_vdc_t_dn3 = assign840_e1118_d_n3;
        locals.var_vdc_t_dn4 = assign840_e1118_d_n4;
        locals.var_vdc_t_dn5 = assign840_e1118_d_n5;
        locals.var_vdc_t_dn6 = assign840_e1118_d_n6;
        locals.var_vdc_t_dn7 = assign840_e1118_d_n7;
        locals.var_vdc_t_dn8 = assign840_e1118_d_n8;
        locals.var_vdc_t_dn9 = assign840_e1118_d_n9;
        locals.var_vdc_t_dn10 = assign840_e1118_d_n10;
        locals.var_vdc_t_dn11 = assign840_e1118_d_n11;
        locals.var_vdc_t_rv = 0.0;

        let (assign850_e1132, assign850_e1132_d_n0, assign850_e1132_d_n1, assign850_e1132_d_n3, assign850_e1132_d_n4, assign850_e1132_d_n5, assign850_e1132_d_n6, assign850_e1132_d_n7, assign850_e1132_d_n8, assign850_e1132_d_n9, assign850_e1132_d_n10, assign850_e1132_d_n11,) = {
    if (locals.var_guard11 == 0.0) {
        let assign850_e1125: f64 = (-locals.var_dxa);
        let assign850_e1126: f64 = (assign850_e1125).exp();
        let assign850_e1127: f64 = (1.0 + assign850_e1126);
        let assign850_e1128: f64 = (assign850_e1127).ln();
        let assign850_e1129: f64 = (locals.var_vt * assign850_e1128);
        let assign850_e1130: f64 = (0.05 + assign850_e1129);
        (assign850_e1130, (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn0)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn1)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn3)) / assign850_e1127)), ((locals.var_vt_dn4 * assign850_e1128) + (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn4)) / assign850_e1127))), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn5)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn6)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn7)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn8)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn9)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn10)) / assign850_e1127)), (locals.var_vt * ((assign850_e1126 * (-locals.var_dxa_dn11)) / assign850_e1127)),)
    } else {
        (locals.var_vdc_t, locals.var_vdc_t_dn0, locals.var_vdc_t_dn1, locals.var_vdc_t_dn3, locals.var_vdc_t_dn4, locals.var_vdc_t_dn5, locals.var_vdc_t_dn6, locals.var_vdc_t_dn7, locals.var_vdc_t_dn8, locals.var_vdc_t_dn9, locals.var_vdc_t_dn10, locals.var_vdc_t_dn11,)
    }
};
        locals.var_vdc_t = assign850_e1132;
        locals.var_vdc_t_dn0 = assign850_e1132_d_n0;
        locals.var_vdc_t_dn1 = assign850_e1132_d_n1;
        locals.var_vdc_t_dn3 = assign850_e1132_d_n3;
        locals.var_vdc_t_dn4 = assign850_e1132_d_n4;
        locals.var_vdc_t_dn5 = assign850_e1132_d_n5;
        locals.var_vdc_t_dn6 = assign850_e1132_d_n6;
        locals.var_vdc_t_dn7 = assign850_e1132_d_n7;
        locals.var_vdc_t_dn8 = assign850_e1132_d_n8;
        locals.var_vdc_t_dn9 = assign850_e1132_d_n9;
        locals.var_vdc_t_dn10 = assign850_e1132_d_n10;
        locals.var_vdc_t_dn11 = assign850_e1132_d_n11;
        locals.var_vdc_t_rv = 0.0;

        let assign860_e1134: f64 = (-3.0);
        let assign860_e1136: f64 = (assign860_e1134 * locals.var_vt);
        let assign860_e1138: f64 = (assign860_e1136 * locals.var_lntn);
        let assign860_e1141: f64 = (p.p80 * locals.var_tn);
        let assign860_e1142: f64 = (assign860_e1138 + assign860_e1141);
        let assign860_e1145: f64 = (1.0 - locals.var_tn);
        let assign860_e1147: f64 = (assign860_e1145 * p.p110);
        let assign860_e1148: f64 = (assign860_e1142 + assign860_e1147);
        locals.var_udcext = assign860_e1148;
        locals.var_udcext_dn4 = (((((assign860_e1134 * locals.var_vt_dn4) * locals.var_lntn) + (assign860_e1136 * locals.var_lntn_dn4)) + (p.p80 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p110));
        locals.var_udcext_rv = 0.0;

        let assign870_e1151: f64 = (0.05 - locals.var_udcext);
        let assign870_e1153: f64 = (assign870_e1151 / locals.var_vt);
        locals.var_dxa = assign870_e1153;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udcext_dn4) * locals.var_vt) - (assign870_e1151 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign880_e1156: f64 = if 0.05 < locals.var_udcext { 1.0 } else { 0.0 };
        locals.var_guard12 = assign880_e1156;
        locals.var_guard12_rv = 0.0;

        let (assign890_e1168, assign890_e1168_d_n0, assign890_e1168_d_n1, assign890_e1168_d_n3, assign890_e1168_d_n4, assign890_e1168_d_n5, assign890_e1168_d_n6, assign890_e1168_d_n7, assign890_e1168_d_n8, assign890_e1168_d_n9, assign890_e1168_d_n10, assign890_e1168_d_n11,) = {
    if (locals.var_guard12 != 0.0) {
        let assign890_e1162: f64 = (locals.var_dxa).exp();
        let assign890_e1163: f64 = (1.0 + assign890_e1162);
        let assign890_e1164: f64 = (assign890_e1163).ln();
        let assign890_e1165: f64 = (locals.var_vt * assign890_e1164);
        let assign890_e1166: f64 = (locals.var_udcext + assign890_e1165);
        (assign890_e1166, (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn0) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn1) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn3) / assign890_e1163)), (locals.var_udcext_dn4 + ((locals.var_vt_dn4 * assign890_e1164) + (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn4) / assign890_e1163)))), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn5) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn6) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn7) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn8) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn9) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn10) / assign890_e1163)), (locals.var_vt * ((assign890_e1162 * locals.var_dxa_dn11) / assign890_e1163)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9, locals.var_vdcex_t_dn10, locals.var_vdcex_t_dn11,)
    }
};
        locals.var_vdcex_t = assign890_e1168;
        locals.var_vdcex_t_dn0 = assign890_e1168_d_n0;
        locals.var_vdcex_t_dn1 = assign890_e1168_d_n1;
        locals.var_vdcex_t_dn3 = assign890_e1168_d_n3;
        locals.var_vdcex_t_dn4 = assign890_e1168_d_n4;
        locals.var_vdcex_t_dn5 = assign890_e1168_d_n5;
        locals.var_vdcex_t_dn6 = assign890_e1168_d_n6;
        locals.var_vdcex_t_dn7 = assign890_e1168_d_n7;
        locals.var_vdcex_t_dn8 = assign890_e1168_d_n8;
        locals.var_vdcex_t_dn9 = assign890_e1168_d_n9;
        locals.var_vdcex_t_dn10 = assign890_e1168_d_n10;
        locals.var_vdcex_t_dn11 = assign890_e1168_d_n11;
        locals.var_vdcex_t_rv = 0.0;

        let (assign900_e1182, assign900_e1182_d_n0, assign900_e1182_d_n1, assign900_e1182_d_n3, assign900_e1182_d_n4, assign900_e1182_d_n5, assign900_e1182_d_n6, assign900_e1182_d_n7, assign900_e1182_d_n8, assign900_e1182_d_n9, assign900_e1182_d_n10, assign900_e1182_d_n11,) = {
    if (locals.var_guard12 == 0.0) {
        let assign900_e1175: f64 = (-locals.var_dxa);
        let assign900_e1176: f64 = (assign900_e1175).exp();
        let assign900_e1177: f64 = (1.0 + assign900_e1176);
        let assign900_e1178: f64 = (assign900_e1177).ln();
        let assign900_e1179: f64 = (locals.var_vt * assign900_e1178);
        let assign900_e1180: f64 = (0.05 + assign900_e1179);
        (assign900_e1180, (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn0)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn1)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn3)) / assign900_e1177)), ((locals.var_vt_dn4 * assign900_e1178) + (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn4)) / assign900_e1177))), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn5)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn6)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn7)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn8)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn9)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn10)) / assign900_e1177)), (locals.var_vt * ((assign900_e1176 * (-locals.var_dxa_dn11)) / assign900_e1177)),)
    } else {
        (locals.var_vdcex_t, locals.var_vdcex_t_dn0, locals.var_vdcex_t_dn1, locals.var_vdcex_t_dn3, locals.var_vdcex_t_dn4, locals.var_vdcex_t_dn5, locals.var_vdcex_t_dn6, locals.var_vdcex_t_dn7, locals.var_vdcex_t_dn8, locals.var_vdcex_t_dn9, locals.var_vdcex_t_dn10, locals.var_vdcex_t_dn11,)
    }
};
        locals.var_vdcex_t = assign900_e1182;
        locals.var_vdcex_t_dn0 = assign900_e1182_d_n0;
        locals.var_vdcex_t_dn1 = assign900_e1182_d_n1;
        locals.var_vdcex_t_dn3 = assign900_e1182_d_n3;
        locals.var_vdcex_t_dn4 = assign900_e1182_d_n4;
        locals.var_vdcex_t_dn5 = assign900_e1182_d_n5;
        locals.var_vdcex_t_dn6 = assign900_e1182_d_n6;
        locals.var_vdcex_t_dn7 = assign900_e1182_d_n7;
        locals.var_vdcex_t_dn8 = assign900_e1182_d_n8;
        locals.var_vdcex_t_dn9 = assign900_e1182_d_n9;
        locals.var_vdcex_t_dn10 = assign900_e1182_d_n10;
        locals.var_vdcex_t_dn11 = assign900_e1182_d_n11;
        locals.var_vdcex_t_rv = 0.0;

        let assign910_e1184: f64 = (-3.0);
        let assign910_e1186: f64 = (assign910_e1184 * locals.var_vt);
        let assign910_e1188: f64 = (assign910_e1186 * locals.var_lntn);
        let assign910_e1191: f64 = (p.p71 * locals.var_tn);
        let assign910_e1192: f64 = (assign910_e1188 + assign910_e1191);
        let assign910_e1195: f64 = (1.0 - locals.var_tn);
        let assign910_e1197: f64 = (assign910_e1195 * p.p110);
        let assign910_e1198: f64 = (assign910_e1192 + assign910_e1197);
        locals.var_udct_ctc = assign910_e1198;
        locals.var_udct_ctc_dn4 = (((((assign910_e1184 * locals.var_vt_dn4) * locals.var_lntn) + (assign910_e1186 * locals.var_lntn_dn4)) + (p.p71 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p110));
        locals.var_udct_ctc_rv = 0.0;

        let assign920_e1201: f64 = (0.05 - locals.var_udct_ctc);
        let assign920_e1203: f64 = (assign920_e1201 / locals.var_vt);
        locals.var_dxa = assign920_e1203;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udct_ctc_dn4) * locals.var_vt) - (assign920_e1201 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign930_e1206: f64 = if 0.05 < locals.var_udct_ctc { 1.0 } else { 0.0 };
        locals.var_guard13 = assign930_e1206;
        locals.var_guard13_rv = 0.0;

        let (assign940_e1218, assign940_e1218_d_n0, assign940_e1218_d_n1, assign940_e1218_d_n3, assign940_e1218_d_n4, assign940_e1218_d_n5, assign940_e1218_d_n6, assign940_e1218_d_n7, assign940_e1218_d_n8, assign940_e1218_d_n9, assign940_e1218_d_n10, assign940_e1218_d_n11,) = {
    if (locals.var_guard13 != 0.0) {
        let assign940_e1212: f64 = (locals.var_dxa).exp();
        let assign940_e1213: f64 = (1.0 + assign940_e1212);
        let assign940_e1214: f64 = (assign940_e1213).ln();
        let assign940_e1215: f64 = (locals.var_vt * assign940_e1214);
        let assign940_e1216: f64 = (locals.var_udct_ctc + assign940_e1215);
        (assign940_e1216, (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn0) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn1) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn3) / assign940_e1213)), (locals.var_udct_ctc_dn4 + ((locals.var_vt_dn4 * assign940_e1214) + (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn4) / assign940_e1213)))), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn5) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn6) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn7) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn8) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn9) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn10) / assign940_e1213)), (locals.var_vt * ((assign940_e1212 * locals.var_dxa_dn11) / assign940_e1213)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9, locals.var_vdc_ctc_t_dn10, locals.var_vdc_ctc_t_dn11,)
    }
};
        locals.var_vdc_ctc_t = assign940_e1218;
        locals.var_vdc_ctc_t_dn0 = assign940_e1218_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign940_e1218_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign940_e1218_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign940_e1218_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign940_e1218_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign940_e1218_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign940_e1218_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign940_e1218_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign940_e1218_d_n9;
        locals.var_vdc_ctc_t_dn10 = assign940_e1218_d_n10;
        locals.var_vdc_ctc_t_dn11 = assign940_e1218_d_n11;
        locals.var_vdc_ctc_t_rv = 0.0;

        let (assign950_e1232, assign950_e1232_d_n0, assign950_e1232_d_n1, assign950_e1232_d_n3, assign950_e1232_d_n4, assign950_e1232_d_n5, assign950_e1232_d_n6, assign950_e1232_d_n7, assign950_e1232_d_n8, assign950_e1232_d_n9, assign950_e1232_d_n10, assign950_e1232_d_n11,) = {
    if (locals.var_guard13 == 0.0) {
        let assign950_e1225: f64 = (-locals.var_dxa);
        let assign950_e1226: f64 = (assign950_e1225).exp();
        let assign950_e1227: f64 = (1.0 + assign950_e1226);
        let assign950_e1228: f64 = (assign950_e1227).ln();
        let assign950_e1229: f64 = (locals.var_vt * assign950_e1228);
        let assign950_e1230: f64 = (0.05 + assign950_e1229);
        (assign950_e1230, (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn0)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn1)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn3)) / assign950_e1227)), ((locals.var_vt_dn4 * assign950_e1228) + (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn4)) / assign950_e1227))), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn5)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn6)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn7)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn8)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn9)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn10)) / assign950_e1227)), (locals.var_vt * ((assign950_e1226 * (-locals.var_dxa_dn11)) / assign950_e1227)),)
    } else {
        (locals.var_vdc_ctc_t, locals.var_vdc_ctc_t_dn0, locals.var_vdc_ctc_t_dn1, locals.var_vdc_ctc_t_dn3, locals.var_vdc_ctc_t_dn4, locals.var_vdc_ctc_t_dn5, locals.var_vdc_ctc_t_dn6, locals.var_vdc_ctc_t_dn7, locals.var_vdc_ctc_t_dn8, locals.var_vdc_ctc_t_dn9, locals.var_vdc_ctc_t_dn10, locals.var_vdc_ctc_t_dn11,)
    }
};
        locals.var_vdc_ctc_t = assign950_e1232;
        locals.var_vdc_ctc_t_dn0 = assign950_e1232_d_n0;
        locals.var_vdc_ctc_t_dn1 = assign950_e1232_d_n1;
        locals.var_vdc_ctc_t_dn3 = assign950_e1232_d_n3;
        locals.var_vdc_ctc_t_dn4 = assign950_e1232_d_n4;
        locals.var_vdc_ctc_t_dn5 = assign950_e1232_d_n5;
        locals.var_vdc_ctc_t_dn6 = assign950_e1232_d_n6;
        locals.var_vdc_ctc_t_dn7 = assign950_e1232_d_n7;
        locals.var_vdc_ctc_t_dn8 = assign950_e1232_d_n8;
        locals.var_vdc_ctc_t_dn9 = assign950_e1232_d_n9;
        locals.var_vdc_ctc_t_dn10 = assign950_e1232_d_n10;
        locals.var_vdc_ctc_t_dn11 = assign950_e1232_d_n11;
        locals.var_vdc_ctc_t_rv = 0.0;

        let assign960_e1234: f64 = (-3.0);
        let assign960_e1236: f64 = (assign960_e1234 * locals.var_vt);
        let assign960_e1238: f64 = (assign960_e1236 * locals.var_lntn);
        let assign960_e1241: f64 = (locals.var_vdc_zener * locals.var_tn);
        let assign960_e1242: f64 = (assign960_e1238 + assign960_e1241);
        let assign960_e1245: f64 = (1.0 - locals.var_tn);
        let assign960_e1247: f64 = (assign960_e1245 * p.p110);
        let assign960_e1248: f64 = (assign960_e1242 + assign960_e1247);
        locals.var_udct_zener = assign960_e1248;
        locals.var_udct_zener_dn4 = (((((assign960_e1234 * locals.var_vt_dn4) * locals.var_lntn) + (assign960_e1236 * locals.var_lntn_dn4)) + (locals.var_vdc_zener * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p110));
        locals.var_udct_zener_rv = 0.0;

        let assign970_e1251: f64 = (0.05 - locals.var_udct_zener);
        let assign970_e1253: f64 = (assign970_e1251 / locals.var_vt);
        locals.var_dxa = assign970_e1253;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udct_zener_dn4) * locals.var_vt) - (assign970_e1251 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign980_e1256: f64 = if 0.05 < locals.var_udct_zener { 1.0 } else { 0.0 };
        locals.var_guard14 = assign980_e1256;
        locals.var_guard14_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign990_e1268, assign990_e1268_d_n0, assign990_e1268_d_n1, assign990_e1268_d_n3, assign990_e1268_d_n4, assign990_e1268_d_n5, assign990_e1268_d_n6, assign990_e1268_d_n7, assign990_e1268_d_n8, assign990_e1268_d_n9, assign990_e1268_d_n10, assign990_e1268_d_n11,) = {
    if (locals.var_guard14 != 0.0) {
        let assign990_e1262: f64 = (locals.var_dxa).exp();
        let assign990_e1263: f64 = (1.0 + assign990_e1262);
        let assign990_e1264: f64 = (assign990_e1263).ln();
        let assign990_e1265: f64 = (locals.var_vt * assign990_e1264);
        let assign990_e1266: f64 = (locals.var_udct_zener + assign990_e1265);
        (assign990_e1266, (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn0) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn1) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn3) / assign990_e1263)), (locals.var_udct_zener_dn4 + ((locals.var_vt_dn4 * assign990_e1264) + (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn4) / assign990_e1263)))), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn5) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn6) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn7) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn8) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn9) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn10) / assign990_e1263)), (locals.var_vt * ((assign990_e1262 * locals.var_dxa_dn11) / assign990_e1263)),)
    } else {
        (locals.var_vdc_zener_t, locals.var_vdc_zener_t_dn0, locals.var_vdc_zener_t_dn1, locals.var_vdc_zener_t_dn3, locals.var_vdc_zener_t_dn4, locals.var_vdc_zener_t_dn5, locals.var_vdc_zener_t_dn6, locals.var_vdc_zener_t_dn7, locals.var_vdc_zener_t_dn8, locals.var_vdc_zener_t_dn9, locals.var_vdc_zener_t_dn10, locals.var_vdc_zener_t_dn11,)
    }
};
        locals.var_vdc_zener_t = assign990_e1268;
        locals.var_vdc_zener_t_dn0 = assign990_e1268_d_n0;
        locals.var_vdc_zener_t_dn1 = assign990_e1268_d_n1;
        locals.var_vdc_zener_t_dn3 = assign990_e1268_d_n3;
        locals.var_vdc_zener_t_dn4 = assign990_e1268_d_n4;
        locals.var_vdc_zener_t_dn5 = assign990_e1268_d_n5;
        locals.var_vdc_zener_t_dn6 = assign990_e1268_d_n6;
        locals.var_vdc_zener_t_dn7 = assign990_e1268_d_n7;
        locals.var_vdc_zener_t_dn8 = assign990_e1268_d_n8;
        locals.var_vdc_zener_t_dn9 = assign990_e1268_d_n9;
        locals.var_vdc_zener_t_dn10 = assign990_e1268_d_n10;
        locals.var_vdc_zener_t_dn11 = assign990_e1268_d_n11;
        locals.var_vdc_zener_t_rv = 0.0;

        let (assign1000_e1282, assign1000_e1282_d_n0, assign1000_e1282_d_n1, assign1000_e1282_d_n3, assign1000_e1282_d_n4, assign1000_e1282_d_n5, assign1000_e1282_d_n6, assign1000_e1282_d_n7, assign1000_e1282_d_n8, assign1000_e1282_d_n9, assign1000_e1282_d_n10, assign1000_e1282_d_n11,) = {
    if (locals.var_guard14 == 0.0) {
        let assign1000_e1275: f64 = (-locals.var_dxa);
        let assign1000_e1276: f64 = (assign1000_e1275).exp();
        let assign1000_e1277: f64 = (1.0 + assign1000_e1276);
        let assign1000_e1278: f64 = (assign1000_e1277).ln();
        let assign1000_e1279: f64 = (locals.var_vt * assign1000_e1278);
        let assign1000_e1280: f64 = (0.05 + assign1000_e1279);
        (assign1000_e1280, (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn0)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn1)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn3)) / assign1000_e1277)), ((locals.var_vt_dn4 * assign1000_e1278) + (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn4)) / assign1000_e1277))), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn5)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn6)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn7)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn8)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn9)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn10)) / assign1000_e1277)), (locals.var_vt * ((assign1000_e1276 * (-locals.var_dxa_dn11)) / assign1000_e1277)),)
    } else {
        (locals.var_vdc_zener_t, locals.var_vdc_zener_t_dn0, locals.var_vdc_zener_t_dn1, locals.var_vdc_zener_t_dn3, locals.var_vdc_zener_t_dn4, locals.var_vdc_zener_t_dn5, locals.var_vdc_zener_t_dn6, locals.var_vdc_zener_t_dn7, locals.var_vdc_zener_t_dn8, locals.var_vdc_zener_t_dn9, locals.var_vdc_zener_t_dn10, locals.var_vdc_zener_t_dn11,)
    }
};
        locals.var_vdc_zener_t = assign1000_e1282;
        locals.var_vdc_zener_t_dn0 = assign1000_e1282_d_n0;
        locals.var_vdc_zener_t_dn1 = assign1000_e1282_d_n1;
        locals.var_vdc_zener_t_dn3 = assign1000_e1282_d_n3;
        locals.var_vdc_zener_t_dn4 = assign1000_e1282_d_n4;
        locals.var_vdc_zener_t_dn5 = assign1000_e1282_d_n5;
        locals.var_vdc_zener_t_dn6 = assign1000_e1282_d_n6;
        locals.var_vdc_zener_t_dn7 = assign1000_e1282_d_n7;
        locals.var_vdc_zener_t_dn8 = assign1000_e1282_d_n8;
        locals.var_vdc_zener_t_dn9 = assign1000_e1282_d_n9;
        locals.var_vdc_zener_t_dn10 = assign1000_e1282_d_n10;
        locals.var_vdc_zener_t_dn11 = assign1000_e1282_d_n11;
        locals.var_vdc_zener_t_rv = 0.0;

        let assign1010_e1284: f64 = (-3.0);
        let assign1010_e1286: f64 = (assign1010_e1284 * locals.var_vt);
        let assign1010_e1288: f64 = (assign1010_e1286 * locals.var_lntn);
        let assign1010_e1291: f64 = (p.p27 * locals.var_tn);
        let assign1010_e1292: f64 = (assign1010_e1288 + assign1010_e1291);
        let assign1010_e1295: f64 = (1.0 - locals.var_tn);
        let assign1010_e1297: f64 = (assign1010_e1295 * p.p109);
        let assign1010_e1298: f64 = (assign1010_e1292 + assign1010_e1297);
        locals.var_uknbrt = assign1010_e1298;
        locals.var_uknbrt_dn4 = (((((assign1010_e1284 * locals.var_vt_dn4) * locals.var_lntn) + (assign1010_e1286 * locals.var_lntn_dn4)) + (p.p27 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p109));
        locals.var_uknbrt_rv = 0.0;

        let assign1020_e1301: f64 = (0.05 - locals.var_uknbrt);
        let assign1020_e1303: f64 = (assign1020_e1301 / locals.var_vt);
        locals.var_dxa = assign1020_e1303;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_uknbrt_dn4) * locals.var_vt) - (assign1020_e1301 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign1030_e1306: f64 = if 0.05 < locals.var_uknbrt { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1030_e1306;
        locals.var_guard15_rv = 0.0;

        let (assign1040_e1318, assign1040_e1318_d_n0, assign1040_e1318_d_n1, assign1040_e1318_d_n3, assign1040_e1318_d_n4, assign1040_e1318_d_n5, assign1040_e1318_d_n6, assign1040_e1318_d_n7, assign1040_e1318_d_n8, assign1040_e1318_d_n9, assign1040_e1318_d_n10, assign1040_e1318_d_n11,) = {
    if (locals.var_guard15 != 0.0) {
        let assign1040_e1312: f64 = (locals.var_dxa).exp();
        let assign1040_e1313: f64 = (1.0 + assign1040_e1312);
        let assign1040_e1314: f64 = (assign1040_e1313).ln();
        let assign1040_e1315: f64 = (locals.var_vt * assign1040_e1314);
        let assign1040_e1316: f64 = (locals.var_uknbrt + assign1040_e1315);
        (assign1040_e1316, (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn0) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn1) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn3) / assign1040_e1313)), (locals.var_uknbrt_dn4 + ((locals.var_vt_dn4 * assign1040_e1314) + (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn4) / assign1040_e1313)))), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn5) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn6) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn7) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn8) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn9) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn10) / assign1040_e1313)), (locals.var_vt * ((assign1040_e1312 * locals.var_dxa_dn11) / assign1040_e1313)),)
    } else {
        (locals.var_vknbr_t, locals.var_vknbr_t_dn0, locals.var_vknbr_t_dn1, locals.var_vknbr_t_dn3, locals.var_vknbr_t_dn4, locals.var_vknbr_t_dn5, locals.var_vknbr_t_dn6, locals.var_vknbr_t_dn7, locals.var_vknbr_t_dn8, locals.var_vknbr_t_dn9, locals.var_vknbr_t_dn10, locals.var_vknbr_t_dn11,)
    }
};
        locals.var_vknbr_t = assign1040_e1318;
        locals.var_vknbr_t_dn0 = assign1040_e1318_d_n0;
        locals.var_vknbr_t_dn1 = assign1040_e1318_d_n1;
        locals.var_vknbr_t_dn3 = assign1040_e1318_d_n3;
        locals.var_vknbr_t_dn4 = assign1040_e1318_d_n4;
        locals.var_vknbr_t_dn5 = assign1040_e1318_d_n5;
        locals.var_vknbr_t_dn6 = assign1040_e1318_d_n6;
        locals.var_vknbr_t_dn7 = assign1040_e1318_d_n7;
        locals.var_vknbr_t_dn8 = assign1040_e1318_d_n8;
        locals.var_vknbr_t_dn9 = assign1040_e1318_d_n9;
        locals.var_vknbr_t_dn10 = assign1040_e1318_d_n10;
        locals.var_vknbr_t_dn11 = assign1040_e1318_d_n11;
        locals.var_vknbr_t_rv = 0.0;

        let (assign1050_e1332, assign1050_e1332_d_n0, assign1050_e1332_d_n1, assign1050_e1332_d_n3, assign1050_e1332_d_n4, assign1050_e1332_d_n5, assign1050_e1332_d_n6, assign1050_e1332_d_n7, assign1050_e1332_d_n8, assign1050_e1332_d_n9, assign1050_e1332_d_n10, assign1050_e1332_d_n11,) = {
    if (locals.var_guard15 == 0.0) {
        let assign1050_e1325: f64 = (-locals.var_dxa);
        let assign1050_e1326: f64 = (assign1050_e1325).exp();
        let assign1050_e1327: f64 = (1.0 + assign1050_e1326);
        let assign1050_e1328: f64 = (assign1050_e1327).ln();
        let assign1050_e1329: f64 = (locals.var_vt * assign1050_e1328);
        let assign1050_e1330: f64 = (0.05 + assign1050_e1329);
        (assign1050_e1330, (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn0)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn1)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn3)) / assign1050_e1327)), ((locals.var_vt_dn4 * assign1050_e1328) + (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn4)) / assign1050_e1327))), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn5)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn6)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn7)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn8)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn9)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn10)) / assign1050_e1327)), (locals.var_vt * ((assign1050_e1326 * (-locals.var_dxa_dn11)) / assign1050_e1327)),)
    } else {
        (locals.var_vknbr_t, locals.var_vknbr_t_dn0, locals.var_vknbr_t_dn1, locals.var_vknbr_t_dn3, locals.var_vknbr_t_dn4, locals.var_vknbr_t_dn5, locals.var_vknbr_t_dn6, locals.var_vknbr_t_dn7, locals.var_vknbr_t_dn8, locals.var_vknbr_t_dn9, locals.var_vknbr_t_dn10, locals.var_vknbr_t_dn11,)
    }
};
        locals.var_vknbr_t = assign1050_e1332;
        locals.var_vknbr_t_dn0 = assign1050_e1332_d_n0;
        locals.var_vknbr_t_dn1 = assign1050_e1332_d_n1;
        locals.var_vknbr_t_dn3 = assign1050_e1332_d_n3;
        locals.var_vknbr_t_dn4 = assign1050_e1332_d_n4;
        locals.var_vknbr_t_dn5 = assign1050_e1332_d_n5;
        locals.var_vknbr_t_dn6 = assign1050_e1332_d_n6;
        locals.var_vknbr_t_dn7 = assign1050_e1332_d_n7;
        locals.var_vknbr_t_dn8 = assign1050_e1332_d_n8;
        locals.var_vknbr_t_dn9 = assign1050_e1332_d_n9;
        locals.var_vknbr_t_dn10 = assign1050_e1332_d_n10;
        locals.var_vknbr_t_dn11 = assign1050_e1332_d_n11;
        locals.var_vknbr_t_rv = 0.0;

        let assign1060_e1334: f64 = (-3.0);
        let assign1060_e1336: f64 = (assign1060_e1334 * locals.var_vt);
        let assign1060_e1338: f64 = (assign1060_e1336 * locals.var_lntn);
        let assign1060_e1341: f64 = (p.p138 * locals.var_tn);
        let assign1060_e1342: f64 = (assign1060_e1338 + assign1060_e1341);
        let assign1060_e1345: f64 = (1.0 - locals.var_tn);
        let assign1060_e1347: f64 = (assign1060_e1345 * p.p140);
        let assign1060_e1348: f64 = (assign1060_e1342 + assign1060_e1347);
        locals.var_udst = assign1060_e1348;
        locals.var_udst_dn4 = (((((assign1060_e1334 * locals.var_vt_dn4) * locals.var_lntn) + (assign1060_e1336 * locals.var_lntn_dn4)) + (p.p138 * locals.var_tn_dn4)) + ((-locals.var_tn_dn4) * p.p140));
        locals.var_udst_rv = 0.0;

        let assign1070_e1351: f64 = (0.05 - locals.var_udst);
        let assign1070_e1353: f64 = (assign1070_e1351 / locals.var_vt);
        locals.var_dxa = assign1070_e1353;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = ((((-locals.var_udst_dn4) * locals.var_vt) - (assign1070_e1351 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt));
        locals.var_dxa_dn5 = 0.0;
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = 0.0;
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign1080_e1356: f64 = if 0.05 < locals.var_udst { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1080_e1356;
        locals.var_guard16_rv = 0.0;

        let (assign1090_e1368, assign1090_e1368_d_n0, assign1090_e1368_d_n1, assign1090_e1368_d_n3, assign1090_e1368_d_n4, assign1090_e1368_d_n5, assign1090_e1368_d_n6, assign1090_e1368_d_n7, assign1090_e1368_d_n8, assign1090_e1368_d_n9, assign1090_e1368_d_n10, assign1090_e1368_d_n11,) = {
    if (locals.var_guard16 != 0.0) {
        let assign1090_e1362: f64 = (locals.var_dxa).exp();
        let assign1090_e1363: f64 = (1.0 + assign1090_e1362);
        let assign1090_e1364: f64 = (assign1090_e1363).ln();
        let assign1090_e1365: f64 = (locals.var_vt * assign1090_e1364);
        let assign1090_e1366: f64 = (locals.var_udst + assign1090_e1365);
        (assign1090_e1366, (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn0) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn1) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn3) / assign1090_e1363)), (locals.var_udst_dn4 + ((locals.var_vt_dn4 * assign1090_e1364) + (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn4) / assign1090_e1363)))), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn5) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn6) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn7) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn8) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn9) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn10) / assign1090_e1363)), (locals.var_vt * ((assign1090_e1362 * locals.var_dxa_dn11) / assign1090_e1363)),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn0, locals.var_vds_t_dn1, locals.var_vds_t_dn3, locals.var_vds_t_dn4, locals.var_vds_t_dn5, locals.var_vds_t_dn6, locals.var_vds_t_dn7, locals.var_vds_t_dn8, locals.var_vds_t_dn9, locals.var_vds_t_dn10, locals.var_vds_t_dn11,)
    }
};
        locals.var_vds_t = assign1090_e1368;
        locals.var_vds_t_dn0 = assign1090_e1368_d_n0;
        locals.var_vds_t_dn1 = assign1090_e1368_d_n1;
        locals.var_vds_t_dn3 = assign1090_e1368_d_n3;
        locals.var_vds_t_dn4 = assign1090_e1368_d_n4;
        locals.var_vds_t_dn5 = assign1090_e1368_d_n5;
        locals.var_vds_t_dn6 = assign1090_e1368_d_n6;
        locals.var_vds_t_dn7 = assign1090_e1368_d_n7;
        locals.var_vds_t_dn8 = assign1090_e1368_d_n8;
        locals.var_vds_t_dn9 = assign1090_e1368_d_n9;
        locals.var_vds_t_dn10 = assign1090_e1368_d_n10;
        locals.var_vds_t_dn11 = assign1090_e1368_d_n11;
        locals.var_vds_t_rv = 0.0;

        let (assign1100_e1382, assign1100_e1382_d_n0, assign1100_e1382_d_n1, assign1100_e1382_d_n3, assign1100_e1382_d_n4, assign1100_e1382_d_n5, assign1100_e1382_d_n6, assign1100_e1382_d_n7, assign1100_e1382_d_n8, assign1100_e1382_d_n9, assign1100_e1382_d_n10, assign1100_e1382_d_n11,) = {
    if (locals.var_guard16 == 0.0) {
        let assign1100_e1375: f64 = (-locals.var_dxa);
        let assign1100_e1376: f64 = (assign1100_e1375).exp();
        let assign1100_e1377: f64 = (1.0 + assign1100_e1376);
        let assign1100_e1378: f64 = (assign1100_e1377).ln();
        let assign1100_e1379: f64 = (locals.var_vt * assign1100_e1378);
        let assign1100_e1380: f64 = (0.05 + assign1100_e1379);
        (assign1100_e1380, (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn0)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn1)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn3)) / assign1100_e1377)), ((locals.var_vt_dn4 * assign1100_e1378) + (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn4)) / assign1100_e1377))), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn5)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn6)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn7)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn8)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn9)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn10)) / assign1100_e1377)), (locals.var_vt * ((assign1100_e1376 * (-locals.var_dxa_dn11)) / assign1100_e1377)),)
    } else {
        (locals.var_vds_t, locals.var_vds_t_dn0, locals.var_vds_t_dn1, locals.var_vds_t_dn3, locals.var_vds_t_dn4, locals.var_vds_t_dn5, locals.var_vds_t_dn6, locals.var_vds_t_dn7, locals.var_vds_t_dn8, locals.var_vds_t_dn9, locals.var_vds_t_dn10, locals.var_vds_t_dn11,)
    }
};
        locals.var_vds_t = assign1100_e1382;
        locals.var_vds_t_dn0 = assign1100_e1382_d_n0;
        locals.var_vds_t_dn1 = assign1100_e1382_d_n1;
        locals.var_vds_t_dn3 = assign1100_e1382_d_n3;
        locals.var_vds_t_dn4 = assign1100_e1382_d_n4;
        locals.var_vds_t_dn5 = assign1100_e1382_d_n5;
        locals.var_vds_t_dn6 = assign1100_e1382_d_n6;
        locals.var_vds_t_dn7 = assign1100_e1382_d_n7;
        locals.var_vds_t_dn8 = assign1100_e1382_d_n8;
        locals.var_vds_t_dn9 = assign1100_e1382_d_n9;
        locals.var_vds_t_dn10 = assign1100_e1382_d_n10;
        locals.var_vds_t_dn11 = assign1100_e1382_d_n11;
        locals.var_vds_t_rv = 0.0;

        let assign1110_e1385: f64 = (1.0 / locals.var_vde_t);
        locals.var_inv_vde_t = assign1110_e1385;
        locals.var_inv_vde_t_dn0 = (-(locals.var_vde_t_dn0 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn1 = (-(locals.var_vde_t_dn1 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn3 = (-(locals.var_vde_t_dn3 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn4 = (-(locals.var_vde_t_dn4 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn5 = (-(locals.var_vde_t_dn5 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn6 = (-(locals.var_vde_t_dn6 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn7 = (-(locals.var_vde_t_dn7 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn8 = (-(locals.var_vde_t_dn8 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn9 = (-(locals.var_vde_t_dn9 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn10 = (-(locals.var_vde_t_dn10 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_dn11 = (-(locals.var_vde_t_dn11 / (locals.var_vde_t * locals.var_vde_t)));
        locals.var_inv_vde_t_rv = 0.0;

        let assign1120_e1388: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign1120_e1388;
        locals.var_inv_vdc_zener_t_dn0 = (-(locals.var_vdc_zener_t_dn0 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn1 = (-(locals.var_vdc_zener_t_dn1 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn3 = (-(locals.var_vdc_zener_t_dn3 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn4 = (-(locals.var_vdc_zener_t_dn4 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn5 = (-(locals.var_vdc_zener_t_dn5 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn6 = (-(locals.var_vdc_zener_t_dn6 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn7 = (-(locals.var_vdc_zener_t_dn7 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn8 = (-(locals.var_vdc_zener_t_dn8 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn9 = (-(locals.var_vdc_zener_t_dn9 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn10 = (-(locals.var_vdc_zener_t_dn10 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn11 = (-(locals.var_vdc_zener_t_dn11 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_rv = 0.0;

        let assign1130_e1391: f64 = (p.p66 * locals.var_inv_vde_t);
        let assign1130_e1393: f64 = (assign1130_e1391).powf(p.p67);
        locals.var_cje_t_div_cje = assign1130_e1393;
        locals.var_cje_t_div_cje_dn0 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn0))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn0) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn1 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn1))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn1) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn3 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn3))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn3) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn4 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn4))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn4) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn5 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn5))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn5) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn6 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn6))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn6) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn7 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn7))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn7) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn8 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn8))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn8) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn9 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn9))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn9) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn10 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn10))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn10) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_dn11 = if 0.0 == 0.0 && ((p.p67) as f64).is_finite() && ((p.p67) as f64).fract() == 0.0 { if p.p67 == 0.0 { 0.0 } else { (p.p67 * ((assign1130_e1391).powf(p.p67 - 1.0) * (p.p66 * locals.var_inv_vde_t_dn11))) } } else { (assign1130_e1393 * (p.p67 * ((p.p66 * locals.var_inv_vde_t_dn11) / assign1130_e1391))) };
        locals.var_cje_t_div_cje_rv = 0.0;

        let assign1140_e1396: f64 = (locals.var_vdc_zener * locals.var_inv_vdc_zener_t);
        let assign1140_e1398: f64 = (assign1140_e1396).powf(locals.var_pc_zener);
        locals.var_cjc_t_div_cjc_zener = assign1140_e1398;
        locals.var_cjc_t_div_cjc_zener_dn0 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn0))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn0) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn1 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn1))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn1) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn3 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn3))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn3) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn4 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn4))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn4) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn5 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn5))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn5) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn6 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn6))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn6) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn7 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn7))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn7) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn8 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn8))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn8) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn9 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn9))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn9) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn10 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn10))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn10) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_dn11 = if 0.0 == 0.0 && ((locals.var_pc_zener) as f64).is_finite() && ((locals.var_pc_zener) as f64).fract() == 0.0 { if locals.var_pc_zener == 0.0 { 0.0 } else { (locals.var_pc_zener * ((assign1140_e1396).powf(locals.var_pc_zener - 1.0) * (locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn11))) } } else { (assign1140_e1398 * (locals.var_pc_zener * ((locals.var_vdc_zener * locals.var_inv_vdc_zener_t_dn11) / assign1140_e1396))) };
        locals.var_cjc_t_div_cjc_zener_rv = 0.0;

        let assign1150_e1401: f64 = (p.p65 * locals.var_cje_t_div_cje);
        locals.var_cje_t = assign1150_e1401;
        locals.var_cje_t_dn0 = (p.p65 * locals.var_cje_t_div_cje_dn0);
        locals.var_cje_t_dn1 = (p.p65 * locals.var_cje_t_div_cje_dn1);
        locals.var_cje_t_dn3 = (p.p65 * locals.var_cje_t_div_cje_dn3);
        locals.var_cje_t_dn4 = (p.p65 * locals.var_cje_t_div_cje_dn4);
        locals.var_cje_t_dn5 = (p.p65 * locals.var_cje_t_div_cje_dn5);
        locals.var_cje_t_dn6 = (p.p65 * locals.var_cje_t_div_cje_dn6);
        locals.var_cje_t_dn7 = (p.p65 * locals.var_cje_t_div_cje_dn7);
        locals.var_cje_t_dn8 = (p.p65 * locals.var_cje_t_div_cje_dn8);
        locals.var_cje_t_dn9 = (p.p65 * locals.var_cje_t_div_cje_dn9);
        locals.var_cje_t_dn10 = (p.p65 * locals.var_cje_t_div_cje_dn10);
        locals.var_cje_t_dn11 = (p.p65 * locals.var_cje_t_div_cje_dn11);
        locals.var_cje_t_rv = 0.0;

        let assign1160_e1405: f64 = (p.p138 / locals.var_vds_t);
        let assign1160_e1407: f64 = (assign1160_e1405).powf(p.p139);
        let assign1160_e1408: f64 = (p.p137 * assign1160_e1407);
        locals.var_cjs_t = assign1160_e1408;
        locals.var_cjs_t_dn0 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn0) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn0) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn1 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn1) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn1) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn3 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn3) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn3) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn4 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn5 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn5) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn5) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn6 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn6) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn6) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn7 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn7) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn7) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn8 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn8) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn8) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn9 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn9) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn9) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn10 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn10) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn10) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_dn11 = (p.p137 * if 0.0 == 0.0 && ((p.p139) as f64).is_finite() && ((p.p139) as f64).fract() == 0.0 { if p.p139 == 0.0 { 0.0 } else { (p.p139 * ((assign1160_e1405).powf(p.p139 - 1.0) * (-((p.p138 * locals.var_vds_t_dn11) / (locals.var_vds_t * locals.var_vds_t))))) } } else { (assign1160_e1407 * (p.p139 * ((-((p.p138 * locals.var_vds_t_dn11) / (locals.var_vds_t * locals.var_vds_t))) / assign1160_e1405))) });
        locals.var_cjs_t_rv = 0.0;

        let assign1170_e1411: f64 = (1.0 - p.p75);
        let assign1170_e1414: f64 = (p.p71 / locals.var_vdc_ctc_t);
        let assign1170_e1416: f64 = (assign1170_e1414).powf(p.p72);
        let assign1170_e1417: f64 = (assign1170_e1411 * assign1170_e1416);
        let assign1170_e1419: f64 = (assign1170_e1417 + p.p75);
        locals.var_cjc_scale = assign1170_e1419;
        locals.var_cjc_scale_dn0 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn0) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn1 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn1) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn3 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn3) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn4 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn4) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn5 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn5) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn6 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn6) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn7 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn7) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn8 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn8) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn9 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn9) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn10 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn10) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn10) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_dn11 = (assign1170_e1411 * if 0.0 == 0.0 && ((p.p72) as f64).is_finite() && ((p.p72) as f64).fract() == 0.0 { if p.p72 == 0.0 { 0.0 } else { (p.p72 * ((assign1170_e1414).powf(p.p72 - 1.0) * (-((p.p71 * locals.var_vdc_ctc_t_dn11) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign1170_e1416 * (p.p72 * ((-((p.p71 * locals.var_vdc_ctc_t_dn11) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign1170_e1414))) });
        locals.var_cjc_scale_rv = 0.0;

        let assign1180_e1422: f64 = (1.0 / locals.var_cjc_scale);
        locals.var_cjc_scale_inv = assign1180_e1422;
        locals.var_cjc_scale_inv_dn0 = (-(locals.var_cjc_scale_dn0 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn1 = (-(locals.var_cjc_scale_dn1 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn3 = (-(locals.var_cjc_scale_dn3 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn4 = (-(locals.var_cjc_scale_dn4 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn5 = (-(locals.var_cjc_scale_dn5 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn6 = (-(locals.var_cjc_scale_dn6 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn7 = (-(locals.var_cjc_scale_dn7 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn8 = (-(locals.var_cjc_scale_dn8 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn9 = (-(locals.var_cjc_scale_dn9 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn10 = (-(locals.var_cjc_scale_dn10 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_dn11 = (-(locals.var_cjc_scale_dn11 / (locals.var_cjc_scale * locals.var_cjc_scale)));
        locals.var_cjc_scale_inv_rv = 0.0;

        let assign1190_e1425: f64 = (p.p70 * locals.var_cjc_scale);
        locals.var_cjc_t = assign1190_e1425;
        locals.var_cjc_t_dn0 = (p.p70 * locals.var_cjc_scale_dn0);
        locals.var_cjc_t_dn1 = (p.p70 * locals.var_cjc_scale_dn1);
        locals.var_cjc_t_dn3 = (p.p70 * locals.var_cjc_scale_dn3);
        locals.var_cjc_t_dn4 = (p.p70 * locals.var_cjc_scale_dn4);
        locals.var_cjc_t_dn5 = (p.p70 * locals.var_cjc_scale_dn5);
        locals.var_cjc_t_dn6 = (p.p70 * locals.var_cjc_scale_dn6);
        locals.var_cjc_t_dn7 = (p.p70 * locals.var_cjc_scale_dn7);
        locals.var_cjc_t_dn8 = (p.p70 * locals.var_cjc_scale_dn8);
        locals.var_cjc_t_dn9 = (p.p70 * locals.var_cjc_scale_dn9);
        locals.var_cjc_t_dn10 = (p.p70 * locals.var_cjc_scale_dn10);
        locals.var_cjc_t_dn11 = (p.p70 * locals.var_cjc_scale_dn11);
        locals.var_cjc_t_rv = 0.0;

        let assign1200_e1428: f64 = (p.p75 * locals.var_cjc_scale_inv);
        locals.var_xp_t = assign1200_e1428;
        locals.var_xp_t_dn0 = (p.p75 * locals.var_cjc_scale_inv_dn0);
        locals.var_xp_t_dn1 = (p.p75 * locals.var_cjc_scale_inv_dn1);
        locals.var_xp_t_dn3 = (p.p75 * locals.var_cjc_scale_inv_dn3);
        locals.var_xp_t_dn4 = (p.p75 * locals.var_cjc_scale_inv_dn4);
        locals.var_xp_t_dn5 = (p.p75 * locals.var_cjc_scale_inv_dn5);
        locals.var_xp_t_dn6 = (p.p75 * locals.var_cjc_scale_inv_dn6);
        locals.var_xp_t_dn7 = (p.p75 * locals.var_cjc_scale_inv_dn7);
        locals.var_xp_t_dn8 = (p.p75 * locals.var_cjc_scale_inv_dn8);
        locals.var_xp_t_dn9 = (p.p75 * locals.var_cjc_scale_inv_dn9);
        locals.var_xp_t_dn10 = (p.p75 * locals.var_cjc_scale_inv_dn10);
        locals.var_xp_t_dn11 = (p.p75 * locals.var_cjc_scale_inv_dn11);
        locals.var_xp_t_rv = 0.0;

        let assign1210_e1432: f64 = (locals.var_lntn * p.p97);
        let assign1210_e1433: f64 = (assign1210_e1432).exp();
        let assign1210_e1434: f64 = (p.p54 * assign1210_e1433);
        locals.var_re_t = assign1210_e1434;
        locals.var_re_t_dn4 = (p.p54 * (assign1210_e1433 * (locals.var_lntn_dn4 * p.p97)));
        locals.var_re_t_rv = 0.0;

        let assign1220_e1437: f64 = if locals.var_re_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1220_e1437;
        locals.var_guard17_rv = 0.0;

        let (assign1230_e1441, assign1230_e1441_d_n4,) = {
    if (locals.var_guard17 != 0.0) {
        (locals.var_minr_m, 0.0,)
    } else {
        (locals.var_re_t, locals.var_re_t_dn4,)
    }
};
        locals.var_re_t = assign1230_e1441;
        locals.var_re_t_dn4 = assign1230_e1441_d_n4;
        locals.var_re_t_rv = 0.0;

        let assign1240_e1446: f64 = (p.p98 - p.p96);
        let assign1240_e1447: f64 = (locals.var_lntn * assign1240_e1446);
        let assign1240_e1448: f64 = (assign1240_e1447).exp();
        let assign1240_e1449: f64 = (p.p56 * assign1240_e1448);
        locals.var_rbv_t = assign1240_e1449;
        locals.var_rbv_t_dn4 = (p.p56 * (assign1240_e1448 * (locals.var_lntn_dn4 * assign1240_e1446)));
        locals.var_rbv_t_rv = 0.0;

        let assign1250_e1453: f64 = (locals.var_lntn * p.p101);
        let assign1250_e1454: f64 = (assign1250_e1453).exp();
        let assign1250_e1455: f64 = (p.p55 * assign1250_e1454);
        locals.var_rbc_t = assign1250_e1455;
        locals.var_rbc_t_dn4 = (p.p55 * (assign1250_e1454 * (locals.var_lntn_dn4 * p.p101)));
        locals.var_rbc_t_rv = 0.0;

        let assign1260_e1458: f64 = if locals.var_rbc_t < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1260_e1458;
        locals.var_guard18_rv = 0.0;

        let (assign1270_e1462, assign1270_e1462_d_n4,) = {
    if (locals.var_guard18 != 0.0) {
        (locals.var_minr_m, 0.0,)
    } else {
        (locals.var_rbc_t, locals.var_rbc_t_dn4,)
    }
};
        locals.var_rbc_t = assign1270_e1462;
        locals.var_rbc_t_dn4 = assign1270_e1462_d_n4;
        locals.var_rbc_t_rv = 0.0;

        let assign1280_e1466: f64 = (locals.var_lntn * p.p102);
        let assign1280_e1467: f64 = (assign1280_e1466).exp();
        let assign1280_e1468: f64 = (p.p57 * assign1280_e1467);
        locals.var_rcc_xx_t = assign1280_e1468;
        locals.var_rcc_xx_t_dn4 = (p.p57 * (assign1280_e1467 * (locals.var_lntn_dn4 * p.p102)));
        locals.var_rcc_xx_t_rv = 0.0;

        let assign1310_e1484: f64 = (locals.var_lntn * p.p99);
        let assign1310_e1485: f64 = (assign1310_e1484).exp();
        let assign1310_e1486: f64 = (p.p60 * assign1310_e1485);
        locals.var_rcv_t = assign1310_e1486;
        locals.var_rcv_t_dn4 = (p.p60 * (assign1310_e1485 * (locals.var_lntn_dn4 * p.p99)));
        locals.var_rcv_t_rv = 0.0;

        let assign1320_e1489: f64 = if p.p122 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign1320_e1489;
        locals.var_guard19_rv = 0.0;

        let (assign1330_e1499, assign1330_e1499_d_n0, assign1330_e1499_d_n1, assign1330_e1499_d_n3, assign1330_e1499_d_n4, assign1330_e1499_d_n5, assign1330_e1499_d_n6, assign1330_e1499_d_n7, assign1330_e1499_d_n8, assign1330_e1499_d_n9, assign1330_e1499_d_n10, assign1330_e1499_d_n11,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1330_e1495: f64 = (locals.var_dt * p.p122);
        let assign1330_e1496: f64 = (1.0 + assign1330_e1495);
        let assign1330_e1497: f64 = (p.p10 * assign1330_e1496);
        (assign1330_e1497, 0.0, 0.0, 0.0, (p.p10 * (locals.var_dt_dn4 * p.p122)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10, locals.var_nff_t_tmp_dn11,)
    }
};
        locals.var_nff_t_tmp = assign1330_e1499;
        locals.var_nff_t_tmp_dn0 = assign1330_e1499_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1330_e1499_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1330_e1499_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1330_e1499_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1330_e1499_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1330_e1499_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1330_e1499_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1330_e1499_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1330_e1499_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1330_e1499_d_n10;
        locals.var_nff_t_tmp_dn11 = assign1330_e1499_d_n11;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1340_e1507, assign1340_e1507_d_n0, assign1340_e1507_d_n1, assign1340_e1507_d_n3, assign1340_e1507_d_n4, assign1340_e1507_d_n5, assign1340_e1507_d_n6, assign1340_e1507_d_n7, assign1340_e1507_d_n8, assign1340_e1507_d_n9, assign1340_e1507_d_n10, assign1340_e1507_d_n11,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1340_e1503: f64 = (locals.var_nff_t_tmp - 1.0);
        let assign1340_e1505: f64 = (assign1340_e1503 / locals.var_eps_nf);
        (assign1340_e1505, (locals.var_nff_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn9 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn10 / locals.var_eps_nf), (locals.var_nff_t_tmp_dn11 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign1340_e1507;
        locals.var_dxa_dn0 = assign1340_e1507_d_n0;
        locals.var_dxa_dn1 = assign1340_e1507_d_n1;
        locals.var_dxa_dn3 = assign1340_e1507_d_n3;
        locals.var_dxa_dn4 = assign1340_e1507_d_n4;
        locals.var_dxa_dn5 = assign1340_e1507_d_n5;
        locals.var_dxa_dn6 = assign1340_e1507_d_n6;
        locals.var_dxa_dn7 = assign1340_e1507_d_n7;
        locals.var_dxa_dn8 = assign1340_e1507_d_n8;
        locals.var_dxa_dn9 = assign1340_e1507_d_n9;
        locals.var_dxa_dn10 = assign1340_e1507_d_n10;
        locals.var_dxa_dn11 = assign1340_e1507_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign1350_e1510: f64 = if locals.var_nff_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1350_e1510;
        locals.var_guard20_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1360_e1524, assign1360_e1524_d_n0, assign1360_e1524_d_n1, assign1360_e1524_d_n3, assign1360_e1524_d_n4, assign1360_e1524_d_n5, assign1360_e1524_d_n6, assign1360_e1524_d_n7, assign1360_e1524_d_n8, assign1360_e1524_d_n9, assign1360_e1524_d_n10, assign1360_e1524_d_n11,) = {
    if ((locals.var_guard19 != 0.0) && (locals.var_guard20 != 0.0)) {
        let assign1360_e1518: f64 = (locals.var_dxa).exp();
        let assign1360_e1519: f64 = (1.0 + assign1360_e1518);
        let assign1360_e1520: f64 = (assign1360_e1519).ln();
        let assign1360_e1521: f64 = (locals.var_eps_nf * assign1360_e1520);
        let assign1360_e1522: f64 = (1.0 + assign1360_e1521);
        (assign1360_e1522, (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn0) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn1) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn3) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn4) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn5) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn6) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn7) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn8) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn9) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn10) / assign1360_e1519)), (locals.var_eps_nf * ((assign1360_e1518 * locals.var_dxa_dn11) / assign1360_e1519)),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10, locals.var_nff_t_tmp_dn11,)
    }
};
        locals.var_nff_t_tmp = assign1360_e1524;
        locals.var_nff_t_tmp_dn0 = assign1360_e1524_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1360_e1524_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1360_e1524_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1360_e1524_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1360_e1524_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1360_e1524_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1360_e1524_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1360_e1524_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1360_e1524_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1360_e1524_d_n10;
        locals.var_nff_t_tmp_dn11 = assign1360_e1524_d_n11;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1370_e1540, assign1370_e1540_d_n0, assign1370_e1540_d_n1, assign1370_e1540_d_n3, assign1370_e1540_d_n4, assign1370_e1540_d_n5, assign1370_e1540_d_n6, assign1370_e1540_d_n7, assign1370_e1540_d_n8, assign1370_e1540_d_n9, assign1370_e1540_d_n10, assign1370_e1540_d_n11,) = {
    if ((locals.var_guard19 != 0.0) && (locals.var_guard20 == 0.0)) {
        let assign1370_e1533: f64 = (-locals.var_dxa);
        let assign1370_e1534: f64 = (assign1370_e1533).exp();
        let assign1370_e1535: f64 = (1.0 + assign1370_e1534);
        let assign1370_e1536: f64 = (assign1370_e1535).ln();
        let assign1370_e1537: f64 = (locals.var_eps_nf * assign1370_e1536);
        let assign1370_e1538: f64 = (locals.var_nff_t_tmp + assign1370_e1537);
        (assign1370_e1538, (locals.var_nff_t_tmp_dn0 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn0)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn1 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn1)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn3 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn3)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn4 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn4)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn5 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn5)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn6 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn6)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn7 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn7)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn8 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn8)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn9 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn9)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn10 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn10)) / assign1370_e1535))), (locals.var_nff_t_tmp_dn11 + (locals.var_eps_nf * ((assign1370_e1534 * (-locals.var_dxa_dn11)) / assign1370_e1535))),)
    } else {
        (locals.var_nff_t_tmp, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10, locals.var_nff_t_tmp_dn11,)
    }
};
        locals.var_nff_t_tmp = assign1370_e1540;
        locals.var_nff_t_tmp_dn0 = assign1370_e1540_d_n0;
        locals.var_nff_t_tmp_dn1 = assign1370_e1540_d_n1;
        locals.var_nff_t_tmp_dn3 = assign1370_e1540_d_n3;
        locals.var_nff_t_tmp_dn4 = assign1370_e1540_d_n4;
        locals.var_nff_t_tmp_dn5 = assign1370_e1540_d_n5;
        locals.var_nff_t_tmp_dn6 = assign1370_e1540_d_n6;
        locals.var_nff_t_tmp_dn7 = assign1370_e1540_d_n7;
        locals.var_nff_t_tmp_dn8 = assign1370_e1540_d_n8;
        locals.var_nff_t_tmp_dn9 = assign1370_e1540_d_n9;
        locals.var_nff_t_tmp_dn10 = assign1370_e1540_d_n10;
        locals.var_nff_t_tmp_dn11 = assign1370_e1540_d_n11;
        locals.var_nff_t_tmp_rv = 0.0;

        let (assign1380_e1548, assign1380_e1548_d_n0, assign1380_e1548_d_n1, assign1380_e1548_d_n3, assign1380_e1548_d_n4, assign1380_e1548_d_n5, assign1380_e1548_d_n6, assign1380_e1548_d_n7, assign1380_e1548_d_n8, assign1380_e1548_d_n9, assign1380_e1548_d_n10, assign1380_e1548_d_n11,) = {
    if (locals.var_guard19 != 0.0) {
        let assign1380_e1545: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1380_e1546: f64 = (locals.var_nff_t_tmp - assign1380_e1545);
        (assign1380_e1546, locals.var_nff_t_tmp_dn0, locals.var_nff_t_tmp_dn1, locals.var_nff_t_tmp_dn3, locals.var_nff_t_tmp_dn4, locals.var_nff_t_tmp_dn5, locals.var_nff_t_tmp_dn6, locals.var_nff_t_tmp_dn7, locals.var_nff_t_tmp_dn8, locals.var_nff_t_tmp_dn9, locals.var_nff_t_tmp_dn10, locals.var_nff_t_tmp_dn11,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9, locals.var_nff_t_dn10, locals.var_nff_t_dn11,)
    }
};
        locals.var_nff_t = assign1380_e1548;
        locals.var_nff_t_dn0 = assign1380_e1548_d_n0;
        locals.var_nff_t_dn1 = assign1380_e1548_d_n1;
        locals.var_nff_t_dn3 = assign1380_e1548_d_n3;
        locals.var_nff_t_dn4 = assign1380_e1548_d_n4;
        locals.var_nff_t_dn5 = assign1380_e1548_d_n5;
        locals.var_nff_t_dn6 = assign1380_e1548_d_n6;
        locals.var_nff_t_dn7 = assign1380_e1548_d_n7;
        locals.var_nff_t_dn8 = assign1380_e1548_d_n8;
        locals.var_nff_t_dn9 = assign1380_e1548_d_n9;
        locals.var_nff_t_dn10 = assign1380_e1548_d_n10;
        locals.var_nff_t_dn11 = assign1380_e1548_d_n11;
        locals.var_nff_t_rv = 0.0;

        let (assign1390_e1553, assign1390_e1553_d_n0, assign1390_e1553_d_n1, assign1390_e1553_d_n3, assign1390_e1553_d_n4, assign1390_e1553_d_n5, assign1390_e1553_d_n6, assign1390_e1553_d_n7, assign1390_e1553_d_n8, assign1390_e1553_d_n9, assign1390_e1553_d_n10, assign1390_e1553_d_n11,) = {
    if (locals.var_guard19 == 0.0) {
        (p.p10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nff_t, locals.var_nff_t_dn0, locals.var_nff_t_dn1, locals.var_nff_t_dn3, locals.var_nff_t_dn4, locals.var_nff_t_dn5, locals.var_nff_t_dn6, locals.var_nff_t_dn7, locals.var_nff_t_dn8, locals.var_nff_t_dn9, locals.var_nff_t_dn10, locals.var_nff_t_dn11,)
    }
};
        locals.var_nff_t = assign1390_e1553;
        locals.var_nff_t_dn0 = assign1390_e1553_d_n0;
        locals.var_nff_t_dn1 = assign1390_e1553_d_n1;
        locals.var_nff_t_dn3 = assign1390_e1553_d_n3;
        locals.var_nff_t_dn4 = assign1390_e1553_d_n4;
        locals.var_nff_t_dn5 = assign1390_e1553_d_n5;
        locals.var_nff_t_dn6 = assign1390_e1553_d_n6;
        locals.var_nff_t_dn7 = assign1390_e1553_d_n7;
        locals.var_nff_t_dn8 = assign1390_e1553_d_n8;
        locals.var_nff_t_dn9 = assign1390_e1553_d_n9;
        locals.var_nff_t_dn10 = assign1390_e1553_d_n10;
        locals.var_nff_t_dn11 = assign1390_e1553_d_n11;
        locals.var_nff_t_rv = 0.0;

        let assign1400_e1556: f64 = if p.p123 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1400_e1556;
        locals.var_guard21_rv = 0.0;

        let (assign1410_e1566, assign1410_e1566_d_n0, assign1410_e1566_d_n1, assign1410_e1566_d_n3, assign1410_e1566_d_n4, assign1410_e1566_d_n5, assign1410_e1566_d_n6, assign1410_e1566_d_n7, assign1410_e1566_d_n8, assign1410_e1566_d_n9, assign1410_e1566_d_n10, assign1410_e1566_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1410_e1562: f64 = (locals.var_dt * p.p123);
        let assign1410_e1563: f64 = (1.0 + assign1410_e1562);
        let assign1410_e1564: f64 = (p.p11 * assign1410_e1563);
        (assign1410_e1564, 0.0, 0.0, 0.0, (p.p11 * (locals.var_dt_dn4 * p.p123)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10, locals.var_nfr_t_tmp_dn11,)
    }
};
        locals.var_nfr_t_tmp = assign1410_e1566;
        locals.var_nfr_t_tmp_dn0 = assign1410_e1566_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1410_e1566_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1410_e1566_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1410_e1566_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1410_e1566_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1410_e1566_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1410_e1566_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1410_e1566_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1410_e1566_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1410_e1566_d_n10;
        locals.var_nfr_t_tmp_dn11 = assign1410_e1566_d_n11;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1420_e1574, assign1420_e1574_d_n0, assign1420_e1574_d_n1, assign1420_e1574_d_n3, assign1420_e1574_d_n4, assign1420_e1574_d_n5, assign1420_e1574_d_n6, assign1420_e1574_d_n7, assign1420_e1574_d_n8, assign1420_e1574_d_n9, assign1420_e1574_d_n10, assign1420_e1574_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1420_e1570: f64 = (locals.var_nfr_t_tmp - 1.0);
        let assign1420_e1572: f64 = (assign1420_e1570 / locals.var_eps_nf);
        (assign1420_e1572, (locals.var_nfr_t_tmp_dn0 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn1 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn3 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn4 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn5 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn6 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn7 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn8 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn9 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn10 / locals.var_eps_nf), (locals.var_nfr_t_tmp_dn11 / locals.var_eps_nf),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign1420_e1574;
        locals.var_dxa_dn0 = assign1420_e1574_d_n0;
        locals.var_dxa_dn1 = assign1420_e1574_d_n1;
        locals.var_dxa_dn3 = assign1420_e1574_d_n3;
        locals.var_dxa_dn4 = assign1420_e1574_d_n4;
        locals.var_dxa_dn5 = assign1420_e1574_d_n5;
        locals.var_dxa_dn6 = assign1420_e1574_d_n6;
        locals.var_dxa_dn7 = assign1420_e1574_d_n7;
        locals.var_dxa_dn8 = assign1420_e1574_d_n8;
        locals.var_dxa_dn9 = assign1420_e1574_d_n9;
        locals.var_dxa_dn10 = assign1420_e1574_d_n10;
        locals.var_dxa_dn11 = assign1420_e1574_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign1430_e1577: f64 = if locals.var_nfr_t_tmp < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1430_e1577;
        locals.var_guard22_rv = 0.0;

        let (assign1440_e1591, assign1440_e1591_d_n0, assign1440_e1591_d_n1, assign1440_e1591_d_n3, assign1440_e1591_d_n4, assign1440_e1591_d_n5, assign1440_e1591_d_n6, assign1440_e1591_d_n7, assign1440_e1591_d_n8, assign1440_e1591_d_n9, assign1440_e1591_d_n10, assign1440_e1591_d_n11,) = {
    if ((locals.var_guard21 != 0.0) && (locals.var_guard22 != 0.0)) {
        let assign1440_e1585: f64 = (locals.var_dxa).exp();
        let assign1440_e1586: f64 = (1.0 + assign1440_e1585);
        let assign1440_e1587: f64 = (assign1440_e1586).ln();
        let assign1440_e1588: f64 = (locals.var_eps_nf * assign1440_e1587);
        let assign1440_e1589: f64 = (1.0 + assign1440_e1588);
        (assign1440_e1589, (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn0) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn1) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn3) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn4) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn5) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn6) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn7) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn8) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn9) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn10) / assign1440_e1586)), (locals.var_eps_nf * ((assign1440_e1585 * locals.var_dxa_dn11) / assign1440_e1586)),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10, locals.var_nfr_t_tmp_dn11,)
    }
};
        locals.var_nfr_t_tmp = assign1440_e1591;
        locals.var_nfr_t_tmp_dn0 = assign1440_e1591_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1440_e1591_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1440_e1591_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1440_e1591_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1440_e1591_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1440_e1591_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1440_e1591_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1440_e1591_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1440_e1591_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1440_e1591_d_n10;
        locals.var_nfr_t_tmp_dn11 = assign1440_e1591_d_n11;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1450_e1607, assign1450_e1607_d_n0, assign1450_e1607_d_n1, assign1450_e1607_d_n3, assign1450_e1607_d_n4, assign1450_e1607_d_n5, assign1450_e1607_d_n6, assign1450_e1607_d_n7, assign1450_e1607_d_n8, assign1450_e1607_d_n9, assign1450_e1607_d_n10, assign1450_e1607_d_n11,) = {
    if ((locals.var_guard21 != 0.0) && (locals.var_guard22 == 0.0)) {
        let assign1450_e1600: f64 = (-locals.var_dxa);
        let assign1450_e1601: f64 = (assign1450_e1600).exp();
        let assign1450_e1602: f64 = (1.0 + assign1450_e1601);
        let assign1450_e1603: f64 = (assign1450_e1602).ln();
        let assign1450_e1604: f64 = (locals.var_eps_nf * assign1450_e1603);
        let assign1450_e1605: f64 = (locals.var_nfr_t_tmp + assign1450_e1604);
        (assign1450_e1605, (locals.var_nfr_t_tmp_dn0 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn0)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn1 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn1)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn3 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn3)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn4 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn4)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn5 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn5)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn6 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn6)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn7 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn7)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn8 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn8)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn9 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn9)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn10 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn10)) / assign1450_e1602))), (locals.var_nfr_t_tmp_dn11 + (locals.var_eps_nf * ((assign1450_e1601 * (-locals.var_dxa_dn11)) / assign1450_e1602))),)
    } else {
        (locals.var_nfr_t_tmp, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10, locals.var_nfr_t_tmp_dn11,)
    }
};
        locals.var_nfr_t_tmp = assign1450_e1607;
        locals.var_nfr_t_tmp_dn0 = assign1450_e1607_d_n0;
        locals.var_nfr_t_tmp_dn1 = assign1450_e1607_d_n1;
        locals.var_nfr_t_tmp_dn3 = assign1450_e1607_d_n3;
        locals.var_nfr_t_tmp_dn4 = assign1450_e1607_d_n4;
        locals.var_nfr_t_tmp_dn5 = assign1450_e1607_d_n5;
        locals.var_nfr_t_tmp_dn6 = assign1450_e1607_d_n6;
        locals.var_nfr_t_tmp_dn7 = assign1450_e1607_d_n7;
        locals.var_nfr_t_tmp_dn8 = assign1450_e1607_d_n8;
        locals.var_nfr_t_tmp_dn9 = assign1450_e1607_d_n9;
        locals.var_nfr_t_tmp_dn10 = assign1450_e1607_d_n10;
        locals.var_nfr_t_tmp_dn11 = assign1450_e1607_d_n11;
        locals.var_nfr_t_tmp_rv = 0.0;

        let (assign1460_e1615, assign1460_e1615_d_n0, assign1460_e1615_d_n1, assign1460_e1615_d_n3, assign1460_e1615_d_n4, assign1460_e1615_d_n5, assign1460_e1615_d_n6, assign1460_e1615_d_n7, assign1460_e1615_d_n8, assign1460_e1615_d_n9, assign1460_e1615_d_n10, assign1460_e1615_d_n11,) = {
    if (locals.var_guard21 != 0.0) {
        let assign1460_e1612: f64 = (locals.var_eps_nf * 0.6931471805599453);
        let assign1460_e1613: f64 = (locals.var_nfr_t_tmp - assign1460_e1612);
        (assign1460_e1613, locals.var_nfr_t_tmp_dn0, locals.var_nfr_t_tmp_dn1, locals.var_nfr_t_tmp_dn3, locals.var_nfr_t_tmp_dn4, locals.var_nfr_t_tmp_dn5, locals.var_nfr_t_tmp_dn6, locals.var_nfr_t_tmp_dn7, locals.var_nfr_t_tmp_dn8, locals.var_nfr_t_tmp_dn9, locals.var_nfr_t_tmp_dn10, locals.var_nfr_t_tmp_dn11,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9, locals.var_nfr_t_dn10, locals.var_nfr_t_dn11,)
    }
};
        locals.var_nfr_t = assign1460_e1615;
        locals.var_nfr_t_dn0 = assign1460_e1615_d_n0;
        locals.var_nfr_t_dn1 = assign1460_e1615_d_n1;
        locals.var_nfr_t_dn3 = assign1460_e1615_d_n3;
        locals.var_nfr_t_dn4 = assign1460_e1615_d_n4;
        locals.var_nfr_t_dn5 = assign1460_e1615_d_n5;
        locals.var_nfr_t_dn6 = assign1460_e1615_d_n6;
        locals.var_nfr_t_dn7 = assign1460_e1615_d_n7;
        locals.var_nfr_t_dn8 = assign1460_e1615_d_n8;
        locals.var_nfr_t_dn9 = assign1460_e1615_d_n9;
        locals.var_nfr_t_dn10 = assign1460_e1615_d_n10;
        locals.var_nfr_t_dn11 = assign1460_e1615_d_n11;
        locals.var_nfr_t_rv = 0.0;

        let (assign1470_e1620, assign1470_e1620_d_n0, assign1470_e1620_d_n1, assign1470_e1620_d_n3, assign1470_e1620_d_n4, assign1470_e1620_d_n5, assign1470_e1620_d_n6, assign1470_e1620_d_n7, assign1470_e1620_d_n8, assign1470_e1620_d_n9, assign1470_e1620_d_n10, assign1470_e1620_d_n11,) = {
    if (locals.var_guard21 == 0.0) {
        (p.p11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nfr_t, locals.var_nfr_t_dn0, locals.var_nfr_t_dn1, locals.var_nfr_t_dn3, locals.var_nfr_t_dn4, locals.var_nfr_t_dn5, locals.var_nfr_t_dn6, locals.var_nfr_t_dn7, locals.var_nfr_t_dn8, locals.var_nfr_t_dn9, locals.var_nfr_t_dn10, locals.var_nfr_t_dn11,)
    }
};
        locals.var_nfr_t = assign1470_e1620;
        locals.var_nfr_t_dn0 = assign1470_e1620_d_n0;
        locals.var_nfr_t_dn1 = assign1470_e1620_d_n1;
        locals.var_nfr_t_dn3 = assign1470_e1620_d_n3;
        locals.var_nfr_t_dn4 = assign1470_e1620_d_n4;
        locals.var_nfr_t_dn5 = assign1470_e1620_d_n5;
        locals.var_nfr_t_dn6 = assign1470_e1620_d_n6;
        locals.var_nfr_t_dn7 = assign1470_e1620_d_n7;
        locals.var_nfr_t_dn8 = assign1470_e1620_d_n8;
        locals.var_nfr_t_dn9 = assign1470_e1620_d_n9;
        locals.var_nfr_t_dn10 = assign1470_e1620_d_n10;
        locals.var_nfr_t_dn11 = assign1470_e1620_d_n11;
        locals.var_nfr_t_rv = 0.0;

        let assign1480_e1625: f64 = (p.p124 * locals.var_dt);
        let assign1480_e1626: f64 = (1.0 + assign1480_e1625);
        let assign1480_e1627: f64 = (p.p43 * assign1480_e1626);
        locals.var_bavl_t_tmp = assign1480_e1627;
        locals.var_bavl_t_tmp_dn4 = (p.p43 * (p.p124 * locals.var_dt_dn4));
        locals.var_bavl_t_tmp_rv = 0.0;

        let assign1490_e1630: f64 = (locals.var_eps_bavl_t * locals.var_eps_bavl_t);
        locals.var_eps2 = assign1490_e1630;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_dn11 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign1500_e1633: f64 = (locals.var_bavl_t_tmp * locals.var_bavl_t_tmp);
        locals.var_x2 = assign1500_e1633;
        locals.var_x2_dn0 = 0.0;
        locals.var_x2_dn1 = 0.0;
        locals.var_x2_dn3 = 0.0;
        locals.var_x2_dn4 = ((locals.var_bavl_t_tmp_dn4 * locals.var_bavl_t_tmp) + (locals.var_bavl_t_tmp * locals.var_bavl_t_tmp_dn4));
        locals.var_x2_dn5 = 0.0;
        locals.var_x2_dn6 = 0.0;
        locals.var_x2_dn7 = 0.0;
        locals.var_x2_dn8 = 0.0;
        locals.var_x2_dn9 = 0.0;
        locals.var_x2_dn10 = 0.0;
        locals.var_x2_dn11 = 0.0;
        locals.var_x2_rv = 0.0;

        let assign1510_e1636: f64 = if locals.var_bavl_t_tmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1510_e1636;
        locals.var_guard23_rv = 0.0;

        let (assign1520_e1649, assign1520_e1649_d_n0, assign1520_e1649_d_n1, assign1520_e1649_d_n3, assign1520_e1649_d_n4, assign1520_e1649_d_n5, assign1520_e1649_d_n6, assign1520_e1649_d_n7, assign1520_e1649_d_n8, assign1520_e1649_d_n9, assign1520_e1649_d_n10, assign1520_e1649_d_n11,) = {
    if (locals.var_guard23 != 0.0) {
        let assign1520_e1640: f64 = (0.5 * locals.var_eps2);
        let assign1520_e1643: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1520_e1644: f64 = (assign1520_e1643).sqrt();
        let assign1520_e1646: f64 = (assign1520_e1644 - locals.var_bavl_t_tmp);
        let assign1520_e1647: f64 = (assign1520_e1640 / assign1520_e1646);
        (assign1520_e1647, ((((0.5 * locals.var_eps2_dn0) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn1) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn3) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn4) * assign1520_e1646) - (assign1520_e1640 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1520_e1644)) - locals.var_bavl_t_tmp_dn4))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn5) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn6) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn7) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn8) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn9) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn10) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)), ((((0.5 * locals.var_eps2_dn11) * assign1520_e1646) - (assign1520_e1640 * ((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign1520_e1644)))) / (assign1520_e1646 * assign1520_e1646)),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9, locals.var_bavl_t_dn10, locals.var_bavl_t_dn11,)
    }
};
        locals.var_bavl_t = assign1520_e1649;
        locals.var_bavl_t_dn0 = assign1520_e1649_d_n0;
        locals.var_bavl_t_dn1 = assign1520_e1649_d_n1;
        locals.var_bavl_t_dn3 = assign1520_e1649_d_n3;
        locals.var_bavl_t_dn4 = assign1520_e1649_d_n4;
        locals.var_bavl_t_dn5 = assign1520_e1649_d_n5;
        locals.var_bavl_t_dn6 = assign1520_e1649_d_n6;
        locals.var_bavl_t_dn7 = assign1520_e1649_d_n7;
        locals.var_bavl_t_dn8 = assign1520_e1649_d_n8;
        locals.var_bavl_t_dn9 = assign1520_e1649_d_n9;
        locals.var_bavl_t_dn10 = assign1520_e1649_d_n10;
        locals.var_bavl_t_dn11 = assign1520_e1649_d_n11;
        locals.var_bavl_t_rv = 0.0;

        let (assign1530_e1661, assign1530_e1661_d_n0, assign1530_e1661_d_n1, assign1530_e1661_d_n3, assign1530_e1661_d_n4, assign1530_e1661_d_n5, assign1530_e1661_d_n6, assign1530_e1661_d_n7, assign1530_e1661_d_n8, assign1530_e1661_d_n9, assign1530_e1661_d_n10, assign1530_e1661_d_n11,) = {
    if (locals.var_guard23 == 0.0) {
        let assign1530_e1655: f64 = (locals.var_x2 + locals.var_eps2);
        let assign1530_e1656: f64 = (assign1530_e1655).sqrt();
        let assign1530_e1658: f64 = (assign1530_e1656 + locals.var_bavl_t_tmp);
        let assign1530_e1659: f64 = (0.5 * assign1530_e1658);
        (assign1530_e1659, (0.5 * ((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign1530_e1656))), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign1530_e1656)) + locals.var_bavl_t_tmp_dn4)), (0.5 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign1530_e1656))), (0.5 * ((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign1530_e1656))),)
    } else {
        (locals.var_bavl_t, locals.var_bavl_t_dn0, locals.var_bavl_t_dn1, locals.var_bavl_t_dn3, locals.var_bavl_t_dn4, locals.var_bavl_t_dn5, locals.var_bavl_t_dn6, locals.var_bavl_t_dn7, locals.var_bavl_t_dn8, locals.var_bavl_t_dn9, locals.var_bavl_t_dn10, locals.var_bavl_t_dn11,)
    }
};
        locals.var_bavl_t = assign1530_e1661;
        locals.var_bavl_t_dn0 = assign1530_e1661_d_n0;
        locals.var_bavl_t_dn1 = assign1530_e1661_d_n1;
        locals.var_bavl_t_dn3 = assign1530_e1661_d_n3;
        locals.var_bavl_t_dn4 = assign1530_e1661_d_n4;
        locals.var_bavl_t_dn5 = assign1530_e1661_d_n5;
        locals.var_bavl_t_dn6 = assign1530_e1661_d_n6;
        locals.var_bavl_t_dn7 = assign1530_e1661_d_n7;
        locals.var_bavl_t_dn8 = assign1530_e1661_d_n8;
        locals.var_bavl_t_dn9 = assign1530_e1661_d_n9;
        locals.var_bavl_t_dn10 = assign1530_e1661_d_n10;
        locals.var_bavl_t_dn11 = assign1530_e1661_d_n11;
        locals.var_bavl_t_rv = 0.0;

        let assign1540_e1666: f64 = (4.0 - p.p98);
        let assign1540_e1668: f64 = (assign1540_e1666 - p.p96);
        let assign1540_e1670: f64 = (assign1540_e1668 + p.p121);
        let assign1540_e1671: f64 = (locals.var_lntn * assign1540_e1670);
        let assign1540_e1673: f64 = (assign1540_e1671 / locals.var_nff_t);
        let assign1540_e1674: f64 = (assign1540_e1673).exp();
        let assign1540_e1675: f64 = (p.p9 * assign1540_e1674);
        let assign1540_e1677: f64 = (-p.p105);
        let assign1540_e1679: f64 = (assign1540_e1677 * locals.var_vdtinv);
        let assign1540_e1681: f64 = (assign1540_e1679 / locals.var_nff_t);
        let assign1540_e1682: f64 = (assign1540_e1681).exp();
        let assign1540_e1683: f64 = (assign1540_e1675 * assign1540_e1682);
        locals.var_is_t = assign1540_e1683;
        locals.var_is_t_dn0 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn1 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn3 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn4 = (((p.p9 * (assign1540_e1674 * ((((locals.var_lntn_dn4 * assign1540_e1670) * locals.var_nff_t) - (assign1540_e1671 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * ((((assign1540_e1677 * locals.var_vdtinv_dn4) * locals.var_nff_t) - (assign1540_e1679 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t)))));
        locals.var_is_t_dn5 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn5) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn6 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn7 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn7) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn8 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn9 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn10 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_dn11 = (((p.p9 * (assign1540_e1674 * (-((assign1540_e1671 * locals.var_nff_t_dn11) / (locals.var_nff_t * locals.var_nff_t))))) * assign1540_e1682) + (assign1540_e1675 * (assign1540_e1682 * (-((assign1540_e1679 * locals.var_nff_t_dn11) / (locals.var_nff_t * locals.var_nff_t))))));
        locals.var_is_t_rv = 0.0;

        let assign1550_e1688: f64 = (1.0 - p.p98);
        let assign1550_e1689: f64 = (locals.var_lntn * assign1550_e1688);
        let assign1550_e1690: f64 = (assign1550_e1689).exp();
        let assign1550_e1691: f64 = (p.p12 * assign1550_e1690);
        locals.var_ik_t = assign1550_e1691;
        locals.var_ik_t_dn4 = (p.p12 * (assign1550_e1690 * (locals.var_lntn_dn4 * assign1550_e1688)));
        locals.var_ik_t_rv = 0.0;

        let assign1560_e1696: f64 = (1.0 - p.p103);
        let assign1560_e1697: f64 = (locals.var_lntn * assign1560_e1696);
        let assign1560_e1698: f64 = (assign1560_e1697).exp();
        let assign1560_e1699: f64 = (p.p30 * assign1560_e1698);
        locals.var_ikbx_t = assign1560_e1699;
        locals.var_ikbx_t_dn4 = (p.p30 * (assign1560_e1698 * (locals.var_lntn_dn4 * assign1560_e1696)));
        locals.var_ikbx_t_rv = 0.0;

        let assign1590_e1740: f64 = (4.0 - p.p97);
        let assign1590_e1742: f64 = (assign1590_e1740 + p.p121);
        let assign1590_e1743: f64 = (locals.var_lntn * assign1590_e1742);
        let assign1590_e1745: f64 = (assign1590_e1743 / p.p17);
        let assign1590_e1746: f64 = (assign1590_e1745).exp();
        let assign1590_e1747: f64 = (p.p16 * assign1590_e1746);
        let assign1590_e1749: f64 = (-p.p111);
        let assign1590_e1751: f64 = (assign1590_e1749 * locals.var_vdtinv);
        let assign1590_e1753: f64 = (assign1590_e1751 / p.p17);
        let assign1590_e1754: f64 = (assign1590_e1753).exp();
        let assign1590_e1755: f64 = (assign1590_e1747 * assign1590_e1754);
        locals.var_ibi_t = assign1590_e1755;
        locals.var_ibi_t_dn4 = (((p.p16 * (assign1590_e1746 * ((locals.var_lntn_dn4 * assign1590_e1742) / p.p17))) * assign1590_e1754) + (assign1590_e1747 * (assign1590_e1754 * ((assign1590_e1749 * locals.var_vdtinv_dn4) / p.p17))));
        locals.var_ibi_t_rv = 0.0;

        let assign1650_e1817: f64 = (4.0 - p.p103);
        let assign1650_e1819: f64 = (assign1650_e1817 + p.p121);
        let assign1650_e1820: f64 = (locals.var_lntn * assign1650_e1819);
        let assign1650_e1821: f64 = (assign1650_e1820).exp();
        let assign1650_e1822: f64 = (p.p29 * assign1650_e1821);
        let assign1650_e1824: f64 = (-p.p112);
        let assign1650_e1826: f64 = (assign1650_e1824 * locals.var_vdtinv);
        let assign1650_e1827: f64 = (assign1650_e1826).exp();
        let assign1650_e1828: f64 = (assign1650_e1822 * assign1650_e1827);
        locals.var_ibx_t = assign1650_e1828;
        locals.var_ibx_t_dn4 = (((p.p29 * (assign1650_e1821 * (locals.var_lntn_dn4 * assign1650_e1819))) * assign1650_e1827) + (assign1650_e1822 * (assign1650_e1827 * (assign1650_e1824 * locals.var_vdtinv_dn4))));
        locals.var_ibx_t_rv = 0.0;

        let assign1690_e1874: f64 = (locals.var_vgzeb_t * locals.var_inv_vgzeb_tr);
        let assign1690_e1876: f64 = (-0.5);
        let assign1690_e1877: f64 = (assign1690_e1874).powf(assign1690_e1876);
        locals.var_x = assign1690_e1877;
        locals.var_x_dn0 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn0 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn1 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn1 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn3 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn3 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn4 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn4 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn5 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn5 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn6 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn6 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn7 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn7 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn8 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn8 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn9 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn9 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn10 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn10 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn10 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_dn11 = if 0.0 == 0.0 && ((assign1690_e1876) as f64).is_finite() && ((assign1690_e1876) as f64).fract() == 0.0 { if assign1690_e1876 == 0.0 { 0.0 } else { (assign1690_e1876 * ((assign1690_e1874).powf(assign1690_e1876 - 1.0) * (locals.var_vgzeb_t_dn11 * locals.var_inv_vgzeb_tr))) } } else { (assign1690_e1877 * (assign1690_e1876 * ((locals.var_vgzeb_t_dn11 * locals.var_inv_vgzeb_tr) / assign1690_e1874))) };
        locals.var_x_rv = 0.0;

        let assign1700_e1880: f64 = (1.0 / locals.var_cje_t_div_cje);
        locals.var_y = assign1700_e1880;
        locals.var_y_dn0 = (-(locals.var_cje_t_div_cje_dn0 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn1 = (-(locals.var_cje_t_div_cje_dn1 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn3 = (-(locals.var_cje_t_div_cje_dn3 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn4 = (-(locals.var_cje_t_div_cje_dn4 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn5 = (-(locals.var_cje_t_div_cje_dn5 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn6 = (-(locals.var_cje_t_div_cje_dn6 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn7 = (-(locals.var_cje_t_div_cje_dn7 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn8 = (-(locals.var_cje_t_div_cje_dn8 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn9 = (-(locals.var_cje_t_div_cje_dn9 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn10 = (-(locals.var_cje_t_div_cje_dn10 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_dn11 = (-(locals.var_cje_t_div_cje_dn11 / (locals.var_cje_t_div_cje * locals.var_cje_t_div_cje)));
        locals.var_y_rv = 0.0;

        let assign1710_e1883: f64 = (p.p35 * locals.var_vgzeb_t);
        let assign1710_e1885: f64 = (assign1710_e1883 * locals.var_vgzeb_t);
        let assign1710_e1887: f64 = (assign1710_e1885 * locals.var_x);
        let assign1710_e1889: f64 = (assign1710_e1887 * locals.var_y);
        let assign1710_e1891: f64 = (assign1710_e1889 * p.p66);
        let assign1710_e1893: f64 = (assign1710_e1891 * locals.var_inv_vde_t);
        let assign1710_e1895: f64 = (assign1710_e1893 * locals.var_inv_vgzeb_tr);
        let assign1710_e1897: f64 = (assign1710_e1895 * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t = assign1710_e1897;
        locals.var_nzeb_t_dn0 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn0) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn0)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn0)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn0)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn0)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn1 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn1) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn1)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn1)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn1)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn1)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn3 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn3) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn3)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn3)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn3)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn3)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn4 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn4) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn4)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn4)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn4)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn4)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn5 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn5) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn5)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn5)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn5)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn5)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn6 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn6) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn6)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn6)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn6)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn6)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn7 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn7) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn7)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn7)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn7)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn7)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn8 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn8) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn8)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn8)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn8)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn8)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn9 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn9) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn9)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn9)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn9)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn9)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn10 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn10) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn10)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn10)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn10)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn10)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_dn11 = ((((((((((((p.p35 * locals.var_vgzeb_t_dn11) * locals.var_vgzeb_t) + (assign1710_e1883 * locals.var_vgzeb_t_dn11)) * locals.var_x) + (assign1710_e1885 * locals.var_x_dn11)) * locals.var_y) + (assign1710_e1887 * locals.var_y_dn11)) * p.p66) * locals.var_inv_vde_t) + (assign1710_e1891 * locals.var_inv_vde_t_dn11)) * locals.var_inv_vgzeb_tr) * locals.var_inv_vgzeb_tr);
        locals.var_nzeb_t_rv = 0.0;

        let assign1730_e1918: f64 = (1.0 / locals.var_vdc_zener_t);
        locals.var_inv_vdc_zener_t = assign1730_e1918;
        locals.var_inv_vdc_zener_t_dn0 = (-(locals.var_vdc_zener_t_dn0 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn1 = (-(locals.var_vdc_zener_t_dn1 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn3 = (-(locals.var_vdc_zener_t_dn3 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn4 = (-(locals.var_vdc_zener_t_dn4 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn5 = (-(locals.var_vdc_zener_t_dn5 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn6 = (-(locals.var_vdc_zener_t_dn6 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn7 = (-(locals.var_vdc_zener_t_dn7 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn8 = (-(locals.var_vdc_zener_t_dn8 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn9 = (-(locals.var_vdc_zener_t_dn9 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn10 = (-(locals.var_vdc_zener_t_dn10 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_dn11 = (-(locals.var_vdc_zener_t_dn11 / (locals.var_vdc_zener_t * locals.var_vdc_zener_t)));
        locals.var_inv_vdc_zener_t_rv = 0.0;

        let assign1740_e1921: f64 = (locals.var_vgzcb_t * locals.var_inv_vgzcb_tr);
        let assign1740_e1923: f64 = (-0.5);
        let assign1740_e1924: f64 = (assign1740_e1921).powf(assign1740_e1923);
        locals.var_xx = assign1740_e1924;
        locals.var_xx_dn0 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn0 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn0 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn1 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn1 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn1 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn3 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn3 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn3 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn4 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn4 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn4 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn5 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn5 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn5 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn6 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn6 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn6 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn7 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn7 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn7 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn8 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn8 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn8 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn9 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn9 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn9 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn10 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn10 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn10 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_dn11 = if 0.0 == 0.0 && ((assign1740_e1923) as f64).is_finite() && ((assign1740_e1923) as f64).fract() == 0.0 { if assign1740_e1923 == 0.0 { 0.0 } else { (assign1740_e1923 * ((assign1740_e1921).powf(assign1740_e1923 - 1.0) * (locals.var_vgzcb_t_dn11 * locals.var_inv_vgzcb_tr))) } } else { (assign1740_e1924 * (assign1740_e1923 * ((locals.var_vgzcb_t_dn11 * locals.var_inv_vgzcb_tr) / assign1740_e1921))) };
        locals.var_xx_rv = 0.0;

        let assign1750_e1927: f64 = (1.0 / locals.var_cjc_t_div_cjc_zener);
        locals.var_yy = assign1750_e1927;
        locals.var_yy_dn0 = (-(locals.var_cjc_t_div_cjc_zener_dn0 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn1 = (-(locals.var_cjc_t_div_cjc_zener_dn1 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn3 = (-(locals.var_cjc_t_div_cjc_zener_dn3 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn4 = (-(locals.var_cjc_t_div_cjc_zener_dn4 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn5 = (-(locals.var_cjc_t_div_cjc_zener_dn5 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn6 = (-(locals.var_cjc_t_div_cjc_zener_dn6 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn7 = (-(locals.var_cjc_t_div_cjc_zener_dn7 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn8 = (-(locals.var_cjc_t_div_cjc_zener_dn8 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn9 = (-(locals.var_cjc_t_div_cjc_zener_dn9 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn10 = (-(locals.var_cjc_t_div_cjc_zener_dn10 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_dn11 = (-(locals.var_cjc_t_div_cjc_zener_dn11 / (locals.var_cjc_t_div_cjc_zener * locals.var_cjc_t_div_cjc_zener)));
        locals.var_yy_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let assign1760_e1930: f64 = (p.p37 * locals.var_vgzcb_t);
        let assign1760_e1932: f64 = (assign1760_e1930 * locals.var_vgzcb_t);
        let assign1760_e1934: f64 = (assign1760_e1932 * locals.var_xx);
        let assign1760_e1936: f64 = (assign1760_e1934 * locals.var_yy);
        let assign1760_e1938: f64 = (assign1760_e1936 * locals.var_vdc_zener);
        let assign1760_e1940: f64 = (assign1760_e1938 * locals.var_inv_vdc_zener_t);
        let assign1760_e1942: f64 = (assign1760_e1940 * locals.var_inv_vgzcb_tr);
        let assign1760_e1944: f64 = (assign1760_e1942 * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t = assign1760_e1944;
        locals.var_nzcb_t_dn0 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn0) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn0)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn0)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn0)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn0)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn1 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn1) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn1)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn1)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn1)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn1)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn3 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn3) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn3)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn3)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn3)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn3)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn4 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn4) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn4)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn4)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn4)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn4)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn5 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn5) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn5)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn5)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn5)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn5)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn6 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn6) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn6)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn6)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn6)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn6)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn7 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn7) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn7)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn7)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn7)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn7)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn8 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn8) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn8)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn8)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn8)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn8)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn9 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn9) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn9)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn9)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn9)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn9)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn10 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn10) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn10)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn10)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn10)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn10)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_dn11 = ((((((((((((p.p37 * locals.var_vgzcb_t_dn11) * locals.var_vgzcb_t) + (assign1760_e1930 * locals.var_vgzcb_t_dn11)) * locals.var_xx) + (assign1760_e1932 * locals.var_xx_dn11)) * locals.var_yy) + (assign1760_e1934 * locals.var_yy_dn11)) * locals.var_vdc_zener) * locals.var_inv_vdc_zener_t) + (assign1760_e1938 * locals.var_inv_vdc_zener_t_dn11)) * locals.var_inv_vgzcb_tr) * locals.var_inv_vgzcb_tr);
        locals.var_nzcb_t_rv = 0.0;

        let assign1780_e1965: f64 = (locals.var_lntn * p.p96);
        let assign1780_e1966: f64 = (assign1780_e1965).exp();
        locals.var_x = assign1780_e1966;
        locals.var_x_dn0 = 0.0;
        locals.var_x_dn1 = 0.0;
        locals.var_x_dn3 = 0.0;
        locals.var_x_dn4 = (assign1780_e1966 * (locals.var_lntn_dn4 * p.p96));
        locals.var_x_dn5 = 0.0;
        locals.var_x_dn6 = 0.0;
        locals.var_x_dn7 = 0.0;
        locals.var_x_dn8 = 0.0;
        locals.var_x_dn9 = 0.0;
        locals.var_x_dn10 = 0.0;
        locals.var_x_dn11 = 0.0;
        locals.var_x_rv = 0.0;

        let assign1790_e1969: f64 = (p.p14 * locals.var_x);
        let assign1790_e1971: f64 = (assign1790_e1969 * locals.var_cjc_scale_inv);
        locals.var_vef_t = assign1790_e1971;
        locals.var_vef_t_dn0 = (((p.p14 * locals.var_x_dn0) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn0));
        locals.var_vef_t_dn1 = (((p.p14 * locals.var_x_dn1) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn1));
        locals.var_vef_t_dn3 = (((p.p14 * locals.var_x_dn3) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn3));
        locals.var_vef_t_dn4 = (((p.p14 * locals.var_x_dn4) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn4));
        locals.var_vef_t_dn5 = (((p.p14 * locals.var_x_dn5) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn5));
        locals.var_vef_t_dn6 = (((p.p14 * locals.var_x_dn6) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn6));
        locals.var_vef_t_dn7 = (((p.p14 * locals.var_x_dn7) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn7));
        locals.var_vef_t_dn8 = (((p.p14 * locals.var_x_dn8) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn8));
        locals.var_vef_t_dn9 = (((p.p14 * locals.var_x_dn9) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn9));
        locals.var_vef_t_dn10 = (((p.p14 * locals.var_x_dn10) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn10));
        locals.var_vef_t_dn11 = (((p.p14 * locals.var_x_dn11) * locals.var_cjc_scale_inv) + (assign1790_e1969 * locals.var_cjc_scale_inv_dn11));
        locals.var_vef_t_rv = 0.0;

        let assign1800_e1974: f64 = (p.p13 * locals.var_x);
        let assign1800_e1976: f64 = (assign1800_e1974 * locals.var_y);
        locals.var_ver_t = assign1800_e1976;
        locals.var_ver_t_dn0 = (((p.p13 * locals.var_x_dn0) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn0));
        locals.var_ver_t_dn1 = (((p.p13 * locals.var_x_dn1) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn1));
        locals.var_ver_t_dn3 = (((p.p13 * locals.var_x_dn3) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn3));
        locals.var_ver_t_dn4 = (((p.p13 * locals.var_x_dn4) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn4));
        locals.var_ver_t_dn5 = (((p.p13 * locals.var_x_dn5) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn5));
        locals.var_ver_t_dn6 = (((p.p13 * locals.var_x_dn6) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn6));
        locals.var_ver_t_dn7 = (((p.p13 * locals.var_x_dn7) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn7));
        locals.var_ver_t_dn8 = (((p.p13 * locals.var_x_dn8) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn8));
        locals.var_ver_t_dn9 = (((p.p13 * locals.var_x_dn9) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn9));
        locals.var_ver_t_dn10 = (((p.p13 * locals.var_x_dn10) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn10));
        locals.var_ver_t_dn11 = (((p.p13 * locals.var_x_dn11) * locals.var_y) + (assign1800_e1974 * locals.var_y_dn11));
        locals.var_ver_t_rv = 0.0;

        let assign1810_e1981: f64 = (4.0 - p.p141);
        let assign1810_e1982: f64 = (locals.var_lntn * assign1810_e1981);
        let assign1810_e1983: f64 = (assign1810_e1982).exp();
        let assign1810_e1984: f64 = (p.p133 * assign1810_e1983);
        let assign1810_e1986: f64 = (-p.p140);
        let assign1810_e1988: f64 = (assign1810_e1986 * locals.var_vdtinv);
        let assign1810_e1989: f64 = (assign1810_e1988).exp();
        let assign1810_e1990: f64 = (assign1810_e1984 * assign1810_e1989);
        locals.var_iss_t = assign1810_e1990;
        locals.var_iss_t_dn4 = (((p.p133 * (assign1810_e1983 * (locals.var_lntn_dn4 * assign1810_e1981))) * assign1810_e1989) + (assign1810_e1984 * (assign1810_e1989 * (assign1810_e1986 * locals.var_vdtinv_dn4))));
        locals.var_iss_t_rv = 0.0;

        let assign1830_e2011: f64 = (1.0 - p.p141);
        let assign1830_e2012: f64 = (locals.var_lntn * assign1830_e2011);
        let assign1830_e2013: f64 = (assign1830_e2012).exp();
        let assign1830_e2014: f64 = (p.p135 * assign1830_e2013);
        locals.var_iks_t = assign1830_e2014;
        locals.var_iks_t_dn4 = (p.p135 * (assign1830_e2013 * (locals.var_lntn_dn4 * assign1830_e2011)));
        locals.var_iks_t_rv = 0.0;

        let assign1850_e2027: f64 = (p.p98 - 2.0);
        let assign1850_e2028: f64 = (locals.var_lntn * assign1850_e2027);
        let assign1850_e2029: f64 = (assign1850_e2028).exp();
        let assign1850_e2030: f64 = (p.p86 * assign1850_e2029);
        let assign1850_e2032: f64 = (-p.p120);
        let assign1850_e2034: f64 = (assign1850_e2032 * locals.var_vdtinv);
        let assign1850_e2035: f64 = (assign1850_e2034).exp();
        let assign1850_e2036: f64 = (assign1850_e2030 * assign1850_e2035);
        locals.var_taue_t = assign1850_e2036;
        locals.var_taue_t_dn4 = (((p.p86 * (assign1850_e2029 * (locals.var_lntn_dn4 * assign1850_e2027))) * assign1850_e2035) + (assign1850_e2030 * (assign1850_e2035 * (assign1850_e2032 * locals.var_vdtinv_dn4))));
        locals.var_taue_t_rv = 0.0;

        let assign1860_e2041: f64 = (p.p96 + p.p98);
        let assign1860_e2043: f64 = (assign1860_e2041 - 1.0);
        let assign1860_e2044: f64 = (locals.var_lntn * assign1860_e2043);
        let assign1860_e2045: f64 = (assign1860_e2044).exp();
        let assign1860_e2046: f64 = (p.p87 * assign1860_e2045);
        locals.var_taub_t = assign1860_e2046;
        locals.var_taub_t_dn4 = (p.p87 * (assign1860_e2045 * (locals.var_lntn_dn4 * assign1860_e2043)));
        locals.var_taub_t_rv = 0.0;

        let assign1870_e2051: f64 = (p.p99 - 1.0);
        let assign1870_e2052: f64 = (locals.var_lntn * assign1870_e2051);
        let assign1870_e2053: f64 = (assign1870_e2052).exp();
        let assign1870_e2054: f64 = (p.p88 * assign1870_e2053);
        locals.var_tepi_t = assign1870_e2054;
        locals.var_tepi_t_dn4 = (p.p88 * (assign1870_e2053 * (locals.var_lntn_dn4 * assign1870_e2051)));
        locals.var_tepi_t_rv = 0.0;

        let assign1880_e2058: f64 = (locals.var_taub_t + locals.var_tepi_t);
        let assign1880_e2059: f64 = (p.p89 * assign1880_e2058);
        let assign1880_e2062: f64 = (p.p87 + p.p88);
        let assign1880_e2063: f64 = (assign1880_e2059 / assign1880_e2062);
        locals.var_taur_t = assign1880_e2063;
        locals.var_taur_t_dn4 = ((p.p89 * (locals.var_taub_t_dn4 + locals.var_tepi_t_dn4)) / assign1880_e2062);
        locals.var_taur_t_rv = 0.0;

        let assign1890_e2068: f64 = (p.p100 - 1.0);
        let assign1890_e2069: f64 = (locals.var_lntn * assign1890_e2068);
        let assign1890_e2070: f64 = (assign1890_e2069).exp();
        let assign1890_e2071: f64 = (p.p90 * assign1890_e2070);
        locals.var_tauex_t = assign1890_e2071;
        locals.var_tauex_t_dn4 = (p.p90 * (assign1890_e2070 * (locals.var_lntn_dn4 * assign1890_e2068)));
        locals.var_tauex_t_rv = 0.0;

        let assign1900_e2074: f64 = (locals.var_tk - 300.0);
        locals.var_tk300 = assign1900_e2074;
        locals.var_tk300_dn4 = locals.var_tk_dn4;
        locals.var_tk300_rv = 0.0;

        let assign1910_e2077: f64 = if locals.var_tk < 525.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign1910_e2077;
        locals.var_guard25_rv = 0.0;

        let (assign1920_e2093, assign1920_e2093_d_n4,) = {
    if (locals.var_guard25 != 0.0) {
        let assign1920_e2083: f64 = (0.00072 * locals.var_tk300);
        let assign1920_e2084: f64 = (1.0 + assign1920_e2083);
        let assign1920_e2087: f64 = (1.6e-6 * locals.var_tk300);
        let assign1920_e2089: f64 = (assign1920_e2087 * locals.var_tk300);
        let assign1920_e2090: f64 = (assign1920_e2084 - assign1920_e2089);
        let assign1920_e2091: f64 = (locals.var_bn * assign1920_e2090);
        (assign1920_e2091, (locals.var_bn * ((0.00072 * locals.var_tk300_dn4) - (((1.6e-6 * locals.var_tk300_dn4) * locals.var_tk300) + (assign1920_e2087 * locals.var_tk300_dn4)))),)
    } else {
        (locals.var_bnt, locals.var_bnt_dn4,)
    }
};
        locals.var_bnt = assign1920_e2093;
        locals.var_bnt_dn4 = assign1920_e2093_d_n4;
        locals.var_bnt_rv = 0.0;

        let (assign1930_e2100, assign1930_e2100_d_n4,) = {
    if (locals.var_guard25 == 0.0) {
        let assign1930_e2098: f64 = (locals.var_bn * 1.081);
        (assign1930_e2098, 0.0,)
    } else {
        (locals.var_bnt, locals.var_bnt_dn4,)
    }
};
        locals.var_bnt = assign1930_e2100;
        locals.var_bnt_dn4 = assign1930_e2100_d_n4;
        locals.var_bnt_rv = 0.0;

        let assign1940_e2104: f64 = (locals.var_lntn * p.p96);
        let assign1940_e2105: f64 = (assign1940_e2104).exp();
        let assign1940_e2106: f64 = (p.p92 * assign1940_e2105);
        locals.var_deg_t = assign1940_e2106;
        locals.var_deg_t_dn4 = (p.p92 * (assign1940_e2105 * (locals.var_lntn_dn4 * p.p96)));
        locals.var_deg_t_rv = 0.0;

        let assign2110_e2185: f64 = (p.p3 * (nv7 - nv8));
        locals.var_vb2c1 = assign2110_e2185;
        locals.var_vb2c1_dn7 = p.p3;
        locals.var_vb2c1_dn8 = (-p.p3);
        locals.var_vb2c1_rv = 0.0;

        let assign2120_e2188: f64 = (p.p3 * (nv7 - nv9));
        locals.var_vb2c2 = assign2120_e2188;
        locals.var_vb2c2_dn7 = p.p3;
        locals.var_vb2c2_dn9 = (-p.p3);
        locals.var_vb2c2_rv = 0.0;

        let assign2130_e2191: f64 = (p.p3 * (nv7 - nv5));
        locals.var_vb2e1 = assign2130_e2191;
        locals.var_vb2e1_dn5 = (-p.p3);
        locals.var_vb2e1_dn7 = p.p3;
        locals.var_vb2e1_rv = 0.0;

        let assign2140_e2194: f64 = (p.p3 * (nv6 - nv5));
        locals.var_vb1e1 = assign2140_e2194;
        locals.var_vb1e1_dn5 = (-p.p3);
        locals.var_vb1e1_dn6 = p.p3;
        locals.var_vb1e1_rv = 0.0;

        let assign2150_e2197: f64 = (p.p3 * (nv6 - nv7));
        locals.var_vb1b2 = assign2150_e2197;
        locals.var_vb1b2_dn6 = p.p3;
        locals.var_vb1b2_dn7 = (-p.p3);
        locals.var_vb1b2_rv = 0.0;

        let assign2160_e2200: f64 = (p.p3 * (nv3 - nv8));
        locals.var_vsc1 = assign2160_e2200;
        locals.var_vsc1_dn3 = p.p3;
        locals.var_vsc1_dn8 = (-p.p3);
        locals.var_vsc1_rv = 0.0;

        let assign2170_e2203: f64 = (p.p3 * (nv8 - nv9));
        locals.var_vc1c2 = assign2170_e2203;
        locals.var_vc1c2_dn8 = p.p3;
        locals.var_vc1c2_dn9 = (-p.p3);
        locals.var_vc1c2_rv = 0.0;

        let assign2190_e2209: f64 = (p.p3 * (nv1 - nv6));
        locals.var_vbb1 = assign2190_e2209;
        locals.var_vbb1_dn1 = p.p3;
        locals.var_vbb1_dn6 = (-p.p3);
        locals.var_vbb1_rv = 0.0;

        let assign2200_e2212: f64 = (p.p3 * (nv1 - nv2));
        locals.var_vbe = assign2200_e2212;
        locals.var_vbe_dn1 = p.p3;
        locals.var_vbe_dn2 = (-p.p3);
        locals.var_vbe_rv = 0.0;

        let assign2210_e2215: f64 = (p.p3 * (nv1 - nv0));
        locals.var_vbc = assign2210_e2215;
        locals.var_vbc_dn0 = (-p.p3);
        locals.var_vbc_dn1 = p.p3;
        locals.var_vbc_rv = 0.0;

        let assign2220_e2218: f64 = (p.p3 * (nv11 - nv8));
        locals.var_vc4c1 = assign2220_e2218;
        locals.var_vc4c1_dn8 = (-p.p3);
        locals.var_vc4c1_dn11 = p.p3;
        locals.var_vc4c1_rv = 0.0;

        let assign2230_e2221: f64 = (p.p3 * (nv10 - nv11));
        locals.var_vc3c4 = assign2230_e2221;
        locals.var_vc3c4_dn10 = p.p3;
        locals.var_vc3c4_dn11 = (-p.p3);
        locals.var_vc3c4_rv = 0.0;

        let assign2240_e2224: f64 = (locals.var_vb1b2 + locals.var_vb2c2);
        let assign2240_e2226: f64 = (assign2240_e2224 - locals.var_vc1c2);
        let assign2240_e2228: f64 = (assign2240_e2226 - locals.var_vc4c1);
        locals.var_vb1c4 = assign2240_e2228;
        locals.var_vb1c4_dn6 = locals.var_vb1b2_dn6;
        locals.var_vb1c4_dn7 = (locals.var_vb1b2_dn7 + locals.var_vb2c2_dn7);
        locals.var_vb1c4_dn8 = ((-locals.var_vc1c2_dn8) - locals.var_vc4c1_dn8);
        locals.var_vb1c4_dn9 = (locals.var_vb2c2_dn9 - locals.var_vc1c2_dn9);
        locals.var_vb1c4_dn11 = (-locals.var_vc4c1_dn11);
        locals.var_vb1c4_rv = 0.0;

        let assign2250_e2230: f64 = (-locals.var_vbc);
        let assign2250_e2232: f64 = (assign2250_e2230 + locals.var_vbb1);
        let assign2250_e2234: f64 = (assign2250_e2232 + locals.var_vb1c4);
        let assign2250_e2236: f64 = (assign2250_e2234 - locals.var_vc3c4);
        locals.var_vcc3 = assign2250_e2236;
        locals.var_vcc3_dn0 = (-locals.var_vbc_dn0);
        locals.var_vcc3_dn1 = ((-locals.var_vbc_dn1) + locals.var_vbb1_dn1);
        locals.var_vcc3_dn6 = (locals.var_vbb1_dn6 + locals.var_vb1c4_dn6);
        locals.var_vcc3_dn7 = locals.var_vb1c4_dn7;
        locals.var_vcc3_dn8 = locals.var_vb1c4_dn8;
        locals.var_vcc3_dn9 = locals.var_vb1c4_dn9;
        locals.var_vcc3_dn10 = (-locals.var_vc3c4_dn10);
        locals.var_vcc3_dn11 = (locals.var_vb1c4_dn11 - locals.var_vc3c4_dn11);
        locals.var_vcc3_rv = 0.0;

        let assign2260_e2239: f64 = (locals.var_vbc + locals.var_vcc3);
        locals.var_vbc3 = assign2260_e2239;
        locals.var_vbc3_dn0 = (locals.var_vbc_dn0 + locals.var_vcc3_dn0);
        locals.var_vbc3_dn1 = (locals.var_vbc_dn1 + locals.var_vcc3_dn1);
        locals.var_vbc3_dn6 = locals.var_vcc3_dn6;
        locals.var_vbc3_dn7 = locals.var_vcc3_dn7;
        locals.var_vbc3_dn8 = locals.var_vcc3_dn8;
        locals.var_vbc3_dn9 = locals.var_vcc3_dn9;
        locals.var_vbc3_dn10 = locals.var_vcc3_dn10;
        locals.var_vbc3_dn11 = locals.var_vcc3_dn11;
        locals.var_vbc3_rv = 0.0;

        let assign2270_e2242: f64 = (locals.var_vsc1 - locals.var_vc4c1);
        locals.var_vsc4 = assign2270_e2242;
        locals.var_vsc4_dn3 = locals.var_vsc1_dn3;
        locals.var_vsc4_dn8 = (locals.var_vsc1_dn8 - locals.var_vc4c1_dn8);
        locals.var_vsc4_dn11 = (-locals.var_vc4c1_dn11);
        locals.var_vsc4_rv = 0.0;

        let assign2280_e2245: f64 = (locals.var_vsc4 - locals.var_vc3c4);
        locals.var_vsc3 = assign2280_e2245;
        locals.var_vsc3_dn3 = locals.var_vsc4_dn3;
        locals.var_vsc3_dn8 = locals.var_vsc4_dn8;
        locals.var_vsc3_dn10 = (-locals.var_vc3c4_dn10);
        locals.var_vsc3_dn11 = (locals.var_vsc4_dn11 - locals.var_vc3c4_dn11);
        locals.var_vsc3_rv = 0.0;

        let assign2290_e2248: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2290_e2250: f64 = if assign2290_e2248 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign2290_e2250;
        locals.var_guard32_rv = 0.0;

        let (assign2300_e2257, assign2300_e2257_d_n4, assign2300_e2257_d_n7, assign2300_e2257_d_n9,) = {
    if (locals.var_guard32 != 0.0) {
        let assign2300_e2254: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2300_e2255: f64 = (assign2300_e2254).exp();
        (assign2300_e2255, (assign2300_e2255 * (locals.var_vb2c2 * locals.var_vtinv_dn4)), (assign2300_e2255 * (locals.var_vb2c2_dn7 * locals.var_vtinv)), (assign2300_e2255 * (locals.var_vb2c2_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn4, locals.var_evb2c2_dn7, locals.var_evb2c2_dn9,)
    }
};
        locals.var_evb2c2 = assign2300_e2257;
        locals.var_evb2c2_dn4 = assign2300_e2257_d_n4;
        locals.var_evb2c2_dn7 = assign2300_e2257_d_n7;
        locals.var_evb2c2_dn9 = assign2300_e2257_d_n9;
        locals.var_evb2c2_rv = 0.0;

        let (assign2310_e2263,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2310_e2261: f64 = (p.p151).exp();
        (assign2310_e2261,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2310_e2263;
        locals.var_expl_rv = 0.0;

        let (assign2320_e2276, assign2320_e2276_d_n4, assign2320_e2276_d_n7, assign2320_e2276_d_n9,) = {
    if (locals.var_guard32 == 0.0) {
        let assign2320_e2270: f64 = (locals.var_vb2c2 * locals.var_vtinv);
        let assign2320_e2272: f64 = (assign2320_e2270 - p.p151);
        let assign2320_e2273: f64 = (1.0 + assign2320_e2272);
        let assign2320_e2274: f64 = (locals.var_expl * assign2320_e2273);
        (assign2320_e2274, (locals.var_expl * (locals.var_vb2c2 * locals.var_vtinv_dn4)), (locals.var_expl * (locals.var_vb2c2_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb2c2_dn9 * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2, locals.var_evb2c2_dn4, locals.var_evb2c2_dn7, locals.var_evb2c2_dn9,)
    }
};
        locals.var_evb2c2 = assign2320_e2276;
        locals.var_evb2c2_dn4 = assign2320_e2276_d_n4;
        locals.var_evb2c2_dn7 = assign2320_e2276_d_n7;
        locals.var_evb2c2_dn9 = assign2320_e2276_d_n9;
        locals.var_evb2c2_rv = 0.0;

        let assign2330_e2279: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2330_e2281: f64 = (assign2330_e2279 / locals.var_nff_t);
        let assign2330_e2283: f64 = if assign2330_e2281 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign2330_e2283;
        locals.var_guard33_rv = 0.0;

        let (assign2340_e2292, assign2340_e2292_d_n0, assign2340_e2292_d_n1, assign2340_e2292_d_n3, assign2340_e2292_d_n4, assign2340_e2292_d_n5, assign2340_e2292_d_n6, assign2340_e2292_d_n7, assign2340_e2292_d_n8, assign2340_e2292_d_n9, assign2340_e2292_d_n10, assign2340_e2292_d_n11,) = {
    if (locals.var_guard33 != 0.0) {
        let assign2340_e2287: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2340_e2289: f64 = (assign2340_e2287 / locals.var_nff_t);
        let assign2340_e2290: f64 = (assign2340_e2289).exp();
        (assign2340_e2290, (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * ((((locals.var_vb2e1 * locals.var_vtinv_dn4) * locals.var_nff_t) - (assign2340_e2287 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t))), (assign2340_e2290 * ((((locals.var_vb2e1_dn5 * locals.var_vtinv) * locals.var_nff_t) - (assign2340_e2287 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * ((((locals.var_vb2e1_dn7 * locals.var_vtinv) * locals.var_nff_t) - (assign2340_e2287 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t)))), (assign2340_e2290 * (-((assign2340_e2287 * locals.var_nff_t_dn11) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9, locals.var_evb2e1_dn10, locals.var_evb2e1_dn11,)
    }
};
        locals.var_evb2e1 = assign2340_e2292;
        locals.var_evb2e1_dn0 = assign2340_e2292_d_n0;
        locals.var_evb2e1_dn1 = assign2340_e2292_d_n1;
        locals.var_evb2e1_dn3 = assign2340_e2292_d_n3;
        locals.var_evb2e1_dn4 = assign2340_e2292_d_n4;
        locals.var_evb2e1_dn5 = assign2340_e2292_d_n5;
        locals.var_evb2e1_dn6 = assign2340_e2292_d_n6;
        locals.var_evb2e1_dn7 = assign2340_e2292_d_n7;
        locals.var_evb2e1_dn8 = assign2340_e2292_d_n8;
        locals.var_evb2e1_dn9 = assign2340_e2292_d_n9;
        locals.var_evb2e1_dn10 = assign2340_e2292_d_n10;
        locals.var_evb2e1_dn11 = assign2340_e2292_d_n11;
        locals.var_evb2e1_rv = 0.0;

        let (assign2350_e2298,) = {
    if (locals.var_guard33 == 0.0) {
        let assign2350_e2296: f64 = (p.p151).exp();
        (assign2350_e2296,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2350_e2298;
        locals.var_expl_rv = 0.0;

        let (assign2360_e2313, assign2360_e2313_d_n0, assign2360_e2313_d_n1, assign2360_e2313_d_n3, assign2360_e2313_d_n4, assign2360_e2313_d_n5, assign2360_e2313_d_n6, assign2360_e2313_d_n7, assign2360_e2313_d_n8, assign2360_e2313_d_n9, assign2360_e2313_d_n10, assign2360_e2313_d_n11,) = {
    if (locals.var_guard33 == 0.0) {
        let assign2360_e2305: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign2360_e2307: f64 = (assign2360_e2305 / locals.var_nff_t);
        let assign2360_e2309: f64 = (assign2360_e2307 - p.p151);
        let assign2360_e2310: f64 = (1.0 + assign2360_e2309);
        let assign2360_e2311: f64 = (locals.var_expl * assign2360_e2310);
        (assign2360_e2311, (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn0) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn1) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn3) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1 * locals.var_vtinv_dn4) * locals.var_nff_t) - (assign2360_e2305 * locals.var_nff_t_dn4)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * ((((locals.var_vb2e1_dn5 * locals.var_vtinv) * locals.var_nff_t) - (assign2360_e2305 * locals.var_nff_t_dn5)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn6) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * ((((locals.var_vb2e1_dn7 * locals.var_vtinv) * locals.var_nff_t) - (assign2360_e2305 * locals.var_nff_t_dn7)) / (locals.var_nff_t * locals.var_nff_t))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn8) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn9) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn10) / (locals.var_nff_t * locals.var_nff_t)))), (locals.var_expl * (-((assign2360_e2305 * locals.var_nff_t_dn11) / (locals.var_nff_t * locals.var_nff_t)))),)
    } else {
        (locals.var_evb2e1, locals.var_evb2e1_dn0, locals.var_evb2e1_dn1, locals.var_evb2e1_dn3, locals.var_evb2e1_dn4, locals.var_evb2e1_dn5, locals.var_evb2e1_dn6, locals.var_evb2e1_dn7, locals.var_evb2e1_dn8, locals.var_evb2e1_dn9, locals.var_evb2e1_dn10, locals.var_evb2e1_dn11,)
    }
};
        locals.var_evb2e1 = assign2360_e2313;
        locals.var_evb2e1_dn0 = assign2360_e2313_d_n0;
        locals.var_evb2e1_dn1 = assign2360_e2313_d_n1;
        locals.var_evb2e1_dn3 = assign2360_e2313_d_n3;
        locals.var_evb2e1_dn4 = assign2360_e2313_d_n4;
        locals.var_evb2e1_dn5 = assign2360_e2313_d_n5;
        locals.var_evb2e1_dn6 = assign2360_e2313_d_n6;
        locals.var_evb2e1_dn7 = assign2360_e2313_d_n7;
        locals.var_evb2e1_dn8 = assign2360_e2313_d_n8;
        locals.var_evb2e1_dn9 = assign2360_e2313_d_n9;
        locals.var_evb2e1_dn10 = assign2360_e2313_d_n10;
        locals.var_evb2e1_dn11 = assign2360_e2313_d_n11;
        locals.var_evb2e1_rv = 0.0;

        let assign2370_e2316: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2370_e2318: f64 = if assign2370_e2316 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign2370_e2318;
        locals.var_guard34_rv = 0.0;

        let (assign2380_e2325, assign2380_e2325_d_n4, assign2380_e2325_d_n6, assign2380_e2325_d_n7, assign2380_e2325_d_n8, assign2380_e2325_d_n9, assign2380_e2325_d_n11,) = {
    if (locals.var_guard34 != 0.0) {
        let assign2380_e2322: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2380_e2323: f64 = (assign2380_e2322).exp();
        (assign2380_e2323, (assign2380_e2323 * (locals.var_vb1c4 * locals.var_vtinv_dn4)), (assign2380_e2323 * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (assign2380_e2323 * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (assign2380_e2323 * (locals.var_vb1c4_dn8 * locals.var_vtinv)), (assign2380_e2323 * (locals.var_vb1c4_dn9 * locals.var_vtinv)), (assign2380_e2323 * (locals.var_vb1c4_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn4, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn8, locals.var_evb1c4_dn9, locals.var_evb1c4_dn11,)
    }
};
        locals.var_evb1c4 = assign2380_e2325;
        locals.var_evb1c4_dn4 = assign2380_e2325_d_n4;
        locals.var_evb1c4_dn6 = assign2380_e2325_d_n6;
        locals.var_evb1c4_dn7 = assign2380_e2325_d_n7;
        locals.var_evb1c4_dn8 = assign2380_e2325_d_n8;
        locals.var_evb1c4_dn9 = assign2380_e2325_d_n9;
        locals.var_evb1c4_dn11 = assign2380_e2325_d_n11;
        locals.var_evb1c4_rv = 0.0;

        let (assign2390_e2331,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2390_e2329: f64 = (p.p151).exp();
        (assign2390_e2329,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2390_e2331;
        locals.var_expl_rv = 0.0;

        let (assign2400_e2344, assign2400_e2344_d_n4, assign2400_e2344_d_n6, assign2400_e2344_d_n7, assign2400_e2344_d_n8, assign2400_e2344_d_n9, assign2400_e2344_d_n11,) = {
    if (locals.var_guard34 == 0.0) {
        let assign2400_e2338: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign2400_e2340: f64 = (assign2400_e2338 - p.p151);
        let assign2400_e2341: f64 = (1.0 + assign2400_e2340);
        let assign2400_e2342: f64 = (locals.var_expl * assign2400_e2341);
        (assign2400_e2342, (locals.var_expl * (locals.var_vb1c4 * locals.var_vtinv_dn4)), (locals.var_expl * (locals.var_vb1c4_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn9 * locals.var_vtinv)), (locals.var_expl * (locals.var_vb1c4_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4, locals.var_evb1c4_dn4, locals.var_evb1c4_dn6, locals.var_evb1c4_dn7, locals.var_evb1c4_dn8, locals.var_evb1c4_dn9, locals.var_evb1c4_dn11,)
    }
};
        locals.var_evb1c4 = assign2400_e2344;
        locals.var_evb1c4_dn4 = assign2400_e2344_d_n4;
        locals.var_evb1c4_dn6 = assign2400_e2344_d_n6;
        locals.var_evb1c4_dn7 = assign2400_e2344_d_n7;
        locals.var_evb1c4_dn8 = assign2400_e2344_d_n8;
        locals.var_evb1c4_dn9 = assign2400_e2344_d_n9;
        locals.var_evb1c4_dn11 = assign2400_e2344_d_n11;
        locals.var_evb1c4_rv = 0.0;

        let assign2410_e2347: f64 = (locals.var_vb1b2 * locals.var_vtinv);
        let assign2410_e2349: f64 = if assign2410_e2347 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign2410_e2349;
        locals.var_guard35_rv = 0.0;

        let (assign2430_e2362,) = {
    if (locals.var_guard35 == 0.0) {
        let assign2430_e2360: f64 = (p.p151).exp();
        (assign2430_e2360,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2430_e2362;
        locals.var_expl_rv = 0.0;

        let assign2450_e2378: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2450_e2380: f64 = if assign2450_e2378 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign2450_e2380;
        locals.var_guard36_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2460_e2387, assign2460_e2387_d_n0, assign2460_e2387_d_n1, assign2460_e2387_d_n4, assign2460_e2387_d_n6, assign2460_e2387_d_n7, assign2460_e2387_d_n8, assign2460_e2387_d_n9, assign2460_e2387_d_n10, assign2460_e2387_d_n11,) = {
    if (locals.var_guard36 != 0.0) {
        let assign2460_e2384: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2460_e2385: f64 = (assign2460_e2384).exp();
        (assign2460_e2385, (assign2460_e2385 * (locals.var_vbc3_dn0 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn1 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3 * locals.var_vtinv_dn4)), (assign2460_e2385 * (locals.var_vbc3_dn6 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn7 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn8 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn9 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn10 * locals.var_vtinv)), (assign2460_e2385 * (locals.var_vbc3_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn4, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9, locals.var_evbc3_dn10, locals.var_evbc3_dn11,)
    }
};
        locals.var_evbc3 = assign2460_e2387;
        locals.var_evbc3_dn0 = assign2460_e2387_d_n0;
        locals.var_evbc3_dn1 = assign2460_e2387_d_n1;
        locals.var_evbc3_dn4 = assign2460_e2387_d_n4;
        locals.var_evbc3_dn6 = assign2460_e2387_d_n6;
        locals.var_evbc3_dn7 = assign2460_e2387_d_n7;
        locals.var_evbc3_dn8 = assign2460_e2387_d_n8;
        locals.var_evbc3_dn9 = assign2460_e2387_d_n9;
        locals.var_evbc3_dn10 = assign2460_e2387_d_n10;
        locals.var_evbc3_dn11 = assign2460_e2387_d_n11;
        locals.var_evbc3_rv = 0.0;

        let (assign2470_e2393,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2470_e2391: f64 = (p.p151).exp();
        (assign2470_e2391,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2470_e2393;
        locals.var_expl_rv = 0.0;

        let (assign2480_e2406, assign2480_e2406_d_n0, assign2480_e2406_d_n1, assign2480_e2406_d_n4, assign2480_e2406_d_n6, assign2480_e2406_d_n7, assign2480_e2406_d_n8, assign2480_e2406_d_n9, assign2480_e2406_d_n10, assign2480_e2406_d_n11,) = {
    if (locals.var_guard36 == 0.0) {
        let assign2480_e2400: f64 = (locals.var_vbc3 * locals.var_vtinv);
        let assign2480_e2402: f64 = (assign2480_e2400 - p.p151);
        let assign2480_e2403: f64 = (1.0 + assign2480_e2402);
        let assign2480_e2404: f64 = (locals.var_expl * assign2480_e2403);
        (assign2480_e2404, (locals.var_expl * (locals.var_vbc3_dn0 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn1 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3 * locals.var_vtinv_dn4)), (locals.var_expl * (locals.var_vbc3_dn6 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn7 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn9 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn10 * locals.var_vtinv)), (locals.var_expl * (locals.var_vbc3_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3, locals.var_evbc3_dn0, locals.var_evbc3_dn1, locals.var_evbc3_dn4, locals.var_evbc3_dn6, locals.var_evbc3_dn7, locals.var_evbc3_dn8, locals.var_evbc3_dn9, locals.var_evbc3_dn10, locals.var_evbc3_dn11,)
    }
};
        locals.var_evbc3 = assign2480_e2406;
        locals.var_evbc3_dn0 = assign2480_e2406_d_n0;
        locals.var_evbc3_dn1 = assign2480_e2406_d_n1;
        locals.var_evbc3_dn4 = assign2480_e2406_d_n4;
        locals.var_evbc3_dn6 = assign2480_e2406_d_n6;
        locals.var_evbc3_dn7 = assign2480_e2406_d_n7;
        locals.var_evbc3_dn8 = assign2480_e2406_d_n8;
        locals.var_evbc3_dn9 = assign2480_e2406_d_n9;
        locals.var_evbc3_dn10 = assign2480_e2406_d_n10;
        locals.var_evbc3_dn11 = assign2480_e2406_d_n11;
        locals.var_evbc3_rv = 0.0;

        let assign2490_e2409: f64 = (locals.var_vsc1 * locals.var_vtinv);
        let assign2490_e2411: f64 = if assign2490_e2409 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign2490_e2411;
        locals.var_guard37_rv = 0.0;

        let (assign2510_e2424,) = {
    if (locals.var_guard37 == 0.0) {
        let assign2510_e2422: f64 = (p.p151).exp();
        (assign2510_e2422,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2510_e2424;
        locals.var_expl_rv = 0.0;

        let assign2530_e2440: f64 = (locals.var_vsc3 * locals.var_vtinv);
        let assign2530_e2442: f64 = if assign2530_e2440 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign2530_e2442;
        locals.var_guard38_rv = 0.0;

        let (assign2540_e2449, assign2540_e2449_d_n3, assign2540_e2449_d_n4, assign2540_e2449_d_n8, assign2540_e2449_d_n10, assign2540_e2449_d_n11,) = {
    if (locals.var_guard38 != 0.0) {
        let assign2540_e2446: f64 = (locals.var_vsc3 * locals.var_vtinv);
        let assign2540_e2447: f64 = (assign2540_e2446).exp();
        (assign2540_e2447, (assign2540_e2447 * (locals.var_vsc3_dn3 * locals.var_vtinv)), (assign2540_e2447 * (locals.var_vsc3 * locals.var_vtinv_dn4)), (assign2540_e2447 * (locals.var_vsc3_dn8 * locals.var_vtinv)), (assign2540_e2447 * (locals.var_vsc3_dn10 * locals.var_vtinv)), (assign2540_e2447 * (locals.var_vsc3_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evsc3, locals.var_evsc3_dn3, locals.var_evsc3_dn4, locals.var_evsc3_dn8, locals.var_evsc3_dn10, locals.var_evsc3_dn11,)
    }
};
        locals.var_evsc3 = assign2540_e2449;
        locals.var_evsc3_dn3 = assign2540_e2449_d_n3;
        locals.var_evsc3_dn4 = assign2540_e2449_d_n4;
        locals.var_evsc3_dn8 = assign2540_e2449_d_n8;
        locals.var_evsc3_dn10 = assign2540_e2449_d_n10;
        locals.var_evsc3_dn11 = assign2540_e2449_d_n11;
        locals.var_evsc3_rv = 0.0;

        let (assign2550_e2455,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2550_e2453: f64 = (p.p151).exp();
        (assign2550_e2453,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2550_e2455;
        locals.var_expl_rv = 0.0;

        let (assign2560_e2468, assign2560_e2468_d_n3, assign2560_e2468_d_n4, assign2560_e2468_d_n8, assign2560_e2468_d_n10, assign2560_e2468_d_n11,) = {
    if (locals.var_guard38 == 0.0) {
        let assign2560_e2462: f64 = (locals.var_vsc3 * locals.var_vtinv);
        let assign2560_e2464: f64 = (assign2560_e2462 - p.p151);
        let assign2560_e2465: f64 = (1.0 + assign2560_e2464);
        let assign2560_e2466: f64 = (locals.var_expl * assign2560_e2465);
        (assign2560_e2466, (locals.var_expl * (locals.var_vsc3_dn3 * locals.var_vtinv)), (locals.var_expl * (locals.var_vsc3 * locals.var_vtinv_dn4)), (locals.var_expl * (locals.var_vsc3_dn8 * locals.var_vtinv)), (locals.var_expl * (locals.var_vsc3_dn10 * locals.var_vtinv)), (locals.var_expl * (locals.var_vsc3_dn11 * locals.var_vtinv)),)
    } else {
        (locals.var_evsc3, locals.var_evsc3_dn3, locals.var_evsc3_dn4, locals.var_evsc3_dn8, locals.var_evsc3_dn10, locals.var_evsc3_dn11,)
    }
};
        locals.var_evsc3 = assign2560_e2468;
        locals.var_evsc3_dn3 = assign2560_e2468_d_n3;
        locals.var_evsc3_dn4 = assign2560_e2468_d_n4;
        locals.var_evsc3_dn8 = assign2560_e2468_d_n8;
        locals.var_evsc3_dn10 = assign2560_e2468_d_n10;
        locals.var_evsc3_dn11 = assign2560_e2468_d_n11;
        locals.var_evsc3_rv = 0.0;

        let assign2570_e2471: f64 = (locals.var_vsc4 * locals.var_vtinv);
        let assign2570_e2473: f64 = if assign2570_e2471 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign2570_e2473;
        locals.var_guard39_rv = 0.0;

        let (assign2590_e2486,) = {
    if (locals.var_guard39 == 0.0) {
        let assign2590_e2484: f64 = (p.p151).exp();
        (assign2590_e2484,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2590_e2486;
        locals.var_expl_rv = 0.0;

        let assign2610_e2502: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2610_e2504: f64 = (assign2610_e2502 * locals.var_vtinv);
        let assign2610_e2506: f64 = if assign2610_e2504 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign2610_e2506;
        locals.var_guard40_rv = 0.0;

        let (assign2620_e2515, assign2620_e2515_d_n0, assign2620_e2515_d_n1, assign2620_e2515_d_n3, assign2620_e2515_d_n4, assign2620_e2515_d_n5, assign2620_e2515_d_n6, assign2620_e2515_d_n7, assign2620_e2515_d_n8, assign2620_e2515_d_n9, assign2620_e2515_d_n10, assign2620_e2515_d_n11,) = {
    if (locals.var_guard40 != 0.0) {
        let assign2620_e2510: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2620_e2512: f64 = (assign2620_e2510 * locals.var_vtinv);
        let assign2620_e2513: f64 = (assign2620_e2512).exp();
        (assign2620_e2513, (assign2620_e2513 * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2620_e2513 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2620_e2513 * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2620_e2510 * locals.var_vtinv_dn4))), (assign2620_e2513 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)), (assign2620_e2513 * ((locals.var_vbc3_dn11 - locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9, locals.var_evbc3vdc_dn10, locals.var_evbc3vdc_dn11,)
    }
};
        locals.var_evbc3vdc = assign2620_e2515;
        locals.var_evbc3vdc_dn0 = assign2620_e2515_d_n0;
        locals.var_evbc3vdc_dn1 = assign2620_e2515_d_n1;
        locals.var_evbc3vdc_dn3 = assign2620_e2515_d_n3;
        locals.var_evbc3vdc_dn4 = assign2620_e2515_d_n4;
        locals.var_evbc3vdc_dn5 = assign2620_e2515_d_n5;
        locals.var_evbc3vdc_dn6 = assign2620_e2515_d_n6;
        locals.var_evbc3vdc_dn7 = assign2620_e2515_d_n7;
        locals.var_evbc3vdc_dn8 = assign2620_e2515_d_n8;
        locals.var_evbc3vdc_dn9 = assign2620_e2515_d_n9;
        locals.var_evbc3vdc_dn10 = assign2620_e2515_d_n10;
        locals.var_evbc3vdc_dn11 = assign2620_e2515_d_n11;
        locals.var_evbc3vdc_rv = 0.0;

        let (assign2630_e2521,) = {
    if (locals.var_guard40 == 0.0) {
        let assign2630_e2519: f64 = (p.p151).exp();
        (assign2630_e2519,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2630_e2521;
        locals.var_expl_rv = 0.0;

        let (assign2640_e2536, assign2640_e2536_d_n0, assign2640_e2536_d_n1, assign2640_e2536_d_n3, assign2640_e2536_d_n4, assign2640_e2536_d_n5, assign2640_e2536_d_n6, assign2640_e2536_d_n7, assign2640_e2536_d_n8, assign2640_e2536_d_n9, assign2640_e2536_d_n10, assign2640_e2536_d_n11,) = {
    if (locals.var_guard40 == 0.0) {
        let assign2640_e2528: f64 = (locals.var_vbc3 - locals.var_vdc_t);
        let assign2640_e2530: f64 = (assign2640_e2528 * locals.var_vtinv);
        let assign2640_e2532: f64 = (assign2640_e2530 - p.p151);
        let assign2640_e2533: f64 = (1.0 + assign2640_e2532);
        let assign2640_e2534: f64 = (locals.var_expl * assign2640_e2533);
        (assign2640_e2534, (locals.var_expl * ((locals.var_vbc3_dn0 - locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn1 - locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2640_e2528 * locals.var_vtinv_dn4))), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn10 - locals.var_vdc_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vbc3_dn11 - locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evbc3vdc, locals.var_evbc3vdc_dn0, locals.var_evbc3vdc_dn1, locals.var_evbc3vdc_dn3, locals.var_evbc3vdc_dn4, locals.var_evbc3vdc_dn5, locals.var_evbc3vdc_dn6, locals.var_evbc3vdc_dn7, locals.var_evbc3vdc_dn8, locals.var_evbc3vdc_dn9, locals.var_evbc3vdc_dn10, locals.var_evbc3vdc_dn11,)
    }
};
        locals.var_evbc3vdc = assign2640_e2536;
        locals.var_evbc3vdc_dn0 = assign2640_e2536_d_n0;
        locals.var_evbc3vdc_dn1 = assign2640_e2536_d_n1;
        locals.var_evbc3vdc_dn3 = assign2640_e2536_d_n3;
        locals.var_evbc3vdc_dn4 = assign2640_e2536_d_n4;
        locals.var_evbc3vdc_dn5 = assign2640_e2536_d_n5;
        locals.var_evbc3vdc_dn6 = assign2640_e2536_d_n6;
        locals.var_evbc3vdc_dn7 = assign2640_e2536_d_n7;
        locals.var_evbc3vdc_dn8 = assign2640_e2536_d_n8;
        locals.var_evbc3vdc_dn9 = assign2640_e2536_d_n9;
        locals.var_evbc3vdc_dn10 = assign2640_e2536_d_n10;
        locals.var_evbc3vdc_dn11 = assign2640_e2536_d_n11;
        locals.var_evbc3vdc_rv = 0.0;

        let assign2650_e2539: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2650_e2541: f64 = (assign2650_e2539 * locals.var_vtinv);
        let assign2650_e2543: f64 = if assign2650_e2541 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign2650_e2543;
        locals.var_guard41_rv = 0.0;

        let (assign2660_e2552, assign2660_e2552_d_n0, assign2660_e2552_d_n1, assign2660_e2552_d_n3, assign2660_e2552_d_n4, assign2660_e2552_d_n5, assign2660_e2552_d_n6, assign2660_e2552_d_n7, assign2660_e2552_d_n8, assign2660_e2552_d_n9, assign2660_e2552_d_n10, assign2660_e2552_d_n11,) = {
    if (locals.var_guard41 != 0.0) {
        let assign2660_e2547: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2660_e2549: f64 = (assign2660_e2547 * locals.var_vtinv);
        let assign2660_e2550: f64 = (assign2660_e2549).exp();
        (assign2660_e2550, (assign2660_e2550 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2660_e2550 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2660_e2550 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2660_e2550 * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2660_e2547 * locals.var_vtinv_dn4))), (assign2660_e2550 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2660_e2550 * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2660_e2550 * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2660_e2550 * ((locals.var_vb1c4_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2660_e2550 * ((locals.var_vb1c4_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2660_e2550 * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (assign2660_e2550 * ((locals.var_vb1c4_dn11 - locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9, locals.var_evb1c4vdc_dn10, locals.var_evb1c4vdc_dn11,)
    }
};
        locals.var_evb1c4vdc = assign2660_e2552;
        locals.var_evb1c4vdc_dn0 = assign2660_e2552_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2660_e2552_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2660_e2552_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2660_e2552_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2660_e2552_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2660_e2552_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2660_e2552_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2660_e2552_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2660_e2552_d_n9;
        locals.var_evb1c4vdc_dn10 = assign2660_e2552_d_n10;
        locals.var_evb1c4vdc_dn11 = assign2660_e2552_d_n11;
        locals.var_evb1c4vdc_rv = 0.0;

        let (assign2670_e2558,) = {
    if (locals.var_guard41 == 0.0) {
        let assign2670_e2556: f64 = (p.p151).exp();
        (assign2670_e2556,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2670_e2558;
        locals.var_expl_rv = 0.0;

        let (assign2680_e2573, assign2680_e2573_d_n0, assign2680_e2573_d_n1, assign2680_e2573_d_n3, assign2680_e2573_d_n4, assign2680_e2573_d_n5, assign2680_e2573_d_n6, assign2680_e2573_d_n7, assign2680_e2573_d_n8, assign2680_e2573_d_n9, assign2680_e2573_d_n10, assign2680_e2573_d_n11,) = {
    if (locals.var_guard41 == 0.0) {
        let assign2680_e2565: f64 = (locals.var_vb1c4 - locals.var_vdc_t);
        let assign2680_e2567: f64 = (assign2680_e2565 * locals.var_vtinv);
        let assign2680_e2569: f64 = (assign2680_e2567 - p.p151);
        let assign2680_e2570: f64 = (1.0 + assign2680_e2569);
        let assign2680_e2571: f64 = (locals.var_expl * assign2680_e2570);
        (assign2680_e2571, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2680_e2565 * locals.var_vtinv_dn4))), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn6 - locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1c4_dn11 - locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb1c4vdc, locals.var_evb1c4vdc_dn0, locals.var_evb1c4vdc_dn1, locals.var_evb1c4vdc_dn3, locals.var_evb1c4vdc_dn4, locals.var_evb1c4vdc_dn5, locals.var_evb1c4vdc_dn6, locals.var_evb1c4vdc_dn7, locals.var_evb1c4vdc_dn8, locals.var_evb1c4vdc_dn9, locals.var_evb1c4vdc_dn10, locals.var_evb1c4vdc_dn11,)
    }
};
        locals.var_evb1c4vdc = assign2680_e2573;
        locals.var_evb1c4vdc_dn0 = assign2680_e2573_d_n0;
        locals.var_evb1c4vdc_dn1 = assign2680_e2573_d_n1;
        locals.var_evb1c4vdc_dn3 = assign2680_e2573_d_n3;
        locals.var_evb1c4vdc_dn4 = assign2680_e2573_d_n4;
        locals.var_evb1c4vdc_dn5 = assign2680_e2573_d_n5;
        locals.var_evb1c4vdc_dn6 = assign2680_e2573_d_n6;
        locals.var_evb1c4vdc_dn7 = assign2680_e2573_d_n7;
        locals.var_evb1c4vdc_dn8 = assign2680_e2573_d_n8;
        locals.var_evb1c4vdc_dn9 = assign2680_e2573_d_n9;
        locals.var_evb1c4vdc_dn10 = assign2680_e2573_d_n10;
        locals.var_evb1c4vdc_dn11 = assign2680_e2573_d_n11;
        locals.var_evb1c4vdc_rv = 0.0;

        let assign2690_e2576: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2690_e2578: f64 = (assign2690_e2576 * locals.var_vtinv);
        let assign2690_e2580: f64 = if assign2690_e2578 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign2690_e2580;
        locals.var_guard42_rv = 0.0;

        let (assign2700_e2589, assign2700_e2589_d_n0, assign2700_e2589_d_n1, assign2700_e2589_d_n3, assign2700_e2589_d_n4, assign2700_e2589_d_n5, assign2700_e2589_d_n6, assign2700_e2589_d_n7, assign2700_e2589_d_n8, assign2700_e2589_d_n9, assign2700_e2589_d_n10, assign2700_e2589_d_n11,) = {
    if (locals.var_guard42 != 0.0) {
        let assign2700_e2584: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2700_e2586: f64 = (assign2700_e2584 * locals.var_vtinv);
        let assign2700_e2587: f64 = (assign2700_e2586).exp();
        (assign2700_e2587, (assign2700_e2587 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2700_e2587 * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2700_e2584 * locals.var_vtinv_dn4))), (assign2700_e2587 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2700_e2587 * ((locals.var_vb2c2_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2700_e2587 * ((locals.var_vb2c2_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (assign2700_e2587 * ((-locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9, locals.var_evb2c2vdc_dn10, locals.var_evb2c2vdc_dn11,)
    }
};
        locals.var_evb2c2vdc = assign2700_e2589;
        locals.var_evb2c2vdc_dn0 = assign2700_e2589_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2700_e2589_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2700_e2589_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2700_e2589_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2700_e2589_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2700_e2589_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2700_e2589_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2700_e2589_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2700_e2589_d_n9;
        locals.var_evb2c2vdc_dn10 = assign2700_e2589_d_n10;
        locals.var_evb2c2vdc_dn11 = assign2700_e2589_d_n11;
        locals.var_evb2c2vdc_rv = 0.0;

        let (assign2710_e2595,) = {
    if (locals.var_guard42 == 0.0) {
        let assign2710_e2593: f64 = (p.p151).exp();
        (assign2710_e2593,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2710_e2595;
        locals.var_expl_rv = 0.0;

        let (assign2720_e2610, assign2720_e2610_d_n0, assign2720_e2610_d_n1, assign2720_e2610_d_n3, assign2720_e2610_d_n4, assign2720_e2610_d_n5, assign2720_e2610_d_n6, assign2720_e2610_d_n7, assign2720_e2610_d_n8, assign2720_e2610_d_n9, assign2720_e2610_d_n10, assign2720_e2610_d_n11,) = {
    if (locals.var_guard42 == 0.0) {
        let assign2720_e2602: f64 = (locals.var_vb2c2 - locals.var_vdc_t);
        let assign2720_e2604: f64 = (assign2720_e2602 * locals.var_vtinv);
        let assign2720_e2606: f64 = (assign2720_e2604 - p.p151);
        let assign2720_e2607: f64 = (1.0 + assign2720_e2606);
        let assign2720_e2608: f64 = (locals.var_expl * assign2720_e2607);
        (assign2720_e2608, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2720_e2602 * locals.var_vtinv_dn4))), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c2_dn9 - locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c2vdc, locals.var_evb2c2vdc_dn0, locals.var_evb2c2vdc_dn1, locals.var_evb2c2vdc_dn3, locals.var_evb2c2vdc_dn4, locals.var_evb2c2vdc_dn5, locals.var_evb2c2vdc_dn6, locals.var_evb2c2vdc_dn7, locals.var_evb2c2vdc_dn8, locals.var_evb2c2vdc_dn9, locals.var_evb2c2vdc_dn10, locals.var_evb2c2vdc_dn11,)
    }
};
        locals.var_evb2c2vdc = assign2720_e2610;
        locals.var_evb2c2vdc_dn0 = assign2720_e2610_d_n0;
        locals.var_evb2c2vdc_dn1 = assign2720_e2610_d_n1;
        locals.var_evb2c2vdc_dn3 = assign2720_e2610_d_n3;
        locals.var_evb2c2vdc_dn4 = assign2720_e2610_d_n4;
        locals.var_evb2c2vdc_dn5 = assign2720_e2610_d_n5;
        locals.var_evb2c2vdc_dn6 = assign2720_e2610_d_n6;
        locals.var_evb2c2vdc_dn7 = assign2720_e2610_d_n7;
        locals.var_evb2c2vdc_dn8 = assign2720_e2610_d_n8;
        locals.var_evb2c2vdc_dn9 = assign2720_e2610_d_n9;
        locals.var_evb2c2vdc_dn10 = assign2720_e2610_d_n10;
        locals.var_evb2c2vdc_dn11 = assign2720_e2610_d_n11;
        locals.var_evb2c2vdc_rv = 0.0;

        let assign2730_e2613: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2730_e2615: f64 = (assign2730_e2613 * locals.var_vtinv);
        let assign2730_e2617: f64 = if assign2730_e2615 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign2730_e2617;
        locals.var_guard43_rv = 0.0;

        let (assign2740_e2626, assign2740_e2626_d_n0, assign2740_e2626_d_n1, assign2740_e2626_d_n3, assign2740_e2626_d_n4, assign2740_e2626_d_n5, assign2740_e2626_d_n6, assign2740_e2626_d_n7, assign2740_e2626_d_n8, assign2740_e2626_d_n9, assign2740_e2626_d_n10, assign2740_e2626_d_n11,) = {
    if (locals.var_guard43 != 0.0) {
        let assign2740_e2621: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2740_e2623: f64 = (assign2740_e2621 * locals.var_vtinv);
        let assign2740_e2624: f64 = (assign2740_e2623).exp();
        (assign2740_e2624, (assign2740_e2624 * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (assign2740_e2624 * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2740_e2621 * locals.var_vtinv_dn4))), (assign2740_e2624 * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (assign2740_e2624 * ((locals.var_vb2c1_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (assign2740_e2624 * ((locals.var_vb2c1_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (assign2740_e2624 * ((-locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9, locals.var_evb2c1vdc_dn10, locals.var_evb2c1vdc_dn11,)
    }
};
        locals.var_evb2c1vdc = assign2740_e2626;
        locals.var_evb2c1vdc_dn0 = assign2740_e2626_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2740_e2626_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2740_e2626_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2740_e2626_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2740_e2626_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2740_e2626_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2740_e2626_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2740_e2626_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2740_e2626_d_n9;
        locals.var_evb2c1vdc_dn10 = assign2740_e2626_d_n10;
        locals.var_evb2c1vdc_dn11 = assign2740_e2626_d_n11;
        locals.var_evb2c1vdc_rv = 0.0;

        let (assign2750_e2632,) = {
    if (locals.var_guard43 == 0.0) {
        let assign2750_e2630: f64 = (p.p151).exp();
        (assign2750_e2630,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign2750_e2632;
        locals.var_expl_rv = 0.0;

        let (assign2760_e2647, assign2760_e2647_d_n0, assign2760_e2647_d_n1, assign2760_e2647_d_n3, assign2760_e2647_d_n4, assign2760_e2647_d_n5, assign2760_e2647_d_n6, assign2760_e2647_d_n7, assign2760_e2647_d_n8, assign2760_e2647_d_n9, assign2760_e2647_d_n10, assign2760_e2647_d_n11,) = {
    if (locals.var_guard43 == 0.0) {
        let assign2760_e2639: f64 = (locals.var_vb2c1 - locals.var_vdc_t);
        let assign2760_e2641: f64 = (assign2760_e2639 * locals.var_vtinv);
        let assign2760_e2643: f64 = (assign2760_e2641 - p.p151);
        let assign2760_e2644: f64 = (1.0 + assign2760_e2643);
        let assign2760_e2645: f64 = (locals.var_expl * assign2760_e2644);
        (assign2760_e2645, (locals.var_expl * ((-locals.var_vdc_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vdc_t_dn4) * locals.var_vtinv) + (assign2760_e2639 * locals.var_vtinv_dn4))), (locals.var_expl * ((-locals.var_vdc_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn7 - locals.var_vdc_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2c1_dn8 - locals.var_vdc_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vdc_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_evb2c1vdc, locals.var_evb2c1vdc_dn0, locals.var_evb2c1vdc_dn1, locals.var_evb2c1vdc_dn3, locals.var_evb2c1vdc_dn4, locals.var_evb2c1vdc_dn5, locals.var_evb2c1vdc_dn6, locals.var_evb2c1vdc_dn7, locals.var_evb2c1vdc_dn8, locals.var_evb2c1vdc_dn9, locals.var_evb2c1vdc_dn10, locals.var_evb2c1vdc_dn11,)
    }
};
        locals.var_evb2c1vdc = assign2760_e2647;
        locals.var_evb2c1vdc_dn0 = assign2760_e2647_d_n0;
        locals.var_evb2c1vdc_dn1 = assign2760_e2647_d_n1;
        locals.var_evb2c1vdc_dn3 = assign2760_e2647_d_n3;
        locals.var_evb2c1vdc_dn4 = assign2760_e2647_d_n4;
        locals.var_evb2c1vdc_dn5 = assign2760_e2647_d_n5;
        locals.var_evb2c1vdc_dn6 = assign2760_e2647_d_n6;
        locals.var_evb2c1vdc_dn7 = assign2760_e2647_d_n7;
        locals.var_evb2c1vdc_dn8 = assign2760_e2647_d_n8;
        locals.var_evb2c1vdc_dn9 = assign2760_e2647_d_n9;
        locals.var_evb2c1vdc_dn10 = assign2760_e2647_d_n10;
        locals.var_evb2c1vdc_dn11 = assign2760_e2647_d_n11;
        locals.var_evb2c1vdc_rv = 0.0;

        let assign2770_e2651: f64 = (4.0 * locals.var_evb2c2vdc);
        let assign2770_e2652: f64 = (1.0 + assign2770_e2651);
        let assign2770_e2653: f64 = (assign2770_e2652).sqrt();
        locals.var_k0 = assign2770_e2653;
        locals.var_k0_dn0 = ((4.0 * locals.var_evb2c2vdc_dn0) / (2.0 * assign2770_e2653));
        locals.var_k0_dn1 = ((4.0 * locals.var_evb2c2vdc_dn1) / (2.0 * assign2770_e2653));
        locals.var_k0_dn3 = ((4.0 * locals.var_evb2c2vdc_dn3) / (2.0 * assign2770_e2653));
        locals.var_k0_dn4 = ((4.0 * locals.var_evb2c2vdc_dn4) / (2.0 * assign2770_e2653));
        locals.var_k0_dn5 = ((4.0 * locals.var_evb2c2vdc_dn5) / (2.0 * assign2770_e2653));
        locals.var_k0_dn6 = ((4.0 * locals.var_evb2c2vdc_dn6) / (2.0 * assign2770_e2653));
        locals.var_k0_dn7 = ((4.0 * locals.var_evb2c2vdc_dn7) / (2.0 * assign2770_e2653));
        locals.var_k0_dn8 = ((4.0 * locals.var_evb2c2vdc_dn8) / (2.0 * assign2770_e2653));
        locals.var_k0_dn9 = ((4.0 * locals.var_evb2c2vdc_dn9) / (2.0 * assign2770_e2653));
        locals.var_k0_dn10 = ((4.0 * locals.var_evb2c2vdc_dn10) / (2.0 * assign2770_e2653));
        locals.var_k0_dn11 = ((4.0 * locals.var_evb2c2vdc_dn11) / (2.0 * assign2770_e2653));
        locals.var_k0_rv = 0.0;

        let assign2780_e2657: f64 = (4.0 * locals.var_evb2c1vdc);
        let assign2780_e2658: f64 = (1.0 + assign2780_e2657);
        let assign2780_e2659: f64 = (assign2780_e2658).sqrt();
        locals.var_kw = assign2780_e2659;
        locals.var_kw_dn0 = ((4.0 * locals.var_evb2c1vdc_dn0) / (2.0 * assign2780_e2659));
        locals.var_kw_dn1 = ((4.0 * locals.var_evb2c1vdc_dn1) / (2.0 * assign2780_e2659));
        locals.var_kw_dn3 = ((4.0 * locals.var_evb2c1vdc_dn3) / (2.0 * assign2780_e2659));
        locals.var_kw_dn4 = ((4.0 * locals.var_evb2c1vdc_dn4) / (2.0 * assign2780_e2659));
        locals.var_kw_dn5 = ((4.0 * locals.var_evb2c1vdc_dn5) / (2.0 * assign2780_e2659));
        locals.var_kw_dn6 = ((4.0 * locals.var_evb2c1vdc_dn6) / (2.0 * assign2780_e2659));
        locals.var_kw_dn7 = ((4.0 * locals.var_evb2c1vdc_dn7) / (2.0 * assign2780_e2659));
        locals.var_kw_dn8 = ((4.0 * locals.var_evb2c1vdc_dn8) / (2.0 * assign2780_e2659));
        locals.var_kw_dn9 = ((4.0 * locals.var_evb2c1vdc_dn9) / (2.0 * assign2780_e2659));
        locals.var_kw_dn10 = ((4.0 * locals.var_evb2c1vdc_dn10) / (2.0 * assign2780_e2659));
        locals.var_kw_dn11 = ((4.0 * locals.var_evb2c1vdc_dn11) / (2.0 * assign2780_e2659));
        locals.var_kw_rv = 0.0;

        let assign2790_e2662: f64 = (2.0 * locals.var_evb2c1vdc);
        let assign2790_e2665: f64 = (1.0 + locals.var_kw);
        let assign2790_e2666: f64 = (assign2790_e2662 / assign2790_e2665);
        locals.var_pw = assign2790_e2666;
        locals.var_pw_dn0 = ((((2.0 * locals.var_evb2c1vdc_dn0) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn0)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn1 = ((((2.0 * locals.var_evb2c1vdc_dn1) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn1)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn3 = ((((2.0 * locals.var_evb2c1vdc_dn3) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn3)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn4 = ((((2.0 * locals.var_evb2c1vdc_dn4) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn4)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn5 = ((((2.0 * locals.var_evb2c1vdc_dn5) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn5)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn6 = ((((2.0 * locals.var_evb2c1vdc_dn6) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn6)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn7 = ((((2.0 * locals.var_evb2c1vdc_dn7) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn7)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn8 = ((((2.0 * locals.var_evb2c1vdc_dn8) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn8)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn9 = ((((2.0 * locals.var_evb2c1vdc_dn9) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn9)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn10 = ((((2.0 * locals.var_evb2c1vdc_dn10) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn10)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_dn11 = ((((2.0 * locals.var_evb2c1vdc_dn11) * assign2790_e2665) - (assign2790_e2662 * locals.var_kw_dn11)) / (assign2790_e2665 * assign2790_e2665));
        locals.var_pw_rv = 0.0;

        let assign2800_e2669: f64 = if locals.var_pw < p.p153 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2800_e2669;
        locals.var_guard44_rv = 0.0;

        let (assign2810_e2673, assign2810_e2673_d_n0, assign2810_e2673_d_n1, assign2810_e2673_d_n3, assign2810_e2673_d_n4, assign2810_e2673_d_n5, assign2810_e2673_d_n6, assign2810_e2673_d_n7, assign2810_e2673_d_n8, assign2810_e2673_d_n9, assign2810_e2673_d_n10, assign2810_e2673_d_n11,) = {
    if (locals.var_guard44 != 0.0) {
        (p.p153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pw, locals.var_pw_dn0, locals.var_pw_dn1, locals.var_pw_dn3, locals.var_pw_dn4, locals.var_pw_dn5, locals.var_pw_dn6, locals.var_pw_dn7, locals.var_pw_dn8, locals.var_pw_dn9, locals.var_pw_dn10, locals.var_pw_dn11,)
    }
};
        locals.var_pw = assign2810_e2673;
        locals.var_pw_dn0 = assign2810_e2673_d_n0;
        locals.var_pw_dn1 = assign2810_e2673_d_n1;
        locals.var_pw_dn3 = assign2810_e2673_d_n3;
        locals.var_pw_dn4 = assign2810_e2673_d_n4;
        locals.var_pw_dn5 = assign2810_e2673_d_n5;
        locals.var_pw_dn6 = assign2810_e2673_d_n6;
        locals.var_pw_dn7 = assign2810_e2673_d_n7;
        locals.var_pw_dn8 = assign2810_e2673_d_n8;
        locals.var_pw_dn9 = assign2810_e2673_d_n9;
        locals.var_pw_dn10 = assign2810_e2673_d_n10;
        locals.var_pw_dn11 = assign2810_e2673_d_n11;
        locals.var_pw_rv = 0.0;

        let assign2820_e2677: f64 = (locals.var_k0 - locals.var_kw);
        let assign2820_e2680: f64 = (locals.var_k0 + 1.0);
        let assign2820_e2683: f64 = (locals.var_kw + 1.0);
        let assign2820_e2684: f64 = (assign2820_e2680 / assign2820_e2683);
        let assign2820_e2685: f64 = (assign2820_e2684).ln();
        let assign2820_e2686: f64 = (assign2820_e2677 - assign2820_e2685);
        let assign2820_e2687: f64 = (locals.var_vt * assign2820_e2686);
        locals.var_ec = assign2820_e2687;
        locals.var_ec_dn0 = (locals.var_vt * ((locals.var_k0_dn0 - locals.var_kw_dn0) - ((((locals.var_k0_dn0 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn0)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn1 = (locals.var_vt * ((locals.var_k0_dn1 - locals.var_kw_dn1) - ((((locals.var_k0_dn1 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn1)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn3 = (locals.var_vt * ((locals.var_k0_dn3 - locals.var_kw_dn3) - ((((locals.var_k0_dn3 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn3)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn4 = ((locals.var_vt_dn4 * assign2820_e2686) + (locals.var_vt * ((locals.var_k0_dn4 - locals.var_kw_dn4) - ((((locals.var_k0_dn4 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn4)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684))));
        locals.var_ec_dn5 = (locals.var_vt * ((locals.var_k0_dn5 - locals.var_kw_dn5) - ((((locals.var_k0_dn5 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn5)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn6 = (locals.var_vt * ((locals.var_k0_dn6 - locals.var_kw_dn6) - ((((locals.var_k0_dn6 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn6)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn7 = (locals.var_vt * ((locals.var_k0_dn7 - locals.var_kw_dn7) - ((((locals.var_k0_dn7 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn7)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn8 = (locals.var_vt * ((locals.var_k0_dn8 - locals.var_kw_dn8) - ((((locals.var_k0_dn8 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn8)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn9 = (locals.var_vt * ((locals.var_k0_dn9 - locals.var_kw_dn9) - ((((locals.var_k0_dn9 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn9)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn10 = (locals.var_vt * ((locals.var_k0_dn10 - locals.var_kw_dn10) - ((((locals.var_k0_dn10 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn10)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_dn11 = (locals.var_vt * ((locals.var_k0_dn11 - locals.var_kw_dn11) - ((((locals.var_k0_dn11 * assign2820_e2683) - (assign2820_e2680 * locals.var_kw_dn11)) / (assign2820_e2683 * assign2820_e2683)) / assign2820_e2684)));
        locals.var_ec_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2830_e2690: f64 = (locals.var_ec + locals.var_vc1c2);
        let assign2830_e2692: f64 = (assign2830_e2690 / locals.var_rcv_t);
        locals.var_ic1c2 = assign2830_e2692;
        locals.var_ic1c2_dn0 = (locals.var_ec_dn0 / locals.var_rcv_t);
        locals.var_ic1c2_dn1 = (locals.var_ec_dn1 / locals.var_rcv_t);
        locals.var_ic1c2_dn3 = (locals.var_ec_dn3 / locals.var_rcv_t);
        locals.var_ic1c2_dn4 = (((locals.var_ec_dn4 * locals.var_rcv_t) - (assign2830_e2690 * locals.var_rcv_t_dn4)) / (locals.var_rcv_t * locals.var_rcv_t));
        locals.var_ic1c2_dn5 = (locals.var_ec_dn5 / locals.var_rcv_t);
        locals.var_ic1c2_dn6 = (locals.var_ec_dn6 / locals.var_rcv_t);
        locals.var_ic1c2_dn7 = (locals.var_ec_dn7 / locals.var_rcv_t);
        locals.var_ic1c2_dn8 = ((locals.var_ec_dn8 + locals.var_vc1c2_dn8) / locals.var_rcv_t);
        locals.var_ic1c2_dn9 = ((locals.var_ec_dn9 + locals.var_vc1c2_dn9) / locals.var_rcv_t);
        locals.var_ic1c2_dn10 = (locals.var_ec_dn10 / locals.var_rcv_t);
        locals.var_ic1c2_dn11 = (locals.var_ec_dn11 / locals.var_rcv_t);
        locals.var_ic1c2_rv = 0.0;

        let assign2840_e2695: f64 = if locals.var_ic1c2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign2840_e2695;
        locals.var_guard45_rv = 0.0;

        let assign2850_e2698: f64 = if locals.var_vb2c1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign2850_e2698;
        locals.var_guard46_rv = 0.0;

        let (assign2860_e2704, assign2860_e2704_d_n7, assign2860_e2704_d_n8,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard46 != 0.0)) {
        (locals.var_vb2c1, locals.var_vb2c1_dn7, locals.var_vb2c1_dn8,)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn7, locals.var_tmpv_dn8,)
    }
};
        locals.var_tmpv = assign2860_e2704;
        locals.var_tmpv_dn7 = assign2860_e2704_d_n7;
        locals.var_tmpv_dn8 = assign2860_e2704_d_n8;
        locals.var_tmpv_rv = 0.0;

        let (assign2870_e2718, assign2870_e2718_d_n7, assign2870_e2718_d_n8,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard46 == 0.0)) {
        let assign2870_e2713: f64 = (locals.var_vb2c1 - 100.0);
        let assign2870_e2714: f64 = (1.0 + assign2870_e2713);
        let assign2870_e2715: f64 = (assign2870_e2714).ln();
        let assign2870_e2716: f64 = (100.0 + assign2870_e2715);
        (assign2870_e2716, (locals.var_vb2c1_dn7 / assign2870_e2714), (locals.var_vb2c1_dn8 / assign2870_e2714),)
    } else {
        (locals.var_tmpv, locals.var_tmpv_dn7, locals.var_tmpv_dn8,)
    }
};
        locals.var_tmpv = assign2870_e2718;
        locals.var_tmpv_dn7 = assign2870_e2718_d_n7;
        locals.var_tmpv_dn8 = assign2870_e2718_d_n8;
        locals.var_tmpv_rv = 0.0;

        let (assign2880_e2739, assign2880_e2739_d_n0, assign2880_e2739_d_n1, assign2880_e2739_d_n3, assign2880_e2739_d_n4, assign2880_e2739_d_n5, assign2880_e2739_d_n6, assign2880_e2739_d_n7, assign2880_e2739_d_n8, assign2880_e2739_d_n9, assign2880_e2739_d_n10, assign2880_e2739_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2880_e2723: f64 = (2.0 * locals.var_vt);
        let assign2880_e2726: f64 = (0.5 * locals.var_ic1c2);
        let assign2880_e2728: f64 = (assign2880_e2726 * locals.var_rcv_t);
        let assign2880_e2730: f64 = (assign2880_e2728 * locals.var_vtinv);
        let assign2880_e2732: f64 = (assign2880_e2730 + 1.0);
        let assign2880_e2733: f64 = (assign2880_e2732).ln();
        let assign2880_e2734: f64 = (assign2880_e2723 * assign2880_e2733);
        let assign2880_e2735: f64 = (locals.var_vdc_t + assign2880_e2734);
        let assign2880_e2737: f64 = (assign2880_e2735 - locals.var_tmpv);
        (assign2880_e2737, (locals.var_vdc_t_dn0 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn1 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn3 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn4 + (((2.0 * locals.var_vt_dn4) * assign2880_e2733) + (assign2880_e2723 * ((((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) + (assign2880_e2726 * locals.var_rcv_t_dn4)) * locals.var_vtinv) + (assign2880_e2728 * locals.var_vtinv_dn4)) / assign2880_e2732)))), (locals.var_vdc_t_dn5 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn6 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), ((locals.var_vdc_t_dn7 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))) - locals.var_tmpv_dn7), ((locals.var_vdc_t_dn8 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))) - locals.var_tmpv_dn8), (locals.var_vdc_t_dn9 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn10 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn10) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))), (locals.var_vdc_t_dn11 + (assign2880_e2723 * ((((0.5 * locals.var_ic1c2_dn11) * locals.var_rcv_t) * locals.var_vtinv) / assign2880_e2732))),)
    } else {
        (locals.var_vqs_th, locals.var_vqs_th_dn0, locals.var_vqs_th_dn1, locals.var_vqs_th_dn3, locals.var_vqs_th_dn4, locals.var_vqs_th_dn5, locals.var_vqs_th_dn6, locals.var_vqs_th_dn7, locals.var_vqs_th_dn8, locals.var_vqs_th_dn9, locals.var_vqs_th_dn10, locals.var_vqs_th_dn11,)
    }
};
        locals.var_vqs_th = assign2880_e2739;
        locals.var_vqs_th_dn0 = assign2880_e2739_d_n0;
        locals.var_vqs_th_dn1 = assign2880_e2739_d_n1;
        locals.var_vqs_th_dn3 = assign2880_e2739_d_n3;
        locals.var_vqs_th_dn4 = assign2880_e2739_d_n4;
        locals.var_vqs_th_dn5 = assign2880_e2739_d_n5;
        locals.var_vqs_th_dn6 = assign2880_e2739_d_n6;
        locals.var_vqs_th_dn7 = assign2880_e2739_d_n7;
        locals.var_vqs_th_dn8 = assign2880_e2739_d_n8;
        locals.var_vqs_th_dn9 = assign2880_e2739_d_n9;
        locals.var_vqs_th_dn10 = assign2880_e2739_d_n10;
        locals.var_vqs_th_dn11 = assign2880_e2739_d_n11;
        locals.var_vqs_th_rv = 0.0;

        let (assign2890_e2745, assign2890_e2745_d_n0, assign2890_e2745_d_n1, assign2890_e2745_d_n3, assign2890_e2745_d_n4, assign2890_e2745_d_n5, assign2890_e2745_d_n6, assign2890_e2745_d_n7, assign2890_e2745_d_n8, assign2890_e2745_d_n9, assign2890_e2745_d_n10, assign2890_e2745_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2890_e2743: f64 = (0.2 * locals.var_vdc_t);
        (assign2890_e2743, (0.2 * locals.var_vdc_t_dn0), (0.2 * locals.var_vdc_t_dn1), (0.2 * locals.var_vdc_t_dn3), (0.2 * locals.var_vdc_t_dn4), (0.2 * locals.var_vdc_t_dn5), (0.2 * locals.var_vdc_t_dn6), (0.2 * locals.var_vdc_t_dn7), (0.2 * locals.var_vdc_t_dn8), (0.2 * locals.var_vdc_t_dn9), (0.2 * locals.var_vdc_t_dn10), (0.2 * locals.var_vdc_t_dn11),)
    } else {
        (locals.var_eps_vdc, locals.var_eps_vdc_dn0, locals.var_eps_vdc_dn1, locals.var_eps_vdc_dn3, locals.var_eps_vdc_dn4, locals.var_eps_vdc_dn5, locals.var_eps_vdc_dn6, locals.var_eps_vdc_dn7, locals.var_eps_vdc_dn8, locals.var_eps_vdc_dn9, locals.var_eps_vdc_dn10, locals.var_eps_vdc_dn11,)
    }
};
        locals.var_eps_vdc = assign2890_e2745;
        locals.var_eps_vdc_dn0 = assign2890_e2745_d_n0;
        locals.var_eps_vdc_dn1 = assign2890_e2745_d_n1;
        locals.var_eps_vdc_dn3 = assign2890_e2745_d_n3;
        locals.var_eps_vdc_dn4 = assign2890_e2745_d_n4;
        locals.var_eps_vdc_dn5 = assign2890_e2745_d_n5;
        locals.var_eps_vdc_dn6 = assign2890_e2745_d_n6;
        locals.var_eps_vdc_dn7 = assign2890_e2745_d_n7;
        locals.var_eps_vdc_dn8 = assign2890_e2745_d_n8;
        locals.var_eps_vdc_dn9 = assign2890_e2745_d_n9;
        locals.var_eps_vdc_dn10 = assign2890_e2745_d_n10;
        locals.var_eps_vdc_dn11 = assign2890_e2745_d_n11;
        locals.var_eps_vdc_rv = 0.0;

        let (assign2900_e2751, assign2900_e2751_d_n0, assign2900_e2751_d_n1, assign2900_e2751_d_n3, assign2900_e2751_d_n4, assign2900_e2751_d_n5, assign2900_e2751_d_n6, assign2900_e2751_d_n7, assign2900_e2751_d_n8, assign2900_e2751_d_n9, assign2900_e2751_d_n10, assign2900_e2751_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2900_e2749: f64 = (locals.var_eps_vdc * locals.var_eps_vdc);
        (assign2900_e2749, ((locals.var_eps_vdc_dn0 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn0)), ((locals.var_eps_vdc_dn1 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn1)), ((locals.var_eps_vdc_dn3 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn3)), ((locals.var_eps_vdc_dn4 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn4)), ((locals.var_eps_vdc_dn5 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn5)), ((locals.var_eps_vdc_dn6 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn6)), ((locals.var_eps_vdc_dn7 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn7)), ((locals.var_eps_vdc_dn8 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn8)), ((locals.var_eps_vdc_dn9 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn9)), ((locals.var_eps_vdc_dn10 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn10)), ((locals.var_eps_vdc_dn11 * locals.var_eps_vdc) + (locals.var_eps_vdc * locals.var_eps_vdc_dn11)),)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10, locals.var_eps2_dn11,)
    }
};
        locals.var_eps2 = assign2900_e2751;
        locals.var_eps2_dn0 = assign2900_e2751_d_n0;
        locals.var_eps2_dn1 = assign2900_e2751_d_n1;
        locals.var_eps2_dn3 = assign2900_e2751_d_n3;
        locals.var_eps2_dn4 = assign2900_e2751_d_n4;
        locals.var_eps2_dn5 = assign2900_e2751_d_n5;
        locals.var_eps2_dn6 = assign2900_e2751_d_n6;
        locals.var_eps2_dn7 = assign2900_e2751_d_n7;
        locals.var_eps2_dn8 = assign2900_e2751_d_n8;
        locals.var_eps2_dn9 = assign2900_e2751_d_n9;
        locals.var_eps2_dn10 = assign2900_e2751_d_n10;
        locals.var_eps2_dn11 = assign2900_e2751_d_n11;
        locals.var_eps2_rv = 0.0;

        let (assign2910_e2757, assign2910_e2757_d_n0, assign2910_e2757_d_n1, assign2910_e2757_d_n3, assign2910_e2757_d_n4, assign2910_e2757_d_n5, assign2910_e2757_d_n6, assign2910_e2757_d_n7, assign2910_e2757_d_n8, assign2910_e2757_d_n9, assign2910_e2757_d_n10, assign2910_e2757_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2910_e2755: f64 = (locals.var_vqs_th * locals.var_vqs_th);
        (assign2910_e2755, ((locals.var_vqs_th_dn0 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn0)), ((locals.var_vqs_th_dn1 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn1)), ((locals.var_vqs_th_dn3 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn3)), ((locals.var_vqs_th_dn4 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn4)), ((locals.var_vqs_th_dn5 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn5)), ((locals.var_vqs_th_dn6 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn6)), ((locals.var_vqs_th_dn7 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn7)), ((locals.var_vqs_th_dn8 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn8)), ((locals.var_vqs_th_dn9 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn9)), ((locals.var_vqs_th_dn10 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn10)), ((locals.var_vqs_th_dn11 * locals.var_vqs_th) + (locals.var_vqs_th * locals.var_vqs_th_dn11)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11,)
    }
};
        locals.var_x2 = assign2910_e2757;
        locals.var_x2_dn0 = assign2910_e2757_d_n0;
        locals.var_x2_dn1 = assign2910_e2757_d_n1;
        locals.var_x2_dn3 = assign2910_e2757_d_n3;
        locals.var_x2_dn4 = assign2910_e2757_d_n4;
        locals.var_x2_dn5 = assign2910_e2757_d_n5;
        locals.var_x2_dn6 = assign2910_e2757_d_n6;
        locals.var_x2_dn7 = assign2910_e2757_d_n7;
        locals.var_x2_dn8 = assign2910_e2757_d_n8;
        locals.var_x2_dn9 = assign2910_e2757_d_n9;
        locals.var_x2_dn10 = assign2910_e2757_d_n10;
        locals.var_x2_dn11 = assign2910_e2757_d_n11;
        locals.var_x2_rv = 0.0;

        let assign2920_e2760: f64 = if locals.var_vqs_th < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign2920_e2760;
        locals.var_guard47_rv = 0.0;

        let (assign2930_e2775, assign2930_e2775_d_n0, assign2930_e2775_d_n1, assign2930_e2775_d_n3, assign2930_e2775_d_n4, assign2930_e2775_d_n5, assign2930_e2775_d_n6, assign2930_e2775_d_n7, assign2930_e2775_d_n8, assign2930_e2775_d_n9, assign2930_e2775_d_n10, assign2930_e2775_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard47 != 0.0)) {
        let assign2930_e2766: f64 = (0.5 * locals.var_eps2);
        let assign2930_e2769: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2930_e2770: f64 = (assign2930_e2769).sqrt();
        let assign2930_e2772: f64 = (assign2930_e2770 - locals.var_vqs_th);
        let assign2930_e2773: f64 = (assign2930_e2766 / assign2930_e2772);
        (assign2930_e2773, ((((0.5 * locals.var_eps2_dn0) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn0))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn1) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn1))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn3) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn3))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn4) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn4))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn5) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn5))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn6) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn6))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn7) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn7))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn8) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn8))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn9) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn9))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn10) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn10))) / (assign2930_e2772 * assign2930_e2772)), ((((0.5 * locals.var_eps2_dn11) * assign2930_e2772) - (assign2930_e2766 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign2930_e2770)) - locals.var_vqs_th_dn11))) / (assign2930_e2772 * assign2930_e2772)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9, locals.var_vqs_dn10, locals.var_vqs_dn11,)
    }
};
        locals.var_vqs = assign2930_e2775;
        locals.var_vqs_dn0 = assign2930_e2775_d_n0;
        locals.var_vqs_dn1 = assign2930_e2775_d_n1;
        locals.var_vqs_dn3 = assign2930_e2775_d_n3;
        locals.var_vqs_dn4 = assign2930_e2775_d_n4;
        locals.var_vqs_dn5 = assign2930_e2775_d_n5;
        locals.var_vqs_dn6 = assign2930_e2775_d_n6;
        locals.var_vqs_dn7 = assign2930_e2775_d_n7;
        locals.var_vqs_dn8 = assign2930_e2775_d_n8;
        locals.var_vqs_dn9 = assign2930_e2775_d_n9;
        locals.var_vqs_dn10 = assign2930_e2775_d_n10;
        locals.var_vqs_dn11 = assign2930_e2775_d_n11;
        locals.var_vqs_rv = 0.0;

        let (assign2940_e2789, assign2940_e2789_d_n0, assign2940_e2789_d_n1, assign2940_e2789_d_n3, assign2940_e2789_d_n4, assign2940_e2789_d_n5, assign2940_e2789_d_n6, assign2940_e2789_d_n7, assign2940_e2789_d_n8, assign2940_e2789_d_n9, assign2940_e2789_d_n10, assign2940_e2789_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard47 == 0.0)) {
        let assign2940_e2783: f64 = (locals.var_x2 + locals.var_eps2);
        let assign2940_e2784: f64 = (assign2940_e2783).sqrt();
        let assign2940_e2786: f64 = (assign2940_e2784 + locals.var_vqs_th);
        let assign2940_e2787: f64 = (0.5 * assign2940_e2786);
        (assign2940_e2787, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign2940_e2784)) + locals.var_vqs_th_dn11)),)
    } else {
        (locals.var_vqs, locals.var_vqs_dn0, locals.var_vqs_dn1, locals.var_vqs_dn3, locals.var_vqs_dn4, locals.var_vqs_dn5, locals.var_vqs_dn6, locals.var_vqs_dn7, locals.var_vqs_dn8, locals.var_vqs_dn9, locals.var_vqs_dn10, locals.var_vqs_dn11,)
    }
};
        locals.var_vqs = assign2940_e2789;
        locals.var_vqs_dn0 = assign2940_e2789_d_n0;
        locals.var_vqs_dn1 = assign2940_e2789_d_n1;
        locals.var_vqs_dn3 = assign2940_e2789_d_n3;
        locals.var_vqs_dn4 = assign2940_e2789_d_n4;
        locals.var_vqs_dn5 = assign2940_e2789_d_n5;
        locals.var_vqs_dn6 = assign2940_e2789_d_n6;
        locals.var_vqs_dn7 = assign2940_e2789_d_n7;
        locals.var_vqs_dn8 = assign2940_e2789_d_n8;
        locals.var_vqs_dn9 = assign2940_e2789_d_n9;
        locals.var_vqs_dn10 = assign2940_e2789_d_n10;
        locals.var_vqs_dn11 = assign2940_e2789_d_n11;
        locals.var_vqs_rv = 0.0;

        let (assign2950_e2807, assign2950_e2807_d_n0, assign2950_e2807_d_n1, assign2950_e2807_d_n3, assign2950_e2807_d_n4, assign2950_e2807_d_n5, assign2950_e2807_d_n6, assign2950_e2807_d_n7, assign2950_e2807_d_n8, assign2950_e2807_d_n9, assign2950_e2807_d_n10, assign2950_e2807_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2950_e2795: f64 = (p.p62 * p.p61);
        let assign2950_e2796: f64 = (locals.var_vqs + assign2950_e2795);
        let assign2950_e2797: f64 = (locals.var_vqs * assign2950_e2796);
        let assign2950_e2802: f64 = (p.p62 * locals.var_rcv_t);
        let assign2950_e2803: f64 = (locals.var_vqs + assign2950_e2802);
        let assign2950_e2804: f64 = (p.p61 * assign2950_e2803);
        let assign2950_e2805: f64 = (assign2950_e2797 / assign2950_e2804);
        (assign2950_e2805, (((((locals.var_vqs_dn0 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn0)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn0))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn1 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn1)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn1))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn3 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn3)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn3))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn4 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn4)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * (locals.var_vqs_dn4 + (p.p62 * locals.var_rcv_t_dn4))))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn5 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn5)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn5))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn6 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn6)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn6))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn7 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn7)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn7))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn8 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn8)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn8))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn9 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn9)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn9))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn10 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn10)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn10))) / (assign2950_e2804 * assign2950_e2804)), (((((locals.var_vqs_dn11 * assign2950_e2796) + (locals.var_vqs * locals.var_vqs_dn11)) * assign2950_e2804) - (assign2950_e2797 * (p.p61 * locals.var_vqs_dn11))) / (assign2950_e2804 * assign2950_e2804)),)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9, locals.var_iqs_dn10, locals.var_iqs_dn11,)
    }
};
        locals.var_iqs = assign2950_e2807;
        locals.var_iqs_dn0 = assign2950_e2807_d_n0;
        locals.var_iqs_dn1 = assign2950_e2807_d_n1;
        locals.var_iqs_dn3 = assign2950_e2807_d_n3;
        locals.var_iqs_dn4 = assign2950_e2807_d_n4;
        locals.var_iqs_dn5 = assign2950_e2807_d_n5;
        locals.var_iqs_dn6 = assign2950_e2807_d_n6;
        locals.var_iqs_dn7 = assign2950_e2807_d_n7;
        locals.var_iqs_dn8 = assign2950_e2807_d_n8;
        locals.var_iqs_dn9 = assign2950_e2807_d_n9;
        locals.var_iqs_dn10 = assign2950_e2807_d_n10;
        locals.var_iqs_dn11 = assign2950_e2807_d_n11;
        locals.var_iqs_rv = 0.0;

        let (assign2960_e2813, assign2960_e2813_d_n0, assign2960_e2813_d_n1, assign2960_e2813_d_n3, assign2960_e2813_d_n4, assign2960_e2813_d_n5, assign2960_e2813_d_n6, assign2960_e2813_d_n7, assign2960_e2813_d_n8, assign2960_e2813_d_n9, assign2960_e2813_d_n10, assign2960_e2813_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2960_e2811: f64 = (locals.var_ic1c2 / locals.var_iqs);
        (assign2960_e2811, (((locals.var_ic1c2_dn0 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn0)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn1 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn1)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn3 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn3)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn4 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn4)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn5 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn5)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn6 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn6)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn7 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn7)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn8 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn8)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn9 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn9)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn10 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn10)) / (locals.var_iqs * locals.var_iqs)), (((locals.var_ic1c2_dn11 * locals.var_iqs) - (locals.var_ic1c2 * locals.var_iqs_dn11)) / (locals.var_iqs * locals.var_iqs)),)
    } else {
        (locals.var_ic1c2_iqs, locals.var_ic1c2_iqs_dn0, locals.var_ic1c2_iqs_dn1, locals.var_ic1c2_iqs_dn3, locals.var_ic1c2_iqs_dn4, locals.var_ic1c2_iqs_dn5, locals.var_ic1c2_iqs_dn6, locals.var_ic1c2_iqs_dn7, locals.var_ic1c2_iqs_dn8, locals.var_ic1c2_iqs_dn9, locals.var_ic1c2_iqs_dn10, locals.var_ic1c2_iqs_dn11,)
    }
};
        locals.var_ic1c2_iqs = assign2960_e2813;
        locals.var_ic1c2_iqs_dn0 = assign2960_e2813_d_n0;
        locals.var_ic1c2_iqs_dn1 = assign2960_e2813_d_n1;
        locals.var_ic1c2_iqs_dn3 = assign2960_e2813_d_n3;
        locals.var_ic1c2_iqs_dn4 = assign2960_e2813_d_n4;
        locals.var_ic1c2_iqs_dn5 = assign2960_e2813_d_n5;
        locals.var_ic1c2_iqs_dn6 = assign2960_e2813_d_n6;
        locals.var_ic1c2_iqs_dn7 = assign2960_e2813_d_n7;
        locals.var_ic1c2_iqs_dn8 = assign2960_e2813_d_n8;
        locals.var_ic1c2_iqs_dn9 = assign2960_e2813_d_n9;
        locals.var_ic1c2_iqs_dn10 = assign2960_e2813_d_n10;
        locals.var_ic1c2_iqs_dn11 = assign2960_e2813_d_n11;
        locals.var_ic1c2_iqs_rv = 0.0;

        let (assign2970_e2821, assign2970_e2821_d_n0, assign2970_e2821_d_n1, assign2970_e2821_d_n3, assign2970_e2821_d_n4, assign2970_e2821_d_n5, assign2970_e2821_d_n6, assign2970_e2821_d_n7, assign2970_e2821_d_n8, assign2970_e2821_d_n9, assign2970_e2821_d_n10, assign2970_e2821_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign2970_e2817: f64 = (locals.var_ic1c2_iqs - 1.0);
        let assign2970_e2819: f64 = (assign2970_e2817 / p.p63);
        (assign2970_e2819, (locals.var_ic1c2_iqs_dn0 / p.p63), (locals.var_ic1c2_iqs_dn1 / p.p63), (locals.var_ic1c2_iqs_dn3 / p.p63), (locals.var_ic1c2_iqs_dn4 / p.p63), (locals.var_ic1c2_iqs_dn5 / p.p63), (locals.var_ic1c2_iqs_dn6 / p.p63), (locals.var_ic1c2_iqs_dn7 / p.p63), (locals.var_ic1c2_iqs_dn8 / p.p63), (locals.var_ic1c2_iqs_dn9 / p.p63), (locals.var_ic1c2_iqs_dn10 / p.p63), (locals.var_ic1c2_iqs_dn11 / p.p63),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign2970_e2821;
        locals.var_dxa_dn0 = assign2970_e2821_d_n0;
        locals.var_dxa_dn1 = assign2970_e2821_d_n1;
        locals.var_dxa_dn3 = assign2970_e2821_d_n3;
        locals.var_dxa_dn4 = assign2970_e2821_d_n4;
        locals.var_dxa_dn5 = assign2970_e2821_d_n5;
        locals.var_dxa_dn6 = assign2970_e2821_d_n6;
        locals.var_dxa_dn7 = assign2970_e2821_d_n7;
        locals.var_dxa_dn8 = assign2970_e2821_d_n8;
        locals.var_dxa_dn9 = assign2970_e2821_d_n9;
        locals.var_dxa_dn10 = assign2970_e2821_d_n10;
        locals.var_dxa_dn11 = assign2970_e2821_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign2980_e2824: f64 = if locals.var_ic1c2_iqs < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign2980_e2824;
        locals.var_guard48_rv = 0.0;

        let (assign2990_e2838, assign2990_e2838_d_n0, assign2990_e2838_d_n1, assign2990_e2838_d_n3, assign2990_e2838_d_n4, assign2990_e2838_d_n5, assign2990_e2838_d_n6, assign2990_e2838_d_n7, assign2990_e2838_d_n8, assign2990_e2838_d_n9, assign2990_e2838_d_n10, assign2990_e2838_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard48 != 0.0)) {
        let assign2990_e2832: f64 = (locals.var_dxa).exp();
        let assign2990_e2833: f64 = (1.0 + assign2990_e2832);
        let assign2990_e2834: f64 = (assign2990_e2833).ln();
        let assign2990_e2835: f64 = (p.p63 * assign2990_e2834);
        let assign2990_e2836: f64 = (1.0 + assign2990_e2835);
        (assign2990_e2836, (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn0) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn1) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn3) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn4) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn5) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn6) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn7) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn8) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn9) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn10) / assign2990_e2833)), (p.p63 * ((assign2990_e2832 * locals.var_dxa_dn11) / assign2990_e2833)),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign2990_e2838;
        locals.var_alpha1_dn0 = assign2990_e2838_d_n0;
        locals.var_alpha1_dn1 = assign2990_e2838_d_n1;
        locals.var_alpha1_dn3 = assign2990_e2838_d_n3;
        locals.var_alpha1_dn4 = assign2990_e2838_d_n4;
        locals.var_alpha1_dn5 = assign2990_e2838_d_n5;
        locals.var_alpha1_dn6 = assign2990_e2838_d_n6;
        locals.var_alpha1_dn7 = assign2990_e2838_d_n7;
        locals.var_alpha1_dn8 = assign2990_e2838_d_n8;
        locals.var_alpha1_dn9 = assign2990_e2838_d_n9;
        locals.var_alpha1_dn10 = assign2990_e2838_d_n10;
        locals.var_alpha1_dn11 = assign2990_e2838_d_n11;
        locals.var_alpha1_rv = 0.0;

        let (assign3000_e2854, assign3000_e2854_d_n0, assign3000_e2854_d_n1, assign3000_e2854_d_n3, assign3000_e2854_d_n4, assign3000_e2854_d_n5, assign3000_e2854_d_n6, assign3000_e2854_d_n7, assign3000_e2854_d_n8, assign3000_e2854_d_n9, assign3000_e2854_d_n10, assign3000_e2854_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard48 == 0.0)) {
        let assign3000_e2847: f64 = (-locals.var_dxa);
        let assign3000_e2848: f64 = (assign3000_e2847).exp();
        let assign3000_e2849: f64 = (1.0 + assign3000_e2848);
        let assign3000_e2850: f64 = (assign3000_e2849).ln();
        let assign3000_e2851: f64 = (p.p63 * assign3000_e2850);
        let assign3000_e2852: f64 = (locals.var_ic1c2_iqs + assign3000_e2851);
        (assign3000_e2852, (locals.var_ic1c2_iqs_dn0 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn0)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn1 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn1)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn3 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn3)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn4 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn4)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn5 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn5)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn6 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn6)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn7 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn7)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn8 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn8)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn9 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn9)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn10 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn10)) / assign3000_e2849))), (locals.var_ic1c2_iqs_dn11 + (p.p63 * ((assign3000_e2848 * (-locals.var_dxa_dn11)) / assign3000_e2849))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn0, locals.var_alpha1_dn1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign3000_e2854;
        locals.var_alpha1_dn0 = assign3000_e2854_d_n0;
        locals.var_alpha1_dn1 = assign3000_e2854_d_n1;
        locals.var_alpha1_dn3 = assign3000_e2854_d_n3;
        locals.var_alpha1_dn4 = assign3000_e2854_d_n4;
        locals.var_alpha1_dn5 = assign3000_e2854_d_n5;
        locals.var_alpha1_dn6 = assign3000_e2854_d_n6;
        locals.var_alpha1_dn7 = assign3000_e2854_d_n7;
        locals.var_alpha1_dn8 = assign3000_e2854_d_n8;
        locals.var_alpha1_dn9 = assign3000_e2854_d_n9;
        locals.var_alpha1_dn10 = assign3000_e2854_d_n10;
        locals.var_alpha1_dn11 = assign3000_e2854_d_n11;
        locals.var_alpha1_rv = 0.0;

        let (assign3010_e2871, assign3010_e2871_d_n0, assign3010_e2871_d_n1, assign3010_e2871_d_n3, assign3010_e2871_d_n4, assign3010_e2871_d_n5, assign3010_e2871_d_n6, assign3010_e2871_d_n7, assign3010_e2871_d_n8, assign3010_e2871_d_n9, assign3010_e2871_d_n10, assign3010_e2871_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3010_e2861: f64 = (-1.0);
        let assign3010_e2863: f64 = (assign3010_e2861 / p.p63);
        let assign3010_e2864: f64 = (assign3010_e2863).exp();
        let assign3010_e2865: f64 = (1.0 + assign3010_e2864);
        let assign3010_e2866: f64 = (assign3010_e2865).ln();
        let assign3010_e2867: f64 = (p.p63 * assign3010_e2866);
        let assign3010_e2868: f64 = (1.0 + assign3010_e2867);
        let assign3010_e2869: f64 = (locals.var_alpha1 / assign3010_e2868);
        (assign3010_e2869, (locals.var_alpha1_dn0 / assign3010_e2868), (locals.var_alpha1_dn1 / assign3010_e2868), (locals.var_alpha1_dn3 / assign3010_e2868), (locals.var_alpha1_dn4 / assign3010_e2868), (locals.var_alpha1_dn5 / assign3010_e2868), (locals.var_alpha1_dn6 / assign3010_e2868), (locals.var_alpha1_dn7 / assign3010_e2868), (locals.var_alpha1_dn8 / assign3010_e2868), (locals.var_alpha1_dn9 / assign3010_e2868), (locals.var_alpha1_dn10 / assign3010_e2868), (locals.var_alpha1_dn11 / assign3010_e2868),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn1, locals.var_alpha_dn3, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11,)
    }
};
        locals.var_alpha = assign3010_e2871;
        locals.var_alpha_dn0 = assign3010_e2871_d_n0;
        locals.var_alpha_dn1 = assign3010_e2871_d_n1;
        locals.var_alpha_dn3 = assign3010_e2871_d_n3;
        locals.var_alpha_dn4 = assign3010_e2871_d_n4;
        locals.var_alpha_dn5 = assign3010_e2871_d_n5;
        locals.var_alpha_dn6 = assign3010_e2871_d_n6;
        locals.var_alpha_dn7 = assign3010_e2871_d_n7;
        locals.var_alpha_dn8 = assign3010_e2871_d_n8;
        locals.var_alpha_dn9 = assign3010_e2871_d_n9;
        locals.var_alpha_dn10 = assign3010_e2871_d_n10;
        locals.var_alpha_dn11 = assign3010_e2871_d_n11;
        locals.var_alpha_rv = 0.0;

        let (assign3020_e2879, assign3020_e2879_d_n0, assign3020_e2879_d_n1, assign3020_e2879_d_n3, assign3020_e2879_d_n4, assign3020_e2879_d_n5, assign3020_e2879_d_n6, assign3020_e2879_d_n7, assign3020_e2879_d_n8, assign3020_e2879_d_n9, assign3020_e2879_d_n10, assign3020_e2879_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3020_e2876: f64 = (p.p62 * p.p61);
        let assign3020_e2877: f64 = (locals.var_vqs / assign3020_e2876);
        (assign3020_e2877, (locals.var_vqs_dn0 / assign3020_e2876), (locals.var_vqs_dn1 / assign3020_e2876), (locals.var_vqs_dn3 / assign3020_e2876), (locals.var_vqs_dn4 / assign3020_e2876), (locals.var_vqs_dn5 / assign3020_e2876), (locals.var_vqs_dn6 / assign3020_e2876), (locals.var_vqs_dn7 / assign3020_e2876), (locals.var_vqs_dn8 / assign3020_e2876), (locals.var_vqs_dn9 / assign3020_e2876), (locals.var_vqs_dn10 / assign3020_e2876), (locals.var_vqs_dn11 / assign3020_e2876),)
    } else {
        (locals.var_vyi, locals.var_vyi_dn0, locals.var_vyi_dn1, locals.var_vyi_dn3, locals.var_vyi_dn4, locals.var_vyi_dn5, locals.var_vyi_dn6, locals.var_vyi_dn7, locals.var_vyi_dn8, locals.var_vyi_dn9, locals.var_vyi_dn10, locals.var_vyi_dn11,)
    }
};
        locals.var_vyi = assign3020_e2879;
        locals.var_vyi_dn0 = assign3020_e2879_d_n0;
        locals.var_vyi_dn1 = assign3020_e2879_d_n1;
        locals.var_vyi_dn3 = assign3020_e2879_d_n3;
        locals.var_vyi_dn4 = assign3020_e2879_d_n4;
        locals.var_vyi_dn5 = assign3020_e2879_d_n5;
        locals.var_vyi_dn6 = assign3020_e2879_d_n6;
        locals.var_vyi_dn7 = assign3020_e2879_d_n7;
        locals.var_vyi_dn8 = assign3020_e2879_d_n8;
        locals.var_vyi_dn9 = assign3020_e2879_d_n9;
        locals.var_vyi_dn10 = assign3020_e2879_d_n10;
        locals.var_vyi_dn11 = assign3020_e2879_d_n11;
        locals.var_vyi_rv = 0.0;

        let (assign3030_e2904, assign3030_e2904_d_n0, assign3030_e2904_d_n1, assign3030_e2904_d_n3, assign3030_e2904_d_n4, assign3030_e2904_d_n5, assign3030_e2904_d_n6, assign3030_e2904_d_n7, assign3030_e2904_d_n8, assign3030_e2904_d_n9, assign3030_e2904_d_n10, assign3030_e2904_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3030_e2885: f64 = (4.0 * locals.var_alpha);
        let assign3030_e2887: f64 = (assign3030_e2885 * locals.var_vyi);
        let assign3030_e2890: f64 = (1.0 + locals.var_vyi);
        let assign3030_e2891: f64 = (assign3030_e2887 * assign3030_e2890);
        let assign3030_e2892: f64 = (1.0 + assign3030_e2891);
        let assign3030_e2893: f64 = (assign3030_e2892).sqrt();
        let assign3030_e2894: f64 = (1.0 + assign3030_e2893);
        let assign3030_e2897: f64 = (2.0 * locals.var_alpha);
        let assign3030_e2900: f64 = (1.0 + locals.var_vyi);
        let assign3030_e2901: f64 = (assign3030_e2897 * assign3030_e2900);
        let assign3030_e2902: f64 = (assign3030_e2894 / assign3030_e2901);
        (assign3030_e2902, (((((((((4.0 * locals.var_alpha_dn0) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn0)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn0)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn0) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn0)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn1) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn1)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn1)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn1) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn1)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn3) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn3)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn3)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn3) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn3)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn4) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn4)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn4)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn4) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn4)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn5) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn5)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn5)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn5) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn5)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn6) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn6)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn6)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn6) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn6)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn7) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn7)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn7)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn7) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn7)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn8) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn8)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn8)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn8) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn8)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn9) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn9)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn9)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn9) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn9)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn10) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn10)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn10)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn10) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn10)))) / (assign3030_e2901 * assign3030_e2901)), (((((((((4.0 * locals.var_alpha_dn11) * locals.var_vyi) + (assign3030_e2885 * locals.var_vyi_dn11)) * assign3030_e2890) + (assign3030_e2887 * locals.var_vyi_dn11)) / (2.0 * assign3030_e2893)) * assign3030_e2901) - (assign3030_e2894 * (((2.0 * locals.var_alpha_dn11) * assign3030_e2900) + (assign3030_e2897 * locals.var_vyi_dn11)))) / (assign3030_e2901 * assign3030_e2901)),)
    } else {
        (locals.var_yi, locals.var_yi_dn0, locals.var_yi_dn1, locals.var_yi_dn3, locals.var_yi_dn4, locals.var_yi_dn5, locals.var_yi_dn6, locals.var_yi_dn7, locals.var_yi_dn8, locals.var_yi_dn9, locals.var_yi_dn10, locals.var_yi_dn11,)
    }
};
        locals.var_yi = assign3030_e2904;
        locals.var_yi_dn0 = assign3030_e2904_d_n0;
        locals.var_yi_dn1 = assign3030_e2904_d_n1;
        locals.var_yi_dn3 = assign3030_e2904_d_n3;
        locals.var_yi_dn4 = assign3030_e2904_d_n4;
        locals.var_yi_dn5 = assign3030_e2904_d_n5;
        locals.var_yi_dn6 = assign3030_e2904_d_n6;
        locals.var_yi_dn7 = assign3030_e2904_d_n7;
        locals.var_yi_dn8 = assign3030_e2904_d_n8;
        locals.var_yi_dn9 = assign3030_e2904_d_n9;
        locals.var_yi_dn10 = assign3030_e2904_d_n10;
        locals.var_yi_dn11 = assign3030_e2904_d_n11;
        locals.var_yi_rv = 0.0;

        let (assign3040_e2920, assign3040_e2920_d_n0, assign3040_e2920_d_n1, assign3040_e2920_d_n3, assign3040_e2920_d_n4, assign3040_e2920_d_n5, assign3040_e2920_d_n6, assign3040_e2920_d_n7, assign3040_e2920_d_n8, assign3040_e2920_d_n9, assign3040_e2920_d_n10, assign3040_e2920_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3040_e2908: f64 = (1.0 - locals.var_yi);
        let assign3040_e2911: f64 = (locals.var_pw * locals.var_yi);
        let assign3040_e2912: f64 = (assign3040_e2908 + assign3040_e2911);
        let assign3040_e2916: f64 = (locals.var_pw * locals.var_yi);
        let assign3040_e2917: f64 = (1.0 + assign3040_e2916);
        let assign3040_e2918: f64 = (assign3040_e2912 / assign3040_e2917);
        (assign3040_e2918, (((((-locals.var_yi_dn0) + ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn0 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn0)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn1) + ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn1 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn1)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn3) + ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn3 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn3)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn4) + ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn4 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn4)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn5) + ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn5 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn5)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn6) + ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn6 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn6)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn7) + ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn7 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn7)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn8) + ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn8 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn8)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn9) + ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn9 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn9)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn10) + ((locals.var_pw_dn10 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn10))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn10 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn10)))) / (assign3040_e2917 * assign3040_e2917)), (((((-locals.var_yi_dn11) + ((locals.var_pw_dn11 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn11))) * assign3040_e2917) - (assign3040_e2912 * ((locals.var_pw_dn11 * locals.var_yi) + (locals.var_pw * locals.var_yi_dn11)))) / (assign3040_e2917 * assign3040_e2917)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10, locals.var_xi_w_dn11,)
    }
};
        locals.var_xi_w = assign3040_e2920;
        locals.var_xi_w_dn0 = assign3040_e2920_d_n0;
        locals.var_xi_w_dn1 = assign3040_e2920_d_n1;
        locals.var_xi_w_dn3 = assign3040_e2920_d_n3;
        locals.var_xi_w_dn4 = assign3040_e2920_d_n4;
        locals.var_xi_w_dn5 = assign3040_e2920_d_n5;
        locals.var_xi_w_dn6 = assign3040_e2920_d_n6;
        locals.var_xi_w_dn7 = assign3040_e2920_d_n7;
        locals.var_xi_w_dn8 = assign3040_e2920_d_n8;
        locals.var_xi_w_dn9 = assign3040_e2920_d_n9;
        locals.var_xi_w_dn10 = assign3040_e2920_d_n10;
        locals.var_xi_w_dn11 = assign3040_e2920_d_n11;
        locals.var_xi_w_rv = 0.0;

        let (assign3050_e2932, assign3050_e2932_d_n0, assign3050_e2932_d_n1, assign3050_e2932_d_n3, assign3050_e2932_d_n4, assign3050_e2932_d_n5, assign3050_e2932_d_n6, assign3050_e2932_d_n7, assign3050_e2932_d_n8, assign3050_e2932_d_n9, assign3050_e2932_d_n10, assign3050_e2932_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3050_e2924: f64 = (0.5 * locals.var_ic1c2);
        let assign3050_e2926: f64 = (assign3050_e2924 * locals.var_rcv_t);
        let assign3050_e2928: f64 = (assign3050_e2926 * locals.var_xi_w);
        let assign3050_e2930: f64 = (assign3050_e2928 * locals.var_vtinv);
        (assign3050_e2930, (((((0.5 * locals.var_ic1c2_dn0) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn0)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn1) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn1)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn3) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn3)) * locals.var_vtinv), (((((((0.5 * locals.var_ic1c2_dn4) * locals.var_rcv_t) + (assign3050_e2924 * locals.var_rcv_t_dn4)) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn4)) * locals.var_vtinv) + (assign3050_e2928 * locals.var_vtinv_dn4)), (((((0.5 * locals.var_ic1c2_dn5) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn5)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn6) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn6)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn7) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn7)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn8) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn8)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn9) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn9)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn10) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn10)) * locals.var_vtinv), (((((0.5 * locals.var_ic1c2_dn11) * locals.var_rcv_t) * locals.var_xi_w) + (assign3050_e2926 * locals.var_xi_w_dn11)) * locals.var_vtinv),)
    } else {
        (locals.var_gp0, locals.var_gp0_dn0, locals.var_gp0_dn1, locals.var_gp0_dn3, locals.var_gp0_dn4, locals.var_gp0_dn5, locals.var_gp0_dn6, locals.var_gp0_dn7, locals.var_gp0_dn8, locals.var_gp0_dn9, locals.var_gp0_dn10, locals.var_gp0_dn11,)
    }
};
        locals.var_gp0 = assign3050_e2932;
        locals.var_gp0_dn0 = assign3050_e2932_d_n0;
        locals.var_gp0_dn1 = assign3050_e2932_d_n1;
        locals.var_gp0_dn3 = assign3050_e2932_d_n3;
        locals.var_gp0_dn4 = assign3050_e2932_d_n4;
        locals.var_gp0_dn5 = assign3050_e2932_d_n5;
        locals.var_gp0_dn6 = assign3050_e2932_d_n6;
        locals.var_gp0_dn7 = assign3050_e2932_d_n7;
        locals.var_gp0_dn8 = assign3050_e2932_d_n8;
        locals.var_gp0_dn9 = assign3050_e2932_d_n9;
        locals.var_gp0_dn10 = assign3050_e2932_d_n10;
        locals.var_gp0_dn11 = assign3050_e2932_d_n11;
        locals.var_gp0_rv = 0.0;

        let (assign3060_e2946, assign3060_e2946_d_n0, assign3060_e2946_d_n1, assign3060_e2946_d_n3, assign3060_e2946_d_n4, assign3060_e2946_d_n5, assign3060_e2946_d_n6, assign3060_e2946_d_n7, assign3060_e2946_d_n8, assign3060_e2946_d_n9, assign3060_e2946_d_n10, assign3060_e2946_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3060_e2936: f64 = (2.0 * locals.var_gp0);
        let assign3060_e2940: f64 = (locals.var_pw + locals.var_gp0);
        let assign3060_e2942: f64 = (assign3060_e2940 + 1.0);
        let assign3060_e2943: f64 = (locals.var_pw * assign3060_e2942);
        let assign3060_e2944: f64 = (assign3060_e2936 + assign3060_e2943);
        (assign3060_e2944, ((2.0 * locals.var_gp0_dn0) + ((locals.var_pw_dn0 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn0 + locals.var_gp0_dn0)))), ((2.0 * locals.var_gp0_dn1) + ((locals.var_pw_dn1 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn1 + locals.var_gp0_dn1)))), ((2.0 * locals.var_gp0_dn3) + ((locals.var_pw_dn3 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn3 + locals.var_gp0_dn3)))), ((2.0 * locals.var_gp0_dn4) + ((locals.var_pw_dn4 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn4 + locals.var_gp0_dn4)))), ((2.0 * locals.var_gp0_dn5) + ((locals.var_pw_dn5 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn5 + locals.var_gp0_dn5)))), ((2.0 * locals.var_gp0_dn6) + ((locals.var_pw_dn6 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn6 + locals.var_gp0_dn6)))), ((2.0 * locals.var_gp0_dn7) + ((locals.var_pw_dn7 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn7 + locals.var_gp0_dn7)))), ((2.0 * locals.var_gp0_dn8) + ((locals.var_pw_dn8 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn8 + locals.var_gp0_dn8)))), ((2.0 * locals.var_gp0_dn9) + ((locals.var_pw_dn9 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn9 + locals.var_gp0_dn9)))), ((2.0 * locals.var_gp0_dn10) + ((locals.var_pw_dn10 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn10 + locals.var_gp0_dn10)))), ((2.0 * locals.var_gp0_dn11) + ((locals.var_pw_dn11 * assign3060_e2942) + (locals.var_pw * (locals.var_pw_dn11 + locals.var_gp0_dn11)))),)
    } else {
        (locals.var_gp0_help, locals.var_gp0_help_dn0, locals.var_gp0_help_dn1, locals.var_gp0_help_dn3, locals.var_gp0_help_dn4, locals.var_gp0_help_dn5, locals.var_gp0_help_dn6, locals.var_gp0_help_dn7, locals.var_gp0_help_dn8, locals.var_gp0_help_dn9, locals.var_gp0_help_dn10, locals.var_gp0_help_dn11,)
    }
};
        locals.var_gp0_help = assign3060_e2946;
        locals.var_gp0_help_dn0 = assign3060_e2946_d_n0;
        locals.var_gp0_help_dn1 = assign3060_e2946_d_n1;
        locals.var_gp0_help_dn3 = assign3060_e2946_d_n3;
        locals.var_gp0_help_dn4 = assign3060_e2946_d_n4;
        locals.var_gp0_help_dn5 = assign3060_e2946_d_n5;
        locals.var_gp0_help_dn6 = assign3060_e2946_d_n6;
        locals.var_gp0_help_dn7 = assign3060_e2946_d_n7;
        locals.var_gp0_help_dn8 = assign3060_e2946_d_n8;
        locals.var_gp0_help_dn9 = assign3060_e2946_d_n9;
        locals.var_gp0_help_dn10 = assign3060_e2946_d_n10;
        locals.var_gp0_help_dn11 = assign3060_e2946_d_n11;
        locals.var_gp0_help_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3070_e2954, assign3070_e2954_d_n0, assign3070_e2954_d_n1, assign3070_e2954_d_n3, assign3070_e2954_d_n4, assign3070_e2954_d_n5, assign3070_e2954_d_n6, assign3070_e2954_d_n7, assign3070_e2954_d_n8, assign3070_e2954_d_n9, assign3070_e2954_d_n10, assign3070_e2954_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3070_e2951: f64 = (locals.var_gp0 - 1.0);
        let assign3070_e2952: f64 = (0.5 * assign3070_e2951);
        (assign3070_e2952, (0.5 * locals.var_gp0_dn0), (0.5 * locals.var_gp0_dn1), (0.5 * locals.var_gp0_dn3), (0.5 * locals.var_gp0_dn4), (0.5 * locals.var_gp0_dn5), (0.5 * locals.var_gp0_dn6), (0.5 * locals.var_gp0_dn7), (0.5 * locals.var_gp0_dn8), (0.5 * locals.var_gp0_dn9), (0.5 * locals.var_gp0_dn10), (0.5 * locals.var_gp0_dn11),)
    } else {
        (locals.var_gp02, locals.var_gp02_dn0, locals.var_gp02_dn1, locals.var_gp02_dn3, locals.var_gp02_dn4, locals.var_gp02_dn5, locals.var_gp02_dn6, locals.var_gp02_dn7, locals.var_gp02_dn8, locals.var_gp02_dn9, locals.var_gp02_dn10, locals.var_gp02_dn11,)
    }
};
        locals.var_gp02 = assign3070_e2954;
        locals.var_gp02_dn0 = assign3070_e2954_d_n0;
        locals.var_gp02_dn1 = assign3070_e2954_d_n1;
        locals.var_gp02_dn3 = assign3070_e2954_d_n3;
        locals.var_gp02_dn4 = assign3070_e2954_d_n4;
        locals.var_gp02_dn5 = assign3070_e2954_d_n5;
        locals.var_gp02_dn6 = assign3070_e2954_d_n6;
        locals.var_gp02_dn7 = assign3070_e2954_d_n7;
        locals.var_gp02_dn8 = assign3070_e2954_d_n8;
        locals.var_gp02_dn9 = assign3070_e2954_d_n9;
        locals.var_gp02_dn10 = assign3070_e2954_d_n10;
        locals.var_gp02_dn11 = assign3070_e2954_d_n11;
        locals.var_gp02_rv = 0.0;

        let (assign3080_e2962, assign3080_e2962_d_n0, assign3080_e2962_d_n1, assign3080_e2962_d_n3, assign3080_e2962_d_n4, assign3080_e2962_d_n5, assign3080_e2962_d_n6, assign3080_e2962_d_n7, assign3080_e2962_d_n8, assign3080_e2962_d_n9, assign3080_e2962_d_n10, assign3080_e2962_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3080_e2958: f64 = (locals.var_gp02 * locals.var_gp02);
        let assign3080_e2960: f64 = (assign3080_e2958 + locals.var_gp0_help);
        (assign3080_e2960, (((locals.var_gp02_dn0 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn0)) + locals.var_gp0_help_dn0), (((locals.var_gp02_dn1 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn1)) + locals.var_gp0_help_dn1), (((locals.var_gp02_dn3 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn3)) + locals.var_gp0_help_dn3), (((locals.var_gp02_dn4 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn4)) + locals.var_gp0_help_dn4), (((locals.var_gp02_dn5 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn5)) + locals.var_gp0_help_dn5), (((locals.var_gp02_dn6 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn6)) + locals.var_gp0_help_dn6), (((locals.var_gp02_dn7 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn7)) + locals.var_gp0_help_dn7), (((locals.var_gp02_dn8 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn8)) + locals.var_gp0_help_dn8), (((locals.var_gp02_dn9 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn9)) + locals.var_gp0_help_dn9), (((locals.var_gp02_dn10 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn10)) + locals.var_gp0_help_dn10), (((locals.var_gp02_dn11 * locals.var_gp02) + (locals.var_gp02 * locals.var_gp02_dn11)) + locals.var_gp0_help_dn11),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10, locals.var_sqr_arg_dn11,)
    }
};
        locals.var_sqr_arg = assign3080_e2962;
        locals.var_sqr_arg_dn0 = assign3080_e2962_d_n0;
        locals.var_sqr_arg_dn1 = assign3080_e2962_d_n1;
        locals.var_sqr_arg_dn3 = assign3080_e2962_d_n3;
        locals.var_sqr_arg_dn4 = assign3080_e2962_d_n4;
        locals.var_sqr_arg_dn5 = assign3080_e2962_d_n5;
        locals.var_sqr_arg_dn6 = assign3080_e2962_d_n6;
        locals.var_sqr_arg_dn7 = assign3080_e2962_d_n7;
        locals.var_sqr_arg_dn8 = assign3080_e2962_d_n8;
        locals.var_sqr_arg_dn9 = assign3080_e2962_d_n9;
        locals.var_sqr_arg_dn10 = assign3080_e2962_d_n10;
        locals.var_sqr_arg_dn11 = assign3080_e2962_d_n11;
        locals.var_sqr_arg_rv = 0.0;

        let assign3090_e2965: f64 = if locals.var_gp0 >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign3090_e2965;
        locals.var_guard49_rv = 0.0;

        let (assign3100_e2974, assign3100_e2974_d_n0, assign3100_e2974_d_n1, assign3100_e2974_d_n3, assign3100_e2974_d_n4, assign3100_e2974_d_n5, assign3100_e2974_d_n6, assign3100_e2974_d_n7, assign3100_e2974_d_n8, assign3100_e2974_d_n9, assign3100_e2974_d_n10, assign3100_e2974_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign3100_e2971: f64 = (locals.var_sqr_arg).sqrt();
        let assign3100_e2972: f64 = (locals.var_gp02 + assign3100_e2971);
        (assign3100_e2972, (locals.var_gp02_dn0 + (locals.var_sqr_arg_dn0 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn1 + (locals.var_sqr_arg_dn1 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn3 + (locals.var_sqr_arg_dn3 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn4 + (locals.var_sqr_arg_dn4 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn5 + (locals.var_sqr_arg_dn5 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn6 + (locals.var_sqr_arg_dn6 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn7 + (locals.var_sqr_arg_dn7 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn8 + (locals.var_sqr_arg_dn8 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn9 + (locals.var_sqr_arg_dn9 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn10 + (locals.var_sqr_arg_dn10 / (2.0 * assign3100_e2971))), (locals.var_gp02_dn11 + (locals.var_sqr_arg_dn11 / (2.0 * assign3100_e2971))),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10, locals.var_p0star_dn11,)
    }
};
        locals.var_p0star = assign3100_e2974;
        locals.var_p0star_dn0 = assign3100_e2974_d_n0;
        locals.var_p0star_dn1 = assign3100_e2974_d_n1;
        locals.var_p0star_dn3 = assign3100_e2974_d_n3;
        locals.var_p0star_dn4 = assign3100_e2974_d_n4;
        locals.var_p0star_dn5 = assign3100_e2974_d_n5;
        locals.var_p0star_dn6 = assign3100_e2974_d_n6;
        locals.var_p0star_dn7 = assign3100_e2974_d_n7;
        locals.var_p0star_dn8 = assign3100_e2974_d_n8;
        locals.var_p0star_dn9 = assign3100_e2974_d_n9;
        locals.var_p0star_dn10 = assign3100_e2974_d_n10;
        locals.var_p0star_dn11 = assign3100_e2974_d_n11;
        locals.var_p0star_rv = 0.0;

        let (assign3110_e2986, assign3110_e2986_d_n0, assign3110_e2986_d_n1, assign3110_e2986_d_n3, assign3110_e2986_d_n4, assign3110_e2986_d_n5, assign3110_e2986_d_n6, assign3110_e2986_d_n7, assign3110_e2986_d_n8, assign3110_e2986_d_n9, assign3110_e2986_d_n10, assign3110_e2986_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign3110_e2981: f64 = (locals.var_sqr_arg).sqrt();
        let assign3110_e2983: f64 = (assign3110_e2981 - locals.var_gp02);
        let assign3110_e2984: f64 = (locals.var_gp0_help / assign3110_e2983);
        (assign3110_e2984, (((locals.var_gp0_help_dn0 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn0 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn0))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn1 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn1 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn1))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn3 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn3 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn3))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn4 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn4 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn4))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn5 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn5 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn5))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn6 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn6 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn6))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn7 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn7 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn7))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn8 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn8 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn8))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn9 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn9 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn9))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn10 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn10 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn10))) / (assign3110_e2983 * assign3110_e2983)), (((locals.var_gp0_help_dn11 * assign3110_e2983) - (locals.var_gp0_help * ((locals.var_sqr_arg_dn11 / (2.0 * assign3110_e2981)) - locals.var_gp02_dn11))) / (assign3110_e2983 * assign3110_e2983)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10, locals.var_p0star_dn11,)
    }
};
        locals.var_p0star = assign3110_e2986;
        locals.var_p0star_dn0 = assign3110_e2986_d_n0;
        locals.var_p0star_dn1 = assign3110_e2986_d_n1;
        locals.var_p0star_dn3 = assign3110_e2986_d_n3;
        locals.var_p0star_dn4 = assign3110_e2986_d_n4;
        locals.var_p0star_dn5 = assign3110_e2986_d_n5;
        locals.var_p0star_dn6 = assign3110_e2986_d_n6;
        locals.var_p0star_dn7 = assign3110_e2986_d_n7;
        locals.var_p0star_dn8 = assign3110_e2986_d_n8;
        locals.var_p0star_dn9 = assign3110_e2986_d_n9;
        locals.var_p0star_dn10 = assign3110_e2986_d_n10;
        locals.var_p0star_dn11 = assign3110_e2986_d_n11;
        locals.var_p0star_rv = 0.0;

        let assign3120_e2989: f64 = if locals.var_p0star < p.p152 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign3120_e2989;
        locals.var_guard50_rv = 0.0;

        let (assign3130_e2995, assign3130_e2995_d_n0, assign3130_e2995_d_n1, assign3130_e2995_d_n3, assign3130_e2995_d_n4, assign3130_e2995_d_n5, assign3130_e2995_d_n6, assign3130_e2995_d_n7, assign3130_e2995_d_n8, assign3130_e2995_d_n9, assign3130_e2995_d_n10, assign3130_e2995_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard50 != 0.0)) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10, locals.var_p0star_dn11,)
    }
};
        locals.var_p0star = assign3130_e2995;
        locals.var_p0star_dn0 = assign3130_e2995_d_n0;
        locals.var_p0star_dn1 = assign3130_e2995_d_n1;
        locals.var_p0star_dn3 = assign3130_e2995_d_n3;
        locals.var_p0star_dn4 = assign3130_e2995_d_n4;
        locals.var_p0star_dn5 = assign3130_e2995_d_n5;
        locals.var_p0star_dn6 = assign3130_e2995_d_n6;
        locals.var_p0star_dn7 = assign3130_e2995_d_n7;
        locals.var_p0star_dn8 = assign3130_e2995_d_n8;
        locals.var_p0star_dn9 = assign3130_e2995_d_n9;
        locals.var_p0star_dn10 = assign3130_e2995_d_n10;
        locals.var_p0star_dn11 = assign3130_e2995_d_n11;
        locals.var_p0star_rv = 0.0;

        let (assign3140_e3008, assign3140_e3008_d_n0, assign3140_e3008_d_n1, assign3140_e3008_d_n3, assign3140_e3008_d_n4, assign3140_e3008_d_n5, assign3140_e3008_d_n6, assign3140_e3008_d_n7, assign3140_e3008_d_n8, assign3140_e3008_d_n9, assign3140_e3008_d_n10, assign3140_e3008_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3140_e3000: f64 = (locals.var_p0star + 1.0);
        let assign3140_e3001: f64 = (locals.var_p0star * assign3140_e3000);
        let assign3140_e3004: f64 = (locals.var_vdc_t * locals.var_vtinv);
        let assign3140_e3005: f64 = (assign3140_e3004).exp();
        let assign3140_e3006: f64 = (assign3140_e3001 * assign3140_e3005);
        (assign3140_e3006, ((((locals.var_p0star_dn0 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn0)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn0 * locals.var_vtinv)))), ((((locals.var_p0star_dn1 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn1)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn1 * locals.var_vtinv)))), ((((locals.var_p0star_dn3 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn3)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn3 * locals.var_vtinv)))), ((((locals.var_p0star_dn4 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn4)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * ((locals.var_vdc_t_dn4 * locals.var_vtinv) + (locals.var_vdc_t * locals.var_vtinv_dn4))))), ((((locals.var_p0star_dn5 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn5)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn5 * locals.var_vtinv)))), ((((locals.var_p0star_dn6 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn6)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn6 * locals.var_vtinv)))), ((((locals.var_p0star_dn7 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn7)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn7 * locals.var_vtinv)))), ((((locals.var_p0star_dn8 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn8)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn8 * locals.var_vtinv)))), ((((locals.var_p0star_dn9 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn9)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn9 * locals.var_vtinv)))), ((((locals.var_p0star_dn10 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn10)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn10 * locals.var_vtinv)))), ((((locals.var_p0star_dn11 * assign3140_e3000) + (locals.var_p0star * locals.var_p0star_dn11)) * assign3140_e3005) + (assign3140_e3001 * (assign3140_e3005 * (locals.var_vdc_t_dn11 * locals.var_vtinv)))),)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9, locals.var_evb2c2star_dn10, locals.var_evb2c2star_dn11,)
    }
};
        locals.var_evb2c2star = assign3140_e3008;
        locals.var_evb2c2star_dn0 = assign3140_e3008_d_n0;
        locals.var_evb2c2star_dn1 = assign3140_e3008_d_n1;
        locals.var_evb2c2star_dn3 = assign3140_e3008_d_n3;
        locals.var_evb2c2star_dn4 = assign3140_e3008_d_n4;
        locals.var_evb2c2star_dn5 = assign3140_e3008_d_n5;
        locals.var_evb2c2star_dn6 = assign3140_e3008_d_n6;
        locals.var_evb2c2star_dn7 = assign3140_e3008_d_n7;
        locals.var_evb2c2star_dn8 = assign3140_e3008_d_n8;
        locals.var_evb2c2star_dn9 = assign3140_e3008_d_n9;
        locals.var_evb2c2star_dn10 = assign3140_e3008_d_n10;
        locals.var_evb2c2star_dn11 = assign3140_e3008_d_n11;
        locals.var_evb2c2star_rv = 0.0;

        let (assign3150_e3018, assign3150_e3018_d_n0, assign3150_e3018_d_n1, assign3150_e3018_d_n3, assign3150_e3018_d_n4, assign3150_e3018_d_n5, assign3150_e3018_d_n6, assign3150_e3018_d_n7, assign3150_e3018_d_n8, assign3150_e3018_d_n9, assign3150_e3018_d_n10, assign3150_e3018_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3150_e3012: f64 = (0.5 * p.p61);
        let assign3150_e3015: f64 = (locals.var_ic1c2 - p.p62);
        let assign3150_e3016: f64 = (assign3150_e3012 * assign3150_e3015);
        (assign3150_e3016, (assign3150_e3012 * locals.var_ic1c2_dn0), (assign3150_e3012 * locals.var_ic1c2_dn1), (assign3150_e3012 * locals.var_ic1c2_dn3), (assign3150_e3012 * locals.var_ic1c2_dn4), (assign3150_e3012 * locals.var_ic1c2_dn5), (assign3150_e3012 * locals.var_ic1c2_dn6), (assign3150_e3012 * locals.var_ic1c2_dn7), (assign3150_e3012 * locals.var_ic1c2_dn8), (assign3150_e3012 * locals.var_ic1c2_dn9), (assign3150_e3012 * locals.var_ic1c2_dn10), (assign3150_e3012 * locals.var_ic1c2_dn11),)
    } else {
        (locals.var_b1, locals.var_b1_dn0, locals.var_b1_dn1, locals.var_b1_dn3, locals.var_b1_dn4, locals.var_b1_dn5, locals.var_b1_dn6, locals.var_b1_dn7, locals.var_b1_dn8, locals.var_b1_dn9, locals.var_b1_dn10, locals.var_b1_dn11,)
    }
};
        locals.var_b1 = assign3150_e3018;
        locals.var_b1_dn0 = assign3150_e3018_d_n0;
        locals.var_b1_dn1 = assign3150_e3018_d_n1;
        locals.var_b1_dn3 = assign3150_e3018_d_n3;
        locals.var_b1_dn4 = assign3150_e3018_d_n4;
        locals.var_b1_dn5 = assign3150_e3018_d_n5;
        locals.var_b1_dn6 = assign3150_e3018_d_n6;
        locals.var_b1_dn7 = assign3150_e3018_d_n7;
        locals.var_b1_dn8 = assign3150_e3018_d_n8;
        locals.var_b1_dn9 = assign3150_e3018_d_n9;
        locals.var_b1_dn10 = assign3150_e3018_d_n10;
        locals.var_b1_dn11 = assign3150_e3018_d_n11;
        locals.var_b1_rv = 0.0;

        let (assign3160_e3028, assign3160_e3028_d_n0, assign3160_e3028_d_n1, assign3160_e3028_d_n3, assign3160_e3028_d_n4, assign3160_e3028_d_n5, assign3160_e3028_d_n6, assign3160_e3028_d_n7, assign3160_e3028_d_n8, assign3160_e3028_d_n9, assign3160_e3028_d_n10, assign3160_e3028_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3160_e3022: f64 = (p.p61 * locals.var_rcv_t);
        let assign3160_e3024: f64 = (assign3160_e3022 * p.p62);
        let assign3160_e3026: f64 = (assign3160_e3024 * locals.var_ic1c2);
        (assign3160_e3026, (assign3160_e3024 * locals.var_ic1c2_dn0), (assign3160_e3024 * locals.var_ic1c2_dn1), (assign3160_e3024 * locals.var_ic1c2_dn3), ((((p.p61 * locals.var_rcv_t_dn4) * p.p62) * locals.var_ic1c2) + (assign3160_e3024 * locals.var_ic1c2_dn4)), (assign3160_e3024 * locals.var_ic1c2_dn5), (assign3160_e3024 * locals.var_ic1c2_dn6), (assign3160_e3024 * locals.var_ic1c2_dn7), (assign3160_e3024 * locals.var_ic1c2_dn8), (assign3160_e3024 * locals.var_ic1c2_dn9), (assign3160_e3024 * locals.var_ic1c2_dn10), (assign3160_e3024 * locals.var_ic1c2_dn11),)
    } else {
        (locals.var_b2, locals.var_b2_dn0, locals.var_b2_dn1, locals.var_b2_dn3, locals.var_b2_dn4, locals.var_b2_dn5, locals.var_b2_dn6, locals.var_b2_dn7, locals.var_b2_dn8, locals.var_b2_dn9, locals.var_b2_dn10, locals.var_b2_dn11,)
    }
};
        locals.var_b2 = assign3160_e3028;
        locals.var_b2_dn0 = assign3160_e3028_d_n0;
        locals.var_b2_dn1 = assign3160_e3028_d_n1;
        locals.var_b2_dn3 = assign3160_e3028_d_n3;
        locals.var_b2_dn4 = assign3160_e3028_d_n4;
        locals.var_b2_dn5 = assign3160_e3028_d_n5;
        locals.var_b2_dn6 = assign3160_e3028_d_n6;
        locals.var_b2_dn7 = assign3160_e3028_d_n7;
        locals.var_b2_dn8 = assign3160_e3028_d_n8;
        locals.var_b2_dn9 = assign3160_e3028_d_n9;
        locals.var_b2_dn10 = assign3160_e3028_d_n10;
        locals.var_b2_dn11 = assign3160_e3028_d_n11;
        locals.var_b2_rv = 0.0;

        let (assign3170_e3039, assign3170_e3039_d_n0, assign3170_e3039_d_n1, assign3170_e3039_d_n3, assign3170_e3039_d_n4, assign3170_e3039_d_n5, assign3170_e3039_d_n6, assign3170_e3039_d_n7, assign3170_e3039_d_n8, assign3170_e3039_d_n9, assign3170_e3039_d_n10, assign3170_e3039_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3170_e3033: f64 = (locals.var_b1 * locals.var_b1);
        let assign3170_e3035: f64 = (assign3170_e3033 + locals.var_b2);
        let assign3170_e3036: f64 = (assign3170_e3035).sqrt();
        let assign3170_e3037: f64 = (locals.var_b1 + assign3170_e3036);
        (assign3170_e3037, (locals.var_b1_dn0 + ((((locals.var_b1_dn0 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn0)) + locals.var_b2_dn0) / (2.0 * assign3170_e3036))), (locals.var_b1_dn1 + ((((locals.var_b1_dn1 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn1)) + locals.var_b2_dn1) / (2.0 * assign3170_e3036))), (locals.var_b1_dn3 + ((((locals.var_b1_dn3 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn3)) + locals.var_b2_dn3) / (2.0 * assign3170_e3036))), (locals.var_b1_dn4 + ((((locals.var_b1_dn4 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn4)) + locals.var_b2_dn4) / (2.0 * assign3170_e3036))), (locals.var_b1_dn5 + ((((locals.var_b1_dn5 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn5)) + locals.var_b2_dn5) / (2.0 * assign3170_e3036))), (locals.var_b1_dn6 + ((((locals.var_b1_dn6 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn6)) + locals.var_b2_dn6) / (2.0 * assign3170_e3036))), (locals.var_b1_dn7 + ((((locals.var_b1_dn7 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn7)) + locals.var_b2_dn7) / (2.0 * assign3170_e3036))), (locals.var_b1_dn8 + ((((locals.var_b1_dn8 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn8)) + locals.var_b2_dn8) / (2.0 * assign3170_e3036))), (locals.var_b1_dn9 + ((((locals.var_b1_dn9 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn9)) + locals.var_b2_dn9) / (2.0 * assign3170_e3036))), (locals.var_b1_dn10 + ((((locals.var_b1_dn10 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn10)) + locals.var_b2_dn10) / (2.0 * assign3170_e3036))), (locals.var_b1_dn11 + ((((locals.var_b1_dn11 * locals.var_b1) + (locals.var_b1 * locals.var_b1_dn11)) + locals.var_b2_dn11) / (2.0 * assign3170_e3036))),)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9, locals.var_vxi0_dn10, locals.var_vxi0_dn11,)
    }
};
        locals.var_vxi0 = assign3170_e3039;
        locals.var_vxi0_dn0 = assign3170_e3039_d_n0;
        locals.var_vxi0_dn1 = assign3170_e3039_d_n1;
        locals.var_vxi0_dn3 = assign3170_e3039_d_n3;
        locals.var_vxi0_dn4 = assign3170_e3039_d_n4;
        locals.var_vxi0_dn5 = assign3170_e3039_d_n5;
        locals.var_vxi0_dn6 = assign3170_e3039_d_n6;
        locals.var_vxi0_dn7 = assign3170_e3039_d_n7;
        locals.var_vxi0_dn8 = assign3170_e3039_d_n8;
        locals.var_vxi0_dn9 = assign3170_e3039_d_n9;
        locals.var_vxi0_dn10 = assign3170_e3039_d_n10;
        locals.var_vxi0_dn11 = assign3170_e3039_d_n11;
        locals.var_vxi0_rv = 0.0;

        let assign3180_e3042: f64 = if p.p73 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign3180_e3042;
        locals.var_guard51_rv = 0.0;

        let (assign3190_e3050, assign3190_e3050_d_n0, assign3190_e3050_d_n1, assign3190_e3050_d_n3, assign3190_e3050_d_n4, assign3190_e3050_d_n5, assign3190_e3050_d_n6, assign3190_e3050_d_n7, assign3190_e3050_d_n8, assign3190_e3050_d_n9, assign3190_e3050_d_n10, assign3190_e3050_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign3190_e3048: f64 = (locals.var_vdc_ctc_t * 0.1);
        (assign3190_e3048, (locals.var_vdc_ctc_t_dn0 * 0.1), (locals.var_vdc_ctc_t_dn1 * 0.1), (locals.var_vdc_ctc_t_dn3 * 0.1), (locals.var_vdc_ctc_t_dn4 * 0.1), (locals.var_vdc_ctc_t_dn5 * 0.1), (locals.var_vdc_ctc_t_dn6 * 0.1), (locals.var_vdc_ctc_t_dn7 * 0.1), (locals.var_vdc_ctc_t_dn8 * 0.1), (locals.var_vdc_ctc_t_dn9 * 0.1), (locals.var_vdc_ctc_t_dn10 * 0.1), (locals.var_vdc_ctc_t_dn11 * 0.1),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10, locals.var_vch_dn11,)
    }
};
        locals.var_vch = assign3190_e3050;
        locals.var_vch_dn0 = assign3190_e3050_d_n0;
        locals.var_vch_dn1 = assign3190_e3050_d_n1;
        locals.var_vch_dn3 = assign3190_e3050_d_n3;
        locals.var_vch_dn4 = assign3190_e3050_d_n4;
        locals.var_vch_dn5 = assign3190_e3050_d_n5;
        locals.var_vch_dn6 = assign3190_e3050_d_n6;
        locals.var_vch_dn7 = assign3190_e3050_d_n7;
        locals.var_vch_dn8 = assign3190_e3050_d_n8;
        locals.var_vch_dn9 = assign3190_e3050_d_n9;
        locals.var_vch_dn10 = assign3190_e3050_d_n10;
        locals.var_vch_dn11 = assign3190_e3050_d_n11;
        locals.var_vch_rv = 0.0;

        let (assign3200_e3067, assign3200_e3067_d_n0, assign3200_e3067_d_n1, assign3200_e3067_d_n3, assign3200_e3067_d_n4, assign3200_e3067_d_n5, assign3200_e3067_d_n6, assign3200_e3067_d_n7, assign3200_e3067_d_n8, assign3200_e3067_d_n9, assign3200_e3067_d_n10, assign3200_e3067_d_n11,) = {
    if ((locals.var_guard45 != 0.0) && (locals.var_guard51 == 0.0)) {
        let assign3200_e3059: f64 = (2.0 * locals.var_ic1c2);
        let assign3200_e3062: f64 = (locals.var_ic1c2 + locals.var_iqs);
        let assign3200_e3063: f64 = (assign3200_e3059 / assign3200_e3062);
        let assign3200_e3064: f64 = (0.1 + assign3200_e3063);
        let assign3200_e3065: f64 = (locals.var_vdc_ctc_t * assign3200_e3064);
        (assign3200_e3065, ((locals.var_vdc_ctc_t_dn0 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn0) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn0 + locals.var_iqs_dn0))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn1 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn1) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn1 + locals.var_iqs_dn1))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn3 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn3) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn3 + locals.var_iqs_dn3))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn4 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn4) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn4 + locals.var_iqs_dn4))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn5 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn5) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn5 + locals.var_iqs_dn5))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn6 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn6) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn6 + locals.var_iqs_dn6))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn7 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn7) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn7 + locals.var_iqs_dn7))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn8 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn8) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn8 + locals.var_iqs_dn8))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn9 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn9) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn9 + locals.var_iqs_dn9))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn10 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn10) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn10 + locals.var_iqs_dn10))) / (assign3200_e3062 * assign3200_e3062)))), ((locals.var_vdc_ctc_t_dn11 * assign3200_e3064) + (locals.var_vdc_ctc_t * ((((2.0 * locals.var_ic1c2_dn11) * assign3200_e3062) - (assign3200_e3059 * (locals.var_ic1c2_dn11 + locals.var_iqs_dn11))) / (assign3200_e3062 * assign3200_e3062)))),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10, locals.var_vch_dn11,)
    }
};
        locals.var_vch = assign3200_e3067;
        locals.var_vch_dn0 = assign3200_e3067_d_n0;
        locals.var_vch_dn1 = assign3200_e3067_d_n1;
        locals.var_vch_dn3 = assign3200_e3067_d_n3;
        locals.var_vch_dn4 = assign3200_e3067_d_n4;
        locals.var_vch_dn5 = assign3200_e3067_d_n5;
        locals.var_vch_dn6 = assign3200_e3067_d_n6;
        locals.var_vch_dn7 = assign3200_e3067_d_n7;
        locals.var_vch_dn8 = assign3200_e3067_d_n8;
        locals.var_vch_dn9 = assign3200_e3067_d_n9;
        locals.var_vch_dn10 = assign3200_e3067_d_n10;
        locals.var_vch_dn11 = assign3200_e3067_d_n11;
        locals.var_vch_rv = 0.0;

        let (assign3210_e3077, assign3210_e3077_d_n0, assign3210_e3077_d_n1, assign3210_e3077_d_n3, assign3210_e3077_d_n4, assign3210_e3077_d_n5, assign3210_e3077_d_n6, assign3210_e3077_d_n7, assign3210_e3077_d_n8, assign3210_e3077_d_n9, assign3210_e3077_d_n10, assign3210_e3077_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3210_e3071: f64 = (p.p62 * locals.var_ic1c2);
        let assign3210_e3074: f64 = (p.p62 + locals.var_ic1c2);
        let assign3210_e3075: f64 = (assign3210_e3071 / assign3210_e3074);
        (assign3210_e3075, ((((p.p62 * locals.var_ic1c2_dn0) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn0)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn1) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn1)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn3) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn3)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn4) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn4)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn5) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn5)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn6) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn6)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn7) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn7)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn8) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn8)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn9) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn9)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn10) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn10)) / (assign3210_e3074 * assign3210_e3074)), ((((p.p62 * locals.var_ic1c2_dn11) * assign3210_e3074) - (assign3210_e3071 * locals.var_ic1c2_dn11)) / (assign3210_e3074 * assign3210_e3074)),)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9, locals.var_icap_dn10, locals.var_icap_dn11,)
    }
};
        locals.var_icap = assign3210_e3077;
        locals.var_icap_dn0 = assign3210_e3077_d_n0;
        locals.var_icap_dn1 = assign3210_e3077_d_n1;
        locals.var_icap_dn3 = assign3210_e3077_d_n3;
        locals.var_icap_dn4 = assign3210_e3077_d_n4;
        locals.var_icap_dn5 = assign3210_e3077_d_n5;
        locals.var_icap_dn6 = assign3210_e3077_d_n6;
        locals.var_icap_dn7 = assign3210_e3077_d_n7;
        locals.var_icap_dn8 = assign3210_e3077_d_n8;
        locals.var_icap_dn9 = assign3210_e3077_d_n9;
        locals.var_icap_dn10 = assign3210_e3077_d_n10;
        locals.var_icap_dn11 = assign3210_e3077_d_n11;
        locals.var_icap_rv = 0.0;

        let (assign3220_e3085, assign3220_e3085_d_n0, assign3220_e3085_d_n1, assign3220_e3085_d_n3, assign3220_e3085_d_n4, assign3220_e3085_d_n5, assign3220_e3085_d_n6, assign3220_e3085_d_n7, assign3220_e3085_d_n8, assign3220_e3085_d_n9, assign3220_e3085_d_n10, assign3220_e3085_d_n11,) = {
    if (locals.var_guard45 != 0.0) {
        let assign3220_e3082: f64 = (p.p62 + locals.var_ic1c2);
        let assign3220_e3083: f64 = (p.p62 / assign3220_e3082);
        (assign3220_e3083, (-((p.p62 * locals.var_ic1c2_dn0) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn1) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn3) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn4) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn5) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn6) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn7) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn8) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn9) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn10) / (assign3220_e3082 * assign3220_e3082))), (-((p.p62 * locals.var_ic1c2_dn11) / (assign3220_e3082 * assign3220_e3082))),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9, locals.var_icap_ihc_dn10, locals.var_icap_ihc_dn11,)
    }
};
        locals.var_icap_ihc = assign3220_e3085;
        locals.var_icap_ihc_dn0 = assign3220_e3085_d_n0;
        locals.var_icap_ihc_dn1 = assign3220_e3085_d_n1;
        locals.var_icap_ihc_dn3 = assign3220_e3085_d_n3;
        locals.var_icap_ihc_dn4 = assign3220_e3085_d_n4;
        locals.var_icap_ihc_dn5 = assign3220_e3085_d_n5;
        locals.var_icap_ihc_dn6 = assign3220_e3085_d_n6;
        locals.var_icap_ihc_dn7 = assign3220_e3085_d_n7;
        locals.var_icap_ihc_dn8 = assign3220_e3085_d_n8;
        locals.var_icap_ihc_dn9 = assign3220_e3085_d_n9;
        locals.var_icap_ihc_dn10 = assign3220_e3085_d_n10;
        locals.var_icap_ihc_dn11 = assign3220_e3085_d_n11;
        locals.var_icap_ihc_rv = 0.0;

        let (assign3230_e3090, assign3230_e3090_d_n0, assign3230_e3090_d_n1, assign3230_e3090_d_n3, assign3230_e3090_d_n4, assign3230_e3090_d_n5, assign3230_e3090_d_n6, assign3230_e3090_d_n7, assign3230_e3090_d_n8, assign3230_e3090_d_n9, assign3230_e3090_d_n10, assign3230_e3090_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqs, locals.var_iqs_dn0, locals.var_iqs_dn1, locals.var_iqs_dn3, locals.var_iqs_dn4, locals.var_iqs_dn5, locals.var_iqs_dn6, locals.var_iqs_dn7, locals.var_iqs_dn8, locals.var_iqs_dn9, locals.var_iqs_dn10, locals.var_iqs_dn11,)
    }
};
        locals.var_iqs = assign3230_e3090;
        locals.var_iqs_dn0 = assign3230_e3090_d_n0;
        locals.var_iqs_dn1 = assign3230_e3090_d_n1;
        locals.var_iqs_dn3 = assign3230_e3090_d_n3;
        locals.var_iqs_dn4 = assign3230_e3090_d_n4;
        locals.var_iqs_dn5 = assign3230_e3090_d_n5;
        locals.var_iqs_dn6 = assign3230_e3090_d_n6;
        locals.var_iqs_dn7 = assign3230_e3090_d_n7;
        locals.var_iqs_dn8 = assign3230_e3090_d_n8;
        locals.var_iqs_dn9 = assign3230_e3090_d_n9;
        locals.var_iqs_dn10 = assign3230_e3090_d_n10;
        locals.var_iqs_dn11 = assign3230_e3090_d_n11;
        locals.var_iqs_rv = 0.0;

        let (assign3240_e3101, assign3240_e3101_d_n0, assign3240_e3101_d_n1, assign3240_e3101_d_n3, assign3240_e3101_d_n4, assign3240_e3101_d_n5, assign3240_e3101_d_n6, assign3240_e3101_d_n7, assign3240_e3101_d_n8, assign3240_e3101_d_n9, assign3240_e3101_d_n10, assign3240_e3101_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        let assign3240_e3095: f64 = (2.0 * locals.var_evb2c2vdc);
        let assign3240_e3098: f64 = (1.0 + locals.var_k0);
        let assign3240_e3099: f64 = (assign3240_e3095 / assign3240_e3098);
        (assign3240_e3099, ((((2.0 * locals.var_evb2c2vdc_dn0) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn0)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn1) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn1)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn3) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn3)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn4) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn4)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn5) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn5)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn6) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn6)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn7) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn7)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn8) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn8)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn9) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn9)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn10) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn10)) / (assign3240_e3098 * assign3240_e3098)), ((((2.0 * locals.var_evb2c2vdc_dn11) * assign3240_e3098) - (assign3240_e3095 * locals.var_k0_dn11)) / (assign3240_e3098 * assign3240_e3098)),)
    } else {
        (locals.var_p0star, locals.var_p0star_dn0, locals.var_p0star_dn1, locals.var_p0star_dn3, locals.var_p0star_dn4, locals.var_p0star_dn5, locals.var_p0star_dn6, locals.var_p0star_dn7, locals.var_p0star_dn8, locals.var_p0star_dn9, locals.var_p0star_dn10, locals.var_p0star_dn11,)
    }
};
        locals.var_p0star = assign3240_e3101;
        locals.var_p0star_dn0 = assign3240_e3101_d_n0;
        locals.var_p0star_dn1 = assign3240_e3101_d_n1;
        locals.var_p0star_dn3 = assign3240_e3101_d_n3;
        locals.var_p0star_dn4 = assign3240_e3101_d_n4;
        locals.var_p0star_dn5 = assign3240_e3101_d_n5;
        locals.var_p0star_dn6 = assign3240_e3101_d_n6;
        locals.var_p0star_dn7 = assign3240_e3101_d_n7;
        locals.var_p0star_dn8 = assign3240_e3101_d_n8;
        locals.var_p0star_dn9 = assign3240_e3101_d_n9;
        locals.var_p0star_dn10 = assign3240_e3101_d_n10;
        locals.var_p0star_dn11 = assign3240_e3101_d_n11;
        locals.var_p0star_rv = 0.0;

        let (assign3250_e3106, assign3250_e3106_d_n0, assign3250_e3106_d_n1, assign3250_e3106_d_n3, assign3250_e3106_d_n4, assign3250_e3106_d_n5, assign3250_e3106_d_n6, assign3250_e3106_d_n7, assign3250_e3106_d_n8, assign3250_e3106_d_n9, assign3250_e3106_d_n10, assign3250_e3106_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        (locals.var_evb2c2, 0.0, 0.0, 0.0, locals.var_evb2c2_dn4, 0.0, 0.0, locals.var_evb2c2_dn7, 0.0, locals.var_evb2c2_dn9, 0.0, 0.0,)
    } else {
        (locals.var_evb2c2star, locals.var_evb2c2star_dn0, locals.var_evb2c2star_dn1, locals.var_evb2c2star_dn3, locals.var_evb2c2star_dn4, locals.var_evb2c2star_dn5, locals.var_evb2c2star_dn6, locals.var_evb2c2star_dn7, locals.var_evb2c2star_dn8, locals.var_evb2c2star_dn9, locals.var_evb2c2star_dn10, locals.var_evb2c2star_dn11,)
    }
};
        locals.var_evb2c2star = assign3250_e3106;
        locals.var_evb2c2star_dn0 = assign3250_e3106_d_n0;
        locals.var_evb2c2star_dn1 = assign3250_e3106_d_n1;
        locals.var_evb2c2star_dn3 = assign3250_e3106_d_n3;
        locals.var_evb2c2star_dn4 = assign3250_e3106_d_n4;
        locals.var_evb2c2star_dn5 = assign3250_e3106_d_n5;
        locals.var_evb2c2star_dn6 = assign3250_e3106_d_n6;
        locals.var_evb2c2star_dn7 = assign3250_e3106_d_n7;
        locals.var_evb2c2star_dn8 = assign3250_e3106_d_n8;
        locals.var_evb2c2star_dn9 = assign3250_e3106_d_n9;
        locals.var_evb2c2star_dn10 = assign3250_e3106_d_n10;
        locals.var_evb2c2star_dn11 = assign3250_e3106_d_n11;
        locals.var_evb2c2star_rv = 0.0;

        let assign3260_e3108: f64 = (locals.var_vc1c2).abs();
        let assign3260_e3111: f64 = (1e-5 * locals.var_vt);
        let assign3260_e3114: f64 = (locals.var_ec).abs();
        let assign3260_e3117: f64 = (1e-40 * locals.var_vt);
        let assign3260_e3120: f64 = (locals.var_k0 + locals.var_kw);
        let assign3260_e3121: f64 = (assign3260_e3117 * assign3260_e3120);
        let assign3260_e3123: f64 = if ((assign3260_e3108 < assign3260_e3111) || (assign3260_e3114 < assign3260_e3121)) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign3260_e3123;
        locals.var_guard52_rv = 0.0;

        let (assign3270_e3134, assign3270_e3134_d_n0, assign3270_e3134_d_n1, assign3270_e3134_d_n3, assign3270_e3134_d_n4, assign3270_e3134_d_n5, assign3270_e3134_d_n6, assign3270_e3134_d_n7, assign3270_e3134_d_n8, assign3270_e3134_d_n9, assign3270_e3134_d_n10, assign3270_e3134_d_n11,) = {
    if ((locals.var_guard45 == 0.0) && (locals.var_guard52 != 0.0)) {
        let assign3270_e3131: f64 = (locals.var_p0star + locals.var_pw);
        let assign3270_e3132: f64 = (0.5 * assign3270_e3131);
        (assign3270_e3132, (0.5 * (locals.var_p0star_dn0 + locals.var_pw_dn0)), (0.5 * (locals.var_p0star_dn1 + locals.var_pw_dn1)), (0.5 * (locals.var_p0star_dn3 + locals.var_pw_dn3)), (0.5 * (locals.var_p0star_dn4 + locals.var_pw_dn4)), (0.5 * (locals.var_p0star_dn5 + locals.var_pw_dn5)), (0.5 * (locals.var_p0star_dn6 + locals.var_pw_dn6)), (0.5 * (locals.var_p0star_dn7 + locals.var_pw_dn7)), (0.5 * (locals.var_p0star_dn8 + locals.var_pw_dn8)), (0.5 * (locals.var_p0star_dn9 + locals.var_pw_dn9)), (0.5 * (locals.var_p0star_dn10 + locals.var_pw_dn10)), (0.5 * (locals.var_p0star_dn11 + locals.var_pw_dn11)),)
    } else {
        (locals.var_pav, locals.var_pav_dn0, locals.var_pav_dn1, locals.var_pav_dn3, locals.var_pav_dn4, locals.var_pav_dn5, locals.var_pav_dn6, locals.var_pav_dn7, locals.var_pav_dn8, locals.var_pav_dn9, locals.var_pav_dn10, locals.var_pav_dn11,)
    }
};
        locals.var_pav = assign3270_e3134;
        locals.var_pav_dn0 = assign3270_e3134_d_n0;
        locals.var_pav_dn1 = assign3270_e3134_d_n1;
        locals.var_pav_dn3 = assign3270_e3134_d_n3;
        locals.var_pav_dn4 = assign3270_e3134_d_n4;
        locals.var_pav_dn5 = assign3270_e3134_d_n5;
        locals.var_pav_dn6 = assign3270_e3134_d_n6;
        locals.var_pav_dn7 = assign3270_e3134_d_n7;
        locals.var_pav_dn8 = assign3270_e3134_d_n8;
        locals.var_pav_dn9 = assign3270_e3134_d_n9;
        locals.var_pav_dn10 = assign3270_e3134_d_n10;
        locals.var_pav_dn11 = assign3270_e3134_d_n11;
        locals.var_pav_rv = 0.0;

        let (assign3280_e3145, assign3280_e3145_d_n0, assign3280_e3145_d_n1, assign3280_e3145_d_n3, assign3280_e3145_d_n4, assign3280_e3145_d_n5, assign3280_e3145_d_n6, assign3280_e3145_d_n7, assign3280_e3145_d_n8, assign3280_e3145_d_n9, assign3280_e3145_d_n10, assign3280_e3145_d_n11,) = {
    if ((locals.var_guard45 == 0.0) && (locals.var_guard52 != 0.0)) {
        let assign3280_e3142: f64 = (locals.var_pav + 1.0);
        let assign3280_e3143: f64 = (locals.var_pav / assign3280_e3142);
        (assign3280_e3143, (((locals.var_pav_dn0 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn0)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn1 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn1)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn3 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn3)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn4 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn4)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn5 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn5)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn6 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn6)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn7 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn7)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn8 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn8)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn9 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn9)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn10 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn10)) / (assign3280_e3142 * assign3280_e3142)), (((locals.var_pav_dn11 * assign3280_e3142) - (locals.var_pav * locals.var_pav_dn11)) / (assign3280_e3142 * assign3280_e3142)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10, locals.var_xi_w_dn11,)
    }
};
        locals.var_xi_w = assign3280_e3145;
        locals.var_xi_w_dn0 = assign3280_e3145_d_n0;
        locals.var_xi_w_dn1 = assign3280_e3145_d_n1;
        locals.var_xi_w_dn3 = assign3280_e3145_d_n3;
        locals.var_xi_w_dn4 = assign3280_e3145_d_n4;
        locals.var_xi_w_dn5 = assign3280_e3145_d_n5;
        locals.var_xi_w_dn6 = assign3280_e3145_d_n6;
        locals.var_xi_w_dn7 = assign3280_e3145_d_n7;
        locals.var_xi_w_dn8 = assign3280_e3145_d_n8;
        locals.var_xi_w_dn9 = assign3280_e3145_d_n9;
        locals.var_xi_w_dn10 = assign3280_e3145_d_n10;
        locals.var_xi_w_dn11 = assign3280_e3145_d_n11;
        locals.var_xi_w_rv = 0.0;

        let (assign3290_e3159, assign3290_e3159_d_n0, assign3290_e3159_d_n1, assign3290_e3159_d_n3, assign3290_e3159_d_n4, assign3290_e3159_d_n5, assign3290_e3159_d_n6, assign3290_e3159_d_n7, assign3290_e3159_d_n8, assign3290_e3159_d_n9, assign3290_e3159_d_n10, assign3290_e3159_d_n11,) = {
    if ((locals.var_guard45 == 0.0) && (locals.var_guard52 == 0.0)) {
        let assign3290_e3154: f64 = (locals.var_ec + locals.var_vb2c2);
        let assign3290_e3156: f64 = (assign3290_e3154 - locals.var_vb2c1);
        let assign3290_e3157: f64 = (locals.var_ec / assign3290_e3156);
        (assign3290_e3157, (((locals.var_ec_dn0 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn0)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn1 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn1)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn3 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn3)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn4 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn4)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn5 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn5)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn6 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn6)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn7 * assign3290_e3156) - (locals.var_ec * ((locals.var_ec_dn7 + locals.var_vb2c2_dn7) - locals.var_vb2c1_dn7))) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn8 * assign3290_e3156) - (locals.var_ec * (locals.var_ec_dn8 - locals.var_vb2c1_dn8))) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn9 * assign3290_e3156) - (locals.var_ec * (locals.var_ec_dn9 + locals.var_vb2c2_dn9))) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn10 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn10)) / (assign3290_e3156 * assign3290_e3156)), (((locals.var_ec_dn11 * assign3290_e3156) - (locals.var_ec * locals.var_ec_dn11)) / (assign3290_e3156 * assign3290_e3156)),)
    } else {
        (locals.var_xi_w, locals.var_xi_w_dn0, locals.var_xi_w_dn1, locals.var_xi_w_dn3, locals.var_xi_w_dn4, locals.var_xi_w_dn5, locals.var_xi_w_dn6, locals.var_xi_w_dn7, locals.var_xi_w_dn8, locals.var_xi_w_dn9, locals.var_xi_w_dn10, locals.var_xi_w_dn11,)
    }
};
        locals.var_xi_w = assign3290_e3159;
        locals.var_xi_w_dn0 = assign3290_e3159_d_n0;
        locals.var_xi_w_dn1 = assign3290_e3159_d_n1;
        locals.var_xi_w_dn3 = assign3290_e3159_d_n3;
        locals.var_xi_w_dn4 = assign3290_e3159_d_n4;
        locals.var_xi_w_dn5 = assign3290_e3159_d_n5;
        locals.var_xi_w_dn6 = assign3290_e3159_d_n6;
        locals.var_xi_w_dn7 = assign3290_e3159_d_n7;
        locals.var_xi_w_dn8 = assign3290_e3159_d_n8;
        locals.var_xi_w_dn9 = assign3290_e3159_d_n9;
        locals.var_xi_w_dn10 = assign3290_e3159_d_n10;
        locals.var_xi_w_dn11 = assign3290_e3159_d_n11;
        locals.var_xi_w_rv = 0.0;

        let (assign3300_e3164, assign3300_e3164_d_n0, assign3300_e3164_d_n1, assign3300_e3164_d_n3, assign3300_e3164_d_n4, assign3300_e3164_d_n5, assign3300_e3164_d_n6, assign3300_e3164_d_n7, assign3300_e3164_d_n8, assign3300_e3164_d_n9, assign3300_e3164_d_n10, assign3300_e3164_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        (locals.var_vc1c2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vc1c2_dn8, locals.var_vc1c2_dn9, 0.0, 0.0,)
    } else {
        (locals.var_vxi0, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, locals.var_vxi0_dn7, locals.var_vxi0_dn8, locals.var_vxi0_dn9, locals.var_vxi0_dn10, locals.var_vxi0_dn11,)
    }
};
        locals.var_vxi0 = assign3300_e3164;
        locals.var_vxi0_dn0 = assign3300_e3164_d_n0;
        locals.var_vxi0_dn1 = assign3300_e3164_d_n1;
        locals.var_vxi0_dn3 = assign3300_e3164_d_n3;
        locals.var_vxi0_dn4 = assign3300_e3164_d_n4;
        locals.var_vxi0_dn5 = assign3300_e3164_d_n5;
        locals.var_vxi0_dn6 = assign3300_e3164_d_n6;
        locals.var_vxi0_dn7 = assign3300_e3164_d_n7;
        locals.var_vxi0_dn8 = assign3300_e3164_d_n8;
        locals.var_vxi0_dn9 = assign3300_e3164_d_n9;
        locals.var_vxi0_dn10 = assign3300_e3164_d_n10;
        locals.var_vxi0_dn11 = assign3300_e3164_d_n11;
        locals.var_vxi0_rv = 0.0;

        let (assign3310_e3171, assign3310_e3171_d_n0, assign3310_e3171_d_n1, assign3310_e3171_d_n3, assign3310_e3171_d_n4, assign3310_e3171_d_n5, assign3310_e3171_d_n6, assign3310_e3171_d_n7, assign3310_e3171_d_n8, assign3310_e3171_d_n9, assign3310_e3171_d_n10, assign3310_e3171_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        let assign3310_e3169: f64 = (0.1 * locals.var_vdc_ctc_t);
        (assign3310_e3169, (0.1 * locals.var_vdc_ctc_t_dn0), (0.1 * locals.var_vdc_ctc_t_dn1), (0.1 * locals.var_vdc_ctc_t_dn3), (0.1 * locals.var_vdc_ctc_t_dn4), (0.1 * locals.var_vdc_ctc_t_dn5), (0.1 * locals.var_vdc_ctc_t_dn6), (0.1 * locals.var_vdc_ctc_t_dn7), (0.1 * locals.var_vdc_ctc_t_dn8), (0.1 * locals.var_vdc_ctc_t_dn9), (0.1 * locals.var_vdc_ctc_t_dn10), (0.1 * locals.var_vdc_ctc_t_dn11),)
    } else {
        (locals.var_vch, locals.var_vch_dn0, locals.var_vch_dn1, locals.var_vch_dn3, locals.var_vch_dn4, locals.var_vch_dn5, locals.var_vch_dn6, locals.var_vch_dn7, locals.var_vch_dn8, locals.var_vch_dn9, locals.var_vch_dn10, locals.var_vch_dn11,)
    }
};
        locals.var_vch = assign3310_e3171;
        locals.var_vch_dn0 = assign3310_e3171_d_n0;
        locals.var_vch_dn1 = assign3310_e3171_d_n1;
        locals.var_vch_dn3 = assign3310_e3171_d_n3;
        locals.var_vch_dn4 = assign3310_e3171_d_n4;
        locals.var_vch_dn5 = assign3310_e3171_d_n5;
        locals.var_vch_dn6 = assign3310_e3171_d_n6;
        locals.var_vch_dn7 = assign3310_e3171_d_n7;
        locals.var_vch_dn8 = assign3310_e3171_d_n8;
        locals.var_vch_dn9 = assign3310_e3171_d_n9;
        locals.var_vch_dn10 = assign3310_e3171_d_n10;
        locals.var_vch_dn11 = assign3310_e3171_d_n11;
        locals.var_vch_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3320_e3176, assign3320_e3176_d_n0, assign3320_e3176_d_n1, assign3320_e3176_d_n3, assign3320_e3176_d_n4, assign3320_e3176_d_n5, assign3320_e3176_d_n6, assign3320_e3176_d_n7, assign3320_e3176_d_n8, assign3320_e3176_d_n9, assign3320_e3176_d_n10, assign3320_e3176_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        (locals.var_ic1c2, locals.var_ic1c2_dn0, locals.var_ic1c2_dn1, locals.var_ic1c2_dn3, locals.var_ic1c2_dn4, locals.var_ic1c2_dn5, locals.var_ic1c2_dn6, locals.var_ic1c2_dn7, locals.var_ic1c2_dn8, locals.var_ic1c2_dn9, locals.var_ic1c2_dn10, locals.var_ic1c2_dn11,)
    } else {
        (locals.var_icap, locals.var_icap_dn0, locals.var_icap_dn1, locals.var_icap_dn3, locals.var_icap_dn4, locals.var_icap_dn5, locals.var_icap_dn6, locals.var_icap_dn7, locals.var_icap_dn8, locals.var_icap_dn9, locals.var_icap_dn10, locals.var_icap_dn11,)
    }
};
        locals.var_icap = assign3320_e3176;
        locals.var_icap_dn0 = assign3320_e3176_d_n0;
        locals.var_icap_dn1 = assign3320_e3176_d_n1;
        locals.var_icap_dn3 = assign3320_e3176_d_n3;
        locals.var_icap_dn4 = assign3320_e3176_d_n4;
        locals.var_icap_dn5 = assign3320_e3176_d_n5;
        locals.var_icap_dn6 = assign3320_e3176_d_n6;
        locals.var_icap_dn7 = assign3320_e3176_d_n7;
        locals.var_icap_dn8 = assign3320_e3176_d_n8;
        locals.var_icap_dn9 = assign3320_e3176_d_n9;
        locals.var_icap_dn10 = assign3320_e3176_d_n10;
        locals.var_icap_dn11 = assign3320_e3176_d_n11;
        locals.var_icap_rv = 0.0;

        let (assign3330_e3185, assign3330_e3185_d_n0, assign3330_e3185_d_n1, assign3330_e3185_d_n3, assign3330_e3185_d_n4, assign3330_e3185_d_n5, assign3330_e3185_d_n6, assign3330_e3185_d_n7, assign3330_e3185_d_n8, assign3330_e3185_d_n9, assign3330_e3185_d_n10, assign3330_e3185_d_n11,) = {
    if (locals.var_guard45 == 0.0) {
        let assign3330_e3182: f64 = (locals.var_icap / p.p62);
        let assign3330_e3183: f64 = (1.0 - assign3330_e3182);
        (assign3330_e3183, (-(locals.var_icap_dn0 / p.p62)), (-(locals.var_icap_dn1 / p.p62)), (-(locals.var_icap_dn3 / p.p62)), (-(locals.var_icap_dn4 / p.p62)), (-(locals.var_icap_dn5 / p.p62)), (-(locals.var_icap_dn6 / p.p62)), (-(locals.var_icap_dn7 / p.p62)), (-(locals.var_icap_dn8 / p.p62)), (-(locals.var_icap_dn9 / p.p62)), (-(locals.var_icap_dn10 / p.p62)), (-(locals.var_icap_dn11 / p.p62)),)
    } else {
        (locals.var_icap_ihc, locals.var_icap_ihc_dn0, locals.var_icap_ihc_dn1, locals.var_icap_ihc_dn3, locals.var_icap_ihc_dn4, locals.var_icap_ihc_dn5, locals.var_icap_ihc_dn6, locals.var_icap_ihc_dn7, locals.var_icap_ihc_dn8, locals.var_icap_ihc_dn9, locals.var_icap_ihc_dn10, locals.var_icap_ihc_dn11,)
    }
};
        locals.var_icap_ihc = assign3330_e3185;
        locals.var_icap_ihc_dn0 = assign3330_e3185_d_n0;
        locals.var_icap_ihc_dn1 = assign3330_e3185_d_n1;
        locals.var_icap_ihc_dn3 = assign3330_e3185_d_n3;
        locals.var_icap_ihc_dn4 = assign3330_e3185_d_n4;
        locals.var_icap_ihc_dn5 = assign3330_e3185_d_n5;
        locals.var_icap_ihc_dn6 = assign3330_e3185_d_n6;
        locals.var_icap_ihc_dn7 = assign3330_e3185_d_n7;
        locals.var_icap_ihc_dn8 = assign3330_e3185_d_n8;
        locals.var_icap_ihc_dn9 = assign3330_e3185_d_n9;
        locals.var_icap_ihc_dn10 = assign3330_e3185_d_n10;
        locals.var_icap_ihc_dn11 = assign3330_e3185_d_n11;
        locals.var_icap_ihc_rv = 0.0;

        let assign3340_e3190: f64 = (-1.0);
        let assign3340_e3192: f64 = (assign3340_e3190 / p.p67);
        let assign3340_e3193: f64 = (3.0_f64).powf(assign3340_e3192);
        let assign3340_e3194: f64 = (1.0 - assign3340_e3193);
        let assign3340_e3195: f64 = (locals.var_vde_t * assign3340_e3194);
        locals.var_vfe = assign3340_e3195;
        locals.var_vfe_dn0 = (locals.var_vde_t_dn0 * assign3340_e3194);
        locals.var_vfe_dn1 = (locals.var_vde_t_dn1 * assign3340_e3194);
        locals.var_vfe_dn3 = (locals.var_vde_t_dn3 * assign3340_e3194);
        locals.var_vfe_dn4 = (locals.var_vde_t_dn4 * assign3340_e3194);
        locals.var_vfe_dn5 = (locals.var_vde_t_dn5 * assign3340_e3194);
        locals.var_vfe_dn6 = (locals.var_vde_t_dn6 * assign3340_e3194);
        locals.var_vfe_dn7 = (locals.var_vde_t_dn7 * assign3340_e3194);
        locals.var_vfe_dn8 = (locals.var_vde_t_dn8 * assign3340_e3194);
        locals.var_vfe_dn9 = (locals.var_vde_t_dn9 * assign3340_e3194);
        locals.var_vfe_dn10 = (locals.var_vde_t_dn10 * assign3340_e3194);
        locals.var_vfe_dn11 = (locals.var_vde_t_dn11 * assign3340_e3194);
        locals.var_vfe_rv = 0.0;

        let assign3350_e3198: f64 = (0.1 * locals.var_vde_t);
        locals.var_a_vde = assign3350_e3198;
        locals.var_a_vde_dn0 = (0.1 * locals.var_vde_t_dn0);
        locals.var_a_vde_dn1 = (0.1 * locals.var_vde_t_dn1);
        locals.var_a_vde_dn3 = (0.1 * locals.var_vde_t_dn3);
        locals.var_a_vde_dn4 = (0.1 * locals.var_vde_t_dn4);
        locals.var_a_vde_dn5 = (0.1 * locals.var_vde_t_dn5);
        locals.var_a_vde_dn6 = (0.1 * locals.var_vde_t_dn6);
        locals.var_a_vde_dn7 = (0.1 * locals.var_vde_t_dn7);
        locals.var_a_vde_dn8 = (0.1 * locals.var_vde_t_dn8);
        locals.var_a_vde_dn9 = (0.1 * locals.var_vde_t_dn9);
        locals.var_a_vde_dn10 = (0.1 * locals.var_vde_t_dn10);
        locals.var_a_vde_dn11 = (0.1 * locals.var_vde_t_dn11);
        locals.var_a_vde_rv = 0.0;

        let assign3360_e3201: f64 = (locals.var_vb2e1 - locals.var_vfe);
        let assign3360_e3203: f64 = (assign3360_e3201 / locals.var_a_vde);
        locals.var_dxa = assign3360_e3203;
        locals.var_dxa_dn0 = ((((-locals.var_vfe_dn0) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn0)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn1 = ((((-locals.var_vfe_dn1) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn1)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn3 = ((((-locals.var_vfe_dn3) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn3)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn4 = ((((-locals.var_vfe_dn4) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn4)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn5 = ((((locals.var_vb2e1_dn5 - locals.var_vfe_dn5) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn5)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn6 = ((((-locals.var_vfe_dn6) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn6)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn7 = ((((locals.var_vb2e1_dn7 - locals.var_vfe_dn7) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn7)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn8 = ((((-locals.var_vfe_dn8) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn8)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn9 = ((((-locals.var_vfe_dn9) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn9)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn10 = ((((-locals.var_vfe_dn10) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn10)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_dn11 = ((((-locals.var_vfe_dn11) * locals.var_a_vde) - (assign3360_e3201 * locals.var_a_vde_dn11)) / (locals.var_a_vde * locals.var_a_vde));
        locals.var_dxa_rv = 0.0;

        let assign3370_e3206: f64 = if locals.var_vb2e1 < locals.var_vfe { 1.0 } else { 0.0 };
        locals.var_guard53 = assign3370_e3206;
        locals.var_guard53_rv = 0.0;

        let (assign3380_e3218, assign3380_e3218_d_n0, assign3380_e3218_d_n1, assign3380_e3218_d_n3, assign3380_e3218_d_n4, assign3380_e3218_d_n5, assign3380_e3218_d_n6, assign3380_e3218_d_n7, assign3380_e3218_d_n8, assign3380_e3218_d_n9, assign3380_e3218_d_n10, assign3380_e3218_d_n11,) = {
    if (locals.var_guard53 != 0.0) {
        let assign3380_e3212: f64 = (locals.var_dxa).exp();
        let assign3380_e3213: f64 = (1.0 + assign3380_e3212);
        let assign3380_e3214: f64 = (assign3380_e3213).ln();
        let assign3380_e3215: f64 = (locals.var_a_vde * assign3380_e3214);
        let assign3380_e3216: f64 = (locals.var_vb2e1 - assign3380_e3215);
        (assign3380_e3216, (-((locals.var_a_vde_dn0 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn0) / assign3380_e3213)))), (-((locals.var_a_vde_dn1 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn1) / assign3380_e3213)))), (-((locals.var_a_vde_dn3 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn3) / assign3380_e3213)))), (-((locals.var_a_vde_dn4 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn4) / assign3380_e3213)))), (locals.var_vb2e1_dn5 - ((locals.var_a_vde_dn5 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn5) / assign3380_e3213)))), (-((locals.var_a_vde_dn6 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn6) / assign3380_e3213)))), (locals.var_vb2e1_dn7 - ((locals.var_a_vde_dn7 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn7) / assign3380_e3213)))), (-((locals.var_a_vde_dn8 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn8) / assign3380_e3213)))), (-((locals.var_a_vde_dn9 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn9) / assign3380_e3213)))), (-((locals.var_a_vde_dn10 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn10) / assign3380_e3213)))), (-((locals.var_a_vde_dn11 * assign3380_e3214) + (locals.var_a_vde * ((assign3380_e3212 * locals.var_dxa_dn11) / assign3380_e3213)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9, locals.var_vje_dn10, locals.var_vje_dn11,)
    }
};
        locals.var_vje = assign3380_e3218;
        locals.var_vje_dn0 = assign3380_e3218_d_n0;
        locals.var_vje_dn1 = assign3380_e3218_d_n1;
        locals.var_vje_dn3 = assign3380_e3218_d_n3;
        locals.var_vje_dn4 = assign3380_e3218_d_n4;
        locals.var_vje_dn5 = assign3380_e3218_d_n5;
        locals.var_vje_dn6 = assign3380_e3218_d_n6;
        locals.var_vje_dn7 = assign3380_e3218_d_n7;
        locals.var_vje_dn8 = assign3380_e3218_d_n8;
        locals.var_vje_dn9 = assign3380_e3218_d_n9;
        locals.var_vje_dn10 = assign3380_e3218_d_n10;
        locals.var_vje_dn11 = assign3380_e3218_d_n11;
        locals.var_vje_rv = 0.0;

        let (assign3390_e3232, assign3390_e3232_d_n0, assign3390_e3232_d_n1, assign3390_e3232_d_n3, assign3390_e3232_d_n4, assign3390_e3232_d_n5, assign3390_e3232_d_n6, assign3390_e3232_d_n7, assign3390_e3232_d_n8, assign3390_e3232_d_n9, assign3390_e3232_d_n10, assign3390_e3232_d_n11,) = {
    if (locals.var_guard53 == 0.0) {
        let assign3390_e3225: f64 = (-locals.var_dxa);
        let assign3390_e3226: f64 = (assign3390_e3225).exp();
        let assign3390_e3227: f64 = (1.0 + assign3390_e3226);
        let assign3390_e3228: f64 = (assign3390_e3227).ln();
        let assign3390_e3229: f64 = (locals.var_a_vde * assign3390_e3228);
        let assign3390_e3230: f64 = (locals.var_vfe - assign3390_e3229);
        (assign3390_e3230, (locals.var_vfe_dn0 - ((locals.var_a_vde_dn0 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn0)) / assign3390_e3227)))), (locals.var_vfe_dn1 - ((locals.var_a_vde_dn1 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn1)) / assign3390_e3227)))), (locals.var_vfe_dn3 - ((locals.var_a_vde_dn3 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn3)) / assign3390_e3227)))), (locals.var_vfe_dn4 - ((locals.var_a_vde_dn4 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn4)) / assign3390_e3227)))), (locals.var_vfe_dn5 - ((locals.var_a_vde_dn5 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn5)) / assign3390_e3227)))), (locals.var_vfe_dn6 - ((locals.var_a_vde_dn6 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn6)) / assign3390_e3227)))), (locals.var_vfe_dn7 - ((locals.var_a_vde_dn7 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn7)) / assign3390_e3227)))), (locals.var_vfe_dn8 - ((locals.var_a_vde_dn8 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn8)) / assign3390_e3227)))), (locals.var_vfe_dn9 - ((locals.var_a_vde_dn9 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn9)) / assign3390_e3227)))), (locals.var_vfe_dn10 - ((locals.var_a_vde_dn10 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn10)) / assign3390_e3227)))), (locals.var_vfe_dn11 - ((locals.var_a_vde_dn11 * assign3390_e3228) + (locals.var_a_vde * ((assign3390_e3226 * (-locals.var_dxa_dn11)) / assign3390_e3227)))),)
    } else {
        (locals.var_vje, locals.var_vje_dn0, locals.var_vje_dn1, locals.var_vje_dn3, locals.var_vje_dn4, locals.var_vje_dn5, locals.var_vje_dn6, locals.var_vje_dn7, locals.var_vje_dn8, locals.var_vje_dn9, locals.var_vje_dn10, locals.var_vje_dn11,)
    }
};
        locals.var_vje = assign3390_e3232;
        locals.var_vje_dn0 = assign3390_e3232_d_n0;
        locals.var_vje_dn1 = assign3390_e3232_d_n1;
        locals.var_vje_dn3 = assign3390_e3232_d_n3;
        locals.var_vje_dn4 = assign3390_e3232_d_n4;
        locals.var_vje_dn5 = assign3390_e3232_d_n5;
        locals.var_vje_dn6 = assign3390_e3232_d_n6;
        locals.var_vje_dn7 = assign3390_e3232_d_n7;
        locals.var_vje_dn8 = assign3390_e3232_d_n8;
        locals.var_vje_dn9 = assign3390_e3232_d_n9;
        locals.var_vje_dn10 = assign3390_e3232_d_n10;
        locals.var_vje_dn11 = assign3390_e3232_d_n11;
        locals.var_vje_rv = 0.0;

        let assign3400_e3236: f64 = (locals.var_vje * locals.var_inv_vde_t);
        let assign3400_e3237: f64 = (1.0 - assign3400_e3236);
        let assign3400_e3240: f64 = (1.0 - p.p67);
        let assign3400_e3241: f64 = (assign3400_e3237).powf(assign3400_e3240);
        locals.var_e0eb = assign3400_e3241;
        locals.var_e0eb_dn0 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn0 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn0))) / assign3400_e3237))) };
        locals.var_e0eb_dn1 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn1 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn1))) / assign3400_e3237))) };
        locals.var_e0eb_dn3 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn3 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn3))) / assign3400_e3237))) };
        locals.var_e0eb_dn4 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn4 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn4))) / assign3400_e3237))) };
        locals.var_e0eb_dn5 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn5 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn5))) / assign3400_e3237))) };
        locals.var_e0eb_dn6 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn6 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn6))) / assign3400_e3237))) };
        locals.var_e0eb_dn7 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn7 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn7))) / assign3400_e3237))) };
        locals.var_e0eb_dn8 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn8 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn8))) / assign3400_e3237))) };
        locals.var_e0eb_dn9 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn9 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn9))) / assign3400_e3237))) };
        locals.var_e0eb_dn10 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn10 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn10))) / assign3400_e3237))) };
        locals.var_e0eb_dn11 = if 0.0 == 0.0 && ((assign3400_e3240) as f64).is_finite() && ((assign3400_e3240) as f64).fract() == 0.0 { if assign3400_e3240 == 0.0 { 0.0 } else { (assign3400_e3240 * ((assign3400_e3237).powf(assign3400_e3240 - 1.0) * (-((locals.var_vje_dn11 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn11))))) } } else { (assign3400_e3241 * (assign3400_e3240 * ((-((locals.var_vje_dn11 * locals.var_inv_vde_t) + (locals.var_vje * locals.var_inv_vde_t_dn11))) / assign3400_e3237))) };
        locals.var_e0eb_rv = 0.0;

        let assign3410_e3245: f64 = (1.0 - p.p67);
        let assign3410_e3246: f64 = (locals.var_vde_t / assign3410_e3245);
        let assign3410_e3249: f64 = (1.0 - locals.var_e0eb);
        let assign3410_e3250: f64 = (assign3410_e3246 * assign3410_e3249);
        let assign3410_e3254: f64 = (locals.var_vb2e1 - locals.var_vje);
        let assign3410_e3255: f64 = (3.0 * assign3410_e3254);
        let assign3410_e3256: f64 = (assign3410_e3250 + assign3410_e3255);
        locals.var_vte = assign3410_e3256;
        locals.var_vte_dn0 = ((((locals.var_vde_t_dn0 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn0))) + (3.0 * (-locals.var_vje_dn0)));
        locals.var_vte_dn1 = ((((locals.var_vde_t_dn1 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn1))) + (3.0 * (-locals.var_vje_dn1)));
        locals.var_vte_dn3 = ((((locals.var_vde_t_dn3 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn3))) + (3.0 * (-locals.var_vje_dn3)));
        locals.var_vte_dn4 = ((((locals.var_vde_t_dn4 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn4))) + (3.0 * (-locals.var_vje_dn4)));
        locals.var_vte_dn5 = ((((locals.var_vde_t_dn5 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn5))) + (3.0 * (locals.var_vb2e1_dn5 - locals.var_vje_dn5)));
        locals.var_vte_dn6 = ((((locals.var_vde_t_dn6 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn6))) + (3.0 * (-locals.var_vje_dn6)));
        locals.var_vte_dn7 = ((((locals.var_vde_t_dn7 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn7))) + (3.0 * (locals.var_vb2e1_dn7 - locals.var_vje_dn7)));
        locals.var_vte_dn8 = ((((locals.var_vde_t_dn8 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn8))) + (3.0 * (-locals.var_vje_dn8)));
        locals.var_vte_dn9 = ((((locals.var_vde_t_dn9 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn9))) + (3.0 * (-locals.var_vje_dn9)));
        locals.var_vte_dn10 = ((((locals.var_vde_t_dn10 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn10))) + (3.0 * (-locals.var_vje_dn10)));
        locals.var_vte_dn11 = ((((locals.var_vde_t_dn11 / assign3410_e3245) * assign3410_e3249) + (assign3410_e3246 * (-locals.var_e0eb_dn11))) + (3.0 * (-locals.var_vje_dn11)));
        locals.var_vte_rv = 0.0;

        let assign3420_e3259: f64 = if p.p74 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign3420_e3259;
        locals.var_guard54_rv = 0.0;

        let (assign3430_e3263, assign3430_e3263_d_n0, assign3430_e3263_d_n1, assign3430_e3263_d_n3, assign3430_e3263_d_n4, assign3430_e3263_d_n5, assign3430_e3263_d_n6, assign3430_e3263_d_n7, assign3430_e3263_d_n8, assign3430_e3263_d_n9, assign3430_e3263_d_n10, assign3430_e3263_d_n11,) = {
    if (locals.var_guard54 != 0.0) {
        (locals.var_vb2c1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c1_dn7, locals.var_vb2c1_dn8, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10, locals.var_vjunc_dn11,)
    }
};
        locals.var_vjunc = assign3430_e3263;
        locals.var_vjunc_dn0 = assign3430_e3263_d_n0;
        locals.var_vjunc_dn1 = assign3430_e3263_d_n1;
        locals.var_vjunc_dn3 = assign3430_e3263_d_n3;
        locals.var_vjunc_dn4 = assign3430_e3263_d_n4;
        locals.var_vjunc_dn5 = assign3430_e3263_d_n5;
        locals.var_vjunc_dn6 = assign3430_e3263_d_n6;
        locals.var_vjunc_dn7 = assign3430_e3263_d_n7;
        locals.var_vjunc_dn8 = assign3430_e3263_d_n8;
        locals.var_vjunc_dn9 = assign3430_e3263_d_n9;
        locals.var_vjunc_dn10 = assign3430_e3263_d_n10;
        locals.var_vjunc_dn11 = assign3430_e3263_d_n11;
        locals.var_vjunc_rv = 0.0;

        let assign3440_e3266: f64 = if p.p74 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign3440_e3266;
        locals.var_guard55_rv = 0.0;

        let (assign3450_e3275, assign3450_e3275_d_n0, assign3450_e3275_d_n1, assign3450_e3275_d_n3, assign3450_e3275_d_n4, assign3450_e3275_d_n5, assign3450_e3275_d_n6, assign3450_e3275_d_n7, assign3450_e3275_d_n8, assign3450_e3275_d_n9, assign3450_e3275_d_n10, assign3450_e3275_d_n11,) = {
    if ((locals.var_guard54 == 0.0) && (locals.var_guard55 != 0.0)) {
        let assign3450_e3273: f64 = (locals.var_vb2c1 + locals.var_vxi0);
        (assign3450_e3273, locals.var_vxi0_dn0, locals.var_vxi0_dn1, locals.var_vxi0_dn3, locals.var_vxi0_dn4, locals.var_vxi0_dn5, locals.var_vxi0_dn6, (locals.var_vb2c1_dn7 + locals.var_vxi0_dn7), (locals.var_vb2c1_dn8 + locals.var_vxi0_dn8), locals.var_vxi0_dn9, locals.var_vxi0_dn10, locals.var_vxi0_dn11,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10, locals.var_vjunc_dn11,)
    }
};
        locals.var_vjunc = assign3450_e3275;
        locals.var_vjunc_dn0 = assign3450_e3275_d_n0;
        locals.var_vjunc_dn1 = assign3450_e3275_d_n1;
        locals.var_vjunc_dn3 = assign3450_e3275_d_n3;
        locals.var_vjunc_dn4 = assign3450_e3275_d_n4;
        locals.var_vjunc_dn5 = assign3450_e3275_d_n5;
        locals.var_vjunc_dn6 = assign3450_e3275_d_n6;
        locals.var_vjunc_dn7 = assign3450_e3275_d_n7;
        locals.var_vjunc_dn8 = assign3450_e3275_d_n8;
        locals.var_vjunc_dn9 = assign3450_e3275_d_n9;
        locals.var_vjunc_dn10 = assign3450_e3275_d_n10;
        locals.var_vjunc_dn11 = assign3450_e3275_d_n11;
        locals.var_vjunc_rv = 0.0;

        let (assign3460_e3283, assign3460_e3283_d_n0, assign3460_e3283_d_n1, assign3460_e3283_d_n3, assign3460_e3283_d_n4, assign3460_e3283_d_n5, assign3460_e3283_d_n6, assign3460_e3283_d_n7, assign3460_e3283_d_n8, assign3460_e3283_d_n9, assign3460_e3283_d_n10, assign3460_e3283_d_n11,) = {
    if ((locals.var_guard54 == 0.0) && (locals.var_guard55 == 0.0)) {
        (locals.var_vb2c2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vb2c2_dn7, 0.0, locals.var_vb2c2_dn9, 0.0, 0.0,)
    } else {
        (locals.var_vjunc, locals.var_vjunc_dn0, locals.var_vjunc_dn1, locals.var_vjunc_dn3, locals.var_vjunc_dn4, locals.var_vjunc_dn5, locals.var_vjunc_dn6, locals.var_vjunc_dn7, locals.var_vjunc_dn8, locals.var_vjunc_dn9, locals.var_vjunc_dn10, locals.var_vjunc_dn11,)
    }
};
        locals.var_vjunc = assign3460_e3283;
        locals.var_vjunc_dn0 = assign3460_e3283_d_n0;
        locals.var_vjunc_dn1 = assign3460_e3283_d_n1;
        locals.var_vjunc_dn3 = assign3460_e3283_d_n3;
        locals.var_vjunc_dn4 = assign3460_e3283_d_n4;
        locals.var_vjunc_dn5 = assign3460_e3283_d_n5;
        locals.var_vjunc_dn6 = assign3460_e3283_d_n6;
        locals.var_vjunc_dn7 = assign3460_e3283_d_n7;
        locals.var_vjunc_dn8 = assign3460_e3283_d_n8;
        locals.var_vjunc_dn9 = assign3460_e3283_d_n9;
        locals.var_vjunc_dn10 = assign3460_e3283_d_n10;
        locals.var_vjunc_dn11 = assign3460_e3283_d_n11;
        locals.var_vjunc_rv = 0.0;

        let assign3470_e3286: f64 = (2.0 - locals.var_xp_t);
        let assign3470_e3289: f64 = (1.0 - locals.var_xp_t);
        let assign3470_e3290: f64 = (assign3470_e3286 / assign3470_e3289);
        locals.var_bjc = assign3470_e3290;
        locals.var_bjc_dn0 = ((((-locals.var_xp_t_dn0) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn0))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn1 = ((((-locals.var_xp_t_dn1) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn1))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn3 = ((((-locals.var_xp_t_dn3) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn3))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn4 = ((((-locals.var_xp_t_dn4) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn4))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn5 = ((((-locals.var_xp_t_dn5) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn5))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn6 = ((((-locals.var_xp_t_dn6) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn6))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn7 = ((((-locals.var_xp_t_dn7) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn7))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn8 = ((((-locals.var_xp_t_dn8) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn8))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn9 = ((((-locals.var_xp_t_dn9) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn9))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn10 = ((((-locals.var_xp_t_dn10) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn10))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_dn11 = ((((-locals.var_xp_t_dn11) * assign3470_e3289) - (assign3470_e3286 * (-locals.var_xp_t_dn11))) / (assign3470_e3289 * assign3470_e3289));
        locals.var_bjc_rv = 0.0;

        let assign3480_e3295: f64 = (-1.0);
        let assign3480_e3297: f64 = (assign3480_e3295 / p.p72);
        let assign3480_e3298: f64 = (locals.var_bjc).powf(assign3480_e3297);
        let assign3480_e3299: f64 = (1.0 - assign3480_e3298);
        let assign3480_e3300: f64 = (locals.var_vdc_ctc_t * assign3480_e3299);
        locals.var_vfc = assign3480_e3300;
        locals.var_vfc_dn0 = ((locals.var_vdc_ctc_t_dn0 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn0)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn0 / locals.var_bjc))) })));
        locals.var_vfc_dn1 = ((locals.var_vdc_ctc_t_dn1 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn1)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn1 / locals.var_bjc))) })));
        locals.var_vfc_dn3 = ((locals.var_vdc_ctc_t_dn3 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn3)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn3 / locals.var_bjc))) })));
        locals.var_vfc_dn4 = ((locals.var_vdc_ctc_t_dn4 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn4)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn4 / locals.var_bjc))) })));
        locals.var_vfc_dn5 = ((locals.var_vdc_ctc_t_dn5 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn5)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn5 / locals.var_bjc))) })));
        locals.var_vfc_dn6 = ((locals.var_vdc_ctc_t_dn6 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn6)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn6 / locals.var_bjc))) })));
        locals.var_vfc_dn7 = ((locals.var_vdc_ctc_t_dn7 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn7)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn7 / locals.var_bjc))) })));
        locals.var_vfc_dn8 = ((locals.var_vdc_ctc_t_dn8 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn8)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn8 / locals.var_bjc))) })));
        locals.var_vfc_dn9 = ((locals.var_vdc_ctc_t_dn9 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn9)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn9 / locals.var_bjc))) })));
        locals.var_vfc_dn10 = ((locals.var_vdc_ctc_t_dn10 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn10)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn10 / locals.var_bjc))) })));
        locals.var_vfc_dn11 = ((locals.var_vdc_ctc_t_dn11 * assign3480_e3299) + (locals.var_vdc_ctc_t * (-if 0.0 == 0.0 && ((assign3480_e3297) as f64).is_finite() && ((assign3480_e3297) as f64).fract() == 0.0 { if assign3480_e3297 == 0.0 { 0.0 } else { (assign3480_e3297 * ((locals.var_bjc).powf(assign3480_e3297 - 1.0) * locals.var_bjc_dn11)) } } else { (assign3480_e3298 * (assign3480_e3297 * (locals.var_bjc_dn11 / locals.var_bjc))) })));
        locals.var_vfc_rv = 0.0;

        let assign3490_e3303: f64 = (locals.var_vjunc - locals.var_vfc);
        let assign3490_e3305: f64 = (assign3490_e3303 / locals.var_vch);
        locals.var_dxa = assign3490_e3305;
        locals.var_dxa_dn0 = ((((locals.var_vjunc_dn0 - locals.var_vfc_dn0) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn0)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn1 = ((((locals.var_vjunc_dn1 - locals.var_vfc_dn1) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn1)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn3 = ((((locals.var_vjunc_dn3 - locals.var_vfc_dn3) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn3)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn4 = ((((locals.var_vjunc_dn4 - locals.var_vfc_dn4) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn4)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn5 = ((((locals.var_vjunc_dn5 - locals.var_vfc_dn5) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn5)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn6 = ((((locals.var_vjunc_dn6 - locals.var_vfc_dn6) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn6)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn7 = ((((locals.var_vjunc_dn7 - locals.var_vfc_dn7) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn7)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn8 = ((((locals.var_vjunc_dn8 - locals.var_vfc_dn8) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn8)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn9 = ((((locals.var_vjunc_dn9 - locals.var_vfc_dn9) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn9)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn10 = ((((locals.var_vjunc_dn10 - locals.var_vfc_dn10) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn10)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_dn11 = ((((locals.var_vjunc_dn11 - locals.var_vfc_dn11) * locals.var_vch) - (assign3490_e3303 * locals.var_vch_dn11)) / (locals.var_vch * locals.var_vch));
        locals.var_dxa_rv = 0.0;

        let assign3500_e3308: f64 = if locals.var_vjunc < locals.var_vfc { 1.0 } else { 0.0 };
        locals.var_guard56 = assign3500_e3308;
        locals.var_guard56_rv = 0.0;

        let (assign3510_e3320, assign3510_e3320_d_n0, assign3510_e3320_d_n1, assign3510_e3320_d_n3, assign3510_e3320_d_n4, assign3510_e3320_d_n5, assign3510_e3320_d_n6, assign3510_e3320_d_n7, assign3510_e3320_d_n8, assign3510_e3320_d_n9, assign3510_e3320_d_n10, assign3510_e3320_d_n11,) = {
    if (locals.var_guard56 != 0.0) {
        let assign3510_e3314: f64 = (locals.var_dxa).exp();
        let assign3510_e3315: f64 = (1.0 + assign3510_e3314);
        let assign3510_e3316: f64 = (assign3510_e3315).ln();
        let assign3510_e3317: f64 = (locals.var_vch * assign3510_e3316);
        let assign3510_e3318: f64 = (locals.var_vjunc - assign3510_e3317);
        (assign3510_e3318, (locals.var_vjunc_dn0 - ((locals.var_vch_dn0 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn0) / assign3510_e3315)))), (locals.var_vjunc_dn1 - ((locals.var_vch_dn1 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn1) / assign3510_e3315)))), (locals.var_vjunc_dn3 - ((locals.var_vch_dn3 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn3) / assign3510_e3315)))), (locals.var_vjunc_dn4 - ((locals.var_vch_dn4 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn4) / assign3510_e3315)))), (locals.var_vjunc_dn5 - ((locals.var_vch_dn5 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn5) / assign3510_e3315)))), (locals.var_vjunc_dn6 - ((locals.var_vch_dn6 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn6) / assign3510_e3315)))), (locals.var_vjunc_dn7 - ((locals.var_vch_dn7 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn7) / assign3510_e3315)))), (locals.var_vjunc_dn8 - ((locals.var_vch_dn8 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn8) / assign3510_e3315)))), (locals.var_vjunc_dn9 - ((locals.var_vch_dn9 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn9) / assign3510_e3315)))), (locals.var_vjunc_dn10 - ((locals.var_vch_dn10 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn10) / assign3510_e3315)))), (locals.var_vjunc_dn11 - ((locals.var_vch_dn11 * assign3510_e3316) + (locals.var_vch * ((assign3510_e3314 * locals.var_dxa_dn11) / assign3510_e3315)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9, locals.var_vjc_dn10, locals.var_vjc_dn11,)
    }
};
        locals.var_vjc = assign3510_e3320;
        locals.var_vjc_dn0 = assign3510_e3320_d_n0;
        locals.var_vjc_dn1 = assign3510_e3320_d_n1;
        locals.var_vjc_dn3 = assign3510_e3320_d_n3;
        locals.var_vjc_dn4 = assign3510_e3320_d_n4;
        locals.var_vjc_dn5 = assign3510_e3320_d_n5;
        locals.var_vjc_dn6 = assign3510_e3320_d_n6;
        locals.var_vjc_dn7 = assign3510_e3320_d_n7;
        locals.var_vjc_dn8 = assign3510_e3320_d_n8;
        locals.var_vjc_dn9 = assign3510_e3320_d_n9;
        locals.var_vjc_dn10 = assign3510_e3320_d_n10;
        locals.var_vjc_dn11 = assign3510_e3320_d_n11;
        locals.var_vjc_rv = 0.0;

        let (assign3520_e3334, assign3520_e3334_d_n0, assign3520_e3334_d_n1, assign3520_e3334_d_n3, assign3520_e3334_d_n4, assign3520_e3334_d_n5, assign3520_e3334_d_n6, assign3520_e3334_d_n7, assign3520_e3334_d_n8, assign3520_e3334_d_n9, assign3520_e3334_d_n10, assign3520_e3334_d_n11,) = {
    if (locals.var_guard56 == 0.0) {
        let assign3520_e3327: f64 = (-locals.var_dxa);
        let assign3520_e3328: f64 = (assign3520_e3327).exp();
        let assign3520_e3329: f64 = (1.0 + assign3520_e3328);
        let assign3520_e3330: f64 = (assign3520_e3329).ln();
        let assign3520_e3331: f64 = (locals.var_vch * assign3520_e3330);
        let assign3520_e3332: f64 = (locals.var_vfc - assign3520_e3331);
        (assign3520_e3332, (locals.var_vfc_dn0 - ((locals.var_vch_dn0 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn0)) / assign3520_e3329)))), (locals.var_vfc_dn1 - ((locals.var_vch_dn1 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn1)) / assign3520_e3329)))), (locals.var_vfc_dn3 - ((locals.var_vch_dn3 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn3)) / assign3520_e3329)))), (locals.var_vfc_dn4 - ((locals.var_vch_dn4 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn4)) / assign3520_e3329)))), (locals.var_vfc_dn5 - ((locals.var_vch_dn5 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn5)) / assign3520_e3329)))), (locals.var_vfc_dn6 - ((locals.var_vch_dn6 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn6)) / assign3520_e3329)))), (locals.var_vfc_dn7 - ((locals.var_vch_dn7 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn7)) / assign3520_e3329)))), (locals.var_vfc_dn8 - ((locals.var_vch_dn8 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn8)) / assign3520_e3329)))), (locals.var_vfc_dn9 - ((locals.var_vch_dn9 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn9)) / assign3520_e3329)))), (locals.var_vfc_dn10 - ((locals.var_vch_dn10 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn10)) / assign3520_e3329)))), (locals.var_vfc_dn11 - ((locals.var_vch_dn11 * assign3520_e3330) + (locals.var_vch * ((assign3520_e3328 * (-locals.var_dxa_dn11)) / assign3520_e3329)))),)
    } else {
        (locals.var_vjc, locals.var_vjc_dn0, locals.var_vjc_dn1, locals.var_vjc_dn3, locals.var_vjc_dn4, locals.var_vjc_dn5, locals.var_vjc_dn6, locals.var_vjc_dn7, locals.var_vjc_dn8, locals.var_vjc_dn9, locals.var_vjc_dn10, locals.var_vjc_dn11,)
    }
};
        locals.var_vjc = assign3520_e3334;
        locals.var_vjc_dn0 = assign3520_e3334_d_n0;
        locals.var_vjc_dn1 = assign3520_e3334_d_n1;
        locals.var_vjc_dn3 = assign3520_e3334_d_n3;
        locals.var_vjc_dn4 = assign3520_e3334_d_n4;
        locals.var_vjc_dn5 = assign3520_e3334_d_n5;
        locals.var_vjc_dn6 = assign3520_e3334_d_n6;
        locals.var_vjc_dn7 = assign3520_e3334_d_n7;
        locals.var_vjc_dn8 = assign3520_e3334_d_n8;
        locals.var_vjc_dn9 = assign3520_e3334_d_n9;
        locals.var_vjc_dn10 = assign3520_e3334_d_n10;
        locals.var_vjc_dn11 = assign3520_e3334_d_n11;
        locals.var_vjc_rv = 0.0;

        let assign3530_e3337: f64 = (locals.var_icap_ihc).powf(p.p76);
        locals.var_fi = assign3530_e3337;
        locals.var_fi_dn0 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn0)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn0 / locals.var_icap_ihc))) };
        locals.var_fi_dn1 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn1)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn1 / locals.var_icap_ihc))) };
        locals.var_fi_dn3 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn3)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn3 / locals.var_icap_ihc))) };
        locals.var_fi_dn4 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn4)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn4 / locals.var_icap_ihc))) };
        locals.var_fi_dn5 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn5)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn5 / locals.var_icap_ihc))) };
        locals.var_fi_dn6 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn6)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn6 / locals.var_icap_ihc))) };
        locals.var_fi_dn7 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn7)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn7 / locals.var_icap_ihc))) };
        locals.var_fi_dn8 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn8)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn8 / locals.var_icap_ihc))) };
        locals.var_fi_dn9 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn9)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn9 / locals.var_icap_ihc))) };
        locals.var_fi_dn10 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn10)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn10 / locals.var_icap_ihc))) };
        locals.var_fi_dn11 = if 0.0 == 0.0 && ((p.p76) as f64).is_finite() && ((p.p76) as f64).fract() == 0.0 { if p.p76 == 0.0 { 0.0 } else { (p.p76 * ((locals.var_icap_ihc).powf(p.p76 - 1.0) * locals.var_icap_ihc_dn11)) } } else { (assign3530_e3337 * (p.p76 * (locals.var_icap_ihc_dn11 / locals.var_icap_ihc))) };
        locals.var_fi_rv = 0.0;

        let assign3540_e3341: f64 = (1.0 - p.p72);
        let assign3540_e3342: f64 = (locals.var_vdc_ctc_t / assign3540_e3341);
        let assign3540_e3348: f64 = (locals.var_vjc / locals.var_vdc_ctc_t);
        let assign3540_e3349: f64 = (1.0 - assign3540_e3348);
        let assign3540_e3352: f64 = (1.0 - p.p72);
        let assign3540_e3353: f64 = (assign3540_e3349).powf(assign3540_e3352);
        let assign3540_e3354: f64 = (locals.var_fi * assign3540_e3353);
        let assign3540_e3355: f64 = (1.0 - assign3540_e3354);
        let assign3540_e3356: f64 = (assign3540_e3342 * assign3540_e3355);
        let assign3540_e3359: f64 = (locals.var_fi * locals.var_bjc);
        let assign3540_e3362: f64 = (locals.var_vjunc - locals.var_vjc);
        let assign3540_e3363: f64 = (assign3540_e3359 * assign3540_e3362);
        let assign3540_e3364: f64 = (assign3540_e3356 + assign3540_e3363);
        locals.var_vcv = assign3540_e3364;
        locals.var_vcv_dn0 = ((((locals.var_vdc_ctc_t_dn0 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn0 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn0 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn0)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn0 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn0)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn0 - locals.var_vjc_dn0))));
        locals.var_vcv_dn1 = ((((locals.var_vdc_ctc_t_dn1 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn1 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn1 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn1)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn1 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn1)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn1 - locals.var_vjc_dn1))));
        locals.var_vcv_dn3 = ((((locals.var_vdc_ctc_t_dn3 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn3 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn3 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn3)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn3 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn3)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn3 - locals.var_vjc_dn3))));
        locals.var_vcv_dn4 = ((((locals.var_vdc_ctc_t_dn4 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn4 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn4 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn4)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn4 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn4)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn4 - locals.var_vjc_dn4))));
        locals.var_vcv_dn5 = ((((locals.var_vdc_ctc_t_dn5 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn5 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn5 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn5)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn5 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn5)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn5 - locals.var_vjc_dn5))));
        locals.var_vcv_dn6 = ((((locals.var_vdc_ctc_t_dn6 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn6 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn6 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn6)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn6 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn6)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn6 - locals.var_vjc_dn6))));
        locals.var_vcv_dn7 = ((((locals.var_vdc_ctc_t_dn7 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn7 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn7 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn7)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn7 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn7)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn7 - locals.var_vjc_dn7))));
        locals.var_vcv_dn8 = ((((locals.var_vdc_ctc_t_dn8 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn8 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn8 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn8)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn8 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn8)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn8 - locals.var_vjc_dn8))));
        locals.var_vcv_dn9 = ((((locals.var_vdc_ctc_t_dn9 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn9 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn9 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn9)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn9 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn9)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn9 - locals.var_vjc_dn9))));
        locals.var_vcv_dn10 = ((((locals.var_vdc_ctc_t_dn10 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn10 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn10 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn10)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn10 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn10)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn10 - locals.var_vjc_dn10))));
        locals.var_vcv_dn11 = ((((locals.var_vdc_ctc_t_dn11 / assign3540_e3341) * assign3540_e3355) + (assign3540_e3342 * (-((locals.var_fi_dn11 * assign3540_e3353) + (locals.var_fi * if 0.0 == 0.0 && ((assign3540_e3352) as f64).is_finite() && ((assign3540_e3352) as f64).fract() == 0.0 { if assign3540_e3352 == 0.0 { 0.0 } else { (assign3540_e3352 * ((assign3540_e3349).powf(assign3540_e3352 - 1.0) * (-(((locals.var_vjc_dn11 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))))) } } else { (assign3540_e3353 * (assign3540_e3352 * ((-(((locals.var_vjc_dn11 * locals.var_vdc_ctc_t) - (locals.var_vjc * locals.var_vdc_ctc_t_dn11)) / (locals.var_vdc_ctc_t * locals.var_vdc_ctc_t))) / assign3540_e3349))) }))))) + ((((locals.var_fi_dn11 * locals.var_bjc) + (locals.var_fi * locals.var_bjc_dn11)) * assign3540_e3362) + (assign3540_e3359 * (locals.var_vjunc_dn11 - locals.var_vjc_dn11))));
        locals.var_vcv_rv = 0.0;

        let assign3550_e3367: f64 = (1.0 - locals.var_xp_t);
        let assign3550_e3369: f64 = (assign3550_e3367 * locals.var_vcv);
        let assign3550_e3372: f64 = (locals.var_xp_t * locals.var_vb2c1);
        let assign3550_e3373: f64 = (assign3550_e3369 + assign3550_e3372);
        locals.var_vtc = assign3550_e3373;
        locals.var_vtc_dn0 = ((((-locals.var_xp_t_dn0) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn0)) + (locals.var_xp_t_dn0 * locals.var_vb2c1));
        locals.var_vtc_dn1 = ((((-locals.var_xp_t_dn1) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn1)) + (locals.var_xp_t_dn1 * locals.var_vb2c1));
        locals.var_vtc_dn3 = ((((-locals.var_xp_t_dn3) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn3)) + (locals.var_xp_t_dn3 * locals.var_vb2c1));
        locals.var_vtc_dn4 = ((((-locals.var_xp_t_dn4) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn4)) + (locals.var_xp_t_dn4 * locals.var_vb2c1));
        locals.var_vtc_dn5 = ((((-locals.var_xp_t_dn5) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn5)) + (locals.var_xp_t_dn5 * locals.var_vb2c1));
        locals.var_vtc_dn6 = ((((-locals.var_xp_t_dn6) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn6)) + (locals.var_xp_t_dn6 * locals.var_vb2c1));
        locals.var_vtc_dn7 = ((((-locals.var_xp_t_dn7) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn7)) + ((locals.var_xp_t_dn7 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn7)));
        locals.var_vtc_dn8 = ((((-locals.var_xp_t_dn8) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn8)) + ((locals.var_xp_t_dn8 * locals.var_vb2c1) + (locals.var_xp_t * locals.var_vb2c1_dn8)));
        locals.var_vtc_dn9 = ((((-locals.var_xp_t_dn9) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn9)) + (locals.var_xp_t_dn9 * locals.var_vb2c1));
        locals.var_vtc_dn10 = ((((-locals.var_xp_t_dn10) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn10)) + (locals.var_xp_t_dn10 * locals.var_vb2c1));
        locals.var_vtc_dn11 = ((((-locals.var_xp_t_dn11) * locals.var_vcv) + (assign3550_e3367 * locals.var_vcv_dn11)) + (locals.var_xp_t_dn11 * locals.var_vb2c1));
        locals.var_vtc_rv = 0.0;

        let assign3560_e3376: f64 = (4.0 * locals.var_is_t);
        let assign3560_e3378: f64 = (assign3560_e3376 / locals.var_ik_t);
        locals.var_if0 = assign3560_e3378;
        locals.var_if0_dn0 = ((4.0 * locals.var_is_t_dn0) / locals.var_ik_t);
        locals.var_if0_dn1 = ((4.0 * locals.var_is_t_dn1) / locals.var_ik_t);
        locals.var_if0_dn3 = ((4.0 * locals.var_is_t_dn3) / locals.var_ik_t);
        locals.var_if0_dn4 = ((((4.0 * locals.var_is_t_dn4) * locals.var_ik_t) - (assign3560_e3376 * locals.var_ik_t_dn4)) / (locals.var_ik_t * locals.var_ik_t));
        locals.var_if0_dn5 = ((4.0 * locals.var_is_t_dn5) / locals.var_ik_t);
        locals.var_if0_dn6 = ((4.0 * locals.var_is_t_dn6) / locals.var_ik_t);
        locals.var_if0_dn7 = ((4.0 * locals.var_is_t_dn7) / locals.var_ik_t);
        locals.var_if0_dn8 = ((4.0 * locals.var_is_t_dn8) / locals.var_ik_t);
        locals.var_if0_dn9 = ((4.0 * locals.var_is_t_dn9) / locals.var_ik_t);
        locals.var_if0_dn10 = ((4.0 * locals.var_is_t_dn10) / locals.var_ik_t);
        locals.var_if0_dn11 = ((4.0 * locals.var_is_t_dn11) / locals.var_ik_t);
        locals.var_if0_rv = 0.0;

        let assign3570_e3381: f64 = (locals.var_if0 * locals.var_evb2e1);
        locals.var_f1 = assign3570_e3381;
        locals.var_f1_dn0 = ((locals.var_if0_dn0 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn0));
        locals.var_f1_dn1 = ((locals.var_if0_dn1 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn1));
        locals.var_f1_dn3 = ((locals.var_if0_dn3 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn3));
        locals.var_f1_dn4 = ((locals.var_if0_dn4 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn4));
        locals.var_f1_dn5 = ((locals.var_if0_dn5 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn5));
        locals.var_f1_dn6 = ((locals.var_if0_dn6 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn6));
        locals.var_f1_dn7 = ((locals.var_if0_dn7 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn7));
        locals.var_f1_dn8 = ((locals.var_if0_dn8 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn8));
        locals.var_f1_dn9 = ((locals.var_if0_dn9 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn9));
        locals.var_f1_dn10 = ((locals.var_if0_dn10 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn10));
        locals.var_f1_dn11 = ((locals.var_if0_dn11 * locals.var_evb2e1) + (locals.var_if0 * locals.var_evb2e1_dn11));
        locals.var_f1_rv = 0.0;

        let assign3580_e3386: f64 = (1.0 + locals.var_f1);
        let assign3580_e3387: f64 = (assign3580_e3386).sqrt();
        let assign3580_e3388: f64 = (1.0 + assign3580_e3387);
        let assign3580_e3389: f64 = (locals.var_f1 / assign3580_e3388);
        locals.var_n0 = assign3580_e3389;
        locals.var_n0_dn0 = (((locals.var_f1_dn0 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn0 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn1 = (((locals.var_f1_dn1 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn1 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn3 = (((locals.var_f1_dn3 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn3 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn4 = (((locals.var_f1_dn4 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn4 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn5 = (((locals.var_f1_dn5 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn5 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn6 = (((locals.var_f1_dn6 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn6 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn7 = (((locals.var_f1_dn7 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn7 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn8 = (((locals.var_f1_dn8 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn8 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn9 = (((locals.var_f1_dn9 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn9 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn10 = (((locals.var_f1_dn10 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn10 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_dn11 = (((locals.var_f1_dn11 * assign3580_e3388) - (locals.var_f1 * (locals.var_f1_dn11 / (2.0 * assign3580_e3387)))) / (assign3580_e3388 * assign3580_e3388));
        locals.var_n0_rv = 0.0;

        let assign3590_e3393: f64 = (1.0 / locals.var_nfr_t);
        let assign3590_e3394: f64 = (locals.var_evb2c2star).powf(assign3590_e3393);
        locals.var_evb2c2star_nfr = assign3590_e3394;
        locals.var_evb2c2star_nfr_dn0 = if (-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn0)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn0 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn0 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn1 = if (-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn1)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn1 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn1 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn3 = if (-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn3)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn3 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn3 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn4 = if (-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn4)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn4 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn4 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn5 = if (-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn5)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn5 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn5 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn6 = if (-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn6)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn6 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn6 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn7 = if (-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn7)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn7 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn7 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn8 = if (-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn8)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn8 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn8 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn9 = if (-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn9)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn9 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn9 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn10 = if (-(locals.var_nfr_t_dn10 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn10)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn10 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn10 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_dn11 = if (-(locals.var_nfr_t_dn11 / (locals.var_nfr_t * locals.var_nfr_t))) == 0.0 && ((assign3590_e3393) as f64).is_finite() && ((assign3590_e3393) as f64).fract() == 0.0 { if assign3590_e3393 == 0.0 { 0.0 } else { (assign3590_e3393 * ((locals.var_evb2c2star).powf(assign3590_e3393 - 1.0) * locals.var_evb2c2star_dn11)) } } else { (assign3590_e3394 * (((-(locals.var_nfr_t_dn11 / (locals.var_nfr_t * locals.var_nfr_t))) * (locals.var_evb2c2star).ln()) + (assign3590_e3393 * (locals.var_evb2c2star_dn11 / locals.var_evb2c2star)))) };
        locals.var_evb2c2star_nfr_rv = 0.0;

        let assign3600_e3397: f64 = (locals.var_if0 * locals.var_evb2c2star_nfr);
        locals.var_f2 = assign3600_e3397;
        locals.var_f2_dn0 = ((locals.var_if0_dn0 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn0));
        locals.var_f2_dn1 = ((locals.var_if0_dn1 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn1));
        locals.var_f2_dn3 = ((locals.var_if0_dn3 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn3));
        locals.var_f2_dn4 = ((locals.var_if0_dn4 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn4));
        locals.var_f2_dn5 = ((locals.var_if0_dn5 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn5));
        locals.var_f2_dn6 = ((locals.var_if0_dn6 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn6));
        locals.var_f2_dn7 = ((locals.var_if0_dn7 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn7));
        locals.var_f2_dn8 = ((locals.var_if0_dn8 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn8));
        locals.var_f2_dn9 = ((locals.var_if0_dn9 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn9));
        locals.var_f2_dn10 = ((locals.var_if0_dn10 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn10));
        locals.var_f2_dn11 = ((locals.var_if0_dn11 * locals.var_evb2c2star_nfr) + (locals.var_if0 * locals.var_evb2c2star_nfr_dn11));
        locals.var_f2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3610_e3402: f64 = (1.0 + locals.var_f2);
        let assign3610_e3403: f64 = (assign3610_e3402).sqrt();
        let assign3610_e3404: f64 = (1.0 + assign3610_e3403);
        let assign3610_e3405: f64 = (locals.var_f2 / assign3610_e3404);
        locals.var_nb = assign3610_e3405;
        locals.var_nb_dn0 = (((locals.var_f2_dn0 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn0 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn1 = (((locals.var_f2_dn1 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn1 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn3 = (((locals.var_f2_dn3 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn3 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn4 = (((locals.var_f2_dn4 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn4 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn5 = (((locals.var_f2_dn5 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn5 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn6 = (((locals.var_f2_dn6 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn6 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn7 = (((locals.var_f2_dn7 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn7 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn8 = (((locals.var_f2_dn8 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn8 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn9 = (((locals.var_f2_dn9 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn9 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn10 = (((locals.var_f2_dn10 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn10 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_dn11 = (((locals.var_f2_dn11 * assign3610_e3404) - (locals.var_f2 * (locals.var_f2_dn11 / (2.0 * assign3610_e3403)))) / (assign3610_e3404 * assign3610_e3404));
        locals.var_nb_rv = 0.0;

        let assign3620_e3408: f64 = if p.p92 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign3620_e3408;
        locals.var_guard57_rv = 0.0;

        let (assign3630_e3420, assign3630_e3420_d_n0, assign3630_e3420_d_n1, assign3630_e3420_d_n3, assign3630_e3420_d_n4, assign3630_e3420_d_n5, assign3630_e3420_d_n6, assign3630_e3420_d_n7, assign3630_e3420_d_n8, assign3630_e3420_d_n9, assign3630_e3420_d_n10, assign3630_e3420_d_n11,) = {
    if (locals.var_guard57 != 0.0) {
        let assign3630_e3413: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3630_e3414: f64 = (1.0 + assign3630_e3413);
        let assign3630_e3417: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign3630_e3418: f64 = (assign3630_e3414 + assign3630_e3417);
        (assign3630_e3418, ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn10 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t))), ((((locals.var_vte_dn11 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn11)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn11 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn11)) / (locals.var_vef_t * locals.var_vef_t))),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9, locals.var_q0i_dn10, locals.var_q0i_dn11,)
    }
};
        locals.var_q0i = assign3630_e3420;
        locals.var_q0i_dn0 = assign3630_e3420_d_n0;
        locals.var_q0i_dn1 = assign3630_e3420_d_n1;
        locals.var_q0i_dn3 = assign3630_e3420_d_n3;
        locals.var_q0i_dn4 = assign3630_e3420_d_n4;
        locals.var_q0i_dn5 = assign3630_e3420_d_n5;
        locals.var_q0i_dn6 = assign3630_e3420_d_n6;
        locals.var_q0i_dn7 = assign3630_e3420_d_n7;
        locals.var_q0i_dn8 = assign3630_e3420_d_n8;
        locals.var_q0i_dn9 = assign3630_e3420_d_n9;
        locals.var_q0i_dn10 = assign3630_e3420_d_n10;
        locals.var_q0i_dn11 = assign3630_e3420_d_n11;
        locals.var_q0i_rv = 0.0;

        let (assign3640_e3433, assign3640_e3433_d_n0, assign3640_e3433_d_n1, assign3640_e3433_d_n3, assign3640_e3433_d_n4, assign3640_e3433_d_n5, assign3640_e3433_d_n6, assign3640_e3433_d_n7, assign3640_e3433_d_n8, assign3640_e3433_d_n9, assign3640_e3433_d_n10, assign3640_e3433_d_n11,) = {
    if (locals.var_guard57 == 0.0) {
        let assign3640_e3425: f64 = (locals.var_vte / locals.var_ver_t);
        let assign3640_e3427: f64 = (assign3640_e3425 + 1.0);
        let assign3640_e3429: f64 = (assign3640_e3427 * locals.var_deg_t);
        let assign3640_e3431: f64 = (assign3640_e3429 * locals.var_vtinv);
        (assign3640_e3431, (((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) + (assign3640_e3427 * locals.var_deg_t_dn4)) * locals.var_vtinv) + (assign3640_e3429 * locals.var_vtinv_dn4)), (((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv), (((((locals.var_vte_dn11 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn11)) / (locals.var_ver_t * locals.var_ver_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_terme, locals.var_terme_dn0, locals.var_terme_dn1, locals.var_terme_dn3, locals.var_terme_dn4, locals.var_terme_dn5, locals.var_terme_dn6, locals.var_terme_dn7, locals.var_terme_dn8, locals.var_terme_dn9, locals.var_terme_dn10, locals.var_terme_dn11,)
    }
};
        locals.var_terme = assign3640_e3433;
        locals.var_terme_dn0 = assign3640_e3433_d_n0;
        locals.var_terme_dn1 = assign3640_e3433_d_n1;
        locals.var_terme_dn3 = assign3640_e3433_d_n3;
        locals.var_terme_dn4 = assign3640_e3433_d_n4;
        locals.var_terme_dn5 = assign3640_e3433_d_n5;
        locals.var_terme_dn6 = assign3640_e3433_d_n6;
        locals.var_terme_dn7 = assign3640_e3433_d_n7;
        locals.var_terme_dn8 = assign3640_e3433_d_n8;
        locals.var_terme_dn9 = assign3640_e3433_d_n9;
        locals.var_terme_dn10 = assign3640_e3433_d_n10;
        locals.var_terme_dn11 = assign3640_e3433_d_n11;
        locals.var_terme_rv = 0.0;

        let (assign3650_e3445, assign3650_e3445_d_n0, assign3650_e3445_d_n1, assign3650_e3445_d_n3, assign3650_e3445_d_n4, assign3650_e3445_d_n5, assign3650_e3445_d_n6, assign3650_e3445_d_n7, assign3650_e3445_d_n8, assign3650_e3445_d_n9, assign3650_e3445_d_n10, assign3650_e3445_d_n11,) = {
    if (locals.var_guard57 == 0.0) {
        let assign3650_e3437: f64 = (-locals.var_vtc);
        let assign3650_e3439: f64 = (assign3650_e3437 / locals.var_vef_t);
        let assign3650_e3441: f64 = (assign3650_e3439 * locals.var_deg_t);
        let assign3650_e3443: f64 = (assign3650_e3441 * locals.var_vtinv);
        (assign3650_e3443, ((((((-locals.var_vtc_dn0) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn1) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn3) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((((-locals.var_vtc_dn4) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) + (assign3650_e3439 * locals.var_deg_t_dn4)) * locals.var_vtinv) + (assign3650_e3441 * locals.var_vtinv_dn4)), ((((((-locals.var_vtc_dn5) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn6) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn7) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn8) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn9) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn10) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv), ((((((-locals.var_vtc_dn11) * locals.var_vef_t) - (assign3650_e3437 * locals.var_vef_t_dn11)) / (locals.var_vef_t * locals.var_vef_t)) * locals.var_deg_t) * locals.var_vtinv),)
    } else {
        (locals.var_termc, locals.var_termc_dn0, locals.var_termc_dn1, locals.var_termc_dn3, locals.var_termc_dn4, locals.var_termc_dn5, locals.var_termc_dn6, locals.var_termc_dn7, locals.var_termc_dn8, locals.var_termc_dn9, locals.var_termc_dn10, locals.var_termc_dn11,)
    }
};
        locals.var_termc = assign3650_e3445;
        locals.var_termc_dn0 = assign3650_e3445_d_n0;
        locals.var_termc_dn1 = assign3650_e3445_d_n1;
        locals.var_termc_dn3 = assign3650_e3445_d_n3;
        locals.var_termc_dn4 = assign3650_e3445_d_n4;
        locals.var_termc_dn5 = assign3650_e3445_d_n5;
        locals.var_termc_dn6 = assign3650_e3445_d_n6;
        locals.var_termc_dn7 = assign3650_e3445_d_n7;
        locals.var_termc_dn8 = assign3650_e3445_d_n8;
        locals.var_termc_dn9 = assign3650_e3445_d_n9;
        locals.var_termc_dn10 = assign3650_e3445_d_n10;
        locals.var_termc_dn11 = assign3650_e3445_d_n11;
        locals.var_termc_rv = 0.0;

        let (assign3660_e3461, assign3660_e3461_d_n0, assign3660_e3461_d_n1, assign3660_e3461_d_n3, assign3660_e3461_d_n4, assign3660_e3461_d_n5, assign3660_e3461_d_n6, assign3660_e3461_d_n7, assign3660_e3461_d_n8, assign3660_e3461_d_n9, assign3660_e3461_d_n10, assign3660_e3461_d_n11,) = {
    if (locals.var_guard57 == 0.0) {
        let assign3660_e3449: f64 = (locals.var_terme).exp();
        let assign3660_e3451: f64 = (locals.var_termc).exp();
        let assign3660_e3452: f64 = (assign3660_e3449 - assign3660_e3451);
        let assign3660_e3455: f64 = (locals.var_deg_t * locals.var_vtinv);
        let assign3660_e3456: f64 = (assign3660_e3455).exp();
        let assign3660_e3458: f64 = (assign3660_e3456 - 1.0);
        let assign3660_e3459: f64 = (assign3660_e3452 / assign3660_e3458);
        (assign3660_e3459, (((assign3660_e3449 * locals.var_terme_dn0) - (assign3660_e3451 * locals.var_termc_dn0)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn1) - (assign3660_e3451 * locals.var_termc_dn1)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn3) - (assign3660_e3451 * locals.var_termc_dn3)) / assign3660_e3458), (((((assign3660_e3449 * locals.var_terme_dn4) - (assign3660_e3451 * locals.var_termc_dn4)) * assign3660_e3458) - (assign3660_e3452 * (assign3660_e3456 * ((locals.var_deg_t_dn4 * locals.var_vtinv) + (locals.var_deg_t * locals.var_vtinv_dn4))))) / (assign3660_e3458 * assign3660_e3458)), (((assign3660_e3449 * locals.var_terme_dn5) - (assign3660_e3451 * locals.var_termc_dn5)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn6) - (assign3660_e3451 * locals.var_termc_dn6)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn7) - (assign3660_e3451 * locals.var_termc_dn7)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn8) - (assign3660_e3451 * locals.var_termc_dn8)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn9) - (assign3660_e3451 * locals.var_termc_dn9)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn10) - (assign3660_e3451 * locals.var_termc_dn10)) / assign3660_e3458), (((assign3660_e3449 * locals.var_terme_dn11) - (assign3660_e3451 * locals.var_termc_dn11)) / assign3660_e3458),)
    } else {
        (locals.var_q0i, locals.var_q0i_dn0, locals.var_q0i_dn1, locals.var_q0i_dn3, locals.var_q0i_dn4, locals.var_q0i_dn5, locals.var_q0i_dn6, locals.var_q0i_dn7, locals.var_q0i_dn8, locals.var_q0i_dn9, locals.var_q0i_dn10, locals.var_q0i_dn11,)
    }
};
        locals.var_q0i = assign3660_e3461;
        locals.var_q0i_dn0 = assign3660_e3461_d_n0;
        locals.var_q0i_dn1 = assign3660_e3461_d_n1;
        locals.var_q0i_dn3 = assign3660_e3461_d_n3;
        locals.var_q0i_dn4 = assign3660_e3461_d_n4;
        locals.var_q0i_dn5 = assign3660_e3461_d_n5;
        locals.var_q0i_dn6 = assign3660_e3461_d_n6;
        locals.var_q0i_dn7 = assign3660_e3461_d_n7;
        locals.var_q0i_dn8 = assign3660_e3461_d_n8;
        locals.var_q0i_dn9 = assign3660_e3461_d_n9;
        locals.var_q0i_dn10 = assign3660_e3461_d_n10;
        locals.var_q0i_dn11 = assign3660_e3461_d_n11;
        locals.var_q0i_rv = 0.0;

        let assign3670_e3464: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign3670_e3464;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_dn11 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign3680_e3467: f64 = (locals.var_q0i * locals.var_q0i);
        locals.var_x2 = assign3680_e3467;
        locals.var_x2_dn0 = ((locals.var_q0i_dn0 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn0));
        locals.var_x2_dn1 = ((locals.var_q0i_dn1 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn1));
        locals.var_x2_dn3 = ((locals.var_q0i_dn3 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn3));
        locals.var_x2_dn4 = ((locals.var_q0i_dn4 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn4));
        locals.var_x2_dn5 = ((locals.var_q0i_dn5 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn5));
        locals.var_x2_dn6 = ((locals.var_q0i_dn6 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn6));
        locals.var_x2_dn7 = ((locals.var_q0i_dn7 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn7));
        locals.var_x2_dn8 = ((locals.var_q0i_dn8 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn8));
        locals.var_x2_dn9 = ((locals.var_q0i_dn9 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn9));
        locals.var_x2_dn10 = ((locals.var_q0i_dn10 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn10));
        locals.var_x2_dn11 = ((locals.var_q0i_dn11 * locals.var_q0i) + (locals.var_q0i * locals.var_q0i_dn11));
        locals.var_x2_rv = 0.0;

        let assign3690_e3470: f64 = if locals.var_q0i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign3690_e3470;
        locals.var_guard58_rv = 0.0;

        let (assign3700_e3483, assign3700_e3483_d_n0, assign3700_e3483_d_n1, assign3700_e3483_d_n3, assign3700_e3483_d_n4, assign3700_e3483_d_n5, assign3700_e3483_d_n6, assign3700_e3483_d_n7, assign3700_e3483_d_n8, assign3700_e3483_d_n9, assign3700_e3483_d_n10, assign3700_e3483_d_n11,) = {
    if (locals.var_guard58 != 0.0) {
        let assign3700_e3474: f64 = (0.5 * locals.var_eps2);
        let assign3700_e3477: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3700_e3478: f64 = (assign3700_e3477).sqrt();
        let assign3700_e3480: f64 = (assign3700_e3478 - locals.var_q0i);
        let assign3700_e3481: f64 = (assign3700_e3474 / assign3700_e3480);
        (assign3700_e3481, ((((0.5 * locals.var_eps2_dn0) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn0))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn1) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn1))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn3) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn3))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn4) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn4))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn5) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn5))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn6) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn6))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn7) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn7))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn8) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn8))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn9) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn9))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn10) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn10))) / (assign3700_e3480 * assign3700_e3480)), ((((0.5 * locals.var_eps2_dn11) * assign3700_e3480) - (assign3700_e3474 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign3700_e3478)) - locals.var_q0i_dn11))) / (assign3700_e3480 * assign3700_e3480)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9, locals.var_q1i_dn10, locals.var_q1i_dn11,)
    }
};
        locals.var_q1i = assign3700_e3483;
        locals.var_q1i_dn0 = assign3700_e3483_d_n0;
        locals.var_q1i_dn1 = assign3700_e3483_d_n1;
        locals.var_q1i_dn3 = assign3700_e3483_d_n3;
        locals.var_q1i_dn4 = assign3700_e3483_d_n4;
        locals.var_q1i_dn5 = assign3700_e3483_d_n5;
        locals.var_q1i_dn6 = assign3700_e3483_d_n6;
        locals.var_q1i_dn7 = assign3700_e3483_d_n7;
        locals.var_q1i_dn8 = assign3700_e3483_d_n8;
        locals.var_q1i_dn9 = assign3700_e3483_d_n9;
        locals.var_q1i_dn10 = assign3700_e3483_d_n10;
        locals.var_q1i_dn11 = assign3700_e3483_d_n11;
        locals.var_q1i_rv = 0.0;

        let (assign3710_e3495, assign3710_e3495_d_n0, assign3710_e3495_d_n1, assign3710_e3495_d_n3, assign3710_e3495_d_n4, assign3710_e3495_d_n5, assign3710_e3495_d_n6, assign3710_e3495_d_n7, assign3710_e3495_d_n8, assign3710_e3495_d_n9, assign3710_e3495_d_n10, assign3710_e3495_d_n11,) = {
    if (locals.var_guard58 == 0.0) {
        let assign3710_e3489: f64 = (locals.var_x2 + locals.var_eps2);
        let assign3710_e3490: f64 = (assign3710_e3489).sqrt();
        let assign3710_e3492: f64 = (assign3710_e3490 + locals.var_q0i);
        let assign3710_e3493: f64 = (0.5 * assign3710_e3492);
        (assign3710_e3493, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign3710_e3490)) + locals.var_q0i_dn11)),)
    } else {
        (locals.var_q1i, locals.var_q1i_dn0, locals.var_q1i_dn1, locals.var_q1i_dn3, locals.var_q1i_dn4, locals.var_q1i_dn5, locals.var_q1i_dn6, locals.var_q1i_dn7, locals.var_q1i_dn8, locals.var_q1i_dn9, locals.var_q1i_dn10, locals.var_q1i_dn11,)
    }
};
        locals.var_q1i = assign3710_e3495;
        locals.var_q1i_dn0 = assign3710_e3495_d_n0;
        locals.var_q1i_dn1 = assign3710_e3495_d_n1;
        locals.var_q1i_dn3 = assign3710_e3495_d_n3;
        locals.var_q1i_dn4 = assign3710_e3495_d_n4;
        locals.var_q1i_dn5 = assign3710_e3495_d_n5;
        locals.var_q1i_dn6 = assign3710_e3495_d_n6;
        locals.var_q1i_dn7 = assign3710_e3495_d_n7;
        locals.var_q1i_dn8 = assign3710_e3495_d_n8;
        locals.var_q1i_dn9 = assign3710_e3495_d_n9;
        locals.var_q1i_dn10 = assign3710_e3495_d_n10;
        locals.var_q1i_dn11 = assign3710_e3495_d_n11;
        locals.var_q1i_rv = 0.0;

        let assign3720_e3501: f64 = (locals.var_n0 + locals.var_nb);
        let assign3720_e3502: f64 = (0.5 * assign3720_e3501);
        let assign3720_e3503: f64 = (1.0 + assign3720_e3502);
        let assign3720_e3504: f64 = (locals.var_q1i * assign3720_e3503);
        locals.var_qbi = assign3720_e3504;
        locals.var_qbi_dn0 = ((locals.var_q1i_dn0 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbi_dn1 = ((locals.var_q1i_dn1 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbi_dn3 = ((locals.var_q1i_dn3 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbi_dn4 = ((locals.var_q1i_dn4 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbi_dn5 = ((locals.var_q1i_dn5 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbi_dn6 = ((locals.var_q1i_dn6 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbi_dn7 = ((locals.var_q1i_dn7 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbi_dn8 = ((locals.var_q1i_dn8 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbi_dn9 = ((locals.var_q1i_dn9 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));
        locals.var_qbi_dn10 = ((locals.var_q1i_dn10 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn10 + locals.var_nb_dn10))));
        locals.var_qbi_dn11 = ((locals.var_q1i_dn11 * assign3720_e3503) + (locals.var_q1i * (0.5 * (locals.var_n0_dn11 + locals.var_nb_dn11))));
        locals.var_qbi_rv = 0.0;

        let assign3730_e3507: f64 = (p.p15 * locals.var_is_t);
        let assign3730_e3509: f64 = (assign3730_e3507 * locals.var_evb2c2star_nfr);
        locals.var_ir = assign3730_e3509;
        locals.var_ir_dn0 = (((p.p15 * locals.var_is_t_dn0) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn0));
        locals.var_ir_dn1 = (((p.p15 * locals.var_is_t_dn1) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn1));
        locals.var_ir_dn3 = (((p.p15 * locals.var_is_t_dn3) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn3));
        locals.var_ir_dn4 = (((p.p15 * locals.var_is_t_dn4) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn4));
        locals.var_ir_dn5 = (((p.p15 * locals.var_is_t_dn5) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn5));
        locals.var_ir_dn6 = (((p.p15 * locals.var_is_t_dn6) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn6));
        locals.var_ir_dn7 = (((p.p15 * locals.var_is_t_dn7) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn7));
        locals.var_ir_dn8 = (((p.p15 * locals.var_is_t_dn8) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn8));
        locals.var_ir_dn9 = (((p.p15 * locals.var_is_t_dn9) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn9));
        locals.var_ir_dn10 = (((p.p15 * locals.var_is_t_dn10) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn10));
        locals.var_ir_dn11 = (((p.p15 * locals.var_is_t_dn11) * locals.var_evb2c2star_nfr) + (assign3730_e3507 * locals.var_evb2c2star_nfr_dn11));
        locals.var_ir_rv = 0.0;

        let assign3740_e3512: f64 = (locals.var_is_t * locals.var_evb2e1);
        locals.var_if_ = assign3740_e3512;
        locals.var_if__dn0 = ((locals.var_is_t_dn0 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn0));
        locals.var_if__dn1 = ((locals.var_is_t_dn1 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn1));
        locals.var_if__dn3 = ((locals.var_is_t_dn3 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn3));
        locals.var_if__dn4 = ((locals.var_is_t_dn4 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn4));
        locals.var_if__dn5 = ((locals.var_is_t_dn5 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn5));
        locals.var_if__dn6 = ((locals.var_is_t_dn6 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn6));
        locals.var_if__dn7 = ((locals.var_is_t_dn7 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn7));
        locals.var_if__dn8 = ((locals.var_is_t_dn8 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn8));
        locals.var_if__dn9 = ((locals.var_is_t_dn9 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn9));
        locals.var_if__dn10 = ((locals.var_is_t_dn10 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn10));
        locals.var_if__dn11 = ((locals.var_is_t_dn11 * locals.var_evb2e1) + (locals.var_is_t * locals.var_evb2e1_dn11));
        locals.var_if__rv = 0.0;

        let assign3750_e3515: f64 = (locals.var_if_ - locals.var_ir);
        let assign3750_e3517: f64 = (assign3750_e3515 / locals.var_qbi);
        locals.var_in_ = assign3750_e3517;
        locals.var_in__dn0 = ((((locals.var_if__dn0 - locals.var_ir_dn0) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn0)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn1 = ((((locals.var_if__dn1 - locals.var_ir_dn1) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn1)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn3 = ((((locals.var_if__dn3 - locals.var_ir_dn3) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn3)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn4 = ((((locals.var_if__dn4 - locals.var_ir_dn4) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn4)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn5 = ((((locals.var_if__dn5 - locals.var_ir_dn5) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn5)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn6 = ((((locals.var_if__dn6 - locals.var_ir_dn6) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn6)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn7 = ((((locals.var_if__dn7 - locals.var_ir_dn7) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn7)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn8 = ((((locals.var_if__dn8 - locals.var_ir_dn8) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn8)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn9 = ((((locals.var_if__dn9 - locals.var_ir_dn9) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn9)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn10 = ((((locals.var_if__dn10 - locals.var_ir_dn10) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn10)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__dn11 = ((((locals.var_if__dn11 - locals.var_ir_dn11) * locals.var_qbi) - (assign3750_e3515 * locals.var_qbi_dn11)) / (locals.var_qbi * locals.var_qbi));
        locals.var_in__rv = 0.0;

        let assign3760_e3520: f64 = locals.var_vb2e1;
        let assign3760_e3522: f64 = (assign3760_e3520 / 0.0001);
        locals.var_dxa = assign3760_e3522;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = (locals.var_vb2e1_dn5 / 0.0001);
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = (locals.var_vb2e1_dn7 / 0.0001);
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign3770_e3525: f64 = if locals.var_vb2e1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign3770_e3525;
        locals.var_guard59_rv = 0.0;

        let (assign3780_e3537, assign3780_e3537_d_n0, assign3780_e3537_d_n1, assign3780_e3537_d_n3, assign3780_e3537_d_n4, assign3780_e3537_d_n5, assign3780_e3537_d_n6, assign3780_e3537_d_n7, assign3780_e3537_d_n8, assign3780_e3537_d_n9, assign3780_e3537_d_n10, assign3780_e3537_d_n11,) = {
    if (locals.var_guard59 != 0.0) {
        let assign3780_e3531: f64 = (locals.var_dxa).exp();
        let assign3780_e3532: f64 = (1.0 + assign3780_e3531);
        let assign3780_e3533: f64 = (assign3780_e3532).ln();
        let assign3780_e3534: f64 = (0.0001 * assign3780_e3533);
        let assign3780_e3535: f64 = assign3780_e3534;
        (assign3780_e3535, (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn0) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn1) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn3) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn4) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn5) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn6) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn7) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn8) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn9) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn10) / assign3780_e3532)), (0.0001 * ((assign3780_e3531 * locals.var_dxa_dn11) / assign3780_e3532)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign3780_e3537;
        locals.var_tmpexp_dn0 = assign3780_e3537_d_n0;
        locals.var_tmpexp_dn1 = assign3780_e3537_d_n1;
        locals.var_tmpexp_dn3 = assign3780_e3537_d_n3;
        locals.var_tmpexp_dn4 = assign3780_e3537_d_n4;
        locals.var_tmpexp_dn5 = assign3780_e3537_d_n5;
        locals.var_tmpexp_dn6 = assign3780_e3537_d_n6;
        locals.var_tmpexp_dn7 = assign3780_e3537_d_n7;
        locals.var_tmpexp_dn8 = assign3780_e3537_d_n8;
        locals.var_tmpexp_dn9 = assign3780_e3537_d_n9;
        locals.var_tmpexp_dn10 = assign3780_e3537_d_n10;
        locals.var_tmpexp_dn11 = assign3780_e3537_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign3790_e3551, assign3790_e3551_d_n0, assign3790_e3551_d_n1, assign3790_e3551_d_n3, assign3790_e3551_d_n4, assign3790_e3551_d_n5, assign3790_e3551_d_n6, assign3790_e3551_d_n7, assign3790_e3551_d_n8, assign3790_e3551_d_n9, assign3790_e3551_d_n10, assign3790_e3551_d_n11,) = {
    if (locals.var_guard59 == 0.0) {
        let assign3790_e3544: f64 = (-locals.var_dxa);
        let assign3790_e3545: f64 = (assign3790_e3544).exp();
        let assign3790_e3546: f64 = (1.0 + assign3790_e3545);
        let assign3790_e3547: f64 = (assign3790_e3546).ln();
        let assign3790_e3548: f64 = (0.0001 * assign3790_e3547);
        let assign3790_e3549: f64 = (locals.var_vb2e1 + assign3790_e3548);
        (assign3790_e3549, (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn0)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn1)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn3)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn4)) / assign3790_e3546)), (locals.var_vb2e1_dn5 + (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn5)) / assign3790_e3546))), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn6)) / assign3790_e3546)), (locals.var_vb2e1_dn7 + (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn7)) / assign3790_e3546))), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn8)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn9)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn10)) / assign3790_e3546)), (0.0001 * ((assign3790_e3545 * (-locals.var_dxa_dn11)) / assign3790_e3546)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign3790_e3551;
        locals.var_tmpexp_dn0 = assign3790_e3551_d_n0;
        locals.var_tmpexp_dn1 = assign3790_e3551_d_n1;
        locals.var_tmpexp_dn3 = assign3790_e3551_d_n3;
        locals.var_tmpexp_dn4 = assign3790_e3551_d_n4;
        locals.var_tmpexp_dn5 = assign3790_e3551_d_n5;
        locals.var_tmpexp_dn6 = assign3790_e3551_d_n6;
        locals.var_tmpexp_dn7 = assign3790_e3551_d_n7;
        locals.var_tmpexp_dn8 = assign3790_e3551_d_n8;
        locals.var_tmpexp_dn9 = assign3790_e3551_d_n9;
        locals.var_tmpexp_dn10 = assign3790_e3551_d_n10;
        locals.var_tmpexp_dn11 = assign3790_e3551_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign3800_e3554: f64 = (locals.var_tmpexp / p.p156);
        locals.var_tmpexp1 = assign3800_e3554;
        locals.var_tmpexp1_dn0 = (locals.var_tmpexp_dn0 / p.p156);
        locals.var_tmpexp1_dn1 = (locals.var_tmpexp_dn1 / p.p156);
        locals.var_tmpexp1_dn3 = (locals.var_tmpexp_dn3 / p.p156);
        locals.var_tmpexp1_dn4 = (locals.var_tmpexp_dn4 / p.p156);
        locals.var_tmpexp1_dn5 = (locals.var_tmpexp_dn5 / p.p156);
        locals.var_tmpexp1_dn6 = (locals.var_tmpexp_dn6 / p.p156);
        locals.var_tmpexp1_dn7 = (locals.var_tmpexp_dn7 / p.p156);
        locals.var_tmpexp1_dn8 = (locals.var_tmpexp_dn8 / p.p156);
        locals.var_tmpexp1_dn9 = (locals.var_tmpexp_dn9 / p.p156);
        locals.var_tmpexp1_dn10 = (locals.var_tmpexp_dn10 / p.p156);
        locals.var_tmpexp1_dn11 = (locals.var_tmpexp_dn11 / p.p156);
        locals.var_tmpexp1_rv = 0.0;

        let assign3810_e3557: f64 = if locals.var_tmpexp1 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign3810_e3557;
        locals.var_guard60_rv = 0.0;

        let (assign3830_e3568,) = {
    if (locals.var_guard60 == 0.0) {
        let assign3830_e3566: f64 = (p.p151).exp();
        (assign3830_e3566,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3830_e3568;
        locals.var_expl_rv = 0.0;

        let assign3860_e3587: f64 = (locals.var_vb2e1 - p.p158);
        let assign3860_e3589: f64 = (assign3860_e3587 / 0.001);
        locals.var_dxa = assign3860_e3589;
        locals.var_dxa_dn0 = 0.0;
        locals.var_dxa_dn1 = 0.0;
        locals.var_dxa_dn3 = 0.0;
        locals.var_dxa_dn4 = 0.0;
        locals.var_dxa_dn5 = (locals.var_vb2e1_dn5 / 0.001);
        locals.var_dxa_dn6 = 0.0;
        locals.var_dxa_dn7 = (locals.var_vb2e1_dn7 / 0.001);
        locals.var_dxa_dn8 = 0.0;
        locals.var_dxa_dn9 = 0.0;
        locals.var_dxa_dn10 = 0.0;
        locals.var_dxa_dn11 = 0.0;
        locals.var_dxa_rv = 0.0;

        let assign3910_e3630: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3910_e3632: f64 = (assign3910_e3630 / p.p17);
        let assign3910_e3634: f64 = if assign3910_e3632 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign3910_e3634;
        locals.var_guard62_rv = 0.0;

        let (assign3920_e3643, assign3920_e3643_d_n0, assign3920_e3643_d_n1, assign3920_e3643_d_n3, assign3920_e3643_d_n4, assign3920_e3643_d_n5, assign3920_e3643_d_n6, assign3920_e3643_d_n7, assign3920_e3643_d_n8, assign3920_e3643_d_n9, assign3920_e3643_d_n10, assign3920_e3643_d_n11,) = {
    if (locals.var_guard62 != 0.0) {
        let assign3920_e3638: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3920_e3640: f64 = (assign3920_e3638 / p.p17);
        let assign3920_e3641: f64 = (assign3920_e3640).exp();
        (assign3920_e3641, 0.0, 0.0, 0.0, (assign3920_e3641 * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p17)), (assign3920_e3641 * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p17)), 0.0, (assign3920_e3641 * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p17)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign3920_e3643;
        locals.var_tmpexp_dn0 = assign3920_e3643_d_n0;
        locals.var_tmpexp_dn1 = assign3920_e3643_d_n1;
        locals.var_tmpexp_dn3 = assign3920_e3643_d_n3;
        locals.var_tmpexp_dn4 = assign3920_e3643_d_n4;
        locals.var_tmpexp_dn5 = assign3920_e3643_d_n5;
        locals.var_tmpexp_dn6 = assign3920_e3643_d_n6;
        locals.var_tmpexp_dn7 = assign3920_e3643_d_n7;
        locals.var_tmpexp_dn8 = assign3920_e3643_d_n8;
        locals.var_tmpexp_dn9 = assign3920_e3643_d_n9;
        locals.var_tmpexp_dn10 = assign3920_e3643_d_n10;
        locals.var_tmpexp_dn11 = assign3920_e3643_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign3930_e3649,) = {
    if (locals.var_guard62 == 0.0) {
        let assign3930_e3647: f64 = (p.p151).exp();
        (assign3930_e3647,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3930_e3649;
        locals.var_expl_rv = 0.0;

        let (assign3940_e3664, assign3940_e3664_d_n0, assign3940_e3664_d_n1, assign3940_e3664_d_n3, assign3940_e3664_d_n4, assign3940_e3664_d_n5, assign3940_e3664_d_n6, assign3940_e3664_d_n7, assign3940_e3664_d_n8, assign3940_e3664_d_n9, assign3940_e3664_d_n10, assign3940_e3664_d_n11,) = {
    if (locals.var_guard62 == 0.0) {
        let assign3940_e3656: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign3940_e3658: f64 = (assign3940_e3656 / p.p17);
        let assign3940_e3660: f64 = (assign3940_e3658 - p.p151);
        let assign3940_e3661: f64 = (1.0 + assign3940_e3660);
        let assign3940_e3662: f64 = (locals.var_expl * assign3940_e3661);
        (assign3940_e3662, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p17)), (locals.var_expl * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p17)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p17)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign3940_e3664;
        locals.var_tmpexp_dn0 = assign3940_e3664_d_n0;
        locals.var_tmpexp_dn1 = assign3940_e3664_d_n1;
        locals.var_tmpexp_dn3 = assign3940_e3664_d_n3;
        locals.var_tmpexp_dn4 = assign3940_e3664_d_n4;
        locals.var_tmpexp_dn5 = assign3940_e3664_d_n5;
        locals.var_tmpexp_dn6 = assign3940_e3664_d_n6;
        locals.var_tmpexp_dn7 = assign3940_e3664_d_n7;
        locals.var_tmpexp_dn8 = assign3940_e3664_d_n8;
        locals.var_tmpexp_dn9 = assign3940_e3664_d_n9;
        locals.var_tmpexp_dn10 = assign3940_e3664_d_n10;
        locals.var_tmpexp_dn11 = assign3940_e3664_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign3950_e3667: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign3950_e3667;
        locals.var_guard63_rv = 0.0;

        let assign3960_e3670: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3960_e3672: f64 = (assign3960_e3670 * locals.var_vtinv);
        let assign3960_e3674: f64 = if assign3960_e3672 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign3960_e3674;
        locals.var_guard64_rv = 0.0;

        let (assign3970_e3685, assign3970_e3685_d_n0, assign3970_e3685_d_n1, assign3970_e3685_d_n3, assign3970_e3685_d_n4, assign3970_e3685_d_n5, assign3970_e3685_d_n6, assign3970_e3685_d_n7, assign3970_e3685_d_n8, assign3970_e3685_d_n9, assign3970_e3685_d_n10, assign3970_e3685_d_n11,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign3970_e3680: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3970_e3682: f64 = (assign3970_e3680 * locals.var_vtinv);
        let assign3970_e3683: f64 = (assign3970_e3682).exp();
        (assign3970_e3683, (assign3970_e3683 * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (assign3970_e3683 * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign3970_e3680 * locals.var_vtinv_dn4))), (assign3970_e3683 * ((locals.var_vb2e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn6) * locals.var_vtinv)), (assign3970_e3683 * ((locals.var_vb2e1_dn7 - locals.var_vknbr_t_dn7) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (assign3970_e3683 * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign3970_e3685;
        locals.var_tmpexp1_dn0 = assign3970_e3685_d_n0;
        locals.var_tmpexp1_dn1 = assign3970_e3685_d_n1;
        locals.var_tmpexp1_dn3 = assign3970_e3685_d_n3;
        locals.var_tmpexp1_dn4 = assign3970_e3685_d_n4;
        locals.var_tmpexp1_dn5 = assign3970_e3685_d_n5;
        locals.var_tmpexp1_dn6 = assign3970_e3685_d_n6;
        locals.var_tmpexp1_dn7 = assign3970_e3685_d_n7;
        locals.var_tmpexp1_dn8 = assign3970_e3685_d_n8;
        locals.var_tmpexp1_dn9 = assign3970_e3685_d_n9;
        locals.var_tmpexp1_dn10 = assign3970_e3685_d_n10;
        locals.var_tmpexp1_dn11 = assign3970_e3685_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let (assign3980_e3693,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 == 0.0)) {
        let assign3980_e3691: f64 = (p.p151).exp();
        (assign3980_e3691,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign3980_e3693;
        locals.var_expl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3990_e3710, assign3990_e3710_d_n0, assign3990_e3710_d_n1, assign3990_e3710_d_n3, assign3990_e3710_d_n4, assign3990_e3710_d_n5, assign3990_e3710_d_n6, assign3990_e3710_d_n7, assign3990_e3710_d_n8, assign3990_e3710_d_n9, assign3990_e3710_d_n10, assign3990_e3710_d_n11,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 == 0.0)) {
        let assign3990_e3702: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3990_e3704: f64 = (assign3990_e3702 * locals.var_vtinv);
        let assign3990_e3706: f64 = (assign3990_e3704 - p.p151);
        let assign3990_e3707: f64 = (1.0 + assign3990_e3706);
        let assign3990_e3708: f64 = (locals.var_expl * assign3990_e3707);
        (assign3990_e3708, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign3990_e3702 * locals.var_vtinv_dn4))), (locals.var_expl * ((locals.var_vb2e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2e1_dn7 - locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign3990_e3710;
        locals.var_tmpexp1_dn0 = assign3990_e3710_d_n0;
        locals.var_tmpexp1_dn1 = assign3990_e3710_d_n1;
        locals.var_tmpexp1_dn3 = assign3990_e3710_d_n3;
        locals.var_tmpexp1_dn4 = assign3990_e3710_d_n4;
        locals.var_tmpexp1_dn5 = assign3990_e3710_d_n5;
        locals.var_tmpexp1_dn6 = assign3990_e3710_d_n6;
        locals.var_tmpexp1_dn7 = assign3990_e3710_d_n7;
        locals.var_tmpexp1_dn8 = assign3990_e3710_d_n8;
        locals.var_tmpexp1_dn9 = assign3990_e3710_d_n9;
        locals.var_tmpexp1_dn10 = assign3990_e3710_d_n10;
        locals.var_tmpexp1_dn11 = assign3990_e3710_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let assign4000_e3713: f64 = (locals.var_in_ / locals.var_is_t);
        let assign4000_e3715: f64 = (assign4000_e3713 - 1000.0);
        let assign4000_e3717: f64 = if assign4000_e3715 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign4000_e3717;
        locals.var_guard65_rv = 0.0;

        let (assign4020_e3736,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard65 == 0.0)) {
        let assign4020_e3734: f64 = (40.0_f64).exp();
        (assign4020_e3734,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4020_e3736;
        locals.var_expl_rv = 0.0;

        let assign4080_e3843: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4080_e3845: f64 = (assign4080_e3843 / p.p19);
        let assign4080_e3847: f64 = if assign4080_e3845 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4080_e3847;
        locals.var_guard67_rv = 0.0;

        let (assign4090_e3856, assign4090_e3856_d_n0, assign4090_e3856_d_n1, assign4090_e3856_d_n3, assign4090_e3856_d_n4, assign4090_e3856_d_n5, assign4090_e3856_d_n6, assign4090_e3856_d_n7, assign4090_e3856_d_n8, assign4090_e3856_d_n9, assign4090_e3856_d_n10, assign4090_e3856_d_n11,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4090_e3851: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4090_e3853: f64 = (assign4090_e3851 / p.p19);
        let assign4090_e3854: f64 = (assign4090_e3853).exp();
        (assign4090_e3854, 0.0, 0.0, 0.0, (assign4090_e3854 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p19)), (assign4090_e3854 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p19)), (assign4090_e3854 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p19)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4090_e3856;
        locals.var_tmpexp_dn0 = assign4090_e3856_d_n0;
        locals.var_tmpexp_dn1 = assign4090_e3856_d_n1;
        locals.var_tmpexp_dn3 = assign4090_e3856_d_n3;
        locals.var_tmpexp_dn4 = assign4090_e3856_d_n4;
        locals.var_tmpexp_dn5 = assign4090_e3856_d_n5;
        locals.var_tmpexp_dn6 = assign4090_e3856_d_n6;
        locals.var_tmpexp_dn7 = assign4090_e3856_d_n7;
        locals.var_tmpexp_dn8 = assign4090_e3856_d_n8;
        locals.var_tmpexp_dn9 = assign4090_e3856_d_n9;
        locals.var_tmpexp_dn10 = assign4090_e3856_d_n10;
        locals.var_tmpexp_dn11 = assign4090_e3856_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4100_e3862,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4100_e3860: f64 = (p.p151).exp();
        (assign4100_e3860,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4100_e3862;
        locals.var_expl_rv = 0.0;

        let (assign4110_e3877, assign4110_e3877_d_n0, assign4110_e3877_d_n1, assign4110_e3877_d_n3, assign4110_e3877_d_n4, assign4110_e3877_d_n5, assign4110_e3877_d_n6, assign4110_e3877_d_n7, assign4110_e3877_d_n8, assign4110_e3877_d_n9, assign4110_e3877_d_n10, assign4110_e3877_d_n11,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4110_e3869: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4110_e3871: f64 = (assign4110_e3869 / p.p19);
        let assign4110_e3873: f64 = (assign4110_e3871 - p.p151);
        let assign4110_e3874: f64 = (1.0 + assign4110_e3873);
        let assign4110_e3875: f64 = (locals.var_expl * assign4110_e3874);
        (assign4110_e3875, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p19)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p19)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p19)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4110_e3877;
        locals.var_tmpexp_dn0 = assign4110_e3877_d_n0;
        locals.var_tmpexp_dn1 = assign4110_e3877_d_n1;
        locals.var_tmpexp_dn3 = assign4110_e3877_d_n3;
        locals.var_tmpexp_dn4 = assign4110_e3877_d_n4;
        locals.var_tmpexp_dn5 = assign4110_e3877_d_n5;
        locals.var_tmpexp_dn6 = assign4110_e3877_d_n6;
        locals.var_tmpexp_dn7 = assign4110_e3877_d_n7;
        locals.var_tmpexp_dn8 = assign4110_e3877_d_n8;
        locals.var_tmpexp_dn9 = assign4110_e3877_d_n9;
        locals.var_tmpexp_dn10 = assign4110_e3877_d_n10;
        locals.var_tmpexp_dn11 = assign4110_e3877_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4120_e3880: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4120_e3880;
        locals.var_guard68_rv = 0.0;

        let assign4130_e3883: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4130_e3885: f64 = (assign4130_e3883 * locals.var_vtinv);
        let assign4130_e3887: f64 = if assign4130_e3885 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4130_e3887;
        locals.var_guard69_rv = 0.0;

        let (assign4140_e3898, assign4140_e3898_d_n0, assign4140_e3898_d_n1, assign4140_e3898_d_n3, assign4140_e3898_d_n4, assign4140_e3898_d_n5, assign4140_e3898_d_n6, assign4140_e3898_d_n7, assign4140_e3898_d_n8, assign4140_e3898_d_n9, assign4140_e3898_d_n10, assign4140_e3898_d_n11,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign4140_e3893: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4140_e3895: f64 = (assign4140_e3893 * locals.var_vtinv);
        let assign4140_e3896: f64 = (assign4140_e3895).exp();
        (assign4140_e3896, (assign4140_e3896 * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (assign4140_e3896 * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign4140_e3893 * locals.var_vtinv_dn4))), (assign4140_e3896 * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (assign4140_e3896 * ((locals.var_vb1e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign4140_e3898;
        locals.var_tmpexp1_dn0 = assign4140_e3898_d_n0;
        locals.var_tmpexp1_dn1 = assign4140_e3898_d_n1;
        locals.var_tmpexp1_dn3 = assign4140_e3898_d_n3;
        locals.var_tmpexp1_dn4 = assign4140_e3898_d_n4;
        locals.var_tmpexp1_dn5 = assign4140_e3898_d_n5;
        locals.var_tmpexp1_dn6 = assign4140_e3898_d_n6;
        locals.var_tmpexp1_dn7 = assign4140_e3898_d_n7;
        locals.var_tmpexp1_dn8 = assign4140_e3898_d_n8;
        locals.var_tmpexp1_dn9 = assign4140_e3898_d_n9;
        locals.var_tmpexp1_dn10 = assign4140_e3898_d_n10;
        locals.var_tmpexp1_dn11 = assign4140_e3898_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let (assign4150_e3906,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 == 0.0)) {
        let assign4150_e3904: f64 = (p.p151).exp();
        (assign4150_e3904,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4150_e3906;
        locals.var_expl_rv = 0.0;

        let (assign4160_e3923, assign4160_e3923_d_n0, assign4160_e3923_d_n1, assign4160_e3923_d_n3, assign4160_e3923_d_n4, assign4160_e3923_d_n5, assign4160_e3923_d_n6, assign4160_e3923_d_n7, assign4160_e3923_d_n8, assign4160_e3923_d_n9, assign4160_e3923_d_n10, assign4160_e3923_d_n11,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 == 0.0)) {
        let assign4160_e3915: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4160_e3917: f64 = (assign4160_e3915 * locals.var_vtinv);
        let assign4160_e3919: f64 = (assign4160_e3917 - p.p151);
        let assign4160_e3920: f64 = (1.0 + assign4160_e3919);
        let assign4160_e3921: f64 = (locals.var_expl * assign4160_e3920);
        (assign4160_e3921, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign4160_e3915 * locals.var_vtinv_dn4))), (locals.var_expl * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign4160_e3923;
        locals.var_tmpexp1_dn0 = assign4160_e3923_d_n0;
        locals.var_tmpexp1_dn1 = assign4160_e3923_d_n1;
        locals.var_tmpexp1_dn3 = assign4160_e3923_d_n3;
        locals.var_tmpexp1_dn4 = assign4160_e3923_d_n4;
        locals.var_tmpexp1_dn5 = assign4160_e3923_d_n5;
        locals.var_tmpexp1_dn6 = assign4160_e3923_d_n6;
        locals.var_tmpexp1_dn7 = assign4160_e3923_d_n7;
        locals.var_tmpexp1_dn8 = assign4160_e3923_d_n8;
        locals.var_tmpexp1_dn9 = assign4160_e3923_d_n9;
        locals.var_tmpexp1_dn10 = assign4160_e3923_d_n10;
        locals.var_tmpexp1_dn11 = assign4160_e3923_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let assign4190_e3960: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4190_e3962: f64 = (assign4190_e3960 / p.p21);
        let assign4190_e3964: f64 = if assign4190_e3962 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4190_e3964;
        locals.var_guard70_rv = 0.0;

        let (assign4200_e3973, assign4200_e3973_d_n0, assign4200_e3973_d_n1, assign4200_e3973_d_n3, assign4200_e3973_d_n4, assign4200_e3973_d_n5, assign4200_e3973_d_n6, assign4200_e3973_d_n7, assign4200_e3973_d_n8, assign4200_e3973_d_n9, assign4200_e3973_d_n10, assign4200_e3973_d_n11,) = {
    if (locals.var_guard70 != 0.0) {
        let assign4200_e3968: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4200_e3970: f64 = (assign4200_e3968 / p.p21);
        let assign4200_e3971: f64 = (assign4200_e3970).exp();
        (assign4200_e3971, 0.0, 0.0, 0.0, (assign4200_e3971 * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p21)), (assign4200_e3971 * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p21)), 0.0, (assign4200_e3971 * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p21)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4200_e3973;
        locals.var_tmpexp_dn0 = assign4200_e3973_d_n0;
        locals.var_tmpexp_dn1 = assign4200_e3973_d_n1;
        locals.var_tmpexp_dn3 = assign4200_e3973_d_n3;
        locals.var_tmpexp_dn4 = assign4200_e3973_d_n4;
        locals.var_tmpexp_dn5 = assign4200_e3973_d_n5;
        locals.var_tmpexp_dn6 = assign4200_e3973_d_n6;
        locals.var_tmpexp_dn7 = assign4200_e3973_d_n7;
        locals.var_tmpexp_dn8 = assign4200_e3973_d_n8;
        locals.var_tmpexp_dn9 = assign4200_e3973_d_n9;
        locals.var_tmpexp_dn10 = assign4200_e3973_d_n10;
        locals.var_tmpexp_dn11 = assign4200_e3973_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4210_e3979,) = {
    if (locals.var_guard70 == 0.0) {
        let assign4210_e3977: f64 = (p.p151).exp();
        (assign4210_e3977,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4210_e3979;
        locals.var_expl_rv = 0.0;

        let (assign4220_e3994, assign4220_e3994_d_n0, assign4220_e3994_d_n1, assign4220_e3994_d_n3, assign4220_e3994_d_n4, assign4220_e3994_d_n5, assign4220_e3994_d_n6, assign4220_e3994_d_n7, assign4220_e3994_d_n8, assign4220_e3994_d_n9, assign4220_e3994_d_n10, assign4220_e3994_d_n11,) = {
    if (locals.var_guard70 == 0.0) {
        let assign4220_e3986: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4220_e3988: f64 = (assign4220_e3986 / p.p21);
        let assign4220_e3990: f64 = (assign4220_e3988 - p.p151);
        let assign4220_e3991: f64 = (1.0 + assign4220_e3990);
        let assign4220_e3992: f64 = (locals.var_expl * assign4220_e3991);
        (assign4220_e3992, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p21)), (locals.var_expl * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p21)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p21)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4220_e3994;
        locals.var_tmpexp_dn0 = assign4220_e3994_d_n0;
        locals.var_tmpexp_dn1 = assign4220_e3994_d_n1;
        locals.var_tmpexp_dn3 = assign4220_e3994_d_n3;
        locals.var_tmpexp_dn4 = assign4220_e3994_d_n4;
        locals.var_tmpexp_dn5 = assign4220_e3994_d_n5;
        locals.var_tmpexp_dn6 = assign4220_e3994_d_n6;
        locals.var_tmpexp_dn7 = assign4220_e3994_d_n7;
        locals.var_tmpexp_dn8 = assign4220_e3994_d_n8;
        locals.var_tmpexp_dn9 = assign4220_e3994_d_n9;
        locals.var_tmpexp_dn10 = assign4220_e3994_d_n10;
        locals.var_tmpexp_dn11 = assign4220_e3994_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4240_e4002: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4240_e4004: f64 = (assign4240_e4002 / p.p23);
        let assign4240_e4006: f64 = if assign4240_e4004 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4240_e4006;
        locals.var_guard71_rv = 0.0;

        let (assign4250_e4015, assign4250_e4015_d_n0, assign4250_e4015_d_n1, assign4250_e4015_d_n3, assign4250_e4015_d_n4, assign4250_e4015_d_n5, assign4250_e4015_d_n6, assign4250_e4015_d_n7, assign4250_e4015_d_n8, assign4250_e4015_d_n9, assign4250_e4015_d_n10, assign4250_e4015_d_n11,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4250_e4010: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4250_e4012: f64 = (assign4250_e4010 / p.p23);
        let assign4250_e4013: f64 = (assign4250_e4012).exp();
        (assign4250_e4013, 0.0, 0.0, 0.0, (assign4250_e4013 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p23)), (assign4250_e4013 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p23)), (assign4250_e4013 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p23)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4250_e4015;
        locals.var_tmpexp_dn0 = assign4250_e4015_d_n0;
        locals.var_tmpexp_dn1 = assign4250_e4015_d_n1;
        locals.var_tmpexp_dn3 = assign4250_e4015_d_n3;
        locals.var_tmpexp_dn4 = assign4250_e4015_d_n4;
        locals.var_tmpexp_dn5 = assign4250_e4015_d_n5;
        locals.var_tmpexp_dn6 = assign4250_e4015_d_n6;
        locals.var_tmpexp_dn7 = assign4250_e4015_d_n7;
        locals.var_tmpexp_dn8 = assign4250_e4015_d_n8;
        locals.var_tmpexp_dn9 = assign4250_e4015_d_n9;
        locals.var_tmpexp_dn10 = assign4250_e4015_d_n10;
        locals.var_tmpexp_dn11 = assign4250_e4015_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4260_e4021,) = {
    if (locals.var_guard71 == 0.0) {
        let assign4260_e4019: f64 = (p.p151).exp();
        (assign4260_e4019,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4260_e4021;
        locals.var_expl_rv = 0.0;

        let (assign4270_e4036, assign4270_e4036_d_n0, assign4270_e4036_d_n1, assign4270_e4036_d_n3, assign4270_e4036_d_n4, assign4270_e4036_d_n5, assign4270_e4036_d_n6, assign4270_e4036_d_n7, assign4270_e4036_d_n8, assign4270_e4036_d_n9, assign4270_e4036_d_n10, assign4270_e4036_d_n11,) = {
    if (locals.var_guard71 == 0.0) {
        let assign4270_e4028: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4270_e4030: f64 = (assign4270_e4028 / p.p23);
        let assign4270_e4032: f64 = (assign4270_e4030 - p.p151);
        let assign4270_e4033: f64 = (1.0 + assign4270_e4032);
        let assign4270_e4034: f64 = (locals.var_expl * assign4270_e4033);
        (assign4270_e4034, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p23)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p23)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p23)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4270_e4036;
        locals.var_tmpexp_dn0 = assign4270_e4036_d_n0;
        locals.var_tmpexp_dn1 = assign4270_e4036_d_n1;
        locals.var_tmpexp_dn3 = assign4270_e4036_d_n3;
        locals.var_tmpexp_dn4 = assign4270_e4036_d_n4;
        locals.var_tmpexp_dn5 = assign4270_e4036_d_n5;
        locals.var_tmpexp_dn6 = assign4270_e4036_d_n6;
        locals.var_tmpexp_dn7 = assign4270_e4036_d_n7;
        locals.var_tmpexp_dn8 = assign4270_e4036_d_n8;
        locals.var_tmpexp_dn9 = assign4270_e4036_d_n9;
        locals.var_tmpexp_dn10 = assign4270_e4036_d_n10;
        locals.var_tmpexp_dn11 = assign4270_e4036_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4290_e4044: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4290_e4046: f64 = (assign4290_e4044 / p.p32);
        let assign4290_e4048: f64 = if assign4290_e4046 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4290_e4048;
        locals.var_guard72_rv = 0.0;

        let (assign4300_e4057, assign4300_e4057_d_n0, assign4300_e4057_d_n1, assign4300_e4057_d_n3, assign4300_e4057_d_n4, assign4300_e4057_d_n5, assign4300_e4057_d_n6, assign4300_e4057_d_n7, assign4300_e4057_d_n8, assign4300_e4057_d_n9, assign4300_e4057_d_n10, assign4300_e4057_d_n11,) = {
    if (locals.var_guard72 != 0.0) {
        let assign4300_e4052: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4300_e4054: f64 = (assign4300_e4052 / p.p32);
        let assign4300_e4055: f64 = (assign4300_e4054).exp();
        (assign4300_e4055, 0.0, 0.0, 0.0, (assign4300_e4055 * ((locals.var_vb1c4 * locals.var_vtinv_dn4) / p.p32)), 0.0, (assign4300_e4055 * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p32)), 0.0, (assign4300_e4055 * ((locals.var_vb1c4_dn11 * locals.var_vtinv) / p.p32)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4300_e4057;
        locals.var_tmpexp_dn0 = assign4300_e4057_d_n0;
        locals.var_tmpexp_dn1 = assign4300_e4057_d_n1;
        locals.var_tmpexp_dn3 = assign4300_e4057_d_n3;
        locals.var_tmpexp_dn4 = assign4300_e4057_d_n4;
        locals.var_tmpexp_dn5 = assign4300_e4057_d_n5;
        locals.var_tmpexp_dn6 = assign4300_e4057_d_n6;
        locals.var_tmpexp_dn7 = assign4300_e4057_d_n7;
        locals.var_tmpexp_dn8 = assign4300_e4057_d_n8;
        locals.var_tmpexp_dn9 = assign4300_e4057_d_n9;
        locals.var_tmpexp_dn10 = assign4300_e4057_d_n10;
        locals.var_tmpexp_dn11 = assign4300_e4057_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4310_e4063,) = {
    if (locals.var_guard72 == 0.0) {
        let assign4310_e4061: f64 = (p.p151).exp();
        (assign4310_e4061,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4310_e4063;
        locals.var_expl_rv = 0.0;

        let (assign4320_e4078, assign4320_e4078_d_n0, assign4320_e4078_d_n1, assign4320_e4078_d_n3, assign4320_e4078_d_n4, assign4320_e4078_d_n5, assign4320_e4078_d_n6, assign4320_e4078_d_n7, assign4320_e4078_d_n8, assign4320_e4078_d_n9, assign4320_e4078_d_n10, assign4320_e4078_d_n11,) = {
    if (locals.var_guard72 == 0.0) {
        let assign4320_e4070: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4320_e4072: f64 = (assign4320_e4070 / p.p32);
        let assign4320_e4074: f64 = (assign4320_e4072 - p.p151);
        let assign4320_e4075: f64 = (1.0 + assign4320_e4074);
        let assign4320_e4076: f64 = (locals.var_expl * assign4320_e4075);
        (assign4320_e4076, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1c4 * locals.var_vtinv_dn4) / p.p32)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p32)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn11 * locals.var_vtinv) / p.p32)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4320_e4078;
        locals.var_tmpexp_dn0 = assign4320_e4078_d_n0;
        locals.var_tmpexp_dn1 = assign4320_e4078_d_n1;
        locals.var_tmpexp_dn3 = assign4320_e4078_d_n3;
        locals.var_tmpexp_dn4 = assign4320_e4078_d_n4;
        locals.var_tmpexp_dn5 = assign4320_e4078_d_n5;
        locals.var_tmpexp_dn6 = assign4320_e4078_d_n6;
        locals.var_tmpexp_dn7 = assign4320_e4078_d_n7;
        locals.var_tmpexp_dn8 = assign4320_e4078_d_n8;
        locals.var_tmpexp_dn9 = assign4320_e4078_d_n9;
        locals.var_tmpexp_dn10 = assign4320_e4078_d_n10;
        locals.var_tmpexp_dn11 = assign4320_e4078_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4340_e4086: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4340_e4088: f64 = (assign4340_e4086 / p.p150);
        let assign4340_e4090: f64 = if assign4340_e4088 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4340_e4090;
        locals.var_guard73_rv = 0.0;

        let (assign4350_e4099, assign4350_e4099_d_n0, assign4350_e4099_d_n1, assign4350_e4099_d_n3, assign4350_e4099_d_n4, assign4350_e4099_d_n5, assign4350_e4099_d_n6, assign4350_e4099_d_n7, assign4350_e4099_d_n8, assign4350_e4099_d_n9, assign4350_e4099_d_n10, assign4350_e4099_d_n11,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4350_e4094: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4350_e4096: f64 = (assign4350_e4094 / p.p150);
        let assign4350_e4097: f64 = (assign4350_e4096).exp();
        (assign4350_e4097, 0.0, 0.0, 0.0, (assign4350_e4097 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p150)), (assign4350_e4097 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p150)), (assign4350_e4097 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p150)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4350_e4099;
        locals.var_tmpexp_dn0 = assign4350_e4099_d_n0;
        locals.var_tmpexp_dn1 = assign4350_e4099_d_n1;
        locals.var_tmpexp_dn3 = assign4350_e4099_d_n3;
        locals.var_tmpexp_dn4 = assign4350_e4099_d_n4;
        locals.var_tmpexp_dn5 = assign4350_e4099_d_n5;
        locals.var_tmpexp_dn6 = assign4350_e4099_d_n6;
        locals.var_tmpexp_dn7 = assign4350_e4099_d_n7;
        locals.var_tmpexp_dn8 = assign4350_e4099_d_n8;
        locals.var_tmpexp_dn9 = assign4350_e4099_d_n9;
        locals.var_tmpexp_dn10 = assign4350_e4099_d_n10;
        locals.var_tmpexp_dn11 = assign4350_e4099_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4360_e4105,) = {
    if (locals.var_guard73 == 0.0) {
        let assign4360_e4103: f64 = (p.p151).exp();
        (assign4360_e4103,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4360_e4105;
        locals.var_expl_rv = 0.0;

        let (assign4370_e4120, assign4370_e4120_d_n0, assign4370_e4120_d_n1, assign4370_e4120_d_n3, assign4370_e4120_d_n4, assign4370_e4120_d_n5, assign4370_e4120_d_n6, assign4370_e4120_d_n7, assign4370_e4120_d_n8, assign4370_e4120_d_n9, assign4370_e4120_d_n10, assign4370_e4120_d_n11,) = {
    if (locals.var_guard73 == 0.0) {
        let assign4370_e4112: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4370_e4114: f64 = (assign4370_e4112 / p.p150);
        let assign4370_e4116: f64 = (assign4370_e4114 - p.p151);
        let assign4370_e4117: f64 = (1.0 + assign4370_e4116);
        let assign4370_e4118: f64 = (locals.var_expl * assign4370_e4117);
        (assign4370_e4118, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p150)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p150)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p150)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4370_e4120;
        locals.var_tmpexp_dn0 = assign4370_e4120_d_n0;
        locals.var_tmpexp_dn1 = assign4370_e4120_d_n1;
        locals.var_tmpexp_dn3 = assign4370_e4120_d_n3;
        locals.var_tmpexp_dn4 = assign4370_e4120_d_n4;
        locals.var_tmpexp_dn5 = assign4370_e4120_d_n5;
        locals.var_tmpexp_dn6 = assign4370_e4120_d_n6;
        locals.var_tmpexp_dn7 = assign4370_e4120_d_n7;
        locals.var_tmpexp_dn8 = assign4370_e4120_d_n8;
        locals.var_tmpexp_dn9 = assign4370_e4120_d_n9;
        locals.var_tmpexp_dn10 = assign4370_e4120_d_n10;
        locals.var_tmpexp_dn11 = assign4370_e4120_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4390_e4136: f64 = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (locals.var_vb2e1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4390_e4136;
        locals.var_guard74_rv = 0.0;

        let assign4400_e4142: f64 = (2.0 * locals.var_e0eb);
        let assign4400_e4143: f64 = (locals.var_pow2_2m_pe / assign4400_e4142);
        let assign4400_e4144: f64 = (1.0 - assign4400_e4143);
        let assign4400_e4145: f64 = (locals.var_nzeb_t * assign4400_e4144);
        let assign4400_e4147: f64 = if assign4400_e4145 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4400_e4147;
        locals.var_guard75_rv = 0.0;

        let (assign4420_e4170,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        let assign4420_e4168: f64 = (p.p151).exp();
        (assign4420_e4168,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4420_e4170;
        locals.var_expl_rv = 0.0;

        let (assign4440_e4197, assign4440_e4197_d_n0, assign4440_e4197_d_n1, assign4440_e4197_d_n3, assign4440_e4197_d_n4, assign4440_e4197_d_n5, assign4440_e4197_d_n6, assign4440_e4197_d_n7, assign4440_e4197_d_n8, assign4440_e4197_d_n9, assign4440_e4197_d_n10, assign4440_e4197_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4440_e4195: f64 = (locals.var_vb2e1 * locals.var_inv_vde_t);
        (assign4440_e4195, (locals.var_vb2e1 * locals.var_inv_vde_t_dn0), (locals.var_vb2e1 * locals.var_inv_vde_t_dn1), (locals.var_vb2e1 * locals.var_inv_vde_t_dn3), (locals.var_vb2e1 * locals.var_inv_vde_t_dn4), ((locals.var_vb2e1_dn5 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn5)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn6), ((locals.var_vb2e1_dn7 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn7)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn8), (locals.var_vb2e1 * locals.var_inv_vde_t_dn9), (locals.var_vb2e1 * locals.var_inv_vde_t_dn10), (locals.var_vb2e1 * locals.var_inv_vde_t_dn11),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10, locals.var_x_dn11,)
    }
};
        locals.var_x = assign4440_e4197;
        locals.var_x_dn0 = assign4440_e4197_d_n0;
        locals.var_x_dn1 = assign4440_e4197_d_n1;
        locals.var_x_dn3 = assign4440_e4197_d_n3;
        locals.var_x_dn4 = assign4440_e4197_d_n4;
        locals.var_x_dn5 = assign4440_e4197_d_n5;
        locals.var_x_dn6 = assign4440_e4197_d_n6;
        locals.var_x_dn7 = assign4440_e4197_d_n7;
        locals.var_x_dn8 = assign4440_e4197_d_n8;
        locals.var_x_dn9 = assign4440_e4197_d_n9;
        locals.var_x_dn10 = assign4440_e4197_d_n10;
        locals.var_x_dn11 = assign4440_e4197_d_n11;
        locals.var_x_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4450_e4241, assign4450_e4241_d_n0, assign4450_e4241_d_n1, assign4450_e4241_d_n3, assign4450_e4241_d_n4, assign4450_e4241_d_n5, assign4450_e4241_d_n6, assign4450_e4241_d_n7, assign4450_e4241_d_n8, assign4450_e4241_d_n9, assign4450_e4241_d_n10, assign4450_e4241_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4450_e4201: f64 = (locals.var_x * locals.var_x);
        let assign4450_e4203: f64 = (assign4450_e4201 + 1e-30);
        let assign4450_e4204: f64 = (assign4450_e4203).sqrt();
        let assign4450_e4206: f64 = (-2.0);
        let assign4450_e4208: f64 = (assign4450_e4206 - p.p67);
        let assign4450_e4209: f64 = (assign4450_e4204).powf(assign4450_e4208);
        let assign4450_e4214: f64 = (p.p67 * p.p67);
        let assign4450_e4215: f64 = (1.0 - assign4450_e4214);
        let assign4450_e4218: f64 = (3.0 * locals.var_x);
        let assign4450_e4221: f64 = (p.p67 - 1.0);
        let assign4450_e4222: f64 = (assign4450_e4218 * assign4450_e4221);
        let assign4450_e4223: f64 = (assign4450_e4215 - assign4450_e4222);
        let assign4450_e4224: f64 = (p.p67 * assign4450_e4223);
        let assign4450_e4227: f64 = (6.0 * locals.var_x);
        let assign4450_e4229: f64 = (assign4450_e4227 * locals.var_x);
        let assign4450_e4232: f64 = (p.p67 - 1.0);
        let assign4450_e4234: f64 = (assign4450_e4232 + locals.var_x);
        let assign4450_e4235: f64 = (assign4450_e4229 * assign4450_e4234);
        let assign4450_e4236: f64 = (assign4450_e4224 - assign4450_e4235);
        let assign4450_e4237: f64 = (assign4450_e4209 * assign4450_e4236);
        let assign4450_e4239: f64 = (assign4450_e4237 * 0.16666666666666666);
        (assign4450_e4239, (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn0) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn0) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn0)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn1) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn1) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn1)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn3) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn3) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn3)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn4) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn4) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn4)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn5) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn5) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn5)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn6) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn6) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn6)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn7) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn7) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn7)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn8) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn8) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn8)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn9) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn9) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn9)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn10) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn10) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn10)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn10))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn11 * locals.var_x) + (locals.var_x * locals.var_x_dn11)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn11 * locals.var_x) + (locals.var_x * locals.var_x_dn11)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn11) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn11) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn11)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn11))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0eb, locals.var_de0eb_dn0, locals.var_de0eb_dn1, locals.var_de0eb_dn3, locals.var_de0eb_dn4, locals.var_de0eb_dn5, locals.var_de0eb_dn6, locals.var_de0eb_dn7, locals.var_de0eb_dn8, locals.var_de0eb_dn9, locals.var_de0eb_dn10, locals.var_de0eb_dn11,)
    }
};
        locals.var_de0eb = assign4450_e4241;
        locals.var_de0eb_dn0 = assign4450_e4241_d_n0;
        locals.var_de0eb_dn1 = assign4450_e4241_d_n1;
        locals.var_de0eb_dn3 = assign4450_e4241_d_n3;
        locals.var_de0eb_dn4 = assign4450_e4241_d_n4;
        locals.var_de0eb_dn5 = assign4450_e4241_d_n5;
        locals.var_de0eb_dn6 = assign4450_e4241_d_n6;
        locals.var_de0eb_dn7 = assign4450_e4241_d_n7;
        locals.var_de0eb_dn8 = assign4450_e4241_d_n8;
        locals.var_de0eb_dn9 = assign4450_e4241_d_n9;
        locals.var_de0eb_dn10 = assign4450_e4241_d_n10;
        locals.var_de0eb_dn11 = assign4450_e4241_d_n11;
        locals.var_de0eb_rv = 0.0;

        let (assign4460_e4253, assign4460_e4253_d_n0, assign4460_e4253_d_n1, assign4460_e4253_d_n3, assign4460_e4253_d_n4, assign4460_e4253_d_n5, assign4460_e4253_d_n6, assign4460_e4253_d_n7, assign4460_e4253_d_n8, assign4460_e4253_d_n9, assign4460_e4253_d_n10, assign4460_e4253_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4460_e4245: f64 = (locals.var_vb2e1 * locals.var_pow2_2m_pe);
        let assign4460_e4247: f64 = (assign4460_e4245 * locals.var_nzeb_t);
        let assign4460_e4250: f64 = (locals.var_vgzeb_t * locals.var_de0eb);
        let assign4460_e4251: f64 = (assign4460_e4247 / assign4460_e4250);
        (assign4460_e4251, ((((assign4460_e4245 * locals.var_nzeb_t_dn0) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn0 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn0)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn1) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn1 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn1)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn3) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn3 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn3)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn4) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn4 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn4)))) / (assign4460_e4250 * assign4460_e4250)), ((((((locals.var_vb2e1_dn5 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4460_e4245 * locals.var_nzeb_t_dn5)) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn5 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn5)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn6) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn6 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn6)))) / (assign4460_e4250 * assign4460_e4250)), ((((((locals.var_vb2e1_dn7 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4460_e4245 * locals.var_nzeb_t_dn7)) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn7 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn7)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn8) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn8 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn8)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn9) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn9 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn9)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn10) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn10 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn10)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn11) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn11 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn11)))) / (assign4460_e4250 * assign4460_e4250)),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10, locals.var_x_dn11,)
    }
};
        locals.var_x = assign4460_e4253;
        locals.var_x_dn0 = assign4460_e4253_d_n0;
        locals.var_x_dn1 = assign4460_e4253_d_n1;
        locals.var_x_dn3 = assign4460_e4253_d_n3;
        locals.var_x_dn4 = assign4460_e4253_d_n4;
        locals.var_x_dn5 = assign4460_e4253_d_n5;
        locals.var_x_dn6 = assign4460_e4253_d_n6;
        locals.var_x_dn7 = assign4460_e4253_d_n7;
        locals.var_x_dn8 = assign4460_e4253_d_n8;
        locals.var_x_dn9 = assign4460_e4253_d_n9;
        locals.var_x_dn10 = assign4460_e4253_d_n10;
        locals.var_x_dn11 = assign4460_e4253_d_n11;
        locals.var_x_rv = 0.0;

        let assign4470_e4256: f64 = (-0.001);
        let assign4470_e4257: f64 = if locals.var_x < assign4470_e4256 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign4470_e4257;
        locals.var_guard76_rv = 0.0;

        let assign4480_e4260: f64 = if locals.var_x < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard77 = assign4480_e4260;
        locals.var_guard77_rv = 0.0;

        let (assign4500_e4279,) = {
    if (((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign4500_e4277: f64 = (p.p151).exp();
        (assign4500_e4277,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4500_e4279;
        locals.var_expl_rv = 0.0;

        let assign4570_e4369: f64 = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (locals.var_vb2c1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign4570_e4369;
        locals.var_guard78_rv = 0.0;

        let (assign4580_e4381, assign4580_e4381_d_n0, assign4580_e4381_d_n1, assign4580_e4381_d_n3, assign4580_e4381_d_n4, assign4580_e4381_d_n5, assign4580_e4381_d_n6, assign4580_e4381_d_n7, assign4580_e4381_d_n8, assign4580_e4381_d_n9, assign4580_e4381_d_n10, assign4580_e4381_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4580_e4374: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        let assign4580_e4375: f64 = (1.0 - assign4580_e4374);
        let assign4580_e4378: f64 = (1.0 - locals.var_pc_zener);
        let assign4580_e4379: f64 = (assign4580_e4375).powf(assign4580_e4378);
        (assign4580_e4379, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8))))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8))) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11)) / assign4580_e4375))) },)
    } else {
        (locals.var_e0cb, locals.var_e0cb_dn0, locals.var_e0cb_dn1, locals.var_e0cb_dn3, locals.var_e0cb_dn4, locals.var_e0cb_dn5, locals.var_e0cb_dn6, locals.var_e0cb_dn7, locals.var_e0cb_dn8, locals.var_e0cb_dn9, locals.var_e0cb_dn10, locals.var_e0cb_dn11,)
    }
};
        locals.var_e0cb = assign4580_e4381;
        locals.var_e0cb_dn0 = assign4580_e4381_d_n0;
        locals.var_e0cb_dn1 = assign4580_e4381_d_n1;
        locals.var_e0cb_dn3 = assign4580_e4381_d_n3;
        locals.var_e0cb_dn4 = assign4580_e4381_d_n4;
        locals.var_e0cb_dn5 = assign4580_e4381_d_n5;
        locals.var_e0cb_dn6 = assign4580_e4381_d_n6;
        locals.var_e0cb_dn7 = assign4580_e4381_d_n7;
        locals.var_e0cb_dn8 = assign4580_e4381_d_n8;
        locals.var_e0cb_dn9 = assign4580_e4381_d_n9;
        locals.var_e0cb_dn10 = assign4580_e4381_d_n10;
        locals.var_e0cb_dn11 = assign4580_e4381_d_n11;
        locals.var_e0cb_rv = 0.0;

        let assign4590_e4387: f64 = (2.0 * locals.var_e0cb);
        let assign4590_e4388: f64 = (locals.var_pow2_2m_pc / assign4590_e4387);
        let assign4590_e4389: f64 = (1.0 - assign4590_e4388);
        let assign4590_e4390: f64 = (locals.var_nzcb_t * assign4590_e4389);
        let assign4590_e4392: f64 = if assign4590_e4390 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign4590_e4392;
        locals.var_guard79_rv = 0.0;

        let (assign4610_e4415,) = {
    if ((locals.var_guard78 != 0.0) && (locals.var_guard79 == 0.0)) {
        let assign4610_e4413: f64 = (p.p151).exp();
        (assign4610_e4413,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4610_e4415;
        locals.var_expl_rv = 0.0;

        let (assign4630_e4442, assign4630_e4442_d_n0, assign4630_e4442_d_n1, assign4630_e4442_d_n3, assign4630_e4442_d_n4, assign4630_e4442_d_n5, assign4630_e4442_d_n6, assign4630_e4442_d_n7, assign4630_e4442_d_n8, assign4630_e4442_d_n9, assign4630_e4442_d_n10, assign4630_e4442_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4630_e4440: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        (assign4630_e4440, (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6), ((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7)), ((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8)), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10, locals.var_xx_dn11,)
    }
};
        locals.var_xx = assign4630_e4442;
        locals.var_xx_dn0 = assign4630_e4442_d_n0;
        locals.var_xx_dn1 = assign4630_e4442_d_n1;
        locals.var_xx_dn3 = assign4630_e4442_d_n3;
        locals.var_xx_dn4 = assign4630_e4442_d_n4;
        locals.var_xx_dn5 = assign4630_e4442_d_n5;
        locals.var_xx_dn6 = assign4630_e4442_d_n6;
        locals.var_xx_dn7 = assign4630_e4442_d_n7;
        locals.var_xx_dn8 = assign4630_e4442_d_n8;
        locals.var_xx_dn9 = assign4630_e4442_d_n9;
        locals.var_xx_dn10 = assign4630_e4442_d_n10;
        locals.var_xx_dn11 = assign4630_e4442_d_n11;
        locals.var_xx_rv = 0.0;

        let (assign4640_e4486, assign4640_e4486_d_n0, assign4640_e4486_d_n1, assign4640_e4486_d_n3, assign4640_e4486_d_n4, assign4640_e4486_d_n5, assign4640_e4486_d_n6, assign4640_e4486_d_n7, assign4640_e4486_d_n8, assign4640_e4486_d_n9, assign4640_e4486_d_n10, assign4640_e4486_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4640_e4446: f64 = (locals.var_xx * locals.var_xx);
        let assign4640_e4448: f64 = (assign4640_e4446 + 1e-30);
        let assign4640_e4449: f64 = (assign4640_e4448).sqrt();
        let assign4640_e4451: f64 = (-2.0);
        let assign4640_e4453: f64 = (assign4640_e4451 - locals.var_pc_zener);
        let assign4640_e4454: f64 = (assign4640_e4449).powf(assign4640_e4453);
        let assign4640_e4459: f64 = (locals.var_pc_zener * locals.var_pc_zener);
        let assign4640_e4460: f64 = (1.0 - assign4640_e4459);
        let assign4640_e4463: f64 = (3.0 * locals.var_xx);
        let assign4640_e4466: f64 = (locals.var_pc_zener - 1.0);
        let assign4640_e4467: f64 = (assign4640_e4463 * assign4640_e4466);
        let assign4640_e4468: f64 = (assign4640_e4460 - assign4640_e4467);
        let assign4640_e4469: f64 = (locals.var_pc_zener * assign4640_e4468);
        let assign4640_e4472: f64 = (6.0 * locals.var_xx);
        let assign4640_e4474: f64 = (assign4640_e4472 * locals.var_xx);
        let assign4640_e4477: f64 = (locals.var_pc_zener - 1.0);
        let assign4640_e4479: f64 = (assign4640_e4477 + locals.var_xx);
        let assign4640_e4480: f64 = (assign4640_e4474 * assign4640_e4479);
        let assign4640_e4481: f64 = (assign4640_e4469 - assign4640_e4480);
        let assign4640_e4482: f64 = (assign4640_e4454 * assign4640_e4481);
        let assign4640_e4484: f64 = (assign4640_e4482 * 0.16666666666666666);
        (assign4640_e4484, (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn0) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn0) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn0)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn1) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn1) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn1)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn3) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn3) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn3)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn4) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn4) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn4)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn5) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn5) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn5)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn6) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn6) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn6)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn7) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn7) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn7)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn8) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn8) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn8)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn9) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn9) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn9)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn10) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn10) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn10)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn10))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn11 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn11)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn11 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn11)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn11) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn11) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn11)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn11))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0cb, locals.var_de0cb_dn0, locals.var_de0cb_dn1, locals.var_de0cb_dn3, locals.var_de0cb_dn4, locals.var_de0cb_dn5, locals.var_de0cb_dn6, locals.var_de0cb_dn7, locals.var_de0cb_dn8, locals.var_de0cb_dn9, locals.var_de0cb_dn10, locals.var_de0cb_dn11,)
    }
};
        locals.var_de0cb = assign4640_e4486;
        locals.var_de0cb_dn0 = assign4640_e4486_d_n0;
        locals.var_de0cb_dn1 = assign4640_e4486_d_n1;
        locals.var_de0cb_dn3 = assign4640_e4486_d_n3;
        locals.var_de0cb_dn4 = assign4640_e4486_d_n4;
        locals.var_de0cb_dn5 = assign4640_e4486_d_n5;
        locals.var_de0cb_dn6 = assign4640_e4486_d_n6;
        locals.var_de0cb_dn7 = assign4640_e4486_d_n7;
        locals.var_de0cb_dn8 = assign4640_e4486_d_n8;
        locals.var_de0cb_dn9 = assign4640_e4486_d_n9;
        locals.var_de0cb_dn10 = assign4640_e4486_d_n10;
        locals.var_de0cb_dn11 = assign4640_e4486_d_n11;
        locals.var_de0cb_rv = 0.0;

        let (assign4650_e4498, assign4650_e4498_d_n0, assign4650_e4498_d_n1, assign4650_e4498_d_n3, assign4650_e4498_d_n4, assign4650_e4498_d_n5, assign4650_e4498_d_n6, assign4650_e4498_d_n7, assign4650_e4498_d_n8, assign4650_e4498_d_n9, assign4650_e4498_d_n10, assign4650_e4498_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4650_e4490: f64 = (locals.var_vb2c1 * locals.var_pow2_2m_pc);
        let assign4650_e4492: f64 = (assign4650_e4490 * locals.var_nzcb_t);
        let assign4650_e4495: f64 = (locals.var_vgzcb_t * locals.var_de0cb);
        let assign4650_e4496: f64 = (assign4650_e4492 / assign4650_e4495);
        (assign4650_e4496, ((((assign4650_e4490 * locals.var_nzcb_t_dn0) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn0 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn0)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn1) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn1 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn1)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn3) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn3 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn3)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn4) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn4 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn4)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn5) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn5 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn5)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn6) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn6 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn6)))) / (assign4650_e4495 * assign4650_e4495)), ((((((locals.var_vb2c1_dn7 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4650_e4490 * locals.var_nzcb_t_dn7)) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn7 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn7)))) / (assign4650_e4495 * assign4650_e4495)), ((((((locals.var_vb2c1_dn8 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4650_e4490 * locals.var_nzcb_t_dn8)) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn8 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn8)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn9) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn9 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn9)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn10) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn10 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn10)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn11) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn11 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn11)))) / (assign4650_e4495 * assign4650_e4495)),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10, locals.var_xx_dn11,)
    }
};
        locals.var_xx = assign4650_e4498;
        locals.var_xx_dn0 = assign4650_e4498_d_n0;
        locals.var_xx_dn1 = assign4650_e4498_d_n1;
        locals.var_xx_dn3 = assign4650_e4498_d_n3;
        locals.var_xx_dn4 = assign4650_e4498_d_n4;
        locals.var_xx_dn5 = assign4650_e4498_d_n5;
        locals.var_xx_dn6 = assign4650_e4498_d_n6;
        locals.var_xx_dn7 = assign4650_e4498_d_n7;
        locals.var_xx_dn8 = assign4650_e4498_d_n8;
        locals.var_xx_dn9 = assign4650_e4498_d_n9;
        locals.var_xx_dn10 = assign4650_e4498_d_n10;
        locals.var_xx_dn11 = assign4650_e4498_d_n11;
        locals.var_xx_rv = 0.0;

        let assign4660_e4501: f64 = (-0.001);
        let assign4660_e4502: f64 = if locals.var_xx < assign4660_e4501 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign4660_e4502;
        locals.var_guard80_rv = 0.0;

        let assign4670_e4505: f64 = if locals.var_xx < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign4670_e4505;
        locals.var_guard81_rv = 0.0;

        let (assign4690_e4524,) = {
    if (((locals.var_guard78 != 0.0) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) {
        let assign4690_e4522: f64 = (p.p151).exp();
        (assign4690_e4522,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4690_e4524;
        locals.var_expl_rv = 0.0;

        let assign4760_e4606: f64 = (locals.var_if0 * locals.var_evb1c4);
        locals.var_g1 = assign4760_e4606;
        locals.var_g1_dn0 = (locals.var_if0_dn0 * locals.var_evb1c4);
        locals.var_g1_dn1 = (locals.var_if0_dn1 * locals.var_evb1c4);
        locals.var_g1_dn3 = (locals.var_if0_dn3 * locals.var_evb1c4);
        locals.var_g1_dn4 = ((locals.var_if0_dn4 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn4));
        locals.var_g1_dn5 = (locals.var_if0_dn5 * locals.var_evb1c4);
        locals.var_g1_dn6 = ((locals.var_if0_dn6 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn6));
        locals.var_g1_dn7 = ((locals.var_if0_dn7 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn7));
        locals.var_g1_dn8 = ((locals.var_if0_dn8 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn8));
        locals.var_g1_dn9 = ((locals.var_if0_dn9 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn9));
        locals.var_g1_dn10 = (locals.var_if0_dn10 * locals.var_evb1c4);
        locals.var_g1_dn11 = ((locals.var_if0_dn11 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn11));
        locals.var_g1_rv = 0.0;

        let assign4770_e4609: f64 = (4.0 * locals.var_evb1c4vdc);
        locals.var_g2 = assign4770_e4609;
        locals.var_g2_dn0 = (4.0 * locals.var_evb1c4vdc_dn0);
        locals.var_g2_dn1 = (4.0 * locals.var_evb1c4vdc_dn1);
        locals.var_g2_dn3 = (4.0 * locals.var_evb1c4vdc_dn3);
        locals.var_g2_dn4 = (4.0 * locals.var_evb1c4vdc_dn4);
        locals.var_g2_dn5 = (4.0 * locals.var_evb1c4vdc_dn5);
        locals.var_g2_dn6 = (4.0 * locals.var_evb1c4vdc_dn6);
        locals.var_g2_dn7 = (4.0 * locals.var_evb1c4vdc_dn7);
        locals.var_g2_dn8 = (4.0 * locals.var_evb1c4vdc_dn8);
        locals.var_g2_dn9 = (4.0 * locals.var_evb1c4vdc_dn9);
        locals.var_g2_dn10 = (4.0 * locals.var_evb1c4vdc_dn10);
        locals.var_g2_dn11 = (4.0 * locals.var_evb1c4vdc_dn11);
        locals.var_g2_rv = 0.0;

        let assign4780_e4612: f64 = (locals.var_g1 - locals.var_if0);
        let assign4780_e4616: f64 = (1.0 + locals.var_g1);
        let assign4780_e4617: f64 = (assign4780_e4616).sqrt();
        let assign4780_e4618: f64 = (1.0 + assign4780_e4617);
        let assign4780_e4619: f64 = (assign4780_e4612 / assign4780_e4618);
        locals.var_nbex = assign4780_e4619;
        locals.var_nbex_dn0 = ((((locals.var_g1_dn0 - locals.var_if0_dn0) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn0 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn1 = ((((locals.var_g1_dn1 - locals.var_if0_dn1) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn1 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn3 = ((((locals.var_g1_dn3 - locals.var_if0_dn3) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn3 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn4 = ((((locals.var_g1_dn4 - locals.var_if0_dn4) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn4 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn5 = ((((locals.var_g1_dn5 - locals.var_if0_dn5) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn5 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn6 = ((((locals.var_g1_dn6 - locals.var_if0_dn6) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn6 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn7 = ((((locals.var_g1_dn7 - locals.var_if0_dn7) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn7 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn8 = ((((locals.var_g1_dn8 - locals.var_if0_dn8) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn8 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn9 = ((((locals.var_g1_dn9 - locals.var_if0_dn9) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn9 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn10 = ((((locals.var_g1_dn10 - locals.var_if0_dn10) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn10 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn11 = ((((locals.var_g1_dn11 - locals.var_if0_dn11) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn11 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_rv = 0.0;

        let assign4790_e4624: f64 = (1.0 + locals.var_g2);
        let assign4790_e4625: f64 = (assign4790_e4624).sqrt();
        let assign4790_e4626: f64 = (1.0 + assign4790_e4625);
        let assign4790_e4627: f64 = (locals.var_g2 / assign4790_e4626);
        locals.var_pwex = assign4790_e4627;
        locals.var_pwex_dn0 = (((locals.var_g2_dn0 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn0 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn1 = (((locals.var_g2_dn1 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn1 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn3 = (((locals.var_g2_dn3 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn3 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn4 = (((locals.var_g2_dn4 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn4 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn5 = (((locals.var_g2_dn5 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn5 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn6 = (((locals.var_g2_dn6 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn6 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn7 = (((locals.var_g2_dn7 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn7 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn8 = (((locals.var_g2_dn8 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn8 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn9 = (((locals.var_g2_dn9 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn9 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn10 = (((locals.var_g2_dn10 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn10 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn11 = (((locals.var_g2_dn11 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn11 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_rv = 0.0;

        let assign4880_e4798: f64 = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign4880_e4798;
        locals.var_guard83_rv = 0.0;

        let (assign4910_e4835, assign4910_e4835_d_n0, assign4910_e4835_d_n1, assign4910_e4835_d_n4, assign4910_e4835_d_n6, assign4910_e4835_d_n7, assign4910_e4835_d_n8, assign4910_e4835_d_n9, assign4910_e4835_d_n10, assign4910_e4835_d_n11,) = {
    if (locals.var_guard83 != 0.0) {
        let assign4910_e4814: f64 = (p.p33 * 2.0);
        let assign4910_e4816: f64 = (assign4910_e4814 * locals.var_ibx_t);
        let assign4910_e4819: f64 = (locals.var_evbc3 - 1.0);
        let assign4910_e4820: f64 = (assign4910_e4816 * assign4910_e4819);
        let assign4910_e4825: f64 = (4.0 * locals.var_ibx_t);
        let assign4910_e4827: f64 = (assign4910_e4825 / locals.var_ikbx_t);
        let assign4910_e4829: f64 = (assign4910_e4827 * locals.var_evbc3);
        let assign4910_e4830: f64 = (1.0 + assign4910_e4829);
        let assign4910_e4831: f64 = (assign4910_e4830).sqrt();
        let assign4910_e4832: f64 = (1.0 + assign4910_e4831);
        let assign4910_e4833: f64 = (assign4910_e4820 / assign4910_e4832);
        (assign4910_e4833, ((((assign4910_e4816 * locals.var_evbc3_dn0) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn0) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn1) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn1) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((((assign4910_e4814 * locals.var_ibx_t_dn4) * assign4910_e4819) + (assign4910_e4816 * locals.var_evbc3_dn4)) * assign4910_e4832) - (assign4910_e4820 * (((((((4.0 * locals.var_ibx_t_dn4) * locals.var_ikbx_t) - (assign4910_e4825 * locals.var_ikbx_t_dn4)) / (locals.var_ikbx_t * locals.var_ikbx_t)) * locals.var_evbc3) + (assign4910_e4827 * locals.var_evbc3_dn4)) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn6) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn6) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn7) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn7) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn8) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn8) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn9) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn9) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn10) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn10) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn11) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn11) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)),)
    } else {
        (locals.var_ximex, locals.var_ximex_dn0, locals.var_ximex_dn1, locals.var_ximex_dn4, locals.var_ximex_dn6, locals.var_ximex_dn7, locals.var_ximex_dn8, locals.var_ximex_dn9, locals.var_ximex_dn10, locals.var_ximex_dn11,)
    }
};
        locals.var_ximex = assign4910_e4835;
        locals.var_ximex_dn0 = assign4910_e4835_d_n0;
        locals.var_ximex_dn1 = assign4910_e4835_d_n1;
        locals.var_ximex_dn4 = assign4910_e4835_d_n4;
        locals.var_ximex_dn6 = assign4910_e4835_d_n6;
        locals.var_ximex_dn7 = assign4910_e4835_d_n7;
        locals.var_ximex_dn8 = assign4910_e4835_d_n8;
        locals.var_ximex_dn9 = assign4910_e4835_d_n9;
        locals.var_ximex_dn10 = assign4910_e4835_d_n10;
        locals.var_ximex_dn11 = assign4910_e4835_d_n11;
        locals.var_ximex_rv = 0.0;

        let assign4920_e4838: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign4920_e4838;
        locals.var_guard84_rv = 0.0;

        let (assign4930_e4873, assign4930_e4873_d_n0, assign4930_e4873_d_n1, assign4930_e4873_d_n3, assign4930_e4873_d_n4, assign4930_e4873_d_n6, assign4930_e4873_d_n7, assign4930_e4873_d_n8, assign4930_e4873_d_n9, assign4930_e4873_d_n10, assign4930_e4873_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign4930_e4844: f64 = (1.0 - p.p143);
        let assign4930_e4846: f64 = (assign4930_e4844 * p.p33);
        let assign4930_e4848: f64 = (assign4930_e4846 * 2.0);
        let assign4930_e4850: f64 = (assign4930_e4848 * locals.var_iss_t);
        let assign4930_e4853: f64 = (locals.var_evbc3 - locals.var_evsc3);
        let assign4930_e4854: f64 = (assign4930_e4850 * assign4930_e4853);
        let assign4930_e4859: f64 = (4.0 * locals.var_iss_t);
        let assign4930_e4861: f64 = (assign4930_e4859 / locals.var_iks_t);
        let assign4930_e4865: f64 = (p.p144 * locals.var_evsc3);
        let assign4930_e4866: f64 = (locals.var_evbc3 + assign4930_e4865);
        let assign4930_e4867: f64 = (assign4930_e4861 * assign4930_e4866);
        let assign4930_e4868: f64 = (1.0 + assign4930_e4867);
        let assign4930_e4869: f64 = (assign4930_e4868).sqrt();
        let assign4930_e4870: f64 = (1.0 + assign4930_e4869);
        let assign4930_e4871: f64 = (assign4930_e4854 / assign4930_e4870);
        (assign4930_e4871, ((((assign4930_e4850 * locals.var_evbc3_dn0) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn0) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn1) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn1) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (-locals.var_evsc3_dn3)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (p.p144 * locals.var_evsc3_dn3)) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((((assign4930_e4848 * locals.var_iss_t_dn4) * assign4930_e4853) + (assign4930_e4850 * (locals.var_evbc3_dn4 - locals.var_evsc3_dn4))) * assign4930_e4870) - (assign4930_e4854 * (((((((4.0 * locals.var_iss_t_dn4) * locals.var_iks_t) - (assign4930_e4859 * locals.var_iks_t_dn4)) / (locals.var_iks_t * locals.var_iks_t)) * assign4930_e4866) + (assign4930_e4861 * (locals.var_evbc3_dn4 + (p.p144 * locals.var_evsc3_dn4)))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn6) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn6) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn7) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn7) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn8 - locals.var_evsc3_dn8)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn8 + (p.p144 * locals.var_evsc3_dn8))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn9) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn9) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn10 - locals.var_evsc3_dn10)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn10 + (p.p144 * locals.var_evsc3_dn10))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn11 - locals.var_evsc3_dn11)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn11 + (p.p144 * locals.var_evsc3_dn11))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)),)
    } else {
        (locals.var_ximsub, locals.var_ximsub_dn0, locals.var_ximsub_dn1, locals.var_ximsub_dn3, locals.var_ximsub_dn4, locals.var_ximsub_dn6, locals.var_ximsub_dn7, locals.var_ximsub_dn8, locals.var_ximsub_dn9, locals.var_ximsub_dn10, locals.var_ximsub_dn11,)
    }
};
        locals.var_ximsub = assign4930_e4873;
        locals.var_ximsub_dn0 = assign4930_e4873_d_n0;
        locals.var_ximsub_dn1 = assign4930_e4873_d_n1;
        locals.var_ximsub_dn3 = assign4930_e4873_d_n3;
        locals.var_ximsub_dn4 = assign4930_e4873_d_n4;
        locals.var_ximsub_dn6 = assign4930_e4873_d_n6;
        locals.var_ximsub_dn7 = assign4930_e4873_d_n7;
        locals.var_ximsub_dn8 = assign4930_e4873_d_n8;
        locals.var_ximsub_dn9 = assign4930_e4873_d_n9;
        locals.var_ximsub_dn10 = assign4930_e4873_d_n10;
        locals.var_ximsub_dn11 = assign4930_e4873_d_n11;
        locals.var_ximsub_rv = 0.0;

        let (assign4940_e4905, assign4940_e4905_d_n0, assign4940_e4905_d_n1, assign4940_e4905_d_n3, assign4940_e4905_d_n4, assign4940_e4905_d_n6, assign4940_e4905_d_n7, assign4940_e4905_d_n8, assign4940_e4905_d_n9, assign4940_e4905_d_n10, assign4940_e4905_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) {
        let assign4940_e4880: f64 = (1.0 - p.p143);
        let assign4940_e4882: f64 = (assign4940_e4880 * p.p33);
        let assign4940_e4884: f64 = (assign4940_e4882 * 2.0);
        let assign4940_e4886: f64 = (assign4940_e4884 * locals.var_iss_t);
        let assign4940_e4889: f64 = (locals.var_evbc3 - 1.0);
        let assign4940_e4890: f64 = (assign4940_e4886 * assign4940_e4889);
        let assign4940_e4895: f64 = (4.0 * locals.var_iss_t);
        let assign4940_e4897: f64 = (assign4940_e4895 / locals.var_iks_t);
        let assign4940_e4899: f64 = (assign4940_e4897 * locals.var_evbc3);
        let assign4940_e4900: f64 = (1.0 + assign4940_e4899);
        let assign4940_e4901: f64 = (assign4940_e4900).sqrt();
        let assign4940_e4902: f64 = (1.0 + assign4940_e4901);
        let assign4940_e4903: f64 = (assign4940_e4890 / assign4940_e4902);
        (assign4940_e4903, ((((assign4940_e4886 * locals.var_evbc3_dn0) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn0) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn1) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn1) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), 0.0, ((((((assign4940_e4884 * locals.var_iss_t_dn4) * assign4940_e4889) + (assign4940_e4886 * locals.var_evbc3_dn4)) * assign4940_e4902) - (assign4940_e4890 * (((((((4.0 * locals.var_iss_t_dn4) * locals.var_iks_t) - (assign4940_e4895 * locals.var_iks_t_dn4)) / (locals.var_iks_t * locals.var_iks_t)) * locals.var_evbc3) + (assign4940_e4897 * locals.var_evbc3_dn4)) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn6) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn6) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn7) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn7) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn8) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn8) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn9) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn9) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn10) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn10) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn11) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn11) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)),)
    } else {
        (locals.var_ximsub, locals.var_ximsub_dn0, locals.var_ximsub_dn1, locals.var_ximsub_dn3, locals.var_ximsub_dn4, locals.var_ximsub_dn6, locals.var_ximsub_dn7, locals.var_ximsub_dn8, locals.var_ximsub_dn9, locals.var_ximsub_dn10, locals.var_ximsub_dn11,)
    }
};
        locals.var_ximsub = assign4940_e4905;
        locals.var_ximsub_dn0 = assign4940_e4905_d_n0;
        locals.var_ximsub_dn1 = assign4940_e4905_d_n1;
        locals.var_ximsub_dn3 = assign4940_e4905_d_n3;
        locals.var_ximsub_dn4 = assign4940_e4905_d_n4;
        locals.var_ximsub_dn6 = assign4940_e4905_d_n6;
        locals.var_ximsub_dn7 = assign4940_e4905_d_n7;
        locals.var_ximsub_dn8 = assign4940_e4905_d_n8;
        locals.var_ximsub_dn9 = assign4940_e4905_d_n9;
        locals.var_ximsub_dn10 = assign4940_e4905_d_n10;
        locals.var_ximsub_dn11 = assign4940_e4905_d_n11;
        locals.var_ximsub_rv = 0.0;

        let assign4950_e4908: f64 = if p.p5 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign4950_e4908;
        locals.var_guard85_rv = 0.0;

        let (assign4960_e4920, assign4960_e4920_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4960_e4915: f64 = (locals.var_ibx_t + locals.var_iss_t);
        let assign4960_e4916: f64 = (p.p33 * assign4960_e4915);
        let assign4960_e4918: f64 = (assign4960_e4916 * locals.var_rcc_xx_t);
        (assign4960_e4918, (((p.p33 * (locals.var_ibx_t_dn4 + locals.var_iss_t_dn4)) * locals.var_rcc_xx_t) + (assign4960_e4916 * locals.var_rcc_xx_t_dn4)),)
    } else {
        (locals.var_vex_bias, locals.var_vex_bias_dn4,)
    }
};
        locals.var_vex_bias = assign4960_e4920;
        locals.var_vex_bias_dn4 = assign4960_e4920_d_n4;
        locals.var_vex_bias_rv = 0.0;

        let (assign4970_e4933, assign4970_e4933_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4970_e4928: f64 = (locals.var_vex_bias * locals.var_vtinv);
        let assign4970_e4929: f64 = (assign4970_e4928).ln();
        let assign4970_e4930: f64 = (2.0 - assign4970_e4929);
        let assign4970_e4931: f64 = (locals.var_vt * assign4970_e4930);
        (assign4970_e4931, ((locals.var_vt_dn4 * assign4970_e4930) + (locals.var_vt * (-(((locals.var_vex_bias_dn4 * locals.var_vtinv) + (locals.var_vex_bias * locals.var_vtinv_dn4)) / assign4970_e4928)))),)
    } else {
        (locals.var_vex, locals.var_vex_dn4,)
    }
};
        locals.var_vex = assign4970_e4933;
        locals.var_vex_dn4 = assign4970_e4933_d_n4;
        locals.var_vex_rv = 0.0;

        let (assign4980_e4941, assign4980_e4941_d_n0, assign4980_e4941_d_n1, assign4980_e4941_d_n4, assign4980_e4941_d_n6, assign4980_e4941_d_n7, assign4980_e4941_d_n8, assign4980_e4941_d_n9, assign4980_e4941_d_n10, assign4980_e4941_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4980_e4939: f64 = (locals.var_vbc3 - locals.var_vex);
        (assign4980_e4939, locals.var_vbc3_dn0, locals.var_vbc3_dn1, (-locals.var_vex_dn4), locals.var_vbc3_dn6, locals.var_vbc3_dn7, locals.var_vbc3_dn8, locals.var_vbc3_dn9, locals.var_vbc3_dn10, locals.var_vbc3_dn11,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10, locals.var_vdif_dn11,)
    }
};
        locals.var_vdif = assign4980_e4941;
        locals.var_vdif_dn0 = assign4980_e4941_d_n0;
        locals.var_vdif_dn1 = assign4980_e4941_d_n1;
        locals.var_vdif_dn4 = assign4980_e4941_d_n4;
        locals.var_vdif_dn6 = assign4980_e4941_d_n6;
        locals.var_vdif_dn7 = assign4980_e4941_d_n7;
        locals.var_vdif_dn8 = assign4980_e4941_d_n8;
        locals.var_vdif_dn9 = assign4980_e4941_d_n9;
        locals.var_vdif_dn10 = assign4980_e4941_d_n10;
        locals.var_vdif_dn11 = assign4980_e4941_d_n11;
        locals.var_vdif_rv = 0.0;

        let (assign4990_e4949, assign4990_e4949_d_n0, assign4990_e4949_d_n1, assign4990_e4949_d_n3, assign4990_e4949_d_n4, assign4990_e4949_d_n5, assign4990_e4949_d_n6, assign4990_e4949_d_n7, assign4990_e4949_d_n8, assign4990_e4949_d_n9, assign4990_e4949_d_n10, assign4990_e4949_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4990_e4947: f64 = (0.11 * 0.11);
        (assign4990_e4947, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10, locals.var_eps2_dn11,)
    }
};
        locals.var_eps2 = assign4990_e4949;
        locals.var_eps2_dn0 = assign4990_e4949_d_n0;
        locals.var_eps2_dn1 = assign4990_e4949_d_n1;
        locals.var_eps2_dn3 = assign4990_e4949_d_n3;
        locals.var_eps2_dn4 = assign4990_e4949_d_n4;
        locals.var_eps2_dn5 = assign4990_e4949_d_n5;
        locals.var_eps2_dn6 = assign4990_e4949_d_n6;
        locals.var_eps2_dn7 = assign4990_e4949_d_n7;
        locals.var_eps2_dn8 = assign4990_e4949_d_n8;
        locals.var_eps2_dn9 = assign4990_e4949_d_n9;
        locals.var_eps2_dn10 = assign4990_e4949_d_n10;
        locals.var_eps2_dn11 = assign4990_e4949_d_n11;
        locals.var_eps2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5000_e4957, assign5000_e4957_d_n0, assign5000_e4957_d_n1, assign5000_e4957_d_n3, assign5000_e4957_d_n4, assign5000_e4957_d_n5, assign5000_e4957_d_n6, assign5000_e4957_d_n7, assign5000_e4957_d_n8, assign5000_e4957_d_n9, assign5000_e4957_d_n10, assign5000_e4957_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign5000_e4955: f64 = (locals.var_vdif * locals.var_vdif);
        (assign5000_e4955, ((locals.var_vdif_dn0 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn0)), ((locals.var_vdif_dn1 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn1)), 0.0, ((locals.var_vdif_dn4 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn4)), 0.0, ((locals.var_vdif_dn6 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn6)), ((locals.var_vdif_dn7 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn7)), ((locals.var_vdif_dn8 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn8)), ((locals.var_vdif_dn9 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn9)), ((locals.var_vdif_dn10 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn10)), ((locals.var_vdif_dn11 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn11)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11,)
    }
};
        locals.var_x2 = assign5000_e4957;
        locals.var_x2_dn0 = assign5000_e4957_d_n0;
        locals.var_x2_dn1 = assign5000_e4957_d_n1;
        locals.var_x2_dn3 = assign5000_e4957_d_n3;
        locals.var_x2_dn4 = assign5000_e4957_d_n4;
        locals.var_x2_dn5 = assign5000_e4957_d_n5;
        locals.var_x2_dn6 = assign5000_e4957_d_n6;
        locals.var_x2_dn7 = assign5000_e4957_d_n7;
        locals.var_x2_dn8 = assign5000_e4957_d_n8;
        locals.var_x2_dn9 = assign5000_e4957_d_n9;
        locals.var_x2_dn10 = assign5000_e4957_d_n10;
        locals.var_x2_dn11 = assign5000_e4957_d_n11;
        locals.var_x2_rv = 0.0;

        let assign5010_e4960: f64 = if locals.var_vdif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign5010_e4960;
        locals.var_guard86_rv = 0.0;

        let (assign5020_e4977, assign5020_e4977_d_n0, assign5020_e4977_d_n1, assign5020_e4977_d_n3, assign5020_e4977_d_n4, assign5020_e4977_d_n5, assign5020_e4977_d_n6, assign5020_e4977_d_n7, assign5020_e4977_d_n8, assign5020_e4977_d_n9, assign5020_e4977_d_n10, assign5020_e4977_d_n11,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign5020_e4968: f64 = (0.5 * locals.var_eps2);
        let assign5020_e4971: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5020_e4972: f64 = (assign5020_e4971).sqrt();
        let assign5020_e4974: f64 = (assign5020_e4972 - locals.var_vdif);
        let assign5020_e4975: f64 = (assign5020_e4968 / assign5020_e4974);
        (assign5020_e4975, ((((0.5 * locals.var_eps2_dn0) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn0))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn1) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn1))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn3) * assign5020_e4974) - (assign5020_e4968 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5020_e4972)))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn4) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn4))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn5) * assign5020_e4974) - (assign5020_e4968 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5020_e4972)))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn6) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn6))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn7) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn7))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn8) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn8))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn9) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn9))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn10) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn10))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn11) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn11))) / (assign5020_e4974 * assign5020_e4974)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5020_e4977;
        locals.var_vbex_dn0 = assign5020_e4977_d_n0;
        locals.var_vbex_dn1 = assign5020_e4977_d_n1;
        locals.var_vbex_dn3 = assign5020_e4977_d_n3;
        locals.var_vbex_dn4 = assign5020_e4977_d_n4;
        locals.var_vbex_dn5 = assign5020_e4977_d_n5;
        locals.var_vbex_dn6 = assign5020_e4977_d_n6;
        locals.var_vbex_dn7 = assign5020_e4977_d_n7;
        locals.var_vbex_dn8 = assign5020_e4977_d_n8;
        locals.var_vbex_dn9 = assign5020_e4977_d_n9;
        locals.var_vbex_dn10 = assign5020_e4977_d_n10;
        locals.var_vbex_dn11 = assign5020_e4977_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5030_e4993, assign5030_e4993_d_n0, assign5030_e4993_d_n1, assign5030_e4993_d_n3, assign5030_e4993_d_n4, assign5030_e4993_d_n5, assign5030_e4993_d_n6, assign5030_e4993_d_n7, assign5030_e4993_d_n8, assign5030_e4993_d_n9, assign5030_e4993_d_n10, assign5030_e4993_d_n11,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 == 0.0)) {
        let assign5030_e4987: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5030_e4988: f64 = (assign5030_e4987).sqrt();
        let assign5030_e4990: f64 = (assign5030_e4988 + locals.var_vdif);
        let assign5030_e4991: f64 = (0.5 * assign5030_e4990);
        (assign5030_e4991, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn1)), (0.5 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5030_e4988))), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn4)), (0.5 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5030_e4988))), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn11)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5030_e4993;
        locals.var_vbex_dn0 = assign5030_e4993_d_n0;
        locals.var_vbex_dn1 = assign5030_e4993_d_n1;
        locals.var_vbex_dn3 = assign5030_e4993_d_n3;
        locals.var_vbex_dn4 = assign5030_e4993_d_n4;
        locals.var_vbex_dn5 = assign5030_e4993_d_n5;
        locals.var_vbex_dn6 = assign5030_e4993_d_n6;
        locals.var_vbex_dn7 = assign5030_e4993_d_n7;
        locals.var_vbex_dn8 = assign5030_e4993_d_n8;
        locals.var_vbex_dn9 = assign5030_e4993_d_n9;
        locals.var_vbex_dn10 = assign5030_e4993_d_n10;
        locals.var_vbex_dn11 = assign5030_e4993_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5040_e5009, assign5040_e5009_d_n0, assign5040_e5009_d_n1, assign5040_e5009_d_n3, assign5040_e5009_d_n4, assign5040_e5009_d_n5, assign5040_e5009_d_n6, assign5040_e5009_d_n7, assign5040_e5009_d_n8, assign5040_e5009_d_n9, assign5040_e5009_d_n10, assign5040_e5009_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign5040_e5001: f64 = (locals.var_ximex + locals.var_ximsub);
        let assign5040_e5003: f64 = (assign5040_e5001 * locals.var_rcc_xx_t);
        let assign5040_e5004: f64 = (locals.var_vex_bias + assign5040_e5003);
        let assign5040_e5006: f64 = (assign5040_e5004 + locals.var_vbex);
        let assign5040_e5007: f64 = (locals.var_vbex / assign5040_e5006);
        (assign5040_e5007, (((locals.var_vbex_dn0 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn0 + locals.var_ximsub_dn0) * locals.var_rcc_xx_t) + locals.var_vbex_dn0))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn1 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn1 + locals.var_ximsub_dn1) * locals.var_rcc_xx_t) + locals.var_vbex_dn1))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn3 * assign5040_e5006) - (locals.var_vbex * ((locals.var_ximsub_dn3 * locals.var_rcc_xx_t) + locals.var_vbex_dn3))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn4 * assign5040_e5006) - (locals.var_vbex * ((locals.var_vex_bias_dn4 + (((locals.var_ximex_dn4 + locals.var_ximsub_dn4) * locals.var_rcc_xx_t) + (assign5040_e5001 * locals.var_rcc_xx_t_dn4))) + locals.var_vbex_dn4))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn5 * assign5040_e5006) - (locals.var_vbex * locals.var_vbex_dn5)) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn6 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn6 + locals.var_ximsub_dn6) * locals.var_rcc_xx_t) + locals.var_vbex_dn6))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn7 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn7 + locals.var_ximsub_dn7) * locals.var_rcc_xx_t) + locals.var_vbex_dn7))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn8 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn8 + locals.var_ximsub_dn8) * locals.var_rcc_xx_t) + locals.var_vbex_dn8))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn9 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn9 + locals.var_ximsub_dn9) * locals.var_rcc_xx_t) + locals.var_vbex_dn9))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn10 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn10 + locals.var_ximsub_dn10) * locals.var_rcc_xx_t) + locals.var_vbex_dn10))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn11 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn11 + locals.var_ximsub_dn11) * locals.var_rcc_xx_t) + locals.var_vbex_dn11))) / (assign5040_e5006 * assign5040_e5006)),)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10, locals.var_fex_dn11,)
    }
};
        locals.var_fex = assign5040_e5009;
        locals.var_fex_dn0 = assign5040_e5009_d_n0;
        locals.var_fex_dn1 = assign5040_e5009_d_n1;
        locals.var_fex_dn3 = assign5040_e5009_d_n3;
        locals.var_fex_dn4 = assign5040_e5009_d_n4;
        locals.var_fex_dn5 = assign5040_e5009_d_n5;
        locals.var_fex_dn6 = assign5040_e5009_d_n6;
        locals.var_fex_dn7 = assign5040_e5009_d_n7;
        locals.var_fex_dn8 = assign5040_e5009_d_n8;
        locals.var_fex_dn9 = assign5040_e5009_d_n9;
        locals.var_fex_dn10 = assign5040_e5009_d_n10;
        locals.var_fex_dn11 = assign5040_e5009_d_n11;
        locals.var_fex_rv = 0.0;

        let (assign5050_e5016, assign5050_e5016_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_vex, locals.var_vex_dn4,)
    }
};
        locals.var_vex = assign5050_e5016;
        locals.var_vex_dn4 = assign5050_e5016_d_n4;
        locals.var_vex_rv = 0.0;

        let (assign5060_e5023, assign5060_e5023_d_n0, assign5060_e5023_d_n1, assign5060_e5023_d_n4, assign5060_e5023_d_n6, assign5060_e5023_d_n7, assign5060_e5023_d_n8, assign5060_e5023_d_n9, assign5060_e5023_d_n10, assign5060_e5023_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10, locals.var_vdif_dn11,)
    }
};
        locals.var_vdif = assign5060_e5023;
        locals.var_vdif_dn0 = assign5060_e5023_d_n0;
        locals.var_vdif_dn1 = assign5060_e5023_d_n1;
        locals.var_vdif_dn4 = assign5060_e5023_d_n4;
        locals.var_vdif_dn6 = assign5060_e5023_d_n6;
        locals.var_vdif_dn7 = assign5060_e5023_d_n7;
        locals.var_vdif_dn8 = assign5060_e5023_d_n8;
        locals.var_vdif_dn9 = assign5060_e5023_d_n9;
        locals.var_vdif_dn10 = assign5060_e5023_d_n10;
        locals.var_vdif_dn11 = assign5060_e5023_d_n11;
        locals.var_vdif_rv = 0.0;

        let (assign5070_e5030, assign5070_e5030_d_n0, assign5070_e5030_d_n1, assign5070_e5030_d_n3, assign5070_e5030_d_n4, assign5070_e5030_d_n5, assign5070_e5030_d_n6, assign5070_e5030_d_n7, assign5070_e5030_d_n8, assign5070_e5030_d_n9, assign5070_e5030_d_n10, assign5070_e5030_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5070_e5030;
        locals.var_vbex_dn0 = assign5070_e5030_d_n0;
        locals.var_vbex_dn1 = assign5070_e5030_d_n1;
        locals.var_vbex_dn3 = assign5070_e5030_d_n3;
        locals.var_vbex_dn4 = assign5070_e5030_d_n4;
        locals.var_vbex_dn5 = assign5070_e5030_d_n5;
        locals.var_vbex_dn6 = assign5070_e5030_d_n6;
        locals.var_vbex_dn7 = assign5070_e5030_d_n7;
        locals.var_vbex_dn8 = assign5070_e5030_d_n8;
        locals.var_vbex_dn9 = assign5070_e5030_d_n9;
        locals.var_vbex_dn10 = assign5070_e5030_d_n10;
        locals.var_vbex_dn11 = assign5070_e5030_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5080_e5037, assign5080_e5037_d_n0, assign5080_e5037_d_n1, assign5080_e5037_d_n3, assign5080_e5037_d_n4, assign5080_e5037_d_n5, assign5080_e5037_d_n6, assign5080_e5037_d_n7, assign5080_e5037_d_n8, assign5080_e5037_d_n9, assign5080_e5037_d_n10, assign5080_e5037_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10, locals.var_fex_dn11,)
    }
};
        locals.var_fex = assign5080_e5037;
        locals.var_fex_dn0 = assign5080_e5037_d_n0;
        locals.var_fex_dn1 = assign5080_e5037_d_n1;
        locals.var_fex_dn3 = assign5080_e5037_d_n3;
        locals.var_fex_dn4 = assign5080_e5037_d_n4;
        locals.var_fex_dn5 = assign5080_e5037_d_n5;
        locals.var_fex_dn6 = assign5080_e5037_d_n6;
        locals.var_fex_dn7 = assign5080_e5037_d_n7;
        locals.var_fex_dn8 = assign5080_e5037_d_n8;
        locals.var_fex_dn9 = assign5080_e5037_d_n9;
        locals.var_fex_dn10 = assign5080_e5037_d_n10;
        locals.var_fex_dn11 = assign5080_e5037_d_n11;
        locals.var_fex_rv = 0.0;

        let assign5110_e5052: f64 = if p.p84 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5110_e5052;
        locals.var_guard87_rv = 0.0;

        let (assign5120_e5058, assign5120_e5058_d_n6, assign5120_e5058_d_n7, assign5120_e5058_d_n8,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5120_e5056: f64 = (locals.var_vb1b2 + locals.var_vb2c1);
        (assign5120_e5056, locals.var_vb1b2_dn6, (locals.var_vb1b2_dn7 + locals.var_vb2c1_dn7), locals.var_vb2c1_dn8,)
    } else {
        (locals.var_vb1c1, locals.var_vb1c1_dn6, locals.var_vb1c1_dn7, locals.var_vb1c1_dn8,)
    }
};
        locals.var_vb1c1 = assign5120_e5058;
        locals.var_vb1c1_dn6 = assign5120_e5058_d_n6;
        locals.var_vb1c1_dn7 = assign5120_e5058_d_n7;
        locals.var_vb1c1_dn8 = assign5120_e5058_d_n8;
        locals.var_vb1c1_rv = 0.0;

        let (assign5130_e5064, assign5130_e5064_d_n0, assign5130_e5064_d_n1, assign5130_e5064_d_n3, assign5130_e5064_d_n4, assign5130_e5064_d_n5, assign5130_e5064_d_n6, assign5130_e5064_d_n7, assign5130_e5064_d_n8, assign5130_e5064_d_n9, assign5130_e5064_d_n10, assign5130_e5064_d_n11,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5130_e5062: f64 = (1e-6 * 1e-6);
        (assign5130_e5062, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10, locals.var_eps2_dn11,)
    }
};
        locals.var_eps2 = assign5130_e5064;
        locals.var_eps2_dn0 = assign5130_e5064_d_n0;
        locals.var_eps2_dn1 = assign5130_e5064_d_n1;
        locals.var_eps2_dn3 = assign5130_e5064_d_n3;
        locals.var_eps2_dn4 = assign5130_e5064_d_n4;
        locals.var_eps2_dn5 = assign5130_e5064_d_n5;
        locals.var_eps2_dn6 = assign5130_e5064_d_n6;
        locals.var_eps2_dn7 = assign5130_e5064_d_n7;
        locals.var_eps2_dn8 = assign5130_e5064_d_n8;
        locals.var_eps2_dn9 = assign5130_e5064_d_n9;
        locals.var_eps2_dn10 = assign5130_e5064_d_n10;
        locals.var_eps2_dn11 = assign5130_e5064_d_n11;
        locals.var_eps2_rv = 0.0;

        let (assign5140_e5076, assign5140_e5076_d_n0, assign5140_e5076_d_n1, assign5140_e5076_d_n3, assign5140_e5076_d_n4, assign5140_e5076_d_n5, assign5140_e5076_d_n6, assign5140_e5076_d_n7, assign5140_e5076_d_n8, assign5140_e5076_d_n9, assign5140_e5076_d_n10, assign5140_e5076_d_n11,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5140_e5067: f64 = (-1.0);
        let assign5140_e5069: f64 = (assign5140_e5067 * locals.var_vb1c1);
        let assign5140_e5071: f64 = (-1.0);
        let assign5140_e5072: f64 = (assign5140_e5069 * assign5140_e5071);
        let assign5140_e5074: f64 = (assign5140_e5072 * locals.var_vb1c1);
        (assign5140_e5074, 0.0, 0.0, 0.0, 0.0, 0.0, ((((assign5140_e5067 * locals.var_vb1c1_dn6) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn6)), ((((assign5140_e5067 * locals.var_vb1c1_dn7) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn7)), ((((assign5140_e5067 * locals.var_vb1c1_dn8) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn8)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11,)
    }
};
        locals.var_x2 = assign5140_e5076;
        locals.var_x2_dn0 = assign5140_e5076_d_n0;
        locals.var_x2_dn1 = assign5140_e5076_d_n1;
        locals.var_x2_dn3 = assign5140_e5076_d_n3;
        locals.var_x2_dn4 = assign5140_e5076_d_n4;
        locals.var_x2_dn5 = assign5140_e5076_d_n5;
        locals.var_x2_dn6 = assign5140_e5076_d_n6;
        locals.var_x2_dn7 = assign5140_e5076_d_n7;
        locals.var_x2_dn8 = assign5140_e5076_d_n8;
        locals.var_x2_dn9 = assign5140_e5076_d_n9;
        locals.var_x2_dn10 = assign5140_e5076_d_n10;
        locals.var_x2_dn11 = assign5140_e5076_d_n11;
        locals.var_x2_rv = 0.0;

        let assign5290_e5200: f64 = (locals.var_vte / locals.var_ver_t);
        let assign5290_e5201: f64 = (1.0 + assign5290_e5200);
        let assign5290_e5204: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign5290_e5205: f64 = (assign5290_e5201 + assign5290_e5204);
        locals.var_q0q = assign5290_e5205;
        locals.var_q0q_dn0 = ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn1 = ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn3 = ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn4 = ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn5 = ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn6 = ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn7 = ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn8 = ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn9 = ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn10 = ((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn10 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn11 = ((((locals.var_vte_dn11 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn11)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn11 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn11)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_rv = 0.0;

        let assign5300_e5208: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign5300_e5208;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_dn11 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign5310_e5211: f64 = (locals.var_q0q * locals.var_q0q);
        locals.var_x2 = assign5310_e5211;
        locals.var_x2_dn0 = ((locals.var_q0q_dn0 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn0));
        locals.var_x2_dn1 = ((locals.var_q0q_dn1 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn1));
        locals.var_x2_dn3 = ((locals.var_q0q_dn3 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn3));
        locals.var_x2_dn4 = ((locals.var_q0q_dn4 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn4));
        locals.var_x2_dn5 = ((locals.var_q0q_dn5 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn5));
        locals.var_x2_dn6 = ((locals.var_q0q_dn6 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn6));
        locals.var_x2_dn7 = ((locals.var_q0q_dn7 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn7));
        locals.var_x2_dn8 = ((locals.var_q0q_dn8 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn8));
        locals.var_x2_dn9 = ((locals.var_q0q_dn9 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn9));
        locals.var_x2_dn10 = ((locals.var_q0q_dn10 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn10));
        locals.var_x2_dn11 = ((locals.var_q0q_dn11 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn11));
        locals.var_x2_rv = 0.0;

        let assign5320_e5214: f64 = if locals.var_q0q < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5320_e5214;
        locals.var_guard90_rv = 0.0;

        let (assign5330_e5227, assign5330_e5227_d_n0, assign5330_e5227_d_n1, assign5330_e5227_d_n3, assign5330_e5227_d_n4, assign5330_e5227_d_n5, assign5330_e5227_d_n6, assign5330_e5227_d_n7, assign5330_e5227_d_n8, assign5330_e5227_d_n9, assign5330_e5227_d_n10, assign5330_e5227_d_n11,) = {
    if (locals.var_guard90 != 0.0) {
        let assign5330_e5218: f64 = (0.5 * locals.var_eps2);
        let assign5330_e5221: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5330_e5222: f64 = (assign5330_e5221).sqrt();
        let assign5330_e5224: f64 = (assign5330_e5222 - locals.var_q0q);
        let assign5330_e5225: f64 = (assign5330_e5218 / assign5330_e5224);
        (assign5330_e5225, ((((0.5 * locals.var_eps2_dn0) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn0))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn1) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn1))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn3) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn3))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn4) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn4))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn5) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn5))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn6) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn6))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn7) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn7))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn8) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn8))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn9) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn9))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn10) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn10))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn11) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn11))) / (assign5330_e5224 * assign5330_e5224)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10, locals.var_q1q_dn11,)
    }
};
        locals.var_q1q = assign5330_e5227;
        locals.var_q1q_dn0 = assign5330_e5227_d_n0;
        locals.var_q1q_dn1 = assign5330_e5227_d_n1;
        locals.var_q1q_dn3 = assign5330_e5227_d_n3;
        locals.var_q1q_dn4 = assign5330_e5227_d_n4;
        locals.var_q1q_dn5 = assign5330_e5227_d_n5;
        locals.var_q1q_dn6 = assign5330_e5227_d_n6;
        locals.var_q1q_dn7 = assign5330_e5227_d_n7;
        locals.var_q1q_dn8 = assign5330_e5227_d_n8;
        locals.var_q1q_dn9 = assign5330_e5227_d_n9;
        locals.var_q1q_dn10 = assign5330_e5227_d_n10;
        locals.var_q1q_dn11 = assign5330_e5227_d_n11;
        locals.var_q1q_rv = 0.0;

        let (assign5340_e5239, assign5340_e5239_d_n0, assign5340_e5239_d_n1, assign5340_e5239_d_n3, assign5340_e5239_d_n4, assign5340_e5239_d_n5, assign5340_e5239_d_n6, assign5340_e5239_d_n7, assign5340_e5239_d_n8, assign5340_e5239_d_n9, assign5340_e5239_d_n10, assign5340_e5239_d_n11,) = {
    if (locals.var_guard90 == 0.0) {
        let assign5340_e5233: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5340_e5234: f64 = (assign5340_e5233).sqrt();
        let assign5340_e5236: f64 = (assign5340_e5234 + locals.var_q0q);
        let assign5340_e5237: f64 = (0.5 * assign5340_e5236);
        (assign5340_e5237, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn11)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10, locals.var_q1q_dn11,)
    }
};
        locals.var_q1q = assign5340_e5239;
        locals.var_q1q_dn0 = assign5340_e5239_d_n0;
        locals.var_q1q_dn1 = assign5340_e5239_d_n1;
        locals.var_q1q_dn3 = assign5340_e5239_d_n3;
        locals.var_q1q_dn4 = assign5340_e5239_d_n4;
        locals.var_q1q_dn5 = assign5340_e5239_d_n5;
        locals.var_q1q_dn6 = assign5340_e5239_d_n6;
        locals.var_q1q_dn7 = assign5340_e5239_d_n7;
        locals.var_q1q_dn8 = assign5340_e5239_d_n8;
        locals.var_q1q_dn9 = assign5340_e5239_d_n9;
        locals.var_q1q_dn10 = assign5340_e5239_d_n10;
        locals.var_q1q_dn11 = assign5340_e5239_d_n11;
        locals.var_q1q_rv = 0.0;

        let assign5350_e5245: f64 = (locals.var_n0 + locals.var_nb);
        let assign5350_e5246: f64 = (0.5 * assign5350_e5245);
        let assign5350_e5247: f64 = (1.0 + assign5350_e5246);
        let assign5350_e5248: f64 = (locals.var_q1q * assign5350_e5247);
        locals.var_qbq = assign5350_e5248;
        locals.var_qbq_dn0 = ((locals.var_q1q_dn0 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbq_dn1 = ((locals.var_q1q_dn1 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbq_dn3 = ((locals.var_q1q_dn3 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbq_dn4 = ((locals.var_q1q_dn4 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbq_dn5 = ((locals.var_q1q_dn5 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbq_dn6 = ((locals.var_q1q_dn6 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbq_dn7 = ((locals.var_q1q_dn7 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbq_dn8 = ((locals.var_q1q_dn8 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbq_dn9 = ((locals.var_q1q_dn9 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));
        locals.var_qbq_dn10 = ((locals.var_q1q_dn10 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn10 + locals.var_nb_dn10))));
        locals.var_qbq_dn11 = ((locals.var_q1q_dn11 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn11 + locals.var_nb_dn11))));
        locals.var_qbq_rv = 0.0;

        let assign5360_e5251: f64 = (locals.var_rbv_t / locals.var_qbq);
        locals.var_rbvtemp = assign5360_e5251;
        locals.var_rbvtemp_dn0 = (-((locals.var_rbv_t * locals.var_qbq_dn0) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn1 = (-((locals.var_rbv_t * locals.var_qbq_dn1) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn3 = (-((locals.var_rbv_t * locals.var_qbq_dn3) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn4 = (((locals.var_rbv_t_dn4 * locals.var_qbq) - (locals.var_rbv_t * locals.var_qbq_dn4)) / (locals.var_qbq * locals.var_qbq));
        locals.var_rbvtemp_dn5 = (-((locals.var_rbv_t * locals.var_qbq_dn5) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn6 = (-((locals.var_rbv_t * locals.var_qbq_dn6) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn7 = (-((locals.var_rbv_t * locals.var_qbq_dn7) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn8 = (-((locals.var_rbv_t * locals.var_qbq_dn8) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn9 = (-((locals.var_rbv_t * locals.var_qbq_dn9) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn10 = (-((locals.var_rbv_t * locals.var_qbq_dn10) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn11 = (-((locals.var_rbv_t * locals.var_qbq_dn11) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_rv = 0.0;

        let assign5370_e5254: f64 = if locals.var_rbvtemp < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5370_e5254;
        locals.var_guard91_rv = 0.0;

        let (assign5380_e5258, assign5380_e5258_d_n0, assign5380_e5258_d_n1, assign5380_e5258_d_n3, assign5380_e5258_d_n4, assign5380_e5258_d_n5, assign5380_e5258_d_n6, assign5380_e5258_d_n7, assign5380_e5258_d_n8, assign5380_e5258_d_n9, assign5380_e5258_d_n10, assign5380_e5258_d_n11,) = {
    if (locals.var_guard91 != 0.0) {
        (locals.var_minr_m, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbvtemp, locals.var_rbvtemp_dn0, locals.var_rbvtemp_dn1, locals.var_rbvtemp_dn3, locals.var_rbvtemp_dn4, locals.var_rbvtemp_dn5, locals.var_rbvtemp_dn6, locals.var_rbvtemp_dn7, locals.var_rbvtemp_dn8, locals.var_rbvtemp_dn9, locals.var_rbvtemp_dn10, locals.var_rbvtemp_dn11,)
    }
};
        locals.var_rbvtemp = assign5380_e5258;
        locals.var_rbvtemp_dn0 = assign5380_e5258_d_n0;
        locals.var_rbvtemp_dn1 = assign5380_e5258_d_n1;
        locals.var_rbvtemp_dn3 = assign5380_e5258_d_n3;
        locals.var_rbvtemp_dn4 = assign5380_e5258_d_n4;
        locals.var_rbvtemp_dn5 = assign5380_e5258_d_n5;
        locals.var_rbvtemp_dn6 = assign5380_e5258_d_n6;
        locals.var_rbvtemp_dn7 = assign5380_e5258_d_n7;
        locals.var_rbvtemp_dn8 = assign5380_e5258_d_n8;
        locals.var_rbvtemp_dn9 = assign5380_e5258_d_n9;
        locals.var_rbvtemp_dn10 = assign5380_e5258_d_n10;
        locals.var_rbvtemp_dn11 = assign5380_e5258_d_n11;
        locals.var_rbvtemp_rv = 0.0;

        let assign5390_e5261: f64 = (3.0 * locals.var_rbvtemp);
        locals.var_rb2 = assign5390_e5261;
        locals.var_rb2_dn0 = (3.0 * locals.var_rbvtemp_dn0);
        locals.var_rb2_dn1 = (3.0 * locals.var_rbvtemp_dn1);
        locals.var_rb2_dn3 = (3.0 * locals.var_rbvtemp_dn3);
        locals.var_rb2_dn4 = (3.0 * locals.var_rbvtemp_dn4);
        locals.var_rb2_dn5 = (3.0 * locals.var_rbvtemp_dn5);
        locals.var_rb2_dn6 = (3.0 * locals.var_rbvtemp_dn6);
        locals.var_rb2_dn7 = (3.0 * locals.var_rbvtemp_dn7);
        locals.var_rb2_dn8 = (3.0 * locals.var_rbvtemp_dn8);
        locals.var_rb2_dn9 = (3.0 * locals.var_rbvtemp_dn9);
        locals.var_rb2_dn10 = (3.0 * locals.var_rbvtemp_dn10);
        locals.var_rb2_dn11 = (3.0 * locals.var_rbvtemp_dn11);
        locals.var_rb2_rv = 0.0;

        let assign5410_e5275: f64 = if locals.var_in_ > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5410_e5275;
        locals.var_guard92_rv = 0.0;

        let assign5420_e5278: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5420_e5278;
        locals.var_guard93_rv = 0.0;

        let assign5430_e5281: f64 = if locals.var_vb2c1 < p.p44 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5430_e5281;
        locals.var_guard94_rv = 0.0;

        let assign5440_e5283: f64 = (-locals.var_in_);
        let assign5440_e5285: f64 = (assign5440_e5283 / p.p42);
        let assign5440_e5287: f64 = if assign5440_e5285 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5440_e5287;
        locals.var_guard95_rv = 0.0;

        let (assign5450_e5301, assign5450_e5301_d_n0, assign5450_e5301_d_n1, assign5450_e5301_d_n3, assign5450_e5301_d_n4, assign5450_e5301_d_n5, assign5450_e5301_d_n6, assign5450_e5301_d_n7, assign5450_e5301_d_n8, assign5450_e5301_d_n9, assign5450_e5301_d_n10, assign5450_e5301_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5450_e5296: f64 = (-locals.var_in_);
        let assign5450_e5298: f64 = (assign5450_e5296 / p.p42);
        let assign5450_e5299: f64 = (assign5450_e5298).exp();
        (assign5450_e5299, (assign5450_e5299 * ((-locals.var_in__dn0) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn1) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn3) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn4) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn5) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn6) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn7) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn8) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn9) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn10) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn11) / p.p42)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10, locals.var_expin_dn11,)
    }
};
        locals.var_expin = assign5450_e5301;
        locals.var_expin_dn0 = assign5450_e5301_d_n0;
        locals.var_expin_dn1 = assign5450_e5301_d_n1;
        locals.var_expin_dn3 = assign5450_e5301_d_n3;
        locals.var_expin_dn4 = assign5450_e5301_d_n4;
        locals.var_expin_dn5 = assign5450_e5301_d_n5;
        locals.var_expin_dn6 = assign5450_e5301_d_n6;
        locals.var_expin_dn7 = assign5450_e5301_d_n7;
        locals.var_expin_dn8 = assign5450_e5301_d_n8;
        locals.var_expin_dn9 = assign5450_e5301_d_n9;
        locals.var_expin_dn10 = assign5450_e5301_d_n10;
        locals.var_expin_dn11 = assign5450_e5301_d_n11;
        locals.var_expin_rv = 0.0;

        let (assign5460_e5313,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5460_e5311: f64 = (p.p151).exp();
        (assign5460_e5311,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5460_e5313;
        locals.var_expl_rv = 0.0;

        let (assign5470_e5333, assign5470_e5333_d_n0, assign5470_e5333_d_n1, assign5470_e5333_d_n3, assign5470_e5333_d_n4, assign5470_e5333_d_n5, assign5470_e5333_d_n6, assign5470_e5333_d_n7, assign5470_e5333_d_n8, assign5470_e5333_d_n9, assign5470_e5333_d_n10, assign5470_e5333_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5470_e5325: f64 = (-locals.var_in_);
        let assign5470_e5327: f64 = (assign5470_e5325 / p.p42);
        let assign5470_e5329: f64 = (assign5470_e5327 - p.p151);
        let assign5470_e5330: f64 = (1.0 + assign5470_e5329);
        let assign5470_e5331: f64 = (locals.var_expl * assign5470_e5330);
        (assign5470_e5331, (locals.var_expl * ((-locals.var_in__dn0) / p.p42)), (locals.var_expl * ((-locals.var_in__dn1) / p.p42)), (locals.var_expl * ((-locals.var_in__dn3) / p.p42)), (locals.var_expl * ((-locals.var_in__dn4) / p.p42)), (locals.var_expl * ((-locals.var_in__dn5) / p.p42)), (locals.var_expl * ((-locals.var_in__dn6) / p.p42)), (locals.var_expl * ((-locals.var_in__dn7) / p.p42)), (locals.var_expl * ((-locals.var_in__dn8) / p.p42)), (locals.var_expl * ((-locals.var_in__dn9) / p.p42)), (locals.var_expl * ((-locals.var_in__dn10) / p.p42)), (locals.var_expl * ((-locals.var_in__dn11) / p.p42)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10, locals.var_expin_dn11,)
    }
};
        locals.var_expin = assign5470_e5333;
        locals.var_expin_dn0 = assign5470_e5333_d_n0;
        locals.var_expin_dn1 = assign5470_e5333_d_n1;
        locals.var_expin_dn3 = assign5470_e5333_d_n3;
        locals.var_expin_dn4 = assign5470_e5333_d_n4;
        locals.var_expin_dn5 = assign5470_e5333_d_n5;
        locals.var_expin_dn6 = assign5470_e5333_d_n6;
        locals.var_expin_dn7 = assign5470_e5333_d_n7;
        locals.var_expin_dn8 = assign5470_e5333_d_n8;
        locals.var_expin_dn9 = assign5470_e5333_d_n9;
        locals.var_expin_dn10 = assign5470_e5333_d_n10;
        locals.var_expin_dn11 = assign5470_e5333_d_n11;
        locals.var_expin_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5480_e5345, assign5480_e5345_d_n0, assign5480_e5345_d_n1, assign5480_e5345_d_n3, assign5480_e5345_d_n4, assign5480_e5345_d_n5, assign5480_e5345_d_n6, assign5480_e5345_d_n7, assign5480_e5345_d_n8, assign5480_e5345_d_n9, assign5480_e5345_d_n10, assign5480_e5345_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5480_e5341: f64 = (p.p44 - locals.var_vb2c1);
        let assign5480_e5343: f64 = (assign5480_e5341 * locals.var_expin);
        (assign5480_e5343, (assign5480_e5341 * locals.var_expin_dn0), (assign5480_e5341 * locals.var_expin_dn1), (assign5480_e5341 * locals.var_expin_dn3), (assign5480_e5341 * locals.var_expin_dn4), (assign5480_e5341 * locals.var_expin_dn5), (assign5480_e5341 * locals.var_expin_dn6), (((-locals.var_vb2c1_dn7) * locals.var_expin) + (assign5480_e5341 * locals.var_expin_dn7)), (((-locals.var_vb2c1_dn8) * locals.var_expin) + (assign5480_e5341 * locals.var_expin_dn8)), (assign5480_e5341 * locals.var_expin_dn9), (assign5480_e5341 * locals.var_expin_dn10), (assign5480_e5341 * locals.var_expin_dn11),)
    } else {
        (locals.var_vl, locals.var_vl_dn0, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5, locals.var_vl_dn6, locals.var_vl_dn7, locals.var_vl_dn8, locals.var_vl_dn9, locals.var_vl_dn10, locals.var_vl_dn11,)
    }
};
        locals.var_vl = assign5480_e5345;
        locals.var_vl_dn0 = assign5480_e5345_d_n0;
        locals.var_vl_dn1 = assign5480_e5345_d_n1;
        locals.var_vl_dn3 = assign5480_e5345_d_n3;
        locals.var_vl_dn4 = assign5480_e5345_d_n4;
        locals.var_vl_dn5 = assign5480_e5345_d_n5;
        locals.var_vl_dn6 = assign5480_e5345_d_n6;
        locals.var_vl_dn7 = assign5480_e5345_d_n7;
        locals.var_vl_dn8 = assign5480_e5345_d_n8;
        locals.var_vl_dn9 = assign5480_e5345_d_n9;
        locals.var_vl_dn10 = assign5480_e5345_d_n10;
        locals.var_vl_dn11 = assign5480_e5345_d_n11;
        locals.var_vl_rv = 0.0;

        let assign5490_e5347: f64 = (-locals.var_bavl_t);
        let assign5490_e5350: f64 = (locals.var_vl).powf(p.p41);
        let assign5490_e5351: f64 = (assign5490_e5347 * assign5490_e5350);
        let assign5490_e5353: f64 = if assign5490_e5351 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5490_e5353;
        locals.var_guard96_rv = 0.0;

        let (assign5500_e5369, assign5500_e5369_d_n0, assign5500_e5369_d_n1, assign5500_e5369_d_n3, assign5500_e5369_d_n4, assign5500_e5369_d_n5, assign5500_e5369_d_n6, assign5500_e5369_d_n7, assign5500_e5369_d_n8, assign5500_e5369_d_n9, assign5500_e5369_d_n10, assign5500_e5369_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5500_e5362: f64 = (-locals.var_bavl_t);
        let assign5500_e5365: f64 = (locals.var_vl).powf(p.p41);
        let assign5500_e5366: f64 = (assign5500_e5362 * assign5500_e5365);
        let assign5500_e5367: f64 = (assign5500_e5366).exp();
        (assign5500_e5367, (assign5500_e5367 * (((-locals.var_bavl_t_dn0) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn0)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn0 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn1) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn1)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn1 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn3) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn3)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn3 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn4) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn4)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn4 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn5) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn5)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn5 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn6) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn6)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn6 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn7) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn7)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn7 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn8) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn8)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn8 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn9) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn9)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn9 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn10) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn10)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn10 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn11) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn11)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn11 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5500_e5369;
        locals.var_expmm1_dn0 = assign5500_e5369_d_n0;
        locals.var_expmm1_dn1 = assign5500_e5369_d_n1;
        locals.var_expmm1_dn3 = assign5500_e5369_d_n3;
        locals.var_expmm1_dn4 = assign5500_e5369_d_n4;
        locals.var_expmm1_dn5 = assign5500_e5369_d_n5;
        locals.var_expmm1_dn6 = assign5500_e5369_d_n6;
        locals.var_expmm1_dn7 = assign5500_e5369_d_n7;
        locals.var_expmm1_dn8 = assign5500_e5369_d_n8;
        locals.var_expmm1_dn9 = assign5500_e5369_d_n9;
        locals.var_expmm1_dn10 = assign5500_e5369_d_n10;
        locals.var_expmm1_dn11 = assign5500_e5369_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5510_e5381,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign5510_e5379: f64 = (p.p151).exp();
        (assign5510_e5379,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5510_e5381;
        locals.var_expl_rv = 0.0;

        let (assign5520_e5403, assign5520_e5403_d_n0, assign5520_e5403_d_n1, assign5520_e5403_d_n3, assign5520_e5403_d_n4, assign5520_e5403_d_n5, assign5520_e5403_d_n6, assign5520_e5403_d_n7, assign5520_e5403_d_n8, assign5520_e5403_d_n9, assign5520_e5403_d_n10, assign5520_e5403_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign5520_e5393: f64 = (-locals.var_bavl_t);
        let assign5520_e5396: f64 = (locals.var_vl).powf(p.p41);
        let assign5520_e5397: f64 = (assign5520_e5393 * assign5520_e5396);
        let assign5520_e5399: f64 = (assign5520_e5397 - p.p151);
        let assign5520_e5400: f64 = (1.0 + assign5520_e5399);
        let assign5520_e5401: f64 = (locals.var_expl * assign5520_e5400);
        (assign5520_e5401, (locals.var_expl * (((-locals.var_bavl_t_dn0) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn0)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn0 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn1)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn1 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn3)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn3 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn4)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn4 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn5)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn5 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn6)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn6 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn7)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn7 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn8)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn8 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn9)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn9 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn10)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn10 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn11) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn11)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn11 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5520_e5403;
        locals.var_expmm1_dn0 = assign5520_e5403_d_n0;
        locals.var_expmm1_dn1 = assign5520_e5403_d_n1;
        locals.var_expmm1_dn3 = assign5520_e5403_d_n3;
        locals.var_expmm1_dn4 = assign5520_e5403_d_n4;
        locals.var_expmm1_dn5 = assign5520_e5403_d_n5;
        locals.var_expmm1_dn6 = assign5520_e5403_d_n6;
        locals.var_expmm1_dn7 = assign5520_e5403_d_n7;
        locals.var_expmm1_dn8 = assign5520_e5403_d_n8;
        locals.var_expmm1_dn9 = assign5520_e5403_d_n9;
        locals.var_expmm1_dn10 = assign5520_e5403_d_n10;
        locals.var_expmm1_dn11 = assign5520_e5403_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5530_e5417, assign5530_e5417_d_n0, assign5530_e5417_d_n1, assign5530_e5417_d_n3, assign5530_e5417_d_n4, assign5530_e5417_d_n5, assign5530_e5417_d_n6, assign5530_e5417_d_n7, assign5530_e5417_d_n8, assign5530_e5417_d_n9, assign5530_e5417_d_n10, assign5530_e5417_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5530_e5411: f64 = (p.p40 / locals.var_bavl_t);
        let assign5530_e5413: f64 = (assign5530_e5411 * locals.var_vl);
        let assign5530_e5415: f64 = (assign5530_e5413 * locals.var_expmm1);
        (assign5530_e5415, (((((-((p.p40 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn0)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn0)), (((((-((p.p40 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn1)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn1)), (((((-((p.p40 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn3)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn3)), (((((-((p.p40 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn4)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn4)), (((((-((p.p40 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn5)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn5)), (((((-((p.p40 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn6)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn6)), (((((-((p.p40 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn7)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn7)), (((((-((p.p40 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn8)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn8)), (((((-((p.p40 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn9)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn9)), (((((-((p.p40 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn10)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn10)), (((((-((p.p40 * locals.var_bavl_t_dn11) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn11)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn11)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5530_e5417;
        locals.var_gem_dn0 = assign5530_e5417_d_n0;
        locals.var_gem_dn1 = assign5530_e5417_d_n1;
        locals.var_gem_dn3 = assign5530_e5417_d_n3;
        locals.var_gem_dn4 = assign5530_e5417_d_n4;
        locals.var_gem_dn5 = assign5530_e5417_d_n5;
        locals.var_gem_dn6 = assign5530_e5417_d_n6;
        locals.var_gem_dn7 = assign5530_e5417_d_n7;
        locals.var_gem_dn8 = assign5530_e5417_d_n8;
        locals.var_gem_dn9 = assign5530_e5417_d_n9;
        locals.var_gem_dn10 = assign5530_e5417_d_n10;
        locals.var_gem_dn11 = assign5530_e5417_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5540_e5420: f64 = if p.p39 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5540_e5420;
        locals.var_guard97_rv = 0.0;

        let assign5550_e5423: f64 = if locals.var_vb2c1 < locals.var_vdc_t { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5550_e5423;
        locals.var_guard98_rv = 0.0;

        let (assign5560_e5440,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5560_e5434: f64 = (2.0 * p.p46);
        let assign5560_e5437: f64 = (p.p45 * p.p45);
        let assign5560_e5438: f64 = (assign5560_e5434 / assign5560_e5437);
        (assign5560_e5438,)
    } else {
        (locals.var_dedx0,)
    }
};
        locals.var_dedx0 = assign5560_e5440;
        locals.var_dedx0_rv = 0.0;

        let (assign5570_e5455, assign5570_e5455_d_n0, assign5570_e5455_d_n1, assign5570_e5455_d_n3, assign5570_e5455_d_n4, assign5570_e5455_d_n5, assign5570_e5455_d_n6, assign5570_e5455_d_n7, assign5570_e5455_d_n8, assign5570_e5455_d_n9, assign5570_e5455_d_n10, assign5570_e5455_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5570_e5451: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5570_e5453: f64 = (assign5570_e5451 / locals.var_icap_ihc);
        (assign5570_e5453, (((locals.var_vdc_t_dn0 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn0)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn1 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn1)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn3 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn3)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn4 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn4)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn5 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn5)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn6 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn6)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn7)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn8 - locals.var_vb2c1_dn8) * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn8)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn9 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn9)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn10 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn10)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn11 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn11)) / (locals.var_icap_ihc * locals.var_icap_ihc)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10, locals.var_sqr_arg_dn11,)
    }
};
        locals.var_sqr_arg = assign5570_e5455;
        locals.var_sqr_arg_dn0 = assign5570_e5455_d_n0;
        locals.var_sqr_arg_dn1 = assign5570_e5455_d_n1;
        locals.var_sqr_arg_dn3 = assign5570_e5455_d_n3;
        locals.var_sqr_arg_dn4 = assign5570_e5455_d_n4;
        locals.var_sqr_arg_dn5 = assign5570_e5455_d_n5;
        locals.var_sqr_arg_dn6 = assign5570_e5455_d_n6;
        locals.var_sqr_arg_dn7 = assign5570_e5455_d_n7;
        locals.var_sqr_arg_dn8 = assign5570_e5455_d_n8;
        locals.var_sqr_arg_dn9 = assign5570_e5455_d_n9;
        locals.var_sqr_arg_dn10 = assign5570_e5455_d_n10;
        locals.var_sqr_arg_dn11 = assign5570_e5455_d_n11;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5580_e5471, assign5580_e5471_d_n0, assign5580_e5471_d_n1, assign5580_e5471_d_n3, assign5580_e5471_d_n4, assign5580_e5471_d_n5, assign5580_e5471_d_n6, assign5580_e5471_d_n7, assign5580_e5471_d_n8, assign5580_e5471_d_n9, assign5580_e5471_d_n10, assign5580_e5471_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5580_e5466: f64 = (2.0 * locals.var_sqr_arg);
        let assign5580_e5468: f64 = (assign5580_e5466 / locals.var_dedx0);
        let assign5580_e5469: f64 = (assign5580_e5468).sqrt();
        (assign5580_e5469, (((2.0 * locals.var_sqr_arg_dn0) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn1) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn3) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn4) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn5) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn6) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn7) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn8) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn9) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn10) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn11) / locals.var_dedx0) / (2.0 * assign5580_e5469)),)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn1, locals.var_xd_dn3, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11,)
    }
};
        locals.var_xd = assign5580_e5471;
        locals.var_xd_dn0 = assign5580_e5471_d_n0;
        locals.var_xd_dn1 = assign5580_e5471_d_n1;
        locals.var_xd_dn3 = assign5580_e5471_d_n3;
        locals.var_xd_dn4 = assign5580_e5471_d_n4;
        locals.var_xd_dn5 = assign5580_e5471_d_n5;
        locals.var_xd_dn6 = assign5580_e5471_d_n6;
        locals.var_xd_dn7 = assign5580_e5471_d_n7;
        locals.var_xd_dn8 = assign5580_e5471_d_n8;
        locals.var_xd_dn9 = assign5580_e5471_d_n9;
        locals.var_xd_dn10 = assign5580_e5471_d_n10;
        locals.var_xd_dn11 = assign5580_e5471_d_n11;
        locals.var_xd_rv = 0.0;

        let assign5590_e5474: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5590_e5474;
        locals.var_guard99_rv = 0.0;

        let (assign5600_e5487, assign5600_e5487_d_n0, assign5600_e5487_d_n1, assign5600_e5487_d_n3, assign5600_e5487_d_n4, assign5600_e5487_d_n5, assign5600_e5487_d_n6, assign5600_e5487_d_n7, assign5600_e5487_d_n8, assign5600_e5487_d_n9, assign5600_e5487_d_n10, assign5600_e5487_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        (p.p45, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10, locals.var_weff_dn11,)
    }
};
        locals.var_weff = assign5600_e5487;
        locals.var_weff_dn0 = assign5600_e5487_d_n0;
        locals.var_weff_dn1 = assign5600_e5487_d_n1;
        locals.var_weff_dn3 = assign5600_e5487_d_n3;
        locals.var_weff_dn4 = assign5600_e5487_d_n4;
        locals.var_weff_dn5 = assign5600_e5487_d_n5;
        locals.var_weff_dn6 = assign5600_e5487_d_n6;
        locals.var_weff_dn7 = assign5600_e5487_d_n7;
        locals.var_weff_dn8 = assign5600_e5487_d_n8;
        locals.var_weff_dn9 = assign5600_e5487_d_n9;
        locals.var_weff_dn10 = assign5600_e5487_d_n10;
        locals.var_weff_dn11 = assign5600_e5487_d_n11;
        locals.var_weff_rv = 0.0;

        let (assign5610_e5505, assign5610_e5505_d_n0, assign5610_e5505_d_n1, assign5610_e5505_d_n3, assign5610_e5505_d_n4, assign5610_e5505_d_n5, assign5610_e5505_d_n6, assign5610_e5505_d_n7, assign5610_e5505_d_n8, assign5610_e5505_d_n9, assign5610_e5505_d_n10, assign5610_e5505_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5610_e5502: f64 = (0.5 * locals.var_xi_w);
        let assign5610_e5503: f64 = (1.0 - assign5610_e5502);
        (assign5610_e5503, (-(0.5 * locals.var_xi_w_dn0)), (-(0.5 * locals.var_xi_w_dn1)), (-(0.5 * locals.var_xi_w_dn3)), (-(0.5 * locals.var_xi_w_dn4)), (-(0.5 * locals.var_xi_w_dn5)), (-(0.5 * locals.var_xi_w_dn6)), (-(0.5 * locals.var_xi_w_dn7)), (-(0.5 * locals.var_xi_w_dn8)), (-(0.5 * locals.var_xi_w_dn9)), (-(0.5 * locals.var_xi_w_dn10)), (-(0.5 * locals.var_xi_w_dn11)),)
    } else {
        (locals.var_xi_w1, locals.var_xi_w1_dn0, locals.var_xi_w1_dn1, locals.var_xi_w1_dn3, locals.var_xi_w1_dn4, locals.var_xi_w1_dn5, locals.var_xi_w1_dn6, locals.var_xi_w1_dn7, locals.var_xi_w1_dn8, locals.var_xi_w1_dn9, locals.var_xi_w1_dn10, locals.var_xi_w1_dn11,)
    }
};
        locals.var_xi_w1 = assign5610_e5505;
        locals.var_xi_w1_dn0 = assign5610_e5505_d_n0;
        locals.var_xi_w1_dn1 = assign5610_e5505_d_n1;
        locals.var_xi_w1_dn3 = assign5610_e5505_d_n3;
        locals.var_xi_w1_dn4 = assign5610_e5505_d_n4;
        locals.var_xi_w1_dn5 = assign5610_e5505_d_n5;
        locals.var_xi_w1_dn6 = assign5610_e5505_d_n6;
        locals.var_xi_w1_dn7 = assign5610_e5505_d_n7;
        locals.var_xi_w1_dn8 = assign5610_e5505_d_n8;
        locals.var_xi_w1_dn9 = assign5610_e5505_d_n9;
        locals.var_xi_w1_dn10 = assign5610_e5505_d_n10;
        locals.var_xi_w1_dn11 = assign5610_e5505_d_n11;
        locals.var_xi_w1_rv = 0.0;

        let (assign5620_e5523, assign5620_e5523_d_n0, assign5620_e5523_d_n1, assign5620_e5523_d_n3, assign5620_e5523_d_n4, assign5620_e5523_d_n5, assign5620_e5523_d_n6, assign5620_e5523_d_n7, assign5620_e5523_d_n8, assign5620_e5523_d_n9, assign5620_e5523_d_n10, assign5620_e5523_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5620_e5519: f64 = (p.p45 * locals.var_xi_w1);
        let assign5620_e5521: f64 = (assign5620_e5519 * locals.var_xi_w1);
        (assign5620_e5521, (((p.p45 * locals.var_xi_w1_dn0) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn0)), (((p.p45 * locals.var_xi_w1_dn1) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn1)), (((p.p45 * locals.var_xi_w1_dn3) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn3)), (((p.p45 * locals.var_xi_w1_dn4) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn4)), (((p.p45 * locals.var_xi_w1_dn5) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn5)), (((p.p45 * locals.var_xi_w1_dn6) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn6)), (((p.p45 * locals.var_xi_w1_dn7) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn7)), (((p.p45 * locals.var_xi_w1_dn8) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn8)), (((p.p45 * locals.var_xi_w1_dn9) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn9)), (((p.p45 * locals.var_xi_w1_dn10) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn10)), (((p.p45 * locals.var_xi_w1_dn11) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn11)),)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10, locals.var_weff_dn11,)
    }
};
        locals.var_weff = assign5620_e5523;
        locals.var_weff_dn0 = assign5620_e5523_d_n0;
        locals.var_weff_dn1 = assign5620_e5523_d_n1;
        locals.var_weff_dn3 = assign5620_e5523_d_n3;
        locals.var_weff_dn4 = assign5620_e5523_d_n4;
        locals.var_weff_dn5 = assign5620_e5523_d_n5;
        locals.var_weff_dn6 = assign5620_e5523_d_n6;
        locals.var_weff_dn7 = assign5620_e5523_d_n7;
        locals.var_weff_dn8 = assign5620_e5523_d_n8;
        locals.var_weff_dn9 = assign5620_e5523_d_n9;
        locals.var_weff_dn10 = assign5620_e5523_d_n10;
        locals.var_weff_dn11 = assign5620_e5523_d_n11;
        locals.var_weff_rv = 0.0;

        let (assign5630_e5545, assign5630_e5545_d_n0, assign5630_e5545_d_n1, assign5630_e5545_d_n3, assign5630_e5545_d_n4, assign5630_e5545_d_n5, assign5630_e5545_d_n6, assign5630_e5545_d_n7, assign5630_e5545_d_n8, assign5630_e5545_d_n9, assign5630_e5545_d_n10, assign5630_e5545_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5630_e5534: f64 = (locals.var_xd * locals.var_weff);
        let assign5630_e5537: f64 = (locals.var_xd * locals.var_xd);
        let assign5630_e5540: f64 = (locals.var_weff * locals.var_weff);
        let assign5630_e5541: f64 = (assign5630_e5537 + assign5630_e5540);
        let assign5630_e5542: f64 = (assign5630_e5541).sqrt();
        let assign5630_e5543: f64 = (assign5630_e5534 / assign5630_e5542);
        (assign5630_e5543, (((((locals.var_xd_dn0 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn0)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn0 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn0)) + ((locals.var_weff_dn0 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn0))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn1 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn1)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn1 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn1)) + ((locals.var_weff_dn1 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn1))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn3 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn3)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn3 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn3)) + ((locals.var_weff_dn3 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn3))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn4 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn4)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn4 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn4)) + ((locals.var_weff_dn4 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn4))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn5 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn5)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn5 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn5)) + ((locals.var_weff_dn5 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn5))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn6 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn6)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn6 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn6)) + ((locals.var_weff_dn6 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn6))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn7 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn7)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn7 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn7)) + ((locals.var_weff_dn7 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn7))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn8 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn8)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn8 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn8)) + ((locals.var_weff_dn8 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn8))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn9 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn9)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn9 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn9)) + ((locals.var_weff_dn9 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn9))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn10 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn10)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn10 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn10)) + ((locals.var_weff_dn10 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn10))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn11 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn11)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn11 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn11)) + ((locals.var_weff_dn11 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn11))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn1, locals.var_wd_dn3, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn11,)
    }
};
        locals.var_wd = assign5630_e5545;
        locals.var_wd_dn0 = assign5630_e5545_d_n0;
        locals.var_wd_dn1 = assign5630_e5545_d_n1;
        locals.var_wd_dn3 = assign5630_e5545_d_n3;
        locals.var_wd_dn4 = assign5630_e5545_d_n4;
        locals.var_wd_dn5 = assign5630_e5545_d_n5;
        locals.var_wd_dn6 = assign5630_e5545_d_n6;
        locals.var_wd_dn7 = assign5630_e5545_d_n7;
        locals.var_wd_dn8 = assign5630_e5545_d_n8;
        locals.var_wd_dn9 = assign5630_e5545_d_n9;
        locals.var_wd_dn10 = assign5630_e5545_d_n10;
        locals.var_wd_dn11 = assign5630_e5545_d_n11;
        locals.var_wd_rv = 0.0;

        let (assign5640_e5560, assign5640_e5560_d_n0, assign5640_e5560_d_n1, assign5640_e5560_d_n3, assign5640_e5560_d_n4, assign5640_e5560_d_n5, assign5640_e5560_d_n6, assign5640_e5560_d_n7, assign5640_e5560_d_n8, assign5640_e5560_d_n9, assign5640_e5560_d_n10, assign5640_e5560_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5640_e5556: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5640_e5558: f64 = (assign5640_e5556 / locals.var_wd);
        (assign5640_e5558, (((locals.var_vdc_t_dn0 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn0)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn1 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn1)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn3 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn3)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn4 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn4)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn5 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn5)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn6 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn6)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn7)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn8 - locals.var_vb2c1_dn8) * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn8)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn9 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn9)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn10 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn10)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn11 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn11)) / (locals.var_wd * locals.var_wd)),)
    } else {
        (locals.var_eav, locals.var_eav_dn0, locals.var_eav_dn1, locals.var_eav_dn3, locals.var_eav_dn4, locals.var_eav_dn5, locals.var_eav_dn6, locals.var_eav_dn7, locals.var_eav_dn8, locals.var_eav_dn9, locals.var_eav_dn10, locals.var_eav_dn11,)
    }
};
        locals.var_eav = assign5640_e5560;
        locals.var_eav_dn0 = assign5640_e5560_d_n0;
        locals.var_eav_dn1 = assign5640_e5560_d_n1;
        locals.var_eav_dn3 = assign5640_e5560_d_n3;
        locals.var_eav_dn4 = assign5640_e5560_d_n4;
        locals.var_eav_dn5 = assign5640_e5560_d_n5;
        locals.var_eav_dn6 = assign5640_e5560_d_n6;
        locals.var_eav_dn7 = assign5640_e5560_d_n7;
        locals.var_eav_dn8 = assign5640_e5560_d_n8;
        locals.var_eav_dn9 = assign5640_e5560_d_n9;
        locals.var_eav_dn10 = assign5640_e5560_d_n10;
        locals.var_eav_dn11 = assign5640_e5560_d_n11;
        locals.var_eav_rv = 0.0;

        let (assign5650_e5579, assign5650_e5579_d_n0, assign5650_e5579_d_n1, assign5650_e5579_d_n3, assign5650_e5579_d_n4, assign5650_e5579_d_n5, assign5650_e5579_d_n6, assign5650_e5579_d_n7, assign5650_e5579_d_n8, assign5650_e5579_d_n9, assign5650_e5579_d_n10, assign5650_e5579_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5650_e5572: f64 = (0.5 * locals.var_wd);
        let assign5650_e5574: f64 = (assign5650_e5572 * locals.var_dedx0);
        let assign5650_e5576: f64 = (assign5650_e5574 * locals.var_icap_ihc);
        let assign5650_e5577: f64 = (locals.var_eav + assign5650_e5576);
        (assign5650_e5577, (locals.var_eav_dn0 + ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn0))), (locals.var_eav_dn1 + ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn1))), (locals.var_eav_dn3 + ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn3))), (locals.var_eav_dn4 + ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn4))), (locals.var_eav_dn5 + ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn5))), (locals.var_eav_dn6 + ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn6))), (locals.var_eav_dn7 + ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn7))), (locals.var_eav_dn8 + ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn8))), (locals.var_eav_dn9 + ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn9))), (locals.var_eav_dn10 + ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn10))), (locals.var_eav_dn11 + ((((0.5 * locals.var_wd_dn11) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn11))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11,)
    }
};
        locals.var_e0 = assign5650_e5579;
        locals.var_e0_dn0 = assign5650_e5579_d_n0;
        locals.var_e0_dn1 = assign5650_e5579_d_n1;
        locals.var_e0_dn3 = assign5650_e5579_d_n3;
        locals.var_e0_dn4 = assign5650_e5579_d_n4;
        locals.var_e0_dn5 = assign5650_e5579_d_n5;
        locals.var_e0_dn6 = assign5650_e5579_d_n6;
        locals.var_e0_dn7 = assign5650_e5579_d_n7;
        locals.var_e0_dn8 = assign5650_e5579_d_n8;
        locals.var_e0_dn9 = assign5650_e5579_d_n9;
        locals.var_e0_dn10 = assign5650_e5579_d_n10;
        locals.var_e0_dn11 = assign5650_e5579_d_n11;
        locals.var_e0_rv = 0.0;

        let assign5660_e5582: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5660_e5582;
        locals.var_guard100_rv = 0.0;

        let (assign5670_e5595, assign5670_e5595_d_n0, assign5670_e5595_d_n1, assign5670_e5595_d_n3, assign5670_e5595_d_n4, assign5670_e5595_d_n5, assign5670_e5595_d_n6, assign5670_e5595_d_n7, assign5670_e5595_d_n8, assign5670_e5595_d_n9, assign5670_e5595_d_n10, assign5670_e5595_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11,)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign5670_e5595;
        locals.var_em_dn0 = assign5670_e5595_d_n0;
        locals.var_em_dn1 = assign5670_e5595_d_n1;
        locals.var_em_dn3 = assign5670_e5595_d_n3;
        locals.var_em_dn4 = assign5670_e5595_d_n4;
        locals.var_em_dn5 = assign5670_e5595_d_n5;
        locals.var_em_dn6 = assign5670_e5595_d_n6;
        locals.var_em_dn7 = assign5670_e5595_d_n7;
        locals.var_em_dn8 = assign5670_e5595_d_n8;
        locals.var_em_dn9 = assign5670_e5595_d_n9;
        locals.var_em_dn10 = assign5670_e5595_d_n10;
        locals.var_em_dn11 = assign5670_e5595_d_n11;
        locals.var_em_rv = 0.0;

        let (assign5680_e5619, assign5680_e5619_d_n0, assign5680_e5619_d_n1, assign5680_e5619_d_n3, assign5680_e5619_d_n4, assign5680_e5619_d_n5, assign5680_e5619_d_n6, assign5680_e5619_d_n7, assign5680_e5619_d_n8, assign5680_e5619_d_n9, assign5680_e5619_d_n10, assign5680_e5619_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5680_e5610: f64 = (2.0 * p.p47);
        let assign5680_e5614: f64 = (2.0 * locals.var_xi_w);
        let assign5680_e5615: f64 = (1.0 + assign5680_e5614);
        let assign5680_e5616: f64 = (assign5680_e5610 * assign5680_e5615);
        let assign5680_e5617: f64 = (1.0 + assign5680_e5616);
        (assign5680_e5617, (assign5680_e5610 * (2.0 * locals.var_xi_w_dn0)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn1)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn3)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn4)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn5)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn6)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn7)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn8)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn9)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn10)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn11)),)
    } else {
        (locals.var_shw, locals.var_shw_dn0, locals.var_shw_dn1, locals.var_shw_dn3, locals.var_shw_dn4, locals.var_shw_dn5, locals.var_shw_dn6, locals.var_shw_dn7, locals.var_shw_dn8, locals.var_shw_dn9, locals.var_shw_dn10, locals.var_shw_dn11,)
    }
};
        locals.var_shw = assign5680_e5619;
        locals.var_shw_dn0 = assign5680_e5619_d_n0;
        locals.var_shw_dn1 = assign5680_e5619_d_n1;
        locals.var_shw_dn3 = assign5680_e5619_d_n3;
        locals.var_shw_dn4 = assign5680_e5619_d_n4;
        locals.var_shw_dn5 = assign5680_e5619_d_n5;
        locals.var_shw_dn6 = assign5680_e5619_d_n6;
        locals.var_shw_dn7 = assign5680_e5619_d_n7;
        locals.var_shw_dn8 = assign5680_e5619_d_n8;
        locals.var_shw_dn9 = assign5680_e5619_d_n9;
        locals.var_shw_dn10 = assign5680_e5619_d_n10;
        locals.var_shw_dn11 = assign5680_e5619_d_n11;
        locals.var_shw_rv = 0.0;

        let (assign5690_e5641,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5690_e5633: f64 = (1.0 + p.p47);
        let assign5690_e5637: f64 = (2.0 * p.p47);
        let assign5690_e5638: f64 = (1.0 + assign5690_e5637);
        let assign5690_e5639: f64 = (assign5690_e5633 / assign5690_e5638);
        (assign5690_e5639,)
    } else {
        (locals.var_efi,)
    }
};
        locals.var_efi = assign5690_e5641;
        locals.var_efi_rv = 0.0;

        let (assign5700_e5669, assign5700_e5669_d_n0, assign5700_e5669_d_n1, assign5700_e5669_d_n3, assign5700_e5669_d_n4, assign5700_e5669_d_n5, assign5700_e5669_d_n6, assign5700_e5669_d_n7, assign5700_e5669_d_n8, assign5700_e5669_d_n9, assign5700_e5669_d_n10, assign5700_e5669_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5700_e5656: f64 = (0.5 * locals.var_wd);
        let assign5700_e5658: f64 = (assign5700_e5656 * locals.var_dedx0);
        let assign5700_e5663: f64 = (p.p62 * locals.var_shw);
        let assign5700_e5664: f64 = (locals.var_in_ / assign5700_e5663);
        let assign5700_e5665: f64 = (locals.var_efi - assign5700_e5664);
        let assign5700_e5666: f64 = (assign5700_e5658 * assign5700_e5665);
        let assign5700_e5667: f64 = (locals.var_eav - assign5700_e5666);
        (assign5700_e5667, (locals.var_eav_dn0 - ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn0 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn0))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn1 - ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn1 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn1))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn3 - ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn3 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn3))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn4 - ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn4 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn4))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn5 - ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn5 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn5))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn6 - ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn6 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn6))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn7 - ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn7 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn7))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn8 - ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn8 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn8))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn9 - ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn9 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn9))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn10 - ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn10 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn10))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn11 - ((((0.5 * locals.var_wd_dn11) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn11 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn11))) / (assign5700_e5663 * assign5700_e5663)))))),)
    } else {
        (locals.var_ew, locals.var_ew_dn0, locals.var_ew_dn1, locals.var_ew_dn3, locals.var_ew_dn4, locals.var_ew_dn5, locals.var_ew_dn6, locals.var_ew_dn7, locals.var_ew_dn8, locals.var_ew_dn9, locals.var_ew_dn10, locals.var_ew_dn11,)
    }
};
        locals.var_ew = assign5700_e5669;
        locals.var_ew_dn0 = assign5700_e5669_d_n0;
        locals.var_ew_dn1 = assign5700_e5669_d_n1;
        locals.var_ew_dn3 = assign5700_e5669_d_n3;
        locals.var_ew_dn4 = assign5700_e5669_d_n4;
        locals.var_ew_dn5 = assign5700_e5669_d_n5;
        locals.var_ew_dn6 = assign5700_e5669_d_n6;
        locals.var_ew_dn7 = assign5700_e5669_d_n7;
        locals.var_ew_dn8 = assign5700_e5669_d_n8;
        locals.var_ew_dn9 = assign5700_e5669_d_n9;
        locals.var_ew_dn10 = assign5700_e5669_d_n10;
        locals.var_ew_dn11 = assign5700_e5669_d_n11;
        locals.var_ew_rv = 0.0;

        let (assign5710_e5699, assign5710_e5699_d_n0, assign5710_e5699_d_n1, assign5710_e5699_d_n3, assign5710_e5699_d_n4, assign5710_e5699_d_n5, assign5710_e5699_d_n6, assign5710_e5699_d_n7, assign5710_e5699_d_n8, assign5710_e5699_d_n9, assign5710_e5699_d_n10, assign5710_e5699_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5710_e5683: f64 = (locals.var_ew - locals.var_e0);
        let assign5710_e5686: f64 = (locals.var_ew - locals.var_e0);
        let assign5710_e5687: f64 = (assign5710_e5683 * assign5710_e5686);
        let assign5710_e5690: f64 = (0.1 * locals.var_eav);
        let assign5710_e5692: f64 = (assign5710_e5690 * locals.var_eav);
        let assign5710_e5694: f64 = (assign5710_e5692 * locals.var_icap);
        let assign5710_e5696: f64 = (assign5710_e5694 / p.p62);
        let assign5710_e5697: f64 = (assign5710_e5687 + assign5710_e5696);
        (assign5710_e5697, ((((locals.var_ew_dn0 - locals.var_e0_dn0) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn0 - locals.var_e0_dn0))) + ((((((0.1 * locals.var_eav_dn0) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn0)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn0)) / p.p62)), ((((locals.var_ew_dn1 - locals.var_e0_dn1) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn1 - locals.var_e0_dn1))) + ((((((0.1 * locals.var_eav_dn1) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn1)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn1)) / p.p62)), ((((locals.var_ew_dn3 - locals.var_e0_dn3) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn3 - locals.var_e0_dn3))) + ((((((0.1 * locals.var_eav_dn3) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn3)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn3)) / p.p62)), ((((locals.var_ew_dn4 - locals.var_e0_dn4) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn4 - locals.var_e0_dn4))) + ((((((0.1 * locals.var_eav_dn4) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn4)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn4)) / p.p62)), ((((locals.var_ew_dn5 - locals.var_e0_dn5) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn5 - locals.var_e0_dn5))) + ((((((0.1 * locals.var_eav_dn5) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn5)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn5)) / p.p62)), ((((locals.var_ew_dn6 - locals.var_e0_dn6) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn6 - locals.var_e0_dn6))) + ((((((0.1 * locals.var_eav_dn6) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn6)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn6)) / p.p62)), ((((locals.var_ew_dn7 - locals.var_e0_dn7) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn7 - locals.var_e0_dn7))) + ((((((0.1 * locals.var_eav_dn7) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn7)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn7)) / p.p62)), ((((locals.var_ew_dn8 - locals.var_e0_dn8) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn8 - locals.var_e0_dn8))) + ((((((0.1 * locals.var_eav_dn8) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn8)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn8)) / p.p62)), ((((locals.var_ew_dn9 - locals.var_e0_dn9) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn9 - locals.var_e0_dn9))) + ((((((0.1 * locals.var_eav_dn9) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn9)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn9)) / p.p62)), ((((locals.var_ew_dn10 - locals.var_e0_dn10) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn10 - locals.var_e0_dn10))) + ((((((0.1 * locals.var_eav_dn10) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn10)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn10)) / p.p62)), ((((locals.var_ew_dn11 - locals.var_e0_dn11) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn11 - locals.var_e0_dn11))) + ((((((0.1 * locals.var_eav_dn11) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn11)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn11)) / p.p62)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10, locals.var_sqr_arg_dn11,)
    }
};
        locals.var_sqr_arg = assign5710_e5699;
        locals.var_sqr_arg_dn0 = assign5710_e5699_d_n0;
        locals.var_sqr_arg_dn1 = assign5710_e5699_d_n1;
        locals.var_sqr_arg_dn3 = assign5710_e5699_d_n3;
        locals.var_sqr_arg_dn4 = assign5710_e5699_d_n4;
        locals.var_sqr_arg_dn5 = assign5710_e5699_d_n5;
        locals.var_sqr_arg_dn6 = assign5710_e5699_d_n6;
        locals.var_sqr_arg_dn7 = assign5710_e5699_d_n7;
        locals.var_sqr_arg_dn8 = assign5710_e5699_d_n8;
        locals.var_sqr_arg_dn9 = assign5710_e5699_d_n9;
        locals.var_sqr_arg_dn10 = assign5710_e5699_d_n10;
        locals.var_sqr_arg_dn11 = assign5710_e5699_d_n11;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5720_e5720, assign5720_e5720_d_n0, assign5720_e5720_d_n1, assign5720_e5720_d_n3, assign5720_e5720_d_n4, assign5720_e5720_d_n5, assign5720_e5720_d_n6, assign5720_e5720_d_n7, assign5720_e5720_d_n8, assign5720_e5720_d_n9, assign5720_e5720_d_n10, assign5720_e5720_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5720_e5714: f64 = (locals.var_ew + locals.var_e0);
        let assign5720_e5716: f64 = (locals.var_sqr_arg).sqrt();
        let assign5720_e5717: f64 = (assign5720_e5714 + assign5720_e5716);
        let assign5720_e5718: f64 = (0.5 * assign5720_e5717);
        (assign5720_e5718, (0.5 * ((locals.var_ew_dn0 + locals.var_e0_dn0) + (locals.var_sqr_arg_dn0 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn1 + locals.var_e0_dn1) + (locals.var_sqr_arg_dn1 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn3 + locals.var_e0_dn3) + (locals.var_sqr_arg_dn3 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn4 + locals.var_e0_dn4) + (locals.var_sqr_arg_dn4 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn5 + locals.var_e0_dn5) + (locals.var_sqr_arg_dn5 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn6 + locals.var_e0_dn6) + (locals.var_sqr_arg_dn6 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn7 + locals.var_e0_dn7) + (locals.var_sqr_arg_dn7 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn8 + locals.var_e0_dn8) + (locals.var_sqr_arg_dn8 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn9 + locals.var_e0_dn9) + (locals.var_sqr_arg_dn9 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn10 + locals.var_e0_dn10) + (locals.var_sqr_arg_dn10 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn11 + locals.var_e0_dn11) + (locals.var_sqr_arg_dn11 / (2.0 * assign5720_e5716)))),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign5720_e5720;
        locals.var_em_dn0 = assign5720_e5720_d_n0;
        locals.var_em_dn1 = assign5720_e5720_d_n1;
        locals.var_em_dn3 = assign5720_e5720_d_n3;
        locals.var_em_dn4 = assign5720_e5720_d_n4;
        locals.var_em_dn5 = assign5720_e5720_d_n5;
        locals.var_em_dn6 = assign5720_e5720_d_n6;
        locals.var_em_dn7 = assign5720_e5720_d_n7;
        locals.var_em_dn8 = assign5720_e5720_d_n8;
        locals.var_em_dn9 = assign5720_e5720_d_n9;
        locals.var_em_dn10 = assign5720_e5720_d_n10;
        locals.var_em_dn11 = assign5720_e5720_d_n11;
        locals.var_em_rv = 0.0;

        let (assign5730_e5735, assign5730_e5735_d_n0, assign5730_e5735_d_n1, assign5730_e5735_d_n3, assign5730_e5735_d_n4, assign5730_e5735_d_n5, assign5730_e5735_d_n6, assign5730_e5735_d_n7, assign5730_e5735_d_n8, assign5730_e5735_d_n9, assign5730_e5735_d_n10, assign5730_e5735_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5730_e5731: f64 = (locals.var_em - locals.var_eav);
        let assign5730_e5733: f64 = (assign5730_e5731 / locals.var_em);
        (assign5730_e5733, ((((locals.var_em_dn0 - locals.var_eav_dn0) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn0)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn1 - locals.var_eav_dn1) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn1)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn3 - locals.var_eav_dn3) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn4 - locals.var_eav_dn4) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn5 - locals.var_eav_dn5) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn5)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn6 - locals.var_eav_dn6) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn6)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn7 - locals.var_eav_dn7) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn7)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn8 - locals.var_eav_dn8) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn8)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn9 - locals.var_eav_dn9) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn9)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn10 - locals.var_eav_dn10) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn10)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn11 - locals.var_eav_dn11) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn11)) / (locals.var_em * locals.var_em)),)
    } else {
        (locals.var_emeav_em, locals.var_emeav_em_dn0, locals.var_emeav_em_dn1, locals.var_emeav_em_dn3, locals.var_emeav_em_dn4, locals.var_emeav_em_dn5, locals.var_emeav_em_dn6, locals.var_emeav_em_dn7, locals.var_emeav_em_dn8, locals.var_emeav_em_dn9, locals.var_emeav_em_dn10, locals.var_emeav_em_dn11,)
    }
};
        locals.var_emeav_em = assign5730_e5735;
        locals.var_emeav_em_dn0 = assign5730_e5735_d_n0;
        locals.var_emeav_em_dn1 = assign5730_e5735_d_n1;
        locals.var_emeav_em_dn3 = assign5730_e5735_d_n3;
        locals.var_emeav_em_dn4 = assign5730_e5735_d_n4;
        locals.var_emeav_em_dn5 = assign5730_e5735_d_n5;
        locals.var_emeav_em_dn6 = assign5730_e5735_d_n6;
        locals.var_emeav_em_dn7 = assign5730_e5735_d_n7;
        locals.var_emeav_em_dn8 = assign5730_e5735_d_n8;
        locals.var_emeav_em_dn9 = assign5730_e5735_d_n9;
        locals.var_emeav_em_dn10 = assign5730_e5735_d_n10;
        locals.var_emeav_em_dn11 = assign5730_e5735_d_n11;
        locals.var_emeav_em_rv = 0.0;

        let assign5740_e5737: f64 = (locals.var_emeav_em).abs();
        let assign5740_e5739: f64 = if assign5740_e5737 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign5740_e5739;
        locals.var_guard101_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5750_e5756, assign5750_e5756_d_n0, assign5750_e5756_d_n1, assign5750_e5756_d_n3, assign5750_e5756_d_n4, assign5750_e5756_d_n5, assign5750_e5756_d_n6, assign5750_e5756_d_n7, assign5750_e5756_d_n8, assign5750_e5756_d_n9, assign5750_e5756_d_n10, assign5750_e5756_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5750_e5752: f64 = (0.5 * locals.var_wd);
        let assign5750_e5754: f64 = (assign5750_e5752 / locals.var_emeav_em);
        (assign5750_e5754, ((((0.5 * locals.var_wd_dn0) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn0)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn1) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn1)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn3) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn3)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn4) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn4)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn5) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn5)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn6) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn6)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn7) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn7)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn8) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn8)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn9) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn9)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn10) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn10)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn11) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn11)) / (locals.var_emeav_em * locals.var_emeav_em)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn0, locals.var_lambda_dn1, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10, locals.var_lambda_dn11,)
    }
};
        locals.var_lambda = assign5750_e5756;
        locals.var_lambda_dn0 = assign5750_e5756_d_n0;
        locals.var_lambda_dn1 = assign5750_e5756_d_n1;
        locals.var_lambda_dn3 = assign5750_e5756_d_n3;
        locals.var_lambda_dn4 = assign5750_e5756_d_n4;
        locals.var_lambda_dn5 = assign5750_e5756_d_n5;
        locals.var_lambda_dn6 = assign5750_e5756_d_n6;
        locals.var_lambda_dn7 = assign5750_e5756_d_n7;
        locals.var_lambda_dn8 = assign5750_e5756_d_n8;
        locals.var_lambda_dn9 = assign5750_e5756_d_n9;
        locals.var_lambda_dn10 = assign5750_e5756_d_n10;
        locals.var_lambda_dn11 = assign5750_e5756_d_n11;
        locals.var_lambda_rv = 0.0;

        let (assign5760_e5793, assign5760_e5793_d_n0, assign5760_e5793_d_n1, assign5760_e5793_d_n3, assign5760_e5793_d_n4, assign5760_e5793_d_n5, assign5760_e5793_d_n6, assign5760_e5793_d_n7, assign5760_e5793_d_n8, assign5760_e5793_d_n9, assign5760_e5793_d_n10, assign5760_e5793_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5760_e5769: f64 = (locals.var_an / locals.var_bnt);
        let assign5760_e5771: f64 = (assign5760_e5769 * locals.var_em);
        let assign5760_e5773: f64 = (assign5760_e5771 * locals.var_lambda);
        let assign5760_e5775: f64 = (-locals.var_bnt);
        let assign5760_e5777: f64 = (assign5760_e5775 / locals.var_em);
        let assign5760_e5778: f64 = (assign5760_e5777).exp();
        let assign5760_e5780: f64 = (-locals.var_bnt);
        let assign5760_e5782: f64 = (assign5760_e5780 / locals.var_em);
        let assign5760_e5786: f64 = (locals.var_weff / locals.var_lambda);
        let assign5760_e5787: f64 = (1.0 + assign5760_e5786);
        let assign5760_e5788: f64 = (assign5760_e5782 * assign5760_e5787);
        let assign5760_e5789: f64 = (assign5760_e5788).exp();
        let assign5760_e5790: f64 = (assign5760_e5778 - assign5760_e5789);
        let assign5760_e5791: f64 = (assign5760_e5773 * assign5760_e5790);
        (assign5760_e5791, (((((assign5760_e5769 * locals.var_em_dn0) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn0)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn0) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn0 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn0)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn1) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn1)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn1) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn1 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn1)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn3) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn3)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn3) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn3 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn3)) / (locals.var_lambda * locals.var_lambda)))))))), (((((((-((locals.var_an * locals.var_bnt_dn4) / (locals.var_bnt * locals.var_bnt))) * locals.var_em) + (assign5760_e5769 * locals.var_em_dn4)) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn4)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * ((((-locals.var_bnt_dn4) * locals.var_em) - (assign5760_e5775 * locals.var_em_dn4)) / (locals.var_em * locals.var_em))) - (assign5760_e5789 * ((((((-locals.var_bnt_dn4) * locals.var_em) - (assign5760_e5780 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn4 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn4)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn5) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn5)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn5) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn5 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn5)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn6) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn6)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn6) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn6 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn6)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn7) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn7)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn7) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn7 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn7)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn8) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn8)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn8) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn8 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn8)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn9) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn9)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn9) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn9 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn9)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn10) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn10)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn10) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn10 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn10)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn11) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn11)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn11) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn11) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn11 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn11)) / (locals.var_lambda * locals.var_lambda)))))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5760_e5793;
        locals.var_gem_dn0 = assign5760_e5793_d_n0;
        locals.var_gem_dn1 = assign5760_e5793_d_n1;
        locals.var_gem_dn3 = assign5760_e5793_d_n3;
        locals.var_gem_dn4 = assign5760_e5793_d_n4;
        locals.var_gem_dn5 = assign5760_e5793_d_n5;
        locals.var_gem_dn6 = assign5760_e5793_d_n6;
        locals.var_gem_dn7 = assign5760_e5793_d_n7;
        locals.var_gem_dn8 = assign5760_e5793_d_n8;
        locals.var_gem_dn9 = assign5760_e5793_d_n9;
        locals.var_gem_dn10 = assign5760_e5793_d_n10;
        locals.var_gem_dn11 = assign5760_e5793_d_n11;
        locals.var_gem_rv = 0.0;

        let (assign5770_e5815, assign5770_e5815_d_n0, assign5770_e5815_d_n1, assign5770_e5815_d_n3, assign5770_e5815_d_n4, assign5770_e5815_d_n5, assign5770_e5815_d_n6, assign5770_e5815_d_n7, assign5770_e5815_d_n8, assign5770_e5815_d_n9, assign5770_e5815_d_n10, assign5770_e5815_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign5770_e5807: f64 = (locals.var_an * locals.var_weff);
        let assign5770_e5809: f64 = (-locals.var_bnt);
        let assign5770_e5811: f64 = (assign5770_e5809 / locals.var_em);
        let assign5770_e5812: f64 = (assign5770_e5811).exp();
        let assign5770_e5813: f64 = (assign5770_e5807 * assign5770_e5812);
        (assign5770_e5813, (((locals.var_an * locals.var_weff_dn0) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn1) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn3) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn4) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * ((((-locals.var_bnt_dn4) * locals.var_em) - (assign5770_e5809 * locals.var_em_dn4)) / (locals.var_em * locals.var_em))))), (((locals.var_an * locals.var_weff_dn5) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn6) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn7) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn8) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn9) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn10) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn11) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn11) / (locals.var_em * locals.var_em)))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5770_e5815;
        locals.var_gem_dn0 = assign5770_e5815_d_n0;
        locals.var_gem_dn1 = assign5770_e5815_d_n1;
        locals.var_gem_dn3 = assign5770_e5815_d_n3;
        locals.var_gem_dn4 = assign5770_e5815_d_n4;
        locals.var_gem_dn5 = assign5770_e5815_d_n5;
        locals.var_gem_dn6 = assign5770_e5815_d_n6;
        locals.var_gem_dn7 = assign5770_e5815_d_n7;
        locals.var_gem_dn8 = assign5770_e5815_d_n8;
        locals.var_gem_dn9 = assign5770_e5815_d_n9;
        locals.var_gem_dn10 = assign5770_e5815_d_n10;
        locals.var_gem_dn11 = assign5770_e5815_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5780_e5818: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5780_e5818;
        locals.var_guard102_rv = 0.0;

        let assign5790_e5821: f64 = if locals.var_vb2c1 < p.p44 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5790_e5821;
        locals.var_guard103_rv = 0.0;

        let (assign5800_e5849, assign5800_e5849_d_n0, assign5800_e5849_d_n1, assign5800_e5849_d_n3, assign5800_e5849_d_n4, assign5800_e5849_d_n5, assign5800_e5849_d_n6, assign5800_e5849_d_n7, assign5800_e5849_d_n8, assign5800_e5849_d_n9, assign5800_e5849_d_n10, assign5800_e5849_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5800_e5835: f64 = (p.p44 - locals.var_vb2c1);
        let assign5800_e5837: f64 = (assign5800_e5835).powf(p.p41);
        let assign5800_e5842: f64 = (p.p48 + locals.var_in_);
        let assign5800_e5843: f64 = (locals.var_in_ / assign5800_e5842);
        let assign5800_e5844: f64 = (1.0 - assign5800_e5843);
        let assign5800_e5846: f64 = (assign5800_e5844).powf(p.p49);
        let assign5800_e5847: f64 = (assign5800_e5837 * assign5800_e5846);
        (assign5800_e5847, (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn0 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn0)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn0 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn0)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn1 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn1)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn1 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn1)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn3 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn3)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn3 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn3)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn4 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn4)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn4 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn4)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn5 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn5)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn5 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn5)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn6 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn6)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn6 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn6)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), ((if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((assign5800_e5835).powf(p.p41 - 1.0) * (-locals.var_vb2c1_dn7))) } } else { (assign5800_e5837 * (p.p41 * ((-locals.var_vb2c1_dn7) / assign5800_e5835))) } * assign5800_e5846) + (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn7 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn7)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn7 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn7)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) })), ((if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((assign5800_e5835).powf(p.p41 - 1.0) * (-locals.var_vb2c1_dn8))) } } else { (assign5800_e5837 * (p.p41 * ((-locals.var_vb2c1_dn8) / assign5800_e5835))) } * assign5800_e5846) + (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn8 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn8)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn8 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn8)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) })), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn9 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn9)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn9 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn9)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn10 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn10)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn10 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn10)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn11 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn11)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn11 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn11)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }),)
    } else {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10, locals.var_vdeptmp_dn11,)
    }
};
        locals.var_vdeptmp = assign5800_e5849;
        locals.var_vdeptmp_dn0 = assign5800_e5849_d_n0;
        locals.var_vdeptmp_dn1 = assign5800_e5849_d_n1;
        locals.var_vdeptmp_dn3 = assign5800_e5849_d_n3;
        locals.var_vdeptmp_dn4 = assign5800_e5849_d_n4;
        locals.var_vdeptmp_dn5 = assign5800_e5849_d_n5;
        locals.var_vdeptmp_dn6 = assign5800_e5849_d_n6;
        locals.var_vdeptmp_dn7 = assign5800_e5849_d_n7;
        locals.var_vdeptmp_dn8 = assign5800_e5849_d_n8;
        locals.var_vdeptmp_dn9 = assign5800_e5849_d_n9;
        locals.var_vdeptmp_dn10 = assign5800_e5849_d_n10;
        locals.var_vdeptmp_dn11 = assign5800_e5849_d_n11;
        locals.var_vdeptmp_rv = 0.0;

        let assign5810_e5852: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5810_e5852;
        locals.var_guard104_rv = 0.0;

        let (assign5820_e5868, assign5820_e5868_d_n0, assign5820_e5868_d_n1, assign5820_e5868_d_n3, assign5820_e5868_d_n4, assign5820_e5868_d_n5, assign5820_e5868_d_n6, assign5820_e5868_d_n7, assign5820_e5868_d_n8, assign5820_e5868_d_n9, assign5820_e5868_d_n10, assign5820_e5868_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 != 0.0)) {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10, locals.var_vdeptmp_dn11,)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10, locals.var_vdep_dn11,)
    }
};
        locals.var_vdep = assign5820_e5868;
        locals.var_vdep_dn0 = assign5820_e5868_d_n0;
        locals.var_vdep_dn1 = assign5820_e5868_d_n1;
        locals.var_vdep_dn3 = assign5820_e5868_d_n3;
        locals.var_vdep_dn4 = assign5820_e5868_d_n4;
        locals.var_vdep_dn5 = assign5820_e5868_d_n5;
        locals.var_vdep_dn6 = assign5820_e5868_d_n6;
        locals.var_vdep_dn7 = assign5820_e5868_d_n7;
        locals.var_vdep_dn8 = assign5820_e5868_d_n8;
        locals.var_vdep_dn9 = assign5820_e5868_d_n9;
        locals.var_vdep_dn10 = assign5820_e5868_d_n10;
        locals.var_vdep_dn11 = assign5820_e5868_d_n11;
        locals.var_vdep_rv = 0.0;

        let (assign5830_e5889, assign5830_e5889_d_n0, assign5830_e5889_d_n1, assign5830_e5889_d_n3, assign5830_e5889_d_n4, assign5830_e5889_d_n5, assign5830_e5889_d_n6, assign5830_e5889_d_n7, assign5830_e5889_d_n8, assign5830_e5889_d_n9, assign5830_e5889_d_n10, assign5830_e5889_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5830_e5885: f64 = (locals.var_in_ - p.p52);
        let assign5830_e5887: f64 = (assign5830_e5885 / p.p48);
        (assign5830_e5887, (locals.var_in__dn0 / p.p48), (locals.var_in__dn1 / p.p48), (locals.var_in__dn3 / p.p48), (locals.var_in__dn4 / p.p48), (locals.var_in__dn5 / p.p48), (locals.var_in__dn6 / p.p48), (locals.var_in__dn7 / p.p48), (locals.var_in__dn8 / p.p48), (locals.var_in__dn9 / p.p48), (locals.var_in__dn10 / p.p48), (locals.var_in__dn11 / p.p48),)
    } else {
        (locals.var_in_shift_ihcavl, locals.var_in_shift_ihcavl_dn0, locals.var_in_shift_ihcavl_dn1, locals.var_in_shift_ihcavl_dn3, locals.var_in_shift_ihcavl_dn4, locals.var_in_shift_ihcavl_dn5, locals.var_in_shift_ihcavl_dn6, locals.var_in_shift_ihcavl_dn7, locals.var_in_shift_ihcavl_dn8, locals.var_in_shift_ihcavl_dn9, locals.var_in_shift_ihcavl_dn10, locals.var_in_shift_ihcavl_dn11,)
    }
};
        locals.var_in_shift_ihcavl = assign5830_e5889;
        locals.var_in_shift_ihcavl_dn0 = assign5830_e5889_d_n0;
        locals.var_in_shift_ihcavl_dn1 = assign5830_e5889_d_n1;
        locals.var_in_shift_ihcavl_dn3 = assign5830_e5889_d_n3;
        locals.var_in_shift_ihcavl_dn4 = assign5830_e5889_d_n4;
        locals.var_in_shift_ihcavl_dn5 = assign5830_e5889_d_n5;
        locals.var_in_shift_ihcavl_dn6 = assign5830_e5889_d_n6;
        locals.var_in_shift_ihcavl_dn7 = assign5830_e5889_d_n7;
        locals.var_in_shift_ihcavl_dn8 = assign5830_e5889_d_n8;
        locals.var_in_shift_ihcavl_dn9 = assign5830_e5889_d_n9;
        locals.var_in_shift_ihcavl_dn10 = assign5830_e5889_d_n10;
        locals.var_in_shift_ihcavl_dn11 = assign5830_e5889_d_n11;
        locals.var_in_shift_ihcavl_rv = 0.0;

        let (assign5840_e5910, assign5840_e5910_d_n0, assign5840_e5910_d_n1, assign5840_e5910_d_n3, assign5840_e5910_d_n4, assign5840_e5910_d_n5, assign5840_e5910_d_n6, assign5840_e5910_d_n7, assign5840_e5910_d_n8, assign5840_e5910_d_n9, assign5840_e5910_d_n10, assign5840_e5910_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5840_e5906: f64 = (locals.var_in_shift_ihcavl - 1.0);
        let assign5840_e5908: f64 = (assign5840_e5906 / p.p51);
        (assign5840_e5908, (locals.var_in_shift_ihcavl_dn0 / p.p51), (locals.var_in_shift_ihcavl_dn1 / p.p51), (locals.var_in_shift_ihcavl_dn3 / p.p51), (locals.var_in_shift_ihcavl_dn4 / p.p51), (locals.var_in_shift_ihcavl_dn5 / p.p51), (locals.var_in_shift_ihcavl_dn6 / p.p51), (locals.var_in_shift_ihcavl_dn7 / p.p51), (locals.var_in_shift_ihcavl_dn8 / p.p51), (locals.var_in_shift_ihcavl_dn9 / p.p51), (locals.var_in_shift_ihcavl_dn10 / p.p51), (locals.var_in_shift_ihcavl_dn11 / p.p51),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign5840_e5910;
        locals.var_dxa_dn0 = assign5840_e5910_d_n0;
        locals.var_dxa_dn1 = assign5840_e5910_d_n1;
        locals.var_dxa_dn3 = assign5840_e5910_d_n3;
        locals.var_dxa_dn4 = assign5840_e5910_d_n4;
        locals.var_dxa_dn5 = assign5840_e5910_d_n5;
        locals.var_dxa_dn6 = assign5840_e5910_d_n6;
        locals.var_dxa_dn7 = assign5840_e5910_d_n7;
        locals.var_dxa_dn8 = assign5840_e5910_d_n8;
        locals.var_dxa_dn9 = assign5840_e5910_d_n9;
        locals.var_dxa_dn10 = assign5840_e5910_d_n10;
        locals.var_dxa_dn11 = assign5840_e5910_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign5850_e5913: f64 = if locals.var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5850_e5913;
        locals.var_guard105_rv = 0.0;

        let (assign5860_e5940, assign5860_e5940_d_n0, assign5860_e5940_d_n1, assign5860_e5940_d_n3, assign5860_e5940_d_n4, assign5860_e5940_d_n5, assign5860_e5940_d_n6, assign5860_e5940_d_n7, assign5860_e5940_d_n8, assign5860_e5940_d_n9, assign5860_e5940_d_n10, assign5860_e5940_d_n11,) = {
    if (((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) && (locals.var_guard105 != 0.0)) {
        let assign5860_e5934: f64 = (locals.var_dxa).exp();
        let assign5860_e5935: f64 = (1.0 + assign5860_e5934);
        let assign5860_e5936: f64 = (assign5860_e5935).ln();
        let assign5860_e5937: f64 = (p.p51 * assign5860_e5936);
        let assign5860_e5938: f64 = (1.0 + assign5860_e5937);
        (assign5860_e5938, (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn0) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn1) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn3) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn4) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn5) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn6) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn7) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn8) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn9) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn10) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn11) / assign5860_e5935)),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10, locals.var_in_shift_n_dn11,)
    }
};
        locals.var_in_shift_n = assign5860_e5940;
        locals.var_in_shift_n_dn0 = assign5860_e5940_d_n0;
        locals.var_in_shift_n_dn1 = assign5860_e5940_d_n1;
        locals.var_in_shift_n_dn3 = assign5860_e5940_d_n3;
        locals.var_in_shift_n_dn4 = assign5860_e5940_d_n4;
        locals.var_in_shift_n_dn5 = assign5860_e5940_d_n5;
        locals.var_in_shift_n_dn6 = assign5860_e5940_d_n6;
        locals.var_in_shift_n_dn7 = assign5860_e5940_d_n7;
        locals.var_in_shift_n_dn8 = assign5860_e5940_d_n8;
        locals.var_in_shift_n_dn9 = assign5860_e5940_d_n9;
        locals.var_in_shift_n_dn10 = assign5860_e5940_d_n10;
        locals.var_in_shift_n_dn11 = assign5860_e5940_d_n11;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5870_e5969, assign5870_e5969_d_n0, assign5870_e5969_d_n1, assign5870_e5969_d_n3, assign5870_e5969_d_n4, assign5870_e5969_d_n5, assign5870_e5969_d_n6, assign5870_e5969_d_n7, assign5870_e5969_d_n8, assign5870_e5969_d_n9, assign5870_e5969_d_n10, assign5870_e5969_d_n11,) = {
    if (((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign5870_e5962: f64 = (-locals.var_dxa);
        let assign5870_e5963: f64 = (assign5870_e5962).exp();
        let assign5870_e5964: f64 = (1.0 + assign5870_e5963);
        let assign5870_e5965: f64 = (assign5870_e5964).ln();
        let assign5870_e5966: f64 = (p.p51 * assign5870_e5965);
        let assign5870_e5967: f64 = (locals.var_in_shift_ihcavl + assign5870_e5966);
        (assign5870_e5967, (locals.var_in_shift_ihcavl_dn0 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn0)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn1 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn1)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn3 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn3)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn4 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn4)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn5 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn5)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn6 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn6)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn7 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn7)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn8 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn8)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn9 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn9)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn10 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn10)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn11 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn11)) / assign5870_e5964))),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10, locals.var_in_shift_n_dn11,)
    }
};
        locals.var_in_shift_n = assign5870_e5969;
        locals.var_in_shift_n_dn0 = assign5870_e5969_d_n0;
        locals.var_in_shift_n_dn1 = assign5870_e5969_d_n1;
        locals.var_in_shift_n_dn3 = assign5870_e5969_d_n3;
        locals.var_in_shift_n_dn4 = assign5870_e5969_d_n4;
        locals.var_in_shift_n_dn5 = assign5870_e5969_d_n5;
        locals.var_in_shift_n_dn6 = assign5870_e5969_d_n6;
        locals.var_in_shift_n_dn7 = assign5870_e5969_d_n7;
        locals.var_in_shift_n_dn8 = assign5870_e5969_d_n8;
        locals.var_in_shift_n_dn9 = assign5870_e5969_d_n9;
        locals.var_in_shift_n_dn10 = assign5870_e5969_d_n10;
        locals.var_in_shift_n_dn11 = assign5870_e5969_d_n11;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5880_e5990, assign5880_e5990_d_n0, assign5880_e5990_d_n1, assign5880_e5990_d_n3, assign5880_e5990_d_n4, assign5880_e5990_d_n5, assign5880_e5990_d_n6, assign5880_e5990_d_n7, assign5880_e5990_d_n8, assign5880_e5990_d_n9, assign5880_e5990_d_n10, assign5880_e5990_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5880_e5987: f64 = (locals.var_in_shift_n).powf(p.p50);
        let assign5880_e5988: f64 = (locals.var_vdeptmp * assign5880_e5987);
        (assign5880_e5988, ((locals.var_vdeptmp_dn0 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn0)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn0 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn1 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn1)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn1 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn3 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn3)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn3 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn4 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn4)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn4 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn5 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn5)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn5 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn6 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn6)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn6 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn7 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn7)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn7 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn8 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn8)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn8 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn9 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn9)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn9 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn10 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn10)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn10 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn11 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn11)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn11 / locals.var_in_shift_n))) })),)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10, locals.var_vdep_dn11,)
    }
};
        locals.var_vdep = assign5880_e5990;
        locals.var_vdep_dn0 = assign5880_e5990_d_n0;
        locals.var_vdep_dn1 = assign5880_e5990_d_n1;
        locals.var_vdep_dn3 = assign5880_e5990_d_n3;
        locals.var_vdep_dn4 = assign5880_e5990_d_n4;
        locals.var_vdep_dn5 = assign5880_e5990_d_n5;
        locals.var_vdep_dn6 = assign5880_e5990_d_n6;
        locals.var_vdep_dn7 = assign5880_e5990_d_n7;
        locals.var_vdep_dn8 = assign5880_e5990_d_n8;
        locals.var_vdep_dn9 = assign5880_e5990_d_n9;
        locals.var_vdep_dn10 = assign5880_e5990_d_n10;
        locals.var_vdep_dn11 = assign5880_e5990_d_n11;
        locals.var_vdep_rv = 0.0;

        let assign5890_e5992: f64 = (-locals.var_bavl_t);
        let assign5890_e5994: f64 = (assign5890_e5992 * locals.var_vdep);
        let assign5890_e5996: f64 = if assign5890_e5994 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign5890_e5996;
        locals.var_guard106_rv = 0.0;

        let (assign5900_e6016, assign5900_e6016_d_n0, assign5900_e6016_d_n1, assign5900_e6016_d_n3, assign5900_e6016_d_n4, assign5900_e6016_d_n5, assign5900_e6016_d_n6, assign5900_e6016_d_n7, assign5900_e6016_d_n8, assign5900_e6016_d_n9, assign5900_e6016_d_n10, assign5900_e6016_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 != 0.0)) {
        let assign5900_e6011: f64 = (-locals.var_bavl_t);
        let assign5900_e6013: f64 = (assign5900_e6011 * locals.var_vdep);
        let assign5900_e6014: f64 = (assign5900_e6013).exp();
        (assign5900_e6014, (assign5900_e6014 * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn0))), (assign5900_e6014 * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn1))), (assign5900_e6014 * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn3))), (assign5900_e6014 * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn4))), (assign5900_e6014 * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn5))), (assign5900_e6014 * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn6))), (assign5900_e6014 * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn7))), (assign5900_e6014 * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn8))), (assign5900_e6014 * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn9))), (assign5900_e6014 * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn10))), (assign5900_e6014 * (((-locals.var_bavl_t_dn11) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn11))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5900_e6016;
        locals.var_expmm1_dn0 = assign5900_e6016_d_n0;
        locals.var_expmm1_dn1 = assign5900_e6016_d_n1;
        locals.var_expmm1_dn3 = assign5900_e6016_d_n3;
        locals.var_expmm1_dn4 = assign5900_e6016_d_n4;
        locals.var_expmm1_dn5 = assign5900_e6016_d_n5;
        locals.var_expmm1_dn6 = assign5900_e6016_d_n6;
        locals.var_expmm1_dn7 = assign5900_e6016_d_n7;
        locals.var_expmm1_dn8 = assign5900_e6016_d_n8;
        locals.var_expmm1_dn9 = assign5900_e6016_d_n9;
        locals.var_expmm1_dn10 = assign5900_e6016_d_n10;
        locals.var_expmm1_dn11 = assign5900_e6016_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5910_e6034,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign5910_e6032: f64 = (p.p151).exp();
        (assign5910_e6032,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5910_e6034;
        locals.var_expl_rv = 0.0;

        let (assign5920_e6060, assign5920_e6060_d_n0, assign5920_e6060_d_n1, assign5920_e6060_d_n3, assign5920_e6060_d_n4, assign5920_e6060_d_n5, assign5920_e6060_d_n6, assign5920_e6060_d_n7, assign5920_e6060_d_n8, assign5920_e6060_d_n9, assign5920_e6060_d_n10, assign5920_e6060_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign5920_e6052: f64 = (-locals.var_bavl_t);
        let assign5920_e6054: f64 = (assign5920_e6052 * locals.var_vdep);
        let assign5920_e6056: f64 = (assign5920_e6054 - p.p151);
        let assign5920_e6057: f64 = (1.0 + assign5920_e6056);
        let assign5920_e6058: f64 = (locals.var_expl * assign5920_e6057);
        (assign5920_e6058, (locals.var_expl * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn0))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn1))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn3))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn4))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn5))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn6))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn7))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn8))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn9))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn10))), (locals.var_expl * (((-locals.var_bavl_t_dn11) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn11))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5920_e6060;
        locals.var_expmm1_dn0 = assign5920_e6060_d_n0;
        locals.var_expmm1_dn1 = assign5920_e6060_d_n1;
        locals.var_expmm1_dn3 = assign5920_e6060_d_n3;
        locals.var_expmm1_dn4 = assign5920_e6060_d_n4;
        locals.var_expmm1_dn5 = assign5920_e6060_d_n5;
        locals.var_expmm1_dn6 = assign5920_e6060_d_n6;
        locals.var_expmm1_dn7 = assign5920_e6060_d_n7;
        locals.var_expmm1_dn8 = assign5920_e6060_d_n8;
        locals.var_expmm1_dn9 = assign5920_e6060_d_n9;
        locals.var_expmm1_dn10 = assign5920_e6060_d_n10;
        locals.var_expmm1_dn11 = assign5920_e6060_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5930_e6082, assign5930_e6082_d_n0, assign5930_e6082_d_n1, assign5930_e6082_d_n3, assign5930_e6082_d_n4, assign5930_e6082_d_n5, assign5930_e6082_d_n6, assign5930_e6082_d_n7, assign5930_e6082_d_n8, assign5930_e6082_d_n9, assign5930_e6082_d_n10, assign5930_e6082_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5930_e6074: f64 = (p.p40 / locals.var_bavl_t);
        let assign5930_e6077: f64 = (p.p44 - locals.var_vb2c1);
        let assign5930_e6078: f64 = (assign5930_e6074 * assign5930_e6077);
        let assign5930_e6080: f64 = (assign5930_e6078 * locals.var_expmm1);
        (assign5930_e6080, ((((-((p.p40 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn0)), ((((-((p.p40 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn1)), ((((-((p.p40 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn3)), ((((-((p.p40 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn4)), ((((-((p.p40 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn5)), ((((-((p.p40 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn6)), (((((-((p.p40 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) + (assign5930_e6074 * (-locals.var_vb2c1_dn7))) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn7)), (((((-((p.p40 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) + (assign5930_e6074 * (-locals.var_vb2c1_dn8))) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn8)), ((((-((p.p40 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn9)), ((((-((p.p40 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn10)), ((((-((p.p40 * locals.var_bavl_t_dn11) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn11)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5930_e6082;
        locals.var_gem_dn0 = assign5930_e6082_d_n0;
        locals.var_gem_dn1 = assign5930_e6082_d_n1;
        locals.var_gem_dn3 = assign5930_e6082_d_n3;
        locals.var_gem_dn4 = assign5930_e6082_d_n4;
        locals.var_gem_dn5 = assign5930_e6082_d_n5;
        locals.var_gem_dn6 = assign5930_e6082_d_n6;
        locals.var_gem_dn7 = assign5930_e6082_d_n7;
        locals.var_gem_dn8 = assign5930_e6082_d_n8;
        locals.var_gem_dn9 = assign5930_e6082_d_n9;
        locals.var_gem_dn10 = assign5930_e6082_d_n10;
        locals.var_gem_dn11 = assign5930_e6082_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5940_e6085: f64 = if locals.var_gem > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5940_e6085;
        locals.var_guard107_rv = 0.0;

        let assign5950_e6088: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5950_e6088;
        locals.var_guard108_rv = 0.0;

        let (assign5960_e6114, assign5960_e6114_d_n0, assign5960_e6114_d_n1, assign5960_e6114_d_n3, assign5960_e6114_d_n4, assign5960_e6114_d_n5, assign5960_e6114_d_n6, assign5960_e6114_d_n7, assign5960_e6114_d_n8, assign5960_e6114_d_n9, assign5960_e6114_d_n10, assign5960_e6114_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) {
        let assign5960_e6098: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5960_e6099: f64 = (locals.var_in_ * assign5960_e6098);
        let assign5960_e6100: f64 = (locals.var_vt / assign5960_e6099);
        let assign5960_e6103: f64 = (locals.var_qbi / locals.var_is_t);
        let assign5960_e6105: f64 = (assign5960_e6103 * locals.var_ibi_t);
        let assign5960_e6106: f64 = (assign5960_e6100 + assign5960_e6105);
        let assign5960_e6110: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5960_e6111: f64 = (locals.var_re_t / assign5960_e6110);
        let assign5960_e6112: f64 = (assign5960_e6106 + assign5960_e6111);
        (assign5960_e6112, (((-((locals.var_vt * ((locals.var_in__dn0 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn0))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn0 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn0)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn0) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn1 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn1))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn1 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn1)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn1) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn3 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn3))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn3 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn3)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn3) / (assign5960_e6110 * assign5960_e6110)))), (((((locals.var_vt_dn4 * assign5960_e6099) - (locals.var_vt * ((locals.var_in__dn4 * assign5960_e6098) + (locals.var_in_ * (locals.var_rbc_t_dn4 + locals.var_rb2_dn4))))) / (assign5960_e6099 * assign5960_e6099)) + (((((locals.var_qbi_dn4 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn4)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t) + (assign5960_e6103 * locals.var_ibi_t_dn4))) + (((locals.var_re_t_dn4 * assign5960_e6110) - (locals.var_re_t * (locals.var_rbc_t_dn4 + locals.var_rb2_dn4))) / (assign5960_e6110 * assign5960_e6110))), (((-((locals.var_vt * ((locals.var_in__dn5 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn5))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn5 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn5)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn5) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn6 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn6))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn6 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn6)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn6) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn7 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn7))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn7 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn7)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn7) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn8 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn8))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn8 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn8)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn8) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn9 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn9))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn9 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn9)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn9) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn10 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn10))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn10 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn10)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn10) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn11 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn11))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn11 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn11)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn11) / (assign5960_e6110 * assign5960_e6110)))),)
    } else {
        (locals.var_gmax, locals.var_gmax_dn0, locals.var_gmax_dn1, locals.var_gmax_dn3, locals.var_gmax_dn4, locals.var_gmax_dn5, locals.var_gmax_dn6, locals.var_gmax_dn7, locals.var_gmax_dn8, locals.var_gmax_dn9, locals.var_gmax_dn10, locals.var_gmax_dn11,)
    }
};
        locals.var_gmax = assign5960_e6114;
        locals.var_gmax_dn0 = assign5960_e6114_d_n0;
        locals.var_gmax_dn1 = assign5960_e6114_d_n1;
        locals.var_gmax_dn3 = assign5960_e6114_d_n3;
        locals.var_gmax_dn4 = assign5960_e6114_d_n4;
        locals.var_gmax_dn5 = assign5960_e6114_d_n5;
        locals.var_gmax_dn6 = assign5960_e6114_d_n6;
        locals.var_gmax_dn7 = assign5960_e6114_d_n7;
        locals.var_gmax_dn8 = assign5960_e6114_d_n8;
        locals.var_gmax_dn9 = assign5960_e6114_d_n9;
        locals.var_gmax_dn10 = assign5960_e6114_d_n10;
        locals.var_gmax_dn11 = assign5960_e6114_d_n11;
        locals.var_gmax_rv = 0.0;

        let assign5970_e6117: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5970_e6117;
        locals.var_guard109_rv = 0.0;

        let (assign5980_e6131, assign5980_e6131_d_n0, assign5980_e6131_d_n1, assign5980_e6131_d_n3, assign5980_e6131_d_n4, assign5980_e6131_d_n5, assign5980_e6131_d_n6, assign5980_e6131_d_n7, assign5980_e6131_d_n8, assign5980_e6131_d_n9, assign5980_e6131_d_n10, assign5980_e6131_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) {
        let assign5980_e6127: f64 = (locals.var_gem - locals.var_gmax);
        let assign5980_e6129: f64 = (assign5980_e6127 / 1e-6);
        (assign5980_e6129, ((locals.var_gem_dn0 - locals.var_gmax_dn0) / 1e-6), ((locals.var_gem_dn1 - locals.var_gmax_dn1) / 1e-6), ((locals.var_gem_dn3 - locals.var_gmax_dn3) / 1e-6), ((locals.var_gem_dn4 - locals.var_gmax_dn4) / 1e-6), ((locals.var_gem_dn5 - locals.var_gmax_dn5) / 1e-6), ((locals.var_gem_dn6 - locals.var_gmax_dn6) / 1e-6), ((locals.var_gem_dn7 - locals.var_gmax_dn7) / 1e-6), ((locals.var_gem_dn8 - locals.var_gmax_dn8) / 1e-6), ((locals.var_gem_dn9 - locals.var_gmax_dn9) / 1e-6), ((locals.var_gem_dn10 - locals.var_gmax_dn10) / 1e-6), ((locals.var_gem_dn11 - locals.var_gmax_dn11) / 1e-6),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign5980_e6131;
        locals.var_dxa_dn0 = assign5980_e6131_d_n0;
        locals.var_dxa_dn1 = assign5980_e6131_d_n1;
        locals.var_dxa_dn3 = assign5980_e6131_d_n3;
        locals.var_dxa_dn4 = assign5980_e6131_d_n4;
        locals.var_dxa_dn5 = assign5980_e6131_d_n5;
        locals.var_dxa_dn6 = assign5980_e6131_d_n6;
        locals.var_dxa_dn7 = assign5980_e6131_d_n7;
        locals.var_dxa_dn8 = assign5980_e6131_d_n8;
        locals.var_dxa_dn9 = assign5980_e6131_d_n9;
        locals.var_dxa_dn10 = assign5980_e6131_d_n10;
        locals.var_dxa_dn11 = assign5980_e6131_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign5990_e6134: f64 = if locals.var_gem < locals.var_gmax { 1.0 } else { 0.0 };
        locals.var_guard110 = assign5990_e6134;
        locals.var_guard110_rv = 0.0;

        let (assign6000_e6154, assign6000_e6154_d_n0, assign6000_e6154_d_n1, assign6000_e6154_d_n3, assign6000_e6154_d_n4, assign6000_e6154_d_n5, assign6000_e6154_d_n6, assign6000_e6154_d_n7, assign6000_e6154_d_n8, assign6000_e6154_d_n9, assign6000_e6154_d_n10, assign6000_e6154_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        let assign6000_e6148: f64 = (locals.var_dxa).exp();
        let assign6000_e6149: f64 = (1.0 + assign6000_e6148);
        let assign6000_e6150: f64 = (assign6000_e6149).ln();
        let assign6000_e6151: f64 = (1e-6 * assign6000_e6150);
        let assign6000_e6152: f64 = (locals.var_gem - assign6000_e6151);
        (assign6000_e6152, (locals.var_gem_dn0 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn0) / assign6000_e6149))), (locals.var_gem_dn1 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn1) / assign6000_e6149))), (locals.var_gem_dn3 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn3) / assign6000_e6149))), (locals.var_gem_dn4 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn4) / assign6000_e6149))), (locals.var_gem_dn5 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn5) / assign6000_e6149))), (locals.var_gem_dn6 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn6) / assign6000_e6149))), (locals.var_gem_dn7 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn7) / assign6000_e6149))), (locals.var_gem_dn8 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn8) / assign6000_e6149))), (locals.var_gem_dn9 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn9) / assign6000_e6149))), (locals.var_gem_dn10 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn10) / assign6000_e6149))), (locals.var_gem_dn11 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn11) / assign6000_e6149))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign6000_e6154;
        locals.var_gem_dn0 = assign6000_e6154_d_n0;
        locals.var_gem_dn1 = assign6000_e6154_d_n1;
        locals.var_gem_dn3 = assign6000_e6154_d_n3;
        locals.var_gem_dn4 = assign6000_e6154_d_n4;
        locals.var_gem_dn5 = assign6000_e6154_d_n5;
        locals.var_gem_dn6 = assign6000_e6154_d_n6;
        locals.var_gem_dn7 = assign6000_e6154_d_n7;
        locals.var_gem_dn8 = assign6000_e6154_d_n8;
        locals.var_gem_dn9 = assign6000_e6154_d_n9;
        locals.var_gem_dn10 = assign6000_e6154_d_n10;
        locals.var_gem_dn11 = assign6000_e6154_d_n11;
        locals.var_gem_rv = 0.0;

        let (assign6010_e6176, assign6010_e6176_d_n0, assign6010_e6176_d_n1, assign6010_e6176_d_n3, assign6010_e6176_d_n4, assign6010_e6176_d_n5, assign6010_e6176_d_n6, assign6010_e6176_d_n7, assign6010_e6176_d_n8, assign6010_e6176_d_n9, assign6010_e6176_d_n10, assign6010_e6176_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 == 0.0)) {
        let assign6010_e6169: f64 = (-locals.var_dxa);
        let assign6010_e6170: f64 = (assign6010_e6169).exp();
        let assign6010_e6171: f64 = (1.0 + assign6010_e6170);
        let assign6010_e6172: f64 = (assign6010_e6171).ln();
        let assign6010_e6173: f64 = (1e-6 * assign6010_e6172);
        let assign6010_e6174: f64 = (locals.var_gmax - assign6010_e6173);
        (assign6010_e6174, (locals.var_gmax_dn0 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn0)) / assign6010_e6171))), (locals.var_gmax_dn1 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn1)) / assign6010_e6171))), (locals.var_gmax_dn3 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn3)) / assign6010_e6171))), (locals.var_gmax_dn4 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn4)) / assign6010_e6171))), (locals.var_gmax_dn5 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn5)) / assign6010_e6171))), (locals.var_gmax_dn6 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn6)) / assign6010_e6171))), (locals.var_gmax_dn7 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn7)) / assign6010_e6171))), (locals.var_gmax_dn8 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn8)) / assign6010_e6171))), (locals.var_gmax_dn9 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn9)) / assign6010_e6171))), (locals.var_gmax_dn10 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn10)) / assign6010_e6171))), (locals.var_gmax_dn11 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn11)) / assign6010_e6171))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign6010_e6176;
        locals.var_gem_dn0 = assign6010_e6176_d_n0;
        locals.var_gem_dn1 = assign6010_e6176_d_n1;
        locals.var_gem_dn3 = assign6010_e6176_d_n3;
        locals.var_gem_dn4 = assign6010_e6176_d_n4;
        locals.var_gem_dn5 = assign6010_e6176_d_n5;
        locals.var_gem_dn6 = assign6010_e6176_d_n6;
        locals.var_gem_dn7 = assign6010_e6176_d_n7;
        locals.var_gem_dn8 = assign6010_e6176_d_n8;
        locals.var_gem_dn9 = assign6010_e6176_d_n9;
        locals.var_gem_dn10 = assign6010_e6176_d_n10;
        locals.var_gem_dn11 = assign6010_e6176_d_n11;
        locals.var_gem_rv = 0.0;

        let assign6120_e6361: f64 = (1.0 - p.p68);
        let assign6120_e6363: f64 = (assign6120_e6361 * locals.var_cje_t);
        let assign6120_e6365: f64 = (assign6120_e6363 * locals.var_vte);
        locals.var_qte = assign6120_e6365;
        locals.var_qte_dn0 = (((assign6120_e6361 * locals.var_cje_t_dn0) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn0));
        locals.var_qte_dn1 = (((assign6120_e6361 * locals.var_cje_t_dn1) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn1));
        locals.var_qte_dn3 = (((assign6120_e6361 * locals.var_cje_t_dn3) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn3));
        locals.var_qte_dn4 = (((assign6120_e6361 * locals.var_cje_t_dn4) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn4));
        locals.var_qte_dn5 = (((assign6120_e6361 * locals.var_cje_t_dn5) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn5));
        locals.var_qte_dn6 = (((assign6120_e6361 * locals.var_cje_t_dn6) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn6));
        locals.var_qte_dn7 = (((assign6120_e6361 * locals.var_cje_t_dn7) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn7));
        locals.var_qte_dn8 = (((assign6120_e6361 * locals.var_cje_t_dn8) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn8));
        locals.var_qte_dn9 = (((assign6120_e6361 * locals.var_cje_t_dn9) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn9));
        locals.var_qte_dn10 = (((assign6120_e6361 * locals.var_cje_t_dn10) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn10));
        locals.var_qte_dn11 = (((assign6120_e6361 * locals.var_cje_t_dn11) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn11));
        locals.var_qte_rv = 0.0;

    }
}
